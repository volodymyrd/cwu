use alloy::{
    primitives::{
        U256, address,
        utils::{Unit, format_ether, format_units},
    },
    providers::ProviderBuilder,
    sol,
};

// Generate bindings for the WETH9 contract
sol! {
    #[sol(rpc)]
    contract WETH9 {
        function deposit() public payable;
        function balanceOf(address) public view returns (uint256);
        function withdraw(uint amount) public;
        function decimals() external view returns (uint8);
        function name() external view returns (string);
        function symbol() external view returns (string);
    }
}
