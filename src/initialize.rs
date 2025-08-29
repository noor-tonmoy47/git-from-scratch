use std::fs;

pub fn init(){
    fs::create_dir(".stalker").unwrap();
    fs::create_dir(".stalker/objects").unwrap();
    fs::create_dir(".stalker/refs").unwrap();
    fs::write(".stalker/HEAD", "ref : refs/heads/main\n").unwrap();
    println!("Assigned a Stalker for this project")
}