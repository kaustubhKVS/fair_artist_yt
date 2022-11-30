import Sdk, { ManifestBuilder } from '@radixdlt/alphanet-walletextension-sdk';
import { StateApi, TransactionApi } from '@radixdlt/alphanet-gateway-api-v0-sdk'

// Initialize the SDK
const sdk = Sdk()
const transactionApi = new TransactionApi()
const stateApi = new StateApi()

// Global states
let accountAddress: string // User account address
let componentAddress= "component_tdx_a_1q2jtnal7czxx4v6tec5k2028nmwly7eccdcrv5vg5nhqta2tzc"  // GumballMachine component address
let resourceAddress: string // GUM resource address
let content_creator: string //content creator string

document.getElementById('fetchAccountAddress').onclick = async function () {
  // Retrieve extension user account address
  const result = await sdk.request({
    accountAddresses: {},
  })

  if (result.isErr()) {
    throw result.error
  }

  const { accountAddresses } = result.value
  if (!accountAddresses) return

  document.getElementById('accountAddress').innerText = accountAddresses[0].address
  accountAddress = accountAddresses[0].address
}

// document.getElementById('instantiateComponent').onclick = async function () {
//   let packageAddress = "package_tdx_a_1qxj3q5gajfyt38puk6a6v6ftw7ngqqmvt04g27zdsszqv45lf8"

//   let manifest = new ManifestBuilder()
//   .callMethod(accountAddress, 'lock_fee', ['Decimal("100")'])
//   .callFunction(packageAddress, 'YtFair', 'instantiate_ytfair', [])
//   .callMethod(accountAddress, "deposit_batch", ['Expression("ENTIRE_WORKTOP")'])
//   .build()
//   .toString();

// console.log('instantiate manifest: ', manifest);

  // Send manifest to extension for signing
  // const hash = await sdk
  //   .sendTransaction(manifest)
  //   .map((response) => response.transactionHash)

  // if (hash.isErr()) throw hash.error

  // // Fetch the receipt from the Gateway SDK
  // const receipt = await transactionApi.transactionReceiptPost({
  //   v0CommittedTransactionRequest: { intent_hash: hash.value },
  // })

  // componentAddress = receipt.committed.receipt.state_updates.new_global_entities[5].global_address
  // document.getElementById('componentAddress').innerText = componentAddress;
  // console.log(receipt)
  
  // resourceAddress = receipt.committed.receipt.state_updates.new_global_entities[0].global_address
  // document.getElementById('gumAddress').innerText = resourceAddress;
//}

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
  
  // resourceAddress = receipt.committed.receipt.state_updates.new_global_entities[0].global_address
  // document.getElementById('gumAddress').innerText = resourceAddress;
}
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
  
  // resourceAddress = receipt.committed.receipt.state_updates.new_global_entities[0].global_address
  // document.getElementById('gumAddress').innerText = resourceAddress;
}
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
  console.log(receipt.committed.receipt.output[1].data_json.elements)
  document.getElementById('cpviu').innerText = receipt.committed.receipt.output[1].data_json.elements[0]['value']
  document.getElementById('cpvt').innerText = receipt.committed.receipt.output[1].data_json.elements[1]['value']
  document.getElementById('cpvl').innerText = receipt.committed.receipt.output[1].data_json.elements[2]['value']
  document.getElementById('cpvv').innerText = receipt.committed.receipt.output[1].data_json.elements[3]['value']
  document.getElementById('can').innerText = receipt.committed.receipt.output[1].data_json.elements[4]['value']
  document.getElementById('csc').innerText = receipt.committed.receipt.output[1].data_json.elements[5]['value']
  //document.getElementById('receipt').innerText = JSON.stringify(receipt.committed.receipt, null, 2);
};

document.getElementById('deposit').onclick = async function () {

  let deposit_amount = document.getElementById("deposit_amt").value;
  let manifest = new ManifestBuilder()
    .callMethod(accountAddress, "lock_fee", ['Decimal("100")'])
    .withdrawFromAccountByAmount(accountAddress, deposit_amount, "resource_tdx_a_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqegh4k9")
    .takeFromWorktopByAmount(deposit_amount, "resource_tdx_a_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqegh4k9", "bucket1")
    .callMethod(componentAddress, "deposit_cc_nft_cc_vault", [`"${content_creator}"`,'Bucket("bucket1")'])
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
    .callMethod(componentAddress, "withdraw_from_cc_vault", [`"${content_creator}"`,`Decimal("${withdraw_amt}")`])
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

// document.getElementById('userBalance').onclick = async function () {
//   // Fetch the state of the account component
//   const account_state = await stateApi.stateComponentPost({
//     v0StateComponentRequest: { component_address: accountAddress }
//   })

//   let account_gum_vault = account_state.owned_vaults.find(vault => vault.resource_amount.resource_address == "resource_tdx_a_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqegh4k9")
//   console.log(accountAddress)
  //document.getElementById('wallet').innerText = account_gum_vault.resource_amount.amount_attos / Math.pow(10,18);

  // // Fetch the state of the machine component
  // const machine_state = await stateApi.stateComponentPost({
  //   v0StateComponentRequest: { component_address: componentAddress }
  // })

  // let machine_gum_vault = machine_state.owned_vaults.find(vault => vault.resource_amount.resource_address == `${resourceAddress}`)

  // // Update the DOM
  // document.getElementById("userBalance").innerText = account_gum_vault.resource_amount.amount_attos / Math.pow(10,18)
  // document.getElementById("machineBalance").innerText = machine_gum_vault.resource_amount.amount_attos / Math.pow(10,18)
//};