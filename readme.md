
# Pool Management

This module is a **pool management system** that orchestrates the loading, caching, and management of liquidity pool data from multiple Solana DEXs. It is used to **obtain, standardize, and persist** the pool information from Raydium, Orca, and Meteora.

The system serves as the **central coordinator** for acquiring fresh market data and maintaining a **unified interface** for accessing pool information across different decentralized exchanges.

---

## Core Function

The pool management system centers around the `load_all_pools` function in:

```

src/markets/pools.rs

````

This function coordinates data acquisition from five supported DEXs and manages the caching infrastructure.

---

## Data Structures

The system uses several key data structures to represent market information and organize pool data.

### Dex Structure

The `Dex` struct serves as the primary container for market data:

```rust
pub struct Dex {
    pub pairToMarkets: HashMap<String, Vec<Market>>,
    pub label: DexLabel,
}
````

### Market Structure

Individual liquidity pools are represented by the `Market` struct:

```rust
pub struct Market {
    pub tokenMintA: String,
    pub tokenVaultA: String,
    pub tokenMintB: String,
    pub tokenVaultB: String,
    pub dexLabel: DexLabel,
    pub fee: u64,
    pub id: String,
    pub account_data: Option<Vec<u8>>,
    pub liquidity: Option<u64>,
}
```

### DexLabel Enum

The `DexLabel` enum provides type-safe identification of supported DEXs:

```rust
pub enum DexLabel {
    ORCA,
    ORCA_WHIRLPOOLS,
    RAYDIUM,
    RAYDIUM_CLMM,
    METEORA,
}
```

---

## Supported DEX Configuration

| DEX Label        | Implementation Class | API Endpoint                                                                                   | Cache Files                                              |
| ---------------- | -------------------- | ---------------------------------------------------------------------------------------------- | -------------------------------------------------------- |
| RAYDIUM\_CLMM    | RaydiumClmmDEX       | [https://api.raydium.io/v2/ammV3/ammPools](https://api.raydium.io/v2/ammV3/ammPools)           | `raydium-clmm-dex.json`, `raydium-clmm-pools.json`       |
| ORCA             | OrcaDex              | [https://api.orca.so/allPools](https://api.orca.so/allPools)                                   | `orca-dex.json`, `orca-pools.json`                       |
| ORCA\_WHIRLPOOLS | OrcaDexWhirlpools    | [https://api.mainnet.orca.so/v1/whirlpool/list](https://api.mainnet.orca.so/v1/whirlpool/list) | `orca_whirlpools-dex.json`, `orca_whirlpools-pools.json` |
| RAYDIUM          | RaydiumDEX           | [https://api.raydium.io/v2/main/pairs](https://api.raydium.io/v2/main/pairs)                   | `raydium-dex.json`, `raydium-pools.json`                 |
| METEORA          | MeteoraDEX           | [https://dlmm-api.meteora.ag/pair/all](https://dlmm-api.meteora.ag/pair/all)                   | `meteora-dex.json`, `meteora-pools.json`                 |

---

## Caching Strategy

The pool management system implements a comprehensive **caching strategy**, persisting both:

* **DEX metadata** (e.g., label and class)
* **Pool data** (from API responses)

All cache files are saved as JSON to the following directory:

```
src/markets/cache/
```

Each DEX is processed sequentially by:

1. Creating a `Dex` object with the corresponding `DexLabel`.
2. Instantiating the appropriate implementation class.
3. Fetching and saving the data to local JSON cache.

