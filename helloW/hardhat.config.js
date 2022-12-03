require("@nomicfoundation/hardhat-toolbox");
require('@nomiclabs/hardhat-ethers');
require('@openzeppelin/hardhat-upgrades');

/** @type import('hardhat/config').HardhatUserConfig */

module.exports = {
  solidity: "0.8.17",
  paths: {
    
  },
  networks: {
    goerli: {
      url: "https://eth-goerli.g.alchemy.com/v2/h22lDcMRadEsaYCqlaOF6qyriCBxDEHN",
      accounts: ["5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133"],
    }
  },
  etherscan: {
    // Your API key for Etherscan
    // Obtain one at https://etherscan.io/
    apiKey: "KZI6SYX2NGGFRBGP75WXY9QBBTQTT7PY78"
  }
};
