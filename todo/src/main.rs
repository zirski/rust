use std::{env, fs};
use chrono::*;
struct item {
    name: String,
    due_date: DateTime<Utc>,
    time_left: Duration,
}

impl item {
    fn new(name: &str, due_date: &str) -> item { //inputted date should follow YY DD MM format
        let now = Utc::now();
        let inputs: Vec<i32> = due_date.split(" ")
                            .map(|x| x.parse::<i32>().unwrap())
                            .collect();

        let d: NaiveDate = NaiveDate::from_ymd_opt(inputs[0], inputs[2] as u32, inputs[1] as u32).unwrap();
        let t: NaiveTime = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        let dt: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::new(d, t), Utc);

        let new_item = item {
            name: name.to_string(),
            due_date: dt,
            time_left: now.signed_duration_since(dt),
        };

        new_item
    }

    fn check(&mut self) {
        let now = Utc::now();

        self.time_left = now.signed_duration_since(self.due_date);
    }
}

fn init_user() {
    let mut entries = fs::read_dir(".").unwrap().collect::<Vec<_>>();
    let mut filenames: Vec<String> = Vec::new();

    for entry in entries {
        filenames.push(entry.unwrap().file_name().into_string().unwrap());
    }

    for v in filenames {
        if v == String::from("todo_config.txt") {
            load_user();
        } else {
            create_config();
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    
}



