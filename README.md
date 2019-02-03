# ziku_driver
Ema's prototype key-value server, in Rust    
Project structure:    
```
src/
├── database
│   └── mod.rs
├── interface
│   └── mod.rs
├── parser
│   └── mod.rs
└── main.rs
```
database/mod.rs -- KVdb infrastructure and wrapped api for better coupling and updating flexibility    
interface/mod.rs -- Communication interface between kvdb and outside TCP stream    
parser/mod.rs -- Utility class for ```interface``` to parse the instruction    
    
WARNING: parser/mod.rs is WIP, now it only gives ```syntax error``` as response    
Other components are ready to use
