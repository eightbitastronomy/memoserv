//  liteexportquery.rs
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


pub struct LiteExportQuery {
    table: String
}


impl LiteExportQuery {

    pub fn new(tablenm: &str) -> LiteExportQuery {
        LiteExportQuery {
            table: tablenm.to_string()
        }
    }

    pub fn form_mark_query(&self, filenm: &str) -> String {
        format!("select mark from {} where file=\'{filenm}\';", self.table)
    }

    pub fn form_type_query(&self, filenm: &str) -> String {
        format!("select distinct type from {} where file=\'{filenm}\';", self.table)
    }

    pub fn form_toc(&self) -> String {
        format!("select distinct file from {};", self.table)
    }

}
