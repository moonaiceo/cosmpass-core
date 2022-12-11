use osmosis_std_derive::CosmwasmExt;
/// ValidatorPreference defines the message structure for
/// CreateValidatorSetPreference. It allows a user to set {val_addr, weight} in
/// state. If a user does not have a validator set preference list set, and has
/// staked, make their preference list default to their current staking
/// distribution.
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.validatorpreference.v1beta1.ValidatorPreference")]
pub struct ValidatorPreference {
    /// val_oper_address holds the validator address the user wants to delegate
    /// funds to.
    #[prost(string, tag = "1")]
    pub val_oper_address: ::prost::alloc::string::String,
    /// weight is decimal between 0 and 1, and they all sum to 1.
    #[prost(string, tag = "2")]
    pub weight: ::prost::alloc::string::String,
}
/// ValidatorSetPreferences defines a delegator's validator set preference.
/// It contains a list of (validator, percent_allocation) pairs.
/// The percent allocation are arranged in decimal notation from 0 to 1 and must
/// add up to 1.
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.validatorpreference.v1beta1.ValidatorSetPreferences")]
pub struct ValidatorSetPreferences {
    /// preference holds {valAddr, weight} for the user who created it.
    #[prost(message, repeated, tag = "2")]
    pub preferences: ::prost::alloc::vec::Vec<ValidatorPreference>,
}
/// MsgCreateValidatorSetPreference is a list that holds validator-set.
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.validatorpreference.v1beta1.MsgSetValidatorSetPreference")]
pub struct MsgSetValidatorSetPreference {
    /// delegator is the user who is trying to create a validator-set.
    #[prost(string, tag = "1")]
    pub delegator: ::prost::alloc::string::String,
    /// list of {valAddr, weight} to delegate to
    #[prost(message, repeated, tag = "2")]
    pub preferences: ::prost::alloc::vec::Vec<ValidatorPreference>,
}
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(
    type_url = "/osmosis.validatorpreference.v1beta1.MsgSetValidatorSetPreferenceResponse"
)]
pub struct MsgSetValidatorSetPreferenceResponse {}
/// MsgDelegateToValidatorSet allows users to delegate to an existing
/// validator-set
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.validatorpreference.v1beta1.MsgDelegateToValidatorSet")]
pub struct MsgDelegateToValidatorSet {
    /// delegator is the user who is trying to delegate.
    #[prost(string, tag = "1")]
    pub delegator: ::prost::alloc::string::String,
    /// the amount of tokens the user is trying to delegate.
    /// For ex: delegate 10osmo with validator-set {ValA -> 0.5, ValB -> 0.3, ValC
    /// -> 0.2} our staking logic would attempt to delegate 5osmo to A , 3osmo to
    /// B, 2osmo to C.
    #[prost(message, optional, tag = "2")]
    pub coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(
    type_url = "/osmosis.validatorpreference.v1beta1.MsgDelegateToValidatorSetResponse"
)]
pub struct MsgDelegateToValidatorSetResponse {}
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.validatorpreference.v1beta1.MsgUndelegateFromValidatorSet")]
pub struct MsgUndelegateFromValidatorSet {
    /// delegator is the user who is trying to undelegate.
    #[prost(string, tag = "1")]
    pub delegator: ::prost::alloc::string::String,
    /// the amount the user wants to undelegate
    /// For ex: Undelegate 10osmo with validator-set {ValA -> 0.5, ValB -> 0.3,
    /// ValC
    /// -> 0.2} our undelegate logic would attempt to undelegate 5osmo from A ,
    /// 3osmo from B, 2osmo from C
    #[prost(message, optional, tag = "3")]
    pub coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(
    type_url = "/osmosis.validatorpreference.v1beta1.MsgUndelegateFromValidatorSetResponse"
)]
pub struct MsgUndelegateFromValidatorSetResponse {}
/// MsgWithdrawDelegationRewards allows user to claim staking rewards from the
/// validator set.
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.validatorpreference.v1beta1.MsgWithdrawDelegationRewards")]
pub struct MsgWithdrawDelegationRewards {
    /// delegator is the user who is trying to claim staking rewards.
    #[prost(string, tag = "1")]
    pub delegator: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(
    type_url = "/osmosis.validatorpreference.v1beta1.MsgWithdrawDelegationRewardsResponse"
)]
pub struct MsgWithdrawDelegationRewardsResponse {}
/// Request type for UserValidatorPreferences.
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.validatorpreference.v1beta1.QueryUserValidatorPreferences")]
#[proto_query(
    path = "/osmosis.validatorpreference.v1beta1.Query/UserValidatorPreferences",
    response_type = QueryUserValidatorPreferenceResponse
)]
pub struct QueryUserValidatorPreferences {
    /// user account address
    #[prost(string, tag = "1")]
    pub user: ::prost::alloc::string::String,
}
/// Response type the QueryUserValidatorPreferences query request
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(
    type_url = "/osmosis.validatorpreference.v1beta1.QueryUserValidatorPreferenceResponse"
)]
pub struct QueryUserValidatorPreferenceResponse {
    #[prost(message, repeated, tag = "1")]
    pub preferences: ::prost::alloc::vec::Vec<ValidatorPreference>,
}
pub struct ValidatorpreferenceQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> ValidatorpreferenceQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn user_validator_preferences(
        &self,
        user: ::prost::alloc::string::String,
    ) -> Result<QueryUserValidatorPreferenceResponse, cosmwasm_std::StdError> {
        QueryUserValidatorPreferences { user }.query(self.querier)
    }
}
