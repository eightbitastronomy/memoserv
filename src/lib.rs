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
pub mod manager;
pub mod configmodifier;


use event_listener::{Event};
use memobook::configuration::Configuration;
use memobook::{MemoBook, Queryable};
use zbus::{interface, object_server::SignalEmitter, Result};
use std::sync::{Arc,Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use memobook::mbfilter::MBFilter;
use memobook::query::Query;
use memobook::modifiers::Modifier;
use crate::parse::*;
use crate::manager::Manager;
use crate::configmodifier::ConfigModifier;



pub struct MemoBookServer {
    pub name: String,
    pub events: Arc<Event>,
    pub exitflag: Arc<AtomicBool>,
    pub cfg: Configuration,
    pub mb: Arc<Mutex<MemoBook>>
}


#[interface(name = "org.memobook.memoserv1")]
impl MemoBookServer {

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
        //let memobk = self.mb.lock().unwrap();
        self.cfg.assemble_backup_info()
    }


    async fn repositories(&self) -> String {
        //let memobk = self.mb.lock().unwrap();
        self.cfg.assemble_repo_info()
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
        let clientcmd: Modifier = match parse_modification_msg(vcommand) {
            Ok(m) => m,
            Err(e) => return format!("Modify request error: {e}")
        };
        let mut memobk = self.mb.lock().unwrap();
        self.cfg.check_backup(true);
        match memobk.modify(&clientcmd) {
            Ok(()) => { self.cfg.mb_alt(true);
                "".to_string()
            },
            Err(th) => format!("Modify request returned error: {th}")
        }
    }


    // NOTE, A REPO CHANGE MIGHT AFFECT FILES IN THE DB, SO A BACKUPMGR CALL
    //  MIGHT BE IN ORDER...

    async fn manage(&mut self, vcommand: Vec<&str>) -> String {
        self.manage_helper(vcommand)
        /* let clientcmd: Manager = match parse_manage_msg(vcommand) {
            Ok(m) => m,
            Err(e) => return format!("Manage request error: {e}")
        };
        match clientcmd {
            Manager::Configure(cfg) => {
                match cfg {
                    ConfigModifier::SetSource(ss) => {
                        let mut memobk = self.mb.lock().unwrap();
                        self.cfg.check_backup(true);
                        self.cfg.set_source(&ss);
                        match memobk.connect(Some(ss)) {
                            Ok(()) => return "".to_string(),
                            Err(e) => return format!("Error managing source: {e}")
                        }
                    },
                    ConfigModifier::SetRepo(sr) => {
                        let mut memobk = self.mb.lock().unwrap();
                        self.cfg.check_backup(true);
                        self.cfg.set_repo_by_repo(sr);
                        match memobk.target(&self.cfg) {
                            Ok(()) => return "".to_string(),
                            Err(e) => return format!("Error managing repo: {e}")
                        }
                    },
                    ConfigModifier::ModifyRepo(mrtuple) => { 
                        let mut memobk = self.mb.lock().unwrap();
                        self.cfg.check_backup(true);
                        self.cfg.modify_repo_by_repo(mrtuple);
                        match memobk.target(&self.cfg) {
                            Ok(()) => return "".to_string(),
                            Err(e) => return format!("Error managing repo: {e}")
                        }
                    },
                    ConfigModifier::ModifyBackup(bu) => {
                        let _memobk = self.mb.lock().unwrap();
                        return match self.cfg.modify_backup(&bu) {
                            Ok(()) => "".to_string(),
                            Err(e) => format!("Error altering backup information: {e}")
                        }
                    }
                }
            },
            // IMPORT WILL ALMOST CERTAINLY ALTER THE DB, SO DO A BACKUP
            Manager::Import(imp) => {
                let memobk = self.mb.lock().unwrap();
                self.cfg.check_backup(true);
                match memobk.import(imp) {
                    Ok(s) => return s,
                    Err(e) => return format!("Error importing files: {e}")
                }
            },
            Manager::Export(exp) => {
                let memobk = self.mb.lock().unwrap();
                match memobk.export(exp) {
                    Ok(s) => return s,
                    Err(e) => return format!("Error exporting: {e}")
                }
            },
            Manager::Backup => {
                let _memobk = self.mb.lock().unwrap();
                match self.cfg.do_backup() {
                    Ok(s) => return s,
                    Err(e) => return format!("Error making backup: {e}")
                }
            }
        } */
    }


    async fn manage_no_reponse(&mut self, vcommand: Vec<&str>) {
        _ = self.manage_helper(vcommand);
    }


    async fn exit(&self) -> String {
        {
            let mut memobk = self.mb.lock().unwrap();
            memobk.disconnect();
            self.cfg.finish();
            self.exitflag.store(true, Ordering::SeqCst);
            self.events.notify(1);
        }
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
                    self.cfg.check_backup(true);
                    self.cfg.set_source(&ss);
                    match memobk.connect(Some(ss)) {
                        Ok(()) => "".to_string(),
                        Err(e) => format!("Error managing source: {e}")
                    }
                },
                ConfigModifier::SetRepo(sr) => {
                    let mut memobk = self.mb.lock().unwrap();
                    self.cfg.check_backup(true);
                    self.cfg.set_repo_by_repo(sr);
                    match memobk.target(&self.cfg) {
                        Ok(()) => "".to_string(),
                        Err(e) => format!("Error managing repo: {e}")
                    }
                },
                ConfigModifier::ModifyRepo(mrtuple) => { 
                    let mut memobk = self.mb.lock().unwrap();
                    self.cfg.check_backup(true);
                    self.cfg.modify_repo_by_repo(mrtuple);
                    match memobk.target(&self.cfg) {
                        Ok(()) => "".to_string(),
                        Err(e) => format!("Error managing repo: {e}")
                    }
                },
                ConfigModifier::ModifyBackup(bu) => {
                    let _memobk = self.mb.lock().unwrap();
                    match self.cfg.modify_backup(&bu) {
                        Ok(()) => "".to_string(),
                        Err(e) => format!("Error altering backup information: {e}")
                    }
                }
            }
        },
        // IMPORT WILL ALMOST CERTAINLY ALTER THE DB, SO DO A BACKUP
        Manager::Import(imp) => {
            let mut memobk = self.mb.lock().unwrap();
            self.cfg.check_backup(true);
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
        Manager::Backup => {
            let _memobk = self.mb.lock().unwrap();
            match self.cfg.do_backup() {
                Ok(s) => s,
                Err(e) => format!("Error making backup: {e}")
            }
        }
    }
}

}
