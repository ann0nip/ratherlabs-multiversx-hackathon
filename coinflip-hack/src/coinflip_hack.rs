#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait CoinflipHacker {
    #[init]
    fn init(&self, coinflip_contract_address: ManagedAddress, donation_receiver: ManagedAddress) {
        self.set_coinflip_contract_address(coinflip_contract_address);
        self.set_donation_receiver(donation_receiver);
    }

    #[endpoint]
    fn hack_coinflip(&self) {
        let coinflip_address = self.get_coinflip_contract_address();
        let _donation_receiver = self.get_donation_receiver(); // Prefijo _ para suprimir la advertencia

        let block_nonce = self.blockchain().get_block_nonce();

        // Debug del valor inicial
        self.nonce_event(block_nonce);


        self.will_call_event();

        // Crear proxy para la llamada
        let mut proxy = self.coinflip_proxy(coinflip_address);

        // Llamada al coinflip
        proxy
            .coinflip()
            .async_call_and_exit();
    }

    #[endpoint]
    fn donate_after_win(&self) {
        let coinflip_address = self.get_coinflip_contract_address();
        let donation_receiver = self.get_donation_receiver();

        let mut proxy = self.coinflip_proxy(coinflip_address);

        proxy
            .donate_bumps(donation_receiver)
            .async_call_and_exit();
    }

    #[endpoint]
    fn set_donation_wallet(&self, new_receiver: ManagedAddress) {
        self.set_donation_receiver(new_receiver);
    }

    #[endpoint]
    fn verify_contract_connection(&self) -> (ManagedAddress, ManagedAddress) {
        (self.get_coinflip_contract_address(), self.get_donation_receiver())
    }

    #[storage_mapper("coinflip_contract_address")]
    fn coinflip_contract_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("donation_receiver")]
    fn donation_receiver(&self) -> SingleValueMapper<ManagedAddress>;

    fn get_coinflip_contract_address(&self) -> ManagedAddress {
        self.coinflip_contract_address().get()
    }

    fn set_coinflip_contract_address(&self, address: ManagedAddress) {
        self.coinflip_contract_address().set(address);
    }

    fn get_donation_receiver(&self) -> ManagedAddress {
        self.donation_receiver().get()
    }

    fn set_donation_receiver(&self, address: ManagedAddress) {
        self.donation_receiver().set(address);
    }

    #[event("nonce")]
    fn nonce_event(&self, #[indexed] block_nonce: u64);

    #[event("calculation")]
    fn calculation_event(
        &self,
        #[indexed] val1: u64,
        #[indexed] val2: u64,
        #[indexed] will_win: bool
    );

    #[event("willCall")]
    fn will_call_event(&self);

    #[event("skipCall")]
    fn skip_call_event(&self);

    #[proxy]
    fn coinflip_proxy(&self, coinflip_address: ManagedAddress) -> coinflip_proxy::Proxy<Self::Api>;
}

mod coinflip_proxy {
    multiversx_sc::imports!();

    #[multiversx_sc::proxy]
    pub trait CoinflipProxy {
        #[endpoint(coinflip)]
        fn coinflip(&self) -> bool;

        #[endpoint(donateBumps)]
        fn donate_bumps(&self, receiver: ManagedAddress);
    }
}