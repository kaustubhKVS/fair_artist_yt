use scrypto::prelude::*;

/*
ADMIN TOKEN for Auth for creation of Creator_NFT
*/

/*
Adding comments to videoNFT using HashMap of <ViewerToken , CommentString>
*/

/*
CODING RULES:
HANGING BUCKET: No resource , bucket etc should be left hanging
WHERE IS THE PERMENANT STORAGE?? : Vaults are permenent storage and cannot be destroyed
HOW TO MAKE NFT PROGRAMATICALLY:
1. create a struct for the nft ( this will be used to add vault manually into NFT, see scrypto101)
2. create a vault to store the NFT even if it;s mindted or not (badges can be transferred to vaults and user wallet too)
INITIALISATION HANDLING: ALL IN STRUCT SHOULD BE MENTINED IN INTIALING FUNCTION
you have to see the Struct YTFAIR class and make sure that initialising function intialises all the struct parameneter, the Self beofre the initialiser will check this class types match
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
        videonftID_ccNFTID_hashmap: HashMap<NonFungibleId, NonFungibleId>,
        ccNFTID_VideoNFTID_hashmap: HashMap<NonFungibleId, NonFungibleId>,
        // bhI id ka naam change kar dena
        // ContentCreator Information array
        cc_username_list: Vec<String>,
        cc_nftID_list: Vec<NonFungibleId>,
        
        // Video Information array 
        video_url_list: Vec<String>,
        video_nftID_list: Vec<NonFungibleId>,
     
    }

    impl YtFair 
    {
        pub fn instantiate_ytfair() -> (ComponentAddress, Bucket)
        {   
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
            ccNFTID_VideoNFTID_hashmap: HashMap::new(),
    
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
        pub fn deposit(&mut self, mut payment: Bucket) -> () 
        {
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
            // Declaring variables for further use
            let _cc_username_checker = content_creator.clone();
            let _temp_video_title = video_title.clone();
            let _clone_video_url = video_url.clone();

            // Check if the User Exists in the system
            assert!(if self.cc_username_list.iter().any(|i| i==&_cc_username_checker)
                        {
                            true
                        }
                    else
                        {
                            false
                        },
                        "[USER DOES NOT EXIST]: Can't Upload Video {} since User {} does not exist",
                        _temp_video_title,
                        _cc_username_checker
                        );

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

            let cc_username: String = content_creator.clone();
            
            // Using the Username in HashMap Fetching the ID of Content Creator NFT 
            let cc_nftID = self.cc_username_cc_nftID_hashmap.get(&cc_username).unwrap().clone();

            // Adding entries IPFS URLS to list
            self.video_url_list.push(_clone_video_url.clone());

            // Adding videoNFT ID to list
            self.video_nftID_list.push(NonFungibleId::from_u64(self.random_videonft_id_counter));
            
            // Adding association of IPFS Hashes to videonftID
            self.video_url_videoNFTID_hashmap.insert(_clone_video_url,NonFungibleId::from_u64(self.random_videonft_id_counter));
            
            // Adding association of ownership of videoNFTs to Content Creators
            self.videonftID_ccNFTID_hashmap.insert(NonFungibleId::from_u64(self.random_videonft_id_counter),cc_nftID.clone());
            
            // Adding association of ownership of videoNFTs to Content Creators
            self.ccNFTID_VideoNFTID_hashmap.insert(cc_nftID,NonFungibleId::from_u64(self.random_videonft_id_counter));

            // Adding VideoNFT to a Bucket
            self.video_vault.put(nft_bucket);
            // info!("created NFT with id counter {}, NFT id {} and title {} created by content creator {}",self.random_card_id_counter,NonFungibleId::from_u64(self.random_card_id_counter),vidz.video_title,vidz.content_creator);
            
            self.random_videonft_id_counter += 1;
            
        }

        //utility function
        fn get_random(&mut self, end: usize) -> usize {
            let num = Runtime::generate_uuid();
            (num % end as u128) as usize
        }

        pub fn playvideo_for_video_feed(&mut self)->(String, String, u64, u64, String, u64)
        {
            let choice = self.get_random(self.video_url_list.len());
            // let temp_list = self.video_url_list.clone();
            let temp_url = &self.video_url_list[choice];
            let vid_url_selected = temp_url.clone();
            return self.fetch_video_details_and_update_view(vid_url_selected);            
        }

        //THIS WILL BE THE TRIGGER FUNCTION FOR UPDATING LIKES TO THE FRONT END
        pub fn update_video_nft_likes_byurl(&mut self, vid_url:String)->()
        {
            let query_url = vid_url.clone();
            let temp_vid_id = self.video_url_videoNFTID_hashmap.get(&query_url).unwrap();
            // info!("{}",query_vid_id);
            let query_vid_id=temp_vid_id.clone();
            self.update_video_nft_likes(query_vid_id);
        }
        //THIS WILL BE THE TRIGGER FUNCTION FOR UPDATING VIEWS TO THE FRONT END
        pub fn update_video_nft_views_byurl(&mut self, vid_url:String)->()
        {
            let query_url = vid_url.clone();
            let temp_vid_id = self.video_url_videoNFTID_hashmap.get(&query_url).unwrap();
            // info!("{}",query_vid_id);
            let query_vid_id=temp_vid_id.clone();
            self.update_video_nft_views(query_vid_id);
        }

        pub fn update_video_nft_likes(&mut self,NFTID:NonFungibleId) -> ()
        {
        
            // let nonfungtok_id_BTreeSet =self.video_vault.non_fungible_ids(); 
            // let actual_nft_id = nonfungtok_id_BTreeSet.get(&NonFungibleId::from_u64(NFTID)).unwrap();
            let actual_nft_id = NFTID;
            info!("NFT ID of the video to Liked {:?}",actual_nft_id);
            let mut temp_nftdata:VideoNFT= borrow_resource_manager!(self.video_nft).get_non_fungible_data(&actual_nft_id);
            let updated_videoNFT = VideoNFT {
                video_title:temp_nftdata.video_title,
                content_creator:temp_nftdata.content_creator,
                video_url: temp_nftdata.video_url,
                likes:temp_nftdata.likes+1,
                views:temp_nftdata.views
            };

            borrow_resource_manager!(self.video_nft).update_non_fungible_data(&actual_nft_id,updated_videoNFT);
            // self.random_card_id_counter += 1;
        }

        pub fn update_video_nft_views(&mut self,NFTID:NonFungibleId) -> ()
        {
        
            // let nonfungtok_id_BTreeSet =self.video_vault.non_fungible_ids(); 
            // let actual_nft_id = nonfungtok_id_BTreeSet.get(&NonFungibleId::from_u64(NFTID)).unwrap();
            let actual_nft_id = NFTID;
            info!("NFT ID of the video to Viewed  {:?}",actual_nft_id);
            let mut temp_nftdata:VideoNFT= borrow_resource_manager!(self.video_nft).get_non_fungible_data(&actual_nft_id);
            let updated_videoNFT = VideoNFT {
                video_title:temp_nftdata.video_title,
                content_creator:temp_nftdata.content_creator,
                video_url: temp_nftdata.video_url,
                likes:temp_nftdata.likes,
                views:temp_nftdata.views+1
            };
            
            borrow_resource_manager!(self.video_nft).update_non_fungible_data(&actual_nft_id,updated_videoNFT);
        }
    

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
            info!("Sending {} XRD to Content Creator {}", payment_bucket.amount(), cc_username);

            // Sending the payment to the owner vault
            cc_ownership_vault.put(payment_bucket);

        }

        // METHOD: send money from content creator vaults to content creators wallet
        pub fn withdraw_from_cc_vault(&mut self , cc_name: String, withdraw_amount: Decimal ) -> Bucket
        {
            let cc_username: String = cc_name.clone();
            
            let cc_nftID = self.cc_username_cc_nftID_hashmap.get(&cc_username).unwrap();

            info!("NFT ID of the {:?}",&cc_nftID);
            info!("Name of CC of the {:?}",&cc_username);

            // Getting the vault for the Content Creator
            let cc_ownership_vault: &mut Vault = self.cc_vaults_hashmap.get_mut(cc_nftID).unwrap();

            info!("VaultID of CC of the {:?}",&cc_ownership_vault);
            info!("Previous Balance of Content Creator Vault {} : {} XRD",cc_username, cc_ownership_vault.amount());
            info!("Amount to be sent to WALLET of Content Creator {} : {}  XRD",cc_username, withdraw_amount.clone());

            // Sending the payment to the owner vault
            let withdraw_bucket: Bucket = cc_ownership_vault.take(withdraw_amount);
            info!("TRANSACTION SENT WALLET OF Content Creator {}",cc_username);
            info!("Avaliable Balance of in Vault of Content Creator {} : {}  XRD",cc_username, cc_ownership_vault.amount());

            return withdraw_bucket;

        }
        

        pub fn fetch_video_details_and_update_view(&mut self, video_link: String) -> (String, String, u64, u64, String, u64)
        {

            let _video_link: String = video_link.clone();
            
            let _video_nftID = self.video_url_videoNFTID_hashmap.get(&_video_link).unwrap();
                // info daal check kar
            let _cc_nftID = self.videonftID_ccNFTID_hashmap.get(&_video_nftID).unwrap();
                // info daal check kar


                // Shayad Yaha gadbad hori hai
            let mut temp_video_nftdata:VideoNFT= borrow_resource_manager!(self.video_nft).get_non_fungible_data(&_video_nftID);
            
            let mut temp_cc_nftdata:ccNFT= borrow_resource_manager!(self.cc_nft).get_non_fungible_data(&_cc_nftID);

            info!("Current Playing Video IPFS URL: {}  ", temp_video_nftdata.video_url );
            info!("Current Playing Video TITLE: {}  ", temp_video_nftdata.video_title);
            info!("Current Playing Video LIKES: {}  ", temp_video_nftdata.likes);
            info!("Current Playing Video VIEWS: {}  ", temp_video_nftdata.views);
            info!("Current ARTIST NAME: {}  ", temp_cc_nftdata.content_creator);
            info!("Current SUBSCRIBER COUNT: {}  ", temp_cc_nftdata.subscribers);


            // let updated_videoNFT = VideoNFT {
            //     video_title:temp_video_nftdata.video_title.clone(),
            //     content_creator:temp_video_nftdata.content_creator.clone(),
            //     video_url: temp_video_nftdata.video_url.clone(),
            //     likes:temp_video_nftdata.likes.clone(),
            //     views:temp_video_nftdata.views.clone()+1
            // };
            
            // borrow_resource_manager!(self.video_nft).update_non_fungible_data(&_video_nftID,updated_videoNFT);

            return(temp_video_nftdata.video_url.to_string(), temp_video_nftdata.video_title.to_string(), temp_video_nftdata.likes, temp_video_nftdata.views, temp_cc_nftdata.content_creator.to_string(), temp_cc_nftdata.subscribers ) ;

        }   

         
        //utility function
        pub fn get_nftid_from_vault(&mut self,vault: Vault,nftid_in_int:u64) -> NonFungibleId
        {
            let temp_nonfungtok_id_BTreeSet =vault.non_fungible_ids(); 
            let nonfungtok_id_BTreeSet =temp_nonfungtok_id_BTreeSet.clone();
            let NFTID=nftid_in_int.clone();
            let actual_nft_id = nonfungtok_id_BTreeSet.get(&NonFungibleId::from_u64(NFTID)).clone().unwrap();
            return actual_nft_id.clone()
        }

       // // METHOD: SHOWING INFORMATION IN THE TOKEN
        // pub fn show_token_info(address: ResourceAddress) {
        //     // We borrow the resource manager of the provided address
        //      let manager: &ResourceManager = borrow_resource_manager!(address);

        //     // Get the resource type
        //     match manager.resource_type() {
        //         ResourceType::Fungible{divisibility} => {
        //             info!("Fungible resource with divisibility of {}", divisibility)
        //         },
        //         ResourceType::NonFungible => {
        //             info!("Non Fungible resource")
        //         }
        //     }

        //     // Get the total supply
        //     info!("Total supply: {}", manager.total_supply());

        //     // Get information stored in the metadata
        //     let metadata: HashMap<String, String> = manager.metadata();
        //     let token_name = metadata.get("name").expect("Token does not have a name");
        //     let token_symbol = metadata.get("symbol").expect("Token does not have a symbol");
        //     info!("Name: {}. Symbol: {}", token_name, token_symbol);
        // }
    





    }

}