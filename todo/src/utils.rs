use core::fmt;
use std::{fs::{read_to_string}, collections::HashMap};
use chrono::{DateTime, NaiveDate, NaiveTime, Utc, NaiveDateTime, OutOfRangeError};

pub struct EmptyListError;

impl fmt::Display for EmptyListError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Attempted to parse empty list")
    }
}
pub struct List {
    pub items: Vec<Item>,
}

impl List {
    pub fn build_from_path(path: &str) -> Result<List, EmptyListError> {
        let file_as_str = read_to_string(path).expect("Invalid File contents");

        let lines: Vec<&str> = file_as_str.lines().collect();
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
    
        Ok(List { items: items })
    }

    pub fn add(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn remove(&mut self, index: usize) {
        self.items.remove(index);
    }

    pub fn update(&mut self) -> Result<(), OutOfRangeError> {
        for i in 0..self.items.len() {
            self.items[i].update_time()?;
        }

        self.items.sort_by(|a, b| a.time_left.cmp(&b.time_left));
        Ok(())
    }
}

pub struct Item {
    pub name: String,
    pub due_date: DateTime<Utc>,
    pub time_left: u64,
}

impl Item {
    //inputted dates should be formatted as YYYY.MM.DD
    pub fn build(name: String, due_date: String) -> Item { 
        let now = Utc::now();
        let inputs: Vec<i32> = due_date.split('.')
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

    pub fn update_time(&mut self) -> Result<(), OutOfRangeError> {
        let now = Utc::now();

        self.time_left = self.due_date.signed_duration_since(now).to_std()?.as_secs();

        Ok(())
    }
}

