# MemoServ
Backend service for MemoBook, communication via DBus. A work in progress, but is functional. Is designed to run as a low-resource service that any application can access so long as that application can send/receive messages via DBus.
## MemoBook?
My other project is now defunct, as I was migrating from Tk to Wx, but ran out of time and interest, plus the concept was too limiting. Instead, a frontend for this service is on the way, written in Python using WxPython. 
## Requirements
- Rust: I'm still new too Rust and haven't investigated what the minimum version of Rust is required.
- Rust libraries: uses crates json, zbus for dbus communications, rusqlite for sqlite3, tokio for async runtime, chrono for dates & times, and sha256 for SHA256 sums. And maybe a few others I'm forgetting.
- Environment: while I have tested a number of features in Windows 11, deal-breakers for Windows environments include not having DBus or Sqlite3 installed. Grep functionality would also be broken without grep, though this not critical to the service. Hence, typical Linux-based distros should be fine (Ubuntu, Fedora, Pop!, and BunsenLabs to name a few). This is 100% Rust, so the aforementioned services/utilities and the availability of Rust on a system are what determines whether MemoService will run.
## What's Next
- I code on an AMD A8-driven system, so the import functionality and its SHA256 sums is quite time-consuming. This is the primary problem for the frontend. Hence, I need to replace recursive directory processing with something parallelized.
- Need to implement an "exiting" signal on the DBus to facilitate frontend operation.
- A "modify" feature or two needs to be implemented for the frontend.
- Clean up the conf.json file to get rid of relics from MemoBook.
