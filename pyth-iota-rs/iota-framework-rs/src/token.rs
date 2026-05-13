#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! The Token module which implements a Closed Loop Token with a configurable
//! policy. The policy is defined by a set of rules that must be satisfied for
//! an action to be performed on the token.
//!
//! The module is designed to be used with a `TreasuryCap` to allow for minting
//! and burning of the `Token`s. And can act as a replacement / extension or a
//! companion to existing open-loop (`Coin`) systems.
//!
//! ```
//! Module:      iota::balance       iota::coin             iota::token
//! Main type:   Balance<T>         Coin<T>               Token<T>
//! Capability:  Supply<T>  <---->  TreasuryCap<T> <----> TreasuryCap<T>
//! Abilities:   store              key + store           key
//! ```
//!
//! The Token system allows for fine-grained control over the actions performed
//! on the token. And hence it is highly suitable for applications that require
//! control over the currency which a simple open-loop system can't provide.
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// The action is not allowed (defined) in the policy.
pub const EUnknownAction: u64 = 0u64;
/// The rule was not approved.
pub const ENotApproved: u64 = 1u64;
/// Trying to perform an admin action with a wrong cap.
pub const ENotAuthorized: u64 = 2u64;
/// The balance is too low to perform the action.
pub const EBalanceTooLow: u64 = 3u64;
/// The balance is not zero.
pub const ENotZero: u64 = 4u64;
/// The balance is not zero when trying to confirm with `TransferPolicyCap`.
pub const ECantConsumeBalance: u64 = 5u64;
/// Rule is trying to access a missing config (with type).
pub const ENoConfig: u64 = 6u64;
/// Using `confirm_request_mut` without `spent_balance`. Immutable version
/// of the function must be used instead.
pub const EUseImmutableConfirm: u64 = 7u64;
/// A Tag for the `spend` action.
pub const SPEND: &[u8] = &[115u8, 112u8, 101u8, 110u8, 100u8];
/// A Tag for the `transfer` action.
pub const TRANSFER: &[u8] = &[116u8, 114u8, 97u8, 110u8, 115u8, 102u8, 101u8, 114u8];
/// A Tag for the `to_coin` action.
pub const TO_COIN: &[u8] = &[116u8, 111u8, 95u8, 99u8, 111u8, 105u8, 110u8];
/// A Tag for the `from_coin` action.
pub const FROM_COIN: &[u8] = &[
    102u8, 114u8, 111u8, 109u8, 95u8, 99u8, 111u8, 105u8, 110u8,
];
/// A single `Token` with `Balance` inside. Can only be owned by an address,
/// and actions performed on it must be confirmed in a matching `TokenPolicy`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Token<T0: MoveType> {
    pub id: UID,
    /// The Balance of the `Token`.
    pub balance: super::balance::Balance<T0>,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for Token<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "token";
    const NAME: &'static str = "Token";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
pub trait ArgumentToken<T0: MoveType>: PTBArgument {
    #[allow(async_fn_in_trait)]
    async fn into_argument(self, b: &mut PtbBuilder) -> Argument
    where
        Self: Sized,
    {
        b.inner.apply_argument(self)
    }
    #[allow(async_fn_in_trait)]
    async fn into_argument_ref(self, b: &mut PtbBuilder) -> Argument
    where
        Self: Sized,
    {
        self.into_argument(b).await
    }
    #[allow(async_fn_in_trait)]
    async fn into_argument_mut(self, b: &mut PtbBuilder) -> Argument
    where
        Self: Sized,
    {
        self.into_argument(b).await
    }
}
impl<T0: MoveType> ArgumentToken<T0> for Argument {}
impl<T0: MoveType> ArgumentToken<T0> for ObjectId {
    async fn into_argument(self, b: &mut PtbBuilder) -> Argument {
        b.resolve_object(self).await
    }
    async fn into_argument_ref(self, b: &mut PtbBuilder) -> Argument {
        b.resolve_object_shared(self, false).await
    }
    async fn into_argument_mut(self, b: &mut PtbBuilder) -> Argument {
        b.resolve_object_shared(self, true).await
    }
}
impl<T0: MoveType> ArgumentToken<T0> for ObjectReference {}
impl<T0: MoveType> ArgumentToken<T0> for Shared<ObjectId> {}
impl<T0: MoveType> ArgumentToken<T0> for SharedMut<ObjectId> {}
impl<T0: MoveType> ArgumentToken<T0> for Receiving<ObjectId> {}
/// A Capability that manages a single `TokenPolicy` specified in the `for`
/// field. Created together with `TokenPolicy` in the `new` function.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenPolicyCap<T0: MoveType> {
    pub id: UID,
    pub r#for: ID,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for TokenPolicyCap<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "token";
    const NAME: &'static str = "TokenPolicyCap";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
