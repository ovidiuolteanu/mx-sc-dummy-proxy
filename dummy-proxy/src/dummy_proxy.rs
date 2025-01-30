multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TypeAbi, TopEncode, TopDecode, PartialEq, Copy, Clone, Debug)]
pub enum CallType {
    Sync,
    Async,
    Promise,
    TransferExecute
}

#[multiversx_sc::module]
pub trait DummyProxyModule {
    /// Simply calls the specified endpoint on given contract address
    #[endpoint(callEndpoint)]
    fn call_endpoint(
        &self,
        call_type: CallType,
        contract_address: ManagedAddress,
        function_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let gas_left = self.blockchain().get_gas_left();
        if call_type == CallType::Sync {
            self.tx()
                .to(&contract_address)
                .gas(gas_left)
                .raw_call(function_name)
                .arguments_raw(args.to_arg_buffer())
                .sync_call()
        }
        else if call_type == CallType::Async {
            self.tx()
                .to(&contract_address)
                .raw_call(function_name)
                .arguments_raw(args.to_arg_buffer())
                .async_call_and_exit()
        }
        else if call_type == CallType::Promise {
            self.tx()
                .to(&contract_address)
                .gas(gas_left)
                .raw_call(function_name)
                .arguments_raw(args.to_arg_buffer())
                .register_promise()
        }
        else if call_type == CallType::TransferExecute {
            self.tx()
                .to(&contract_address)
                .gas(gas_left)
                .raw_call(function_name)
                .arguments_raw(args.to_arg_buffer())
                .transfer_execute()
        }
    }

    /// Calls the specified endpoint on given contract address while it also transfers user given tokens with the call
    #[endpoint(callInternalTransferEndpoint)]
    fn call_int_transfer_endpoint(
        &self,
        call_type: CallType,
        token_id: TokenIdentifier,
        nonce: u64,
        amount: BigUint,
        contract_address: ManagedAddress,
        function_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let gas_left = self.blockchain().get_gas_left();
        if call_type == CallType::Sync {
            self.tx()
                .to(&contract_address)
                .raw_call(function_name)
                .arguments_raw(args.to_arg_buffer())
                .payment((token_id, nonce, amount))
                .sync_call()
        }
        else if call_type == CallType::Async {
            self.tx()
                .to(&contract_address)
                .raw_call(function_name)
                .arguments_raw(args.to_arg_buffer())
                .payment((token_id, nonce, amount))
                .async_call_and_exit()
        }
        else if call_type == CallType::Promise {
            self.tx()
                .to(&contract_address)
                .gas(gas_left)
                .raw_call(function_name)
                .arguments_raw(args.to_arg_buffer())
                .payment((token_id, nonce, amount))
                .register_promise()
        }
        else if call_type == CallType::TransferExecute {
            self.tx()
                .to(&contract_address)
                .raw_call(function_name)
                .arguments_raw(args.to_arg_buffer())
                .payment((token_id, nonce, amount))
                .transfer_execute()
        }
    }

    /// Calls the specified endpoint on given contract address while it also transfers specified tokens owned by the dummy proxy with the call
    #[payable("*")]
    #[endpoint(callTransferEndpoint)]
    fn call_transfer_endpoint(
        &self,
        call_type: CallType,
        contract_address: ManagedAddress,
        function_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let payments = self.call_value().all_esdt_transfers().clone_value();
        let gas_left = self.blockchain().get_gas_left();

        if call_type == CallType::Sync {
            self.tx()
                .to(&contract_address)
                .raw_call(function_name)
                .arguments_raw(args.to_arg_buffer())
                .payment(payments)
                .sync_call()
        }
        else if call_type == CallType::Async {
            self.tx()
                .to(&contract_address)
                .raw_call(function_name)
                .arguments_raw(args.to_arg_buffer())
                .payment(payments)
                .async_call_and_exit()
        }
        else if call_type == CallType::Promise {
            self.tx()
                .to(&contract_address)
                .gas(gas_left)
                .raw_call(function_name)
                .arguments_raw(args.to_arg_buffer())
                .payment(payments)
                .register_promise()
        }
        else if call_type == CallType::TransferExecute {
            self.tx()
                .to(&contract_address)
                .raw_call(function_name)
                .arguments_raw(args.to_arg_buffer())
                .payment(payments)
                .transfer_execute()
        }
    }

    /// Calls the specified endpoint on given contract address while it also transfers user given tokens and internally owned tokens with the call
    #[payable("*")]
    #[endpoint(callHybridTransferEndpoint)]
    fn call_hybrid_transfer_endpoint(
        &self,
        call_type: CallType,
        token_id: TokenIdentifier,
        nonce: u64,
        amount: BigUint,
        contract_address: ManagedAddress,
        function_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let mut payments = self.call_value().all_esdt_transfers().clone_value();
        let internal_payment = EsdtTokenPayment::new(token_id, nonce, amount);
        payments.push(internal_payment);
        let gas_left = self.blockchain().get_gas_left();
        if call_type == CallType::Sync {
            self.tx()
                .to(&contract_address)
                .raw_call(function_name)
                .arguments_raw(args.to_arg_buffer())
                .payment(payments)
                .sync_call()
        }
        else if call_type == CallType::Async {
            self.tx()
                .to(&contract_address)
                .raw_call(function_name)
                .arguments_raw(args.to_arg_buffer())
                .payment(payments)
                .async_call_and_exit()
        }
        else if call_type == CallType::Promise {
            self.tx()
                .to(&contract_address)
                .gas(gas_left)
                .raw_call(function_name)
                .arguments_raw(args.to_arg_buffer())
                .payment(payments)
                .register_promise()
        }
        else if call_type == CallType::TransferExecute {
            self.tx()
                .to(&contract_address)
                .raw_call(function_name)
                .arguments_raw(args.to_arg_buffer())
                .payment(payments)
                .transfer_execute()
        }
    }
}
