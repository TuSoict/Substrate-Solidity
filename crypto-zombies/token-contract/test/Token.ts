import { ethers } from "hardhat";
import { expect } from "chai";

import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";

describe("Token contract", function () {
    async function deployTokenFixture() {
        const Token = await ethers.getContractFactory("Token");
        const [owner, addr1, addr2] = await ethers.getSigners();

        const testToken = await Token.deploy();
        await testToken.deployed();

        return { Token, testToken, owner, addr1, addr2 };
    }

    describe("Deployment", function () {
        it("Should set the right owner", async function () {
            const { testToken, owner } = await loadFixture(deployTokenFixture);
            expect(await testToken.owner()).to.equal(owner.address);
        });

        it("Should assign the total supply of tokens to the owner", async function () {
            const { testToken, owner } = await loadFixture(deployTokenFixture);
            const ownerBalance = await testToken.balanceOf(owner.address);
            expect(await testToken.totalSupply()).to.equal(ownerBalance);
        });
    });

    describe("Transactions", function () {
        it("Should transfer tokens between accounts", async function () {
            const { testToken, addr1, addr2 } = await loadFixture(
                deployTokenFixture
            );

            await testToken.transfer(addr1.address, 50);
            expect(await testToken.balanceOf(addr1.address)).to.equal(50);

            await testToken.connect(addr1).transfer(addr2.address, 50);
            expect(await testToken.balanceOf(addr2.address)).to.equal(50);
            expect(await testToken.balanceOf(addr1.address)).to.equal(0);
        });

        it("should emit Transfer events", async function () {
            const { testToken, owner, addr1, addr2 } = await loadFixture(
                deployTokenFixture
            );

            await expect(testToken.transfer(addr1.address, 50))
                .to.emit(testToken, "Transfer")
                .withArgs(owner.address, addr1.address, 50);

            await expect(testToken.connect(addr1).transfer(addr2.address, 50))
                .to.emit(testToken, "Transfer")
                .withArgs(addr1.address, addr2.address, 50);
        });

        it("Should fail if sender doesn't have enough tokens", async function () {
            const { testToken, owner, addr1 } = await loadFixture(
                deployTokenFixture
            );
            const initialOwnerBalance = await testToken.balanceOf(owner.address);

            await expect(
                testToken.connect(addr1).transfer(owner.address, 1)
            ).to.be.revertedWith("ERC20: transfer amount exceeds balance");

            expect(await testToken.balanceOf(owner.address)).to.equal(
                initialOwnerBalance
            );
        });
    });
});
