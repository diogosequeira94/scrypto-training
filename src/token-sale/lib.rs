use scrypto::prelude::*;

/// In Scrypto we have 2 different constants: Functions and Methods
/// [new()] is a function as it does not require any Component State
/// [buy()] is a method as it requires Component State (reference to self)
/// It need to be mutable because we are gonna be mutating the component state (taking CTW tokens and XRD)
/// 
blueprint! {
    struct TokenSale {
        ctw_token_vault: Vault,
        xrd_token_vault: Vault,
        price_per_token: Decimal,
    }

    impl TokenSale {
        pub fn new(price_per_token: Decimal) => (ComponentAddress, Bucket) {
            let bucket: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "CriticalTechWorks Token")
                .metadata("symbol", "CTW")
                .initial_supply(1000);

            let seller_badge: Bucket = ResourceBuilder::new_fungible()
            .divisibility(DIVISIBILITY_NONE)
            .metadata("name", "Seller Badge")
            .metadata("symbol", "Seller")
            .initial_supply(1);    

            /// Define who can can certain methods
            /// The seller badge is the only thing that can call this methods
            let access_rules: AccessRules = AccessRules::new()
                .method("withdraw_funds", rule!(require(seller_badge.resource_address())))
                .method("change_price", rule!(require(seller_badge.resource_address())))
                .default(rule!(allow_all));

            let component_address: ComponentAddress = Self {
                ctw_token_vault: Vault::with_bucket(bucket),
                xrd_token_vault: Vault::new(RADIX_TOKEN),
                price_per_token: price_per_token,

            }
        }

        Self {
            /// Creates an empty vault and fills it with an initial bucket of resource.
            ctw_token_vault: Vault::with_bucket(bucket),
            xrd_token_vault: Vault::new(RADIX_TOKEN),
        }
        .instantiate()
        .globalize()

        /// Returns the component address and the seller badge (returning because of the tuple)
        (component_address, seller_badge)
    }

    pub fn buy(&mut self, funds: Bucket) => Bucket {
        let purchase_amount: Decimal = funds.amount() / self.price_per_token;
        self.xrd_token_vault.put(funds);
        self.ctw_token_vault.take(purchase_amount)
    }

    pub fn withdraw_funds(&mut self, amount: Decimal) => Bucket {
        /// We will withdraw [amount] tokens from xrd_token_vault
        self.xrd_token_vault.take(amount)
    }

    /// Changing the price doesnt return anything
    pub fn change_price(&mut self, price: Decimal){
        self.price_per_token = price
    }
}