const { expect, assert } = require("chai");
const { ethers } = require("hardhat");
const {
  CONTRACT_NAME,
  CONTRACT_SYMBOL,
  MAX_PURCHASED,
  PIRCES,
  PLACEHODLER_URIS,
  TOTAL_TOKEN_SUPPLY,
} = require("../constants");

describe("Contract Test...", function () {
  let owner = null;
  let addr1 = null;
  let addr2 = null;

  let RIDER = null;
  let RiderNFT = null;
  let riderSaleNFT = null;
  let tokenContract = null;
  let nftContract = null;
  let saleNFTContract = null;

  before(async function () {
    [owner, addr1, addr2] = await ethers.getSigners();

    const RIDER = await ethers.getContractFactory("RIDER");
    const RiderNFT = await ethers.getContractFactory("RiderNFT");
    const NFTSale = await ethers.getContractFactory("NFTSale");

    RIDER = await RIDER.deploy(TOTAL_TOKEN_SUPPLY);

    RiderNFT = await RiderNFT.deploy(CONTRACT_NAME, CONTRACT_SYMBOL);

    riderSaleNFT = await NFTSale.deploy(
      RiderNFT.address,
      RIDER.address,
      PIRCES,
      PLACEHODLER_URIS,
      MAX_PURCHASED
    );

    tokenContract = await RIDER.deployed();
    nftContract = await RiderNFT.deployed();
    saleNFTContract = await riderSaleNFT.deployed();
  });

  console.log("Start test RIDER contract...");

  /* RIDER Contract */
  it("Should claim 10000 token of account from RID Token contract", async () => {
    await tokenContract.connect(addr1).claim();
  });

  it("Should check balanceOf of account from RID Token contract", async () => {
    await tokenContract.connect(addr1).balanceOf(addr1.address);
  });

  /* RiderNFT Contract */
  it("Should NOT setEnabled if caller is not admin", async () => {
    try {
      await RiderNFT
        .connect(addr1)
        .mint(addr2.address, 1, "https://ipfs.io/ipfs/tokenCID/1.json");
    } catch (e) {
      assert(e.message.includes("Only admin role can perform this action"));
      return;
    }
    assert(false);
  });

  it("Should setEnabled if caller is admin", async () => {
    const tranction = await RiderNFT
      .connect(owner)
      .mint(addr1.address, 1, "https://ipfs.io/ipfs/tokenCID/");
    const result = await tranction.wait();
    expect(result.status === 1);
  });

  it("Should NOT setURI if caller is not admin", async () => {
    try {
      await RiderNFT
        .connect(addr1)
        .setURI(1, "https://ipfs.io/ipfs/tokenCID/");
    } catch (e) {
      assert(e.message.includes("Only admin role can perform this action"));
      return;
    }
    assert(false);
  });

  it("Should setURI if caller is admin", async () => {
    const tranction = await RiderNFT
      .connect(owner)
      .setURI(1, "https://ipfs.io/ipfs/tokenCID/");
    const result = await tranction.wait();
    expect(result.status === 1);
  });

  it("Should NOT setBatchURI if caller is not admin", async () => {
    try {
      const tranction = await RiderNFT
      .connect(owner)
      .mint(addr2.address, 2, "https://ipfs.io/ipfs/tokenCID/");
    const result = await tranction.wait();
      await RiderNFT
        .connect(addr1)
        .setBatchURI([1, 2], ["https://ipfs.io/ipfs/tokenCID1/", "https://ipfs.io/ipfs/tokenCID2/"]);
    } catch (e) {
      assert(e.message.includes("Only admin role can perform this action"));
      return;
    }
    assert(false);
  });

  it("Should setBatchURI if caller is admin", async () => {
    const tranction = await RiderNFT
      .connect(owner)
      .setBatchURI([1, 2], ["https://ipfs.io/ipfs/tokenCID1/", "https://ipfs.io/ipfs/tokenCID/2"]);
    const result = await tranction.wait();
    expect(result.status === 1);
  });
});
