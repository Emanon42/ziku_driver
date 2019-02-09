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
    
~~WARNING: parser/mod.rs is WIP, now it only gives ```syntax error``` as response    
Other components are ready to use~~    
Now it's a functional demo    
TODO list:    
 - "documentize" all prompt texts and instruction arugments (xml or json)    
 - update data infrastructure to LSM tree    
 - implement mutex lock and multithread    
     
 Benchmark added, for current version:
```
> benchmark
set 1,000,000 random data costs: 10.447723624s
scan in 1,000,000 random data costs: 197.789551ms
get 1,000,000 random data costs: 6.08817167s
```
