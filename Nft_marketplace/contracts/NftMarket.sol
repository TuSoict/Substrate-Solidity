// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

contract NftMarket is ERC721URIStorage {
    using Counters for Counters.Counter;

    Counters.Counter private _listedItems;
    Counters.Counter private _tokenIds;
    constructor() ERC721("TrungChanNFT", "TCNFT") {}

    mapping(string => bool) private _existsTokenURI;
    mapping(uint256 => NftItem) private _idToNftItem;

    struct NftItem {
        uint256 tokenId;
        uint256 price;
        address owner;
    }

    event NftItemCreate(uint256 tokenId, uint256 price, address owner);


    function mintToken(string memory tokenURI, uint256 price) public payable returns (uint256) {
        require(!checkTokenURIs(tokenURI), 'TokenURI has been existed');
        _listedItems.increment();
        _tokenIds.increment();

        uint newTokenId = _tokenIds.current();

        _safeMint(msg.sender, newTokenId);
        _setTokenURI(newTokenId, tokenURI);
        _NftItemCreated(newTokenId, price);
        _existsTokenURI[tokenURI] = true;

        return newTokenId;
    }

    function buyNft(uint tokenId) public payable {
        address owner = ERC721.ownerOf(tokenId);
        uint price = _idToNftItem[tokenId].price;

        require(msg.sender != owner, "you are owner of this token");
        require(msg.value == price, "you must submit the price");

        _transfer(owner, msg.sender, tokenId);
        payable(owner).transfer(msg.value);
    }

    function checkTokenURIs(string memory tokenURI) public view returns (bool) {
        return _existsTokenURI[tokenURI] == true;
    }

    function getInfoNft(uint256 tokenId) public view returns (NftItem memory) {
        return _idToNftItem[tokenId];
    }

    function listedNftItem() public view returns (uint) {
        return _listedItems.current();
    }

    function _NftItemCreated(uint tokenId, uint price) private {
        require(price >= 0, "Price must be at least 1 wei");

        _idToNftItem[tokenId] = NftItem(tokenId, price, msg.sender);
        emit NftItemCreate(tokenId, price, msg.sender);
    }

}