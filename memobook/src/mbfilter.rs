//  mbfilter.rs
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


use crate::logic::Logic as Logic;
use crate::filtercontainer::FilterContainer;

#[derive(Clone)]
pub struct MBFilter {
    ftype: String,
    logic: Logic,
    list: Vec<String>
}


impl MBFilter {
    
    pub fn new(ftype: String, logic: Logic, list: Vec<String>) -> MBFilter {
        MBFilter { ftype, logic, list }
    }

}


impl FilterContainer for MBFilter {

    fn filtertype(&self) -> &str {
        &self.ftype
    }

    fn logic(&self) -> &Logic {
        &self.logic
    }

    fn len(&self) -> usize {
        self.list.len()
    }
    
    fn iter(&self) -> impl Iterator<Item = &str> {
        MBFilterIter { //ftype: &self.ftype, 
                    //logic: &self.logic,
                    list: &self.list, 
                    index: 0, 
                    len: self.list.len() 
        }
    }

    fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

}


pub struct MBFilterIter<'a> {
    //ftype: &'a String,
    //logic: &'a Logic,
    list: &'a Vec<String>,
    index: usize,
    len: usize
}


impl<'a> Iterator for MBFilterIter<'a> {
    
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = if self.index < self.len {
            Some(self.list[self.index].as_str())
        } else {
            None
        };
        self.index += 1;
        ret   
    }

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_iter() {

        let s = vec!["one".to_string(),"two".to_string(),"three".to_string(),"five".to_string()];
        let filt: MBFilter = MBFilter::new("mark".to_string(), Logic::OR, s);
        for (i,f) in filt.iter().enumerate() {
            match i {
                0 => { assert_eq!("one",f); },
                1 => { assert_eq!("two",f); },
                2 => { assert_eq!("three",f); },
                3 => { assert_eq!("five",f); },
                _ => { panic!("Unknown case in iteration"); }
            };
        }

    }

    #[test]
    fn test_filtertype() {
        let s = vec!["one".to_string(),"two".to_string(),"three".to_string(),"five".to_string()];
        let filt: MBFilter = MBFilter::new("mark".to_string(), Logic::OR, s);
        assert_eq!("mark", filt.filtertype());
    }

}
