// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {Farm} from "../src/Farm.sol";
import {Investment} from "../src/Investment.sol";
import {console} from "forge-std/console.sol";
import {Error} from "../src/Library/Error.sol";
import "../test/Farm.t.sol";

contract InvestmentTest is Test {
    Investment public investment;
    Farm public farm;
    address investor = makeAddr("investor");
    address farmOwner = makeAddr("farmOwner");
    // address productBuyer = makeAddr("productBuyer");

    function setUp() public {
        address TokenAddress = makeAddr("Token");
        address EscrowAddress = makeAddr("Escrow");

        farm = new Farm(TokenAddress, EscrowAddress);

        investment = new Investment(TokenAddress);
    }

    function test_Farm_Registration() public {
        vm.startPrank(farmOwner);
        farm.registerFarms(
            "AbelFarm",
            "AbelFarmPicture",
            "Lagos, Nigeria",
            "7094843",
            farmOwner,
            "example@gmail.com"
        );
        vm.stopPrank();
    }

    function test_Create_Investment() public {
        test_Farm_Registration();
        vm.startPrank(investor);
        investment.createInvestment(
            0,
            "farmPicture",
            "AbelFarm",
            "Famer Of Agric",
            uint256(1),
            uint256(1012026),
            farmOwner
        );
        vm.stopPrank();
    }

    function test_Invest_Ether() public {
        test_Farm_Registration();

        deal(investor, 10 ether);
        hoax(investor, 10 ether);
        // vm.startPrank(investor);

        investment.investEthers{value: 1 ether}(0);
        uint256 totalInvestmentAfter = investment.getTotalInvestment();
        // vm.stopPrank();
        assertEq(totalInvestmentAfter, 1 ether);
    }

    // function test_Claim_Ether_Investment() public {
    //     test_Invest_Ether();
    // }

    function test_ClaimInvestment_Success() public {
        test_Invest_Ether();

        // Simulate farmer claiming funds
        hoax(farmOwner, 0);

        uint256 farmerBalanceBefore = farmOwner.balance;
        uint256 contractBalanceBefore = address(investment).balance;

        // Call claim function
        investment.claimInvestmentEthers(0, 1 ether);

        // Ensure the farmer received funds
        assertEq(farmOwner.balance, 1 ether);

        // Ensure the contract's balance decreased
        assertEq(address(investment).balance, 0 ether);
    }

    // function testFuzz_SetNumber(uint256 x) public {
    //     counter.setNumber(x);
    //     assertEq(counter.number(), x);
    // }
}