pub trait ArgumentTokenPolicyCap<T0: MoveType>: PTBArgument {
    #[allow(async_fn_in_trait)]
    async fn into_argument(self, b: &mut PtbBuilder) -> Argument
    where
        Self: Sized,
    {
        b.inner.apply_argument(self)
    }
    #[allow(async_fn_in_trait)]
    async fn into_argument_ref(self, b: &mut PtbBuilder) -> Argument
    where
        Self: Sized,
    {
        self.into_argument(b).await
    }
    #[allow(async_fn_in_trait)]
    async fn into_argument_mut(self, b: &mut PtbBuilder) -> Argument
    where
        Self: Sized,
    {
        self.into_argument(b).await
    }
}
impl<T0: MoveType> ArgumentTokenPolicyCap<T0> for Argument {}
impl<T0: MoveType> ArgumentTokenPolicyCap<T0> for ObjectId {
    async fn into_argument(self, b: &mut PtbBuilder) -> Argument {
        b.resolve_object(self).await
    }
    async fn into_argument_ref(self, b: &mut PtbBuilder) -> Argument {
        b.resolve_object_shared(self, false).await
    }
    async fn into_argument_mut(self, b: &mut PtbBuilder) -> Argument {
        b.resolve_object_shared(self, true).await
    }
}
impl<T0: MoveType> ArgumentTokenPolicyCap<T0> for ObjectReference {}
impl<T0: MoveType> ArgumentTokenPolicyCap<T0> for Shared<ObjectId> {}
impl<T0: MoveType> ArgumentTokenPolicyCap<T0> for SharedMut<ObjectId> {}
impl<T0: MoveType> ArgumentTokenPolicyCap<T0> for Receiving<ObjectId> {}
/// `TokenPolicy` represents a set of rules that define what actions can be
/// performed on a `Token` and which `Rules` must be satisfied for the
/// action to succeed.
///
/// - For the sake of availability, `TokenPolicy` is a `key`-only object.
/// - Each `TokenPolicy` is managed by a matching `TokenPolicyCap`.
/// - For an action to become available, there needs to be a record in the
/// `rules` VecMap. To allow an action to be performed freely, there's an
/// `allow` function that can be called by the `TokenPolicyCap` owner.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenPolicy<T0: MoveType> {
    pub id: UID,
    /// The balance that is effectively spent by the user on the "spend"
    /// action. However, actual decrease of the supply can only be done by
    /// the `TreasuryCap` owner when `flush` is called.
    ///
    /// This balance is effectively spent and cannot be accessed by anyone
    /// but the `TreasuryCap` owner.
    pub spent_balance: super::balance::Balance<T0>,
    /// The set of rules that define what actions can be performed on the
    /// token. For each "action" there's a set of Rules that must be
    /// satisfied for the `ActionRequest` to be confirmed.
    pub rules: super::vec_map::VecMap<
        String,
        super::vec_set::VecSet<::move_stdlib_rs::type_name::TypeName>,
    >,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for TokenPolicy<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "token";
    const NAME: &'static str = "TokenPolicy";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
