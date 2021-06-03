#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use anyhow::Result;
use bitcoincore_rpc::{
    bitcoin::BlockHash,
    json::{GetBlockResult, GetBlockchainInfoResult},
    Auth, Client, RpcApi,
};
use clap::{crate_authors, crate_name, crate_version, App, Arg};
use once_cell::sync::OnceCell;
use rocket::http::RawStr;
use rocket_contrib::{helmet::SpaceHelmet, json::Json};
use std::str::FromStr;

static CLIENT: OnceCell<Client> = OnceCell::new();

// SAFETY: The client is only initilized once and then the server is started
// meaning it's safe to call get_unchecked inside routes

#[get("/getbestblockhash")]
fn get_best_block_hash() -> Result<Json<BlockHash>> {
    Ok(Json(unsafe {
        CLIENT.get_unchecked().get_best_block_hash()?
    }))
}

#[get("/getblock?<blockhash>")]
fn get_block(blockhash: &RawStr) -> Result<Json<GetBlockResult>> {
    Ok(Json(unsafe {
        CLIENT
            .get_unchecked()
            .get_block_info(&BlockHash::from_str(blockhash)?)?
    }))
}

#[get("/getblockcount")]
fn get_block_count() -> Result<String> {
    Ok(unsafe { CLIENT.get_unchecked().get_block_count()?.to_string() })
}

#[get("/getblockchaininfo")]
fn get_blockchain_info() -> Result<Json<GetBlockchainInfoResult>> {
    Ok(Json(unsafe {
        CLIENT.get_unchecked().get_blockchain_info()?
    }))
}

fn main() -> Result<()> {
    let matches = App::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .arg(
            Arg::new("rpc-address")
                .long("rpc-address")
                .value_name("address")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("rpc-user")
                .long("rpc-user")
                .value_name("user")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("rpc-password")
                .long("rpc-password")
                .value_name("password")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let rpc_address = matches
        .value_of("rpc-address")
        .expect("Failed to get rpc address")
        .to_string();

    let rpc_user = matches
        .value_of("rpc-user")
        .expect("Failed to get rpc user")
        .to_string();

    let rpc_password = matches
        .value_of("rpc-password")
        .expect("Failed to get rpc password")
        .to_string();

    let client = Client::new(rpc_address, Auth::UserPass(rpc_user, rpc_password))?;

    CLIENT.set(client).expect("Wrong initialization of client");

    rocket::ignite()
        .attach(SpaceHelmet::default())
        .mount(
            "/",
            routes![
                get_best_block_hash,
                get_block,
                get_block_count,
                get_blockchain_info
            ],
        )
        .launch();

    Ok(())
}
