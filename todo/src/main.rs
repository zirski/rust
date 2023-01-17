use std::{env, fs::{File, read_to_string}, io::Write};
use chrono::{DateTime, NaiveDate, NaiveTime, Utc, NaiveDateTime};

struct Item {
    name: String,
    due_date: DateTime<Utc>,
    time_left: u64,
}

impl Item {

    //inputted dates should be formatted as YYYY.MM.DD
    fn build(name: String, due_date: String) -> Item { 
        let now = Utc::now();
        let inputs: Vec<i32> = due_date.split(|d| d == '.' || d == '-' || d == '_')
                            .map(|x| x.parse::<i32>().unwrap()) //add error handling here
                            .collect();
    
        let d: NaiveDate = NaiveDate::from_ymd_opt(inputs[0], inputs[1] as u32, inputs[2] as u32).expect("Invalid Date");
        let t: NaiveTime = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        let dt: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::new(d, t), Utc);
    
        let new_item = Item {
            name: name.to_string(),
            due_date: dt,
            time_left: dt.signed_duration_since(now).to_std().unwrap().as_secs(),
        };

        new_item
    }

    fn check(&mut self) {
        let now = Utc::now();

        self.time_left = now.signed_duration_since(self.due_date).to_std().unwrap().as_secs();
    }
}

fn init() {
    let mut file = File::options()
                        .create(true)
                        .write(true)
                        .open("todo_config.txt").unwrap();
    
    file.write_all(b"Items:\n\n").unwrap();
}

fn items_as_vec() -> Vec<Item> {
    let file = read_to_string("todo_config.txt").unwrap();
    let lines: Vec<&str> = file.lines().collect();
    let mut items: Vec<Item> = Vec::new();
    
    let parse_entry_and_create= |e: String| {
        let entries: Vec<&str> = e.split(" ").collect();
        let mut new_item = Item::build(String::from(entries[0]), String::from(entries[1]));
        new_item.check();

        new_item
    };

    for i in 0..(lines.len() / 3) {
        if i == 0 {
            let entry = lines[1].to_owned() + lines[2] + lines[3];
            items.push(parse_entry_and_create(entry));
        } else {
            let entry = lines[i * 3 + 1].to_owned() + " " + lines[i * 3 + 2] + " " + lines[i * 3 + 3];
            items.push(parse_entry_and_create(entry));
        }
    }

    return items
}

fn main() {
    let args: Vec<String> = env::args().collect();
    init();

    match &args[..] {
        [_, cmd, opt1, opt2] => match cmd.as_str() {
                //add success message to cmd line for new entry
            "-n" | "--new" => {
                let item = Item::build(String::from(opt1), String::from(opt2));
                
                let mut file = File::options().append(true).open("todo_config.txt").unwrap();
                let entry: String = "[".to_owned() + &item.name + "]" + "\n\tDue Date: " + opt2 + "\n\tTime left: " + &item.time_left.to_string() + "\n";     
            
                file.write_all(entry.as_bytes()).unwrap();
                
            },
            _ => println!("Error: wrong number of arguments"),
                
        }
        _ => {
            let list = items_as_vec();
                
            for i in 0..list.len() {
                println!("{}: {} -> Due at {}", i + 1, list[i].name, list[i].due_date);
            }
        },
        
    }
}
