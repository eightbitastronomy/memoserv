//  lib.rs
//
//  Author: Miguel Abele (eightbitastronomy@protonmail.com)
//  Copyrighted by Miguel Abele (eightbitastronomy), 2025.
//
//  License information:
//
//  This file is a part of MemoServ.
//
//  MemoServ is free software; you can redistribute it and/or
//  modify it under the terms of the GNU General Public License
//  as published by the Free Software Foundation; either version 3
//  of the License, or (at your option) any later version.
//
//  MemoServ is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program; if not, write to the Free Software
//  Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.


pub mod parse;
pub mod prepare;
pub mod manager;
pub mod configmodifier;


use event_listener::{Event};
use memobook::configuration::Configuration;
use memobook::backer::Backer;
use memobook::backerparserjson::BackerParserJSON;
use memobook::{MemoBook, Queryable};
use zbus::{interface, object_server::SignalEmitter, Result};
use std::sync::{Arc,Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use memobook::mbfilter::MBFilter;
use memobook::query::Query;
use memobook::modifiers::Modifier;
use crate::parse::*;
use crate::prepare::*;
use crate::manager::Manager;
use crate::configmodifier::ConfigModifier;



pub struct MemoBookServer<B>
where B: Backer+BackerParserJSON + std::marker::Send+ 'static
{
    pub name: String,
    pub events: Arc<Event>,
    pub exitflag: Arc<AtomicBool>,
    pub cfg: Arc<Mutex<Configuration<B>>>,
    pub mb: Arc<Mutex<MemoBook>>
}



unsafe impl<B: Backer+BackerParserJSON + std::marker::Send> Send for MemoBookServer<B> {}



#[interface(name = "org.memobook.memoserv1")]
impl<B> MemoBookServer<B>
where B: Backer+BackerParserJSON + std::marker::Send+'static
{

    async fn toc(&self, toctype: &str) -> String {
        let result = match parse_toc_msg(toctype) {
            Ok(q) => {
                let memobk = self.mb.lock().unwrap();
                memobk.search(q)
            },
            Err(e) => { 
                return format!("{e}");
            }
        };
        match result {
            Ok(r) => {
                r.join(", ")
            },
            Err(e) => { 
                format!("{e}")
            }
        }
    }


    async fn backup(&self) -> String {
        let memocfg = self.cfg.lock().unwrap();
        memocfg.assemble_backup_info()
    }


    async fn repositories(&self) -> String {
        let memocfg = self.cfg.lock().unwrap();
        memocfg.assemble_repo_info()
    }


    async fn search(&self, vfilter: Vec<&str>) -> String {
        let clientquery: Query<MBFilter> = match parse_search_msg(vfilter) {
            Ok(q) => q,
            Err(e) => return format!("Search error: {e}")
        };
        let memobk = self.mb.lock().unwrap();
        match memobk.search(clientquery) {
            Ok(cq) => cq.join(", "),
            Err(e) => { format!("Error in search: {e}") }
        }
    }


    async fn modify(&mut self, vcommand: Vec<&str>) -> String {
        let mut clientcmd: Modifier = match parse_modification_msg(vcommand) {
            Ok(m) => m,
            Err(e) => return format!("Modify request error: {e}")
        };
        { // lock memobook
            let mut memobk = self.mb.lock().unwrap();
            match prepare_modification(&memobk, &mut clientcmd) {
                Ok(_) => { },
                Err(e) => { return format!("Error in modification auxiliary search: {e}"); }
            }
            { // lock config
                let mut memocfg = self.cfg.lock().unwrap();
                memocfg.check_backup(true);
                match memobk.modify(&clientcmd) {
                    Ok(()) => { 
                        memocfg.mb_alt(true);
                        "".to_string()
                    },
                    Err(th) => format!("Modify request returned error: {th}")
                }
            } // release config
        } // release memobook
    }


    async fn manage(&mut self, vcommand: Vec<&str>) -> String {
        self.manage_helper(vcommand) 
    }


    async fn manage_no_reponse(&mut self, vcommand: Vec<&str>) {
        _ = self.manage_helper(vcommand);
    }


    async fn exit(&self) -> String {
        { // lock memobook
            let mut memobk = self.mb.lock().unwrap();
            memobk.disconnect();
            { // lock config
                let memocfg = self.cfg.lock().unwrap();
                memocfg.finish();
            } // release config
            self.exitflag.store(true, Ordering::SeqCst);
            self.events.notify(1);
        } // release memobook
        "Exiting".to_string()
    }


    #[zbus(property)]
    async fn memobook_name(&self) -> &str {
        &self.name
    }


    #[zbus(property)]
    async fn set_memobook_name(&mut self, name: String) {
        self.name = name;
    }


    #[zbus(signal)]
    async fn exited(emitter: &SignalEmitter<'_>) -> Result<()>;

//}


fn manage_helper(&mut self, vcommand: Vec<&str>) -> String {
    let clientcmd: Manager = match parse_manage_msg(vcommand) {
        Ok(m) => m,
        Err(e) => return format!("Manage request error: {e}")
    };
    match clientcmd {
        Manager::Configure(cfg) => {
            match cfg {
                ConfigModifier::SetSource(ss) => {
                    let mut memobk = self.mb.lock().unwrap();
                    {
                        let mut memocfg = self.cfg.lock().unwrap();
                        memocfg.check_backup(true);
                        memocfg.set_source(&ss);
                    }
                    match memobk.connect(Some(ss)) {
                        Ok(()) => "".to_string(),
                        Err(e) => format!("Error managing source: {e}")
                    }
                },
                ConfigModifier::SetRepo(sr) => {
                    let mut memobk = self.mb.lock().unwrap();
                    {
                        let mut memocfg = self.cfg.lock().unwrap();
                        memocfg.check_backup(true);
                        memocfg.set_repo_by_repo(sr);   
                        match memobk.target(&memocfg.mb().scan, memocfg.mime()) {
                            Ok(()) => "".to_string(),
                            Err(e) => format!("Error managing repo: {e}")
                        }
                    }
                },
                ConfigModifier::ModifyRepo(mrtuple) => { 
                    let mut memobk = self.mb.lock().unwrap();
                    {
                        let mut memocfg = self.cfg.lock().unwrap();
                        memocfg.check_backup(true);
                        memocfg.modify_repo_by_repo(mrtuple);
                        match memobk.target(&memocfg.mb().scan, memocfg.mime()) {
                            Ok(()) => "".to_string(),
                            Err(e) => format!("Error managing repo: {e}")
                        }
                    }
                },
                /*ConfigModifier::ModifyBackup(bu) => {
                    let _memobk = self.mb.lock().unwrap();
                    match self.cfg.modify_backup(&bu) {
                        Ok(()) => "".to_string(),
                        Err(e) => format!("Error altering backup information: {e}")
                    }
                }*/
            }
        },
        // IMPORT WILL ALMOST CERTAINLY ALTER THE DB, SO DO A BACKUP
        Manager::Import(imp) => {
            let mut memobk = self.mb.lock().unwrap();
            {
                let mut memocfg = self.cfg.lock().unwrap();
                memocfg.check_backup(true);
            }
            match memobk.import(imp) {
                Ok(s) => s,
                Err(e) => format!("Error importing files: {e}")
            }
        },
        Manager::Export(exp) => {
            let memobk = self.mb.lock().unwrap();
            match memobk.export(exp) {
                Ok(s) => s,
                Err(e) => format!("Error exporting: {e}")
            }
        },
        Manager::Backup(bup) => {
            let mut memobk = self.mb.lock().unwrap();
            memobk.disconnect();
            let connectable: Option<String>;
            {
                let mut memocfg = self.cfg.lock().unwrap();
                connectable = match memocfg.process_modify_backup(&bup) {
                    Ok(Some(s)) => Some(s),
                    Ok(None) => None,
                    Err(e) => return format!("Error in backup modification call: {e}")
                };
            }
            match memobk.connect(connectable) {
                Ok(_) => "".to_string(),
                Err(x) => format!("Backup data could not be loaded: {:?}", x)
            }
        }
    }
}

}
