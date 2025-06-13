//  dbgenerator.rs
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


use crate::emptygenerator::EmptyGenerator;
use std::path::{Path};//,PathBuf};
use crate::mberror::MBError;
use std::fs::File;


#[derive(Default)]
pub struct DBGenerator;


impl DBGenerator {

    pub fn new() -> DBGenerator {
        DBGenerator {}
    }

    fn increment(&self, counter: usize) -> Option<String> {
        let numberstr: String = counter.to_string();
        match numberstr.len() {
            x if x >= 4 => None,
            3 => Some(numberstr),
            2 => Some(format!("0{}", numberstr)),
            1 => Some(format!("00{}", numberstr)),
            _ => None
        }
    }

}


impl EmptyGenerator for DBGenerator {

    fn generate(&self, basename: &str, suffs: Vec<&str>) -> Result<Vec<String>, MBError> {
        let mut bufferstr: String = format!("{basename}.{}", suffs[0]);
        let mut counter: usize = 0;
        loop {
            {
                match File::create_new(Path::new(&bufferstr)) {
                    Ok(_) => { break; },
                    Err(ref e) if e.kind() == std::io::ErrorKind::AlreadyExists => {},
                    Err(e) => { return Err(MBError::FileNewError(format!("{e}"))); }
                }
            }
            let post = match self.increment(counter) {
                Some(c) => c,
                None => return Err(MBError::FileOverPop("cannot create database due to filename conflicts".to_string()))
            };
            bufferstr = format!("{basename}{post}.{}", suffs[0]);
            counter += 1;
        }
        Ok(vec![bufferstr])
    }

}
