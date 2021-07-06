use std::{error::Error, result};

pub type Result = result::Result<Vec<String>, Box<dyn Error + Send + Sync>>;
