use crate::Contract;
use crate::ContractExt;


use near_sdk::serde::Deserialize;
// use near_sdk::env::log;
use near_sdk::serde::Serialize;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen, AccountId, Promise, Balance};
use near_sdk::json_types::U128;
use near_sdk::json_types::U64;

// pub const STORAGE_COST: u128 = 1_000_000_000_000_000_000_000;


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
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

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct CouponKey {
    product_id: String,
    code: String, 
    seller: AccountId
  }

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Coupon {
  code: String,
  product_id: String,
  discount_amount: u128, 
  allowed_uses: u128, // if allowed_uses = 0 => coupon is invalid
  seller: AccountId 
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ReviewTrackingKey {
    product_id: String,
    reviewer: AccountId
  }

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone )]
#[serde(crate = "near_sdk::serde")]
pub struct Review {
  product_id: String,
  reviewer: AccountId, 
  content: String,
  star: u64
}


#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ProductJson {
  id: String,
  name: String,
  description: String, 
  img: String, 
  is_active: bool,
  // price: U128,
  seller: AccountId,
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

    log!("{} buying product {} ", buyer, product.name);
    
    
    // create product for the first time
    if self.buyer_addresses.get(&product_id.clone()) == None {
      self.buyer_addresses.insert(&product_id.clone(),&vec![env::predecessor_account_id()]);
    } else {
      let mut current_buyer_ids = self.buyer_addresses.get(&product_id.clone()).unwrap();
      current_buyer_ids.push(env::predecessor_account_id());
      self.buyer_addresses.insert(&product_id.clone(),&current_buyer_ids);
    }

    // Buyer sends Near to seller 
    Promise::new(product.seller).transfer(product.price);
    true 
  }


  pub fn create_product(&mut self, id: String, name: String, price: U128, description: String, img: String, is_active: bool) -> Product {
    
    assert!( self.products.get(&id.clone()).is_none() == true, "This product is is exists already");

    let seller: AccountId = env::predecessor_account_id();
    
    let new_product = Product {
      id: String::from(id),
      name: String::from(name),
      // price: price.parse::<u128>().unwrap(),
      price: u128::from(price),
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
    self.product_list.push(new_product.id.clone());
  
    new_product
  }

  pub fn update_product(&mut self, id: String, name: String, price: U128, description: String, img: String, is_active: bool) -> Product {
    
    assert!( self.products.get(&id.clone()).is_some() == true, "Product with this id is not exist");
    let product: Product = self.products.get(&id.clone()).unwrap();
    assert!( product.seller == env::predecessor_account_id(), "You are not the product's owner");
    let updated_product = Product {
      id: product.id,
      name: String::from(name),
      price: u128::from(price),
      description: String::from(description),
      img: String::from(img),
      is_active: is_active,
      seller: product.seller
    };
    self.products.insert(&id.clone(),&updated_product);

    // add product by seller 
    updated_product
  }

  pub fn create_coupon(&mut self, product_id: String, code: String, allowed_uses: U128, discount_amount: U128) -> Coupon { 
    assert!( self.products.get(&product_id.clone()).is_some() == true, "Product with this id is not exist");
    let product: Product = self.products.get(&product_id.clone()).unwrap();
    assert!( product.seller == env::predecessor_account_id(), "You are not the product's owner");

    assert!( self.coupons.get(&CouponKey {
      product_id: product_id.clone(),
      code: code.clone(),
      seller: product.seller.clone(),
    }).is_none() == true, "This coupon for this product is already exist");
      let new_coupon = Coupon {
        product_id: product_id,
        code: code,
        discount_amount: u128::from(discount_amount),
        allowed_uses: u128::from(allowed_uses), 
        seller: product.seller
      };
      self.coupons.insert(&CouponKey {
        product_id: new_coupon.product_id.clone(),
        code: new_coupon.code.clone(),
        seller: new_coupon.seller.clone(),
      }, &new_coupon);
      
      // create coupon for the first time
      if self.coupons_by_seller.get(&env::predecessor_account_id()).is_none() == true {
        self.coupons_by_seller.insert(&env::predecessor_account_id(),&vec![CouponKey {
          product_id: new_coupon.product_id.clone(),
          code: new_coupon.code.clone(),
          seller: new_coupon.seller.clone(),
        }]);
      } else {
        let mut current_coupons = self.coupons_by_seller.get(&env::predecessor_account_id()).unwrap();
        current_coupons.push(CouponKey {
          product_id: new_coupon.product_id.clone(),
          code: new_coupon.code.clone(),
          seller: new_coupon.seller.clone(),
        });
        self.coupons_by_seller.insert(&env::predecessor_account_id(), &current_coupons);
      }

      new_coupon
  }

  pub fn update_coupon(&mut self, product_id: String, code: String, allowed_uses: U128, discount_amount: U128) -> Coupon { 
    assert!( self.products.get(&product_id.clone()).is_some() == true, "Product with this id is not exist");
    
    let product: Product = self.products.get(&product_id.clone()).unwrap();

    assert!( product.seller == env::predecessor_account_id(), "You are not the product's owner");

    assert!( self.coupons.get(&CouponKey {
      product_id: product_id.clone(),
      code: code.clone(),
      seller: product.seller.clone(),
    }).is_some() == true, "This coupon for this product is already exist");


    // assert!( self.coupons.get(&CouponKey {
    //   product_id: product_id.clone(),
    //   code: code.clone(),
    //   seller: product.seller.clone(),
    // }).is_some() == true, "Coupon is not exist");

    // let current_coupon = self.coupons.get(&CouponKey {
    //   product_id: product_id.clone(),
    //   code: code.clone(),
    //   seller: product.seller.clone(),
    // }).unwrap();

   
      let updated_coupon = Coupon {
        product_id: product_id,
        code: code,
        discount_amount: u128::from(discount_amount),
        allowed_uses: u128::from(allowed_uses), 
        seller: product.seller
      };
      self.coupons.insert(&CouponKey {
        product_id: updated_coupon.product_id.clone(),
        code: updated_coupon.code.clone(),
        seller: updated_coupon.seller.clone(),
      }, &updated_coupon);
      updated_coupon
  }

  pub fn add_review(&mut self, product_id: String, content: String, star: U64) -> bool { 
    assert!( self.products.get(&product_id.clone()).is_some() == true, "Product with this id is not exist");
    let product: Product = self.products.get(&product_id.clone()).unwrap();
    assert!( product.seller != env::predecessor_account_id(), "You can't review your own product");

    let new_review = Review {
      product_id: product.id.clone(),
      reviewer: env::predecessor_account_id(),
      star: u64::from(star),
      content: content
    };

    if let Some(tracking) = self.review_tracking.get(&ReviewTrackingKey { 
      product_id: product.id.clone(),
      reviewer: env::predecessor_account_id()
      }) {
        assert!( tracking == false, "You already reviewd this product");
      } 

    
    let review_copy = new_review.clone();

    // if product has the first review
    if self.reviews.get(&product_id.clone()).is_none() == true {
      self.reviews.insert(&product_id.clone(),&vec![new_review.clone()]);
    }else {
      let mut current_reviews = self.reviews.get(&product_id.clone()).unwrap();
      current_reviews.push(new_review);
      self.reviews.insert(&product.id.clone(),&current_reviews);
    }  

    

    // if user has review the first product 
    if self.my_reviews.get(&env::predecessor_account_id()).is_none() == true {
      self.my_reviews.insert(&env::predecessor_account_id(),&vec![review_copy]);
    }else {
      let mut current_reviews = self.my_reviews.get(&env::predecessor_account_id()).unwrap();
      current_reviews.push(review_copy);
      self.my_reviews.insert(&env::predecessor_account_id(),&current_reviews);
    }

    // update review tracking of
    self.review_tracking.insert(&ReviewTrackingKey { 
        product_id: product.id.clone(),
        reviewer: env::predecessor_account_id()
    }, &true);

    true
  }

  
  pub fn get_all_products(&self) -> Vec<String> {
    self.product_list.clone()
  }

  pub fn get_product(&self, product_id: String) -> Option<Product> { 
    self.products.get(&product_id)
  }

  pub fn get_seller_product(&self, seller: AccountId) -> Option<Vec<String>> {
    self.products_by_sellers.get(&seller.clone())
  }
  // get list buyers of a product
  pub fn get_buyer_addresses(&self, product_id: String) -> Option<Vec<AccountId>> {
    self.buyer_addresses.get(&product_id.clone())
  }
  // get list coupons of a seller has created 
  pub fn get_seller_coupons(&self, seller: AccountId) -> Option<Vec<CouponKey>> {
    self.coupons_by_seller.get(&seller)
  }
  // get review list of a product
  pub fn get_reviews(&self, product_id: String) -> Option<Vec<Review>> {
    self.reviews.get(&product_id)
  }
  // get all review from a user
  pub fn get_my_reviews(&self, reviewer: AccountId) -> Option<Vec<Review>> {
    self.my_reviews.get(&reviewer)
  }

  pub fn get_coupon_details(&self, product_id: String, code: String, seller: AccountId) -> Option<Coupon> {
    self.coupons.get(&CouponKey {
      product_id: product_id,
      code: code,
      seller: seller,
    })
  }

  pub fn get_product_json(&self, product_id: String) -> ProductJson {

    let product_data : Option<Product> = 
        if let Some(product_data) = self.products.get(&product_id) {
            Some( Product {
              id: product_data.id,
              name: product_data.name,
              price: product_data.price,
              description: product_data.description, 
              img: product_data.img, 
              is_active: product_data.is_active,
              seller: product_data.seller,
            })
        } else {
            None
        };
    let product_data = product_data.expect("Product is not exist");

    ProductJson {
          id: product_data.id,
          name: product_data.name,
          description: product_data.description, 
          img: product_data.img, 
          is_active: product_data.is_active,
          seller: product_data.seller

          // price: product_data.price.into(),
          
    }
    
  }
  

 
}