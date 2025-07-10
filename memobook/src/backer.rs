//  backer.rs
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


use crate::mberror::MBError;
use chrono::{Datelike, Timelike};


pub type BuNumber = u16;


pub trait Backer
{
    type BItem: BackCopy + Clone;

    // suffix fctns accept/return Vec<String>, thus are handled internally

    fn check(&self) -> bool;
    fn erase(&self, targ: Option<Self::BItem>); //needed internally
    fn get(&self, target: &str) -> Option<&Self::BItem>;
    fn get_base(&self) -> String;
    fn get_frequency(&self) -> BuNumber;
    fn get_least_recent(&self) -> Option<Self::BItem>; 
    fn get_location(&self) -> String;
    fn get_most_recent(&self) -> Option<Self::BItem>;
    fn get_multiplicity(&self) -> BuNumber;
    fn get_suffix(&self) -> Vec<String>;
    fn iter(&self) -> impl Iterator<Item=&Self::BItem>;
    fn make(&mut self, src: &str, aux: &[&str]) -> Result<(), MBError>;
    fn pop(&mut self, target: &str) -> Option<Self::BItem>;
    fn pop_least_recent(&mut self) -> Option<Self::BItem>; 
    fn pop_most_recent(&mut self) -> Option<Self::BItem>; 
    fn push(&mut self, item: Self::BItem); 
    fn remove(&mut self, target: &str);
    fn remove_and_erase(&mut self, targ: Option<&Self::BItem>);
    fn set_base(&mut self, portion: &str);
    fn set_frequency(&mut self, val: BuNumber);
    fn set_location(&mut self, loc: &str);
    fn set_multiplicity(&mut self, val: BuNumber);
    fn set_suffix(&mut self, portionvec: &[String]);
    fn set_version(&mut self, vers: &str);
    fn version(&self) -> String;
     
}



pub trait BackCopy
{

    type D: PartialEq + Eq + PartialOrd + Ord + Datelike + Timelike + Clone;

    fn date(&self) -> Self::D;
    fn set_date(&mut self, date: &Self::D);
    fn path(&self) -> String;
    fn set_path(&mut self, path: &str);
    fn aux(&self) -> Vec<String>;
    fn set_aux(&mut self, aux: &[&str]);

}



pub struct TransBackStruct {
    pub base: Option<String>,
    pub loc: Option<String>,
    pub freq: Option<BuNumber>,
    pub mult: Option<BuNumber>,
    pub load: Option<String>,
    pub remove: bool,
    pub force: bool
}
