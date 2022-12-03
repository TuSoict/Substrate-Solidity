// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

// Uncomment this line to use console.log
// import "hardhat/console.sol";

contract BoxV2 {
    uint256 private _value;

    event ValueChanged(uint256 _value);
    function store(uint256 v) public {
        _value = v;
        emit ValueChanged(
            v
        );
    }
    function retrieve() public view returns (uint256 v){
        v = _value;
    }

    // Increments the stored value by 1
    function increment() public {
        _value = _value + 1;
        emit ValueChanged(_value);
    }
}