const hre = require("hardhat");

async function main() {
  // Get contract that we want to deploy
  const contractFactory = await hre.ethers.getContractFactory("Auction");

  // Deploy contract with the correct constructor arguments
  const contract = await contractFactory.deploy();

  // Wait for this transaction to be mined
  await contract.deployed();

  // Get contract address
  console.log("Auction contract deployed to:", contract.address);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });