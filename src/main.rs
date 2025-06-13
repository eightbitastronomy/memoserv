//  main.rs
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


use zbus::{connection::Builder, Result};
use event_listener::{Listener};
use memoserv::MemoBookServer;
use memobook::{MemoBook, Queryable};
use memobook::configuration::Configuration;
use std::env;
use std::sync::{Arc,Mutex};
use std::sync::atomic::{AtomicBool, Ordering};



#[tokio::main]
async fn main() -> Result<()> {

    let cmdline: Vec<String> = env::args().collect();
    let confaddr: &str = if cmdline.len() == 2 { &cmdline[1] }
        else { "" };

    if confaddr.is_empty() {
        println!("Valid configuration file needed.");
        return Ok(());
    }

    let mut conf: Configuration = match Configuration::read(confaddr) {
        Ok(c) => c,
        Err(e) => { 
            println!("Could not open configuration file: {:?}", e);
            return Ok(());
        }
    };

    match conf.check_for_initialization() {
        Ok(_) => {},
        Err(e) => { 
            println!("Initialization checks failed: {e}");
            return Ok(());
        }
    }
    let mut d = MemoBook::new(&mut conf);

    match d.connect(None) {
        Ok(_) => println!("db opened"),
        Err(x) => {
            println!("db could not be opened: {:?}", x);
            return Ok(());
        }
    }
    
    let mbcover = Arc::new(Mutex::new(d));
    let exitflag = Arc::new(AtomicBool::new(false));

    let memobook = MemoBookServer {
        name: "MemoBook".to_string(),
        cfg: conf,
        events: Arc::new(event_listener::Event::new()),
        exitflag: exitflag.clone(),
        mb: mbcover
    };
    let events_clone = memobook.events.clone();
    let _connection = Builder::session()?
        .name("org.memobook.memoserv1")?
        .serve_at("/org/memobook/memoserv1", memobook)?
        .build()
        .await?;

    loop {
        if exitflag.load(Ordering::SeqCst) {
            break;
        }
        let events_listener = events_clone.listen();
        events_listener.wait();
    }
    
    println!("Shutting down memoserv.");

    Ok(())

}
