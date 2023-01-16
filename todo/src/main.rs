use core::panic;
use std::{env, fs::{File, read_to_string}, io::{Write, ErrorKind}};
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
    
    file.write_all(b"Items:").unwrap();
}

fn load_config() {
    let lines:Vec<&str> = read_to_string("todo_config.txt").unwrap().lines().collect();
    
}

fn main() {
    let args: Vec<String> = env::args().collect();
    init();

    match &args[..] {
        [_, cmd, opt1, opt2] => match cmd.as_str() {
                "-n" | "--new" => {
                let item = Item::build(String::from(opt1), String::from(opt2));
                
                let mut file = File::options().append(true).open("todo_config.txt").unwrap();
                let entry: String = "Item: ".to_owned() + &item.name + "\n\tDue Date: " + opt2 + "\n\tTime left: " + &item.time_left.to_string() + "\n";     
            
                file.write_all(entry.as_bytes()).unwrap();
                
            },
            _ => println!("Error: argument format error"),
        },
        _ => println!("Error: wrong number of arguments"),

    }
}