require('dotenv').config();
import { hardhatArguments, config as HardhatConfig } from 'hardhat';
import { HttpNetworkConfig } from 'hardhat/types';
import Web3 from 'web3';
import { AbiItem } from "web3-utils";
import {promises as fs} from 'fs';
import * as Config from './config';

let myAddress = process.env.ACCOUNT_ADDR || '0x0000000000000000000000000000000000000000';
let myPrivateKey = process.env.PRIV_KEY || "";
let receiverAddress = process.env.RECEIVER_ADDR || '0x0000000000000000000000000000000000000000';

async function getAbi() : Promise<AbiItem[]>  {
    let config = JSON.parse((await fs.readFile('./artifacts/contracts/Token.sol/Token.json')).toString());
    return config.abi as AbiItem[];
}

async function interact(){
    const network = hardhatArguments.network ? hardhatArguments.network : 'dev';
    await Config.initConfig();

    let config = Config.getConfig();
    let tokenAddress : string = config[network].address;
    if (!tokenAddress) {
        throw new Error(`No token address found for network ${network}`);
    }
    let networkConfig = HardhatConfig.networks[network] as HttpNetworkConfig;
    if (!networkConfig) {
        throw new Error(`No network config found for network ${network}`);
    }

    let tokenAbi = await getAbi();

    let web3 = new Web3(networkConfig.url);
    let tokenContract = new web3.eth.Contract(tokenAbi, tokenAddress);

    // Test call function
    let myBalance = await tokenContract.methods.balanceOf(myAddress).call();
    console.log(myBalance);

    web3.eth.accounts.wallet.add(myPrivateKey);
    let receiverBalanceBefore = await tokenContract.methods.balanceOf(receiverAddress).call();

    // Test send function
    let rs = await tokenContract.methods.transfer(receiverAddress, 100000).send({
        from: myAddress,
        gas: 3000000
    });
    let receiverBalanceAfter = await tokenContract.methods.balanceOf(receiverAddress).call();
    console.log(rs, receiverBalanceBefore, receiverBalanceAfter);
}

interact();
