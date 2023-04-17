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

console.log("checking ", accountAddress, "and", componentAddress)

document.getElementById('CC_NFT').onclick = async function () {
  content_creator = document.getElementById("content_creator_name").value;
  console.log(typeof(content_creator));
  //  let arr = new String; 
  //  arr = ["Seema","blah",'https://www.youtube.com/watch?v=qfRCQ2YsLMM']
  
  let manifest = new ManifestBuilder()
  .callMethod(accountAddress, 'lock_fee', ['Decimal("100")'])
  .callMethod(componentAddress, 'make_cc_nft_cc_vault',[`"${content_creator}"`])
  .callMethod(accountAddress, "deposit_batch", ['Expression("ENTIRE_WORKTOP")'])
  .build()
  .toString();

console.log(content_creator);

  // Send manifest to extension for signing
  const hash = await sdk
    .sendTransaction(manifest)
    .map((response) => response.transactionHash)

  if (hash.isErr()) throw hash.error

  // Fetch the receipt from the Gateway SDK
  const receipt = await transactionApi.transactionReceiptPost({
    v0CommittedTransactionRequest: { intent_hash: hash.value },
  })

  //componentAddress = receipt.committed.receipt.state_updates.new_global_entities[5].global_address
  //document.getElementById('componentAddress').innerText = componentAddress;
  console.log(receipt)
  
  window.localStorage.setItem("winContentCreator", content_creator);

  // resourceAddress = receipt.committed.receipt.state_updates.new_global_entities[0].global_address
  // document.getElementById('gumAddress').innerText = resourceAddress;
}

