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

accountAddress = window.localStorage.getItem("aa");
componentAddress = window.localStorage.getItem("ca");

console.log("checking ", accountAddress, "and", componentAddress)

document.getElementById('details').onclick = async function () {
    let video_url_1 = document.getElementById("video_url_1").value;
    let manifest = new ManifestBuilder()
    .callMethod(accountAddress, 'lock_fee', ['Decimal("100")'])
    .callMethod(componentAddress,'fetch_video_details_and_update_view',[`"${video_url_1}"`])
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
  
    // Show the receipt on the DOM
    // console.log(receipt.committed.receipt.output[1].data_json.elements)
    document.getElementById('cpviu').innerText = receipt.committed.receipt.output[1].data_json.elements[0]['value']
    document.getElementById('cpvt').innerText = receipt.committed.receipt.output[1].data_json.elements[1]['value']
    document.getElementById('cpvl').innerText = receipt.committed.receipt.output[1].data_json.elements[2]['value']
    document.getElementById('cpvv').innerText = receipt.committed.receipt.output[1].data_json.elements[3]['value']
    document.getElementById('can').innerText = receipt.committed.receipt.output[1].data_json.elements[4]['value']
    document.getElementById('csc').innerText = receipt.committed.receipt.output[1].data_json.elements[5]['value']
    //document.getElementById('receipt').innerText = JSON.stringify(receipt.committed.receipt, null, 2);
  };
  
//   document.getElementById('deposit').onclick = async function () {
  
//     let deposit_amount = document.getElementById("deposit_amt").value;
//     let manifest = new ManifestBuilder()
//       .callMethod(accountAddress, "lock_fee", ['Decimal("100")'])
//       .withdrawFromAccountByAmount(accountAddress, deposit_amount, "resource_tdx_a_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqegh4k9")
//       .takeFromWorktopByAmount(deposit_amount, "resource_tdx_a_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqegh4k9", "bucket1")
//       .callMethod(componentAddress, "deposit_cc_nft_cc_vault", [`"${content_creator}"`,'Bucket("bucket1")'])
//       .callMethod(accountAddress, "deposit_batch", ['Expression("ENTIRE_WORKTOP")'])
//       .build()
//       .toString();
  
//   console.log('instantiate manifest: ', manifest);
  
//     // Send manifest to extension for signing
//     const hash = await sdk
//       .sendTransaction(manifest)
//       .map((response) => response.transactionHash)
  
//     if (hash.isErr()) throw hash.error
  
//     // Fetch the receipt from the Gateway SDK
//     const receipt = await transactionApi.transactionReceiptPost({
//       v0CommittedTransactionRequest: { intent_hash: hash.value },
//     })
  
//     // Show the receipt on the DOM
//     console.log(receipt)
//     //document.getElementById('cpviu').innerText = JSON.stringify(receipt.committed.receipt, null, 2);
  
//   //document.getElementById('componentAddress').innerText = componentAddress;
//   };
  
//   document.getElementById('withdraw').onclick = async function () {
//     let withdraw_amt = document.getElementById("withdraw_amt").value;
//     let manifest = new ManifestBuilder()
//       .callMethod(accountAddress, "lock_fee", ['Decimal("100")'])
//       .callMethod(componentAddress, "withdraw_from_cc_vault", [`"${content_creator}"`,`Decimal("${withdraw_amt}")`])
//       .callMethod(accountAddress, "deposit_batch", ['Expression("ENTIRE_WORKTOP")'])
//       .build()
//       .toString();
  
//   console.log('instantiate manifest: ', manifest);
  
//     // Send manifest to extension for signing
//     const hash = await sdk
//       .sendTransaction(manifest)
//       .map((response) => response.transactionHash)
  
//     if (hash.isErr()) throw hash.error
  
//     // Fetch the receipt from the Gateway SDK
//     const receipt = await transactionApi.transactionReceiptPost({
//       v0CommittedTransactionRequest: { intent_hash: hash.value },
//     })
  
//     // Show the receipt on the DOM
//     console.log(receipt)
//   };