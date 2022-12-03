// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
pragma abicoder v2;

import "./RoleValidators.sol";

abstract contract WhitelistPriority is RoleValidators {
    mapping(address => Whitelist) private _whitelisted;
    WhitelistType public activeWhitelistType;

    mapping(address => uint256) public userPurchaseCounter;
    uint256 public purchaseLimit;
    bool public isOpenForAll;

    enum WhitelistType {
        PRIMARY,
        SECONDARY
    }
    struct Whitelist {
        bool isWhitelisted;
        WhitelistType whitelistType;
    }

    event Whitelisted(address _user, WhitelistType whitelistType);
    event RemoveWhiteList(address _user);
    event SetOpenForAll(bool _isOpenForAll);

    // SET WHICH WHITELIST IS ACTIVE
    function setActiveWhitelistType(WhitelistType whitelistType)
        public
        onlyAdmin
    {
        activeWhitelistType = whitelistType;
    }

    // SET PURCHASE LIMIT
    function setPurchaseLimit(uint256 _purchaseLimit) public onlyAdmin {
        purchaseLimit = _purchaseLimit;
    }

    // SET WHITELIST USERS
    function whitelist(address user, WhitelistType whitelistType)
        public
        onlyAdmin
    {
        _whitelisted[user].isWhitelisted = true;
        _whitelisted[user].whitelistType = whitelistType;
        emit Whitelisted(user, whitelistType);
    }

    function whitelistBatch(
        address[] memory users,
        WhitelistType[] memory whitelistTypes
    ) external onlyAdmin {
        for (uint256 i = 0; i < users.length; i++) {
            whitelist(users[i], whitelistTypes[i]);
        }
    }

    function removeWhiteList(address user) public onlyAdmin {
        _whitelisted[user].isWhitelisted = false;
        emit RemoveWhiteList(user);
    }

    function removeWhiteListBatch(address[] memory users) external onlyAdmin {
        for (uint256 i = 0; i < users.length; i++) {
            removeWhiteList(users[i]);
        }
    }

    // SET IS OPENFORALL
    function setOpenForAll(bool _isOpenForAll) external onlyAdmin {
        isOpenForAll = _isOpenForAll;
        emit SetOpenForAll(_isOpenForAll);
    }

    function isWhitelisted(address user)
        public
        view
        returns (Whitelist memory)
    {
        return _whitelisted[user];
    }

    modifier onlyLimited() {
        require(
            userPurchaseCounter[_msgSender()] < purchaseLimit,
            "Purchase limit reached"
        );
        _;
    }

    modifier onlyWhitelisted() {
        require(
            ((_whitelisted[_msgSender()].isWhitelisted) &&
                (_whitelisted[_msgSender()].whitelistType ==
                    activeWhitelistType)) ||
                ((_whitelisted[_msgSender()].isWhitelisted) &&
                    (_whitelisted[_msgSender()].whitelistType ==
                        WhitelistType.PRIMARY)) ||
                _admins[_msgSender()] ||
                _msgSender() == owner() ||
                isOpenForAll,
            "Only whitelisted users can perform this action"
        );
        _;
    }
}
