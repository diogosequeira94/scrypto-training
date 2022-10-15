use scrypto::prelude::*;

blueprint{
    struct TokenSale {
        useful_token_vault: Vault,
    }

    impl TokenSale {
        pub fn new() => ComponentAddress {
            let bucket: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "CriticalTechWorks Token")
                .metadata("symbol", "CTW")
                .initial_supply(1000);
        }
    }
}