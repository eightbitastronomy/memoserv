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


#![allow(clippy::new_without_default)]

pub mod mbmacro;
pub mod mimer;
pub mod query;
pub mod queryer;
pub mod logic;
pub mod mbfilter;
pub mod filtercontainer;
pub mod queryassembler;
pub mod litequeryassembler;
pub mod configuration;
pub mod repository;
pub mod mberror;
pub mod maskingset;
pub mod suffixhash;
pub mod modifierassembler;
pub mod litemarkupdate;
pub mod litefieldreplace;
pub mod litetargetremove;
pub mod liteaddrecord;
pub mod dbopenerassembler;
pub mod liteopen;
pub mod modifiers;
pub mod emptygenerator;
pub mod dbgenerator;
pub mod crawler;
pub mod filecrawler;
pub mod grepcrawler;
pub mod transportstruct;
pub mod importcrawlerp; // <--Change here to use synchronous importcrawler
pub mod liteexportquery;
pub mod exportlogger;
pub mod dbhexgenerator;
pub mod backer;
pub mod utcbackup;
pub mod utckeeper;
pub mod backerparserjson;
pub mod utcparser;


use rusqlite::{Connection, Error};
//use std::time::Instant;
use std::collections::HashMap as HashMap;
use mimer::Mimer as Mimer;
use configuration::MBInfo as MBInfo;
use crate::queryer::Queryer as Queryer;
use crate::filtercontainer::FilterContainer;
use crate::logic::Logic as Logic;
use crate::queryassembler::QueryAssembler;
use crate::litequeryassembler::lite_query_assembler::LiteQueryAssembler as LiteQueryAssembler;
use crate::modifierassembler::ModifierAssembler;
use crate::mberror::MBError;
use crate::configuration::Configuration;
use crate::dbopenerassembler::DBOpenerAssembler;
use crate::liteopen::LiteOpen;
use crate::modifiers::Modifier;
use crate::liteaddrecord::LiteAddRecord;
use crate::litefieldreplace::LiteFieldReplace;
use crate::litemarkupdate::LiteMarkUpdate;
use crate::litetargetremove::LiteTargetRemove;
use crate::grepcrawler::grep_crawler::GrepCrawler;
use crate::crawler::CrawlOption;
use crate::transportstruct::TransPortStruct;
use crate::importcrawlerp::import_crawler::ImportCrawler; // <--Change here to use synchronous importcrawler
use crate::exportlogger::ExportLogger;


fn gather_types(query: &impl (for <'a> Queryer<'a>)) -> Option<Vec<String>> {
    let mut retvec: Vec<String> = vec![];
    'filts: for filter in query.iter_filters() {
        match filter.filtertype() {
            "type" => {
                for term in filter.iter() {
                    retvec.push(term.to_string());
                }
                return Some(retvec); 
            },
            _ => continue 'filts
        }
    }
    None
}


fn gather_marks(query: &impl (for <'a> Queryer<'a>)) -> Result<(Logic,Vec<String>), MBError> {
    let mut rettup = (Logic::OR, vec![]);
    'filts: for filter in query.iter_filters() {
        match filter.filtertype() {
            "mark" => {
                rettup.0 = *filter.logic();
                for term in filter.iter() {
                    rettup.1.push(term.to_string());
                }
                if rettup.1.is_empty() {
                    return Err(MBError::MarkGather("unexpected empty marks list".to_string()));
                }
                return Ok(rettup); 
            },
            _ => continue 'filts
        }
    }
    Err(MBError::MarkGather("no marks filter found in query".to_string()))
}


pub trait Queryable {
    fn initialize(&mut self) -> Result<(), MBError>;
    fn connect(&mut self, source: Option<String>) -> Result<(), MBError>;
    fn search(&self, req: impl (for <'a> Queryer<'a>)) -> Result<Vec<String>, MBError>;
    fn modify(&mut self, cmd: &Modifier) -> Result<(), MBError>;
    fn target(&mut self, cfg: &Configuration) -> Result<(), MBError>;
    fn import(&mut self, portinfo: TransPortStruct) -> Result<String, MBError>;
    fn export(&self, portinfo: TransPortStruct) -> Result<String, MBError>;
    fn disconnect(&mut self);  
}


pub struct MemoBook {
    connection: Option<Connection>,
    info: MBInfo,
    mime: HashMap<String,Mimer>
    //assemblers: DBBundler;
}


impl MemoBook {

    //REWRITE: NEW SHOULD RETURN AN EMPTY MEMOBOOK. INITIALIZE SHOULD BE USED TO SET EVERYTHING.
    pub fn new(config: &mut Configuration /*, dbtype: DBType*/) -> MemoBook {
        MemoBook {
            connection: None, 
            info: config.mb().clone(),
            mime: config.mime().clone()
            //translate: DBBundler::new(dbtype)
        }
    }

