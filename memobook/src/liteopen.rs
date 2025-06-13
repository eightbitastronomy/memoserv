//  liteopen.rs
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


use crate::dbopenerassembler::DBOpenerAssembler;


pub struct LiteOpen;


impl DBOpenerAssembler for LiteOpen {

    fn form_create_table(&self, table: &str) -> String {
        format!("create table {table} (mark NCHAR(64) NOT NULL,file NCHAR(512) NOT NULL,type NCHAR(64));")
    } 

    fn form_select_all(&self, table: &str) -> String {
        format!("select * from sqlite_master where type='table' and name='{table}'")
    }

}
