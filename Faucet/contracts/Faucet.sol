// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

// Uncomment this line to use console.log
// import "hardhat/console.sol";

contract Faucet {
    mapping(address => bool) public users;
    address payable public owner;

    event GetFaucet(uint256 amount);

    constructor() payable {
        owner = payable(msg.sender);
    }

    function get_faucet(uint256 amount) public payable {
        // Users can only get faucet once
        require(!users[msg.sender], "Address already got faucet");
        // Users can only get 0.01 ETH
        require(amount <= 10_000_000_000_000_000);
        // Out of faucet
        require(amount <= address(this).balance, "Out of faucet");
        (bool success, ) = msg.sender.call{value: amount}("");
        require(success, "Failed to get faucet");
        emit GetFaucet(amount);
    } 
    
    function withdraw_all_faucet() public {
        require(owner == msg.sender, "Not owner");
        (bool success, ) = msg.sender.call{value: address(this).balance}("");
        require(success, "Failed to withdraw all faucet");
    }
}
