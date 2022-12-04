import { ethers, hardhatArguments } from 'hardhat';
import * as Config from "./config";

async function main() {
    await Config.initConfig();
    const network = hardhatArguments.network ? hardhatArguments.network : "dev";
    const [deployer] = await ethers.getSigners();
    console.log("deploy from address: ", deployer.address);

    const ZombieHelper = await ethers.getContractFactory("ZombieAttack");
    const zombieHelper = await ZombieHelper.deploy();
    await zombieHelper.deployed();
    console.log("Zombie address: ", zombieHelper.address);
    Config.setConfig(network + '.address', zombieHelper.address);
    await Config.updateConfig();
}

main()
    .then(() => process.exit(0))
    .catch((err) => {
        console.error(err);
        process.exit(1);
    });
