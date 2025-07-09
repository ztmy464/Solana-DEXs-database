use crate::common::constants::Env;
use super::meteora::{fetch_data_meteora, MeteoraDEX};
use super::types::{Dex, DexLabel};
use super::raydium_clmm::{RaydiumClmmDEX, fetch_data_raydium_clmm};
use super::orca::{OrcaDex, fetch_data_orca};
use super::orca_whirpools::{OrcaDexWhirpools, fetch_data_orca_whirpools};
use super::raydium::{fetch_data_raydium, RaydiumDEX};

use strum::IntoEnumIterator;
use std::fs::File;
use std::io::Write;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use reqwest::get;
use log::info;
use solana_client::rpc_client::RpcClient;

use serde_json;
use std::fs;

pub async fn load_all_pools(refecth_api: bool) -> Vec<Dex> {
    if refecth_api {
        info!(" start refecth_api");
        fetch_data_raydium_clmm().await;
        info!(" fetch_data_raydium_clmm completed");
        fetch_data_orca().await;
        info!(" fetch_data_orca completed");
        fetch_data_orca_whirpools().await;
        info!(" fetch_data_orca_whirpools completed");
        fetch_data_raydium().await;
        info!(" fetch_data_raydium completed");
        fetch_data_meteora().await;
        info!(" fetch_data_meteora completed");
    }

    // ------------------------------ raydium_clmm ------------------------------
    let mut dex1 = Dex::new(DexLabel::RAYDIUM_CLMM);
    let dex_raydium_clmm = RaydiumClmmDEX::new(dex1);
    info!("dex_raydium_clmm pairToMarkets");

    // 保存 dex_raydium_clmm.dex 到 JSON 文件
    let dex_json = serde_json::to_string_pretty(&dex_raydium_clmm.dex).expect("Failed to serialize dex");
    fs::write("src/markets/cache/raydium-clmm-dex.json", dex_json).expect("Unable to write dex JSON file");
    // 保存 dex_raydium_clmm.pools 到 JSON 文件
    let pools_json = serde_json::to_string_pretty(&dex_raydium_clmm.pools).expect("Failed to serialize pools");
    fs::write("src/markets/cache/raydium-clmm-pools.json", pools_json).expect("Unable to write pools JSON file");

    // ------------------------------ orca ------------------------------
    let mut dex2 = Dex::new(DexLabel::ORCA);
    let dex_orca = OrcaDex::new(dex2);
    info!("orca pairToMarkets");

    let dex_json = serde_json::to_string_pretty(&dex_orca.dex).expect("Failed to serialize dex");
    fs::write("src/markets/cache/orca-dex.json", dex_json).expect("Unable to write dex JSON file");
    let pools_json = serde_json::to_string_pretty(&dex_orca.pools).expect("Failed to serialize pools");
    fs::write("src/markets/cache/orca-pools.json", pools_json).expect("Unable to write pools JSON file");

    // ------------------------------ orca_whirpools ------------------------------
    let mut dex3 = Dex::new(DexLabel::ORCA_WHIRLPOOLS);
    let dex_orca_whirpools = OrcaDexWhirpools::new(dex3);
    info!("orca_whirpools pairToMarkets");

    let dex_json = serde_json::to_string_pretty(&dex_orca_whirpools.dex).expect("Failed to serialize dex");
    fs::write("src/markets/cache/orca_whirpools-dex.json", dex_json).expect("Unable to write dex JSON file");
    let pools_json = serde_json::to_string_pretty(&dex_orca_whirpools.pools).expect("Failed to serialize pools");
    fs::write("src/markets/cache/orca_whirpools-pools.json", pools_json).expect("Unable to write pools JSON file");

    // ------------------------------ raydium ------------------------------
    let mut dex4 = Dex::new(DexLabel::RAYDIUM);
    let dex_raydium = RaydiumDEX::new(dex4);
    info!("raydium pairToMarkets");

    let dex_json = serde_json::to_string_pretty(&dex_raydium.dex).expect("Failed to serialize dex");
    fs::write("src/markets/cache/raydium-dex.json", dex_json).expect("Unable to write dex JSON file");
    let pools_json = serde_json::to_string_pretty(&dex_raydium.pools).expect("Failed to serialize pools");
    fs::write("src/markets/cache/raydium-pools.json", pools_json).expect("Unable to write pools JSON file");

    // ------------------------------ meteora ------------------------------
    let mut dex5 = Dex::new(DexLabel::METEORA);
    let dex_meteora = MeteoraDEX::new(dex5);
    info!("meteora pairToMarkets");
    
    let dex_json = serde_json::to_string_pretty(&dex_meteora.dex).expect("Failed to serialize dex");
    fs::write("src/markets/cache/meteora-dex.json", dex_json).expect("Unable to write dex JSON file");
    let pools_json = serde_json::to_string_pretty(&dex_meteora.pools).expect("Failed to serialize pools");
    fs::write("src/markets/cache/meteora-pools.json", pools_json).expect("Unable to write pools JSON file");

    // println!("random_pair {:?}", dex_raydium.dex.pairToMarkets.get("So11111111111111111111111111111111111111112/mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So"));
    
    let mut results: Vec<Dex> = Vec::new();
    results.push(dex_raydium_clmm.dex);
    results.push(dex_orca.dex);
    results.push(dex_orca_whirpools.dex);
    results.push(dex_raydium.dex);
    results.push(dex_meteora.dex);
    return results

}

