use scrypto::prelude::*;

// make a struct for content creator NFT
// Make CCNFT
// Make a CCNFT Vault
// 
// create hashmap for CCNFT and vault : You will need a Resourceaddress
// create a method for adding content creator
// create a method send_CC_money() for sending money to main vault and then to the CCNFT based vault
// send_CC_money(&self, CCNFT , amount) : will send money to main vault then main vault will send money to CCNFT based vauly

// ADMIN TOKEN for Auth for creation of Creator_NFT

/*
coding rules:
no resource , bucket etc should be left hanging
vaults are permenent storage and cannot be destroyed
process of making NFT:
1. create a struct for the nft ( this will be used to add vault manually into NFT, see scrypto101)
2. create a vault to store the NFT even if it;s mindted or not (badges can be transferred to vaults and user wallet too)
*/

/*
initialing the blueprint:
you have to see the Struct YTFAIR class and make sure that initialising function intialises all the struct parameneter, the Self beofre the initialiser will check this class tyoes match
ALL IN STRUCT SHOULD BE MENTINED IN INTIALING FUNCTION
*/


// STRUCT FOR Creator_nft
#[derive(NonFungibleData)]
struct ShareHolder {
    /// A struct field which defines the amount of shares owned by this shareholder
    amount_of_shares: Decimal,
}


blueprint! {
    struct YtFair {
        
    collected_xrd: Vault,
    shareholder_badge_resource_address: ResourceAddress,
    internal_admin_badge: Vault,
    vaults: HashMap<NonFungibleId, Vault>,
    dead_vaults: Vec<Vault>,
    is_locked: bool,
    total_amount_of_shares: Decimal,

    // accepted_token_resource_address: ResourceAddress,    
}

    impl YtFair 
    {
        pub fn instantiate_ytfair() -> (ComponentAddress, Bucket)
        {   
            // ComponentAddress, Buckets
            // Creating the admin badge which will allow for adding shareholders and locking of the payment splitter
            let admin_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Admin Badge")
                .metadata(
                    "description",
                    "This is a PaymentSplitter admin badge used to authenticate the admin.",
                )
                .initial_supply(dec!("1"));

            let internal_admin_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Internal Admin Badge")
                .metadata("description", "An internal admin badge used for internal functionality of the PaymentSplitter.")
                .initial_supply(dec!("1"));
            
            let shareholder_badge: ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", "Shareholder Badge")
                .metadata(
                    "description",
                    "A non-fungible-token used to authenticate shareholders.",
                )
                .mintable(
                    rule!(require(internal_admin_badge.resource_address())),
                    Mutability::LOCKED,
                )
                .burnable(
                    rule!(require(internal_admin_badge.resource_address())),
                    Mutability::LOCKED,
                )
                .no_initial_supply();
            


        let mut payment_splitter: YtFairComponent = Self {
            collected_xrd: Vault::new(RADIX_TOKEN),
            // accepted_token_resource_address: accepted_token_resource_address,
            shareholder_badge_resource_address: shareholder_badge,
            internal_admin_badge: Vault::with_bucket(internal_admin_badge),
            vaults: HashMap::new(),
            dead_vaults: Vec::new(),
            is_locked: false,
            total_amount_of_shares: dec!("0"),
        }
        .instantiate();

        return (payment_splitter.globalize(), admin_badge);
            
}
            
            pub fn deposit(&mut self, mut payment: Bucket) -> () {
            // take all the money sent in the bucket in XRD
            // if the caller has sent something other than XRD, they'll get a runtime error           
            self.collected_xrd.put(payment);
            
        }
    }
}