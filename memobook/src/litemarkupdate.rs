//  litemarkupdate.rs
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



pub struct LiteMarkUpdate;


impl ModifierAssembler for LiteMarkUpdate {

    fn form(&self, table: &str, mdfy: &Modifier) -> Result<Vec<String>, MBError> {
        match mdfy {
            Modifier::MarkUpdate(mu) => {
                let mut resultvec: Vec<String> = Vec::new();
                let mut largest: &Vec<String> = &mu.rem;
                let mut difference:i32 = (mu.rem.len() as i32) - (mu.add.len() as i32);
                let mut addingflag:bool = false;
                if difference < 0 {
                    largest = &mu.add;
                    difference = difference.abs();
                    addingflag = true;
                };
                let tail:i32 = difference.abs();
                let mut lastcounted:i32 = 0;
                for pair in mu.rem.iter().zip(mu.add.iter()) {
                    /* did I have other branch arms in mind?
                    match pair {
                        _ => resultvec.push(format!("update {table} set mark=\'{}\' where file=\'{}\' and mark=\'{}\';", pair.1, mu.file, pair.0))
                    }*/
                    resultvec.push(format!("update {table} set mark=\'{}\' where file=\'{}\' and mark=\'{}\';", pair.1, mu.file, pair.0));
                    lastcounted += 1;
                }
                for i in 0..tail {
                    if addingflag {
                        for typ in mu.ftypes.iter() {
                            resultvec.push(format!("insert into {table} (mark, file, type) values (\'{}\', \'{}\', \'{}\');", largest[(lastcounted+i) as usize], mu.file, typ));
                        }
                    } else if mu.ftypes.len() > 1 {
                        for typ in mu.ftypes.iter() {
                            resultvec.push(format!("delete from {table} where mark=\'{}\' and file=\'{}\' and type=\'{}\';", largest[(lastcounted+i) as usize], mu.file, typ));
                        }
                    } else {
                        resultvec.push(format!("delete from {table} where mark=\'{}\' and file=\'{}\';", largest[(lastcounted+i) as usize], mu.file));
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
    use crate::modifiers::ModifyMarkUpdate;

    #[test]
    fn test_1_eq_2() {
        let container: ModifyMarkUpdate = ModifyMarkUpdate::new(
                                        "arbit.txt", 
                                        &vec!["Text".to_string()], 
                                        &vec!["python".to_string(),"wheel".to_string()],
                                        &vec!["rust".to_string(),"nightly".to_string()]
                                );
        let cmd = LiteMarkUpdate;
        for (i, line) in cmd.form("bookmarks", &Modifier::MarkUpdate(container)).unwrap().iter().enumerate() {
            match i {
                0 => assert_eq!(*line, "update bookmarks set mark=\'rust\' where file=\'arbit.txt\' and mark=\'python\';".to_string()),
                1 => assert_eq!(*line, "update bookmarks set mark=\'nightly\' where file=\'arbit.txt\' and mark=\'wheel\';".to_string()),
                _ => panic!("somehow reached i={} with {}", i, line)
            }
        }
    }

    #[test]
    fn test_1_eq_2_types() {
        let container: ModifyMarkUpdate = ModifyMarkUpdate::new(
                                        "arbit.txt", 
                                        &vec!["Text".to_string(), "Code".to_string()], 
                                        &vec!["python".to_string(),"wheel".to_string()],
                                        &vec!["rust".to_string(),"nightly".to_string()]
                                );
        let cmd = LiteMarkUpdate;
        for (i, line) in cmd.form("bookmarks", &Modifier::MarkUpdate(container)).unwrap().iter().enumerate() {
            match i {
                0 => assert_eq!(*line, "update bookmarks set mark=\'rust\' where file=\'arbit.txt\' and mark=\'python\';".to_string()),
                1 => assert_eq!(*line, "update bookmarks set mark=\'nightly\' where file=\'arbit.txt\' and mark=\'wheel\';".to_string()),
                _ => panic!("somehow reached i={} with {}", i, line)
            }
        }
    }


    #[test]
    fn test_1_gt_2() {
        let container: ModifyMarkUpdate = ModifyMarkUpdate::new(
                                        "arbit.txt", 
                                        &vec!["Text".to_string()], 
                                        &vec!["python".to_string(),"wheel".to_string()],
                                        &vec!["rust".to_string(),"nightly".to_string(),"update".to_string(),"branch".to_string()]
                                );
        let cmd = LiteMarkUpdate;
        for (i, line) in cmd.form("bookmarks", &Modifier::MarkUpdate(container)).unwrap().iter().enumerate() {
            match i {
                0 => assert_eq!(*line, "update bookmarks set mark=\'rust\' where file=\'arbit.txt\' and mark=\'python\';".to_string()),
                1 => assert_eq!(*line, "update bookmarks set mark=\'nightly\' where file=\'arbit.txt\' and mark=\'wheel\';".to_string()),
                2 => assert_eq!(*line, "insert into bookmarks (mark, file, type) values (\'update\', \'arbit.txt\', \'Text\');".to_string()),
                3 => assert_eq!(*line, "insert into bookmarks (mark, file, type) values (\'branch\', \'arbit.txt\', \'Text\');".to_string()),
                _ => panic!("somehow reached i={} with {}", i, line)
            }
        }
    }

    #[test]
    fn test_1_gt_2_types() {
        let container: ModifyMarkUpdate = ModifyMarkUpdate::new(
                                        "arbit.txt", 
                                        &vec!["Text".to_string(), "Code".to_string()], 
                                        &vec!["python".to_string(),"wheel".to_string()],
                                        &vec!["rust".to_string(),"nightly".to_string(),"update".to_string(),"branch".to_string()]
                                );
        let cmd = LiteMarkUpdate;
        for (i, line) in cmd.form("bookmarks", &Modifier::MarkUpdate(container)).unwrap().iter().enumerate() {
            match i {
                0 => assert_eq!(*line, "update bookmarks set mark=\'rust\' where file=\'arbit.txt\' and mark=\'python\';".to_string()),
                1 => assert_eq!(*line, "update bookmarks set mark=\'nightly\' where file=\'arbit.txt\' and mark=\'wheel\';".to_string()),
                2 => assert_eq!(*line, "insert into bookmarks (mark, file, type) values (\'update\', \'arbit.txt\', \'Text\');".to_string()),
                3 => assert_eq!(*line, "insert into bookmarks (mark, file, type) values (\'update\', \'arbit.txt\', \'Code\');".to_string()),
                4 => assert_eq!(*line, "insert into bookmarks (mark, file, type) values (\'branch\', \'arbit.txt\', \'Text\');".to_string()),
                5 => assert_eq!(*line, "insert into bookmarks (mark, file, type) values (\'branch\', \'arbit.txt\', \'Code\');".to_string()),
                _ => panic!("somehow reached i={} with {}", i, line)
            }
        }
    }

    #[test]
    fn test_2_gt_1() {
        let container: ModifyMarkUpdate = ModifyMarkUpdate::new(
                                        "arbit.txt", 
                                        &vec!["Text".to_string()], 
                                        &vec!["python".to_string(),"wheel".to_string(), "repository".to_string(), "distro".to_string(), "kernel".to_string()],
                                        &vec!["rust".to_string(),"nightly".to_string()]
                                );
        let cmd = LiteMarkUpdate;
        for (i, line) in cmd.form("bookmarks", &Modifier::MarkUpdate(container)).unwrap().iter().enumerate() {
            match i {
                0 => assert_eq!(*line, "update bookmarks set mark=\'rust\' where file=\'arbit.txt\' and mark=\'python\';".to_string()),
                1 => assert_eq!(*line, "update bookmarks set mark=\'nightly\' where file=\'arbit.txt\' and mark=\'wheel\';".to_string()),
                2 => assert_eq!(*line, "delete from bookmarks where mark=\'repository\' and file=\'arbit.txt\';".to_string()),
                3 => assert_eq!(*line, "delete from bookmarks where mark=\'distro\' and file=\'arbit.txt\';".to_string()),
                4 => assert_eq!(*line, "delete from bookmarks where mark=\'kernel\' and file=\'arbit.txt\';".to_string()),
                _ => panic!("somehow reached i={} with {}", i, line)
            }
        }
    }

    #[test]
    fn test_2_gt_1_types() {
        let container: ModifyMarkUpdate = ModifyMarkUpdate::new(
                                        "arbit.txt", 
                                        &vec!["Text".to_string(), "Code".to_string()], 
                                        &vec!["python".to_string(),"wheel".to_string(), "repository".to_string(), "distro".to_string(), "kernel".to_string()],
                                        &vec!["rust".to_string(),"nightly".to_string()]
                                );
        let cmd = LiteMarkUpdate;
        for (i, line) in cmd.form("bookmarks", &Modifier::MarkUpdate(container)).unwrap().iter().enumerate() {
            match i {
                0 => assert_eq!(*line, "update bookmarks set mark=\'rust\' where file=\'arbit.txt\' and mark=\'python\';".to_string()),
                1 => assert_eq!(*line, "update bookmarks set mark=\'nightly\' where file=\'arbit.txt\' and mark=\'wheel\';".to_string()),
                2 => assert_eq!(*line, "delete from bookmarks where mark=\'repository\' and file=\'arbit.txt\' and type=\'Text\';".to_string()),
                3 => assert_eq!(*line, "delete from bookmarks where mark=\'repository\' and file=\'arbit.txt\' and type=\'Code\';".to_string()),
                4 => assert_eq!(*line, "delete from bookmarks where mark=\'distro\' and file=\'arbit.txt\' and type=\'Text\';".to_string()),
                5 => assert_eq!(*line, "delete from bookmarks where mark=\'distro\' and file=\'arbit.txt\' and type=\'Code\';".to_string()),
                6 => assert_eq!(*line, "delete from bookmarks where mark=\'kernel\' and file=\'arbit.txt\' and type=\'Text\';".to_string()),
                7 => assert_eq!(*line, "delete from bookmarks where mark=\'kernel\' and file=\'arbit.txt\' and type=\'Code\';".to_string()),
                _ => panic!("somehow reached i={} with {}", i, line)
            }
        }
    }

    #[test]
    fn test_1_empty() {
        let container: ModifyMarkUpdate = ModifyMarkUpdate::new(
                                        "arbit.txt", 
                                        &vec!["Text".to_string()], 
                                        &vec!["python".to_string(),"wheel".to_string(), "repository".to_string(), "distro".to_string(), "kernel".to_string()],
                                        &vec![]
                                );
        let cmd = LiteMarkUpdate;
        for (i, line) in cmd.form("bookmarks", &Modifier::MarkUpdate(container)).unwrap().iter().enumerate() {
            match i {
                0 => assert_eq!(*line, "delete from bookmarks where mark=\'python\' and file=\'arbit.txt\';".to_string()),
                1 => assert_eq!(*line, "delete from bookmarks where mark=\'wheel\' and file=\'arbit.txt\';".to_string()),
                2 => assert_eq!(*line, "delete from bookmarks where mark=\'repository\' and file=\'arbit.txt\';".to_string()),
                3 => assert_eq!(*line, "delete from bookmarks where mark=\'distro\' and file=\'arbit.txt\';".to_string()),
                4 => assert_eq!(*line, "delete from bookmarks where mark=\'kernel\' and file=\'arbit.txt\';".to_string()),
                _ => panic!("somehow reached i={} with {}", i, line)
            }
        }
    }

}
