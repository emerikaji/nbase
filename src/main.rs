use std::{env, str::from_utf8};
mod database;
use database::DB;

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
        DB::new(args[2].clone()).write_db_to_file();
        println!("Success!");
        return
    }
    println!("Trying to read from {}.ndb . . .",args[1]);
    let mut db = DB::read_db_from_file(args[1].clone());
    println!("Read successful!");
    match args[2].as_str() {
        "json"=>{
            print!("{}", db.to_json())
        }
        "push"=>{
            if args.len() != 5 {
                println!("Wrong number of arguments");
                return
            }
            if db.add(args[3].clone(), args[4].as_bytes().to_vec()) {
                println!("Push successful, writing into file . . .");
                db.write_db_to_file()
            } else {
                println!("Push ignored");
            }
        },
        "pushmany"=>{
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
                println!("Push many successful, writing into file . . .");
                db.write_db_to_file()
            } else {
                println!("Push many ignored");
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
                db.write_db_to_file()
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
                db.write_db_to_file()
            } else {
                println!("Pop ignored, does the key exist?")
            }
        },
        _=>println!("Invalid argument: {}",args[2])
    }
}

