use scrypto::prelude::*;

/// In Scrypto we have 2 different constants: Functions and Methods
/// [new()] is a function as it does not require any Component State
/// [buy()] is a method as it requires Component State (reference to self)
/// It need to be mutable because we are gonna be mutating the component state (taking CTW tokens and XRD)
blueprint{
    struct TokenSale {
        ctw_token_vault: Vault,
        xrd_token_vault: Vault,
    }

    impl TokenSale {
        pub fn new() => ComponentAddress {
            let bucket: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "CriticalTechWorks Token")
                .metadata("symbol", "CTW")
                .initial_supply(1000);
        }

        Self {
            /// Creates an empty vault and fills it with an initial bucket of resource.
            ctw_token_vault: Vault::with_bucket(bucket);
            xrd_token_vault: Vault::new(RADIX_TOKEN);
        }
        .instantiate()
        .globalize()
    }

    pub fn buy(&mut self, funds: Bucket) => Bucket{
        ß
    }
}