//  repository.rs
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


use std::path::PathBuf;


#[derive(Clone,Default)]
pub struct Repository {

    trunk: Option<PathBuf>,
    include: Vec<PathBuf>,
    exclude: Vec<PathBuf>

}


pub struct RepoIterator<'a> {

    vals: &'a Vec<PathBuf>,
    index: usize,
    len: usize   

}


impl<'a,'b> Repository {

    pub fn new() -> Repository {
        Repository {
            trunk: None,
            include: Vec::new(),
            exclude: Vec::new()
        }
    }


    pub fn set_trunk(&mut self, path: &'a str) {
		self.trunk = Some(PathBuf::from(strip_follower(path)));
    }


    pub fn add_include(&mut self, path: &'a str) {
        if let Some(trunkref) = self.trunk.as_ref() {
			self.include.push(PathBuf::from(format!("{}/{}", trunkref.to_str().unwrap(), strip_leader(path)).as_str()));
        } else {
			self.include.push(PathBuf::from(path));
        }
    }

    pub fn add_include_v(&mut self, pathv: Vec<String>) {
        if let Some(trunkref) = self.trunk.as_ref() {
            for item in pathv {
				self.include.push(PathBuf::from(format!("{}/{}", trunkref.to_str().unwrap(), strip_leader(&item)).as_str()));
            }
        } else {
            for item in pathv {
				self.include.push(PathBuf::from(&item));
            }
        }
    }

    pub fn add_include_paths(&mut self, mut paths: Vec<PathBuf>) {
        self.include.append(&mut paths);
    }

    pub fn add_exclude(&mut self, path: &'a str) {
        if let Some(trunkref) = self.trunk.as_ref() {
			self.exclude.push(PathBuf::from(format!("{}/{}", trunkref.to_str().unwrap(), strip_leader(path)).as_str()));
        } else {
			self.exclude.push(PathBuf::from(path));
        }
    }

    pub fn add_exclude_v(&mut self, pathv: Vec<String>) {
        if let Some(trunkref) = self.trunk.as_ref() {
            for item in pathv {
				self.exclude.push(PathBuf::from(format!("{}/{}", trunkref.to_str().unwrap(), strip_leader(&item)).as_str()));
            }
        } else {
            for item in pathv {
				self.exclude.push(PathBuf::from(&item));
            }
        }
    }

    pub fn add_exclude_paths(&mut self, mut paths: Vec<PathBuf>) {
        self.exclude.append(&mut paths);
    }

    pub fn get_trunk(&'b self) -> &'b str {
        if self.trunk.is_none() {
            ""
        } else {
            self.trunk.as_ref()
                .unwrap()
                .as_path()
                .to_str()
                .unwrap()
        }
    }

    pub fn get_include(&'b self) -> &'b Vec<PathBuf> {
        &self.include
    }

    pub fn get_exclude(&'b self) -> &'b Vec<PathBuf> {
        &self.exclude
    }

    pub fn iter_include(&'b self) -> RepoIterator<'b> {
        RepoIterator { 
            vals: &self.include,
            index: 0,
            len: self.include.len()
        }
    }

    pub fn iter_exclude(&'b self) -> RepoIterator<'b> {
        RepoIterator { 
            vals: &self.exclude,
            index: 0,
            len: self.exclude.len()
        }
    }

}


impl<'a> Iterator for RepoIterator<'a> {

    type Item = &'a PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = if self.index < self.len {
            Some(&self.vals[self.index])
        } else {
            None
        };
        self.index += 1;
        ret
    }
}


fn strip_leader(input: &str) -> &str {
	/*if input.is_empty() {
		return input;
	}
	let start: char = input.chars().next().unwrap();
	if (start == '/') || (start == '\\') {
        &input[1..]
    } else {
        input
    }*/
    if let Some(start) = input.chars().next() {
        if (start == '/') || (start == '\\') {
            &input[1..]
        } else {
            input
        }   
    } else {
        input
    }
}


fn strip_follower(input: &str) -> &str {
    if input.is_empty() {
        return input;
    }
    let end: i32 = input.len() as i32 - 1;
	if end < 1 {
		return "";
	}
	let endchar: char = input.chars().nth(end as usize).unwrap();
	//println!("Analysis of end chars: {} -> {}, becomes {}", input, endchar, &input[0..(end as usize)]);
	if (endchar == '/') || (endchar == '\\') {
        &input[0..(end as usize)]
    } else {
        input
    }
}
