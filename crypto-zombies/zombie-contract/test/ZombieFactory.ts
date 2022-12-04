import { expect } from "chai";
import { loadFixture } from "ethereum-waffle";
import { ethers } from "hardhat";

describe("ZombieFactory", function () {
    // We define a fixture to reuse the same setup in every test.
    // We use loadFixture to run this setup once, snapshot that state,
    // and reset Hardhat Network to that snapshot in every test.
    async function deployOneYearLockFixture() {
        // Contracts are deployed using the first signer/account by default
        const [owner, addr1, addr2] = await ethers.getSigners();

        //const tokenContract = "0x83B5a8807FAede3De5b51a3096ac2C14d55e3cB5";
        const ZombieFactory = await ethers.getContractFactory("ZombieAttack");
        const zombieFactory = await ZombieFactory.deploy();
        await zombieFactory.deployed();

        return { zombieFactory, owner, addr1, addr2 };
    }

    describe("Find Battle", function () {
        it("Attack Zombie", async function () {
            const { zombieFactory, owner, addr1, addr2 } = await loadFixture(
                deployOneYearLockFixture
            );

            const zombie1 = await zombieFactory
                .connect(addr1)
                .createRandomZombie("Duong");
            const zombie2 = await zombieFactory
                .connect(addr2)
                .createRandomZombie("Duong1");

            const zombie = await zombieFactory.connect(addr1).findBattle(0);
            zombieFactory.connect(addr1).attack(0 , 1);
            console.log(zombie);
        });
    });
});
