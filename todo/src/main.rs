use std::{env, fs::{File, read_to_string}, io::Write};
use chrono::{DateTime, NaiveDate, NaiveTime, Utc, NaiveDateTime};
const CONFIG_PATH: &'static str = "todo_items.txt";

#[derive (Debug)]
struct Item {
    name: String,
    due_date: DateTime<Utc>,
    time_left: u64,
}

impl Item {

    //inputted dates should be formatted as YYYY.MM.DD
    fn build(name: String, due_date: String) -> Item { 
        let now = Utc::now();
        let inputs: Vec<i32> = due_date.split('.')
                            .map(|x| x.parse::<i32>().unwrap()) //add error handling here
                            .collect();
        
        println!("{}", due_date);

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
                        .open(CONFIG_PATH).unwrap();
    
    file.write_all(b"Items:\n").unwrap();
}

// reads from todo_items.txt and returns the entries as a vector of Item objects, which is later parsed again to be read by the user
fn items_as_vec() -> Vec<Item> {
    let file = read_to_string(CONFIG_PATH).unwrap();
    let lines: Vec<&str> = file.lines().collect();
    let mut items: Vec<Item> = Vec::new();

    let to_date = |line: &str| -> String {
        let date = line[11..14].to_owned() + &line[16..17] + &line[19..20];
        date
    };

    for i in 2..((lines.len()+ 1) / 2) {
        let s = lines[i];
        let len = s.len();
        let name = &s[1..(len - 1)];
        let date = to_date(lines[i + 1]);
        let new_item = Item::build(name.to_string(), date);

        items.push(new_item);
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
                let mut file = File::options().append(true)
                                    .create(false)
                                    .open(CONFIG_PATH)
                                    .unwrap();

                // evaluates whether the month is a single digit, and adds a zero to the start if it is; for ease in parsing later.
                let month_eval = |opt: String| {
                    let opt_date:Vec<&str> = opt.split('.').collect();
                    if opt_date[1].len() == 1 {
                        
                        let month_with_zero = "0".to_owned() + &(opt_date[1]);
                        let new_date: String = opt_date[0].to_owned() + "." + &month_with_zero + "." + opt_date[2];
                        return new_date;
                        
                    } else {
                        return opt;
                    }
                };

                let entry: String = "[".to_owned() + &item.name + "]" + "\n\tDue Date: " + &month_eval(opt2.to_string()) + "\n";     
            
                file.write_all(entry.as_bytes()).unwrap();
                
            },
            _ => println!("Wrong number of arguments"),
                
        },
        [_, cmd] => match cmd.as_str() {
            "-l" | "--list" => {
                let list = items_as_vec();
                
                for i in 0..list.len() {
                    println!("{}: {} -> Due at {}", i + 1, list[i].name, list[i].due_date);
                }
            },
            _ => println!("Not a valid argument")
        },

        _ => println!("Wrong number of arguments"),
        
    }
}
