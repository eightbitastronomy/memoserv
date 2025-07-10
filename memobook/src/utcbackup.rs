//  utcbackup.rs
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


use chrono::{DateTime,Utc};
use crate::backer::BackCopy;


#[derive(Clone)]
pub struct UtcBackup {

    dbpath: String,
    cfgpath: String,
    date: DateTime<Utc> 

}


impl UtcBackup {

    pub fn new(path: &str, cfg: &str, date: &DateTime<Utc>) -> UtcBackup {
        UtcBackup {
            dbpath: path.to_string(),
            cfgpath: cfg.to_string(),
            date: *date
        }
    }

}


impl BackCopy for UtcBackup {


    type D = DateTime<Utc>;

    fn date(&self) -> Self::D {
        self.date
    }


    fn set_date(&mut self, date: &Self::D) {
        self.date = *date;
    }


    fn path(&self) -> String {
        self.dbpath.clone()
    }


    fn set_path(&mut self, path: &str) {
        self.dbpath = path.to_string();
    }


    fn aux(&self) -> Vec<String> {
        vec![self.cfgpath.clone()]
    }


    fn set_aux(&mut self, aux: &[&str]) {
        if !aux.is_empty() {
            self.cfgpath = aux[0].to_string();
        }
    }


}
