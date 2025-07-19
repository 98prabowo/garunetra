use netracrawl::{error::Result, ethereum::EthereumClient};

pub async fn scan_latest(rpc: &str) -> Result<()> {
    let client = EthereumClient::new(rpc);
    let block_number = client.get_latest_block_number().await?;
    let block = client.get_block_by_number(block_number).await?;

    println!("Block data: {block:#?}");
    Ok(())
}

pub async fn scan_block(block_number: u64, rpc: &str) -> Result<()> {
    let client = EthereumClient::new(rpc);
    let block = client.get_block_by_number(block_number).await?;

    println!("Block data: {block:#?}");
    Ok(())
}
