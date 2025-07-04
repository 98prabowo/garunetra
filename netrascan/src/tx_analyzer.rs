use crate::{
    classify::classify_tx, 
    constants::HEURISTICS_PATH, 
    error::Result, 
    heuristics::Heuristics, 
    ingest::EtheriumClient, 
};

pub async fn scan_latest(rpc: &str) -> Result<()> {
    let client = EtheriumClient::new(rpc);
    let block_number = client.get_latest_block_number().await?;
    let block = client.get_block_by_number(block_number).await?;

    let mut heuristics = Heuristics::new();

    for tx in block.transactions {
        if let Some(to) = tx.to.as_ref() {
            let to = to.to_ascii_lowercase();
            let category = classify_tx(&tx, &heuristics);
            heuristics.push_by_tx_category(&category, &to);
            println!(
                "Tx {} -> {to} | category: {category:?}",
                tx.from
            );
        }
    }

    heuristics.write(HEURISTICS_PATH)?;

    Ok(())
}

pub async fn scan_block(block_number: u64, rpc: &str) -> Result<()> {
    let client = EtheriumClient::new(rpc);
    let block = client.get_block_by_number(block_number).await?;

    let mut heuristics = Heuristics::new();

    for tx in block.transactions {
        if let Some(to) = tx.to.as_ref() {
            let to = to.to_ascii_lowercase();
            let category = classify_tx(&tx, &heuristics);
            heuristics.push_by_tx_category(&category, &to);
            println!(
                "Tx {} -> {to} | category: {category:?}",
                tx.from
            );
        }
    }

    heuristics.write(HEURISTICS_PATH)?;

    Ok(())
}

pub async fn classify(tx_hash: &str, rpc: &str) -> Result<()> {
    let client = EtheriumClient::new(rpc);
    let tx = client.get_transaction_by_hash(tx_hash).await?;

    let mut heuristics = Heuristics::new();

    if let Some(to) = tx.to.as_ref() {
        let to = to.to_ascii_lowercase();
            let category = classify_tx(&tx, &heuristics);
        heuristics.push_by_tx_category(&category, &to);
        println!(
            "Tx {} -> {to} | category: {category:?}",
            tx.from
        );
    }

    heuristics.write(HEURISTICS_PATH)?;

    Ok(())
}

pub async fn list_heuristics(bridge_only: bool, cex_only: bool) -> Result<()> {
    let heuristics = Heuristics::load(HEURISTICS_PATH)?;

    if bridge_only {
        println!("🔗 Known bridge addresses:");
        for (name, addresses) in heuristics.bridge {
            println!("- {name}:");
            for addr in addresses {
                println!("  • {addr}");
            }
        }
    } else if cex_only {
        println!("🏦 Known CEX addresses:");
        for (name, addresses) in heuristics.cex {
            println!("- {name}:");
            for addr in addresses {
                println!("  • {addr}");
            }
        }
    } else {
        println!("🔗 Known bridge addresses:");
        for (name, addresses) in &heuristics.bridge {
            println!("- {name}:");
            for addr in addresses {
                println!("  • {addr}");
            }
        }

        println!("\n🏦 Known CEX addresses:");
        for (name, addresses) in &heuristics.cex {
            println!("- {name}:");
            for addr in addresses {
                println!("  • {addr}");
            }
        }
    }

    Ok(())
}
