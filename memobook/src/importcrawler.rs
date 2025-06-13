//  importcrawler.rs
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


pub mod import_crawler {

use crate::mberror::MBError;
use crate::repository::Repository;
use std::path::PathBuf;
use crate::crawler::{Crawler, CrawlOption};
use crate::filecrawler::FileCrawler;
use sha256::digest;
use json;
use crate::modifiers::ModifyAddRecord;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};


#[derive(Clone)]
pub struct ImportPair {
    pub sum: String,
    pub target: PathBuf
}


//pub struct MBRecord {
//    pub filenm: PathBuf,
//    pub mark: Vec<String>,
//    pub ftype: Vec<String>
//}


pub struct ImportCrawler {

    repo: Option<Repository>,
    followlink: bool,
    importfile: Option<PathBuf>,
    logfile: Option<PathBuf>,
    results: Vec<ModifyAddRecord>,
    results_empty: bool,
    crawler: FileCrawler<ImportPair>

}


impl Default for ImportCrawler {
    fn default() -> Self {
        Self::new()
    }
}


impl ImportCrawler {

    pub fn new() -> ImportCrawler {
        ImportCrawler {
            repo: None,
            followlink: false,
            importfile: None,
            logfile: None,
            results: Vec::new(),
            results_empty: true,
            crawler: FileCrawler::<ImportPair>::new()
        }
    }

    pub fn set_options(&mut self, optsenum: CrawlOption) -> &mut ImportCrawler {
        match optsenum {
            CrawlOption::FollowLinks(ref b) => { self.followlink = *b; },
            CrawlOption::Repository(ref r) => { self.repo = Some(r.clone()); },
            CrawlOption::Transport(ref tp) => {
                self.importfile = PathBuf::from(tp)
                            .as_path()
                            .canonicalize() 
                            .ok();
            },
            CrawlOption::Log(ref l) => {
                self.logfile = Some(PathBuf::from(l));
                //println!("log file is {l}");
                /*self.logfile = PathBuf::from(l)
                            .as_path()
                            .canonicalize() 
                            .ok();
                match self.logfile.as_ref() {
                    Some(lf) => { println!("log file path is {}", lf.display()); },
                    None => { println!("log file holder is empty"); }
                }*/
            },
            _ => {}
        }
        _ = self.crawler.options(optsenum);
        self
    }


