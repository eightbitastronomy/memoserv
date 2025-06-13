//  query.rs
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


use crate::queryer::Queryer;
use crate::filtercontainer::FilterContainer;

#[derive(Clone)]
pub struct Query<M: FilterContainer> {
    filter: Vec<M>,
    equality: String,
    grep: bool,
    grepcase: bool,
    greplinks: bool
}



impl<M: FilterContainer> Query<M> {


    pub fn new(filter: Vec<M>,
            equal: &str, 
            grep: bool, 
            grepcase: bool, 
            greplinks: bool) -> Query<M> {
        Query { filter, 
                equality: equal.to_string(),
                grep, 
                grepcase, 
                greplinks 
        }
    }

}


impl<'a, M: FilterContainer +'a> Queryer<'a> for Query<M> {

    type Q = M;


    fn equality(&self) -> String {
        self.equality.to_string()
    }


    fn grep(&self) -> bool {
        self.grep
    }


    fn pop(&mut self) -> Option<Self::Q> {
        self.filter.pop()
    }


    fn grepcase(&self) -> bool {
        self.grepcase
    }


    fn greplink(&self) -> bool {
        self.greplinks
    }


    fn iter_filters(&'a self) -> impl Iterator<Item = &'a Self::Q> 
    {
        QueryIterator { filter: &self.filter, index: 0, len: self.filter.len() }
    }


    fn has_no_filter(&self) -> bool {
        self.filter.is_empty()
    }


}

/*impl<'a, Q: FilterContainer + 'a> Queryer<'a, Q> for Query<Q> {

    fn grep(&self) -> bool {
        self.grep
    }


    fn grepcase(&self) -> bool {
        self.grepcase
    }


    fn greplink(&self) -> bool {
        self.greplinks
    }


    fn iter_filters(&'a self) -> impl Iterator<Item = &'a Q> 
    {
        QueryIterator { filter: &self.filter, index: 0, len: self.filter.len() }
    }


}*/


//Forget this for the moment
/*impl<M: FilterContainer> IntoIterator for Query<M> {

    type Item = M;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(mut self) -> Self::IntoIter {
        self.filter.into_iter()
    }

}*/



pub struct QueryIterator<'a, M>
where M: FilterContainer 
{
    filter: &'a Vec<M>,
    index: usize,
    len: usize
}


impl<'a, Q> Iterator for QueryIterator<'a, Q>
where Q: FilterContainer {

    type Item = &'a Q;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = if self.index < self.len {
            Some(&self.filter[self.index])
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
    use crate::logic::Logic;
    use crate::mbfilter::MBFilter;

    #[test]
    fn test_filter_iteration() {
        let s1 = vec!["one".to_string(),"two".to_string(),"three".to_string(),"five".to_string()];
        let filt1: MBFilter = MBFilter::new("mark".to_string(), Logic::OR, s1);
        let s2 = vec!["aa".to_string(), "bb".to_string(), "cc".to_string()];
        let filt2: MBFilter = MBFilter::new("type".to_string(), Logic::AND, s2);
        let q = Query::new(vec![filt1, filt2], "file", false, false, false);
        let types: Vec<&MBFilter> = q.iter_filters().collect();
        assert!(types.iter().any(|&s| s.filtertype() == "mark"));
        assert!(types.iter().any(|&s| s.filtertype() == "type"));
        for f in q.iter_filters() {
            if f.filtertype() == "mark" {
                let subs: Vec<&str> = f.iter().collect();
                assert!(subs.iter().any(|&s| s == "one"));
                assert!(subs.iter().any(|&s| s == "two"));
                assert!(subs.iter().any(|&s| s == "three"));
                assert!(subs.iter().any(|&s| s == "five"));
            }
            if f.filtertype() == "type" {
                let subs: Vec<&str> = f.iter().collect();
                assert!(subs.iter().any(|&s| s == "aa"));
                assert!(subs.iter().any(|&s| s == "bb"));
                assert!(subs.iter().any(|&s| s == "cc"));
            }
        }
    }

}
