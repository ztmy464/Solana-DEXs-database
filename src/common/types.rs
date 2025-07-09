use serde::{Deserialize, Serialize};

use crate::arbitrage::types::TokenInArb;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputVec {
    pub tokens_to_arb: Vec<TokenInArb>,
    pub include_1hop: bool,
    pub include_2hop: bool,
    pub numbers_of_best_paths: usize,  // 要寻找的最优路径数量
    pub get_fresh_pools_bool: bool,  // 是否实时更新池子（避免 timeout）
}