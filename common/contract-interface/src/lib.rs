use ethers::prelude::abigen;

#[cfg(not(feature = "dev"))]
abigen!(
    ZgsFlow,
    "../../0g-storage-contracts/artifacts/contracts/dataFlow/Flow.sol/Flow.json"
);

#[cfg(not(feature = "dev"))]
abigen!(
    PoraMine,
    "../../0g-storage-contracts/artifacts/contracts/miner/Mine.sol/PoraMine.json"
);

#[cfg(feature = "dev")]
abigen!(
    ZgsFlow,
    "../../0g-storage-contracts-dev/artifacts/contracts/dataFlow/Flow.sol/Flow.json"
);

#[cfg(feature = "dev")]
abigen!(
    PoraMine,
    "../../0g-storage-contracts-dev/artifacts/contracts/miner/Mine.sol/PoraMine.json"
);
