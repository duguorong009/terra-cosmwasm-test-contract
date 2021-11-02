import Config from '../config';
import { LCDClient, MnemonicKey, MsgStoreCode, isTxError, MsgInstantiateContract } from '@terra-money/terra.js';
import * as fs from 'fs';

const deployContract = async (file_name: string) => {
  const typeMessage = 'DeployContract';

  //LCD Config
  const terra = new LCDClient({
    URL: Config.lcd_url,
    chainID: Config.chaindId,
    gasPrices: { uluna: Config.gasPrice },
    gasAdjustment: Config.gasAdjustment
  });
  const mk = new MnemonicKey({
    mnemonic: Config.wallet_seed,
  });
  const wallet = terra.wallet(mk);


  const storeCode = new MsgStoreCode(
    wallet.key.accAddress,
    fs.readFileSync('./wasm_contracts/' + file_name).toString('base64')
  );

  console.log("-----START-----" + typeMessage + " - " + file_name);
  try {
    const storeCodeTx = await wallet.createAndSignTx({
      msgs: [storeCode],
    });
    const storeCodeTxResult = await terra.tx.broadcast(storeCodeTx);


    console.log(storeCodeTxResult);


    if (isTxError(storeCodeTxResult)) {
      throw new Error(
        `store code failed. code: ${storeCodeTxResult.code}, codespace: ${storeCodeTxResult.codespace}, raw_log: ${storeCodeTxResult.raw_log}`
      );
    }

    const {
      store_code: { code_id },
    } = storeCodeTxResult.logs[0].eventsByType;
  } catch (e) {
    console.log(e);
  }
  console.log("------END-----" + typeMessage + " - " + file_name);
}

const initializeContract = async (code_id: number) => {
  const typeMessage = 'InstantiateContract';

  //LCD Config
  const terra = new LCDClient({
    URL: Config.lcd_url,
    chainID: Config.chaindId,
    gasPrices: { uluna: Config.gasPrice },
    gasAdjustment: Config.gasAdjustment
  });
  const mk = new MnemonicKey({
    mnemonic: Config.wallet_seed,
  });
  const wallet = terra.wallet(mk);

  const instantiateMsg = new MsgInstantiateContract(
    Config.wallet_address,
    Config.wallet_address,
    code_id,
    {}
  );

  // Sign transaction
  try {
    const tx = await wallet.createAndSignTx({
      msgs: [instantiateMsg]
    });

    //Broadcast transaction and check result
    await terra.tx.broadcast(tx).then((txResult) => {
      if (isTxError(txResult)) {
        throw new Error(
          `encountered an error while running the transaction: ${txResult.code} ${txResult.codespace}`
        );
      }

      let raw_log = JSON.parse(txResult.raw_log);
      console.log("-----START-----" + typeMessage);
      console.log("hash is: ", txResult.txhash);
      console.log("height is: ", txResult.height);
      let attributes = raw_log[0]['events'][0]['attributes'];
      for (var i = 0; i < attributes.length; i++) {
        if (attributes[i]["key"] == 'contract_address') {
          console.log("contract_address[" + i + "]: " + attributes[i]["value"]);
        }
      }
      console.log("Logs: ", txResult.logs[0].eventsByType.message);
      console.log("------END-----" + typeMessage);
    });
  } catch (e) {
    console.log(e);
  }
}

(async () => {
  // // Store contract code to the terra network.
  // await deployContract('terra_cosmwasm_test_contract.wasm');

  // Instantiate the contract.
  const code_id = 16972;
  await initializeContract(code_id);

})()
