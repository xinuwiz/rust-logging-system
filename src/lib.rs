pub mod logger {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::Write;
    use std::sync::{Mutex};
    use crate::logger::Level::{ERROR, INFO, WARN};

    #[macro_export]
    macro_rules! info {
        ( $ref:expr, $message:expr ) => {
            let log = crate::logger::Log::new($message, crate::logger::Level::INFO);
            $ref.add(log);
        };
    }

    #[macro_export]
    macro_rules! warn {
        ( $ref:expr, $message:expr ) => {
            let log = crate::logger::Log::new($message, crate::logger::Level::WARN);
            $ref.add(log);
        };
    }

    #[macro_export]
    macro_rules! error {
        ( $ref:expr, $message:expr ) => {
            let log = crate::logger::Log::new($message, crate::logger::Level::ERROR);
            $ref.add(log);
        };
    }

    #[derive(Debug)]
    pub enum Level {
        INFO,
        WARN,
        ERROR
    }

    pub struct Log {
        message: String,
        level: Level
    }

    impl Log {
        pub fn new(message: String, level: Level) -> Log {
            Log {
                message,
                level
            }
        }
    }

    struct Manager {
        loggers: HashMap<String, Stateful>
    }

    pub struct Stateful {
        messages: Mutex<Vec<Log>>
    }

    impl Stateful {
        pub fn new() -> Stateful {
            Stateful {
                messages: Mutex::new(Vec::new())
            }
        }

        pub fn add(&self, log: Log) -> () {
            match self.messages.lock() {
                Ok(mut messages) => {
                    println!("{}", log.message);
                    messages.push(log);
                }
                _ => ()
            }
        }

        pub fn to_file(&self) -> () {
            let mut file = File::create("log.txt").unwrap();

            for log in self.messages.lock().unwrap().iter() {
                let x = format!("{:?}: {}\n", log.level, log.message);
                file.write(x.as_bytes()).unwrap();
            }

            file.flush().unwrap();
        }
    }
}
