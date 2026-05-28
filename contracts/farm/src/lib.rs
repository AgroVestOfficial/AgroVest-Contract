#![no_std]

mod errors;
mod storage;
mod types;

use errors::FarmError;
use types::{Farmer, FarmProduct, Review};

use soroban_sdk::{contract, contractimpl, Address, Env, String, Symbol, Vec};

#[contract]
pub struct FarmContract;

#[contractimpl]
impl FarmContract {
    pub fn initialize(env: Env, admin: Address, token: Address, escrow: Address) {
        if env.storage().instance().has(&Symbol::new(&env, "admin")) {
            panic!("{:?}", FarmError::AlreadyInitialized);
        }
        env.storage().instance().set(&Symbol::new(&env, "admin"), &admin);
        env.storage().instance().set(&Symbol::new(&env, "token"), &token);
        env.storage().instance().set(&Symbol::new(&env, "escrow"), &escrow);
        env.storage().instance().set(&Symbol::new(&env, "farm_ctr"), &0u32);
        env.storage().instance().set(&Symbol::new(&env, "prod_ctr"), &0u32);
        env.storage().instance().set(&Symbol::new(&env, "total_sales"), &0i128);
    }

    pub fn register_farm(
        env: Env,
        caller: Address,
        name: String,
        image: String,
        location: String,
        contact: String,
        farmer_addr: Address,
        email: String,
    ) {
        caller.require_auth();

        if name.is_empty() {
            panic!("{:?}", FarmError::NameCannotBeEmpty);
        }

        let name_key = (Symbol::new(&env, "f_n"), name.clone());
        if env.storage().persistent().has(&name_key) {
            panic!("{:?}", FarmError::NameAlreadyRegistered);
        }

        let mut farm_counter: u32 = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "farm_ctr"))
            .unwrap_or(0);
        farm_counter += 1;

        let farmer = Farmer {
            farm_id: farm_counter,
            business_name: name.clone(),
            business_image: image.clone(),
            business_location: location,
            business_contact: contact,
            business_email: email,
            farmer_address: farmer_addr.clone(),
            is_registered: true,
        };

        let farmer_key = (Symbol::new(&env, "farmer"), caller.clone());
        env.storage().persistent().set(&farmer_key, &farmer);

        let farm_by_id_key = (Symbol::new(&env, "farm_id"), farm_counter);
        env.storage().persistent().set(&farm_by_id_key, &farmer);

        let name_key = (Symbol::new(&env, "f_n"), name);
        env.storage().persistent().set(&name_key, &farm_counter);

        let addr_key = (Symbol::new(&env, "f_a"), caller);
        env.storage().persistent().set(&addr_key, &farm_counter);

        env.storage()
            .instance()
            .set(&Symbol::new(&env, "farm_ctr"), &farm_counter);

        env.events().publish(
            (Symbol::new(&env, "farm"), Symbol::new(&env, "registered")),
            (farmer_addr, farm_counter),
        );
    }

    pub fn update_details(
        env: Env,
        caller: Address,
        index: u32,
        name: String,
        image: String,
        location: String,
        contact: String,
        email: String,
    ) {
        caller.require_auth();

        let farmer_key = (Symbol::new(&env, "farmer"), caller.clone());
        let farmer: Farmer = env
            .storage()
            .persistent()
            .get(&farmer_key)
            .unwrap_or_else(|| panic!("{:?}", FarmError::NotRegistered));

        if farmer.farm_id != index {
            panic!("{:?}", FarmError::InvalidFarmIndex);
        }
        if farmer.farmer_address != caller {
            panic!("{:?}", FarmError::FarmDoesNotBelongToYou);
        }

        let updated = Farmer {
            farm_id: farmer.farm_id,
            business_name: name.clone(),
            business_image: image.clone(),
            business_location: location,
            business_contact: contact,
            business_email: email,
            farmer_address: farmer.farmer_address,
            is_registered: true,
        };

        env.storage().persistent().set(&farmer_key, &updated);
        let farm_by_id_key = (Symbol::new(&env, "farm_id"), index);
        env.storage().persistent().set(&farm_by_id_key, &updated);

        env.events().publish(
            (Symbol::new(&env, "farm"), Symbol::new(&env, "updated")),
            (caller, name),
        );
    }

    pub fn get_farm_index(env: Env, name: String) -> u32 {
        let name_key = (Symbol::new(&env, "f_n"), name);
        env.storage()
            .persistent()
            .get(&name_key)
            .unwrap_or_else(|| panic!("{:?}", FarmError::FarmNotFound))
    }

    pub fn add_farm_product(
        env: Env,
        caller: Address,
        name: String,
        image: String,
        description: String,
        price: i128,
    ) {
        caller.require_auth();

        let farmer_key = (Symbol::new(&env, "farmer"), caller.clone());
        let _farmer: Farmer = env
            .storage()
            .persistent()
            .get(&farmer_key)
            .unwrap_or_else(|| panic!("{:?}", FarmError::NotRegistered));

        let mut product_counter: u32 = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "prod_ctr"))
            .unwrap_or(0);
        product_counter += 1;

        let count_key = (Symbol::new(&env, "p_cnt"), caller.clone());
        let local_index: u32 = env.storage().persistent().get(&count_key).unwrap_or(0);

        let product = FarmProduct {
            product_name: name.clone(),
            product_image: image,
            product_description: description,
            product_price: price,
            product_owner: caller.clone(),
            product_id: product_counter,
            sold: false,
        };

        let prod_key = (Symbol::new(&env, "prod"), caller.clone(), local_index);
        env.storage().persistent().set(&prod_key, &product);

        let global_key = (Symbol::new(&env, "prod_g"), product_counter);
        env.storage()
            .persistent()
            .set(&global_key, &(caller.clone(), local_index));

        env.storage().persistent().set(&count_key, &(local_index + 1));

        env.storage()
            .instance()
            .set(&Symbol::new(&env, "prod_ctr"), &product_counter);

        env.events().publish(
            (Symbol::new(&env, "product"), Symbol::new(&env, "added")),
            (caller, name),
        );
    }

    pub fn update_farm_product(
        env: Env,
        caller: Address,
        index: u32,
        name: String,
        image: String,
        description: String,
        price: i128,
    ) {
        caller.require_auth();

        let count_key = (Symbol::new(&env, "p_cnt"), caller.clone());
        let local_count: u32 = env.storage().persistent().get(&count_key).unwrap_or(0);
        if index >= local_count {
            panic!("{:?}", FarmError::InvalidProductIndex);
        }

        let prod_key = (Symbol::new(&env, "prod"), caller.clone(), index);
        let existing: FarmProduct = env
            .storage()
            .persistent()
            .get(&prod_key)
            .unwrap_or_else(|| panic!("{:?}", FarmError::ProductDoesNotExist));

        if existing.product_owner != caller {
            panic!("{:?}", FarmError::FarmDoesNotBelongToYou);
        }

        let updated = FarmProduct {
            product_name: name.clone(),
            product_image: image,
            product_description: description,
            product_price: price,
            product_owner: existing.product_owner,
            product_id: existing.product_id,
            sold: existing.sold,
        };

        env.storage().persistent().set(&prod_key, &updated);

        env.events().publish(
            (Symbol::new(&env, "product"), Symbol::new(&env, "updated")),
            (caller, name),
        );
    }

    pub fn get_farm_products(env: Env, caller: Address) -> Vec<FarmProduct> {
        let count_key = (Symbol::new(&env, "p_cnt"), caller.clone());
        let count: u32 = env.storage().persistent().get(&count_key).unwrap_or(0);
        let mut products = Vec::new(&env);
        for i in 0..count {
            let prod_key = (Symbol::new(&env, "prod"), caller.clone(), i);
            if let Some(p) = env.storage().persistent().get::<_, FarmProduct>(&prod_key) {
                products.push_back(p);
            }
        }
        products
    }

    pub fn get_all_farm_products(env: Env) -> Vec<FarmProduct> {
        let farm_counter: u32 = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "farm_ctr"))
            .unwrap_or(0);
        let mut all_products = Vec::new(&env);
        for farm_id in 1..=farm_counter {
            let farm_key = (Symbol::new(&env, "farm_id"), farm_id);
            if let Some(farmer) = env.storage().persistent().get::<_, Farmer>(&farm_key) {
                let count_key = (Symbol::new(&env, "p_cnt"), farmer.farmer_address.clone());
                let count: u32 = env.storage().persistent().get(&count_key).unwrap_or(0);
                for i in 0..count {
                    let prod_key = (Symbol::new(&env, "prod"), farmer.farmer_address.clone(), i);
                    if let Some(p) = env.storage().persistent().get::<_, FarmProduct>(&prod_key) {
                        all_products.push_back(p);
                    }
                }
            }
        }
        all_products
    }

    pub fn add_to_cart(env: Env, caller: Address, product_id: u32) {
        caller.require_auth();

        let global_key = (Symbol::new(&env, "prod_g"), product_id);
        let (owner, local_idx): (Address, u32) = env
            .storage()
            .persistent()
            .get(&global_key)
            .unwrap_or_else(|| panic!("{:?}", FarmError::ProductDoesNotExist));

        let prod_key = (Symbol::new(&env, "prod"), owner, local_idx);
        let product: FarmProduct = env
            .storage()
            .persistent()
            .get(&prod_key)
            .unwrap_or_else(|| panic!("{:?}", FarmError::ProductDoesNotExist));

        let cart_count_key = (Symbol::new(&env, "cart_c"), caller.clone());
        let cart_idx: u32 = env.storage().temporary().get(&cart_count_key).unwrap_or(0);

        let cart_key = (Symbol::new(&env, "cart"), caller.clone(), cart_idx);
        env.storage().temporary().set(&cart_key, &product);
        env.storage()
            .temporary()
            .set(&cart_count_key, &(cart_idx + 1));
    }

    pub fn get_cart(env: Env, buyer: Address) -> Vec<FarmProduct> {
        let cart_count_key = (Symbol::new(&env, "cart_c"), buyer.clone());
        let count: u32 = env.storage().temporary().get(&cart_count_key).unwrap_or(0);
        let mut items = Vec::new(&env);
        for i in 0..count {
            let cart_key = (Symbol::new(&env, "cart"), buyer.clone(), i);
            if let Some(p) = env.storage().temporary().get::<_, FarmProduct>(&cart_key) {
                items.push_back(p);
            }
        }
        items
    }

    pub fn purchase_product(env: Env, caller: Address, product_id: u32, amount: i128) {
        caller.require_auth();

        let global_key = (Symbol::new(&env, "prod_g"), product_id);
        let (owner, local_idx): (Address, u32) = env
            .storage()
            .persistent()
            .get(&global_key)
            .unwrap_or_else(|| panic!("{:?}", FarmError::ProductDoesNotExist));

        let prod_key = (Symbol::new(&env, "prod"), owner.clone(), local_idx);
        let mut product: FarmProduct = env
            .storage()
            .persistent()
            .get(&prod_key)
            .unwrap_or_else(|| panic!("{:?}", FarmError::ProductDoesNotExist));

        if product.sold {
            panic!("{:?}", FarmError::ProductAlreadySold);
        }
        if amount != product.product_price {
            panic!("{:?}", FarmError::PriceMismatch);
        }

        let purchase_key = (Symbol::new(&env, "purch"), caller.clone(), product_id);
        if env.storage().persistent().has(&purchase_key) {
            panic!("{:?}", FarmError::AlreadyPurchased);
        }

        product.sold = true;
        env.storage().persistent().set(&prod_key, &product);
        env.storage().persistent().set(&purchase_key, &true);

        let p_count_key = (Symbol::new(&env, "p_cnt"), caller.clone());
        let p_count: u32 = env.storage().persistent().get(&p_count_key).unwrap_or(0);
        let p_prod_key = (Symbol::new(&env, "p_prod"), caller.clone(), p_count);
        env.storage().persistent().set(&p_prod_key, &product);
        env.storage().persistent().set(&p_count_key, &(p_count + 1));

        let mut total_sales: i128 = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "total_sales"))
            .unwrap_or(0);
        total_sales += amount;
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "total_sales"), &total_sales);
    }

    pub fn remove_from_cart(env: Env, caller: Address, product_id: u32) {
        caller.require_auth();

        let cart_count_key = (Symbol::new(&env, "cart_c"), caller.clone());
        let count: u32 = env.storage().temporary().get(&cart_count_key).unwrap_or(0);

        let mut found_idx: Option<u32> = None;
        for i in 0..count {
            let cart_key = (Symbol::new(&env, "cart"), caller.clone(), i);
            if let Some(p) = env.storage().temporary().get::<_, FarmProduct>(&cart_key) {
                if p.product_id == product_id {
                    found_idx = Some(i);
                    break;
                }
            }
        }

        let idx = found_idx.unwrap_or_else(|| panic!("{:?}", FarmError::ProductNotInCart));

        let last_idx = count - 1;
        if idx != last_idx {
            let last_key = (Symbol::new(&env, "cart"), caller.clone(), last_idx);
            let last_product: FarmProduct = env.storage().temporary().get(&last_key).unwrap();
            let swap_key = (Symbol::new(&env, "cart"), caller.clone(), idx);
            env.storage().temporary().set(&swap_key, &last_product);
        }
        let last_key = (Symbol::new(&env, "cart"), caller.clone(), last_idx);
        env.storage().temporary().remove(&last_key);
        env.storage()
            .temporary()
            .set(&cart_count_key, &(count - 1));
    }

    pub fn get_purchased_products(env: Env, buyer: Address) -> Vec<FarmProduct> {
        let p_count_key = (Symbol::new(&env, "p_cnt"), buyer.clone());
        let count: u32 = env.storage().persistent().get(&p_count_key).unwrap_or(0);
        let mut items = Vec::new(&env);
        for i in 0..count {
            let p_prod_key = (Symbol::new(&env, "p_prod"), buyer.clone(), i);
            if let Some(p) = env.storage().persistent().get::<_, FarmProduct>(&p_prod_key) {
                items.push_back(p);
            }
        }
        items
    }

    pub fn has_purchased(env: Env, buyer: Address, product_id: u32) -> bool {
        let purchase_key = (Symbol::new(&env, "purch"), buyer, product_id);
        env.storage().persistent().get(&purchase_key).unwrap_or(false)
    }

    pub fn submit_review(env: Env, caller: Address, product_id: u32, review_text: String) {
        caller.require_auth();

        let purchase_key = (Symbol::new(&env, "purch"), caller.clone(), product_id);
        if !env.storage().persistent().get::<_, bool>(&purchase_key).unwrap_or(false) {
            panic!("{:?}", FarmError::OnlyBuyersCanReview);
        }

        let has_rev_key = (Symbol::new(&env, "has_rev"), product_id, caller.clone());
        if env.storage().persistent().get::<_, bool>(&has_rev_key).unwrap_or(false) {
            panic!("{:?}", FarmError::AlreadyReviewed);
        }

        let review = Review {
            reviewer: caller.clone(),
            review: review_text,
        };

        let rev_count_key = (Symbol::new(&env, "rev_c"), product_id);
        let rev_idx: u32 = env.storage().persistent().get(&rev_count_key).unwrap_or(0);

        let rev_key = (Symbol::new(&env, "rev"), product_id, rev_idx);
        env.storage().persistent().set(&rev_key, &review);
        env.storage().persistent().set(&rev_count_key, &(rev_idx + 1));
        env.storage().persistent().set(&has_rev_key, &true);

        env.events().publish(
            (Symbol::new(&env, "product"), Symbol::new(&env, "reviewed")),
            (caller, product_id),
        );
    }

    pub fn get_product_reviews(env: Env, product_id: u32) -> Vec<Review> {
        let rev_count_key = (Symbol::new(&env, "rev_c"), product_id);
        let count: u32 = env.storage().persistent().get(&rev_count_key).unwrap_or(0);
        let mut reviews = Vec::new(&env);
        for i in 0..count {
            let rev_key = (Symbol::new(&env, "rev"), product_id, i);
            if let Some(r) = env.storage().persistent().get::<_, Review>(&rev_key) {
                reviews.push_back(r);
            }
        }
        reviews
    }

    pub fn get_name(env: Env, user: Address) -> String {
        let farmer_key = (Symbol::new(&env, "farmer"), user);
        let farmer: Farmer = env
            .storage()
            .persistent()
            .get(&farmer_key)
            .unwrap_or_else(|| panic!("{:?}", FarmError::NotRegistered));
        farmer.business_name
    }

    pub fn get_address(env: Env, name: String) -> Address {
        let farm_id: u32 = Self::get_farm_index(env.clone(), name);
        let farm_key = (Symbol::new(&env, "farm_id"), farm_id);
        let farmer: Farmer = env
            .storage()
            .persistent()
            .get(&farm_key)
            .unwrap_or_else(|| panic!("{:?}", FarmError::FarmNotFound));
        farmer.farmer_address
    }

    pub fn get_user(env: Env, caller: Address) -> Farmer {
        caller.require_auth();
        let farmer_key = (Symbol::new(&env, "farmer"), caller);
        env.storage()
            .persistent()
            .get(&farmer_key)
            .unwrap_or_else(|| panic!("{:?}", FarmError::NotRegistered))
    }

    pub fn get_image(env: Env, user: Address) -> String {
        let farmer_key = (Symbol::new(&env, "farmer"), user);
        let farmer: Farmer = env
            .storage()
            .persistent()
            .get(&farmer_key)
            .unwrap_or_else(|| panic!("{:?}", FarmError::NotRegistered));
        farmer.business_image
    }

    pub fn get_all_farms(env: Env) -> Vec<Farmer> {
        let farm_counter: u32 = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "farm_ctr"))
            .unwrap_or(0);
        let mut farms = Vec::new(&env);
        for i in 1..=farm_counter {
            let farm_key = (Symbol::new(&env, "farm_id"), i);
            if let Some(f) = env.storage().persistent().get::<_, Farmer>(&farm_key) {
                farms.push_back(f);
            }
        }
        farms
    }

    pub fn get_total_sales(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&Symbol::new(&env, "total_sales"))
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod test;
