use crate::rout_info::RoutInfo;
use std::io;

pub fn get_max_requests_count() -> isize {
    let mut line = String::new();
    let error_message = "[Main] Expected a number greater than zero.";
    println!("[Main] Enter maximum amount of parallel requests to web services:");
    io::stdin()
        .read_line(&mut line)
        .expect("failed to read from stdin");
    return match line.trim().parse::<u32>() {
        Ok(i) => {
            if i > 0 {
                i as isize
            } else {
                println!("{}", error_message);
                get_max_requests_count()
            }
        }
        Err(..) => {
            println!("{}", error_message);
            get_max_requests_count()
        }
    };
}

pub fn fake_sleep(laps: isize) {
    for _i in 0..laps {
        for _j in 0..laps {}
    }
}

pub fn stringify_top_10(top_10: Vec<&RoutInfo>) -> String {
    let mut msg = "Top 10 more requested routs \n".to_string();
    for i in top_10 {
        msg += &*format!("Rout:{}, N:{}\n", i.rout, i.number_of_trips)
    }
    msg
}
