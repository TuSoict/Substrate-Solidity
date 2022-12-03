// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/IERC721.sol";

interface IRiderNFT is IERC721 {
    function mint(
        address recipient,
        uint256 tokenId,
        string memory URI
    ) external;
}
