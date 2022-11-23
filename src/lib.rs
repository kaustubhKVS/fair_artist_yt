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

        pub fn deposit(&mut self, mut payment: Bucket) -> Bucket {
            // take our price in XRD out of the payment
            // if the caller has sent too few, or sent something other than XRD, they'll get a runtime error
            let our_share = payment.take(10);
            self.collected_xrd.put(our_share);
            // return a tuple containing a gumball, plus whatever change is left on the input payment (if any)
            // if we're out of gumballs to give, we'll see a runtime error when we try to grab one
            payment
        }
    }
}
