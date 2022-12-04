import "typechain";
import "@nomiclabs/hardhat-waffle";
import "@nomiclabs/hardhat-etherscan";
import "@nomiclabs/hardhat-ethers";
import * as dotenv from "dotenv";
dotenv.config({ path: __dirname + "/.env" });

// You need to export an object to set up your config
// Go to https://hardhat.org/config/ to learn more

/**
 * @type import('hardhat/config').HardhatUserConfig
 */
module.exports = {
  solidity: {
    compilers: [
      {
        version: "0.8.16"
      }
    ]
  },
  // defaultNetwork: "dev",
  networks: {
    dev: {
      url: "http://localhost:7545",
      gasPrice: 20,
      accounts: {
        mnemonic: process.env.MNEMONIC,
        count: 10
      },
      saveDeployments: true
    },
    bsctest: {
      url: "https://data-seed-prebsc-1-s1.binance.org:8545",
      chainId: 97,
      accounts: [process.env.PRIV_KEY],
      gasPrice: 20000000000,
      //blockGasLimit: 1000000
    },
    rinkeby: {
      url: "https://eth-rinkeby.alchemyapi.io/v2/Lur41XBqJaVgjDGfXmSvYwzucY1r7ie5",
      accounts: [process.env.PRIV_KEY]
    },
    solanartest: {
      url: "https://api.testnet.solana.com",
      accounts: [process.env.PRIV_KEY],
      gasPrice: 10000000000,
      blockGasLimit: 10000000
    },
    polygontest: {
      url: "https://rpc-mumbai.maticvigil.com",
      accounts: [process.env.PRIV_KEY],
      gasPrice: 10000000000,
      blockGasLimit: 10000000
    },
    main: {
      url: "https://bsc-dataseed1.binance.org",
      accounts: [process.env.PRIV_KEY],
      gasPrice: 5100000000,
      blockGasLimit: 1000000
    }
  },

  etherscan: {
    apiKey: process.env.API_KEY
  }
};