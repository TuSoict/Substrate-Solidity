// SPDX-License-Identifier: UNLICENSE

pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract RIDER is ERC20 {
    constructor(uint256 initialSupply) ERC20("RIDER", "RID") {
        _mint(msg.sender, initialSupply);
        _mint(address(this), initialSupply);
    }

    uint256 amount = 10000000000000000000000; // 10000 Token

    function claim() public {
        ERC20(this).transfer(msg.sender, amount);
    }
}
