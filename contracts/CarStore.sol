// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

struct CarType {
    uint256 carTypeId;
    string carTypeName;
    uint256 carTypeReleaseDate;
    string carTypeBrand;
    uint256 carTypePrice;
}

struct Car {
    uint256 carId;
    uint256 carTypeId;
    bool carAvailable;
}

contract CarStore is Ownable {
    CarType[] public listCarType;
    Car[] public listCar;

    constructor() Ownable() {}

    function addCarType(
        string memory carTypeName,
        uint256 carTypeReleaseDate,
        string memory carTypeBrand,
        uint256 carTypePrice
    ) public onlyOwner {
        listCarType.push(
            CarType(
                listCarType.length,
                carTypeName,
                carTypeReleaseDate,
                carTypeBrand,
                carTypePrice
            )
        );
    }

    function addCart(uint256 carTypeId) public onlyOwner returns(uint256) {
        listCar.push(Car(listCar.length, carTypeId, true));
        return 111;
    }

    function buyCar(uint256 carId) public payable {
        require(0 <= carId && carId < listCar.length, "Car: carId not found");
        Car memory carItem = listCar[carId];
        CarType memory carTypeItem = listCarType[carItem.carTypeId];
        require(
            msg.value == carTypeItem.carTypePrice,
            "Car: not enough money "
        );

        listCar[carId].carAvailable = false;
    }

    function withdraw(address payable to) public payable onlyOwner {
        (bool sent, bytes memory data) = to.call{value: msg.value}("");
        require(sent, "Car: Failed to send Ether");
    }
}
