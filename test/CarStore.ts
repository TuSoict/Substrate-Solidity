import { expect } from "chai";
import { ethers } from "hardhat";

describe("CarStore", function () {
  // We define a fixture to reuse the same setup in every test.
  // We use loadFixture to run this setup once, snapshot that state,
  // and reset Hardhat Network to that snapshot in every test.

  describe("Deployment", function () {
    const price = ethers.utils.parseUnits("1", "ether");
    it("Should fail if the unlockTime is not in the future", async function () {
      // We don't use the fixture here because we want a different deployment
      const [owner, otherAccount] = await ethers.getSigners();
      const CarStore = await ethers.getContractFactory("CarStore");
        const carStore = await CarStore.connect(owner).deploy();
      console.log(carStore.address);
      const tx1 = await carStore.connect(owner).addCarType("bmw1", 1, "BMW", price);

      const carType1 = carStore.listCarType(0);

      const tx2 = await carStore.addCart((await carType1).carTypeId.toNumber());
      const car1 = carStore.listCar(0);
      const tx3 = await carStore.connect(otherAccount).buyCar((await car1).carId.toNumber(), {
        value: price,
      });
      expect(tx3).to.be.reverted;
    });
  });
});
