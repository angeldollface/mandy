/*
MANDY EXTRAS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the method
/// to check whether
/// a file exists.
use coutils::file_is;

/// Importing Mandy's default
/// error structure.
use merrors::MandyError;

/// Importing the method
/// to create a file.
use coutils::create_file;

/// Importing the method to
/// write a string to a file.
use coutils::write_to_file;

/// A data structure to hold
/// information about a logging
/// event.
#[derive(Clone, Debug)]
pub struct LogMessage{
    pub msg: String,
    pub time: String,
    pub dir: String
}

/// Implementing useful
/// methods for the "LogMessage"
/// structure.
impl LogMessage{

    /// This function creates
    /// a new instance of the
    /// "LogMessage" structure.
    pub fn new(
        msg: &str,
        time: &String,
        dir: &String
    ) -> LogMessage {
        return LogMessage{
            msg: msg.to_string(),
            time: time.to_owned(),
            dir: dir.to_owned()
        };
    }

    /// This function creates
    /// a string representation of the
    /// "LogMessage" structure.
    pub fn to_string(&self) -> String {
        return format!(
            "{}@{}: {}",
            &self.dir,
            &self.time,
            &self.msg
        )
    }
}

/// A data structure to hold
/// information about a list
/// of logging events.
#[derive(Clone, Debug)]
pub struct Log {
    log_messages: Vec<LogMessage>
}

/// Implementing useful
/// methods for the "Log"
/// structure.
impl Log {

    /// This function creates
    /// a new instance of the
    /// "Log" structure.
    pub fn new(log_messages: &Vec<LogMessage>) -> Log {
        return Log{
            log_messages: log_messages.to_owned()
        };
    }

    /// This function creates
    /// a string representation of the
    /// "Log" structure.
    pub fn to_string(&self) -> String {
        let mut result: Vec<String> = Vec::new();
        for log_msg in &self.log_messages {
            result.push(log_msg.to_string());
        }
        let final_log: String = result.join("\n");
        return final_log;
    }
}

/// Creates log messages of the site-build process.
pub fn create_log(msgs: &Vec<LogMessage>, dir: &String) -> Result<(), MandyError>{
    let fname: String = format!("{}/dist/build.log", dir);
    if file_is(&fname){
        let e: String = format!("Log file inside \"{}\" already exists.", dir);
        return Err::<(), MandyError>(
            MandyError::new(
                &e.to_string()
            )
        );
    }
    else {
        let msg_string: String = Log::new(msgs).to_string();
        match create_file(&fname){
            Ok(_x) => {}
            Err(e) => {
                return Err::<(), MandyError>(
                    MandyError::new(
                        &e.to_string()
                    )
                );
            }
        };
        match write_to_file(&fname, &msg_string){
            Ok(_x) => {}
            Err(e) => {
                return Err::<(), MandyError>(
                    MandyError::new(
                        &e.to_string()
                    )
                );
            }
        };

    }
    Ok(())
}