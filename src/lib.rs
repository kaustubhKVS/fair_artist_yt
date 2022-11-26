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

// STRUCT FOR video_nft
#[derive(NonFungibleData)]
struct VideoNFT {
    video_title:String,
    content_creator:String,
    video_url: String,
    likes:u64,
    views:u64
}


blueprint!{
    struct YtFair {
        
    collected_xrd: Vault,
    shareholder_badge_resource_address: ResourceAddress,
    internal_admin_badge: Vault,
    cc_vaults: HashMap<NonFungibleId, Vault>,
    video_vault : Vault,
    
    //     hashmap<nonfungible ,  Vec<> >

    // videoNFT_vault: Vault,
    // videoNFid_array: Vec<NonFungibleId>,

    // videoNFT_vault: Vault,
    // videoNFid_array: Vec<NonFungibleId>,
    // // video ,cc, viwer
    
    // // video, cc :
    //     keep track ki konsa video banaya hanging
    //     video_ownership: HashMap<videonftID(NonFungibleId) , cc_NFT(NonFungibleId)>

    // // video, viwer:
    //     likes, views
    
    // // cc , viewer:
    //     viwer sends money to cc:
    //     deposit input: videonftID
    //     output: bucket
    //     process:
    //         hit video_ownership and get cc_NFT
    //         hit cc_vault and get CC ke vault ka resource address
    //         send bucket to that resource address
          

    // videoNFT_vault: Vault,    
    // videoNFT_vault: Vec<NonFungibleId>,

    dead_vaults: Vec<Vault>,
    is_locked: bool,
    total_amount_of_shares: Decimal,
    video_nft:ResourceAddress,
    random_card_id_counter:u64,

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
            
            
            let video_nft = ResourceBuilder::new_non_fungible()
                .metadata("name", "Video NFT")
                .metadata(
                    "description",
                    "A non-fungible-token used to represent videos.",
                )
                .mintable(
                    rule!(allow_all),
                    Mutability::LOCKED,
                )
                .burnable(
                    rule!(allow_all),
                    Mutability::LOCKED,
                )
                .no_initial_supply();
            // random_card_id_counter=0;
                //create vault
        
        

        //Self instantiate
        let mut payment_splitter: YtFairComponent = Self {
            collected_xrd: Vault::new(RADIX_TOKEN),
            // accepted_token_resource_address: accepted_token_resource_address,
            shareholder_badge_resource_address: shareholder_badge,
            internal_admin_badge: Vault::with_bucket(internal_admin_badge),
            cc_vaults: HashMap::new(),
            video_nft:video_nft,
            video_vault :  Vault::new(video_nft),
            dead_vaults: Vec::new(),
            // video_nfts: Vec::new(),
            is_locked: false,
            total_amount_of_shares: dec!("0"),
            
            random_card_id_counter:0,
        }
        .instantiate();

        return (payment_splitter.globalize(), admin_badge);
            
}
            
            pub fn deposit(&mut self, mut payment: Bucket) -> () {
            // take all the money sent in the bucket in XRD
            // if the caller has sent something other than XRD, they'll get a runtime error           
            self.collected_xrd.put(payment);
            
        }

        // pub fn make_video_nft(&mut self,mut title:String,mut desc:String, mut url:String, mut ContentCreatorAddress:String) -> ()
        pub fn make_video_nft(&mut self) -> ()
        {
            let vidz = VideoNFT {
                video_title:"New video".to_string(),
                content_creator:"Shivam".to_string(),
                video_url: "www.google.com".to_string(),
                likes:0,
                views:0
            };
            // let nft_bucket:Bucket = self.internal_admin_badge(borrow_resource_manager!(self.video_nft).mint_non_fungible(
            //     &NonFungibleId::from_u64(self.random_card_id_counter),
            //     new_card,
            // ));
            let nft_bucket = borrow_resource_manager!(self.video_nft).mint_non_fungible(
                &NonFungibleId::from_u64(self.random_card_id_counter),
                vidz,
            );
            self.random_card_id_counter += 1;
            self.video_vault.put(nft_bucket)

    }

       pub fn show_token_info(address: ResourceAddress) {
           // We borrow the resource manager of the provided address
            let manager: &ResourceManager = borrow_resource_manager!(address);
 
           // Get the resource type
           match manager.resource_type() {
               ResourceType::Fungible{divisibility} => {
                   info!("Fungible resource with divisibility of {}", divisibility)
               },
               ResourceType::NonFungible => {
                   info!("Non Fungible resource")
               }
           }
 
           // Get the total supply
           info!("Total supply: {}", manager.total_supply());
 
           // Get information stored in the metadata
           let metadata: HashMap<String, String> = manager.metadata();
           let token_name = metadata.get("name").expect("Token does not have a name");
           let token_symbol = metadata.get("symbol").expect("Token does not have a symbol");
           info!("Name: {}. Symbol: {}", token_name, token_symbol);
       } 
    }
}