const nearAPI = require("near-api-js");
const path = require('path');
// creates keyStore from a provided file
// you will need to pass the location of the .json key pair

const { keyStores,connect  } = nearAPI;
const homedir = require("os").homedir();
const CREDENTIALS_DIR = ".near-credentials";
const credentialsPath = path.join(homedir, CREDENTIALS_DIR);
const keyStore = new keyStores.UnencryptedFileSystemKeyStore(credentialsPath);

const config = {
  networkId: "testnet",
  keyStore, // optional if not signing transactions
  nodeUrl: "https://rpc.testnet.near.org",
  walletUrl: "https://wallet.testnet.near.org",
  helperUrl: "https://helper.testnet.near.org",
  explorerUrl: "https://explorer.testnet.near.org",
};

const getContract = async function getContract(){
  const near = await connect(config);
  
  const account = await near.account("EXAMPLE.testnet");
  const contract = new nearAPI.Contract(
    account, // the account object that is connecting
    "CONTRACT_ACCOUNT",
    {
      // name of contract you're connecting to
      viewMethods: ["get_time_ultima_actualizacion","get_historial"], // view methods do not change state but usually return a value
      changeMethods: ["new","nuevo_rack","nuevo_sensor","actualizar_estado"], // change methods modify state
      sender: account, // account object to initialize and sign transactions.
    }
  );
  return contract;
}

exports.getContract = getContract;