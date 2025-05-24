use std::fmt::Debug;

use colored::Colorize;

pub trait Loggable<T, E> {
    fn log_err(self) -> T;
}

impl<T, E> Loggable<T, E> for Result<T, E>
where
    E: Debug,
{
    fn log_err(self) -> T {
        if let Some(err) = self.as_ref().err() {
            println!("{}", format!("Error: {:#?}", err).red());
        }
        self.unwrap()
    }
}
