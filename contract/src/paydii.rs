use crate::Contract;
use crate::ContractExt;


// use near_sdk::env::log;
use near_sdk::serde::Serialize;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen, AccountId, Promise, Balance};
use near_sdk::json_types::U128;

// pub const STORAGE_COST: u128 = 1_000_000_000_000_000_000_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Donation {
  pub account_id: AccountId, 
  pub total_amount: U128,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Product {
  pub id: String,
  pub name: String,
  pub price: u128,
  pub description: String, 
  pub img: String, 
  pub is_active: bool,
  pub seller: AccountId,
}

#[near_bindgen]
impl Contract {
  

  #[payable] // Buy the product
  pub fn buy_product(&mut self, product_id: String) -> bool {
    
    assert!( self.products.get(&product_id).is_some() == true, "Can't find the product with id {}", product_id);


    let buyer: AccountId = env::predecessor_account_id(); 

    let product = self.products.get(&product_id).unwrap();

    assert!( product.seller != buyer, "You can't buy your own product");
    assert!( product.is_active == true, "Product is in-active");

    // log!("{} buying product {} ", buyer, product.name);
    
    
    // create product for the first time
    // if self.buyer_addresses.get(&product_id.clone()) == None {
    //   self.buyer_addresses.insert(&product_id.clone(),&vec![env::predecessor_account_id()]);
    // } else {
    //   let mut current_buyer_ids = self.buyer_addresses.get(&product_id.clone()).unwrap();
    //   current_buyer_ids.push(env::predecessor_account_id());
    //   self.buyer_addresses.insert(&product_id.clone(),&current_buyer_ids);
    // }

    // Buyer sends Near to seller 
    Promise::new(product.seller).transfer(product.price);
    true 
  }


  pub fn create_product(&mut self, id: String, name: String, price: String, description: String, img: String, is_active: bool) -> Product {
    
    assert!( self.products.get(&id.clone()).is_none() == true, "This product is is exists already");

    let seller: AccountId = env::predecessor_account_id();
    
    let new_product = Product {
      id: String::from(id),
      name: String::from(name),
      price: price.parse::<u128>().unwrap(),
      description: String::from(description),
      img: String::from(img),
      is_active: is_active,
      seller: seller
    };
    self.products.insert(&new_product.id.clone(),&new_product);

    // add product by seller 
    // create product for the first time
    if self.products_by_sellers.get(&env::predecessor_account_id()) == None {
      let new_product_ids = vec![new_product.id.clone()];
      self.products_by_sellers.insert(&env::predecessor_account_id(),&new_product_ids);

    // already has at least 1 product
    } else {
      let mut current_product_ids = self.products_by_sellers.get(&env::predecessor_account_id()).unwrap();
      current_product_ids.push(new_product.id.clone());
      self.products_by_sellers.insert(&env::predecessor_account_id(),&current_product_ids);
    }
  
    new_product
  }

  pub fn update_product(&mut self, id: String, name: String, price: String, description: String, img: String, is_active: bool) -> Product {
    
    assert!( self.products.get(&id.clone()).is_some() == true, "Product with this id is not exist");
    let product: Product = self.products.get(&id.clone()).unwrap();
    assert!( product.seller == env::predecessor_account_id(), "You are not the product's owner");
    let updated_product = Product {
      id: product.id,
      name: String::from(name),
      price: price.parse::<u128>().unwrap(),
      description: String::from(description),
      img: String::from(img),
      is_active: is_active,
      seller: product.seller
    };
    self.products.insert(&id.clone(),&updated_product);

    // add product by seller 
    updated_product
  }

  pub fn get_product(&self, product_id: String) -> Option<Product> {
    self.products.get(&product_id.clone())
  }
  pub fn get_seller_product(&self, seller: AccountId) -> Option<Vec<String>> {
    self.products_by_sellers.get(&seller.clone())
  }
  // get list buyers of a product
  pub fn get_buyer_addresses(&self, product_id: String) -> Option<Vec<AccountId>> {
    self.buyer_addresses.get(&product_id.clone())
  }

 
}