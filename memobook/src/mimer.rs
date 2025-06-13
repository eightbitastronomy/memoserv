//  mimer.rs
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


/********************************************
   Mimer: holder for the file suffixes
    associated with mime types. 
    provides iterator. 
*********************************************/


//stack: https://stackoverflow.com/questions/30218886/how-to-implement-iterator-and-intoiterator-for-a-simple-struct
//blog: https://aloso.github.io/2021/03/09/creating-an-iterator



#[derive(Clone)]
pub struct Mimer 
{
    suffixes: Vec<String>,
}


impl Mimer {

    pub fn new_by_vec(list: Vec<String>) -> Mimer {
        Mimer { suffixes: list.to_vec() }
    }
    
    pub fn new_by_slice(list: & [String]) -> Mimer {
        Mimer { suffixes: list.to_vec() }
    }
    
    pub fn iter(&self) -> MimerIterator {
        MimerIterator { mime: self, index: 0, len: self.suffixes.len() }
    }
    //Test function:
    pub fn display(&self) -> String {
        self.suffixes.join(" ").to_string()
    }

    pub fn rem(&mut self, target: &str) {
        let mut ind: i32 = -1;
        for (i,item) in self.suffixes.iter().enumerate() {
            if item as &str == target {
                ind = i as i32;
                break;
            }
        }
        if ind > -1 {
            self.suffixes.remove(ind as usize);
        }
    }
}


impl IntoIterator for Mimer
{

    type Item = String;
    type IntoIter = MimerIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            mime: self.clone(),
            //index: 0,
            //len: self.suffixes.len()
        }
    }

}


pub struct MimerIterator<'a> {
    mime: &'a Mimer,
    index: usize,
    len: usize
}


impl<'a> Iterator for MimerIterator<'a> {

    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = if self.index < self.len {
            Some(self.mime.suffixes[self.index].as_str())
        } else {
            None
        };
        self.index += 1;
        ret
    }

}


pub struct MimerIntoIterator {
    mime: Mimer,
    //index: usize,
    //len: usize
}


impl Iterator for MimerIntoIterator {

    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        //let ret = if self.index < self.len {
        //    Some(self.mime.suffixes[self.index])
        //} else {
        //    None
        //};
        //self.index += 1;
        self.mime.suffixes.pop()
        //ret
    }

}

