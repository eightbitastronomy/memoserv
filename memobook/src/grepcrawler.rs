//  grepcrawler.rs
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


pub mod grep_crawler {

use crate::logic::Logic;
use crate::mberror::MBError;
use crate::repository::Repository;
use std::collections::HashMap;
use std::process::Command;
use std::path::PathBuf;
use crate::maskingset::MaskingSet;
use crate::suffixhash::SuffixHash;
use crate::crawler::{Crawler, CrawlOption};
use crate::filecrawler::FileCrawler;


pub struct LogicalHash {
    logic: Logic,
    criterion: usize,
    sethash: HashMap<String, usize>
}


impl LogicalHash {

    pub fn new(logic: Logic) -> LogicalHash {
        LogicalHash { logic, criterion: 1, sethash: HashMap::new() }
    }    

    pub fn add(&mut self, item: &str) -> &mut LogicalHash {
        match self.logic {
            Logic::AND => {
                self.sethash.entry(item.to_string()).and_modify(|counter| *counter += 1).or_insert(1);
                let buffercriterion = self.sethash.get(item).unwrap();
                if *buffercriterion > self.criterion {
                    self.criterion = *buffercriterion;
                }
            },
            Logic::OR => {
                self.sethash.entry(item.to_string()).or_insert(1);
            }
        }
        self
    }

    pub fn addv(&mut self, items: &Vec<String>) -> &mut LogicalHash {
        match self.logic {
            Logic::AND => {
                for found in items {
                    self.sethash.entry(found.to_string()).and_modify(|counter| *counter += 1).or_insert(1);
                    let buffercriterion = self.sethash.get(found).unwrap();
                    if *buffercriterion > self.criterion {
                        self.criterion = *buffercriterion;
                    }
                }
            },
            Logic::OR => {
                for found in items {
                    self.sethash.entry(found.to_string()).or_insert(1);
                }
            }
        }
        self
    }

    pub fn express(&self) -> Option<Vec<String>> {
        let mut retvec: Vec<String> = vec![];
        for (k,v) in &self.sethash {
            if *v >= self.criterion {
                retvec.push(k.to_string());
            }
        }
        if retvec.is_empty() {
            None
        } else {
            Some(retvec)
        }
    }

}


/*---------------------------------------------------------------------*/


#[derive(Default)]
pub struct GrepCrawler {

    crawler: FileCrawler<PathBuf>,
    keywords: Option<Vec<String>>,
    suffixes: Option<Vec<String>>,
    repos: Option<Repository>,
    logic: Logic,
    casesens: bool,
    followlink: bool,
    results: Option<Vec<String>>

}


impl GrepCrawler {

    pub fn new() -> GrepCrawler {
        GrepCrawler {
            crawler: FileCrawler::<PathBuf>::new(),
            keywords: None,
            suffixes: None,
            repos: None,
            logic: Logic::OR,
            casesens: false,
            followlink: false,
            results: None
        }
    }

    pub fn set_options(&mut self, optsenum: CrawlOption) -> &mut GrepCrawler {
        match optsenum {
            CrawlOption::CaseSensitive(ref b) => { self.casesens = *b; },
            CrawlOption::FollowLinks(ref b) => { self.followlink = *b; },
            CrawlOption::Repository(ref r) => { self.repos = Some(r.clone()); },
            _ => {}
        }
        _ = self.crawler.options(optsenum);
        self
    }

    pub fn set_search_terms(&mut self, keywordtup: (Logic, Vec<String>), suffixes: Vec<String>) -> &mut GrepCrawler {
        self.logic = match keywordtup.0 {
            Logic::AND => Logic::AND,
            Logic::OR => Logic::OR
        };
        self.keywords = Some(keywordtup.1);
        self.suffixes = Some(suffixes);
        self
    }


    pub fn crawl(&mut self) -> Result<&Self, MBError> {
        let Some(ref searchterms) = self.keywords else {
            return Err(MBError::SearchError("keywords for search unspecified".to_string()));
        };
        let Some(ref filesuffixes) = self.suffixes else {
            return Err(MBError::SearchError("search improperly initialized".to_string()));
        };
        if self.repos.is_none() {
            return Err(MBError::SearchError("directories for search unspecified".to_string()));
        }
        let mut filefilter: HashMap<PathBuf, bool> = HashMap::new();
        let mut suffixfilter: SuffixHash = SuffixHash::new();
        let _ = suffixfilter.addv(filesuffixes);
        let filesvec: Vec<PathBuf> = match self.crawler.crawl(
            &mut |filecanon: PathBuf| {
                if suffixfilter.test(&filecanon.to_str().unwrap().to_string()) && !filefilter.contains_key(&filecanon) {
                    filefilter.insert(filecanon.to_path_buf(), true);
                    Ok(filecanon.to_path_buf())
                } else {
                    Err(MBError::SearchError("Bad file".to_string()))
                }
            }
        ) {
            Ok(c) => { 
                match c.retrieve() {
                    Some(v) => v,
                    None => {
                        self.results = None;
                        return Ok(self);
                    }
                }
            },
            Err(_) => return Err(MBError::Grep("file crawl error".to_string()))
        };
        let mut setofresults: LogicalHash = LogicalHash::new(self.logic);
        for term in searchterms {
			let mut grepcmd = Command::new("grep");
            if !self.casesens {
    			grepcmd.arg("-i");
            }
            grepcmd.arg("-l").arg(term);
			for file in filesvec.iter() {
				grepcmd.arg(file.as_path().to_str().unwrap());
			}
			let grepresult = match grepcmd.output() {
                Ok(x) => x,
                Err(x) => return Err(MBError::Grep(format!("Grep error: {x}")))
            };
            let resultsformark = match String::from_utf8(grepresult.stdout) {
				Ok(strresult) => {
                    strresult.split('\n')
					.filter(|s| !s.is_empty())
					.map(|y| y.to_string())
					.collect()
                },
				Err(z) => { return Err(MBError::Grep(format!("Grep FromUTF8Error: {z}"))); }
			};
            setofresults.addv(&resultsformark);
		}
        self.results = setofresults.express();
        Ok(self)
    }
    
    
    pub fn retrieve(&self) -> Option<Vec<String>> {
        self.results.clone()
    }

}

} // pub mod grep_crawler



/*
#[cfg(test)]
mod tests {

}*/
 
