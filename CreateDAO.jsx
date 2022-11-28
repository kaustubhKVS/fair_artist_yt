import { useState, useEffect } from 'react';
// Import Radix Wallet and Gateway SDKs
import Sdk, { ManifestBuilder } from '@radixdlt/alphanet-walletextension-sdk';
import {
  StateApi,
  TransactionApi,
  // StatusApi,
} from '@radixdlt/alphanet-gateway-api-v0-sdk';

const CreateDAO = () => {
  const [account, setAccount] = useState(
    'account_sim1qdjdyj3x6nxs54d6age8w3vrjm735xsvfrgcqwanydgsjwjfxa'
  );
  const [component, setComponent] = useState(
    'component_tdx_a_1qtjlgxz4c5arzdkqc4pj6audahflna9w2zgdw29kk76ql9xzu7'
  );
  // const [founders_badge, setFounders_badge] = useState('');

  // form fields
  const [name, setName] = useState('');
  //const [price, setPrice] = useState('');
  const [total_shares, setTotal_Shares] = useState('');

  // Initialize the SDK
  const sdk = Sdk();
  const transactionApi = new TransactionApi();
  const stateApi = new StateApi();
  // const statusApi = new StatusApi();

  useEffect(() => {
    const getAddress = async () => {
      const result = await sdk.request({
        accountAddresses: {},
      });
      console.log('accountAddresses: ', result.value);
      const { accountAddresses } = result.value;
      setAccount(accountAddresses[0].address);
    };
    getAddress();
    return () => {};
  }, [sdk]);

  // Send manifest to extension for signing
  const createMemberTokens = async (e) => {
    console.log("entering createmembertoken")
    e.preventDefault();
    // create Transaction Manifest to instantiate Component
    let packageAddress =
    'package_tdx_a_1q9e8m3fvleqpeh6h2phmexpve9k6wpwfz5guzmem5j3sgmdm2u';

    let manifest = new ManifestBuilder()
      .callMethod(account, 'lock_fee', ['Decimal("100")'])
      .callFunction(packageAddress, 'YtFair', 'instantiate_ytfair', [])
      .build()
      .toString();

    console.log('instantiate manifest: ', manifest);

    const hash = await sdk
      .sendTransaction(manifest)
      .map((response) => response.transactionHash);

    if (hash.isErr()) throw hash.error;
    console.log('hash_check: ', hash);
    // Fetch the receipt from the Gateway SDK
    const receipt = await transactionApi.transactionReceiptPost({
      v0CommittedTransactionRequest: { intent_hash: hash.value },
    });
    console.log('receipt: ', receipt);
    let componentAddress = // 'x'
      receipt.committed.receipt.state_updates.new_global_entities[1]
        .global_address;
    console.log('componentAddress: ', componentAddress);
    setComponent(componentAddress);

    console.log("checking the new component address", componentAddress)
     let manifest_deposit = new ManifestBuilder()
      //.callFunction(componentAddress, 'YtFair', 'deposit', ['Bucket("Bucket1")'])
      .callMethod(account, "lock_fee", ['Decimal("100")'])
      .withdrawFromAccountByAmount(account, total_shares, "resource_tdx_a_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqegh4k9")
      .takeFromWorktopByAmount(total_shares, "resource_tdx_a_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqegh4k9", "bucket1")
      .callMethod(componentAddress, "deposit", ['Bucket("bucket1")'])
      .build()
      .toString();

    console.log('deposit: ', manifest_deposit);
   
    const hash1 = await sdk
      .sendTransaction(manifest_deposit)
      .map((response) => response.transactionHash);

    if (hash1.isErr()) throw hash1.error;
    console.log('hash1: ', hash1);
    // Fetch the receipt from the Gateway SDK
    const receipt1 = await transactionApi.transactionReceiptPost({
      v0CommittedTransactionRequest: { intent_hash: hash1.value },
    });
    console.log('Deposit Receipt: ', receipt1);

    const account_state = await stateApi.stateComponentPost({
      v0StateComponentRequest: { component_address: account }
    })
  
    let account_vault = account_state.owned_vaults.find(vault => vault.resource_amount.resource_address === 'resource_tdx_a_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqegh4k9');
    console.log(account_vault.resource_amount.amount_attos / Math.pow(10,18));
  
    const machine_state = await stateApi.stateComponentPost({
      v0StateComponentRequest: { component_address: componentAddress }
    })
  
    let machine_vault = machine_state.owned_vaults.find(vault => vault.resource_amount.resource_address === 'resource_tdx_a_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqegh4k9');
    console.log(machine_vault.resource_amount.amount_attos / Math.pow(10,18));
      
  };

  return (
    <div className="mt-4 p-4">
      <h2 className="text-3xl font-bold mb-2">Send RDX to Creators!</h2>
      {/* <form onSubmit={createMemberTokens}>
        <button
          className="mt-2 mr-4 bg-green-700 hover:bg-green-500"
          type="submit"
        >
          Send RDX
        </button>
      </form> */}
      <form onSubmit={createMemberTokens}>
        <label>
          <span>Video URL:</span>
          <input
            required
            type="text"
            onChange={(e) => setName(e.target.value)}
            value={name}
          />
        </label>
        {/* <label>
          <span>Token Price:</span>
          <input
            required
            type="text"
            onChange={(e) => setPrice(e.target.value)}
            value={price}
          />
        </label>  */}
        <label>
          <span>Price:</span>
          <input
            required
            type="text"
            onChange={(e) => setTotal_Shares(e.target.value)}
            value={total_shares}
          />
        </label>
        <button
          className="mt-2 mr-4 bg-green-700 hover:bg-green-500"
          type="submit"
        >
          Send RDX
        </button>
      </form>
      <p className="p-2 border-2 m-4">
        <strong>Connected Account: </strong> {account}
      </p>
      <p className="p-2 border-2 m-4">
        <strong>Members Component Address: </strong> {component}
      </p>
    </div>
  );
};

export default CreateDAO;
