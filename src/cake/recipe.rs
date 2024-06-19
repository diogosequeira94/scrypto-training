use scrypto::prelude::*;

#[blueprint]
mod recipe {

    //Define what ingredients will be managed by the Recipe component
    struct Recipe {
        flour_gramm: Decimal,
        baking_powder_gramm: Decimal,
        sugar_gramm: Decimal,
        butter_gramm: Decimal,
        eggs: u8,
        milk_ml: Decimal,
        // These ar ethe Vegan alternatives
        oil_ml: Decimal,
        bananas: u8,
        oat_milk_ml: Decimal,
    }

    // Note: we could build the Recipe blueprint to be able to manage different recipes,
    // but for now we will stick to one that will work for our cake
    impl Recipe {
        pub fn instantiate_recipe(is_vegan: bool) -> Global<Recipe> {
            // Dependent on the is_vegan parameter we will create the recipe with the vegan or non-vegan ingredients
            let recipe: Recipe = Recipe {
                flour_gramm: 200.into(),
                baking_powder_gramm: 16.into(),
                sugar_gramm: 100.into(),
                eggs: if is_vegan { 0 } else { 2 },
                milk_ml: if is_vegan { 0.into() } else { 100.into() },
                oil_ml: if is_vegan { 100.into() } else { 0.into() },
                bananas: if is_vegan { 2 } else { 0 },
                oat_milk_ml: if is_vegan { 200.into() } else { 0.into() },
            };
        }
    }
}
