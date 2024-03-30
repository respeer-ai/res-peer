use std::collections::HashSet;

use async_graphql::SimpleObject;
use foundation::{FoundationError, InitializationArgument, RewardType};
use linera_sdk::{
    base::{Amount, Owner},
    views::{linera_views, MapView, RegisterView, RootView, ViewStorageContext},
};

#[derive(RootView, SimpleObject)]
#[view(context = "ViewStorageContext")]
pub struct Foundation {
    pub foundation_balance: RegisterView<Amount>,
    pub review_reward_percent: RegisterView<u8>,
    pub review_reward_balance: RegisterView<Amount>,
    pub review_reward_factor: RegisterView<u8>,
    pub author_reward_percent: RegisterView<u8>,
    pub author_reward_balance: RegisterView<Amount>,
    pub author_reward_factor: RegisterView<u8>,
    pub activity_reward_percent: RegisterView<u8>,
    pub activity_reward_balance: RegisterView<Amount>,
    pub activity_lock_funds: MapView<u64, Amount>,
    pub user_balances: MapView<Owner, Amount>,
}

#[allow(dead_code)]
impl Foundation {
    pub(crate) async fn initialize_foundation(
        &mut self,
        state: InitializationArgument,
    ) -> Result<(), FoundationError> {
        if state.review_reward_percent + state.author_reward_percent + state.activity_reward_percent
            > 100
        {
            return Err(FoundationError::InvalidPercent);
        }
        self.review_reward_percent.set(state.review_reward_percent);
        self.author_reward_percent.set(state.author_reward_percent);
        self.activity_reward_percent
            .set(state.activity_reward_percent);
        self.review_reward_factor.set(state.review_reward_factor);
        self.author_reward_factor.set(state.author_reward_factor);
        Ok(())
    }

    pub(crate) async fn initialization_argument(
        &self,
    ) -> Result<InitializationArgument, FoundationError> {
        Ok(InitializationArgument {
            review_reward_percent: *self.review_reward_percent.get(),
            review_reward_factor: *self.review_reward_factor.get(),
            author_reward_percent: *self.author_reward_percent.get(),
            author_reward_factor: *self.author_reward_factor.get(),
            activity_reward_percent: *self.activity_reward_percent.get(),
        })
    }

    // When transaction happen, transaction fee will be deposited here
    // It'll be separated to different reward balance according to reward ratio
    pub(crate) async fn deposit(
        &mut self,
        from: Owner,
        amount: Amount,
    ) -> Result<(), FoundationError> {
        let from_amount = match self.user_balances.get(&from).await? {
            Some(amount) => amount,
            _ => return Err(FoundationError::InsufficientBalance),
        };
        if from_amount.lt(&amount) {
            return Err(FoundationError::InsufficientBalance);
        }
        self.user_balances
            .insert(&from, from_amount.saturating_sub(amount))?;

        let review_amount = amount.try_mul(*self.review_reward_percent.get() as u128)?;
        let review_amount = review_amount.saturating_div(Amount::from_attos(100 as u128));
        let review_amount = self
            .review_reward_balance
            .get()
            .try_add(Amount::from_attos(review_amount))?;

        let author_amount = amount.try_mul(*self.author_reward_percent.get() as u128)?;
        let author_amount = author_amount.saturating_div(Amount::from_attos(100 as u128));
        let author_amount = self
            .author_reward_balance
            .get()
            .try_add(Amount::from_attos(author_amount))?;

        let activity_amount = amount.try_mul(*self.activity_reward_percent.get() as u128)?;
        let activity_amount = activity_amount.saturating_div(Amount::from_attos(100 as u128));
        let activity_amount = self
            .activity_reward_balance
            .get()
            .try_add(Amount::from_attos(activity_amount))?;

        self.review_reward_balance.set(review_amount);
        self.author_reward_balance.set(author_amount);
        self.activity_reward_balance.set(activity_amount);

        let _amount = amount.try_sub(review_amount)?;
        let _amount = _amount.try_sub(author_amount)?;
        let _amount = _amount.try_sub(activity_amount)?;
        let _amount = self.foundation_balance.get().try_add(_amount)?;

        self.foundation_balance.set(_amount);
        Ok(())
    }

    pub(crate) async fn transfer(
        &mut self,
        from: Owner,
        to: Owner,
        amount: Amount,
    ) -> Result<(), FoundationError> {
        if from == to {
            return Err(FoundationError::InvalidAccount);
        }
        let from_amount = match self.user_balances.get(&from).await? {
            Some(balance) => balance,
            None => return Err(FoundationError::InsufficientBalance),
        };
        if from_amount.lt(&amount) {
            return Err(FoundationError::InsufficientBalance);
        }
        let to_amount = match self.user_balances.get(&to).await? {
            Some(balance) => balance,
            None => Amount::from_attos(0),
        };
        self.user_balances
            .insert(&from, from_amount.saturating_sub(amount))?;
        self.user_balances
            .insert(&to, to_amount.saturating_add(amount))?;
        Ok(())
    }

    pub(crate) async fn user_deposit(
        &mut self,
        owner: Owner,
        amount: Amount,
    ) -> Result<(), FoundationError> {
        let balance = self
            .user_balances
            .get(&owner)
            .await?
            .unwrap_or(Amount::ZERO);
        self.user_balances
            .insert(&owner, balance.saturating_add(amount))?;
        Ok(())
    }

    pub(crate) async fn reward_user(
        &mut self,
        user: Owner,
        amount: Amount,
    ) -> Result<(), FoundationError> {
        let amount = match self.user_balances.get(&user).await? {
            Some(user_balance) => user_balance.try_add(amount)?,
            None => amount,
        };
        self.user_balances.insert(&user, amount)?;
        Ok(())
    }

