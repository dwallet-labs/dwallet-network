// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// @title IBridgeConfig
/// @dev Interface for the BridgeConfig contract.
interface IBridgeConfig {
    /* ========== STRUCTS ========== */

    /// @notice The data struct for the supported bridge tokens.
    struct Token {
        address tokenAddress;
        uint8 ikaDecimal;
        bool native;
    }

    /* ========== VIEW FUNCTIONS ========== */

    /// @notice Returns the address of the token with the given ID.
    /// @param tokenID The ID of the token.
    /// @return address of the provided token.
    function tokenAddressOf(uint8 tokenID) external view returns (address);

    /// @notice Returns the ika decimal places of the token with the given ID.
    /// @param tokenID The ID of the token.
    /// @return amount of ika decimal places of the provided token.
    function tokenIkaDecimalOf(uint8 tokenID) external view returns (uint8);

    /// @notice Returns the price of the token with the given ID.
    /// @param tokenID The ID of the token.
    /// @return price of the provided token.
    function tokenPriceOf(uint8 tokenID) external view returns (uint64);

    /// @notice Returns the supported status of the token with the given ID.
    /// @param tokenID The ID of the token.
    /// @return true if the token is supported, false otherwise.
    function isTokenSupported(uint8 tokenID) external view returns (bool);

    /// @notice Returns whether a chain is supported in IkaBridge with the given ID.
    /// @param chainId The ID of the chain.
    /// @return true if the chain is supported, false otherwise.
    function isChainSupported(uint8 chainId) external view returns (bool);

    /// @notice Returns the chain ID of the bridge.
    function chainID() external view returns (uint8);

    /// @notice Event for the addition of a new token.
    /// @param nonce The governance action nonce.
    /// @param tokenIDs The IDs of the tokens added.
    /// @param tokenAddresses The addresses of the tokens added.
    /// @param ikaDecimals The added token's decimal places on Ika.
    /// @param tokenPrices The prices of the tokens added in USD.
    event TokensAddedV2(
        uint64 nonce,
        uint8[] tokenIDs,
        address[] tokenAddresses,
        uint8[] ikaDecimals,
        uint64[] tokenPrices
    );

    /// @dev (deprecated in favor of TokensAddedV2)
    event TokenAdded(uint8 tokenID, address tokenAddress, uint8 ikaDecimal, uint64 tokenPrice);

    /// @notice Event for the price update of a token.
    /// @param nonce The governance action nonce.
    /// @param tokenID The ID of the token updated.
    /// @param tokenPrice The new price of the token in USD.
    event TokenPriceUpdatedV2(uint64 nonce, uint8 tokenID, uint64 tokenPrice);

    /// @dev (deprecated in favor of TokenPriceUpdatedV2)
    event TokenPriceUpdated(uint8 tokenID, uint64 tokenPrice);
}