pub trait ArgumentTokenPolicy<T0: MoveType>: PTBArgument {
    #[allow(async_fn_in_trait)]
    async fn into_argument(self, b: &mut PtbBuilder) -> Argument
    where
        Self: Sized,
    {
        b.inner.apply_argument(self)
    }
    #[allow(async_fn_in_trait)]
    async fn into_argument_ref(self, b: &mut PtbBuilder) -> Argument
    where
        Self: Sized,
    {
        self.into_argument(b).await
    }
    #[allow(async_fn_in_trait)]
    async fn into_argument_mut(self, b: &mut PtbBuilder) -> Argument
    where
        Self: Sized,
    {
        self.into_argument(b).await
    }
}
impl<T0: MoveType> ArgumentTokenPolicy<T0> for Argument {}
impl<T0: MoveType> ArgumentTokenPolicy<T0> for ObjectId {
    async fn into_argument(self, b: &mut PtbBuilder) -> Argument {
        b.resolve_object(self).await
    }
    async fn into_argument_ref(self, b: &mut PtbBuilder) -> Argument {
        b.resolve_object_shared(self, false).await
    }
    async fn into_argument_mut(self, b: &mut PtbBuilder) -> Argument {
        b.resolve_object_shared(self, true).await
    }
}
impl<T0: MoveType> ArgumentTokenPolicy<T0> for ObjectReference {}
impl<T0: MoveType> ArgumentTokenPolicy<T0> for Shared<ObjectId> {}
impl<T0: MoveType> ArgumentTokenPolicy<T0> for SharedMut<ObjectId> {}
impl<T0: MoveType> ArgumentTokenPolicy<T0> for Receiving<ObjectId> {}
/// A request to perform an "Action" on a token. Stores the information
/// about the action to be performed and must be consumed by the `confirm_request`
/// or `confirm_request_mut` functions when the Rules are satisfied.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActionRequest<T0: MoveType> {
    /// Name of the Action to look up in the Policy. Name can be one of the
    /// default actions: `transfer`, `spend`, `to_coin`, `from_coin` or a
    /// custom action.
    pub name: String,
    /// Amount is present in all of the txs
    pub amount: u64,
    /// Sender is a permanent field always
    pub sender: Address,
    /// Recipient is only available in `transfer` action.
    pub recipient: Option<Address>,
    /// The balance to be "spent" in the `TokenPolicy`, only available
    /// in the `spend` action.
    pub spent_balance: Option<super::balance::Balance<T0>>,
    /// Collected approvals (stamps) from completed `Rules`. They're matched
    /// against `TokenPolicy.rules` to determine if the request can be
    /// confirmed.
    pub approvals: super::vec_set::VecSet<::move_stdlib_rs::type_name::TypeName>,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for ActionRequest<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "token";
    const NAME: &'static str = "ActionRequest";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