    pub(crate) async fn reward_activity(
        &mut self,
        reward_user: Owner,
        activity_id: u64,
    ) -> Result<(), FoundationError> {
        // TODO: check who can reward activity here
        let balance = match self.activity_lock_funds.get(&activity_id).await? {
            Some(balance) => balance,
            None => return Err(FoundationError::InsufficientBalance),
        };
        let amount = Amount::from_tokens(50);
        if balance.le(&amount) {
            return Err(FoundationError::InsufficientBalance);
        }
        // TODO: distribute reward from locked activity balance
        self.reward_user(reward_user, amount).await?;
        self.activity_lock_funds
            .insert(&activity_id, balance.saturating_sub(amount))?;
        Ok(())
    }

    pub(crate) async fn reward_author(
        &mut self,
        reward_user: Owner,
    ) -> Result<(), FoundationError> {
        let balance = self.author_reward_balance.get().clone();
        let amount = Amount::from_attos(
            balance
                .try_mul(*self.author_reward_factor.get() as u128)?
                .saturating_div(Amount::from_attos(100)),
        );
        self.reward_user(reward_user, amount).await?;
        self.author_reward_balance
            .set(balance.saturating_sub(amount));
        Ok(())
    }

    pub(crate) async fn reward_reviewer(
        &mut self,
        reward_user: Owner,
    ) -> Result<(), FoundationError> {
        let balance = self.review_reward_balance.get().clone();
        let amount = balance
            .try_mul(*self.review_reward_factor.get() as u128)?
            .saturating_div(Amount::from_attos(100));
        self.reward_user(reward_user, Amount::from_attos(amount))
            .await?;
        self.review_reward_balance
            .set(balance.saturating_sub(Amount::from_attos(amount)));
        Ok(())
    }

    // Reward user of different type with different balance
    pub(crate) async fn reward(
        &mut self,
        reward_user: Owner,
        reward_type: RewardType,
        activity_id: Option<u64>,
    ) -> Result<(), FoundationError> {
        match reward_type {
            RewardType::Activity => {
                self.reward_activity(reward_user, activity_id.unwrap())
                    .await
            }
            RewardType::Publish => self.reward_author(reward_user).await,
            RewardType::Review => self.reward_reviewer(reward_user).await,
        }
    }

    pub(crate) async fn lock(
        &mut self,
        activity_id: u64,
        amount: Amount,
    ) -> Result<(), FoundationError> {
        // TODO: check application balance
        let locked = match self.activity_lock_funds.get(&activity_id).await? {
            Some(amount) => amount,
            None => Amount::ZERO,
        };
        // TODO: check who can lock funds for activity here
        let amount = locked.try_add(amount)?;
        self.activity_lock_funds.insert(&activity_id, amount)?;
        Ok(())
    }

    pub(crate) async fn balance(&self, owner: Owner) -> Result<Amount, FoundationError> {
        Ok(self
            .user_balances
            .get(&owner)
            .await
            .unwrap()
            .unwrap_or_default())
    }

    pub(crate) async fn spend_activity_funds(
        &mut self,
        activity_id: u64,
        reward_amount: Amount,
    ) -> Result<(), FoundationError> {
        match self.activity_lock_funds.get(&activity_id).await {
            Ok(Some(funds)) => {
                if funds < reward_amount {
                    return Err(FoundationError::InsufficientBalance);
                } else {
                    self.activity_lock_funds
                        .insert(&activity_id, funds.saturating_sub(reward_amount))?;
                    Ok(())
                }
            }
            Ok(None) => return Err(FoundationError::InvalidActivityFunds),
            Err(err) => return Err(FoundationError::ViewError(err)),
        }
    }

    pub(crate) async fn distribute_activity_rewards(
        &mut self,
        winner_user: Owner,
        voter_users: HashSet<Owner>,
        reward_amount: Amount,
        voter_reward_percent: u8,
    ) -> Result<(), FoundationError> {
        if voter_reward_percent > 100 {
            return Err(FoundationError::InvalidPercent);
        }
        let balance = match self.user_balances.get(&winner_user).await {
            Ok(Some(balance)) => balance,
            _ => Amount::ZERO,
        };
        let winner_amount = Amount::from_attos(
            reward_amount
                .saturating_mul(100 - voter_reward_percent as u128)
                .saturating_div(Amount::from_attos(100)),
        );
        let balance = balance.saturating_add(winner_amount);
        self.user_balances.insert(&winner_user, balance)?;

        let mut user_balance_total = Amount::ZERO;
        for user in voter_users.clone().into_iter() {
            let balance = match self.user_balances.get(&user).await {
                Ok(Some(balance)) => balance,
                _ => Amount::ZERO,
            };
            user_balance_total = user_balance_total.saturating_add(balance);
        }

        let voter_amount = reward_amount.saturating_sub(winner_amount);
        for user in voter_users.into_iter() {
            let balance = match self.user_balances.get(&user).await {
                Ok(Some(balance)) => balance,
                _ => Amount::ZERO,
            };
            if balance == Amount::ZERO {
                continue;
            }
            let percent = balance
                .saturating_mul(100 as u128)
                .saturating_div(user_balance_total);
            let balance = balance.saturating_add(Amount::from_attos(
                voter_amount
                    .saturating_div(Amount::from_attos(100))
                    .saturating_mul(percent),
            ));
            self.user_balances.insert(&user, balance)?;
        }
        Ok(())
    }
}