    fn connection_helper(&mut self) -> Result<(), Error> {
        let conn = Connection::open(&self.info.src)?;
        let dbtype = LiteOpen; 
        match conn.prepare(
            dbtype.form_select_all(&self.info.table)
            .as_str()
        )?
        .query([]) 
        {
            Ok(mut q) => {
                match q.next() {
                    Ok(Some(_)) => { },
                    Ok(None) => {
                        conn.execute(
                            dbtype.form_create_table(&self.info.table)
                            .as_str(),
                            ()
                        )?;
                    },
                    Err(r) => { return Err(r); }
                }
            },
            Err(q) => { return Err(q); }
        }
        self.connection = Some(conn);
        Ok(())
    }

    fn resolve_type_suffix(&self, typeopt: Option<Vec<String>>) -> Result<Vec<String>, MBError> {
        let mut resultv: Vec<String> = Vec::new();
        if let Some(typev) = typeopt {
            for item in typev {
                let mut buffv: Vec<String> = self.mime.get(item.to_string().as_str())
                                                .unwrap()
                                                .iter()
                                                .map(|s| s.to_string())
                                                .collect();
                resultv.append(&mut buffv);
            }
        } else {
            resultv.push("*".to_string());
        }
        if resultv.is_empty() {
            return Err(MBError::TypeGather("unexpected empty types list".to_string()));
        }
        Ok(resultv)
    }
   

    fn search_helper(&self, cnx: &Connection, query: String) -> Result<Vec<String>, MBError> {
        let mut v: Vec<String> = Vec::new();
	    let mut state = match cnx.prepare(query.as_str()) {
            Ok(x) => x,
            Err(x) => { return Err(MBError::Sqlite(x)); }
        };
        let mut rows = match state.query([]) {
            Ok(x) => x,
            Err(x) => { return Err(MBError::Sqlite(x)); }
        };
        loop {
            let row = match rows.next() {
                Ok(x) => match x {
                    Some(y) => y,
                    None => break
                },
                Err(x) => { return Err(MBError::Sqlite(x)); }
            };
            let fromrow = match row.get(0) {
                Ok(x) => x,
                Err(x) => { return Err(MBError::Sqlite(x)); }
            };
            v.push(fromrow);
        } 
        Ok(v)
    }
 
}



unsafe impl Send for MemoBook {}


impl Queryable for MemoBook {

    fn initialize(&mut self) -> Result<(), MBError> {
        // Future: this function should handle the case where there was no conf.json given.
        //         It will create the database and give the configuration object the info
        //         necessary to write out the json.
        if self.connection.is_none() {
            self.connection = match Connection::open(&self.info.src) { 
                Ok(x) => Some(x),
                Err(x) => { return Err(MBError::Sqlite(x)); } 
            }
        }
        let opener = LiteOpen;
        let conn = self.connection.as_ref().unwrap();
        //match conn.execute(format!("create table {} (mark NCHAR(255) NOT NULL,file NCHAR(1023) NOT NULL,type SMALLINT)",&self.info.table).as_str(),())
        //match conn.execute(LiteOpen::form_create_table(&self.info.table).as_str(), ())
        match conn.execute(opener.form_create_table(&self.info.table).as_str(), ())
        {   Ok(_) => Ok(()),
            Err(x) => Err(MBError::Sqlite(x)) 
        }    
    }


    ///Connect to database: open table or create it if it doesn't exist
    fn connect(&mut self, source: Option<String>) -> Result<(), MBError> {
        self.connection = None;
        if let Some(newsrc) = source { 
            self.info.src = newsrc.to_string(); 
        }
        match self.connection_helper() {
            Ok(_) => Ok(()),
            Err(x) => Err(MBError::Sqlite(x))
        }
    }

    
    fn search(&self, req: impl (for <'a> Queryer<'a>)) -> Result<Vec<String>, MBError> {
		let mut v = Vec::new();
        if req.grep() {
            /********* windows, no grep functionality?, need a macro or something for windows detection *******/
            // maybe require uutils or coreutils for grep...?
            println!("Grepping...");
            let resolvedtypelist: Vec<String> = self.resolve_type_suffix(gather_types(&req))?;
            let marktuple = gather_marks(&req)?;
            let mut fs_searcher: GrepCrawler = GrepCrawler::new();
            fs_searcher.set_options(CrawlOption::CaseSensitive(req.grepcase())) 
                .set_options(CrawlOption::FollowLinks(req.greplink()))
                .set_options(CrawlOption::Repository(self.info.scan.clone()))
                .set_search_terms(marktuple, resolvedtypelist);
            //let now = Instant::now();
            if let Some(mut resultvec) = fs_searcher.crawl()?.retrieve() {
                v.append(&mut resultvec);
            }
            //println!("Time elapsed for grep operation: {}", now.elapsed().as_millis());
        }
        if let Some(conn) = self.connection.as_ref() {
            let queryassembler = LiteQueryAssembler::new(&self.info.table, req);
            /* here, check for complexity(), need an in-code algorithm if complexity is too high */
            let querystring = queryassembler.form()?;
            match self.search_helper(conn, querystring) {
                Ok(mut res) => { 
                    //res = res.into_iter().filter(|r| !r.is_empty()).collect();
                    res.retain(|r| !r.is_empty());
                    v.append(&mut res); 
                },
                Err(e) => { return Err(e); }
            }
        }
        v = rem_dupes!(&v); // don't remove. grep might give same hits as db does.
        v.sort();
        Ok(v)
    }


