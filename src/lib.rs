use scrypto::prelude::*;

/*
make a struct for content creator NFT
Make CCNFT
Make a CCNFT Vault

create hashmap for CCNFT and vault : You will need a Resourceaddress
create a method for adding content creator
create a method send_CC_money() for sending money to main vault and then to the CCNFT based vault
send_CC_money(&self, CCNFT , amount) : will send money to main vault then main vault will send money to CCNFT based vauly
*/


/*
ADMIN TOKEN for Auth for creation of Creator_NFT
*/

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

// STRUCT FOR cc_nft
#[derive(NonFungibleData)]
struct ccNFT {
    content_creator:String,
    subscribers: u64,
}

blueprint!{
    struct YtFair {
    
    // VAULTS
    collected_xrd_vault: Vault,
    cc_vaults: Vault,
    cc_vaults_hashmap: HashMap<NonFungibleId, Vault>,    
    video_vault : Vault,
    dead_vaults: Vec<Vault>,
    internal_admin_badge_vault: Vault,
    
    // NFTs and Badges
    shareholder_badge_resource_address: ResourceAddress,
    video_nft:ResourceAddress,
    cc_nft: ResourceAddress,
    
    // Int, string and other values
    is_locked: bool,
    random_card_id_counter:u64,
    total_amount_of_shares: Decimal,
    cc_username_cc_nftID_hashmap: HashMap<String, NonFungibleId>
    
    /*
    hashmap<nonfungible ,  Vec<> >
    */

    /*
    videoNFT_vault: Vault,
    videoNFid_array: Vec<NonFungibleId>,
    */

    /*
    videoNFT_vault: Vault,
    videoNFid_array: Vec<NonFungibleId>,
    video ,cc, viwer
    */
    
    /*
        video, cc :
        keep track ki konsa video banaya hanging
        video_ownership: HashMap<videonftID(NonFungibleId) , cc_NFT(NonFungibleId)>
    */

    /*
    video, viwer:
    likes, views
    */
    
    /*
        cc , viewer:
        viwer sends money to cc:
        deposit input: videonftID
        output: bucket
        process:
            hit video_ownership and get cc_NFT
            hit cc_vault and get CC ke vault ka resource address
            send bucket to that resource address
    */
          

    /*
    videoNFT_vault: Vault,    
    videoNFT_vault: Vec<NonFungibleId>,
    */

    

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

            let internal_admin_badge_bucket: Bucket = ResourceBuilder::new_fungible()
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
                    rule!(require(internal_admin_badge_bucket.resource_address())),
                   Mutability::LOCKED,                    
                )
                .burnable(
                    rule!(require(internal_admin_badge_bucket.resource_address())),
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

                let cc_nft = ResourceBuilder::new_non_fungible()
                .metadata("name", "CC NFT")
                .metadata(
                    "description",
                    "A non-fungible-token used to represent content creators.",
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
        

// INSTANTIATING THE RESOURCES
        let mut payment_splitter: YtFairComponent = Self {
            
            // Instantiate Vaults
            collected_xrd_vault: Vault::new(RADIX_TOKEN),
            internal_admin_badge_vault: Vault::with_bucket(internal_admin_badge_bucket),
            
            cc_vaults_hashmap: HashMap::new(),
            cc_vaults: Vault::new(cc_nft),

            video_vault :  Vault::new(video_nft),
            dead_vaults: Vec::new(),

            // Instantiate Nfts and Badges
            shareholder_badge_resource_address: shareholder_badge,
            video_nft:video_nft,
            cc_nft: cc_nft,
            
            // Instantiate int , str, and other datatypes
            is_locked: false,
            total_amount_of_shares: dec!("0"),
            random_card_id_counter:0,
            cc_username_cc_nftID_hashmap: HashMap::new(),
        }
        .instantiate();

        return (payment_splitter.globalize(), admin_badge);
            
}
            





// METHOD: deposit function to deposit the money
        pub fn deposit(&mut self, mut payment: Bucket) -> () {
            // take all the money sent in the bucket in XRD
            // if the caller has sent something other than XRD, they'll get a runtime error           
            self.collected_xrd_vault.put(payment);
            
        }



// METHOD: minting of video NFTs when videos are uploaded
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
            // let nft_bucket:Bucket = self.internal_admin_badge_vault(borrow_resource_manager!(self.video_nft).mint_non_fungible(
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

// METHOD: minting of video NFTs when videos are uploaded
// pub fn make_video_nft(&mut self,mut title:String,mut desc:String, mut url:String, mut ContentCreatorAddress:String) -> ()
pub fn make_cc_nft_cc_vault(&mut self , cc_name: String ) -> ()
{

    let cc_details = ccNFT {
        content_creator: cc_name.clone(),
        subscribers:0
    };
    
    let cc_nft_id: NonFungibleId = NonFungibleId::from_u64(self.random_card_id_counter);
    let _cc_nft_id_clone = cc_nft_id.clone();

    let cc_nft_bucket = borrow_resource_manager!(self.cc_nft).mint_non_fungible(
                                                                                &cc_nft_id,
                                                                                cc_details,
                                                                                );

    // Creating a vault for the shareholder
    self.cc_vaults_hashmap.insert(
        cc_nft_id,
        Vault::new(RADIX_TOKEN),
    );
    
    info!("Adding a new content creator with {} name", &cc_name);
    
    // DNS service type thing for inputting username of content creator and outputting the cc_NFT id of the content creator
    // input username
    // output cc_NFT_ID
    self.cc_username_cc_nftID_hashmap.insert(
        cc_name,
        _cc_nft_id_clone ,
    );

    // incrementing the random counter
    self.random_card_id_counter += 1;

    // adding the cc_NFT to cc_vault 
    self.cc_vaults.put(cc_nft_bucket)

}

// METHOD: SHOWING INFORMATION IN THE TOKEN
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