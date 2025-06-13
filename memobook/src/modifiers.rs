//  modifiers.rs
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


#[derive(Clone)]
pub struct ModifyAddRecord {
    pub file: String,
    pub marks: Vec<String>,
    pub ftypes: Vec<String>
}


impl ModifyAddRecord {

    pub fn new(file: &str, marks: &[String], ftypes: &[String]) -> ModifyAddRecord {
        ModifyAddRecord {
            file: file.to_string(),
            marks: marks.to_vec(),
            ftypes: ftypes.to_vec()
        }
    }

}


/* replaces 1st with 2nd in tuple */
pub struct ModifyFieldReplace {
    pub field: String,
    pub repl: Vec<(String, String)>
}


impl ModifyFieldReplace {

    pub fn new(field: &str, repl: (&str, &str)) -> ModifyFieldReplace {
        ModifyFieldReplace {
            field: field.to_string(),
            repl: vec![(repl.0.to_string(), repl.1.to_string())]
        }
    }

    pub fn add(&mut self, pair: (&str, &str)) -> &mut ModifyFieldReplace {
        self.repl.push((pair.0.to_string(), pair.1.to_string()));
        self
    }

}


pub struct ModifyMarkUpdate {
    pub file: String,
    pub ftypes: Vec<String>,
    pub rem: Vec<String>,
    pub add: Vec<String>
}


impl ModifyMarkUpdate {

    pub fn new(file: &str, ftypes: &[String], rem: &[String], add: &[String]) -> ModifyMarkUpdate {
        ModifyMarkUpdate { 
            file: file.to_string(), 
            ftypes: ftypes.to_vec(),
            rem: rem.to_vec(), 
            add: add.to_vec() 
        }
    } 

}


pub struct ModifyTargetRemove {
    pub ttype: String,
    pub value: String
}


impl ModifyTargetRemove {
    
    pub fn new(ttype: &str, value: &str) -> ModifyTargetRemove {
        ModifyTargetRemove { 
            ttype: ttype.to_string(), 
            value: value.to_string()
        }
    }

}


pub enum Modifier {
    AddRecord(ModifyAddRecord),
    FieldReplace(ModifyFieldReplace),
    MarkUpdate(ModifyMarkUpdate),
    TargetRemove(ModifyTargetRemove)
}
