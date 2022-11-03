extern crate dotenv;
use dotenv::dotenv;
use std::env;
use std::{collections::HashMap, time::Duration};
#[allow(clippy::or_fun_call)]

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let key = env::var("KEY").expect("No KEY provided.");
    let vault_addr = env::var("VAULT_ADDR").expect("No VAULT_ADDR provided.");
    let interval: u64 = env::var("INTERVAL")
        .unwrap_or("5".to_string())
        .parse()
        .expect("Non-numeric value passed for INTERVAL.");

    let client = reqwest::blocking::Client::new();
    let unseal_url = format!("{}/v1/sys/unseal", vault_addr);
    let health_url = format!("{}/v1/sys/health", vault_addr);
    let mut req = HashMap::new();
    req.insert("key", key);

    loop {
        // Check if the vault is sealed.
        let res = client.get(&health_url).send();
        match res {
            Ok(_) => {
                println!("Vault is sealed, trying to unseal...");
                let res = client.post(&unseal_url).json(&req).send();
                match res {
                    Ok(_) => println!("Vault unsealed."),
                    Err(err) => println!("Unable to unseal Vault: {}", err),
                }
            }
            Err(e) => {
                println!("Unable to query Vault health: {}", e);
            }
        }
        std::thread::sleep(Duration::from_secs(interval))
    }
}
