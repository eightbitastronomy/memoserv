//  exportlogger.rs
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


use json::object;
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use rusqlite::Connection;
use crate::mberror::MBError;
use crate::liteexportquery::LiteExportQuery;
use crate::rem_dupes;
use sha256::digest;


pub struct ExportLogger {
    log: String,
    table: String,
    toc: Vec<String>
}


impl ExportLogger {


    pub fn new(lognm: &str, tablenm: &str) -> ExportLogger {
        ExportLogger {
            log: lognm.to_string(),
            table: tablenm.to_string(),
            toc: Vec::new()
        }
    }


    pub fn set_log(&mut self, lognm: &str) {
        self.log = lognm.to_string();
    }


    pub fn prepare(&mut self, conn: &Connection) -> Result<&mut ExportLogger, MBError> {
        let mut toc: Vec<String> = Vec::new();
        let commandformer = LiteExportQuery::new(&self.table);
        let mut state = match conn.prepare(commandformer.form_toc().as_str()) {
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
            toc.push(fromrow);
        }
        self.toc = toc;
        Ok(self)
    }

    fn gather(&self, conn: &Connection, cmd: &str) -> Option<Vec<String>> {
        let mut state = match conn.prepare(cmd) {
            Ok(x) => x,
            Err(_) => { return None; }
        };
        let mut rows = match state.query([]) {
            Ok(x) => x,
            Err(_) => { return None; }
        };
        let mut retvec: Vec<String> = Vec::new();
        loop {
            let row = match rows.next() {
                Ok(x) => match x {
                    Some(y) => y,
                    None => break
                },
                Err(_) => { return None; }
            };
            let fromrow = match row.get(0) {
                Ok(x) => x,
                Err(_) => { return None; }
            };
            retvec.push(fromrow);
        }
        Some(retvec)
    }


    fn checksum(&self, filenm: &str) -> Option<String> {
        let filecanon = match PathBuf::from(filenm).as_path().canonicalize() {
            Ok(f) => f,
            Err(_) => { return None; }
        };
        let inputdata = match fs::read(filecanon.as_path()) {
            Ok(f) => f,
            Err(_) => { return None; }
        };
        Some(digest(inputdata))
    }


    pub fn dump(&self, conn: &Connection) -> Result<(), MBError> {
        let mut recordhash: HashMap<String, json::JsonValue> = HashMap::new();
        for item in self.toc.iter() {
            let commandformer = LiteExportQuery::new(&self.table);
            let marks: Vec<String> = match self.gather(conn, commandformer.form_mark_query(item).as_str()) {
                Some(m) => rem_dupes!(m.iter().filter(|x| !x.is_empty()).collect::<Vec<&String>>()),
                None => { continue; }
            };
            let types: Vec<String> = match self.gather(conn, commandformer.form_type_query(item).as_str()) {
                Some(m) => rem_dupes!(m.iter().filter(|x| !x.is_empty()).collect::<Vec<&String>>()),
                None => { continue; }
            };
            let sum: String = match self.checksum(item) {
                Some(s) => s,
                None => { continue; }
            };
            let record = object!{
                source: item.to_string(),
                mark: marks,
                type: types
            };
            recordhash.insert(sum, record);
        }
        if recordhash.is_empty() {
            return Ok(());
        }
        let recordjson: json::JsonValue = json::JsonValue::from(recordhash);
        match fs::write(&self.log, json::stringify_pretty(recordjson,4)) {
            Ok(_) => Ok(()),
            Err(x) => Err(MBError::FileSys(format!("cannot write export log: {x}")))
        }
    }

}
