//  suffixhash.rs
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


use crate::maskingset::MaskingSet;
use std::collections::HashMap;

#[derive(Default)]
pub struct SuffixHash {
    sethash: HashMap<String, bool>,
    star: bool
}

impl SuffixHash {

    pub fn new() -> SuffixHash {
        SuffixHash { sethash: HashMap::new(), star: false }
    }

}

impl<'a> MaskingSet<'a> for SuffixHash {

    type M = String;    

    fn add(&mut self, item: &Self::M) -> &mut SuffixHash {
        match item.as_str() {
            "*" => {
                self.star = true;
            },
            _ => {
                self.sethash.insert(item.to_string(), true);
            }
        }
        self
    }

    fn addv(&mut self, itemvec: &[Self::M]) -> &mut SuffixHash {
        for item in itemvec {
            match item.as_str() {
                "*" => {
                    self.star = true;
                    self.sethash.clear();
                    break;
                },
                _ => {
                    self.sethash.insert(item.to_string(), true);
                }
            }
        }
        self
    }

    fn test(&self, item: &Self::M) -> bool {
        if self.sethash.is_empty() {
            return false;
        }
        if self.star {
            return true;
        }
        match item.split('.')
        .collect::<Vec<&str>>()
        .pop() 
        {
            Some(x) => {
                self.sethash.contains_key(x)
            },
            None => {
                false
            }
        }   
    }

}
