//  litequeryassembler.rs
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


pub mod lite_query_assembler {


use crate::queryassembler::QueryAssembler;
use crate::queryer::Queryer;
use crate::logic::Logic;
use crate::filtercontainer::FilterContainer;
use crate::mberror::MBError;


pub struct LiteQueryAssembler<Q>
where 
    Q: for<'a> Queryer<'a>
{
    table: String,
    source: Q
}


impl<Q> LiteQueryAssembler<Q>
where
    Q: for<'a> Queryer<'a>,
{
    pub fn new(table: &str, source: Q) -> LiteQueryAssembler<Q> {
        LiteQueryAssembler { table: table.to_string(), source }
    }
}


fn process_query_string<Q>(filt: &Q, equalcol: &str) -> Result<String, MBError>
where
    Q: FilterContainer
{
    if filt.len() == 1 {
        return Ok("select * from $1 where ".to_string() + filt.filtertype() + "=\'" + filt.iter().next().unwrap() + "\'");
    }
    let mut buildstr: String = String::new();
    let mut counter: usize = 1;
    let mut wherevec: Vec<String> = vec![];
    match filt.logic() {
        Logic::AND => {
            buildstr += "select a1.* from ";
            for term in filt.iter() {
                let counterstring: String = counter.to_string();
                buildstr += format!("(select * from $1 where {}=\'{}\') as a{}", filt.filtertype(), term, counterstring.as_str()).as_str();
                if counter > 1 {
                    //let mut tempstr: String = "a1.file = a".to_string();
                    //tempstr += format!("{}.file", counterstring.as_str()).as_str();
                    let tempstr: String = format!("a1.{equalcol} = a{}.{equalcol}", counterstring.as_str()); 
                    wherevec.push(tempstr);
                }
                if counter < filt.len() {
                    buildstr += ", ";
                    counter += 1;
                } else {
                    buildstr += " ";
                }
            }
            buildstr += "where ";
            for (i, whereterm) in wherevec.iter().enumerate() {
                buildstr += whereterm.as_str();
                if (i + 1) < (counter - 1) {
                    buildstr += " and ";
                }
            }
        },
        Logic::OR => {
            for (i, item) in filt.iter().enumerate() {
                if i == 0 {
                    buildstr += format!("select * from $1 where {}='{}'", filt.filtertype(), item).as_str();
                } else {
                    buildstr += format!(" union select * from $1 where {}='{}'", filt.filtertype(), item).as_str();
                }
            }
        }
    }
    Ok(buildstr)
}


impl<Q> QueryAssembler for LiteQueryAssembler<Q>
where
    Q: for<'a> Queryer<'a>,
{
    fn form(&self) -> Result<String, MBError> {
        // handle blanket searches, aka "toc" calls, first:
        if self.source.has_no_filter() {
            return Ok(format!("select distinct {} from {};", self.source.equality().as_str(), self.table.as_str()));
        }
        // Check for equality vs. filter conflicts
        for filtertemp in self.source.iter_filters() {
            if self.source.equality() == filtertemp.filtertype() {
                return Err(MBError::BadQuery("One or more filter columns match equality column".to_string()));
            }
        }
        // Form the query
        let mut buildquery: String = format!("select distinct {} from ($1);", self.source.equality().as_str());
        for (i, filter) in self.source.iter_filters().enumerate() {
            if i == 0 {
                buildquery = buildquery.replace("$1", process_query_string(filter, self.source.equality().as_str())?.as_str());
            } else {
                buildquery = buildquery.replace("$1", format!("({})", process_query_string(filter, self.source.equality().as_str())?.as_str()).as_str());
            }
        }
        Ok(buildquery.replace("$1", self.table.as_str()).to_string())
    }


    fn grep(&self) -> bool {
        self.source.grep()
    }


    fn grepcase(&self) -> bool {
        self.source.grepcase()
    }


    fn greplink(&self) -> bool {
        self.source.greplink()
    }

    fn complexity(&self) -> usize {
        self.source.iter_filters().map(|x| x.len()).product()
    }

}


} // pub mod lite2assembler


#[cfg(test)]
mod tests {

    use crate::queryassembler::QueryAssembler;
    //use crate::queryer::Queryer;
    use crate::query::Query;
    use crate::logic::Logic;
    //use crate::filtercontainer::FilterContainer;
    use crate::mbfilter::MBFilter;
    use crate::mberror::MBError;
    use super::lite_query_assembler::LiteQueryAssembler; 

    #[test]
    fn test_form_with_q_mbf_1_level_1_term() {
        let m1: MBFilter = MBFilter::new("mark".to_string(), Logic::AND, vec!["This".to_string()]);
        let q1: Query<MBFilter> = Query::new(vec![m1], "file", false, false, false);
        let la1: LiteQueryAssembler<Query<MBFilter>> = LiteQueryAssembler::new("bookmarks", q1);
        let output: String = la1.form().unwrap();
        let teststring: String = "select distinct file from (select * from bookmarks where mark='This');".to_string();
        assert_eq!(output, teststring);
    }

