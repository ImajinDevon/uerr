use std::fmt::Display;

/// Unwrap the value contained within the given [Result] and return it, else print the contained
/// [std::io::Error] and exit the process.
///
/// See [std::process::exit(i32)].
pub fn unwrap_io<T>(msg: &str, res: std::io::Result<T>) -> T {
    let err = match res {
        Ok(v) => return v,
        Err(err) => err,
    };

    let code = err.raw_os_error().unwrap_or_else(|| {
        eprintln!("note: the error code could not be found for this variant; reverting to -1...");
        -1
    });
    err.into_user_err().print_all(msg).exit(code);
}

/// A human-readable error interface.
/// ```
/// use uerr::UserError;
///
/// UserError::from("could not open file")
///     .and_reason("The system cannot find the file specified.")
///     .and_help("Does this file exist?")
///     .print_all("uerr/error: ")
///     .exit(1);
/// ```
#[derive(Default)]
pub struct UserError {
    message: String,
    reasons: Vec<String>,
    help: Vec<String>,
}

impl UserError {
    fn enumerator<'a, I>(&self, i: I, first: &str, rest: &str)
    where
        I: IntoIterator<Item = &'a String>,
    {
        let mut it = i.into_iter();

        if let Some(f) = it.next() {
            eprintln!("{first}{f}");
        }

        for f in it {
            eprintln!("{rest}{f}");
        }
    }

    /// Exit the process.
    ///
    /// See [std::process::exit(i32)].
    #[inline]
    pub fn exit(&self, code: i32) -> ! {
        std::process::exit(code);
    }

    /// Print the given prefix followed by the contained error message.
    ///
    /// No padding is inserted between either elements.
    pub fn print_all<D>(&self, prefix: D) -> &Self
    where
        D: Display,
    {
        eprintln!("{prefix}{}", self.message);
        self.enumerator(&self.reasons, " - caused by: ", "     |        ");
        self.enumerator(&self.help, " + help: ", "     |   ");
        self
    }

    /// Add a help message to this UserError.
    #[inline]
    pub fn add_help(&mut self, help: impl Into<String>) {
        self.help.push(help.into());
    }

    /// Add a reason to this UserError.
    #[inline]
    pub fn add_reason(&mut self, reason: impl Into<String>) {
        self.reasons.push(reason.into());
    }

    /// Add a help tip to this UserError.
    ///
    /// Returns the current instance.
    #[inline]
    pub fn and_help(mut self, help: impl Into<String>) -> Self {
        self.help.push(help.into());
        self
    }

    /// Add a reason to this UserError.
    ///
    /// Returns the current instance.
    #[inline]
    pub fn and_reason(mut self, reason: impl Into<String>) -> Self {
        self.reasons.push(reason.into());
        self
    }

    /// Create a new UserError.
    #[inline]
    pub fn new(message: String) -> Self {
        Self {
            message,
            reasons: Vec::new(),
            help: Vec::new(),
        }
    }

    /// Create a new UserError.
    #[inline]
    pub fn from(message: &str) -> Self {
        Self::new(message.to_string())
    }

    #[inline]
    pub const fn message(&self) -> &String {
        &self.message
    }

    #[inline]
    pub const fn reasons(&self) -> &Vec<String> {
        &self.reasons
    }

    #[inline]
    pub fn reasons_mut(&mut self) -> &mut Vec<String> {
        &mut self.reasons
    }

    #[inline]
    pub const fn help(&self) -> &Vec<String> {
        &self.help
    }

    #[inline]
    pub fn help_mut(&mut self) -> &mut Vec<String> {
        &mut self.help
    }
}

/// A trait marking a type as able to be converted into an [UserError].
/// # Examples
/// ```
/// use std::fs;
/// use uerr::IntoUserError;
///
/// let contents = fs::read_to_string("names.txt")
///     .map_err(|err| {
///         err.into_user_err()
///            .print_all("myprogram.exe: ")
///            .exit(err.raw_os_error().unwrap_or(-1))
///     });
/// ```
pub trait IntoUserError {
    /// Convert this value into an [UserError].
    fn into_user_err(self) -> UserError;
}

impl<D> IntoUserError for D
where
    D: Display,
{
    fn into_user_err(self) -> UserError {
        UserError::new(self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::UserError;

    #[test]
    fn sample_error() {
        UserError::from("could not open file")
            .and_reason("The system cannot find the file specified.")
            .and_reason("Filler reason.")
            .and_help("Does this file exist?")
            .and_help("Filler help.")
            .print_all("program.exe: ");
    }
}
