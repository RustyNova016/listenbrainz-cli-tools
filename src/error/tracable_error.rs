use core::fmt::Display;
use std::backtrace::Backtrace;
use std::fmt::Debug;
use std::sync::Mutex;

use color_eyre::owo_colors::OwoColorize;
use directories::BaseDirs;

pub struct TracingError<T> {
    human_trace: Mutex<Vec<Box<dyn Display>>>,
    pub kind: T,
    trace: Backtrace,
}

impl<T> TracingError<T> {
    pub fn new(kind: T) -> Self {
        Self {
            kind,
            human_trace: Mutex::new(Vec::new()),
            trace: Backtrace::capture(),
        }
    }

    pub fn context(&self, msg: impl Display + 'static) {
        let mut comms = self.human_trace.lock().expect("Poisoned lock!");
        comms.push(Box::new(msg));
    }
}

impl<T> Display for TracingError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "\n Fatal error! ".black().bold().on_red())?;
        writeln!(f)?;
        writeln!(f)?;

        writeln!(
            f,
            "Something went wrong in the program and it couldn't recover"
        )?;
        writeln!(f)?;
        writeln!(f, "The error stack is as follows:")?;
        for (num, comment) in self
            .human_trace
            .lock()
            .expect("The lock is poisoned")
            .iter()
            .enumerate()
        {
            writeln!(f, "   [{num}] {comment}")?;
        }

        writeln!(f)?;
        writeln!(f)?;
        writeln!(f, "{}", self.trace)?;

        Ok(())
    }
}

//impl<T> From<T>

#[tokio::test]
#[serial_test::serial]
async fn error_test() {
    let err = TracingError::new("Heelo");
    err.context("This happened here");
    err.context("because we did this");
    err.context("I'm just a passthrough don't mind me".green());

    println!("{}", err)
}