// scripts/upgrade_box.js
const { ethers, upgrades } = require('hardhat');

async function main () {
  const BoxV2 = await ethers.getContractFactory('BoxV2');
  console.log('Upgrading Box...');
  await upgrades.upgradeProxy('0x5992afbBadB9754EF38C4c6D0F96Ec1D9d11D1eC', BoxV2);
  console.log('Box upgraded');
}

main();