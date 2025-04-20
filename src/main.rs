use anyhow::Result;
use bdk::{
    bitcoin::Network,
    database::MemoryDatabase,
    wallet::{AddressIndex, AddressInfo},
    SyncOptions, Wallet,
};
use bdk::blockchain::electrum::ElectrumBlockchain;
use bdk::electrum_client::Client;

fn main() -> Result<()> {
    // Create a new wallet with a random descriptor
    let descriptor = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)";
    let wallet = Wallet::new(
        descriptor,
        None,
        Network::Testnet,
        MemoryDatabase::default(),
    )?;

    // Connect to an Electrum server
    let client = Client::new("ssl://electrum.blockstream.info:60002")?;
    let blockchain = ElectrumBlockchain::from(client);
    wallet.sync(&blockchain, SyncOptions { progress: None })?;

    // Generate a new address
    let address_info: AddressInfo = wallet.get_address(AddressIndex::New)?;
    println!("New address: {}", address_info.address);

    // Get wallet balance
    let balance = wallet.get_balance()?;
    println!("Wallet balance: {} sats", balance.get_total());

    Ok(())
}
