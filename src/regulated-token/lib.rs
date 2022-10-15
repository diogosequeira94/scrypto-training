use scrypto::prelude::*;

blueprint! {
    struct RegulatedToken {
        token_supply: Vault,
        internal_authority: Vault,
        collected_xrd: Vault,
        current_stage: u8,
        admin_badge_resource_address: ResourceAddress,
        freeze_badge_resource_address: ResourceAddress,
    }
}