//  parse.rs
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


use memobook::repository::Repository;
use memobook::mbfilter::MBFilter;
use memobook::logic::Logic;
use memobook::query::Query;
use memobook::mberror::MBError;
use memobook::modifiers::Modifier;
use memobook::modifiers::{ModifyAddRecord,ModifyFieldReplace,ModifyMarkUpdate,ModifyTargetRemove};
use memobook::transportstruct::TransPortStruct;
use crate::manager::Manager;
use crate::configmodifier::ConfigModifier;


pub fn parse_grep_triplet(input: &[&str]) -> std::result::Result<(bool, bool, bool), MBError> {
    let grep1 = match input[0] {
        "true" => true,
        "false" => false,
        _ => return Err(MBError::DBusMessage("improper grep option".to_string()))
    };
    let grep2 = match input[1] {
        "true" => true,
        "false" => false,
        _ => return Err(MBError::DBusMessage("improper grep option".to_string()))
    };
    let grep3 = match input[2] {
        "true" => true,
        "false" => false,
        _ => return Err(MBError::DBusMessage("improper grep option".to_string()))
    };
    Ok((grep1, grep2, grep3))
}


pub fn parse_search_msg(msgvec: Vec<&str>) -> std::result::Result<Query<MBFilter>, MBError> {
    let msglen = msgvec.len();
    let grepoptions = parse_grep_triplet(&msgvec[0..3])?;
    let equality = msgvec[3];
    let count = match msgvec[4].to_string().parse::<usize>() {
        Ok(x) => x,
        Err(_) => return Err(MBError::DBusMessage("improperly formed message (# of filters)".to_string()))
    };
    if count != msglen - 5 {
        return Err(MBError::DBusMessage("improperly formed message (# of reported terms)".to_string()));
    }
    let mut filtervec: Vec<MBFilter> = Vec::new();
    let mut index:usize = 5;
    while index < msglen {
        let ftype: String = match msgvec[index] {
            "mark" => "mark".to_string(),
            "file" => "file".to_string(),
            "type" => "type".to_string(),
            _ => return Err(MBError::DBusMessage("improper filter term: filter type".to_string()))
        };
        let flogic: Logic = match msgvec[index+1] {
            "and" => Logic::AND,
            "or" => Logic::OR,
            _ => return Err(MBError::DBusMessage("improper filter term: filter logic".to_string()))
        };
        let numvecterm: usize = match msgvec[index+2].to_string().parse::<usize>() {
            Ok(x) => x,
            Err(_) => return Err(MBError::DBusMessage("invalid value for # of filter terms".to_string()))
        };
        let mut termsvec: Vec<String> = Vec::new();
        for subindex in 0..numvecterm {
            termsvec.push(msgvec[index+3+subindex].to_string());
        }
        filtervec.push(MBFilter::new(ftype, flogic, termsvec));
        index += numvecterm + 3;
    }
    if index != msglen {
        return Err(MBError::DBusMessage("search format error or unused terms present".to_string()))
    }
    Ok(Query::new(filtervec, equality, grepoptions.0, grepoptions.1, grepoptions.2))
}


pub fn parse_add_record(input: &[&str]) -> std::result::Result<Modifier, MBError> {    
    let count: usize = input.len();
    let mut index: usize = 1;
    let argfile: &str = input[0];
    let mut argtype: Vec<String> = Vec::new();
    let mut argmark: Vec<String> = Vec::new();
    while index < count {
        let targetvec: &mut Vec<String> = match input[index] {
            "mark" => &mut argmark,
            "type" => &mut argtype,
            _ => return Err(MBError::DBusMessage("improper add term: add term type".to_string()))
        };
        let numvecterm: usize = match input[index+1].to_string().parse::<usize>() {
            Ok(x) => x,
            Err(_) => return Err(MBError::DBusMessage("invalid value for # of add terms".to_string()))
        };
        for subindex in 0..numvecterm {
            targetvec.push(input[index+2+subindex].to_string());
        }
        index += numvecterm + 2;
    }
    if index != count {
        return Err(MBError::DBusMessage("add format error or unused terms present".to_string()))
    }
    if argtype.is_empty() || argmark.is_empty() {
        return Err(MBError::DBusMessage("missing terms for add record".to_string()));
    }
    Ok(Modifier::AddRecord(ModifyAddRecord::new(argfile, &argmark, &argtype)))
}


