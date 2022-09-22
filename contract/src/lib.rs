use paydii::{Product, CouponKey, Coupon};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey};
use near_sdk::collections::{UnorderedMap};

mod paydii;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
  // pub beneficiary: AccountId,
  pub donations: UnorderedMap<AccountId, u128>,
  pub products: UnorderedMap<String, Product>, // all products
  pub products_by_sellers: UnorderedMap<AccountId, Vec<String>>, // products created by one seller
  pub buyer_addresses: UnorderedMap<String, Vec<AccountId>>, // one product is purchased by many buyers
  pub coupons: UnorderedMap<CouponKey, Coupon>,
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    Product,
    ProductBySeller,
    BuyerAddresses,
    Coupons,
    ByOwnerId,
    Offers,
    ParasNFTContractIds,
    MarketV2,
    MarketV3,
    OffersV2,
    ParasNFTContractIdsV2,
    Trade,
    MarketDataTransactionFee
}

impl Default for Contract {
  fn default() -> Self {
    Self{
      // beneficiary: "v1.faucet.nonofficial.testnet".parse().unwrap(),
      donations: UnorderedMap::new(b"d"),
      products: UnorderedMap::new(StorageKey::Product),
      products_by_sellers: UnorderedMap::new(StorageKey::ProductBySeller),
      buyer_addresses: UnorderedMap::new(StorageKey::BuyerAddresses),
      coupons: UnorderedMap::new(StorageKey::Coupons)
    }
  }
}

#[near_bindgen]
impl Contract {
  #[init]
  #[private] // Public - but only callable by env::current_account_id()
  pub fn init() -> Self {
    assert!(!env::state_exists(), "Already initialized");
    Self {
      // beneficiary,
      donations: UnorderedMap::new(b"d"),
      products: UnorderedMap::new(StorageKey::Product),
      products_by_sellers: UnorderedMap::new(StorageKey::ProductBySeller),
      buyer_addresses: UnorderedMap::new(StorageKey::BuyerAddresses),
      coupons: UnorderedMap::new(StorageKey::Coupons)
    }
  }

  // // Public - beneficiary getter
  // pub fn get_beneficiary(&self) -> AccountId {
  //   self.beneficiary.clone()
  // }

  // // Public - but only callable by env::current_account_id(). Sets the beneficiary
  // #[private]
  // pub fn change_beneficiary(&mut self, beneficiary: AccountId) {
  //   self.beneficiary = beneficiary;
  // }
}


#[cfg(test)]
mod tests {
  use super::*;
  use near_sdk::testing_env;
  use near_sdk::test_utils::VMContextBuilder;
  use near_sdk::Balance;

  const BENEFICIARY: &str = "beneficiary";
  const NEAR: u128 = 1000000000000000000000000;

  #[test]
  // fn initializes() {
  //     let contract = Contract::init(BENEFICIARY.parse().unwrap());
  //     assert_eq!(contract.beneficiary, BENEFICIARY.parse().unwrap())
  // }

  #[test]
  fn donate() {
      // let mut contract = Contract::init(BENEFICIARY.parse().unwrap());

      // // Make a donation
      // set_context("donor_a", 1*NEAR);
      // contract.donate();
      // let first_donation = contract.get_donation_for_account("donor_a".parse().unwrap());

      // // Check the donation was recorded correctly
      // assert_eq!(first_donation.total_amount.0, 1*NEAR);

      // // Make another donation
      // set_context("donor_b", 2*NEAR);
      // contract.donate();
      // let second_donation = contract.get_donation_for_account("donor_b".parse().unwrap());

      // // Check the donation was recorded correctly
      // assert_eq!(second_donation.total_amount.0, 2*NEAR);

      // // User A makes another donation on top of their original
      // set_context("donor_a", 1*NEAR);
      // contract.donate();
      // let first_donation = contract.get_donation_for_account("donor_a".parse().unwrap());

      // // Check the donation was recorded correctly
      // assert_eq!(first_donation.total_amount.0, 1*NEAR * 2);

      // assert_eq!(contract.number_of_donors(), 2);
  }

  // Auxiliar fn: create a mock context
  fn set_context(predecessor: &str, amount: Balance) {
    let mut builder = VMContextBuilder::new();
    builder.predecessor_account_id(predecessor.parse().unwrap());
    builder.attached_deposit(amount);

    testing_env!(builder.build());
  }
}