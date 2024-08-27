// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Pausable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol";

contract StakingToken is ERC20, ERC20Burnable, ERC20Pausable, Ownable, ERC20Permit {
    bool public _isTradeable;
    mapping (address => uint256) private shares;
    mapping (address => mapping (address => uint256)) private allowances;
    bytes32 internal constant TOTAL_SHARES_POSITION = 0x995b7d48fc5e81e4f46d4dc686e0ca01dee4c0ba7e8e6c94ac9863ca73330b28;
    uint256 constant internal INFINITE_ALLOWANCE = ~uint256(0);
    constructor(address initialOwner, string memory name_, string memory ticker_, uint256 amount_, bool isTradeable_)
        ERC20(name_, ticker_)
        Ownable(initialOwner)
        ERC20Permit(name_)
    {
        _isTradeable = isTradeable_;
        _mint(msg.sender, amount_);
        if (!_isTradeable) _pause();
    }

    function pause() public onlyOwner {
        _pause();
    }

    function unpause() public onlyOwner {
        _unpause();
    }

    function givePoints(address to, uint256 amount) public onlyOwner {
        if(!(_isTradeable)) {
            _unpause();
            _mint(to, amount);
            pause();
        }
        else {
            _mint();
        }
    }

    function _update(address from, address to, uint256 value)
        internal
        override(ERC20, ERC20Pausable)
    {
        super._update(from, to, value);
    }

    event TransferShares(
        address indexed from,
        address indexed to,
        uint256 sharesValue
    );

    event SharesBurnt(
        address indexed account,
        uint256 preRebaseTokenAmount,
        uint256 postRebaseTokenAmount,
        uint256 sharesAmount
    );

    function getTotalShares() external view returns (uint256) {
        return _getTotalShares();
    }
    
    function sharesOf(address _account) external view returns (uint256) {
        return _sharesOf(_account);
    }

    function getSharesByPooledToken(uint256 _ethAmount) public view returns (uint256) {
        return _tokenAmount
            .mul(_getTotalShares())
            .div(_getTotalPooledToken());
    }

    function getPooledTokenByShares(uint256 _sharesAmount) public view returns (uint256) {
        return _sharesAmount
            .mul(_getTotalPooledToken())
            .div(_getTotalShares());
    }

    function transferShares(address _recipient, uint256 _sharesAmount) external returns (uint256) {
        _transferShares(msg.sender, _recipient, _sharesAmount);
        uint256 tokensAmount = getPooledTokenByShares(_sharesAmount);
        _emitTransferEvents(msg.sender, _recipient, tokensAmount, _sharesAmount);
        return tokensAmount;
    }

    function transferSharesFrom(
        address _sender, address _recipient, uint256 _sharesAmount
    ) external returns (uint256) {
        uint256 tokensAmount = getPooledTokenByShares(_sharesAmount);
        _spendAllowance(_sender, msg.sender, tokensAmount);
        _transferShares(_sender, _recipient, _sharesAmount);
        _emitTransferEvents(_sender, _recipient, tokensAmount, _sharesAmount);
        return tokensAmount;
    }

    function _getTotalPooledToken() internal view returns (uint256);

    function _transfer(address _sender, address _recipient, uint256 _amount) internal {
        uint256 _sharesToTransfer = getSharesByPooledToken(_amount);
        _transferShares(_sender, _recipient, _sharesToTransfer);
        _emitTransferEvents(_sender, _recipient, _amount, _sharesToTransfer);
    }

    function _approve(address _owner, address _spender, uint256 _amount) internal {
        require(_owner != address(0), "APPROVE_FROM_ZERO_ADDR");
        require(_spender != address(0), "APPROVE_TO_ZERO_ADDR");

        allowances[_owner][_spender] = _amount;
        emit Approval(_owner, _spender, _amount);
    }

    function _spendAllowance(address _owner, address _spender, uint256 _amount) internal {
        uint256 currentAllowance = allowances[_owner][_spender];
        if (currentAllowance != INFINITE_ALLOWANCE) {
            require(currentAllowance >= _amount, "ALLOWANCE_EXCEEDED");
            _approve(_owner, _spender, currentAllowance - _amount);
        }
    }

    function _getTotalShares() internal view returns (uint256) {
        return TOTAL_SHARES_POSITION.getStorageUint256();
    }

    function _sharesOf(address _account) internal view returns (uint256) {
        return shares[_account];
    }

    function _transferShares(address _sender, address _recipient, uint256 _sharesAmount) internal {
        require(_sender != address(0), "TRANSFER_FROM_ZERO_ADDR");
        require(_recipient != address(0), "TRANSFER_TO_ZERO_ADDR");
        require(_recipient != address(this), "TRANSFER_TO_STETH_CONTRACT");
        uint256 currentSenderShares = shares[_sender];
        require(_sharesAmount <= currentSenderShares, "BALANCE_EXCEEDED");
        shares[_sender] = currentSenderShares.sub(_sharesAmount);
        shares[_recipient] = shares[_recipient].add(_sharesAmount);
    }

    function _mintShares(address _recipient, uint256 _sharesAmount) internal returns (uint256 newTotalShares) {
        require(_recipient != address(0), "MINT_TO_ZERO_ADDR");

        newTotalShares = _getTotalShares().add(_sharesAmount);
        TOTAL_SHARES_POSITION.setStorageUint256(newTotalShares);

        shares[_recipient] = shares[_recipient].add(_sharesAmount);
    }

    function _burnShares(address _account, uint256 _sharesAmount) internal returns (uint256 newTotalShares) {
        require(_account != address(0), "BURN_FROM_ZERO_ADDR");

        uint256 accountShares = shares[_account];
        require(_sharesAmount <= accountShares, "BALANCE_EXCEEDED");

        uint256 preRebaseTokenAmount = getPooledTokenByShares(_sharesAmount);

        newTotalShares = _getTotalShares().sub(_sharesAmount);
        TOTAL_SHARES_POSITION.setStorageUint256(newTotalShares);

        shares[_account] = accountShares.sub(_sharesAmount);

        uint256 postRebaseTokenAmount = getPooledTokenByShares(_sharesAmount);

        emit SharesBurnt(_account, preRebaseTokenAmount, postRebaseTokenAmount, _sharesAmount);
    }

    function _emitTransferEvents(address _from, address _to, uint _tokenAmount, uint256 _sharesAmount) internal {
        emit Transfer(_from, _to, _tokenAmount);
        emit TransferShares(_from, _to, _sharesAmount);
    }

    function _emitTransferAfterMintingShares(address _to, uint256 _sharesAmount) internal {
        _emitTransferEvents(address(0), _to, getPooledTokenByShares(_sharesAmount), _sharesAmount);
    }

    function _mintInitialShares(address initialHolder, uint256 _sharesAmount) internal {
        _mintShares(initialHolder, _sharesAmount);
        _emitTransferAfterMintingShares(initialHolder, _sharesAmount);
    }

}