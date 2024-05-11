#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Market;
use credit::CreditAbi;
use foundation::FoundationAbi;
use linera_sdk::{
    base::{Amount, ApplicationId, ChannelName, Destination, MessageId, Owner, WithContractAbi},
    views::{RootView, View, ViewStorageContext},
    Contract, ContractRuntime,
};
use market::{InstantiationArgument, MarketError, MarketParameters, Message, Operation};

const SUBSCRIPTION_CHANNEL: &[u8] = b"subscriptions";

pub struct MarketContract {
    state: Market,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(MarketContract);

impl WithContractAbi for MarketContract {
    type Abi = market::MarketAbi;
}

impl Contract for MarketContract {
    type Message = Message;
    type InstantiationArgument = InstantiationArgument;
    type Parameters = MarketParameters;

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = Market::load(ViewStorageContext::from(runtime.key_value_store()))
            .await
            .expect("Failed to load state");
        MarketContract { state, runtime }
    }

    async fn instantiate(&mut self, argument: Self::InstantiationArgument) {
        self.runtime.application_parameters();
        self.state.instantiate_market(argument).await;
    }

    async fn execute_operation(&mut self, operation: Self::Operation) -> Self::Response {
        let _ = self
            .require_authenticated_signer()
            .expect("Failed OP: check authenticated signer");
        match operation {
            Operation::MintNFT {
                collection_id,
                uri_index,
                price,
                name,
            } => self
                .on_op_mint_nft(collection_id, uri_index, price, name)
                .expect("Failed OP: mint nft"),
            Operation::BuyNFT {
                collection_id,
                token_id,
                credits,
            } => self
                .on_op_buy_nft(collection_id, token_id, credits)
                .expect("Failed OP: buy nft"),
            Operation::UpdateCreditsPerLinera { credits_per_linera } => self
                .on_op_update_credits_per_linera(credits_per_linera)
                .expect("Failed OP: update credits per linera"),
            Operation::UpdateNFTPrice {
                collection_id,
                token_id,
                price,
            } => self
                .on_op_update_nft_price(collection_id, token_id, price)
                .expect("Failed OP: update nft price"),
            Operation::OnSaleNFT {
                collection_id,
                token_id,
            } => self
                .on_op_on_sale_nft(collection_id, token_id)
                .expect("Failed OP: on sale nft"),
            Operation::OffSaleNFT {
                collection_id,
                token_id,
            } => self
                .on_op_off_sale_nft(collection_id, token_id)
                .expect("Failed OP: off sale nft"),
            Operation::SetAvatar {
                collection_id,
                token_id,
            } => self
                .on_op_set_avatar(collection_id, token_id)
                .expect("Failed OP: set avatar"),
            Operation::RequestSubscribe => self
                .on_op_request_subscribe()
                .expect("Failed OP: subscribe"),
            Operation::CreateCollection {
                base_uri,
                price,
                name,
                uris,
                publisher,
            } => self
                .on_op_create_collection(base_uri, price, name, uris, publisher)
                .expect("Failed OP: create collection"),
        }
    }

    async fn execute_message(&mut self, message: Self::Message) {
        match message {
            Message::InstantiationArgument { argument } => self
                .on_msg_instantiation_argument(argument)
                .await
                .expect("Failed MSG: instantiation argument"),
            Message::CreateCollection {
                base_uri,
                price,
                name,
                uris,
                publisher,
            } => self
                .on_msg_create_collection(base_uri, price, name, uris, publisher)
                .await
                .expect("Failed MSG: create collection"),
            Message::MintNFT {
                collection_id,
                uri_index,
                price,
                name,
            } => self
                .on_msg_mint_nft(collection_id, uri_index, price, name)
                .await
                .expect("Failed MSG: mint nft"),
            Message::BuyNFT {
                collection_id,
                token_id,
                credits,
            } => self
                .on_msg_buy_nft(collection_id, token_id, credits)
                .await
                .expect("Failed MSG: buy NFT"),
            Message::UpdateCreditsPerLinera { credits_per_linera } => self
                .on_msg_update_credits_per_linera(credits_per_linera)
                .await
                .expect("Failed MSG: update credits per linera"),
            Message::UpdateNFTPrice {
                collection_id,
                token_id,
                price,
            } => self
                .on_msg_update_nft_price(collection_id, token_id, price)
                .await
                .expect("Failed MSG: update nft price"),
            Message::OnSaleNFT {
                collection_id,
                token_id,
            } => self
                .on_msg_on_sale_nft(collection_id, token_id)
                .await
                .expect("Failed MSG: on sale nft"),
            Message::OffSaleNFT {
                collection_id,
                token_id,
            } => self
                .on_msg_off_sale_nft(collection_id, token_id)
                .await
                .expect("Failed MSG: off sale nft"),
            Message::SetAvatar {
                collection_id,
                token_id,
            } => self
                .on_msg_set_avatar(collection_id, token_id)
                .await
                .expect("Failed MSG: set avatar"),
            Message::RequestSubscribe => self
                .on_msg_request_subscribe()
                .expect("Failed MSG: subscribe"),
        }
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}

impl MarketContract {
    fn credit_app_id(&mut self) -> ApplicationId<CreditAbi> {
        self.runtime.application_parameters().credit_app_id
    }

