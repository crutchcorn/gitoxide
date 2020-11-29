pub type Error = Box<dyn std::error::Error>;
pub type Result = std::result::Result<(), Error>;

mod read;
