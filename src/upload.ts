import Sdk, { ManifestBuilder } from '@radixdlt/alphanet-walletextension-sdk';
import { StateApi, TransactionApi } from '@radixdlt/alphanet-gateway-api-v0-sdk'

// Initialize the SDK
const sdk = Sdk()
const transactionApi = new TransactionApi()
const stateApi = new StateApi()

// Global states
let accountAddress: string // User account address
let componentAddress: string  // GumballMachine component address
let resourceAddress: string // GUM resource address
let content_creator: string //content creator string

accountAddress = window.localStorage.getItem("winAccountAddress");
componentAddress = window.localStorage.getItem("winComponentAddress");
content_creator = window.localStorage.getItem("winContentCreator");

console.log("checking ", accountAddress, "and", componentAddress)

document.getElementById('upload').onclick = async function () {
    let video_title = document.getElementById("video_title").value;
    let video_url = document.getElementById("video_url").value;
    let manifest = new ManifestBuilder()
    .callMethod(accountAddress, 'lock_fee', ['Decimal("100")'])
    .callMethod(componentAddress,'make_video_nft',[`"${video_title}"`,`"${content_creator}"`,`"${video_url}"`])
    .callMethod(accountAddress, "deposit_batch", ['Expression("ENTIRE_WORKTOP")'])
    .build()
    .toString();
  
  console.log('instantiate manifest: ', manifest);
  
    // Send manifest to extension for signing
    const hash = await sdk
      .sendTransaction(manifest)
      .map((response) => response.transactionHash)
  
    if (hash.isErr()) throw hash.error
  
    // Fetch the receipt from the Gateway SDK
    const receipt = await transactionApi.transactionReceiptPost({
      v0CommittedTransactionRequest: { intent_hash: hash.value },
    })
  
    console.log(receipt)

    window.localStorage.setItem("video_url", video_url);
    
    console.log("var:", window.localStorage.getItem("video_url"))
    // resourceAddress = receipt.committed.receipt.state_updates.new_global_entities[0].global_address
    // document.getElementById('gumAddress').innerText = resourceAddress;
  }

