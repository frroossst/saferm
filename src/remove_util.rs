use std::path::{Path, PathBuf};
use crate::trashcan_config;
use std::process::Command;
use glob::glob;



pub fn move_file_to_trash(file_to_be_trashed: PathBuf)
    {
    let trashcan_str = trashcan_config::get_trashcan_location();
    let trashcan_path = Path::new(&trashcan_str);

    let status = Command::new("mv")
        .arg("-f")
        .arg(&file_to_be_trashed.to_owned())
        .arg(trashcan_path)
        .output();

    match status
        {
        Ok(s) =>
            {
            let cmd_err_status = String::from_utf8(s.stderr);
            match cmd_err_status
                {
                Ok(conv) =>
                    {
                    if conv.trim().is_empty()
                        {
                        println!("removing {:?}", file_to_be_trashed);
                        }
                    else 
                        {
                        println!("unable to move {:?}", file_to_be_trashed);
                        }
                    }
                Err(_) =>
                    {
                    println!("unable to move {:?}", file_to_be_trashed);
                    }
                }
            }
        Err(_) =>
            {
            println!("unable to move {:?}", file_to_be_trashed);
            std::process::exit(1);
            }
        }

    }

pub fn move_pattern_to_trash(pattern: &str)
    {
    for entry in glob(pattern).unwrap()
        {
        match entry
            {
            Ok(p) => 
                { 
                println!("removing: "); 
                print!("{:?}, ", p.display()); 
                move_file_to_trash(p);
                },
            Err(_) => 
                { 
                std::process::exit(1);
                },
            }
        }
    }