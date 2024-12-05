// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Script.sol";
// import "openzeppelin-foundry-upgrades/Upgrades.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";
import "openzeppelin-foundry-upgrades/Options.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import "../contracts/BridgeCommittee.sol";
import "../contracts/BridgeVault.sol";
import "../contracts/BridgeConfig.sol";
import "../contracts/BridgeLimiter.sol";
import "../contracts/IkaBridge.sol";
import "../test/mocks/MockTokens.sol";

contract DeployBridge is Script {
    function parseDeployConfig(string memory path) public returns (DeployConfig memory) {
        string memory json = vm.readFile(path);
        DeployConfig memory config;

        config.committeeMemberStake = abi.decode(vm.parseJson(json, ".committeeMemberStake"), (uint256[]));
        config.committeeMembers = abi.decode(vm.parseJson(json, ".committeeMembers"), (address[]));
        config.minCommitteeStakeRequired = abi.decode(vm.parseJson(json, ".minCommitteeStakeRequired"), (uint256));
        config.sourceChainId = abi.decode(vm.parseJson(json, ".sourceChainId"), (uint256));
        config.supportedChainIds = abi.decode(vm.parseJson(json, ".supportedChainIds"), (uint256[]));
        config.supportedChainLimitsInDollars = abi.decode(vm.parseJson(json, ".supportedChainLimitsInDollars"), (uint256[]));
        config.tokenPrices = abi.decode(vm.parseJson(json, ".tokenPrices"), (uint256[]));
        config.supportedTokens = abi.decode(vm.parseJson(json, ".supportedTokens"), (address[]));
        config.tokenIds = abi.decode(vm.parseJson(json, ".tokenIds"), (uint256[]));
        config.ikaDecimals = abi.decode(vm.parseJson(json, ".ikaDecimals"), (uint256[]));
        config.weth = abi.decode(vm.parseJson(json, ".weth"), (address));

        return config;
    }

    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);
        string memory chainID = Strings.toString(block.chainid);
        bytes32 chainIDHash = keccak256(abi.encode(chainID));
        bool isLocal = chainIDHash != keccak256(abi.encode("11155111"))
            && chainIDHash != keccak256(abi.encode("1"));
        string memory root = vm.projectRoot();
        string memory path = string.concat(root, "/deploy_configs/", chainID, ".json");
        // If this is local deployment, we override the path if OVERRIDE_CONFIG_PATH is set.
        // This is useful in integration tests where config path is not fixed.
        if (isLocal) {
            path = vm.envOr("OVERRIDE_CONFIG_PATH", path);
        }

        console.log("config path: ", path);
        DeployConfig memory deployConfig = parseDeployConfig(path);

        // if deploying to local network, deploy mock tokens
        if (isLocal) {
            console.log("Deploying mock tokens for local network");
            // deploy WETH
            deployConfig.weth = address(new WETH());

            // deploy mock tokens
            MockWBTC wBTC = new MockWBTC();
            MockUSDC USDC = new MockUSDC();
            MockUSDT USDT = new MockUSDT();
            MockKA KA = new MockKA();
            console.log("[Deployed] KA:", address(KA));

            // update deployConfig with test values
            deployConfig.supportedTokens = new address[](5);
            deployConfig.supportedTokens[0] = address(0);
            deployConfig.supportedTokens[1] = address(wBTC);
            deployConfig.supportedTokens[2] = deployConfig.weth;
            deployConfig.supportedTokens[3] = address(USDC);
            deployConfig.supportedTokens[4] = address(USDT);

            deployConfig.tokenIds = new uint256[](5);
            deployConfig.tokenIds[0] = 0;
            deployConfig.tokenIds[1] = 1;
            deployConfig.tokenIds[2] = 2;
            deployConfig.tokenIds[3] = 3;
            deployConfig.tokenIds[4] = 4;

            deployConfig.ikaDecimals = new uint256[](5);
            deployConfig.ikaDecimals[0] = 9;
            deployConfig.ikaDecimals[1] = 8;
            deployConfig.ikaDecimals[2] = 8;
            deployConfig.ikaDecimals[3] = 6;
            deployConfig.ikaDecimals[4] = 6;
        }

        // convert supported chains from uint256 to uint8
        uint8[] memory supportedChainIds = new uint8[](deployConfig.supportedChainIds.length);
        for (uint256 i; i < deployConfig.supportedChainIds.length; i++) {
            supportedChainIds[i] = uint8(deployConfig.supportedChainIds[i]);
        }

        require(
            deployConfig.supportedTokens.length == deployConfig.tokenPrices.length,
            "supportedTokens.length != tokenPrices.length"
        );
        require(
            deployConfig.supportedTokens.length == deployConfig.tokenIds.length,
            "supportedTokens.length != tokenIds.length"
        );
        require(
            deployConfig.supportedTokens.length == deployConfig.ikaDecimals.length,
            "supportedTokens.length != ikaDecimals.length"
        );

        // deploy Bridge Committee ===================================================================

        // convert committeeMembers stake from uint256 to uint16
        uint16[] memory committeeMemberStake =
            new uint16[](deployConfig.committeeMemberStake.length);
        for (uint256 i; i < deployConfig.committeeMemberStake.length; i++) {
            committeeMemberStake[i] = uint16(deployConfig.committeeMemberStake[i]);
        }

        Options memory opts;
        opts.unsafeSkipAllChecks = true;

        address bridgeCommittee = Upgrades.deployUUPSProxy(
            "BridgeCommittee.sol",
            abi.encodeCall(
                BridgeCommittee.initialize,
                (
                    deployConfig.committeeMembers,
                    committeeMemberStake,
                    uint16(deployConfig.minCommitteeStakeRequired)
                )
            ),
            opts
        );

        // deploy bridge config =====================================================================

        // convert token prices from uint256 to uint64
        uint64[] memory tokenPrices = new uint64[](deployConfig.tokenPrices.length);
        for (uint256 i; i < deployConfig.tokenPrices.length; i++) {
            tokenPrices[i] = uint64(deployConfig.tokenPrices[i]);
        }

        // convert Ika Decimals from uint256 to uint8
        uint8[] memory ikaDecimals = new uint8[](deployConfig.ikaDecimals.length);
        for (uint256 i; i < deployConfig.ikaDecimals.length; i++) {
            ikaDecimals[i] = uint8(deployConfig.ikaDecimals[i]);
        }

        // convert Token Id from uint256 to uint8
        uint8[] memory tokenIds = new uint8[](deployConfig.tokenIds.length);
        for (uint256 i; i < deployConfig.tokenIds.length; i++) {
            tokenIds[i] = uint8(deployConfig.tokenIds[i]);
        }

        address bridgeConfig = Upgrades.deployUUPSProxy(
            "BridgeConfig.sol",
            abi.encodeCall(
                BridgeConfig.initialize,
                (
                    address(bridgeCommittee),
                    uint8(deployConfig.sourceChainId),
                    deployConfig.supportedTokens,
                    tokenPrices,
                    tokenIds,
                    ikaDecimals,
                    supportedChainIds
                )
            ),
            opts
        );

        // initialize config in the bridge committee
        BridgeCommittee(bridgeCommittee).initializeConfig(address(bridgeConfig));
        BridgeCommittee committeeImplementation =
            BridgeCommittee(Upgrades.getImplementationAddress(bridgeCommittee));
        committeeImplementation.initializeConfig(address(bridgeConfig));

        // deploy vault =============================================================================

        BridgeVault vault = new BridgeVault(deployConfig.weth);

        // deploy limiter ===========================================================================

        // convert chain limits from uint256 to uint64[]
        uint64[] memory chainLimits =
            new uint64[](deployConfig.supportedChainLimitsInDollars.length);
        for (uint256 i; i < deployConfig.supportedChainLimitsInDollars.length; i++) {
            chainLimits[i] = uint64(deployConfig.supportedChainLimitsInDollars[i]);
        }

        address limiter = Upgrades.deployUUPSProxy(
            "BridgeLimiter.sol",
            abi.encodeCall(
                BridgeLimiter.initialize, (bridgeCommittee, supportedChainIds, chainLimits)
            ),
            opts
        );

        uint8[] memory _destinationChains = new uint8[](1);
        _destinationChains[0] = 1;

        // deploy Ika Bridge ========================================================================

        address ikaBridge = Upgrades.deployUUPSProxy(
            "IkaBridge.sol",
            abi.encodeCall(IkaBridge.initialize, (bridgeCommittee, address(vault), limiter)),
            opts
        );

        // transfer vault ownership to bridge
        vault.transferOwnership(ikaBridge);
        // transfer limiter ownership to bridge
        BridgeLimiter instance = BridgeLimiter(limiter);
        instance.transferOwnership(ikaBridge);

        // print deployed addresses for post deployment setup
        console.log("[Deployed] BridgeConfig:", bridgeConfig);
        console.log("[Deployed] IkaBridge:", ikaBridge);
        console.log("[Deployed] BridgeLimiter:", limiter);
        console.log("[Deployed] BridgeCommittee:", bridgeCommittee);
        console.log("[Deployed] BridgeVault:", address(vault));
        console.log("[Deployed] BTC:", BridgeConfig(bridgeConfig).tokenAddressOf(1));
        console.log("[Deployed] ETH:", BridgeConfig(bridgeConfig).tokenAddressOf(2));
        console.log("[Deployed] USDC:", BridgeConfig(bridgeConfig).tokenAddressOf(3));
        console.log("[Deployed] USDT:", BridgeConfig(bridgeConfig).tokenAddressOf(4));

        vm.stopBroadcast();
    }

    // used to ignore for forge coverage
    function testSkip() public {}
}

/// check the following for guidelines on updating deploy_configs and references:
/// https://book.getfoundry.sh/cheatcodes/parse-json
struct DeployConfig {
    uint256[] committeeMemberStake;
    address[] committeeMembers;
    uint256 minCommitteeStakeRequired;
    uint256 sourceChainId;
    uint256[] supportedChainIds;
    uint256[] supportedChainLimitsInDollars;
    address[] supportedTokens;
    uint256[] tokenPrices;
    uint256[] tokenIds;
    uint256[] ikaDecimals;
    address weth;
}
