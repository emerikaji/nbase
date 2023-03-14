use std::{fs, env, str::from_utf8};

const SEP: u8 = 0x3b;

////DATABASE////
pub struct DB{
    name: String,
    collection: Vec<(String,Vec<u8>)>
}


////DATABASE METHODS////
impl DB {
    pub fn new(s: String) -> DB {
        let new_db = DB {
            name: s,
            collection: Vec::new()
        };
        new_db
    }

    pub fn open(s: String, c: Vec<(String,Vec<u8>)>) -> DB {
        let open_db = DB {
            name: s,
            collection: c
        };
        open_db
    }

    pub fn get(&self, k: String) -> (Option<Vec<u8>>,usize) {
        for (i,pair) in self.collection.iter().enumerate() {
            if pair.0 == k {
                return (Some(pair.1.clone()),i)
            }
        }
        return (None,0)
    }

    pub fn add(&mut self, k: String, v: Vec<u8>) -> bool {
        if self.get(k.clone()).0 == None {
            self.collection.push((k,v));
            return true
        }
        false
    }

    pub fn upd(&mut self, k: String, v: Vec<u8>) -> bool {
        let (o, i) = self.get(k);
        if o == None {
            return false
        }
        self.collection[i].1 = v;
        true
    }

    pub fn rem(&mut self, k: String) -> bool {
        let (o, i) = self.get(k);
        if o == None {      
            return false
        }
        self.collection.remove(i);
        true
    }
}


////FILE METHODS////
fn write_db_to_file(db: &DB) {
    let mut contents = Vec::<u8>::new();
    let name = db.name.clone() + ".ndb";
    for p in &db.collection {
        contents.append(&mut p.0.as_bytes().to_vec());
        contents.push(SEP);
        contents.append(&mut p.1.to_vec());
        contents.push(SEP);
    };
    let mut lzo = minilzo_rs::LZO::init().unwrap();
    let mut out: Vec<u8> = contents.len().to_be_bytes().to_vec();
    out.append(&mut lzo.compress(&contents).unwrap());
    fs::write(name, out).expect("Failed to write");
}

fn read_db_from_file(s: String) -> DB {
    let name = s.clone() + ".ndb";
    let out = fs::read_to_string(name).expect("Failed to read").as_bytes().to_vec();
    let contentsize = usize::from_be_bytes(out[0..8].try_into().expect("Error reading header"));
    let lzo = minilzo_rs::LZO::init().unwrap();
    let contents = lzo.decompress_safe(&out[8..], contentsize).unwrap();
    let mut collection: Vec<(String,Vec<u8>)> = Vec::new();
    let mut mode = true;
    let mut key = String::new();
    let mut value = Vec::<u8>::new();
    for i in contents {
        if mode {
            if i == SEP {
                mode = false;
                continue;
            }
            key.push(i as char)
        } else {
            if i == SEP {
                mode = true;
                collection.push((key.clone(),value.clone()));
                key = String::new();
                value = Vec::<u8>::new();
                continue;
            }
            value.push(i)
        }
    }
    return DB::open(s, collection);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Please give a valid command: init [db name], [db name] push [key] [value], [db name] pull [key], [db name] edit [key] [value], [db name] pop [key]");
        return
    }
    if args[1] == "init" {
        if args.len() != 3 {
            println!("Wrong number of arguments");
            return
        }
        println!("Creating {}.ndb . . .",args[2]);
        write_db_to_file(&DB::new(args[2].clone()));
        println!("Success!");
        return
    }
    println!("Trying to read from {}.ndb . . .",args[1]);
    let mut db = read_db_from_file(args[1].clone());
    println!("Read successful!");
    match args[2].as_str() {
        "push"=>{
            if args.len() != 5 {
                println!("Wrong number of arguments");
                return
            }
            if db.add(args[3].clone(), args[4].as_bytes().to_vec()) {
                println!("Push successful, writing into file . . .");
                write_db_to_file(&db)
            } else {
                println!("Push ignored");
            }
        },
        "multipush"=>{
            if args.len() < 5 {
                println!("Wrong number of arguments");
                return
            } 
            if args.len()%2 != 1 {
                println!("Wrong number of arguments");
                return
            }
            if args.len() == 5 {
                println!("Please use push instead for better performance");
            } 
            let mut j = 0;
            let mut w = false;
            for i in (3..args.len()).step_by(2) {
                j += 1;
                if db.add(args[i].clone(), args[i+1].as_bytes().to_vec()) {
                    println!("Push {} successful", j);
                    w = true;
                } else {
                    println!("Push {} ignored", j);
                }
            }
            if w {
                println!("Multipush successful, writing into file . . .");
                write_db_to_file(&db)
            } else {
                println!("Multipush ignored");
            }  
        },
        "pull"=>{
            if args.len() != 4 {
                println!("Wrong number of arguments");
                return
            }
            let (o, _) = db.get(args[3].clone());
            if o != None {
                println!("{}", from_utf8(&o.unwrap()).expect("Failed to convert data to utf8 string"))
            } else {
                println!("No value associated with key {}", args[3])
            }
        },
        "edit"=>{
            if args.len() != 5 {
                println!("Wrong number of arguments");
                return
            }
            if db.upd(args[3].clone(), args[4].as_bytes().to_vec()) {
                println!("Edit successful, writing into file . . .");
                write_db_to_file(&db)
            } else {
                println!("Edit ignored, does the key exist?")
            }
        },
        "pop"=>{
            if args.len() != 4 {
                println!("Wrong number of arguments");
                return
            }
            if db.rem(args[3].clone()) {
                println!("Pop successful, writing into file . . .");
                write_db_to_file(&db)
            } else {
                println!("Pop ignored, does the key exist?")
            }
        },
        _=>println!("Invalid argument: {}",args[2])
    }
}

