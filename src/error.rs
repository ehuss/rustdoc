//! Error types, traits and aliases.

/// Thrown whenever a crate cannot be found
#[derive(Debug, Fail)]
#[fail(display = "Crate not found: {}", crate_name)]
pub struct CrateErr {
    /// The name of the crate that wasn't found.
    pub crate_name: String,
}

/// Thrown whenever Cargo fails to run properly when getting data for `rustdoc`
#[derive(Debug, Fail)]
#[fail(display = "Cargo failed with status {}. stderr:\n{}", status, stderr)]
pub struct Cargo {
    /// The status Cargo returned when it failed.
    pub status: ::std::process::ExitStatus,
    /// The standard error output.
    pub stderr: String,
}

/// Thrown whenever the `JSON` grabbed from somewhere else is not what is expected.
/// This is usually thrown when grabbing data output from `Cargo`
#[derive(Debug, Fail)]
#[fail(display = "Unexpected JSON response from {}", location)]
pub struct Json {
    /// The location of the incorrect Json.
    pub location: String,
}