pub fn parse_field_replace(input: &[&str]) -> std::result::Result<Modifier, MBError> {
    let count: usize = input.len();
    let mut index: usize;
    let argfield: &str = input[0];
    let mut argtuples: Vec<(&str, &str)> = Vec::new();
    let numvecterm: usize = match input[1].to_string().parse::<usize>() {
        Ok(x) => x,
        Err(_) => return Err(MBError::DBusMessage("improperly formed message (invalid value for # of tuple terms)".to_string()))
    };
    if numvecterm == 0 {
        return Err(MBError::DBusMessage("improperly formed message (# of tuple terms is zero)".to_string()));
    }
    if numvecterm >= count {
        return Err(MBError::DBusMessage("improperly formed message (# of tuple terms does not match reported value)".to_string()));
    }
    index = 2;
    if count - numvecterm != index {
        return Err(MBError::DBusMessage("improperly formed message (# of tuple terms does not match reported value)".to_string()));
    }
    if numvecterm % 2 != 0 {
        return Err(MBError::DBusMessage("improperly formed message (not an even # of tuple terms)".to_string()));
    }
    while index < count {
        argtuples.push((input[index], input[index+1]));
        index += 2;
    }
    let mut modfieldrepl: ModifyFieldReplace = ModifyFieldReplace::new(argfield, argtuples.pop().unwrap());
    for tup in argtuples {
        modfieldrepl.add(tup);
    }
    Ok(Modifier::FieldReplace(modfieldrepl))
}


pub fn parse_mark_update(input: &[&str]) -> std::result::Result<Modifier, MBError> {
    let count: usize = input.len();
    let mut index: usize = 1;
    let argfile: &str = input[0];
    let mut argtype: Vec<String> = Vec::new();
    let mut argrems: Vec<String> = Vec::new();
    let mut argadds: Vec<String> = Vec::new();
    while index < count {
        let targetvec: &mut Vec<String> = match input[index] {
            "rem" => &mut argrems,
            "add" => &mut argadds,
            "type" => &mut argtype,
            _ => return Err(MBError::DBusMessage("improper mark update term type".to_string()))
        };
        let numvecterm: usize = match input[index+1].to_string().parse::<usize>() {
            Ok(x) => x,
            Err(_) => return Err(MBError::DBusMessage("invalid value for # of mark update terms".to_string()))
        };
        for subindex in 0..numvecterm {
            targetvec.push(input[index+2+subindex].to_string());
        }
        index += numvecterm + 2;
    }
    if index != count {
        return Err(MBError::DBusMessage("mark update format error or unused terms present".to_string()))
    }
    if argtype.is_empty() || (argrems.is_empty() && argadds.is_empty()) {
        return Err(MBError::DBusMessage("missing terms for mark update".to_string()));
    }
    Ok(Modifier::MarkUpdate(ModifyMarkUpdate::new(argfile, &argtype, &argrems, &argadds)))
}


pub fn parse_target_remove(input: &[&str]) -> std::result::Result<Modifier, MBError> {
    if input.len() != 2 {
        return Err(MBError::DBusMessage("improperly formed message (# of terms)".to_string()));
    }
    Ok(Modifier::TargetRemove(ModifyTargetRemove::new(input[0], input[1])))
}


pub fn parse_modification_msg(input: Vec<&str>) -> std::result::Result<Modifier, MBError> {
    let modtype: &str = input[0]; 
    let count = match input[1].to_string().parse::<usize>() {
        Ok(x) => x,
        Err(_) => return Err(MBError::DBusMessage("improperly formed message (invalid # of terms)".to_string()))
    };
    if count != input.len() - 2 {
        return Err(MBError::DBusMessage("improperly formed message (# of reported terms)".to_string()))
    }
    match modtype {
        "addrecord" => parse_add_record(&input[2..]),
        "fieldreplace" => parse_field_replace(&input[2..]),
        "markupdate" => parse_mark_update(&input[2..]),
        "targetremove" => parse_target_remove(&input[2..]),
        _ => Err(MBError::DBusMessage("unknown modification type".to_string()))
    } 
}


