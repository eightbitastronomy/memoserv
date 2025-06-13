//  crawler.rs
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
use crate::repository::Repository;
use std::path::PathBuf;


pub enum CrawlOption {
    CaseSensitive(bool),
    FollowLinks(bool),
    Repository(Repository),
    Transport(String),
    Log(String)
}


//was unable to store the &mut FnMut in struct, so here, the trait will simply require that the &mut FnMut is
//passed in with the crawl method.
pub trait Crawler<T>
where T: Clone
{
    fn options(&mut self, optsenum: CrawlOption) -> &mut Self ;
    fn crawl(&mut self, process: &mut impl FnMut(PathBuf) -> Result<T, MBError>) -> Result<&mut Self, MBError>;
    fn retrieve(&self) -> Option<Vec<T>>;
}
