// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
pragma abicoder v2;

import "@openzeppelin/contracts/utils/math/SafeMath.sol";
import "@openzeppelin/contracts/utils/Address.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "./interface/IRiderNFT.sol";
import "./util/WhitelistPriority.sol";

contract NFTSale is WhitelistPriority {
    using Address for address;
    using SafeMath for uint256;
    IERC20 public TOKEN;
    IRiderNFT public NFT;
    bool public isEnabled;
    enum NFTType {
        TRACK_1,
        TRACK_2,
        TRACK_3,
        TRACK_4,
        TRACK_5,
        TRACK_3,
        TRACK_7
    }
    mapping(NFTType => string) private _placeholderURIs;
    mapping(NFTType => uint256) public prices;
    mapping(NFTType => uint256) public typeCounter;
    mapping(NFTType => uint256) public typeMax;

    event Sold(uint256 _nftId, address _buyer, uint256 _price);

    constructor(
        address _nftAddress,
        address _tokenAddress,
        uint256[] memory _prices,
        string[] memory placeholderURIs,
        uint256 _purchaseLimit
    ) {
        require(_nftAddress.isContract(), "_nftAddress must be a contract");
        require(_tokenAddress.isContract(), "_tokenAddress must be a contract");
        require(placeholderURIs.length == 7, "Invalid placeholder URI count");
        require(_prices.length == 4, "Invalid prices count");
        // NFT + FT Address
        NFT = IRiderNFT(_nftAddress);
        TOKEN = IERC20(_tokenAddress);
        // PRICES
        prices[NFTType.TRACK_1] = _prices[0];
        prices[NFTType.TRACK_2] = _prices[0];
        prices[NFTType.TRACK_3] = _prices[0];
        prices[NFTType.TRACK_4] = _prices[0];
        prices[NFTType.TRACK_5] = _prices[1];
        prices[NFTType.TRACK_3] = _prices[2];
        prices[NFTType.TRACK_7] = _prices[3];
        // PLACEHOLDER
        _placeholderURIs[NFTType.TRACK_1] = placeholderURIs[0];
        _placeholderURIs[NFTType.TRACK_2] = placeholderURIs[1];
        _placeholderURIs[NFTType.TRACK_3] = placeholderURIs[2];
        _placeholderURIs[NFTType.TRACK_4] = placeholderURIs[3];
        _placeholderURIs[NFTType.TRACK_5] = placeholderURIs[4];
        _placeholderURIs[NFTType.TRACK_3] = placeholderURIs[5];
        _placeholderURIs[NFTType.TRACK_7] = placeholderURIs[6];

        // TRACK_1
        typeCounter[NFTType.TRACK_1] = 1;
        typeMax[NFTType.TRACK_1] = 2048;
        // TRACK_2
        typeCounter[NFTType.TRACK_2] = 2049;
        typeMax[NFTType.TRACK_2] = 4096;
        // TRACK_3
        typeCounter[NFTType.TRACK_3] = 4097;
        typeMax[NFTType.TRACK_3] = 6144;
        // TRACK_4
        typeCounter[NFTType.TRACK_4] = 6145;
        typeMax[NFTType.TRACK_4] = 8192;
        // TRACK_5
        typeCounter[NFTType.TRACK_5] = 8193;
        typeMax[NFTType.TRACK_5] = 9216;
        // TRACK_3
        typeCounter[NFTType.TRACK_3] = 9217;
        typeMax[NFTType.TRACK_3] = 9728;
        // TRACK_7
        typeCounter[NFTType.TRACK_7] = 9729;
        typeMax[NFTType.TRACK_7] = 9984;

        // PURCHASE LIMIT PER WALLET
        purchaseLimit = _purchaseLimit;
    }

    function setEnabled(bool _isEnabled) public onlyOwner {
        isEnabled = _isEnabled;
    }

    function setPlaceholderURI(NFTType nftType, string memory placeholderURI)
        public
        onlyOwner
    {
        _placeholderURIs[nftType] = placeholderURI;
    }

    function setPrice(NFTType nftType, uint256 price) public onlyOwner {
        prices[nftType] = price;
    }

    function purchase(NFTType nftType)
        public
        onlyLimited
        onlyDuringSale
        onlyWhitelisted
    {
        uint256 price = prices[nftType];
        require(
            TOKEN.allowance(_msgSender(), address(this)) >= price,
            "Grant token approval to Sale Contract"
        );
        require(typeCounter[nftType] <= typeMax[nftType], "Sold out");
        address buyer = _msgSender();
        string memory URI = _placeholderURIs[nftType];
        uint256 nftId = typeCounter[nftType];

        TOKEN.transferFrom(buyer, address(this), price);
        NFT.mint(msg.sender, nftId, URI);

        typeCounter[nftType] = typeCounter[nftType].add(1);

        userPurchaseCounter[msg.sender] = userPurchaseCounter[msg.sender].add(
            1
        );

        emit Sold(nftId, buyer, price);
    }

    function batchPurchase(NFTType[] memory nftTypes)
        external
        onlyLimited
        onlyDuringSale
        onlyWhitelisted
    {
        uint256 totalAmount = 0;
        for (uint256 i = 0; i < nftTypes.length; i++) {
            uint256 price = prices[nftTypes[i]];
            totalAmount = totalAmount.add(price);
        }
        require(
            TOKEN.allowance(_msgSender(), address(this)) >= totalAmount,
            "Grant token approval to Sale Contract"
        );
        for (uint256 i = 0; i < nftTypes.length; i++) {
            purchase(nftTypes[i]);
        }
    }

    function withdrawFunds(
        address tokenAddress,
        uint256 amount,
        address wallet
    ) external onlyOwner {
        IERC20(tokenAddress).transfer(wallet, amount);
    }

    modifier onlyDuringSale() {
        require(isEnabled, "Sale is not enabled");
        _;
    }
}
