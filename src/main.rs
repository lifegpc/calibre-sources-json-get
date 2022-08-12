mod opts;
mod try_err;

use std::io::Read;

fn main() {
    let opt = match opts::Opts::parse() {
        Ok(opt) => opt,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    };
    let mut f = match std::fs::File::open(&opt.json) {
        Ok(f) => f,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    };
    let mut txt = String::new();
    match f.read_to_string(&mut txt) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    }
    let json = match json::parse(&txt) {
        Ok(json) => json,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    };
    for i in json.members() {
        let name = i["name"].as_str();
        match name {
            Some(name) => {
                if name != opt.name {
                    continue;
                }
            }
            None => {
                continue;
            }
        }
        match opt.cmd {
            opts::Command::Filename => {
                match i["unix"]["filename"].as_str() {
                    Some(filename) => {
                        println!("{}", filename);
                        std::process::exit(0);
                    }
                    None => {
                        println!("Failed to get file name.");
                        std::process::exit(1);
                    }
                }
            }
            opts::Command::Hash => {
                match i["unix"]["hash"].as_str() {
                    Some(hash) => {
                        println!("{}", hash);
                        std::process::exit(0);
                    }
                    None => {
                        println!("Failed to get hash.");
                        std::process::exit(1);
                    }
                }
            }
            opts::Command::Urls(ind) => {
                match i["unix"]["urls"][ind as usize].as_str() {
                    Some(url) => {
                        let u = if url.contains("{filename}") {
                            match i["unix"]["filename"].as_str() {
                                Some(filename) => {
                                    url.replace("{filename}", filename)
                                }
                                None => {
                                    url.to_owned()
                                }
                            }
                        } else {
                            url.to_owned()
                        };
                        println!("{}", u);
                        std::process::exit(0);
                    }
                    None => {
                        println!("Failed to get url.");
                        std::process::exit(1);
                    }
                }
            }
        }
    }
    println!("Failed to find dependecy.");
    std::process::exit(1);
}
