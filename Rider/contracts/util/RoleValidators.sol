// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Context.sol";

abstract contract RoleValidators is Context, Ownable {
    mapping(address => bool) internal _admins;

    event AdminAccessSet(address _admin, bool _enabled);

    function setRoleAdmin(address admin, bool enabled) external onlyOwner {
        _admins[admin] = enabled;
        emit AdminAccessSet(admin, enabled);
    }

    function isAdmin(address admin) public view returns (bool) {
        return _admins[admin];
    }

    modifier onlyAdmin() {
        require(
            _admins[_msgSender()] || _msgSender() == owner(),
            "Only admin role can perform this action"
        );
        _;
    }
}
