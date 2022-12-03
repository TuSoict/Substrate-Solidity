// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
pragma abicoder v2;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "./util/RoleValidators.sol";

contract RiderNFT is ERC721URIStorage, RoleValidators {
    constructor(string memory _name, string memory _symbol)
        ERC721(_name, _symbol) {}

    function mint(
        address recipient,
        uint256 tokenId,
        string memory URI
    ) external onlyAdmin {
        _mint(recipient, tokenId);
        _setTokenURI(tokenId, URI);
    }

    function setURI(uint256 tokenId, string memory tokenURI)
        external
        onlyAdmin
    {
        _setTokenURI(tokenId, tokenURI);
    }

    function setBatchURI(uint256[] memory tokenIds, string[] memory tokenURIs)
        external
        onlyAdmin
    {
        require(
            tokenIds.length == tokenURIs.length,
            "Invalid input for tokenIds and tokenURIs"
        );
        for (uint256 i = 0; i < tokenIds.length; i++) {
            _setTokenURI(tokenIds[i], tokenURIs[i]);
        }
    }
}
