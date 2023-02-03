use chrono::{DateTime, NaiveDate, NaiveTime, Utc, NaiveDateTime};

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

    pub fn update(&mut self) {
        let now = Utc::now();

        self.time_left = now.signed_duration_since(self.due_date).to_std().unwrap().as_secs();
    }
}