//  liteaddrecord.rs
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


pub struct LiteAddRecord;


impl ModifierAssembler for LiteAddRecord {

    fn form(&self, table: &str, mdfy: &Modifier) -> Result<Vec<String>, MBError> {
        match mdfy {
            Modifier::AddRecord(ar) => {
                let mut resultvec: Vec<String> = Vec::new();
                for mark in ar.marks.iter() {
                    for typ in ar.ftypes.iter() {
                        resultvec.push(format!("insert into {table} (mark, file, type) values (\'{}\', \'{}\', \'{}\');", mark, ar.file, typ));
                    }
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
    use crate::modifiers::ModifyAddRecord;

    #[test]
    fn test_add() {
        let container: ModifyAddRecord = ModifyAddRecord::new(
            "linux_pros.txt", 
            &vec![
                "grub".to_string(), 
                "grep".to_string(), 
                "chroot".to_string(), 
                "sudo".to_string()
            ],
            //"PDF"
            &vec!["PDF".to_string()]
        );
        let cmd = LiteAddRecord;
        let test1: String = "insert into bookmarks (mark, file, type) values (\'grub\', \'linux_pros.txt\', \'PDF\');".to_string();
        let test2: String = "insert into bookmarks (mark, file, type) values (\'grep\', \'linux_pros.txt\', \'PDF\');".to_string();
        let test3: String = "insert into bookmarks (mark, file, type) values (\'chroot\', \'linux_pros.txt\', \'PDF\');".to_string();
        let test4: String = "insert into bookmarks (mark, file, type) values (\'sudo\', \'linux_pros.txt\', \'PDF\');".to_string();
        for (i,c) in cmd.form("bookmarks", &Modifier::AddRecord(container)).unwrap().iter().enumerate() {
            match i {
                0 => assert_eq!(test1, *c),
                1 => assert_eq!(test2, *c),
                2 => assert_eq!(test3, *c),
                3 => assert_eq!(test4, *c),
                _ => panic!("somehow reached {i} with {}", c)
            }
        }
    }

    #[test]
    fn test_add_w_types() {
        let container: ModifyAddRecord = ModifyAddRecord::new(
            "linux_pros.txt", 
            &vec![
                "grub".to_string(), 
                "grep".to_string(), 
                "chroot".to_string(), 
                "sudo".to_string()
            ],
            &vec![
                "PDF".to_string(),
                "Image".to_string()
            ]
        );
        let cmd = LiteAddRecord;
        let test1: String = "insert into bookmarks (mark, file, type) values (\'grub\', \'linux_pros.txt\', \'PDF\');".to_string();
        let test3: String = "insert into bookmarks (mark, file, type) values (\'grep\', \'linux_pros.txt\', \'PDF\');".to_string();
        let test5: String = "insert into bookmarks (mark, file, type) values (\'chroot\', \'linux_pros.txt\', \'PDF\');".to_string();
        let test7: String = "insert into bookmarks (mark, file, type) values (\'sudo\', \'linux_pros.txt\', \'PDF\');".to_string();
        let test2: String = "insert into bookmarks (mark, file, type) values (\'grub\', \'linux_pros.txt\', \'Image\');".to_string();
        let test4: String = "insert into bookmarks (mark, file, type) values (\'grep\', \'linux_pros.txt\', \'Image\');".to_string();
        let test6: String = "insert into bookmarks (mark, file, type) values (\'chroot\', \'linux_pros.txt\', \'Image\');".to_string();
        let test8: String = "insert into bookmarks (mark, file, type) values (\'sudo\', \'linux_pros.txt\', \'Image\');".to_string();
        for (i,c) in cmd.form("bookmarks", &Modifier::AddRecord(container)).unwrap().iter().enumerate() {
            match i {
                0 => assert_eq!(test1, *c),
                1 => assert_eq!(test2, *c),
                2 => assert_eq!(test3, *c),
                3 => assert_eq!(test4, *c),
                4 => assert_eq!(test5, *c),
                5 => assert_eq!(test6, *c),
                6 => assert_eq!(test7, *c),
                7 => assert_eq!(test8, *c),
                _ => panic!("somehow reached {i} with {}", c)
            }
        }
    }

}