pub fn parse_and_build_repo(trunk: Option<&str>, terms: &[&str]) -> std::result::Result<Repository, MBError> {
    let mut index: usize = 0;
    let length: usize = terms.len();
    let mut includes: Vec<String> = Vec::new();
    let mut excludes: Vec<String> = Vec::new();
    while index < length {
        match terms[index] {
            "include" => {
                let subcount: usize = match terms[index+1].to_string().parse::<usize>() {
                    Ok(x) => x,
                    Err(_) => return Err(MBError::DBusMessage("configuration manage call: invalid # of includes".to_string()))
                };
                let mut subindex: usize = 0;
                while subindex < subcount {
                    includes.push(terms[index+subindex+2].to_string());
                    subindex += 1;
                }
                index += subcount + 2;
            },
            "exclude" => {
                let subcount: usize = match terms[index+1].to_string().parse::<usize>() {
                    Ok(x) => x,
                    Err(_) => return Err(MBError::DBusMessage("configuration manage call: invalid # of includes".to_string()))
                };
                let mut subindex: usize = 0;
                while subindex < subcount {
                    excludes.push(terms[index+subindex+2].to_string());
                    subindex += 1;
                }
                index += subcount + 2;
            },
            _ => return Err(MBError::DBusMessage("invalid keyword in configuration manage call".to_string()))
        }
    }
    if index != length {
        return Err(MBError::DBusMessage("configuration manage call: format error or unused terms present".to_string()))
    }
    let mut repo: Repository = Repository::new();
    if let Some(trunkpath) = trunk {
        repo.set_trunk(trunkpath);
    }
    repo.add_include_v(includes);
    repo.add_exclude_v(excludes);
    Ok(repo)
}


pub fn parse_repo_modify(terms: &[&str]) -> std::result::Result<(Repository,Repository), MBError> {
    let mut index: usize = 0;
    let length: usize = terms.len();
    let mut adds: Repository = Repository::new();
    let mut rems: Repository = Repository::new();
    while index < length {
        match terms[index] {
            "remove" => {
                let sublength: usize = match terms[index+1].to_string().parse::<usize>() {
                    Ok(x) => x,
                    Err(_) => return Err(MBError::DBusMessage("configuration manage call: invalid # of removals".to_string()))
                };
                rems = match parse_and_build_repo(None, &terms[(index+2)..(index+sublength+2)]) {
                    Ok(a) => a,
                    Err(e) => return Err(MBError::DBusMessage(format!("error parsing include and exclude terms: {e}")))
                };
                index += sublength + 2;
            },
            "add" => {
                let sublength: usize = match terms[index+1].to_string().parse::<usize>() {
                    Ok(x) => x,
                    Err(_) => return Err(MBError::DBusMessage("configuration manage call: invalid # of removals".to_string()))
                };
                adds = match parse_and_build_repo(None, &terms[(index+2)..(index+sublength+2)]) {
                    Ok(a) => a,
                    Err(e) => return Err(MBError::DBusMessage(format!("error parsing include and exclude terms: {e}")))
                };
                index += sublength + 2;
            },
            _ => return Err(MBError::DBusMessage("invalid keyword in configuration manage call".to_string()))
        }
    }
    if index != length {
        return Err(MBError::DBusMessage("configuration manage call: format error or unused terms present".to_string()))
    }
    Ok((rems,adds))
}


