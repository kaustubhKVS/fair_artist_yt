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




/*
Adding comments to videoNFT using HashMap of <ViewerToken , CommentString>
*/

/*
        video_url_list = vector<url1,url2,url3,url4, .........>
        video_url_VideoNFTID_hashmap< url , NFTID  >
        videonftID_ccNFTID_hashmap<   ,   >
        ccNFT_VideoNFT_hashmap<   ,   >
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
    #[scrypto(mutable)]
    likes:u64,
    #[scrypto(mutable)]
    views:u64,

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
    random_videonft_id_counter:u64,
    cc_account_id_counter:u64,
    total_amount_of_shares: Decimal,

    // HashMaps for Corelations
    cc_username_cc_nftID_hashmap: HashMap<String, NonFungibleId>,
    video_url_videoNFTID_hashmap: HashMap<String, NonFungibleId>,
    videonftID_ccNFTID_hashmap: HashMap<NonFungibleId, String>,
    ccNFT_VideoNFT_hashmap: HashMap<NonFungibleId, NonFungibleId>,
    
    // ContentCreator Information array
    cc_username_list: Vec<String>,
    cc_nftID_list: Vec<NonFungibleId>,
    
    // Video Information array 
    video_url_list: Vec<String>,
    video_nftID_list: Vec<NonFungibleId>,
    
    

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
                .updateable_non_fungible_data(
                    rule!(allow_all),
                    LOCKED,
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
        let mut YtFairInitialiser: YtFairComponent = Self {
            
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
            random_videonft_id_counter:0,
            cc_account_id_counter:0,

            // HashMaps for Corelations
            cc_username_cc_nftID_hashmap: HashMap::new(),
            video_url_videoNFTID_hashmap: HashMap::new(),
            videonftID_ccNFTID_hashmap: HashMap::new(),
            ccNFT_VideoNFT_hashmap: HashMap::new(),
    
            // Vectors which are array
            cc_username_list: Vec::new(), 
            cc_nftID_list: Vec::new(),
    
            // Video Information array 
            video_url_list: Vec::new(),
            video_nftID_list: Vec::new(),

        }
        .instantiate();

        return (YtFairInitialiser.globalize(), admin_badge);
            
}
            

// METHOD: deposit function to deposit the money
        pub fn deposit(&mut self, mut payment: Bucket) -> () {
            // take all the money sent in the bucket in XRD
            // if the caller has sent something other than XRD, they'll get a runtime error           
            self.collected_xrd_vault.put(payment);
            
        }

// METHOD: Make Content Creator NFT and Creating Content Creator Vault
        pub fn make_cc_nft_cc_vault(&mut self , cc_name: String ) -> ()
        {
        
            let cc_details = ccNFT {
                content_creator: cc_name.clone(),
                subscribers:0
            };
            
            let cc_nft_id: NonFungibleId = NonFungibleId::from_u64(self.cc_account_id_counter);
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
                cc_name.clone(),
                _cc_nft_id_clone ,
            );

            self.cc_username_list.push(cc_name);
            self.cc_nftID_list.push(NonFungibleId::from_u64(self.cc_account_id_counter));

            // incrementing the random counter
            self.cc_account_id_counter += 1;
        
            // adding the cc_NFT to cc_vault 
            self.cc_vaults.put(cc_nft_bucket)
        
        }

// METHOD: Minting of video NFTs when videos are uploaded
    pub fn make_video_nft(&mut self,video_title : String, content_creator:String, video_url : String) -> ()
        {   
            // Check if the User Exists in the system


            // creating a Initialiaser Struct for Incoming Video 
            let vidz = VideoNFT {
                video_title:video_title.clone(),
                content_creator:content_creator.clone(),
                video_url: video_url.clone(),
                likes:0,
                views:0
            };
            
            // Minting the VideoNFT using the VideoNFT struct
            let nft_bucket = borrow_resource_manager!(self.video_nft).mint_non_fungible( &NonFungibleId::from_u64(self.random_videonft_id_counter),
                                                                                                                                            vidz,);
            

            

            self.video_vault.put(nft_bucket);
            // info!("created NFT with id counter {}, NFT id {} and title {} created by content creator {}",self.random_card_id_counter,NonFungibleId::from_u64(self.random_card_id_counter),vidz.video_title,vidz.content_creator);
            
            self.random_videonft_id_counter += 1;
            
    }

    pub fn update_video_nft_likes(&mut self,NFTID:u64) -> ()
    {
        
        let nonfungtok_id_BTreeSet =self.video_vault.non_fungible_ids(); 
        let actual_nft_id = nonfungtok_id_BTreeSet.get(&NonFungibleId::from_u64(NFTID)).unwrap();
        
        info!("NFT ID of the video to Liked {:?}",actual_nft_id);
        let mut temp_nftdata:VideoNFT= borrow_resource_manager!(self.video_nft).get_non_fungible_data(actual_nft_id);
        let updated_videoNFT = VideoNFT {
            video_title:temp_nftdata.video_title,
            content_creator:temp_nftdata.content_creator,
            video_url: temp_nftdata.video_url,
            likes:temp_nftdata.likes+1,
            views:temp_nftdata.views
        };

        borrow_resource_manager!(self.video_nft).update_non_fungible_data(actual_nft_id,updated_videoNFT);
        // self.random_card_id_counter += 1;
    }

    pub fn update_video_nft_views(&mut self,NFTID:u64) -> ()
    {
        
        let nonfungtok_id_BTreeSet =self.video_vault.non_fungible_ids(); 
        let actual_nft_id = nonfungtok_id_BTreeSet.get(&NonFungibleId::from_u64(NFTID)).unwrap();
        // let rand_varss = borrow_resource_manager!(temp_var);
        info!("NFT ID of the video to Viewed  {:?}",actual_nft_id);
        let mut temp_nftdata:VideoNFT= borrow_resource_manager!(self.video_nft).get_non_fungible_data(actual_nft_id);
        let updated_videoNFT = VideoNFT {
            video_title:temp_nftdata.video_title,
            content_creator:temp_nftdata.content_creator,
            video_url: temp_nftdata.video_url,
            likes:temp_nftdata.likes,
            views:temp_nftdata.views+1
        };
        
        borrow_resource_manager!(self.video_nft).update_non_fungible_data(actual_nft_id,updated_videoNFT);
    }
    
    
    //helper funciton
    // pub fn get_NFT_from_vault_using_NFTID(&mut self,NFTVault : Vault,nftReference : ResourceAddress,NFTID:u64 ) -> (NonFungible<T>)
    // {
    //     let nonfungtok_id_BTreeSet =NFTVault.non_fungible_ids(); 
    //     let temp_nft_id = nonfungtok_id_BTreeSet.get(&NonFungibleId::from_u64(NFTID)).unwrap();
    //     let temp_nftdata= borrow_resource_manager!(nftReference).get_non_fungible_data(temp_nft_id);
    //     return temp_nftdata;
    // }


// METHOD: minting of video NFTs when videos are uploaded
// pub fn make_video_nft(&mut self,mut title:String,mut desc:String, mut url:String, mut ContentCreatorAddress:String) -> ()





// METHOD: send money to content creator vaults
pub fn deposit_cc_nft_cc_vault(&mut self , cc_name: String, payment_bucket: Bucket ) -> ()
{
    let cc_username: String = cc_name.clone();
    
    let cc_nftID = self.cc_username_cc_nftID_hashmap.get(&cc_username).unwrap();

    info!("NFT ID of the {:?}",&cc_nftID);
    info!("Name of CC of the {:?}",&cc_username);

    // Getting the vault for the Content Creator
    let cc_ownership_vault: &mut Vault = self.cc_vaults_hashmap.get_mut(cc_nftID).unwrap();

    info!("VaultID of CC of the {:?}",&cc_ownership_vault);
    info!("Sending {} XRD to Content Creator {} are available.", payment_bucket.amount(), cc_username);

    // Sending the payment to the owner vault
    cc_ownership_vault.put(payment_bucket);

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
       
       pub fn fetch_video_details_and_update_view(&mut self,NFTID:u64) -> ()
       {
           
           let nonfungtok_id_BTreeSet =self.video_vault.non_fungible_ids(); 
           let actual_nft_id = nonfungtok_id_BTreeSet.get(&NonFungibleId::from_u64(NFTID)).unwrap();
           // let rand_varss = borrow_resource_manager!(temp_var);
           info!("NFT ID of the video to Viewed  {:?}",actual_nft_id);
           let mut temp_nftdata:VideoNFT= borrow_resource_manager!(self.video_nft).get_non_fungible_data(actual_nft_id);
           let updated_videoNFT = VideoNFT {
               video_title:temp_nftdata.video_title,
               content_creator:temp_nftdata.content_creator,
               video_url: temp_nftdata.video_url,
               likes:temp_nftdata.likes,
               views:temp_nftdata.views+1
           };
           
           borrow_resource_manager!(self.video_nft).update_non_fungible_data(actual_nft_id,updated_videoNFT);
       }   


    
        





    }
}