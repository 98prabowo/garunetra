use std::{
    fs::{create_dir_all, OpenOptions},
    io::Write,
    path::Path,
};

use common::model::FlowSummary;
use serde_json::to_string;

use crate::error::Result;

pub fn save_summary<P>(summary: &FlowSummary, path: P) -> Result<()> 
where
    P: AsRef<Path>
{
    if let Some(dir) = path.as_ref().parent() {
        create_dir_all(dir)?;
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    let json = to_string(summary)?;
    writeln!(file, "{json}")?;
    Ok(())
}
