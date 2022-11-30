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
let video_url:string
let video_title:string
let video_likes
let video_views
let video_content_creator
let video_subscribers

accountAddress = window.localStorage.getItem("winAccountAddress");
componentAddress = window.localStorage.getItem("winComponentAddress");
content_creator = window.localStorage.getItem("winContentCreator");

console.log("checking ", accountAddress, "and", componentAddress)

document.getElementById('play_random').onclick = async function () {
  
  let manifest = new ManifestBuilder()
  .callMethod(accountAddress, 'lock_fee', ['Decimal("100")'])
  .callMethod(componentAddress,'playvideo_for_video_feed',[])
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
  // var iframe = document.createElement('iframe');
  // iframe.src = receipt.committed.receipt.output[1].data_json.elements[0].value;
  // document.getElementById("play").appendChild(iframe);
  console.log(receipt)
  video_url = receipt.committed.receipt.output[1].data_json.elements[0].value
  video_title = receipt.committed.receipt.output[1].data_json.elements[1].value
  video_likes = receipt.committed.receipt.output[1].data_json.elements[2].value
  video_views =receipt.committed.receipt.output[1].data_json.elements[3].value
  video_content_creator = receipt.committed.receipt.output[1].data_json.elements[4].value
  video_subscribers = receipt.committed.receipt.output[1].data_json.elements[5].value
  document.getElementById("showskill").src =video_url
  document.getElementById('cpviu').innerText = video_url
    document.getElementById('cpvt').innerText = video_title
    document.getElementById('cpvl').innerText = video_likes
    document.getElementById('cpvv').innerText = video_views
    document.getElementById('can').innerText = video_content_creator
    document.getElementById('csc').innerText = video_subscribers

  //document.getElementById('play_url').innerText = receipt.committed.receipt.output[1].data_json.elements[0].value
  //document.getElementById('myIframe').src = receipt.committed.receipt.output[1].data_json.elements[0].value+"&output=embed";
  //console.log(receipt.committed.receipt.output[1].data_json.elements[0].value+"&output=embed")
  
//   if (play_url===null) {
//     document.write("<p>You need to add a ?v={filename} to the url")
// } else {
//     document.write("<p>video controls</p>")
//     document.write("<source src="+play_url+" >")
//     document.write("Sorry, your browser doesn't support embedded videos.")
//     document.write("</video>")
// }
};


// document.getElementById('details').onclick = async function () {
//     let manifest = new ManifestBuilder()
//     .callMethod(accountAddress, 'lock_fee', ['Decimal("100")'])
//     .callMethod(componentAddress,'fetch_video_details_and_update_view',[`"${video_url}"`])
//     .callMethod(accountAddress, "deposit_batch", ['Expression("ENTIRE_WORKTOP")'])
//     .build()
//     .toString();
  
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
//     console.log(receipt.committed.receipt.output[1].data_json.elements)
//     document.getElementById('cpviu').innerText = receipt.committed.receipt.output[1].data_json.elements[0]['value']
//     document.getElementById('cpvt').innerText = receipt.committed.receipt.output[1].data_json.elements[1]['value']
//     document.getElementById('cpvl').innerText = receipt.committed.receipt.output[1].data_json.elements[2]['value']
//     document.getElementById('cpvv').innerText = receipt.committed.receipt.output[1].data_json.elements[3]['value']
//     document.getElementById('can').innerText = receipt.committed.receipt.output[1].data_json.elements[4]['value']
//     document.getElementById('csc').innerText = receipt.committed.receipt.output[1].data_json.elements[5]['value']
//     //document.getElementById('receipt').innerText = JSON.stringify(receipt.committed.receipt, null, 2);
//   };
  
  document.getElementById('deposit').onclick = async function () {

    let deposit_amount = document.getElementById("deposit_amt").value;
    let manifest = new ManifestBuilder()
      .callMethod(accountAddress, "lock_fee", ['Decimal("100")'])
      .withdrawFromAccountByAmount(accountAddress, deposit_amount, "resource_tdx_a_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqegh4k9")
      .takeFromWorktopByAmount(deposit_amount, "resource_tdx_a_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqegh4k9", "bucket1")
      .callMethod(componentAddress, "deposit_cc_nft_cc_vault", [`"${video_content_creator}"`,'Bucket("bucket1")'])
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
    console.log(receipt)
    //document.getElementById('cpviu').innerText = JSON.stringify(receipt.committed.receipt, null, 2);
  
  //document.getElementById('componentAddress').innerText = componentAddress;
   document.getElementById('prev_bal_d').innerText = receipt.committed.receipt.output[3].data_json.elements[0]['value'].replace(/\D/g, '');
    document.getElementById('deposit_amt_d').innerText = receipt.committed.receipt.output[3].data_json.elements[1]['value'].replace(/\D/g, '');
    document.getElementById('curr_bal_d').innerText = receipt.committed.receipt.output[3].data_json.elements[2]['value'].replace(/\D/g, '');
  };
  
  document.getElementById('withdraw').onclick = async function () {
    let withdraw_amt = document.getElementById("withdraw_amt").value;
    let manifest = new ManifestBuilder()
      .callMethod(accountAddress, "lock_fee", ['Decimal("100")'])
      .callMethod(componentAddress, "withdraw_from_cc_vault", [`"${video_content_creator}"`,`Decimal("${withdraw_amt}")`])
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
    console.log(receipt)
    document.getElementById('prev_bal').innerText = receipt.committed.receipt.output[1].data_json.elements[1]['value'].replace(/\D/g, '');
    document.getElementById('withdrew_amt').innerText = receipt.committed.receipt.output[1].data_json.elements[2]['value'].replace(/\D/g, '');
    document.getElementById('curr_bal').innerText = receipt.committed.receipt.output[1].data_json.elements[3]['value'].replace(/\D/g, '');
  };

  document.getElementById('green').onclick = async function name() {
    let manifest = new ManifestBuilder()
    .callMethod(accountAddress, 'lock_fee', ['Decimal("100")'])
    .callMethod(componentAddress,'update_video_nft_likes_byurl',[`"${video_url}"`])
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
    
  };

  document.getElementById('white').onclick = async function name() {
    let manifest = new ManifestBuilder()
    .callMethod(accountAddress, 'lock_fee', ['Decimal("100")'])
    .callMethod(componentAddress,'update_cc_nft_subscribers_byccusername',[`"${video_content_creator}"`])
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
    
  };