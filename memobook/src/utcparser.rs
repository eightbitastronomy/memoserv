//  utcparser.rs
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


use crate::backerparserjson::BackerParserJSON;
use crate::backer::{Backer,BuNumber,BackCopy};
use crate::utckeeper::UtcKeeper;
use crate::utcbackup::UtcBackup;
use json::object;
use chrono::DateTime;


pub struct UtcParser;



impl BackerParserJSON for UtcParser {


    type CopyItem = UtcBackup; 
    type Item = UtcKeeper;


    fn read(&self, source: &json::JsonValue) -> Result<Self::Item, String> {
        let vers: String = match &source["database"]["back"]["version"] {
            json::JsonValue::Short(x) => x.to_string(),
            json::JsonValue::String(z) => z.to_owned(),
            _ => { return Err("Parse error on backup object".to_string()); }
        };
        if vers != "json_utc_01_00" {
            return Err("Backup object version mismatch".to_string());
        }
        let freq: BuNumber = match &source["database"]["back"]["frequency"] {
            json::JsonValue::Number(n) => {
                match n.to_string().parse::<BuNumber>() {
                    Ok(o) => o,
                    Err(_) => { return Err("Parse error on frequency".to_string()); }
                }
            },
            json::JsonValue::Short(x) => { 
                match x.to_string().parse::<BuNumber>() {
                    Ok(val) => val,
                    Err(_) => { return Err("Parse error on backup object".to_string()); }
                }
            },
            json::JsonValue::String(z) => {
                match z.to_string().parse::<BuNumber>() {
                    Ok(val) => val,
                    Err(_) => { return Err("Parse error on backup object".to_string()); }
                }
            },
            _ => { return Err("Parse error on backup object".to_string()); }
        };
        let mult: BuNumber = match &source["database"]["back"]["multiplicity"] {
            json::JsonValue::Number(n) => {
                match n.to_string().parse::<BuNumber>() {
                    Ok(o) => o,
                    Err(_) => { return Err("Parse error on multiplicity".to_string()); }
                }
            },
            json::JsonValue::Short(x) => { 
                match x.to_string().parse::<BuNumber>() {
                    Ok(val) => val,
                    Err(_) => { return Err("Parse error on backup object".to_string()); }
                }
            },
            json::JsonValue::String(z) => {
                match z.to_string().parse::<BuNumber>() {
                    Ok(val) => val,
                    Err(_) => { return Err("Parse error on backup object".to_string()); }
                }
            },
            _ => { return Err("Parse error on backup object".to_string()); }
        };
        let base: &str = match &source["database"]["back"]["base"] {
            json::JsonValue::String(x) => x.as_str(),
            json::JsonValue::Short(x) => x.as_str(),
            _ => { return Err("Parse error on backup object".to_string()); }
        };
        let suffixvec: Vec<String> = match &source["database"]["back"]["suffix"] {
            json::JsonValue::Array(a) => {
                a.iter().map(
                    |y| match y {
                        json::JsonValue::String(z) => z.to_owned(),
                        json::JsonValue::Short(z) => z.to_string(),
                        _ => "".to_string()
                    }
                ).collect()
            },
            _ => {
                return Err("Parse error on backup object suffix array".to_string());
            }
        };
        if suffixvec.len() < 2 {
            return Err("Parse error on backup object suffix array".to_string());
        }
        let suffix: &str = suffixvec[0].as_str();
        let cfgsuffix: &str = suffixvec[1].as_str();
        let location: String = match &source["database"]["back"]["location"] {
            json::JsonValue::String(x) => x.to_owned(),
            json::JsonValue::Short(x) => x.to_string(),
            _ => { return Err("Parse error on backup object".to_string()); }
        };
        let mut copiesvec: Vec<UtcBackup> = Vec::new();
        if source["database"]["back"]["copies"].is_array() {
            if let json::JsonValue::Array(x) = &source["database"]["back"]["copies"] {
                let incvec: Vec<String> = x.iter().map(
                    |y| match y {
                        json::JsonValue::String(z) => z.to_owned(),
                        json::JsonValue::Short(z) => z.to_string(),
                        _ => "".to_string()
                    }
                    ).collect();
                for item in incvec.into_iter() {
                    let sitem = item.split(' ').collect::<Vec<_>>();
                    //println!("item is {:?}", sitem);
                    match sitem[..] {
                        [a,b,c] => { 
                            match DateTime::parse_from_rfc3339(c) {
                                Ok(dt) => copiesvec.push(
                                    UtcBackup::new( 
                                        a, 
                                        b, 
                                        &chrono::DateTime::from(dt)
                                    )
                                ),
                                Err(_) => { return Err("Parse error on datetime object".to_string()); }
                            }
                            
                        },
                        _ => { println!("item in incvec parsed {}", item); continue; }
                    }
                }
            }
        }
        let mut backup = UtcKeeper::new(freq, mult, copiesvec);
        backup.set_base(base);
        backup.set_suffix(&[suffix.to_string(), cfgsuffix.to_string()]);
        backup.set_location(&location);
        Ok(backup)
    }


    fn write(&self, source: &Self::Item) -> Result<json::JsonValue, String> {
        let jback = object!{
            version: source.version().to_string(),
            frequency: source.get_frequency(),
            multiplicity: source.get_multiplicity(),
            copies: source.iter().map(|x: &UtcBackup| 
                {
                    let tempvec = x.aux();
                    if tempvec.is_empty() {
                        None
                    } else {
                        Some(format!("{} {} {}", x.path(), tempvec[0], x.date().format("%+")))
                    }
                }).collect::<Vec<_>>(),
            base: source.get_base().to_string(),
            suffix: source.get_suffix(),
            location: source.get_location().to_string()
        };
        Ok(jback)
    }
}
