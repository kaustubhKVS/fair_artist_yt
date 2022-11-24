use scrypto::prelude::*;

blueprint! {
    struct YtFair {
        collected_xrd: Vault,
    }

    impl YtFair 
    {
        pub fn instantiate_ytfair() -> ComponentAddress 
        {
            Self {    
                collected_xrd: Vault::new(RADIX_TOKEN)
            }
            .instantiate()
            .globalize()
        }

        pub fn deposit(&mut self, mut payment: Bucket) -> () {
            // take all the money sent in the bucket in XRD
            // if the caller has sent something other than XRD, they'll get a runtime error           
            self.collected_xrd.put(payment);
            
        }
    }
}
