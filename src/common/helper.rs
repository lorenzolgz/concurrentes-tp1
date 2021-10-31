use crate::rout_info::RoutInfo;
use std::{fs, io};

pub fn get_max_requests_count() -> isize {
    let mut line = String::new();
    let default_max_requests = 5;
    let error_message = "[Main] Expected a number greater than zero.";
    println!(
        "[Main] Enter maximum amount of parallel requests (or press enter to use default \"{}\")",
        default_max_requests
    );
    io::stdin()
        .read_line(&mut line)
        .expect("failed to read from stdin");
    if line.trim().is_empty() {
        println!(
            "[Main] Using default max parallel request \"{}\"",
            default_max_requests
        );
        default_max_requests
    } else {
        match line.trim().parse::<u32>() {
            Ok(max_parallel_count) => {
                if max_parallel_count > 0 {
                    max_parallel_count as isize
                } else {
                    println!("{}", error_message);
                    get_max_requests_count()
                }
            }
            Err(..) => {
                println!("{}", error_message);
                get_max_requests_count()
            }
        }
    }
}

pub fn get_csv_file_path() -> String {
    let mut line = String::new();
    let default_path = "./resources/reservations.csv".to_string();
    let error_message = "[Main] Expected a string for the path";
    println!(
        "[Main] Enter path for csv file (or press enter to use default \"{}\")",
        default_path
    );
    io::stdin()
        .read_line(&mut line)
        .expect("failed to read from stdin");
    return match line.trim().parse::<String>() {
        Ok(parsed_line) => {
            if parsed_line.is_empty() {
                println!("[Main] Using default csv path \"{}\"", default_path);
                default_path
            } else {
                fs::read_to_string(parsed_line).unwrap_or_else(|error| {
                    println!(
                        "[Main] Expected a valid path but {} was thrown, please try again",
                        error
                    );
                    get_csv_file_path()
                })
            }
        }
        Err(..) => {
            println!("{}", error_message);
            get_csv_file_path()
        }
    };
}

pub fn get_log_output_path(part: String) -> String {
    let mut line = String::new();
    let default_path = format!("./logs/{}/", part);
    let error_message = "[Main] Expected a string for the path";
    println!(
        "[Main] Enter path for log output (or press enter to use default \"{}\")",
        default_path
    );
    io::stdin()
        .read_line(&mut line)
        .expect("failed to read from stdin");
    return match line.trim().parse::<String>() {
        Ok(parsed_line) => {
            if parsed_line.is_empty() {
                println!("[Main] Using default log output path \"{}\"", default_path);
                fs::create_dir_all(default_path.clone()).expect("Error creating logger directory");
                default_path
            } else {
                let file_path = fs::read_to_string(parsed_line).unwrap_or_else(|error| {
                    println!(
                        "[Main] Expected a valid path but {} was thrown, please try again",
                        error
                    );
                    get_log_output_path(part)
                });
                fs::create_dir_all(file_path.clone()).expect("Error creating logger directory");
                file_path
            }
        }
        Err(..) => {
            println!("{}", error_message);
            get_log_output_path(part)
        }
    };
}

pub fn get_log_file_name(base_path: String) -> String {
    format!(
        "{}{}.txt",
        base_path,
        chrono::offset::Local::now()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
    )
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
