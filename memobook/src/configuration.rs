//  configuration.rs
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



/*******************************************************
*  configuration reader and holder
********************************************************/


/***********************
  Notes:
    TrunkType, check_compatibility are remnants from when
    I was going to have modify_repo_by_repo try to maintain
    the presence of (or lack thereof) a trunk in the config's
    repo.
***********************/


use std::path::{Path,PathBuf};
use std::env;
use crate::emptygenerator::EmptyGenerator;
use crate::dbgenerator::DBGenerator;
use crate::repository::Repository;
use crate::backer::{Backer, TransBackStruct};
use crate::backerparserjson::BackerParserJSON;
use std::fs;
use json::object;
use std::collections::HashMap as HashMap;
use crate::mimer::Mimer as Mimer;



#[inline]
fn strip_prefix_dot(src: &str) -> &str {
    match src.strip_prefix(".") {
        Some(s) => s,
        None => src
    }
}



#[derive(Clone)]
pub struct MBInfo
{
    pub src: String,      //path of db file
    pub table: String,    //name of table in db
    pub scan: Repository, //container for search directories
    pub alt: bool        //flag for: needs backed up
}



pub struct Configuration<M>
where M: Backer + BackerParserJSON + std::marker::Send
{
    path: String,
    mb: MBInfo,
    back: Option<M>,           //the backups object
    mime: HashMap<String,Mimer>,
    holdover: Vec<(String,json::JsonValue)>,
    changed: bool
}


/*enum TrunkType {
    TRUNKTRIM,
    TRUNKREADY,
    CONVERT
}*/


