#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Market;
use async_trait::async_trait;
use credit::CreditAbi;
use foundation::FoundationAbi;
use linera_sdk::{
    base::{Amount, ApplicationId, ChannelName, Destination, MessageId, Owner, WithContractAbi},
    Contract, ContractRuntime, ViewStateStorage,
};
use market::{InitializationArgument, MarketError, Message, Operation};

const SUBSCRIPTION_CHANNEL: &[u8] = b"subscriptions";

pub struct MarketContract {
    state: Market,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(MarketContract);

impl WithContractAbi for MarketContract {
    type Abi = market::MarketAbi;
}

#[async_trait]
impl Contract for MarketContract {
    type Error = MarketError;
    type Storage = ViewStateStorage<Self>;
    type State = Market;
    type Message = Message;

    async fn new(state: Market, runtime: ContractRuntime<Self>) -> Result<Self, Self::Error> {
        Ok(MarketContract { state, runtime })
    }

    fn state_mut(&mut self) -> &mut Self::State {
        &mut self.state
    }

    async fn initialize(
        &mut self,
        argument: Self::InitializationArgument,
    ) -> Result<(), Self::Error> {
        self.state.initialize_market(argument).await;
        Ok(())
    }

    async fn execute_operation(&mut self, operation: Self::Operation) -> Result<(), Self::Error> {
        let _ = self.require_authenticated_signer()?;
        match operation {
            Operation::MintNFT {
                collection_id,
                uri_index,
                price,
                name,
            } => self.on_op_mint_nft(collection_id, uri_index, price, name),
            Operation::BuyNFT {
                collection_id,
                token_id,
                credits,
            } => self.on_op_buy_nft(collection_id, token_id, credits),
            Operation::UpdateCreditsPerLinera { credits_per_linera } => {
                self.on_op_update_credits_per_linera(credits_per_linera)
            }
            Operation::UpdateNFTPrice {
                collection_id,
                token_id,
                price,
            } => self.on_op_update_nft_price(collection_id, token_id, price),
            Operation::OnSaleNFT {
                collection_id,
                token_id,
            } => self.on_op_on_sale_nft(collection_id, token_id),
            Operation::OffSaleNFT {
                collection_id,
                token_id,
            } => self.on_op_off_sale_nft(collection_id, token_id),
            Operation::SetAvatar {
                collection_id,
                token_id,
            } => self.on_op_set_avatar(collection_id, token_id),
            Operation::RequestSubscribe => self.on_op_request_subscribe(),
            Operation::CreateCollection {
                base_uri,
                price,
                name,
                uris,
                publisher,
            } => self.on_op_create_collection(base_uri, price, name, uris, publisher),
        }
    }

    async fn execute_message(&mut self, message: Self::Message) -> Result<(), Self::Error> {
        match message {
            Message::InitializationArgument { argument } => {
                self.on_msg_initialization_argument(argument).await
            }
            Message::CreateCollection {
                base_uri,
                price,
                name,
                uris,
                publisher,
            } => {
                self.on_msg_create_collection(base_uri, price, name, uris, publisher)
                    .await
            }
            Message::MintNFT {
                collection_id,
                uri_index,
                price,
                name,
            } => {
                self.on_msg_mint_nft(collection_id, uri_index, price, name)
                    .await
            }
            Message::BuyNFT {
                collection_id,
                token_id,
                credits,
            } => self.on_msg_buy_nft(collection_id, token_id, credits).await,
            Message::UpdateCreditsPerLinera { credits_per_linera } => {
                self.on_msg_update_credits_per_linera(credits_per_linera)
                    .await
            }
            Message::UpdateNFTPrice {
                collection_id,
                token_id,
                price,
            } => {
                self.on_msg_update_nft_price(collection_id, token_id, price)
                    .await
            }
            Message::OnSaleNFT {
                collection_id,
                token_id,
            } => self.on_msg_on_sale_nft(collection_id, token_id).await,
            Message::OffSaleNFT {
                collection_id,
                token_id,
            } => self.on_msg_off_sale_nft(collection_id, token_id).await,
            Message::SetAvatar {
                collection_id,
                token_id,
            } => self.on_msg_set_avatar(collection_id, token_id).await,
            Message::RequestSubscribe => self.on_msg_request_subscribe(),
        }
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
        // TODO: send initialization argument to subscriber
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
        // TODO: send initialization argument to subscriber
        Ok(())
    }

    async fn on_msg_initialization_argument(
        &mut self,
        argument: InitializationArgument,
    ) -> Result<(), MarketError> {
        self.state.initialize_market(argument).await;
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
        if self.require_message_id()?.chain_id != self.runtime.application_id().creation.chain_id {
            return Ok(());
        }
        let message_id = self.require_message_id()?;
        self.runtime.subscribe(
            message_id.chain_id,
            ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()),
        );
        Ok(())
    }
}
