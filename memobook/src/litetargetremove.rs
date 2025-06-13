//  litetargetremove.rs
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


pub struct LiteTargetRemove;


impl ModifierAssembler for LiteTargetRemove {

    fn form(&self, table: &str, mdfy: &Modifier) -> Result<Vec<String>, MBError> {
        match mdfy {
            Modifier::TargetRemove(tr) => {
                Ok(vec![format!("delete from {table} where {}=\'{}\';", tr.ttype, tr.value)])
            },
            _ => Err(MBError::BadModify("incorrect modification type for modification assembler".to_string()))
        }
    }

}


#[cfg(test)]
mod tests {

    use super::*;
    use crate::modifiers::ModifyTargetRemove;

    #[test]
    fn test_remove() {
        let container: ModifyTargetRemove = ModifyTargetRemove::new("file", "arbalest.txt");
        let cmd = LiteTargetRemove;
        let test: Vec<String> = vec!["delete from bookmarks where file=\'arbalest.txt\';".to_string()];
        assert_eq!(cmd.form("bookmarks", &Modifier::TargetRemove(container)).unwrap(), test);
    }

}
