//  dbhexgenerator.rs
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
use std::path::{Path};
use crate::mberror::MBError;
use std::fs;
use std::fs::File;


pub struct DBHexGenerator;

impl DBHexGenerator {

    pub fn new() -> DBHexGenerator {
        DBHexGenerator {}
    }

    fn increment(&self, counter: usize) -> Option<String> {
        let numberstr: String = format!("{counter:X}");
        match numberstr.len() {
            x if x > 4 => None,
            4 => Some(numberstr),
            3 => Some(format!("0{}",numberstr)),
            2 => Some(format!("00{}", numberstr)),
            1 => Some(format!("000{}", numberstr)),
            _ => None
        }
    }

}


impl EmptyGenerator for DBHexGenerator {
    
    
    fn generate(&self, basename: &str, suffs: Vec<&str>) -> Result<Vec<String>, MBError> {
        let mut basebuffer: String = format!("{basename}0000");
        let mut counter: usize = 1;
        let sufflen: usize = suffs.len();
        let mut retvec: Vec<String> = Vec::new();
        'cloop: loop {
            let mut tempnamevec: Vec<String> = Vec::new();
            {
                for suffix in suffs.iter() {
                    let bufferstr: String = format!("{basebuffer}.{suffix}");
                    match File::create_new(Path::new(&bufferstr)) {
                        Ok(_) => { tempnamevec.push(bufferstr); },
                        Err(ref e) if e.kind() == std::io::ErrorKind::AlreadyExists => {},
                        Err(e) => { return Err(MBError::FileNewError(format!("{e}"))); }
                    }
                }
            }
            match tempnamevec.len() {
                0 => {
                },
                x if x == sufflen => {
                    retvec.append(&mut tempnamevec);
                    break 'cloop;
                },
                _ => {
                    for name in tempnamevec.iter() {
                        match fs::remove_file(name) {
                            Ok(_) => {},
                            Err(e) => { return Err(MBError::FileRemError(format!("could not delete temporary file name: {e}"))); }
                        };
                    }
                }
            }
            let post = match self.increment(counter) {
                Some(c) => c,
                None => return Err(MBError::FileOverPop("cannot create database due to filename conflicts".to_string()))
            };
            basebuffer = format!("{basename}{post}");
            counter += 1;
        }
        Ok(retvec)
    }
    

}
