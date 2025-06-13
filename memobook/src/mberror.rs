//  mberror.rs
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


use rusqlite::Error;
use std::fmt;


#[derive(Debug)]
pub enum MBError {
    Sqlite(Error),
    BadQuery(String),
    TypeGather(String),
    MarkGather(String),
    BadModify(String),
    SearchError(String),
    FileSys(String),
    Grep(String),
    SelectorQuery(String),
    DBusMessage(String),
    Config(String),
    FileOverPop(String),
    FileNewError(String),
    FileRemError(String),
    Import(String),
    Backup(String),
    Nil
}


impl fmt::Display for MBError {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //write!(f, "SuperError is here!")
        match self {
            MBError::Sqlite(x) => write!(f, "Sqlite MB error: {x}"),
            MBError::BadQuery(x) => write!(f, "BadQuery MB error: {x}"),
            MBError::TypeGather(x) => write!(f, "TypeGather MB error: {x}"),
            MBError::MarkGather(x) => write!(f, "MarkGather MB error: {x}"),
            MBError::BadModify(x) => write!(f, "BadModify MB error: {x}"),
            MBError::SearchError(x) => write!(f, "SearchError MB error: {x}"),
            MBError::FileSys(x) => write!(f, "FileSys MB error: {x}"),
            MBError::Grep(x) => write!(f, "Grep MB error: {x}"),
            MBError::SelectorQuery(x) => write!(f, "SelectorQuery error: {x}"),
            MBError::DBusMessage(x) => write!(f, "DBus message error: {x}"),
            MBError::Config(x) => write!(f, "Configuration error: {x}"),
            MBError::FileOverPop(x) => write!(f, "File creation naming error: {x}"),
            MBError::FileNewError(x) => write!(f, "File creation attempt error: {x}"),
            MBError::FileRemError(x) => write!(f, "File deletion attempt error: {x}"),
            MBError::Import(x) => write!(f, "Import error: {x}"),
            MBError::Backup(x) => write!(f, "Error backing up bookmarks: {x}"),
            MBError::Nil => write!(f, "nil")
        }
    }

}
