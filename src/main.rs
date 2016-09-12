#[macro_use]
extern crate nickel;
extern crate rand;
extern crate uuid;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate time;
extern crate serde;
extern crate serde_json;

use std::env;
use std::thread;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use log::{LogLevel, LogRecord, LogLevelFilter};
use env_logger::LogBuilder;
use rand::{Rng, thread_rng, sample};
use uuid::Uuid;
use nickel::status::StatusCode;
use nickel::{Nickel, QueryString, HttpRouter, Options};


include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

fn main() {
    let format = |record: &LogRecord| {
        let ts = time::now_utc();
        format!("{} {:010}ns - {} - {}",
                ts.strftime("%Y-%m-%dT%H:%M:%S").unwrap(),
                ts.tm_nsec,
                record.level(),
                record.args())
    };

    let mut builder = LogBuilder::new();
    builder.format(format).filter(Some("test_api"), LogLevelFilter::Debug);
    builder.init().unwrap();

    let mut server = Nickel::new();

    let mut global_data = Arc::new(RwLock::new(Box::new(Data::new())));

    let reader = global_data.clone();
    server.get("/test",
               middleware! { |_|
                   let r = match reader.read() {
                       Ok(r) => r,
                       Err(_) => panic!("lock error"),
                   };
                   let mut rng = thread_rng();
                   let sample = sample(&mut rng, &r.index, 10);
                   serde_json::to_string(&sample).unwrap_or("[]".to_string())
    });

    let stats_reader = global_data.clone();
    server.get("/stats",
               middleware! {|_|
                   let r = match stats_reader.read() {
                       Ok(r) => r,
                       Err(_) => panic!("lock error"),
                   };
                   serde_json::to_string( &DataSize { data_len: r.len() }).unwrap_or("null".to_string())
    });

    {
        let creator = global_data.clone();
        let mut d = creator.write().unwrap();
        for i in 0..1000 {
            let uuid = format!("{}", Uuid::new_v4());
            let data = rand::random::<u64>();
            d.insert(Box::new(DataEntry {
                id: uuid,
                data: data,
            }))
        }
    }

    let writer = global_data.clone();
    let updater = thread::spawn(move || {
        loop {
            debug!("Instantiataing new data structure");
            let mut new_data = Box::new(Data::new());
            {
                debug!("Copying the global data structure elements to the new one.");
                let w = writer.read().unwrap();
                for val in w.index.values() {
                    let v = val.clone();
                    new_data.insert(Box::new(DataEntry {
                        id: v.id.clone(),
                        data: v.data.clone(),
                    }));
                }
            }
            debug!("Updating global data structure to point to the new data.");
            {
                let mut w = writer.write().unwrap();
                *w = new_data;
            }
            debug!("Pausing");
            thread::sleep_ms(1000u32);
        }
    });

    server.options = Options::default()
        .output_on_listen(true)
        .thread_count(Some(8));

    server.listen("127.0.0.1:6767");
}
