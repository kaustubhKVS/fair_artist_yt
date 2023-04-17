# RadTok on Alphanet: Equity in Content Monetization

Radix implementation of <b>TikTok + Payment System</b> using the [Alphanet Javascript SDK](https://docs.radixdlt.com/main/scrypto/alphanet/javascript-sdk.html).

</div>

Problems we have aimed to address:

- SHADOW BLOCKING & TRANSPARENCY
- LATE PAYMENTS TO CONTENT CREATORS
- UNFAIR DISTRIBUTIONS OF MONETISED ASSETS
- UNFAIR IN-APP PURCHASE PRACTICES

## Prefer to watch a video demo?

A video where we demonstrate Alphanet using RadTok and show the functioning of the platform.
[![YouTube](https://upload.wikimedia.org/wikipedia/commons/thumb/b/b8/YouTube_play_button_icon_%282013%E2%80%932017%29.svg/512px-YouTube_play_button_icon_%282013%E2%80%932017%29.svg.png?20190606141903)](https://www.youtube.com/watch?v=nI4sBaPfzmI)

## System Overview:

- IPFS is used to store media items e.g videos
- RadixDLT is used to store assets.
- The payment system is deeply integrated with Wallet and RadixDLT and currently supports XRD transactions.

![System_Design](/public/System_Overview.png)

## Pre-requisites

1. Node >= 12.17.0
2. The Alphanet wallet is installed. Instructions [here](https://docs.radixdlt.com/main/scrypto/alphanet/wallet-extension.html)
3. Scrypto v0.6.0. Instructions to install [here](https://docs.radixdlt.com/main/scrypto/getting-started/install-scrypto.html) and update [here](https://docs.radixdlt.com/main/scrypto/getting-started/updating-scrypto.html)

## Building the Scrypto code

1. Enter the scrypto directory in a terminal: `cd scrypto`
1. Build the code: `scrypto build`
1. Two important files (`gumball_machine.abi` and `gumball_machine.wasm`) will be generated in `scrypto/target/wasm32-unknown-unknown/release/`. You will need them for the next step.

## Deploy the package to Alphanet

1. Go to the [package deployer website](https://alphanet-deployer.radixdlt.com/)
2. Upload both `tiktokfair.abi` and `tiktokfair.wasm`
3. Click on "publish package"
4. The wallet should open up and ask you to submit the transaction
5. On the wallet click on "submit"
6. The deployed package address should get displayed. **You will need it for the next step**.

## Interacting with our package

1. Open `src/index.ts` in the editor of your choice and set the value of the variable `packageAddress` to the previously obtained package address.
2. In a terminal go to the root of this project (Radix-Debitable)
3. Install the npm dependencies: `npm install`
4. Start the local server with `npm start`
5. Open up your browser at the provided URL if it doesn't open automatically.
6. Make sure that you have created an account on the wallet extension.
7. Use the Wallet Extension and Identification Address to interact with the system.