    // behavior of MarkUpdate: 
    //   o  requires the caller to know all the possible types in the case
    //      where # of rems != # of adds. This means the caller will need to query for the types
    //      separately.
    //   o  will insert one record into db for each combination of file, type, & mark given,
    //      hence a cartesian product.
    fn modify(&mut self, cmd: &Modifier) -> Result<(), MBError> {
        if let Some(conn) = self.connection.as_mut() {
            let cmdobj: Box<dyn ModifierAssembler> = match cmd {
                Modifier::AddRecord(_) => Box::new(LiteAddRecord),
                Modifier::FieldReplace(_) => Box::new(LiteFieldReplace),
                Modifier::MarkUpdate(_) => Box::new(LiteMarkUpdate),
                Modifier::TargetRemove(_) => Box::new(LiteTargetRemove)
            };
            let transact = match conn.transaction() {
                Ok(t) => t,
                Err(e) => return Err(MBError::Sqlite(e))
            };
            match transact.execute_batch(cmdobj.form(&self.info.table, cmd)?.join(" ").as_str()) {
                Ok(_) => {},
                Err(e) => return Err(MBError::BadModify(format!("DB modification error: {e}")))
            }
            match transact.commit() {
                Ok(_) => {},
                Err(e) => return Err(MBError::Sqlite(e))
            }
        }
        Ok(())
    }

    fn target(&mut self, cfg: &Configuration) -> Result<(), MBError> {
        self.info.scan = cfg.mb().scan.clone();
        self.mime = cfg.mime().clone();
        Ok(())
    }

    fn import(&mut self, portinfo: TransPortStruct) -> Result<String, MBError> {
        if let Some(conn) = self.connection.as_mut() {
            let mut fs_importer: ImportCrawler = ImportCrawler::new();
            fs_importer.set_options(CrawlOption::FollowLinks(portinfo.links))
                .set_options(CrawlOption::Transport(portinfo.target))
                .set_options(CrawlOption::Log(portinfo.log))
                .set_options(CrawlOption::Repository(self.info.scan.clone()));
            // Setup the async runtime for use with importcrawlerp, or comment all out
            let asyncruntime = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build() 
            {
                Ok(rt) => rt,
                Err(e) => { return Err(MBError::Import(format!("Async error during import: {e}"))); }
            };
            let asyncresult = asyncruntime.block_on(async { fs_importer.crawl().await });
            // Change the next match line according to whether async is being used or not
            //match fs_importer.crawl() {
            match asyncresult {
                Ok(_) => {},
                Err(e) => { return Err(e); }
            }
            ////let start = Instant::now();
            // Start a transaction for the database calls, assert the calls, then commit
            let transact = match conn.transaction() {
                Ok(t) => t,
                Err(e) => return Err(MBError::Sqlite(e))
            };
            for result in fs_importer.iter() {
                let cmdobj = Box::new(LiteAddRecord);
                let cmd: Modifier = Modifier::AddRecord(result.clone());
                match transact.execute_batch(cmdobj.form(&self.info.table, &cmd)?.join(" ").as_str()) {
                    Ok(_) => {},
                    Err(e) => return Err(MBError::BadModify(format!("DB import error: {e}")))
                }
            }
            match transact.commit() {
                Ok(_) => {},
                Err(e) => return Err(MBError::Sqlite(e))
            }
            ////let duration = start.elapsed().as_millis();
            ////println!("Duration of database insertions: {}", duration);
        }
        Ok("pending".to_string())
    }

    fn export(&self, portinfo: TransPortStruct) -> Result<String, MBError> {
        if let Some(conn) = self.connection.as_ref() {
            let mut logger: ExportLogger = ExportLogger::new(&portinfo.log, "bookmarks");
            logger.prepare(conn)?;
            logger.dump(conn)?;
        } 
        Ok(format!("export to {} complete",&portinfo.log))   
    }

    /* instead of implementing something here, must handle writing out the configuration */
    //Nope, this will be done elsewhere. The db only has need-to-know info from the configuration.
    fn disconnect(&mut self) {
    }   


}
