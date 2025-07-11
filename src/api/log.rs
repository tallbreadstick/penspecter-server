use std::error::Error;
use chrono::Local;
use color_print::cprintln;

pub enum LogType {
    SETUP,
    HTTP
}

pub fn log(log_type: LogType, message: &str) {
    match log_type {
        LogType::SETUP => cprintln!("<yellow>{}</yellow><blue>[SETUP]</blue><green>[LOG]</green>: {}", now(), message),
        LogType::HTTP => cprintln!("<yellow>{}</yellow><cyan>[HTTP]</cyan><green>[LOG]</green>: {}", now(), message)
    }
}

pub fn errlog(log_type: LogType, error: &impl Error) {
    match log_type {
        LogType::SETUP => cprintln!("<yellow>{}</yellow><blue>[SETUP]</blue><red>[ERROR]</red>: {}", now(), error),
        LogType::HTTP => cprintln!("<yellow>{}</yellow><cyan>[HTTP]</cyan><red>[ERROR]</red>: {}", now(), error)
    }
}

fn now() -> String {
    Local::now()
        .format("[%d/%m/%y-%I:%M%p]")
        .to_string()
}