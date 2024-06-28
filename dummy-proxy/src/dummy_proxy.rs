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
        self.tx()
            .to(&contract_address)
            .gas(gas_left)
            .raw_call(function_name)
            .arguments_raw(args.to_arg_buffer())
            .sync_call()
    }

    #[endpoint(callInternalTransferEndpoint)]
    fn call_int_transfer_endpoint(
        &self,
        token_id: TokenIdentifier,
        nonce: u64,
        amount: BigUint,
        contract_address: ManagedAddress,
        function_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        self.tx()
            .to(&contract_address)
            .raw_call(function_name)
            .arguments_raw(args.to_arg_buffer())
            .payment((token_id, nonce, amount))
            .sync_call()
    }

    #[payable("*")]
    #[endpoint(callTransferEndpoint)]
    fn call_transfer_endpoint(
        &self,
        contract_address: ManagedAddress,
        function_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let payments = self.call_value().all_esdt_transfers().clone_value();

        self.tx()
            .to(&contract_address)
            .raw_call(function_name)
            .arguments_raw(args.to_arg_buffer())
            .payment(payments)
            .sync_call()
    }
}
