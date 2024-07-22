use std::time;

use bitcoincore_rpc::{Auth, Client, RpcApi};
use chrono::Duration;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref RPC_CLIENT: Client = {
        // Directly assigning the RPC details
        let rpc_url = "http://127.0.0.1:18443".to_string();
        let rpc_user = "polaruser".to_string();
        let rpc_password = "polarpass".to_string();
        Client::new(&rpc_url, Auth::UserPass(rpc_user, rpc_password)).unwrap()
    };
}

// Calculate the time to mine a block
fn time_to_mine(block_height: u64) -> Result<Duration, Box<dyn std::error::Error>> {
    let rpc_client: &Client = &*RPC_CLIENT;
    let block_hash = rpc_client.get_block_hash(block_height)?;
    let block = rpc_client.get_block_info(&block_hash)?;
    let previous_block_hash = block.previousblockhash.ok_or("Previous block hash not found")?;
    let previous_block = rpc_client.get_block_info(&previous_block_hash)?;

    let time_to_mine = block.time - previous_block.time;
    Ok(Duration::seconds(time_to_mine as i64))
}

// Get the number of transactions in a block
fn number_of_transactions(block_height: u64) -> Result<u16, Box<dyn std::error::Error>> {
    let rpc_client: &Client = &*RPC_CLIENT;
    let block_hash = rpc_client.get_block_hash(block_height)?;
    let block = rpc_client.get_block_info(&block_hash)?;

    Ok(block.tx.len() as u16)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Demonstrate usage of the RPC client and the implemented functions
    const TIMEOUT_UTXO_SET_SCANS: time::Duration = time::Duration::from_secs(60 * 8); // 8 minutes

    let rpc_url = "http://127.0.0.1:18443".to_string();
    let rpc_user = "polaruser".to_string();
    let rpc_password = "polarpass".to_string();

    let custom_timeout_transport = bitcoincore_rpc::jsonrpc::simple_http::Builder::new()
        .url(&rpc_url)?
        .auth(rpc_user, Some(rpc_password))
        .timeout(TIMEOUT_UTXO_SET_SCANS)
        .build();
    let custom_timeout_rpc_client =
        bitcoincore_rpc::jsonrpc::client::Client::with_transport(custom_timeout_transport);

    let rpc_client = Client::from_jsonrpc(custom_timeout_rpc_client);
    let res = rpc_client.get_tx_out_set_info(None, None, None)?;
    println!("{:?}", res);

    // Example usage of the implemented functions
    let block_height = 150; // Valid block height based on current node's height
    match time_to_mine(block_height) {
        Ok(mining_duration) => println!("Time to mine block {}: {:?}", block_height, mining_duration),
        Err(e) => eprintln!("Error calculating time to mine block {}: {:?}", block_height, e),
    }

    match number_of_transactions(block_height) {
        Ok(num_transactions) => println!("Number of transactions in block {}: {}", block_height, num_transactions),
        Err(e) => eprintln!("Error getting number of transactions in block {}: {:?}", block_height, e),
    }

    Ok(())
}
