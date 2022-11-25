import Sdk, { ManifestBuilder } from '@radixdlt/alphanet-walletextension-sdk';    //wallet sdk
import { StateApi, TransactionApi } from '@radixdlt/alphanet-gateway-api-v0-sdk';  //gateway api : query and find out info about transactions
// Initialize the SDK
const sdk = Sdk()
const transactionApi = new TransactionApi()
const stateApi = new StateApi()

// Global states
let accountAddress = "account_tdx_a_1q0ej5cth9jknukpsfrnkh4wje263f9e6wdmgq2kalz3sx76zn8" //string // User account address
let packageAddress = "package_tdx_a_1q8nw8wxe37te8fsdeshv8chm34kvjsdvktnw8ssf7wqq64fhql"
let componentAddress = ""  // GumballMachine component address
let resourceAddress = "" // GUM resource address


// document.getElementById('fetchAccountAddress').onclick = async function () { 
//   window.location.href = "gumball-machine-example/src/hello_world.html"
// }
// document.getElementById('fetchAccountAddress').onclick = async function () {      //fetching wallet address
  // Retrieve extension user account address
async function fetch(){
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


//CALL_METHOD ComponentAddress("account_tdx_a_1q0ej5cth9jknukpsfrnkh4wje263f9e6wdmgq2kalz3sx76zn8") "lock_fee" Decimal("100");
//CALL_FUNCTION PackageAddress("package_tdx_a_1q8nw8wxe37te8fsdeshv8chm34kvjsdvktnw8ssf7wqq64fhql") "GumballMachine" "instantiate_gumball_machine" Decimal("10");
//document.getElementById('instantiateComponent').onclick = async function () {
async function instantiateComponent(){
  let packageAddress = "package_tdx_a_1q8nw8wxe37te8fsdeshv8chm34kvjsdvktnw8ssf7wqq64fhql"
  //document.write(packageAddress)
  let accountAddress = "account_tdx_a_1q0ej5cth9jknukpsfrnkh4wje263f9e6wdmgq2kalz3sx76zn8"
  
    
  let manifest = new ManifestBuilder()
    .callMethod(accountAddress, "lock_fee", ['Decimal("100")'])
    .callFunction(packageAddress, "GumballMachine", "instantiate_gumball_machine", ['Decimal("10")'])
    .build()
    .toString();
  
    console.log(accountAddress);
    console.log(packageAddress);
  // Send manifest to extension for signing
  const hash = await sdk
    .sendTransaction(manifest)
    .map((response) => response.transactionHash)

  if (hash.isErr()) throw hash.error
  
  console.log(hash.error);


  // Fetch the receipt from the Gateway SDK
  const receipt = await transactionApi.transactionReceiptPost({
    v0CommittedTransactionRequest: { intent_hash: hash.value },
  })

  componentAddress = receipt.committed.receipt.state_updates.new_global_entities[1].global_address
  //document.getElementById('componentAddress').innerText = componentAddress

  resourceAddress = receipt.committed.receipt.state_updates.new_global_entities[0].global_address
  //document.getElementById('gumAddress').innerText = resourceAddress;
  return [componentAddress,resourceAddress];

}
fetch();
const result = instantiateComponent();
result.then((data) => console.log(data[0]));
result.then((data) => console.log(data[1]));
//Component Address: component_tdx_a_1qfk9r72av2cy6n55qc3akf4jjxn25pp2pzv6rshrhzts03g7q0
//GUM token address: resource_tdx_a_1qqzjxu9qchd9jcurljsd2gppcfjakdy4rzj0745ekllsdcdpum

document.getElementById('buyGumball').onclick = async function () {
//async function buyGumball() {
  let manifest = new ManifestBuilder()
    .callMethod(accountAddress, "lock_fee", ['Decimal("100")'])
    .withdrawFromAccountByAmount(accountAddress, 10, "resource_tdx_a_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqegh4k9")
    .takeFromWorktopByAmount(10, "resource_tdx_a_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqegh4k9", "bucket1")
    .callMethod(componentAddress, "buy_gumball", ['Bucket("bucket1")'])
    .callMethod(accountAddress, "deposit_batch", ['Expression("ENTIRE_WORKTOP")'])
    .build()
    .toString();
   // console.log(componentAddress); - works
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
  document.getElementById('receipt').innerText = JSON.stringify(receipt.committed.receipt, null, 2);
};

document.getElementById('checkBalance').onclick = async function () {

  // Fetch the state of the account component
  const account_state = await stateApi.stateComponentPost({
    v0StateComponentRequest: { component_address: accountAddress }
  })

  let account_gum_vault = account_state.owned_vaults.find(vault => vault.resource_amount.resource_address == `${resourceAddress}`)

  // Fetch the state of the machine component
  const machine_state = await stateApi.stateComponentPost({
    v0StateComponentRequest: { component_address: componentAddress }
  })

  let machine_gum_vault = machine_state.owned_vaults.find(vault => vault.resource_amount.resource_address == `${resourceAddress}`)

  // Update the DOM
  document.getElementById("userBalance").innerText = account_gum_vault.resource_amount.amount_attos / Math.pow(10,18)
  document.getElementById("machineBalance").innerText = machine_gum_vault.resource_amount.amount_attos / Math.pow(10,18)
};