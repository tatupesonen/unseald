extern crate dotenv;
use dotenv::dotenv;
use reqwest::StatusCode;
use std::env;
use std::{collections::HashMap, time::Duration};
#[allow(clippy::or_fun_call)]

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let keys = env::var("KEY").expect("No KEY provided.");
    let vault_addrs = env::var("VAULT_ADDR").expect("No VAULT_ADDR provided.");
    let interval: u64 = env::var("INTERVAL")
        .unwrap_or("5".to_string())
        .parse()
        .expect("Non-numeric value passed for INTERVAL.");

    let client = reqwest::blocking::Client::new();

    loop {
        for addr in vault_addrs.split(',').into_iter() {
            let unseal_url = format!("{}/v1/sys/unseal", addr);
            let health_url = format!("{}/v1/sys/health", addr);
            for (mut idx, key) in keys.split(",").into_iter().enumerate() {
                idx += 1;
                let mut req = HashMap::new();
                req.insert("key", key);
                let res = client.get(&health_url).send();
                match res {
                    Ok(data) => {
                        if data.status() != StatusCode::OK {
                            println!(
                                "Vault at {} is sealed, trying to unseal with key portion {}...",
                                addr, idx
                            );
                            let res = client.post(&unseal_url).json(&req).send();
                            match res {
                                Ok(data) => {
                                    if data.status() == StatusCode::OK {
                                        println!(
                                            "Vault at {} provided with key portion {}.",
                                            addr, idx
                                        )
                                    } else {
                                        println!("Unable to unseal Vault: {}", data.status());
                                    }
                                }
                                Err(err) => println!("Unable to unseal Vault: {}", err),
                            }
                        }
                    }
                    Err(e) => {
                        println!("Unable to query Vault health: {}", e);
                    }
                }
            }
        }
        std::thread::sleep(Duration::from_secs(interval))
    }
}