pub fn parse_manage_config(input: &[&str]) -> std::result::Result<Manager, MBError> {
    match input[0] {
        "setsource" => {
            if input.len() != 2 {
                Err(MBError::DBusMessage("improperly formed configuration manage call (invalid # of terms)".to_string()))
            } else {
                Ok(Manager::Configure(ConfigModifier::SetSource(input[1].to_string())))
            }
        },
        "setrepo" => {
            let count = match input[1].to_string().parse::<usize>() {
                Ok(x) => x,
                Err(_) => return Err(MBError::DBusMessage("improperly formed configuration manage call (invalid # of terms)".to_string()))
            };
            if count != input.len() - 2 {
                return Err(MBError::DBusMessage("improperly formed configuration manage call (# of reported terms)".to_string()))
            }
            let repo: Repository = match parse_and_build_repo(None, &input[2..]) {
                Ok(r) => r,
                Err(e) => return Err(MBError::DBusMessage(format!("error parsing include and exclude terms: {e}")))
            };
            Ok(Manager::Configure(ConfigModifier::SetRepo(repo)))
        },
        "setrepotr" => {
            let trunk = input[1];
            let count = match input[2].to_string().parse::<usize>() {
                Ok(x) => x,
                Err(_) => return Err(MBError::DBusMessage("improperly formed configuration manage call (invalid # of terms)".to_string()))
            };
            if count != input.len() - 3 {
                return Err(MBError::DBusMessage("improperly formed configuration manage call (# of reported terms)".to_string()))
            }
            let repo: Repository = match parse_and_build_repo(Some(trunk), &input[3..]) {
                Ok(r) => r,
                Err(e) => return Err(MBError::DBusMessage(format!("error parsing include and exclude terms: {e}")))
            };
            Ok(Manager::Configure(ConfigModifier::SetRepo(repo)))
        },
        "modrepo" => {
            let count = match input[1].to_string().parse::<usize>() {
                Ok(x) => x,
                Err(_) => return Err(MBError::DBusMessage("improperly formed configuration manage call (invalid # of terms)".to_string()))
            };
            if count != input.len() - 2 {
                return Err(MBError::DBusMessage("improperly formed configuration manage call (# of reported terms)".to_string()))
            }
            let repotuple = match parse_repo_modify(&input[2..]) {
                Ok(r) => r,
                Err(e) => return Err(MBError::DBusMessage(format!("error parsing addition and removal terms: {e}")))
            };
            Ok(Manager::Configure(ConfigModifier::ModifyRepo(repotuple)))
        },
        _ => Err(MBError::DBusMessage("invalid configuration manage call type".to_string()))
    }
}


pub fn parse_manage_import(input: &[&str]) -> std::result::Result<Manager, MBError> {
    let mut importfn: &str = "";
    let mut logfn: &str = "";
    let mut followlinks: bool = false;
    let mut index: usize = 0;
    let count: usize = input.len();
    if count % 2 != 0 {
        return Err(MBError::DBusMessage("invalid format for import command".to_string()));
    }
    while index < count {
        match input[index] {
            "source" => {
                importfn = input[index + 1];
            },
            "log" => {
                logfn = input[index + 1];
            },
            "link" => {
                match input[index + 1] {
                    "true" => {
                        followlinks = true;
                    },
                    "false" => {
                        followlinks = false;
                    },
                    ln => {
                        return Err(MBError::DBusMessage(format!("invalid value [{ln}] for links option in import")));
                    }
                }
            },
            kw => {
                return Err(MBError::DBusMessage(format!("invalid keyword [{kw}] in import command")));
            }
        }
        index += 2;
    }
    Ok(Manager::Import(TransPortStruct {target: importfn.to_string(), log: logfn.to_string(), links: followlinks}))
}


pub fn parse_manage_export(input: &[&str]) -> std::result::Result<Manager, MBError> {
    Ok(Manager::Export(TransPortStruct {target: "".to_string(), log: input[0].to_string(), links: false}))
}


pub fn parse_manage_msg(input: Vec<&str>) -> std::result::Result<Manager, MBError> {
    let managetype: &str = input[0];
    let count = match input[1].to_string().parse::<usize>() {
        Ok(x) => x,
        Err(_) => return Err(MBError::DBusMessage("improperly formed message (invalid # of terms)".to_string()))
    };
    if count != input.len() - 2 {
        return Err(MBError::DBusMessage("improperly formed message (# of reported terms)".to_string()))
    }
    match managetype {
        "configuration" => parse_manage_config(&input[2..]),
        "import" => parse_manage_import(&input[2..]),
        "export" => parse_manage_export(&input[2..]),
        _ => Err(MBError::DBusMessage("unknown manage call type".to_string()))
    }
}


pub fn parse_toc_msg(msg: &str) -> std::result::Result<Query<MBFilter>, MBError> {
    let equality = match msg {
        "file" => "file",
        "mark" => "mark",
        "type" => "type",
        _ => { return Err(MBError::DBusMessage("invalid toc request".to_string())); }
    };
    Ok(Query::new(vec![], equality, false, false, false))
}