impl<T0: MoveType + ::serde::Serialize> MoveArg for ActionRequest<T0> {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentActionRequest<T0: MoveType>: PTBArgument {
    #[allow(async_fn_in_trait)]
    async fn into_argument(self, b: &mut PtbBuilder) -> Argument
    where
        Self: Sized,
    {
        b.inner.apply_argument(self)
    }
    #[allow(async_fn_in_trait)]
    async fn into_argument_ref(self, b: &mut PtbBuilder) -> Argument
    where
        Self: Sized,
    {
        self.into_argument(b).await
    }
    #[allow(async_fn_in_trait)]
    async fn into_argument_mut(self, b: &mut PtbBuilder) -> Argument
    where
        Self: Sized,
    {
        self.into_argument(b).await
    }
}
impl<T0: MoveType + ::serde::Serialize> ArgumentActionRequest<T0> for ActionRequest<T0> {}
impl<T0: MoveType> ArgumentActionRequest<T0> for Argument {}
/// Dynamic field key for the `TokenPolicy` to store the `Config` for a
/// specific action `Rule`. There can be only one configuration per
/// `Rule` per `TokenPolicy`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuleKey<T0: MoveType> {
    pub is_protected: bool,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for RuleKey<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "token";
    const NAME: &'static str = "RuleKey";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
impl<T0: MoveType + ::serde::Serialize> MoveArg for RuleKey<T0> {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentRuleKey<T0: MoveType>: PTBArgument {
    #[allow(async_fn_in_trait)]
    async fn into_argument(self, b: &mut PtbBuilder) -> Argument
    where
        Self: Sized,
    {
        b.inner.apply_argument(self)
    }
    #[allow(async_fn_in_trait)]
    async fn into_argument_ref(self, b: &mut PtbBuilder) -> Argument
    where
        Self: Sized,
    {
        self.into_argument(b).await
    }
    #[allow(async_fn_in_trait)]
    async fn into_argument_mut(self, b: &mut PtbBuilder) -> Argument
    where
        Self: Sized,
    {
        self.into_argument(b).await
    }
}
impl<T0: MoveType + ::serde::Serialize> ArgumentRuleKey<T0> for RuleKey<T0> {}
impl<T0: MoveType> ArgumentRuleKey<T0> for Argument {}
/// An event emitted when a `TokenPolicy` is created and shared. Because
/// `TokenPolicy` can only be shared (and potentially frozen in the future),
/// we emit this event in the `share_policy` function and mark it as mutable.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenPolicyCreated<T0: MoveType> {
    /// ID of the `TokenPolicy` that was created.
    pub id: ID,
    /// Whether the `TokenPolicy` is "shared" (mutable) or "frozen"
    /// (immutable) - TBD.
    pub is_mutable: bool,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for TokenPolicyCreated<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "token";
    const NAME: &'static str = "TokenPolicyCreated";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
impl<T0: MoveType + ::serde::Serialize> MoveArg for TokenPolicyCreated<T0> {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentTokenPolicyCreated<T0: MoveType>: PTBArgument {
    #[allow(async_fn_in_trait)]
    async fn into_argument(self, b: &mut PtbBuilder) -> Argument
    where
        Self: Sized,
    {
        b.inner.apply_argument(self)
    }
    #[allow(async_fn_in_trait)]
    async fn into_argument_ref(self, b: &mut PtbBuilder) -> Argument
    where
        Self: Sized,
    {
        self.into_argument(b).await
    }
    #[allow(async_fn_in_trait)]
    async fn into_argument_mut(self, b: &mut PtbBuilder) -> Argument
    where
        Self: Sized,
    {
        self.into_argument(b).await
    }
}
impl<T0: MoveType + ::serde::Serialize> ArgumentTokenPolicyCreated<T0>
for TokenPolicyCreated<T0> {}
impl<T0: MoveType> ArgumentTokenPolicyCreated<T0> for Argument {}
/// Create a new `TokenPolicy` and a matching `TokenPolicyCap`.
/// The `TokenPolicy` must then be shared using the `share_policy` method.
///
/// `TreasuryCap` guarantees full ownership over the currency, and is unique,
/// hence it is safe to use it for authorization.
///
/// Returns: `(0x2::token::TokenPolicy<T0>, 0x2::token::TokenPolicyCap<T0>)`.
pub async fn new_policy<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::coin::ArgumentTreasuryCap<T0>,
) -> (Argument, Argument) {
    let a0 = arg0.into_argument_ref(b).await;
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "token",
            "new_policy",
            vec![< T0 as MoveType > ::type_tag(b)],
            vec![a0],
            2u16,
        );
    (__r[0], __r[1])
}
/// Share the `TokenPolicy`. Due to `key`-only restriction, it must be
/// shared after initialization.
pub async fn share_policy<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTokenPolicy<T0>,
) {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "share_policy",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    );
}
/// Transfer a `Token` to a `recipient`. Creates an `ActionRequest` for the
/// "transfer" action. The `ActionRequest` contains the `recipient` field
/// to be used in verification.
///
/// Returns: `0x2::token::ActionRequest<T0>`.
pub async fn transfer<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentToken<T0>,
    arg1: impl PureAddress,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "transfer",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Spend a `Token` by unwrapping it and storing the `Balance` in the
