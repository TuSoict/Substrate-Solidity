import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";

const config: HardhatUserConfig = {
  solidity: {
    version: "0.8.17",
    // settings: {
    //   optimizer: {
    //     enabled: true,
    //     runs: 200
    //   }
    // }
  },
  networks: {
    hardhat: {
    },
    ftisu: {
      url: "https://ganache.ftisu.vn",
      accounts: ["0x3c4c05631f19a280a5eda9199797e61999df4d4d7e27fc16f76b57703d437187"]
    }
  },
};

export default config;
