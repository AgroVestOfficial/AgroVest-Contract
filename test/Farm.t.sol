// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {Farm} from "../src/Farm.sol";
import {console} from "forge-std/console.sol";
import {Error} from "../src/Library/Error.sol";

contract FarmTest is Test {
    Farm public farm;
    address farmOwner = makeAddr("farmOwner");
    address productBuyer = makeAddr("productBuyer");

    function setUp() public {
        address TokenAddress = makeAddr("Token");

        address EscrowAddress = makeAddr("Escrow");
        farm = new Farm(TokenAddress, EscrowAddress);
    }

    function test_Farm_Registration() public {
        vm.startPrank(farmOwner);

        string memory phoneContact = "701234";

        vm.expectRevert(Error.NameCannotBeEmpty.selector);

        farm.registerFarms(
            "",
            "AbelFarmPicture",
            "Lagos, Nigeria",
            phoneContact,
            farmOwner,
            "example@gmail.com"
        );

        farm.registerFarms(
            "AbelFarm",
            "AbelFarmPicture",
            "Lagos, Nigeria",
            phoneContact,
            farmOwner,
            "example@gmail.com"
        );
        vm.stopPrank();
    }

    function test_FarmDetails_Update() public {
        test_Farm_Registration();

        address impersonator = makeAddr("Impersonator");
        vm.expectRevert(Error.InvalidFarmIndex.selector);
        farm.updateDetails(
            111,
            "AbelSnailFarm",
            "AbelSnailFarmPicture",
            "Lagos, Nigeria",
            "706555",
            "example@gmail.com"
        );
        vm.expectRevert(Error.YouAreNotRegistered.selector);
        farm.updateDetails(
            0,
            "AbelSnailFarm",
            "AbelSnailFarmPicture",
            "Lagos, Nigeria",
            "706555",
            "example@gmail.com"
        );

        vm.startPrank(impersonator);
        farm.registerFarms(
            "FakeFarm",
            "FakeFarmPicture",
            "Lagos, Nigeria",
            "701233",
            impersonator,
            "example@gmail.com"
        );
        vm.expectRevert(Error.TheFarmDoesNotBelongToYou.selector);
        farm.updateDetails(
            0,
            "AbelSnailFarm",
            "AbelSnailFarmPicture",
            "Lagos, Nigeria",
            "706555",
            "example@gmail.com"
        );
        vm.stopPrank();

        vm.startPrank(farmOwner);
        farm.updateDetails(
            0,
            "AbelSnailFarm",
            "AbelSnailFarmPicture",
            "Lagos, Nigeria",
            "706555",
            "example@gmail.com"
        );
        vm.stopPrank();
    }

    function test_Add_Farm_Product() public {
        test_Farm_Registration();
        vm.startPrank(farmOwner);
        farm.addFarmProduct(
            "Corn",
            "CornPicture",
            "New Harvest of corns",
            uint256(1)
        );
        vm.stopPrank();
    }

    function test_Update_Farm_Product() public {
        test_Add_Farm_Product();
        vm.startPrank(farmOwner);
        vm.expectRevert(Error.InvalidProductIndex.selector);
        farm.updateFarmProduct(
            11,
            "Maize",
            "MaizePicture",
            "New Harvest of Maize",
            uint256(111)
        );
        farm.updateFarmProduct(
            0,
            "Corn",
            "CornPicture",
            "New Harvest of corns",
            uint256(1)
        );
    }

    function test_Add_Product_To_Cart() public {
        test_Add_Farm_Product();
        vm.startPrank(farmOwner);
        farm.addFarmProduct(
            "Ginger",
            "GingerPicture",
            "New Harvest of Gingers",
            uint256(5)
        );
        vm.stopPrank();
        vm.startPrank(productBuyer);
        farm.addProductToCart(1);
        vm.stopPrank();
    }

    //TODO: Test Product Purchase

    function test_Remove_Product_From_Cart() public {
        test_Add_Product_To_Cart();
        vm.startPrank(productBuyer);
        farm.removeProductFromCart(1);
        vm.stopPrank();
    }

    // function testFuzz_SetNumber(uint256 x) public {
    //     counter.setNumber(x);
    //     assertEq(counter.number(), x);
    // }
}