/// `ActionRequest` for the "spend" action. The `ActionRequest` contains
/// the `spent_balance` field to be used in verification.
///
/// Spend action requires `confirm_request_mut` to be called to confirm the
/// request and join the spent balance with the `TokenPolicy.spent_balance`.
///
/// Returns: `0x2::token::ActionRequest<T0>`.
pub async fn spend<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentToken<T0>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "spend",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Convert `Token` into an open `Coin`. Creates an `ActionRequest` for the
/// "to_coin" action.
///
/// Returns: `(0x2::coin::Coin<T0>, 0x2::token::ActionRequest<T0>)`.
pub async fn to_coin<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentToken<T0>,
) -> (Argument, Argument) {
    let a0 = arg0.into_argument(b).await;
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "token",
            "to_coin",
            vec![< T0 as MoveType > ::type_tag(b)],
            vec![a0],
            2u16,
        );
    (__r[0], __r[1])
}
/// Convert an open `Coin` into a `Token`. Creates an `ActionRequest` for
/// the "from_coin" action.
///
/// Returns: `(0x2::token::Token<T0>, 0x2::token::ActionRequest<T0>)`.
pub async fn from_coin<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::coin::ArgumentCoin<T0>,
) -> (Argument, Argument) {
    let a0 = arg0.into_argument(b).await;
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "token",
            "from_coin",
            vec![< T0 as MoveType > ::type_tag(b)],
            vec![a0],
            2u16,
        );
    (__r[0], __r[1])
}
/// Join two `Token`s into one, always available.
pub async fn join<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentToken<T0>,
    arg1: impl ArgumentToken<T0>,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "join",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    );
}
/// Split a `Token` with `amount`.
/// Aborts if the `Token.balance` is lower than `amount`.
///
/// Returns: `0x2::token::Token<T0>`.
pub async fn split<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentToken<T0>,
    arg1: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "split",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Create a zero `Token`.