    fn foundation_app_id(&mut self) -> ApplicationId<FoundationAbi> {
        self.runtime.application_parameters().foundation_app_id
    }

    async fn transfer_credits(
        &mut self,
        from: Owner,
        to: Owner,
        amount: Amount,
    ) -> Result<(), MarketError> {
        let call = credit::Operation::Transfer { from, to, amount };
        let credit_app_id = self.credit_app_id();
        let _ = self.runtime.call_application(true, credit_app_id, &call);
        Ok(())
    }

    async fn deposit_commission(&mut self, from: Owner, amount: Amount) -> Result<(), MarketError> {
        let call = foundation::Operation::Deposit { from, amount };
        let foundation_app_id = self.foundation_app_id();
        let _ = self
            .runtime
            .call_application(true, foundation_app_id, &call);
        Ok(())
    }

    async fn transfer_tokens(
        &mut self,
        from: Owner,
        to: Owner,
        amount: Amount,
    ) -> Result<(), MarketError> {
        let call = foundation::Operation::Transfer { from, to, amount };
        let foundation_app_id = self.foundation_app_id();
        let _ = self
            .runtime
            .call_application(true, foundation_app_id, &call);
        Ok(())
    }

    fn require_message_id(&mut self) -> Result<MessageId, MarketError> {
        match self.runtime.message_id() {
            Some(message_id) => Ok(message_id),
            None => Err(MarketError::InvalidMessageId),
        }
    }

    fn require_authenticated_signer(&mut self) -> Result<Owner, MarketError> {
        match self.runtime.authenticated_signer() {
            Some(owner) => Ok(owner),
            None => Err(MarketError::InvalidSigner),
        }
    }

    fn on_op_mint_nft(
        &mut self,
        collection_id: u64,
        uri_index: u16,
        price: Option<Amount>,
        name: String,
    ) -> Result<(), MarketError> {
        self.runtime
            .prepare_message(Message::MintNFT {
                collection_id,
                uri_index,
                price,
                name,
            })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(())
    }

    fn on_op_buy_nft(
        &mut self,
        collection_id: u64,
        token_id: u16,
        credits: Amount,
    ) -> Result<(), MarketError> {
        self.runtime
            .prepare_message(Message::BuyNFT {
                collection_id,
                token_id,
                credits,
            })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(())
    }

    fn on_op_update_credits_per_linera(
        &mut self,
        credits_per_linera: Amount,
    ) -> Result<(), MarketError> {
        self.runtime
            .prepare_message(Message::UpdateCreditsPerLinera { credits_per_linera })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(())
    }

    fn on_op_update_nft_price(
        &mut self,
        collection_id: u64,
        token_id: Option<u16>,
        price: Amount,
    ) -> Result<(), MarketError> {
        self.runtime
            .prepare_message(Message::UpdateNFTPrice {
                collection_id,
                token_id,
                price,
            })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(())
    }

    fn on_op_on_sale_nft(&mut self, collection_id: u64, token_id: u16) -> Result<(), MarketError> {
        self.runtime
            .prepare_message(Message::OnSaleNFT {
                collection_id,
                token_id,
            })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(())
    }

    fn on_op_off_sale_nft(&mut self, collection_id: u64, token_id: u16) -> Result<(), MarketError> {
        self.runtime
            .prepare_message(Message::OffSaleNFT {
                collection_id,
                token_id,
            })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(())
    }

    fn on_op_set_avatar(&mut self, collection_id: u64, token_id: u16) -> Result<(), MarketError> {
        self.runtime
            .prepare_message(Message::SetAvatar {
                collection_id,
                token_id,
            })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        Ok(())
    }

    fn on_op_request_subscribe(&mut self) -> Result<(), MarketError> {
        self.runtime
            .prepare_message(Message::RequestSubscribe)
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        // TODO: send instantiation argument to subscriber
        Ok(())
    }

