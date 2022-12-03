import { ethers } from "hardhat";

async function main() {
  const [owner, otherAccount] = await ethers.getSigners();
  const CarStore = await ethers.getContractFactory("CarStore");
  const carStore = await CarStore.deploy({ from: owner.address });

  console.log(`deployed to ${carStore.address}`);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