///
/// Returns: `0x2::token::Token<T0>`.
pub async fn zero<T0: MoveType>(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "zero",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![],
    )
}
/// Destroy an empty `Token`, fails if the balance is non-zero.
/// Aborts if the `Token.balance` is not zero.
pub async fn destroy_zero<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentToken<T0>,
) {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "destroy_zero",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    );
}
/// Transfer the `Token` to the transaction sender.
pub async fn keep<T0: MoveType>(b: &mut PtbBuilder, arg0: impl ArgumentToken<T0>) {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "keep",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    );
}
/// Create a new `ActionRequest`.
/// Publicly available method to allow for custom actions.
///
/// Returns: `0x2::token::ActionRequest<T0>`.
pub async fn new_request<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl PureString,
    arg1: impl PureU64,
    arg2: impl PureOption<Address>,
    arg3: impl PureOption<super::balance::Balance<T0>>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    let a3 = arg3.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "new_request",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2, a3],
    )
}
/// Confirm the request against the `TokenPolicy` and return the parameters
/// of the request: (Name, Amount, Sender, Recipient).
///
/// Cannot be used for `spend` and similar actions that deliver `spent_balance`
/// to the `TokenPolicy`. For those actions use `confirm_request_mut`.
///
/// Aborts if:
/// - the action is not allowed (missing record in `rules`)
/// - action contains `spent_balance` (use `confirm_request_mut`)
/// - the `ActionRequest` does not meet the `TokenPolicy` rules for the action
///
/// Returns: `(0x1::string::String, u64, address, 0x1::option::Option<address>)`.
pub async fn confirm_request<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTokenPolicy<T0>,
    arg1: impl ArgumentActionRequest<T0>,
) -> (Argument, Argument, Argument, Argument) {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "token",
            "confirm_request",
            vec![< T0 as MoveType > ::type_tag(b)],
            vec![a0, a1],
            4u16,
        );
    (__r[0], __r[1], __r[2], __r[3])
}
/// Confirm the request against the `TokenPolicy` and return the parameters
/// of the request: (Name, Amount, Sender, Recipient).
///
/// Unlike `confirm_request` this function requires mutable access to the
/// `TokenPolicy` and must be used on `spend` action. After dealing with the
/// spent balance it calls `confirm_request` internally.
///
/// See `confirm_request` for the list of abort conditions.
///
/// Returns: `(0x1::string::String, u64, address, 0x1::option::Option<address>)`.
pub async fn confirm_request_mut<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTokenPolicy<T0>,
    arg1: impl ArgumentActionRequest<T0>,
) -> (Argument, Argument, Argument, Argument) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "token",
            "confirm_request_mut",
            vec![< T0 as MoveType > ::type_tag(b)],
            vec![a0, a1],
            4u16,
        );
    (__r[0], __r[1], __r[2], __r[3])
}
/// Confirm an `ActionRequest` as the `TokenPolicyCap` owner. This function
/// allows `TokenPolicy` owner to perform Capability-gated actions ignoring
/// the ruleset specified in the `TokenPolicy`.
///
/// Aborts if request contains `spent_balance` due to inability of the
/// `TokenPolicyCap` to decrease supply. For scenarios like this a
/// `TreasuryCap` is required (see `confirm_with_treasury_cap`).
///
/// Returns: `(0x1::string::String, u64, address, 0x1::option::Option<address>)`.
pub async fn confirm_with_policy_cap<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTokenPolicyCap<T0>,
    arg1: impl ArgumentActionRequest<T0>,
) -> (Argument, Argument, Argument, Argument) {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "token",
            "confirm_with_policy_cap",
            vec![< T0 as MoveType > ::type_tag(b)],
            vec![a0, a1],
            4u16,
        );
    (__r[0], __r[1], __r[2], __r[3])
}
/// Confirm an `ActionRequest` as the `TreasuryCap` owner. This function
/// allows `TreasuryCap` owner to perform Capability-gated actions ignoring
/// the ruleset specified in the `TokenPolicy`.
///
/// Unlike `confirm_with_policy_cap` this function allows `spent_balance`
/// to be consumed, decreasing the `total_supply` of the `Token`.
///
/// Returns: `(0x1::string::String, u64, address, 0x1::option::Option<address>)`.
pub async fn confirm_with_treasury_cap<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::coin::ArgumentTreasuryCap<T0>,
    arg1: impl ArgumentActionRequest<T0>,
) -> (Argument, Argument, Argument, Argument) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "token",
            "confirm_with_treasury_cap",
            vec![< T0 as MoveType > ::type_tag(b)],
            vec![a0, a1],
            4u16,
        );
    (__r[0], __r[1], __r[2], __r[3])
}
/// Add an "approval" to the `ActionRequest` by providing a Witness.
/// Intended to be used by Rules to add their own approvals, however, can
/// be used to add arbitrary approvals to the request (not only the ones
/// required by the `TokenPolicy`).
pub async fn add_approval<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl ArgumentActionRequest<T0>,
) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "add_approval",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    );
}
/// Add a `Config` for a `Rule` in the `TokenPolicy`. Rule configuration is
/// independent from the `TokenPolicy.rules` and needs to be managed by the
/// Rule itself. Configuration is stored per `Rule` and not per `Rule` per
/// `Action` to allow reuse in different actions.
///
/// - Rule witness guarantees that the `Config` is approved by the Rule.
/// - `TokenPolicyCap` guarantees that the `Config` setup is initiated by
/// the `TokenPolicy` owner.
pub async fn add_rule_config<T0: MoveType, T1: MoveType, T2: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl ArgumentTokenPolicy<T0>,
    arg2: impl ArgumentTokenPolicyCap<T0>,
    arg3: impl ArgumentObject<()>,
) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument_mut(b).await;
    let a2 = arg2.into_argument_ref(b).await;
    let a3 = arg3.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "add_rule_config",
        vec![
            < T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b), < T2 as
            MoveType > ::type_tag(b)
        ],
        vec![a0, a1, a2, a3],
    );
}
/// Get a `Config` for a `Rule` in the `TokenPolicy`. Requires `Rule`
/// witness, hence can only be read by the `Rule` itself. This requirement
/// guarantees safety of the stored `Config` and allows for simpler dynamic
/// field management inside the Rule Config (custom type keys are not needed
/// for access gating).
///
/// Aborts if the Config is not present.
///
/// Returns: `&T2`.
pub async fn rule_config<T0: MoveType, T1: MoveType, T2: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl ArgumentTokenPolicy<T0>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "rule_config",
        vec![
            < T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b), < T2 as
            MoveType > ::type_tag(b)
        ],
        vec![a0, a1],
    )
}
/// Get mutable access to the `Config` for a `Rule` in the `TokenPolicy`.
/// Requires `Rule` witness, hence can only be read by the `Rule` itself,
/// as well as `TokenPolicyCap` to guarantee that the `TokenPolicy` owner
/// is the one who initiated the `Config` modification.
///
/// Aborts if:
/// - the Config is not present
/// - `TokenPolicyCap` is not matching the `TokenPolicy`
///
/// Returns: `&mut T2`.
pub async fn rule_config_mut<T0: MoveType, T1: MoveType, T2: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl ArgumentTokenPolicy<T0>,
    arg2: impl ArgumentTokenPolicyCap<T0>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument_mut(b).await;
    let a2 = arg2.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "rule_config_mut",
        vec![
            < T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b), < T2 as
            MoveType > ::type_tag(b)
        ],
        vec![a0, a1, a2],
    )
}
/// Remove a `Config` for a `Rule` in the `TokenPolicy`.
/// Unlike the `add_rule_config`, this function does not require a `Rule`
/// witness, hence can be performed by the `TokenPolicy` owner on their own.
///
/// Rules need to make sure that the `Config` is present when performing
/// verification of the `ActionRequest`.
///
/// Aborts if:
/// - the Config is not present
/// - `TokenPolicyCap` is not matching the `TokenPolicy`
///
/// Returns: `T2`.
pub async fn remove_rule_config<T0: MoveType, T1: MoveType, T2: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTokenPolicy<T0>,
    arg1: impl ArgumentTokenPolicyCap<T0>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "remove_rule_config",
        vec![
            < T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b), < T2 as
            MoveType > ::type_tag(b)
        ],
        vec![a0, a1],
    )
}
/// Check if a config for a `Rule` is set in the `TokenPolicy` without
/// checking the type of the `Config`.
///
/// Returns: `bool`.
pub async fn has_rule_config<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTokenPolicy<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "has_rule_config",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Check if a `Config` for a `Rule` is set in the `TokenPolicy` and that
/// it matches the type provided.
///
/// Returns: `bool`.
pub async fn has_rule_config_with_type<T0: MoveType, T1: MoveType, T2: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTokenPolicy<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "has_rule_config_with_type",
        vec![
            < T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b), < T2 as
            MoveType > ::type_tag(b)
        ],
        vec![a0],
    )
}
/// Allows an `action` to be performed on the `Token` freely by adding an
/// empty set of `Rules` for the `action`.
///
/// Aborts if the `TokenPolicyCap` is not matching the `TokenPolicy`.
pub async fn allow<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTokenPolicy<T0>,
    arg1: impl ArgumentTokenPolicyCap<T0>,
    arg2: impl PureString,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "allow",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Completely disallows an `action` on the `Token` by removing the record
