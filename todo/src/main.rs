use std::env;
use chrono::*;
struct item {
    name: String,
    due_date: DateTime<Utc>,
    time_left: Duration,
}

impl item {
    fn new(name: String, due_date: &str) -> item { //inputted date should follow YY DD MM format
        let now = Utc::now();
        let inputs: Vec<i32> = due_date.split(" ")
                            .map(|x| x.parse::<i32>().unwrap())
                            .collect();

        let d: NaiveDate = NaiveDate::from_ymd_opt(inputs[0], inputs[2] as u32, inputs[1] as u32).unwrap();
        let t: NaiveTime = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        let dt: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::new(d, t), Utc);

        let new_item = item {
            name,
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
fn main() {
    let args: Vec<String> = env::args().collect();

    
}



