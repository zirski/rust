use std::{fmt, env, fs::{File, read_to_string}, io::{Write, ErrorKind}, collections::HashMap};
use list::Item;
pub mod list;
const CONFIG_PATH: &'static str = "todo_items.txt";

pub struct EmptyListError;

impl fmt::Display for EmptyListError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Attempted to parse empty list")
    }
}

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

// reads from todo_items.txt and returns the entries as a vector of Item objects, which is later parsed again to be read by the user
fn items_as_vec(str: &str) -> Result<Vec<Item>, EmptyListError> {
    let lines: Vec<&str> = str.lines().collect();
    // .map(|l| return l[l.len() - 10..l.len() - 1])
    let mut items: Vec<Item> = Vec::new();

    for i in 1..lines.len() {
        let chars: Vec<char> = lines[i].chars().collect();
        let mut item_attr = HashMap::new();
        
        for j in 1..chars.len() {
            if chars[j] == ']' {
                item_attr.insert("name", (lines[i][1..j]).to_string());
                item_attr.insert("y", (lines[i][(j + 2)..(j + 6)]).to_string());
                item_attr.insert("m", (lines[i][(j + 7)..(j + 9)]).to_string());
                item_attr.insert("d", (lines[i][(j + 10)..(j + 12)]).to_string());
                break;
            }
        }
        if item_attr.len() == 0 {
            return Err(EmptyListError);
        }

        let new_item = Item::build(item_attr.get("name").unwrap().to_string(), 
                                        item_attr.get("y").unwrap().to_string() + "." + item_attr.get("m").unwrap() + "." + item_attr.get("d").unwrap());
        items.push(new_item);
    }

    Ok(items)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    init();
    let mut file = File::options().append(true)
                        .create(false)
                        .open(CONFIG_PATH)
                        .unwrap();

    match &args[..] {
        [_, cmd, opt1, opt2] => match cmd.as_str() {
                //add success message to cmd line for new entry
            "-n" | "--new" => {
                let item = Item::build(String::from(opt1), String::from(opt2));

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
            _ => println!("Wrong number of arguments"),
                
        },
        [_, cmd] => match cmd.as_str() {
            "-l" | "--list" => {
                let file_as_str = read_to_string(CONFIG_PATH).expect("Invalid File contents");
                match items_as_vec(&file_as_str) {
                    Ok(vec) => {
                        for i in 0..vec.len() {
                            println!("{}: [{}]\t-> Due at {}", i + 1, vec[i].name, vec[i].due_date);
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn items_as_vec_length_test() {
        let mut file = File::options().create(true).open("test.txt").unwrap();
        file.write_all(b"Items:\n[test]\n\tDue Date: 2023.02.01\n[test]\n\ntDue Date: 2023.01.02)").unwrap();


    }
}