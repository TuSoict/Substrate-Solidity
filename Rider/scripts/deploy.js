const hre = require("hardhat");
const fs = require("fs");

async function main() {
  const RIDER = await hre.ethers.getContractFactory("RIDER");
  const RiderNFT = await hre.ethers.getContractFactory("RiderNFT");
  const NFTSale = await hre.ethers.getContractFactory("NFTSale");

  const _RIDER = await RIDER.deploy(
    "1000000000000000000000000000000000000000000000000000000000000000000"
  );
  await RIDER.deployed();

  const _RiderNFT = await RiderNFT.deploy("Rider", "RIDER");
  await RiderNFT.deployed();

  // address _nftAddress,
  // address _tokenAddress,
  // uint256[] memory _prices,
  // string[] memory placeholderURIs,
  // uint256 _purchaseLimit

  const nftSale = await NFTSale.deploy(
    RiderNFT.address,
    RIDER.address,
    ["1", "2", "3", "4"],
    [
      "/0.json",

      "/1.json",

      "/2.json",

      "/3.json",

      "/4.json",

      "/5.json",

      "/6.json",
    ],
    3
  );
  await nftSale.deployed();

  console.log("RiderNFT deployed to:", RiderNFT.address);

  fs.writeFileSync(
    "./config.js",
    `
  export const RIDER = "${RIDER.address}"
  export const RiderNFT = "${RiderNFT.address}"
  export const nftSale = "${nftSale.address}"
  `
  );
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
