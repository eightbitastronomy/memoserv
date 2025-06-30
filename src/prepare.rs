use memobook::{MemoBook, Queryable};
use memobook::mbfilter::MBFilter;
use memobook::logic::Logic;
use memobook::query::Query;
use memobook::mberror::MBError;
use memobook::modifiers::Modifier;


pub fn prepare_modification(bk: &MemoBook, cmd: &mut Modifier) -> Result<(), MBError> {
    match cmd {
        Modifier::MarkUpdate(ref mut mu) => {
            let tempfilter: MBFilter = MBFilter::new("file".to_string(), Logic::OR, vec![mu.file.clone()]);
            mu.aux = match bk.search(Query::new(vec![tempfilter], "type", false, false, false)) {
                Ok(aq) => {
                    if aq.is_empty() {
                        None
                    } else {
                        Some(aq)
                    }
                },
                Err(e) => { return Err(e); }
            };
            Ok(())
        },
        Modifier::TypeUpdate(ref mut mu) => {
            let tempfilter: MBFilter = MBFilter::new("file".to_string(), Logic::OR, vec![mu.file.clone()]);
            mu.aux = match bk.search(Query::new(vec![tempfilter], "mark", false, false, false)) {
                Ok(aq) => {
                    if aq.is_empty() {
                        None
                    } else {
                        Some(aq)
                    }
                },
                Err(e) => { return Err(e); }
            };
            Ok(())
        },
        _ => Ok(())
    }
}
