use std::{env, fs::{File}, io::{Write, ErrorKind}};
use utils::{Item, List};
pub mod utils;
const CONFIG_PATH: &'static str = "todo_items.txt";

fn init() {
    File::options().write(true)
                    .open(CONFIG_PATH).unwrap_or_else(|x| {
                        if x.kind() == ErrorKind::NotFound {
                            let mut f = File::create(CONFIG_PATH).unwrap();
                            f.write_all(b"Items:\n").unwrap(); f
                        } else {
                            panic!("Error creating the file")
                        }
                    });
}

fn main() {
    let args: Vec<String> = env::args().collect();
    init();
    
    match &args[..] {
        [_, cmd, opt1, opt2] => match cmd.as_str() {
            //add success message to cmd line for new entry
            "-n" | "--new" => {
                let mut file = File::options().append(true)
                                    .create(false)
                                    .open(CONFIG_PATH)
                                    .unwrap();

                let mut item = Item::build(String::from(opt1), String::from(opt2));
                match item.update_time() {
                    Ok(_) => {
                        // evaluates whether the month is a single digit, and adds a zero to the start if it is; for ease in parsing later.
                        let eval = |opt: String| {
                            let mut opt_date:Vec<String> = opt.split('.').map(|x| x.parse::<String>().unwrap()).collect();
                            
                            for i in 1..opt_date.len() {
                                if opt_date[i].len() == 1 {
                                    // let month_with_zero = "0".to_owned() + &(opt_date[1]);
                                    opt_date[i] = "0".to_owned() + &opt_date[i];
                                }
                            }
                            return opt_date.join("."); 
                        };
        
                        let entry: String = "[".to_owned() + &item.name + "] " + &eval(opt2.to_string()) + "\n";     
                    
                        file.write_all(entry.as_bytes()).unwrap();
                    },
                    Err(_) => println!("Invalid date"),
                }
            },
            _ => println!("Wrong number of arguments"),
                
        },
        [_, cmd] => match cmd.as_str() {
            "-l" | "--list" => {
                match List::build_from_path(CONFIG_PATH) {
                    Ok(mut list) => {
                        list.update().unwrap(); //for now...
                        for i in 0..list.items.len() {
                            println!("{}: [{}]\t-> Due at {}", i + 1, list.items[i].name, list.items[i].due_date);
                        }
                    },
                    Err(_) => println!("No items to display")
                };

            },
            _ => println!("Not a valid argument")
        },

        _ => println!("Wrong number of arguments"),
        
    }
}