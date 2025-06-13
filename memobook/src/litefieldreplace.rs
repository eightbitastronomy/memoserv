//  litefieldreplace.rs
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


use crate::modifierassembler::ModifierAssembler;
use crate::modifiers::Modifier;
use crate::mberror::MBError;


pub struct LiteFieldReplace;


impl ModifierAssembler for LiteFieldReplace {

    fn form(&self, table: &str, mdfy: &Modifier) -> Result<Vec<String>, MBError> {
        match mdfy {
            Modifier::FieldReplace(fr) => {
                let mut resultvec: Vec<String> = Vec::new();
                for pair in fr.repl.iter() {
                    resultvec.push(format!("update {table} set {}=\'{}\' where {}=\'{}\';", fr.field.as_str(), pair.1, fr.field.as_str(), pair.0));
                }
                Ok(resultvec)
            },
            _ => Err(MBError::BadModify("incorrect modification type for modification assembler".to_string()))
        }
    }

}


#[cfg(test)]
mod tests {

    use super::*;
    use crate::modifiers::ModifyFieldReplace;

    #[test]
    fn test_replacement() {
        let mut container: ModifyFieldReplace = ModifyFieldReplace::new("mark",("python","PYTHON"));
        container.add(("rust","RUST"));
        let cmd = LiteFieldReplace;
        for (i, line) in cmd.form("bookmarks", &Modifier::FieldReplace(container)).unwrap().iter().enumerate() {
            match i {
                0 => assert_eq!(*line, "update bookmarks set mark=\'PYTHON\' where mark=\'python\';".to_string()),
                1 => assert_eq!(*line, "update bookmarks set mark=\'RUST\' where mark=\'rust\';".to_string()),
                _ => panic!("somehow reached i={} with {}", i, line)
            }
        }
    }

}
