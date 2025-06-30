# MemoServ
- Backend service for MemoBook, communication via DBus. A work in progress, but is functional. Is designed to run as a low-resource service that any application can access so long as that application can send/receive messages via DBus.
- MemoServ operates an SQLite3 database (which could conceivably be replaced by a different type of DB with some modifications; I made a few attempts to hide the db calls behind trait implementations, but there are some areas where I didn't bother), parsing requests to search, add/remove entries, modify entries. MemoServ also parses requests to manage its configuration and its automatic back-up procedures.
- Any application that can access the DBus and send/receive strings can operate the MemoServ.
- With some small exceptions (that I'm not sure represent substantial use-case scenarios), the MemoServ can handle requests in parallel. The exceptions are only for a subset of configuration requests, not for DB accesses.
## MemoBook?
- My other project is now defunct, as I was migrating from Tk to Wx, but ran out of time and interest, plus the concept was too limiting. Instead, a frontend for this service can be found under my projects called "MemoFace", written in Python using WxPython. 
- Notably, within MemoServ's source, one will find the crate Memobook. The naming of this code is a shoutout, of sorts, to the Python code that start all this nonsense.
## What Does One Do with MemoServ & a Frontend?
- MemoBook was a note-keeping system, so MemoServ is a note-keeping system. By note-keeping system, I mean a way to store keywords that referred to files. Say you have a code snippet that never mentions the word "Doomlike" but that word is exactly what you think of when you want that code. MemoServ will allow you to store such keywords with the file associations so that you can search on "Doomlike" and get the file containing your code.
- While MemoBook operated either by storing keywords in the files as text or by storing keywords in an xml file, MemoServ stores its keywords only in its database.
- So, to use MemoServ, one starts adding files with keywords as one goes about one's daily business. In time, the database gets large enough to be helpful.
- If one should migrate from one computer to another (say, when your computer gets old, you buy a new one, and you copy your files over...), one uses MemoServ to "export" its database to a json file which contains checksums of the original files. Then, on the new system, one uses MemoServ to "import" using the json file and a list of folders to search in (or explicitly NOT to search in). MemoServ uses checksums to match the new files to old and then store your keywords.
- Note, a frontend is not strictly necessary. But, if you don't use a frontend, you're stuck using, e.g., busctl to access MemoServ. The idea is for me to write plugins for vim and emacs like I did once upon a time.
## How-to
- Coming soon.
## Requirements
- Rust: I'm still new to Rust and haven't investigated what the minimum version of Rust is required.
- Rust libraries: uses crates json, zbus (dbus communications), rusqlite (sqlite3), tokio (async runtime), chrono (dates & times), and sha256 (checksums). And maybe a few others I'm forgetting.
- Environment: while I have tested a number of features in Windows 11, deal-breakers for Windows environments include not having DBus or Sqlite3 installed. Sqlite3 is easy-peasy to install but I have yet to venture into the land of DBus on Windows. Grep functionality would also be broken without grep, though this not critical to the service. My understanding however is that uutils/coreutils (found on GitHub) is a rust re-write of the GNU coreutils, and should be able to provide grep functionality. Altogether, typical Linux-based distros should be fine (Ubuntu, Fedora, Pop!, and BunsenLabs to name a few), but Windows may take some work. 
- This is a 100% Rust project, so the aforementioned services/utilities and the availability of Rust on a system are what determines whether MemoService will run. Well, I suppose that's too strong a statement. If you can't use "std" you can't use MemoServ.
## Recent Updates
- The Python frontend is up, but is still a work in progress
- Corrected the modify-record algorithm and added to it so that a mark update or a type update can be done
- Changed the add-record functionality to handle many additions at once
- Fixed the import functionality so it won't try to insert records one at time
## What's Next
- I code on an AMD A8-driven system, so finding places to trim resource usage are on my list.
- Need to implement an "exiting" signal on the DBus to facilitate frontend operation.
- Clean up the conf.json file to get rid of relics from MemoBook.
