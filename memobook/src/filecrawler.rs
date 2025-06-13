//  filecrawler.rs
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


use crate::crawler::{Crawler, CrawlOption};
use std::fs;
use crate::mberror::MBError;
use crate::repository::Repository;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::fs::Metadata;


#[derive(Default)]
pub struct FileCrawler<P>
where P: Clone
{
    case_sens: bool,
    follow_link: bool,
    sources: Option<Repository>,
    results: Option<Vec<P>>
}


impl<P> FileCrawler<P> 
where P: Clone
{

    pub fn new() -> FileCrawler<P> {
        FileCrawler {
            case_sens: false,
            follow_link: false,
            sources: None,
            results: None
        }
    }

}


fn recurse_dirs_files<P: Clone>(path: impl AsRef<Path>,
                    process: &mut impl FnMut(PathBuf) -> Result<P, MBError>,
                    direxclude: &mut HashMap<PathBuf,bool>, 
                    followlinks: bool) -> Option<Vec<P>> {
    let Ok(entries) = fs::read_dir(path) else { 
        return None; 
    };
    let mut retvec: Vec<P> = Vec::new();
    for direntry in entries.flatten() {
        let meta: Metadata = match direntry.metadata() {
            Ok(x) => x,
			Err(_) => { return None; }
    	};
        let filepathbuf: PathBuf = 
            if meta.is_symlink() {
                if !followlinks {
                    continue;
                } else {
                    match direntry.path().read_link() {
                        Ok(f) => f,
                        Err(_) => { continue; }
                    }
                }
            } else {
                direntry.path()
            };
        if filepathbuf.as_path().is_file() {
            let filecanon = filepathbuf.as_path().canonicalize().unwrap();
            //discarding errors here
            match process(filecanon) {
                Ok(f) => retvec.push(f),
                Err(_) => continue
            }
            continue;
    	}
        if filepathbuf.as_path().is_dir() {
            let filecanon = filepathbuf.as_path().canonicalize().unwrap();
		    if direxclude.get(&filecanon).is_none() {
			    direxclude.insert(filecanon.to_path_buf(), true);
    			let Some(mut resultvec) = recurse_dirs_files(
                        filecanon,
                        process, 
                        direxclude, 
                        followlinks) else { 
                    continue; 
                };
	        	if !resultvec.is_empty() {
		    		retvec.append(&mut resultvec);
			   	}
    		}
	    }
	}
	Some(retvec)
}


impl<P> Crawler<P> for FileCrawler<P> 
where P: Clone
{

    fn options(&mut self, optsenum: CrawlOption) -> &mut FileCrawler<P> {
        match optsenum {
            CrawlOption::CaseSensitive(b) => { self.case_sens = b; },
            CrawlOption::FollowLinks(b) => { self.follow_link = b; },
            CrawlOption::Repository(r) => { self.sources = Some(r); },
            _ => {}
        }
        self
    }


    fn crawl(&mut self, process: &mut impl FnMut(PathBuf) -> Result<P, MBError>) -> Result<&mut FileCrawler<P>, MBError> {
        let mut dirfilter: HashMap<PathBuf, bool> = HashMap::new();
    	let mut additions: Vec<P> = Vec::new();
        let Some(ref repo) = self.sources else {
            return Err(MBError::SearchError("crawl directories unspecified".to_string()));
        };
	    for exclusion in repo.iter_exclude() {
            match exclusion.as_path().canonicalize() {
                Ok(exc) => { dirfilter.insert(exc, true); },
                Err(_) => { continue; }
            }
        }
    	for startingdir in repo.iter_include() {
            match startingdir.as_path().canonicalize() {
  		        Ok(startdir) => { 
                    dirfilter.insert(startdir, true);
            		let Some(mut resultvec) = recurse_dirs_files(
                            startingdir.as_path(), 
                            process,
                            &mut dirfilter, 
                            self.follow_link) else { 
                        continue; 
                    };
		            additions.append(&mut resultvec);
                },
                Err(_) => {
                    continue;
                }
            }
	    }
        self.results = Some(additions);
        Ok(self)
    }


    fn retrieve(&self) -> Option<Vec<P>> {
        self.results.clone()
    }


}