impl<M> Configuration<M> 
where 
    M: Backer + BackerParserJSON + std::marker::Send
{

    fn new(path: String,
            mb: MBInfo,
            back: Option<M>, 
            mime: HashMap<String,Mimer>, 
            holdover: Vec<(String, json::JsonValue)>, 
            changed: bool) -> Configuration<M> {
        Configuration {
            path, mb, back, mime, holdover, changed
        }
    }
    
    /* Future: I would like replaceable parts here. Not sure how to implement.
	    Option 1: input should be the path as well as the configuration item.
	The configuration item needs a collection referencing all its configurable parts,
	so that this vector can be iterated. By iterating, each part is has its own method
	for reading in an appropriate portion of the json. E.g., the database portion would
	start reading, then would hand off the json to the Repository, etc., then build itself,
	set itself in the Configuration, then the next portion would take over.
	    Option 2: different structs are passed into the read_configuration, each impl a trait
	corresponding to reading the json in, and the read_configuration would assemble the
	configuration object.
    */

    pub fn read(path: &str, mut back: Option<M>) -> Result<Configuration<M>, String> {

        //Note, json lib handles missing pieces quietly. If I try to load/convert 
        //jsonraw["database"]["scan"] it apparently makes a placeholder and doesn't
        //complain. Hence, I must check for empty strings, etc., elsewhere.

        let mut rep: Repository;
        let mut mimemap: HashMap<String, Mimer>;
        let mut processed: HashMap<&str,bool> = HashMap::new();

        // Read the json file in its entirety
        let fdata: Vec<u8> = match fs::read(path) {
            Ok(x) => x,
            Err(_) => { return Err("Unable to read file".to_string()); }
        };

        // Parse the input into json
        let rawjson: json::JsonValue;
        let rawjsonresult = json::parse(String::from_utf8(fdata).unwrap().as_str());
        if rawjsonresult.is_err() {
            return Err("Unable to parse input".to_string());
        } else {
            rawjson = rawjsonresult.unwrap();
        }

        // Prepare the repository directories
        rep = Repository::new();
        rep.set_trunk( match &rawjson["database"]["scan"]["trunk"] {
                json::JsonValue::String(x) => x,
                json::JsonValue::Short(x) => x,
                _ => ""
            });
        if rawjson["database"]["scan"]["include"].is_array() {
            if let json::JsonValue::Array(x) = &rawjson["database"]["scan"]["include"] {
                let incvec: Vec<String> = x.iter().map(
                    |y| match y {
                        json::JsonValue::String(z) => z.to_owned(),
                        json::JsonValue::Short(z) => z.to_string(),
                        _ => "".to_string()
                    }
                    ).collect();
                rep.add_include_v(incvec);
            }
        }
        if rawjson["database"]["scan"]["exclude"].is_array() {
            if let json::JsonValue::Array(x) = &rawjson["database"]["scan"]["exclude"] {
                let exvec: Vec<String> = x.iter().map(
                    |y| match y {
                        json::JsonValue::String(z) => z.to_owned(),
                        json::JsonValue::Short(z) => z.to_string(),
                        _ => "".to_string()
                    }
                    ).collect();
                rep.add_exclude_v(exvec);
            }
        }

        // Prepare the "backup" object 
        if let Some(backup) = back.as_mut() {
            backup.read(&rawjson)?;
        }

        // Prepare the "memobook" info
        let membook = MBInfo { 
            src: match &rawjson["database"]["src"] {
                json::JsonValue::String(x) => x.to_owned(),
                json::JsonValue::Short(x) => x.to_string(),
                _ => "".to_string()
            }, 
            table: match &rawjson["database"]["table"] {
                json::JsonValue::String(x) => x.to_owned(),
                json::JsonValue::Short(x) => x.to_string(),
                _ => "".to_string()
            },
            scan: rep,
            alt: match &rawjson["database"]["alt"] {
                json::JsonValue::Boolean(x) => *x,
                _ => true
            }
        };
        processed.insert("database", true);

        // Read in the mime types
        mimemap = HashMap::new();
        if !rawjson["mime"].is_null() && rawjson["mime"].is_array() && !rawjson["mime"].is_empty() {
            if let json::JsonValue::Array(types) = &rawjson["mime"] {
                for vecitem in types {
                    match vecitem {
                        json::JsonValue::Array(x) => {
                            let bufvec: Vec<String> = x.iter().map(|y| match y {
                                    json::JsonValue::String(z) => strip_prefix_dot(z).to_owned(),
                                    json::JsonValue::Short(z) => strip_prefix_dot(z).to_string(),
                                    _ => "".to_string()
                                }).collect();
                            let mm = Mimer::new_by_slice(&bufvec[1..]);
                            mimemap.insert(bufvec[0].to_string(), mm);
                        },
                        _ => { break; }
                    };
                }
            }
        }
        processed.insert("mime", true);

        // Collect and store unused json objects for later output
       let holds: Vec<(String, json::JsonValue)> = rawjson.entries()
            .filter_map(|(nm, jv)| if !processed.contains_key(nm) { Some((nm.to_string(), jv.to_owned())) } else { None })
            .collect();

        // Return the configuration object
        Ok(Configuration::new(path.to_string(), membook, back, mimemap, holds, false))

    }


    pub fn assemble_repo_info(&self) -> String {
        let includev = self.mb.scan.get_include();
        let bufferinclude: Vec<&str> = if includev.is_empty() {
            vec![]
        } else {
            includev.iter()
                .map(|x: &PathBuf| x.as_path()
                                .to_str()
                                .unwrap())
                .collect::<Vec<&str>>()
                .to_vec()
        };
        let excludev = self.mb.scan.get_exclude();
        let bufferexclude: Vec<&str> = if excludev.is_empty() {
            vec![]
        } else {
            excludev.iter()
                .map(|x: &PathBuf| x.as_path()
                                .to_str()
                                .unwrap())
                .collect::<Vec<&str>>()
                .to_vec()
        };
        let jrepo = object!{
            include: bufferinclude,
            exclude: bufferexclude
        };
        match jrepo {
            json::JsonValue::Object(o) => json::stringify(o),
            _ => "".to_string()
        }
    }


    pub fn assemble_backup_info(&self) -> String {
        let jback = match &self.back {
            Some(bu) => match bu.write() {
                Ok(j) => j,
                Err(_) => json::JsonValue::Null
            },
            None => json::JsonValue::Null
        };
        match jback {
            json::JsonValue::Object(o) => json::stringify(o),
            _ => "".to_string()
        }
    }


    pub fn do_backup(&mut self) -> Result<String, String> {
        let Some(bu) = self.back.as_mut() else {
            return Ok("".to_string());
        };
        match bu.make(&self.mb.src, &[self.path.as_str()]) {
            Ok(_) => { 
                self.changed = true;
                self.mb.alt = false;
                Ok("".to_string())
            },
            Err(e) => { 
                Err(format!("Backup error: {e}"))
            }
        }
    }


    fn load_backup(&mut self, loadfile: &str) -> Result<String, String> {
        if loadfile.is_empty() {
            return Ok("".to_string());
        }
        let Some(bu) = &self.back else {
            return Ok("".to_string());
        };
        let _buobject = match bu.get(loadfile) {
            Some(ret) => ret,
            None => { 
                return Ok("".to_string()); 
            }
        };
        let loadpath = Path::new(loadfile);
        let name = match loadpath.file_name() {
            Some(n) => n.to_str().unwrap(),
            None => { 
                return Ok("".to_string()); 
            }
        };
        if self.mb.src.is_empty() {
            self.mb.src = format!("{}archive.db", loadfile.strip_suffix(name).or_else(|| Some("")).unwrap());
        }
        match fs::copy(loadfile, self.mb.src.as_str()) {
            Ok(_) => {
                self.mb.alt = false;
                return Ok(self.mb.src.clone());
            },
            Err(e) => {
                return Err(format!("Backup load error: {:?}", e));
            }
        }
    }


    pub fn process_modify_backup(&mut self, bupmod: &TransBackStruct) -> Result<Option<String>, String> {
        let Some(mut bu) = self.back.take() else {
            return Err("cannot modify backup: no backup information present".to_string());
        };
        // apply changes to freq, loc, base
        if let Some(frq) = bupmod.freq {
            bu.set_frequency(frq);
        }
        if let Some(basestr) = &bupmod.base {
            bu.set_base(basestr);
        }
        if let Some(locstr) = &bupmod.loc {
            bu.set_location(locstr);
        }
        // Get list of existing bu's
        let preexcopies: Vec<_> = bu.iter().map(|c| c.clone()).collect();
        // Hold on new mult, use max(old, new) + 1 temporarily
        let preexmult = bu.get_multiplicity();
        let newmult = match bupmod.mult {
            Some(m) => {
                bu.set_multiplicity(
                    if preexmult >= m {
                        preexmult + 1
                    } else {
                        m + 1
                    }
                );
                m 
            },
            None => {
                bu.set_multiplicity(preexmult+1);
                preexmult
            }
        };
        self.back = Some(bu); // because of take(), above
        // if force, make bu
        if bupmod.force {
            self.do_backup()?;
        }
        // if load, load bu (only the step prior makes a bu, not this one)
        let retstring: String = if let Some(loadstr) = &bupmod.load { 
            match self.load_backup(loadstr) {
                Ok(s) => s,
                Err(e) => return Err(e)
            }
        } else {
            self.mb.src.clone()
        };
        let Some(mut bu) = self.back.take() else {
            return Err("cannot modify backup: no backup information present".to_string());
        };
        // if clear, remove all items in the list of old bu's
        if bupmod.remove {
            for oldcopy in preexcopies.iter() {
                bu.remove_and_erase(Some(oldcopy));
            }
        }
        // set the new mult
        bu.set_multiplicity(newmult);
        self.back = Some(bu); // because of take(), above
        // it is not guaranteed this will be set by previous lines
        self.changed = true;
        // by passing back a load path or not, we tell the caller whether to
        //   reconnect the database
        if retstring.is_empty() {
            Ok(None)
        } else {
            Ok(Some(retstring.to_string()))
        }
    }


    pub fn check_backup(&mut self, auto: bool) {
        let Some(bu) = self.back.as_mut() else {
            return;
        };
        if bu.check() && (auto || self.mb.alt || self.changed) {
            match bu.make(&self.mb.src, &[self.path.as_str()]) {
                Ok(_) => { 
                    self.changed = true;
                    self.mb.alt = false;
                },
                Err(e) => { 
                    println!("Backup error: {e}"); 
                }
            }
        }
    }

    
    pub fn check_for_initialization(&mut self) -> Result<(), String> {
        if self.mb.src.is_empty() {
            let dbfilegen: DBGenerator = DBGenerator::new();
            self.mb.src = match dbfilegen.generate("archive",vec!["db"]) {
                Ok(f) => f[0].clone(),
                Err(e) => { return Err(format!("db creation failed: {e}")); }
            };
            self.changed = true;
        }
        if self.mb.table.is_empty() {
            self.mb.table = "bookmarks".to_string();
            self.changed = true;
        }
        if self.mb.scan.iter_include().next().is_none() {
            self.mb.scan.add_include_paths( match env::current_dir() {
                    Ok(p) => vec![p],
                    Err(e) => { return Err(format!("error initializing scan directory: {e}")); }
                }
            );
            self.changed = true;
        }  
        Ok(())
    }


    pub fn path(&self) -> &str {
        &self.path
    }


    pub fn mb(&self) -> &MBInfo {
        &self.mb
    }


    pub fn mb_alt(&mut self, val: bool) {
        self.mb.alt = val;
    }


    pub fn mime(&self) -> &HashMap<String,Mimer> {
        &self.mime
    }


    pub fn set_source(&mut self, target: &str) {
        self.mb.src = target.to_string();
        self.changed = true;
    }


    pub fn set_repo_by_string(&mut self, include: Vec<String>, exclude: Vec<String>, trunk: Option<&str>) {
        self.mb.scan = Repository::new();
        if let Some(target) = trunk {
            self.mb.scan.set_trunk(target);
        }
        self.mb.scan.add_include_v(include);
        self.mb.scan.add_exclude_v(exclude); 
        self.changed = true;
    }


    pub fn set_repo_by_repo(&mut self, repo: Repository) {
        self.mb.scan = repo;
        self.changed = true;
    }

    /*
    fn trunk_compatibility(&self, trunk: &str, repo: &Repository) -> TrunkType {
        let incomingtrunk = repo.get_trunk();
        let trunk = self.mb.scan.get_trunk();
        if incomingtrunk == "" {
            'includes: for pathitem in repo.iter_include() {
                let matchv: Vec<_> = pathitem.to_str().unwrap().match_indices(trunk).collect();
                if matchv.is_empty() {
                    return TrunkType::CONVERT;
                }
                for tup in matchv {
                    if tup.0 == 0 {
                        continue 'includes; // trunk found at start of path
                    }
                }
                return TrunkType::CONVERT; // if this is reached, trunk was found but not at start of path
            }
            'exludes: for pathitem in repo.iter_exclude() {
                let matchv: Vec<_> = pathitem.to_str().unwrap().match_indices(trunk).collect();
                if matchv.is_empty() {
                    return TrunkType::CONVERT;
                }
                for tup in matchv {
                    if tup.0 == 0 {
                        continue 'excludes;
                    }
                }
                return TrunkType::CONVERT;
            }
            return TrunkType::TRUNKTRIM;
        } else {
            if incomingtrunk == trunk {
                return TrunkType::TRUNKREADY; // case for repo having the same trunk
            } else {
                return TrunkType::CONVERT; // repo has a trunk but it's different
            }
        }
    }*/

    pub fn modify_repo_by_repo(&mut self, mods: (Repository, Repository)) {
        // first repo contains removals, 2nd contains additions.
        // So 1st.includes is all the removals from self.includes.
        // Defined behavior: trunk is dropped if this method is used.        
        /* Following is not implemented for the time being...
        let trunk = self.mb.scan.get_trunk();
        let bufferadds: Repository;
        let bufferrems: Repository;
        if trunk == "" {
            process_repo_no_trunk(mods);
        } else {
            bufferrems = match trunk_compatibility(trunk, &mods.0) {
                TrunkType::TRUNKTRIM => {},
                TrunkType::TRUNKREADY => {}
            }
        }*/
        let mut includehash: HashMap<PathBuf,bool> = HashMap::new();
        let mut excludehash: HashMap<PathBuf,bool> = HashMap::new();
        for incitem in self.mb.scan.iter_include() {
            includehash.insert(incitem.to_path_buf(), true);
        }
        for excitem in self.mb.scan.iter_exclude() {
            excludehash.insert(excitem.to_path_buf(), true);
        }
        for remitem in mods.0.iter_include() {
            _ = includehash.remove(remitem.as_path());
        }
        for additem in mods.1.iter_include() {
            includehash.insert(additem.to_path_buf(), true);
        }
        for remitem in mods.0.iter_exclude() {
            _ = excludehash.remove(remitem.as_path());
        }
        for additem in mods.1.iter_exclude() {
            excludehash.insert(additem.to_path_buf(), true);
        }
        let mut newrepo: Repository = Repository::new();
        newrepo.add_include_paths(
            includehash.drain().map(|(key,_)| key).collect()
        );
        newrepo.add_exclude_paths(
            excludehash.drain().map(|(key,_)| key).collect()
        );
        self.mb.scan = newrepo;
        self.changed = true;
    }


    pub fn finish(&self) {
        if self.changed {
            // Piece together the database object inside out, beginning with the repo
            let includev = self.mb.scan.get_include();
            let bufferinclude: Vec<&str> = if includev.is_empty() {
                vec![]
            } else {
                includev.iter()
                    .map(|x: &PathBuf| x.as_path()
                                    .to_str()
                                    .unwrap()
                                    .strip_prefix(self.mb.scan.get_trunk())
                                    .unwrap())
                    .collect::<Vec<&str>>()
                    .to_vec()
            };
            let excludev = self.mb.scan.get_exclude();
            let bufferexclude: Vec<&str> = if excludev.is_empty() {
                vec![]
            } else {
                excludev.iter()
                    .map(|x: &PathBuf| x.as_path()
                                    .to_str()
                                    .unwrap()
                                    .strip_prefix(self.mb.scan.get_trunk())
                                    .unwrap())
                    .collect::<Vec<&str>>()
                    .to_vec()
            };
            let jrepo = object!{
                trunk: self.mb.scan.get_trunk(),
                include: bufferinclude,
                exclude: bufferexclude
            };
            let jback = match &self.back {
                Some(bu) => match bu.write() {
                    Ok(jret) => jret,
                    Err(_) => json::JsonValue::Null
                },
                None => json::JsonValue::Null
            };
            let jdatabase = object!{
                src: self.mb.src.as_str(),
                table: self.mb.table.as_str(),
                scan: jrepo,
                alt: self.mb.alt,
                back: jback
            };
            // Prep the mime object
            let mut jmime = json::JsonValue::new_array();
            for m in self.mime.keys() {
                let mut mimevec: Vec<&str> = vec![m];
                mimevec.append(&mut self.mime.get(m).unwrap().iter().collect::<Vec<&str>>().to_vec());
                let _ = jmime.push(mimevec); 
            }
            // Prep the root object
            let mut jroot = object!{
                database: jdatabase,
                mime: jmime
            };
            // Insert any holdovers into the root
            for (key,val) in self.holdover.iter() {
                let _ = jroot.insert(key.as_str(), val.clone());
            }
            // Take a stringy, pretty dump (O.o)
            match fs::write(self.path.clone(), json::stringify_pretty(jroot,4)) {
                Ok(_) => { },
                Err(_) => { println!("Error attempting to write configuration json file"); }
            };
        } 
    }


} //end impl Configuration