/// from the `TokenPolicy.rules`.
///
/// Aborts if the `TokenPolicyCap` is not matching the `TokenPolicy`.
pub async fn disallow<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTokenPolicy<T0>,
    arg1: impl ArgumentTokenPolicyCap<T0>,
    arg2: impl PureString,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "disallow",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Adds a Rule for an action with `name` in the `TokenPolicy`.
///
/// Aborts if the `TokenPolicyCap` is not matching the `TokenPolicy`.
pub async fn add_rule_for_action<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTokenPolicy<T0>,
    arg1: impl ArgumentTokenPolicyCap<T0>,
    arg2: impl PureString,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "add_rule_for_action",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Removes a rule for an action with `name` in the `TokenPolicy`. Returns
/// the config object to be handled by the sender (or a Rule itself).
///
/// Aborts if the `TokenPolicyCap` is not matching the `TokenPolicy`.
pub async fn remove_rule_for_action<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTokenPolicy<T0>,
    arg1: impl ArgumentTokenPolicyCap<T0>,
    arg2: impl PureString,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "remove_rule_for_action",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
/// Mint a `Token` with a given `amount` using the `TreasuryCap`.
///
/// Returns: `0x2::token::Token<T0>`.
pub async fn mint<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::coin::ArgumentTreasuryCap<T0>,
    arg1: impl PureU64,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "mint",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Burn a `Token` using the `TreasuryCap`.