    #[test]
    fn test_form_with_q_mbf_1_level_2_terms() {
        let m1: MBFilter = MBFilter::new("mark".to_string(), Logic::AND, vec!["This".to_string(), "That".to_string()]);
        let q1: Query<MBFilter> = Query::new(vec![m1], "file", false, false, false);
        let la1: LiteQueryAssembler<Query<MBFilter>> = LiteQueryAssembler::new("bookmarks", q1);
        let output: String = la1.form().unwrap();
        let teststring1: String = "select distinct file from (select a1.* from (select * from bookmarks where mark='This') as a1, (select * from bookmarks where mark='That') as a2 where a1.file = a2.file);".to_string();
        assert_eq!(output, teststring1)
    }

    #[test]
    fn test_form_with_q_mbf_1_level_3_terms() {
        let m2: MBFilter = MBFilter::new("mark".to_string(), Logic::AND, vec!["This".to_string(), "That".to_string(), "There".to_string()]);
        let q2: Query<MBFilter> = Query::new(vec![m2], "file", false, false, false);
        let la2: LiteQueryAssembler<Query<MBFilter>> = LiteQueryAssembler::new("bookmarks", q2);
        let output2: String = la2.form().unwrap();
        let teststring2: String = "select distinct file from (select a1.* from (select * from bookmarks where mark='This') as a1, (select * from bookmarks where mark='That') as a2, (select * from bookmarks where mark='There') as a3 where a1.file = a2.file and a1.file = a3.file);".to_string();
        assert_eq!(output2, teststring2);
    }

    #[test]
    fn test_form_with_q_mbf_2_levels() {
        let m2: MBFilter = MBFilter::new("mark".to_string(), Logic::AND, vec!["This".to_string(), "That".to_string(), "There".to_string()]);
        let m3: MBFilter = MBFilter::new("type".to_string(), Logic::OR, vec!["Text".to_string()]);
        let q3: Query<MBFilter> = Query::new(vec![m2, m3], "file", false, false, false);
        let la3: LiteQueryAssembler<Query<MBFilter>> = LiteQueryAssembler::new("bookmarks", q3);
        let output3: String = la3.form().unwrap();
        let teststring3: String = "select distinct file from (select a1.* from (select * from (select * from bookmarks where type='Text') where mark='This') as a1, (select * from (select * from bookmarks where type='Text') where mark='That') as a2, (select * from (select * from bookmarks where type='Text') where mark='There') as a3 where a1.file = a2.file and a1.file = a3.file);".to_string();
        assert_eq!(output3, teststring3);
    }

    #[test]
    fn test_form_with_bad_equality() {
        let m2: MBFilter = MBFilter::new("mark".to_string(), Logic::AND, vec!["This".to_string(), "That".to_string(), "There".to_string()]);
        let m3: MBFilter = MBFilter::new("type".to_string(), Logic::OR, vec!["Text".to_string()]);
        let q3: Query<MBFilter> = Query::new(vec![m2, m3], "type", false, false, false);
        let la3: LiteQueryAssembler<Query<MBFilter>> = LiteQueryAssembler::new("bookmarks", q3);
        let output3: String;
        match la3.form() {
            Ok(x) => { output3 = x; },
            Err(x) => { output3 = match x { 
                    MBError::BadQuery(y) => y,
                    _ => panic!("unknown problem")
                }
            }
        }
        let teststring3: String = "select distinct type from (select a1.* from (select * from (select * from bookmarks where type='Text') where mark='This') as a1, (select * from (select * from bookmarks where type='Text') where mark='That') as a2, (select * from (select * from bookmarks where type='Text') where mark='There') as a3 where a1.file = a2.file and a1.file = a3.file);".to_string();
        assert_ne!(output3, teststring3);
        assert_eq!(output3, "One or more filter columns match equality column".to_string());

    }

    #[test]
    fn test_form_with_nonfile_equality() {
        let m2: MBFilter = MBFilter::new("file".to_string(), Logic::AND, vec!["This".to_string(), "That".to_string(), "There".to_string()]);
        let m3: MBFilter = MBFilter::new("type".to_string(), Logic::OR, vec!["Text".to_string()]);
        let q3: Query<MBFilter> = Query::new(vec![m2, m3], "mark", false, false, false);
        let la3: LiteQueryAssembler<Query<MBFilter>> = LiteQueryAssembler::new("bookmarks", q3);
        let output4: String = la3.form().unwrap();
        let teststring4: String = "select distinct mark from (select a1.* from (select * from (select * from bookmarks where type='Text') where file='This') as a1, (select * from (select * from bookmarks where type='Text') where file='That') as a2, (select * from (select * from bookmarks where type='Text') where file='There') as a3 where a1.mark = a2.mark and a1.mark = a3.mark);".to_string();
        assert_eq!(output4, teststring4);
    }

    #[test]
    fn test_complexity_with_q_mbf_3_levels() {
        let m2: MBFilter = MBFilter::new("mark".to_string(), Logic::AND, vec!["This".to_string(), "That".to_string(), "There".to_string()]);
        let m3: MBFilter = MBFilter::new("type".to_string(), Logic::OR, vec!["Text".to_string(), "Code".to_string()]);
        let m4: MBFilter = MBFilter::new("color".to_string(), Logic::OR, vec!["Black".to_string(), "Red".to_string()]);
        let q4: Query<MBFilter> = Query::new(vec![m2, m3, m4], "file", false, false, false);
        let la4: LiteQueryAssembler<Query<MBFilter>> = LiteQueryAssembler::new("bookmarks", q4);
        assert_eq!(12, la4.complexity());
    }
} 
