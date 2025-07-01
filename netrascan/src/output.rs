use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;

use serde::Serialize;
use serde_json::{to_string, to_writer_pretty};

pub fn append_jsonl<T>(path: &str, data: &T) -> std::io::Result<()>
where
    T: ?Sized + Serialize,
{
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;

    let json = to_string(data)?;
    writeln!(file, "{json}")?;

    Ok(())
}

pub fn append_address_jsonl(path: &str, addresses: &HashSet<String>) -> std::io::Result<()> {
    let file = OpenOptions::new().create(true).append(true).open(path)?;
    let mut writer = std::io::BufWriter::new(file);

    for address in addresses {
        let json = to_string(address)?;
        writeln!(writer, "{json}")?;
    }

    writer.flush()?;
    Ok(())
}

pub fn write_json<P, T>(path: P, value: &T) -> std::io::Result<()>
where 
    P: AsRef<Path>,
    T: Serialize,
{
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    to_writer_pretty(writer, value)
        .map_err(std::io::Error::other)
}
