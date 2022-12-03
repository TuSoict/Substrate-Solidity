const {
  loadFixture,
} = require("@nomicfoundation/hardhat-network-helpers");
const { anyValue } = require("@nomicfoundation/hardhat-chai-matchers/withArgs");
const { expect } = require("chai");

describe("Faucet", function () {
  // We define a fixture to reuse the same setup in every test.
  // We use loadFixture to run this setup once, snapshot that state,
  // and reset Hardhat Network to that snapshot in every test.
  async function deployFaucetFixture() {
    const ONE_ETH = hre.ethers.utils.parseEther("1");

    const faucetAmount = ONE_ETH;

    // Contracts are deployed using the first signer/account by default
    const [owner, otherAccount] = await ethers.getSigners();

    const Faucet = await ethers.getContractFactory("Faucet");
    const faucet = await Faucet.deploy({ value: faucetAmount });

    return { faucet, faucetAmount, owner, otherAccount };
  }

  describe("Deployment", function () {
    it("Should set the right owner", async function () {
      const { faucet, owner } = await loadFixture(deployFaucetFixture);

      expect(await faucet.owner()).to.equal(owner.address);
    });

    it("Should receive and store the funds to faucet", async function () {
      const { faucet, faucetAmount } = await loadFixture(
        deployFaucetFixture
      );

      expect(await ethers.provider.getBalance(faucet.address)).to.equal(
        faucetAmount
      );
    });
  });

  describe("GetFaucet", function () {
    describe("Events", function () {
      it("Should emit an event when user get correct amount", async function () {
        const { faucet, otherAccount } = await loadFixture(
          deployFaucetFixture
        );
        const someETH = hre.ethers.utils.parseEther("0.01");
        await expect(faucet.connect(otherAccount).get_faucet(someETH))
          .to.emit(faucet, "GetFaucet")
          .withArgs(someETH);
      });
    });

    describe("Transfers", function () {
      it("Should revert with too much amount", async function () {
        const { faucet, otherAccount } = await loadFixture(
          deployFaucetFixture
        );
        const someETH = hre.ethers.utils.parseEther("0.1");
        await expect(faucet.connect(otherAccount).get_faucet(someETH))
          .to.be.reverted;
      });
    });
  });
});