pub async fn burn<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::coin::ArgumentTreasuryCap<T0>,
    arg1: impl ArgumentToken<T0>,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "burn",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    );
}
/// Flush the `TokenPolicy.spent_balance` into the `TreasuryCap`. This
/// action is only available to the `TreasuryCap` owner.
///
/// Returns: `u64`.
pub async fn flush<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTokenPolicy<T0>,
    arg1: impl super::coin::ArgumentTreasuryCap<T0>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "flush",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Check whether an action is present in the rules VecMap.
///
/// Returns: `bool`.
pub async fn is_allowed<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTokenPolicy<T0>,
    arg1: impl PureString,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "is_allowed",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Returns the rules required for a specific action.
///
/// Returns: `0x2::vec_set::VecSet<0x1::type_name::TypeName>`.
pub async fn rules<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTokenPolicy<T0>,
    arg1: impl PureString,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "rules",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Returns the `spent_balance` of the `TokenPolicy`.
///
/// Returns: `u64`.
pub async fn spent_balance<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTokenPolicy<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "spent_balance",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Returns the `balance` of the `Token`.
///
/// Returns: `u64`.
pub async fn value<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentToken<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "value",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Name of the Transfer action.
///
/// Returns: `0x1::string::String`.
pub async fn transfer_action(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "transfer_action",
        Vec::new(),
        vec![],
    )
}
/// Name of the `Spend` action.
///
/// Returns: `0x1::string::String`.
pub async fn spend_action(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "spend_action",
        Vec::new(),
        vec![],
    )
}
/// Name of the `ToCoin` action.
///
/// Returns: `0x1::string::String`.
pub async fn to_coin_action(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "to_coin_action",
        Vec::new(),
        vec![],
    )
}
/// Name of the `FromCoin` action.
///
/// Returns: `0x1::string::String`.
pub async fn from_coin_action(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "from_coin_action",
        Vec::new(),
        vec![],
    )
}
/// The Action in the `ActionRequest`.
///
/// Returns: `0x1::string::String`.
pub async fn action<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentActionRequest<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "action",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Returns: `u64`.
pub async fn amount<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentActionRequest<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "amount",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Returns: `address`.
pub async fn sender<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentActionRequest<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "sender",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Returns: `0x1::option::Option<address>`.
pub async fn recipient<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentActionRequest<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "recipient",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Approvals of the `ActionRequest`.
///
/// Returns: `0x2::vec_set::VecSet<0x1::type_name::TypeName>`.
pub async fn approvals<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentActionRequest<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "approvals",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Burned balance of the `ActionRequest`.
///
/// Returns: `0x1::option::Option<u64>`.
pub async fn spent<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentActionRequest<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "token",
        "spent",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
