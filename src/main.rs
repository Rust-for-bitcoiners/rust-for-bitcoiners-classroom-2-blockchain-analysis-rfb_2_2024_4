use std::{env, path::PathBuf, time};

use bitcoincore_rpc::{bitcoin::{block, network, BlockHash}, json, jsonrpc::{self}, Auth, Client, Error, RpcApi};
use chrono::Duration;
#[macro_use]
extern crate lazy_static;

// bcrt1qeqywuu02z9kmahnausmcy66dwncttmpkzk3dtl
// bcrt1qhesylqc42aaym37xdsuhdtp4acyayudpc7guzh
// bcrt1qyz0z55uc68qsesxlyu5392khlxv8454lr2a935

lazy_static! {
    static ref RPC_CLIENT: Client = {
        dotenv::dotenv().ok();
        let rpc_user: String = env::var("BITCOIN_RPC_USER").expect("BITCOIN_RPC_USER must be set");
        let cookie_url: String = env::var("BITCOIN_COOKIE_URL").expect("BITCOIN_COOKIE_URL must be set");
        let rpc_password: String =
            env::var("BITCOIN_RPC_PASSWORD").expect("BITCOIN_RPC_PASSWORD must be set");
        let rpc_port: String = env::var("BITCOIN_RPC_PORT").expect("BITCOIN_RPC_PORT must be set");
        let rpc_url: String = format!("http://{rpc_user}:{rpc_password}@127.0.0.1:{rpc_port}/");
        let cookie_file: PathBuf = PathBuf::from(cookie_url.clone());

        Client::new(&rpc_url, Auth::CookieFile(cookie_file)).unwrap()
    };
}

fn time_to_mine(block_height_a: u64, block_height_b: u64) -> u32 {
    // * is a deref operator which invokes the Deref trait of the type RPC_CLIENT which was created
    // when the lazy macro is expanded
    // if a value has a static lifetime then it means that value lives as long as the program lives
    let rpc_client: &Client = &*RPC_CLIENT;

    let block_hash_a = rpc_client.get_block_hash(block_height_a).unwrap();
    let block_a = rpc_client.get_block(&block_hash_a).unwrap();
    let block_hash_b = rpc_client.get_block_hash(block_height_b).unwrap();
    let block_b = rpc_client.get_block(&block_hash_b).unwrap();
    // todo!()
    let time_difference = block_b.header.time - block_a.header.time;

    time_difference
}

fn number_of_transactions(block_height: u32) -> u16 {
    const TX_COUNT: usize = 20;
    let some_value = Box::new(TX_COUNT);
    let rpc = RPC::new();
    let rpc_client = rpc.client();

    let mut total_returned = 0;
    let mut total_in_block = 0;

    loop {
        let tx_list = rpc_client.list_transactions(
            None, 
            Some(*some_value),
            Some(total_returned), 
            None
        ).unwrap();

        if tx_list.len() == 0 {
            break;
        } else {
            total_returned += tx_list.len();
            let in_block_count = tx_list.iter().filter(|&tx| tx.info.blockheight == Some(block_height)).collect::<Vec<_>>();
            total_in_block += in_block_count.len();
        }
    }
    
    total_in_block as u16
}

struct RPC {
    user: String,
    port: String,
    password: String,
}

impl RPC {
    pub fn new() -> Self {
        RPC {
            user: env::var("BITCOIN_RPC_USER").expect("BITCOIN_RPC_USER must be set"),
            port: env::var("BITCOIN_RPC_PORT").expect("BITCOIN_RPC_PORT must be set"),
            password: env::var("BITCOIN_RPC_PASSWORD").expect("BITCOIN_RPC_PASSWORD must be set"),
        }
    }

    pub fn rpc_url(&self) -> String {
        let RPC { user, port, password } = self;
        format!("http://{user}:{password}@127.0.0.1:{port}/")
    }

    pub fn client(&self) -> Client {
        const TIMEOUT_UTXO_SET_SCANS: time::Duration = time::Duration::from_secs(60 * 8); // 8 minutes

        let custom_timeout_transport = jsonrpc::simple_http::Builder::new()
            .url(&self.rpc_url())
            .expect("invalid rpc url")
            .auth(self.user.clone(), Some(self.password.clone()))
            .timeout(TIMEOUT_UTXO_SET_SCANS)
            .build();
        let custom_timeout_rpc_client =
            jsonrpc::client::Client::with_transport(custom_timeout_transport);

        Client::from_jsonrpc(custom_timeout_rpc_client)
    }
}

fn main() {
    // you can use rpc_client here as if it was a global variable
    // println!("{:?}", res);
    dotenv::dotenv().ok();

    let duration = time_to_mine(1, 100);
    println!("Time to mine next block: {}", duration);

    let tx_count = number_of_transactions(109);
    println!("Count, {tx_count}");

}