    pub fn crawl(&mut self) -> Result<&Self, MBError> {
        self.results_empty = true;
        let Some(ref importsrc) = self.importfile else {
            return Err(MBError::Import("no source file specified".to_string()));
        };
        let importfdata: Vec<u8> = match fs::read(importsrc) {
            Ok(x) => x,
            Err(_) => { return Err(MBError::Import("source read error".to_string())); }
        };
        let Some(ref logtarget) = self.logfile else {
            return Err(MBError::Import("no log file specified".to_string()));
        };
        let logfile = match File::create(logtarget) {
            Ok(file) => file,
            Err(e) => { return Err(MBError::Import(format!("log file could not be opened: {e}"))); }
        };
        // Parse the input into json
        let mut recordsdict: json::JsonValue = match json::parse(String::from_utf8(importfdata).unwrap().as_str()) {
            Ok(rj) => rj,
            Err(e) => { return Err(MBError::Import(format!("unable to parse import json: {e}"))); }
        };
        // prepare the processor function and other vars
        let resultsvec: Vec<ImportPair> = match self.crawler.crawl(
            &mut |filecanon: PathBuf| {
                let inputdata = match fs::read(filecanon.as_path()) {
                    Ok(f) => f,
                    Err(_) => return Err(MBError::Nil)
                };
                //let mut summer = Sha256::new();
                //summer.update(inputdata);
                //let sum: String = summer.finalize();
                let sum: String = digest(inputdata);
                if recordsdict.has_key(&sum) {
                    return Ok(ImportPair {sum, target: filecanon})
                }
                Err(MBError::Nil)
            }
        ) {
            Ok(c) => match c.retrieve() {
                Some(v) => v,
                None => {
                    self.results = vec![];
                    return Ok(self);
                }
            },
            Err(e) => { return Err(MBError::Import(format!("import error: {e}"))); }
        };
        let mut processedvec: Vec<ModifyAddRecord> = Vec::new();
        for pair in resultsvec.iter() {
            let temprecordjson = recordsdict.remove(&pair.sum);
            match temprecordjson {
                json::JsonValue::Null => { 
                    continue;
                },
                json::JsonValue::Object(jobj) =>{ 
                    processedvec.push(
                        ModifyAddRecord {
                            file: pair.target.as_path().to_str().unwrap().to_string(),
                            marks: match &jobj["mark"] {
                                json::JsonValue::Array(ar) => {
                                    ar.iter().map(|x| 
                                        match x {
                                            json::JsonValue::Short(s) => s.to_string(),
                                            json::JsonValue::String(s) => s.to_owned(),
                                            _ => "".to_string()
                                        }
                                    ).filter(|e| !e.is_empty())
                                    .collect()
                                },
                                json::JsonValue::Short(s) => vec![s.to_string()],
                                json::JsonValue::String(s) => vec![s.to_owned()],
                                _ => vec![] 
                            },
                            ftypes: match &jobj["type"] {
                                json::JsonValue::Array(ar) => {
                                    ar.iter().map(|x| 
                                        match x {
                                            json::JsonValue::Short(s) => s.to_string(),
                                            json::JsonValue::String(s) => s.to_owned(),
                                            _ => "".to_string()
                                        }
                                    ).filter(|e| !e.is_empty())
                                    .collect()
                                },
                                json::JsonValue::Short(s) => vec![s.to_string()],
                                json::JsonValue::String(s) => vec![s.to_owned()],
                                _ => vec![]
                            }
                        }
                    );
                },
                _ => {
                    continue;
                }
            }
        }
        self.results = processedvec;
        self.results_empty = false;
        if !recordsdict.is_empty() {
            let mut logbufwriter = BufWriter::new(logfile);
            match logbufwriter.write_all(json::stringify_pretty(recordsdict,4).as_bytes()) {
                // useless with write_all()
                //Ok(x) if x == 0 => { println!("error writing import log file (zero bytes written)"); },
                Ok(_) => { }, 
                Err(_) => { println!("error writing import log file"); }
            };
            match logbufwriter.flush() {
                Ok(_) => {},
                Err(_) => { println!("error flushing import log"); }
            }
        }
        Ok(self)
    }
    
    
    pub fn retrieve(&self) -> Option<&Vec<ModifyAddRecord>> {
        
        if self.results_empty {
            None
        } else {
            Some(&self.results)
        }
    }


    pub fn iter(&self) -> impl Iterator<Item = &ModifyAddRecord> {
        //let buflist: Vec<ModifyAddRecord> = self.results.clone().unwrap_or(vec![]);
        ImportIterator {
            //list: &buflist,
            list: &self.results,
            index: 0,
            len: self.results.len()
        }
    }

}


pub struct ImportIterator<'a> {
    list: &'a Vec<ModifyAddRecord>,
    index: usize,
    len: usize
}

impl<'a> Iterator for ImportIterator<'a> {
    
    type Item = &'a ModifyAddRecord;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = if self.index < self.len {
            Some(&self.list[self.index])
        } else {
            None
        };
        self.index += 1;
        ret   
    }

}

} // pub mod importcrawler

/*
#[cfg(test)]
mod tests {

    //use crate::assembler::Assembler;
    //use crate::queryer::Queryer;
    //use crate::query::Query;
    use crate::logic::Logic;
    //use crate::filtercontainer::FilterContainer;
    //use crate::mbfilter::MBFilter;
    //use crate::mberror::MBError;
    //use super::Lite2Assembler::LiteAssembler; 
    use super::file_2_crawler::rem_dupes;

    #[test]
    fn test_rem_dupes() {
    }

}*/
 
