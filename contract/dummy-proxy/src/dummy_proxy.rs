multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait DummyProxyModule {

    #[endpoint(callEndpoint)]
    fn call_endpoint(
        &self,
        contract_address: ManagedAddress,
        function_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let gas_left = self.blockchain().get_gas_left();
        let mut contract_call = self
            .send()
            .contract_call::<()>(contract_address, function_name)
            .with_gas_limit(gas_left);

        for arg in args {
            contract_call.push_raw_argument(arg);
        }
        let _: IgnoreValue = contract_call.execute_on_dest_context();
    }

    #[payable("*")]
    #[endpoint(callTransferEndpoint)]
    fn call_transfer_endpoint(
        &self,
        contract_address: ManagedAddress,
        function_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let payments = self.call_value().all_esdt_transfers();

        let mut contract_call = self
            .send()
            .contract_call::<()>(contract_address, function_name)
            .with_multi_token_transfer(payments);

        for arg in args {
            contract_call.push_raw_argument(arg);
        }

        let _: IgnoreValue = contract_call.execute_on_dest_context();
    }
}
