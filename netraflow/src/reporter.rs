use std::{fs::File, io::Write, path::Path};

use csv::Writer;

use crate::aggregator::DailyOutflowSummary;

pub fn export_csv<P: AsRef<Path>>(path: P, summary: &DailyOutflowSummary) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(path)?;
    let mut writer = Writer::from_writer(file);
    writer.write_record(["date", "outflow_usd"])?;

    for (date, amount) in summary.iter().collect::<Vec<_>>() {
        let amount = format!("{amount:.2}");
        writer.write_record([date, &amount])?;
    }

    writer.flush()?;
    Ok(())
}