    fn on_op_create_collection(
        &mut self,
        base_uri: String,
        price: Option<Amount>,
        name: String,
        uris: Vec<String>,
        publisher: Owner,
    ) -> Result<(), MarketError> {
        self.runtime
            .prepare_message(Message::CreateCollection {
                base_uri,
                price,
                name,
                uris,
                publisher,
            })
            .with_authentication()
            .send_to(self.runtime.application_id().creation.chain_id);
        // TODO: send instantiation argument to subscriber
        Ok(())
    }

    async fn on_msg_instantiation_argument(
        &mut self,
        argument: InstantiationArgument,
    ) -> Result<(), MarketError> {
        self.state.instantiate_market(argument).await;
        Ok(())
    }

    async fn on_msg_create_collection(
        &mut self,
        base_uri: String,
        price: Option<Amount>,
        name: String,
        uris: Vec<String>,
        publisher: Owner,
    ) -> Result<(), MarketError> {
        self.state
            .create_collection(
                publisher,
                base_uri.clone(),
                price,
                name.clone(),
                uris.clone(),
                self.runtime.system_time(),
            )
            .await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::CreateCollection {
                base_uri,
                price,
                name,
                uris,
                publisher,
            })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_mint_nft(
        &mut self,
        collection_id: u64,
        uri_index: u16,
        price: Option<Amount>,
        name: String,
    ) -> Result<(), MarketError> {
        let owner = self.require_authenticated_signer()?;
        self.state
            .mint_nft(
                owner,
                collection_id,
                uri_index,
                price,
                name.clone(),
                self.runtime.system_time(),
            )
            .await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::MintNFT {
                collection_id,
                uri_index,
                price,
                name,
            })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_buy_nft(
        &mut self,
        collection_id: u64,
        token_id: u16,
        credits: Amount,
    ) -> Result<(), MarketError> {
        let buyer = self.require_authenticated_signer()?;
        if self.runtime.chain_id() == self.runtime.application_id().creation.chain_id {
            let owner = self.state.nft_owner(collection_id, token_id).await?;
            let price = self.state.nft_price(collection_id, token_id).await?;
            let fee = self.state.trading_fee(price).await?;
            let discount = self.state.credits_to_tokens(credits).await?;
            self.transfer_credits(buyer, owner, credits).await?;
            self.transfer_tokens(
                buyer,
                owner,
                price.saturating_sub(fee).saturating_sub(discount),
            )
            .await?;
            self.deposit_commission(buyer, fee).await?;
        }
        self.state.buy_nft(buyer, collection_id, token_id).await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::BuyNFT {
                collection_id,
                token_id,
                credits,
            })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_update_credits_per_linera(
        &mut self,
        credits_per_linera: Amount,
    ) -> Result<(), MarketError> {
        // TODO: Only creation user can update
        self.state.credits_per_linera.set(credits_per_linera);
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::UpdateCreditsPerLinera { credits_per_linera })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_update_nft_price(
        &mut self,
        collection_id: u64,
        token_id: Option<u16>,
        price: Amount,
    ) -> Result<(), MarketError> {
        let setter = self.require_authenticated_signer()?;
        self.state
            .update_nft_price(setter, collection_id, token_id, price)
            .await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::UpdateNFTPrice {
                collection_id,
                token_id,
                price,
            })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_on_sale_nft(
        &mut self,
        collection_id: u64,
        token_id: u16,
    ) -> Result<(), MarketError> {
        let setter = self.require_authenticated_signer()?;
        self.state
            .on_sale_nft(setter, collection_id, token_id)
            .await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::OnSaleNFT {
                collection_id,
                token_id,
            })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_off_sale_nft(
        &mut self,
        collection_id: u64,
        token_id: u16,
    ) -> Result<(), MarketError> {
        let setter = self.require_authenticated_signer()?;
        self.state
            .off_sale_nft(setter, collection_id, token_id)
            .await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::OffSaleNFT {
                collection_id,
                token_id,
            })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    async fn on_msg_set_avatar(
        &mut self,
        collection_id: u64,
        token_id: u16,
    ) -> Result<(), MarketError> {
        let setter = self.require_authenticated_signer()?;
        self.state
            .set_avatar(setter, collection_id, token_id)
            .await?;
        if self.runtime.chain_id() != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let dest = Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
        self.runtime
            .prepare_message(Message::SetAvatar {
                collection_id,
                token_id,
            })
            .with_authentication()
            .send_to(dest);
        Ok(())
    }

    fn on_msg_request_subscribe(&mut self) -> Result<(), MarketError> {
        let message_id = self.require_message_id()?;
        // The subscribe message must be from another chain
        if message_id.chain_id == self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        self.runtime.subscribe(
            message_id.chain_id,
            ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()),
        );
        Ok(())
    }
}
