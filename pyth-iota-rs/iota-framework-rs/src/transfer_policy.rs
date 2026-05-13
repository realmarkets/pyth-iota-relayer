#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
pub const EPolicyNotSatisfied: u64 = 0u64;
pub const EIllegalRule: u64 = 1u64;
pub const EUnknownRequirement: u64 = 2u64;
pub const ERuleAlreadySet: u64 = 3u64;
pub const ENotOwner: u64 = 4u64;
pub const ENotEnough: u64 = 5u64;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransferRequest<T0: MoveType> {
    pub item: ID,
    pub paid: u64,
    pub from: ID,
    pub receipts: super::vec_set::VecSet<::move_stdlib_rs::type_name::TypeName>,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for TransferRequest<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "transfer_policy";
    const NAME: &'static str = "TransferRequest";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
impl<T0: MoveType + ::serde::Serialize> MoveArg for TransferRequest<T0> {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentTransferRequest<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType + ::serde::Serialize> ArgumentTransferRequest<T0>
for TransferRequest<T0> {}
impl<T0: MoveType> ArgumentTransferRequest<T0> for Argument {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransferPolicy<T0: MoveType> {
    pub id: UID,
    pub balance: super::balance::Balance<super::iota::IOTA>,
    pub rules: super::vec_set::VecSet<::move_stdlib_rs::type_name::TypeName>,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for TransferPolicy<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "transfer_policy";
    const NAME: &'static str = "TransferPolicy";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
pub trait ArgumentTransferPolicy<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType> ArgumentTransferPolicy<T0> for Argument {}
impl<T0: MoveType> ArgumentTransferPolicy<T0> for ObjectId {
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
impl<T0: MoveType> ArgumentTransferPolicy<T0> for ObjectReference {}
impl<T0: MoveType> ArgumentTransferPolicy<T0> for Shared<ObjectId> {}
impl<T0: MoveType> ArgumentTransferPolicy<T0> for SharedMut<ObjectId> {}
impl<T0: MoveType> ArgumentTransferPolicy<T0> for Receiving<ObjectId> {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransferPolicyCap<T0: MoveType> {
    pub id: UID,
    pub policy_id: ID,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for TransferPolicyCap<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "transfer_policy";
    const NAME: &'static str = "TransferPolicyCap";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
pub trait ArgumentTransferPolicyCap<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType> ArgumentTransferPolicyCap<T0> for Argument {}
impl<T0: MoveType> ArgumentTransferPolicyCap<T0> for ObjectId {
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
impl<T0: MoveType> ArgumentTransferPolicyCap<T0> for ObjectReference {}
impl<T0: MoveType> ArgumentTransferPolicyCap<T0> for Shared<ObjectId> {}
impl<T0: MoveType> ArgumentTransferPolicyCap<T0> for SharedMut<ObjectId> {}
impl<T0: MoveType> ArgumentTransferPolicyCap<T0> for Receiving<ObjectId> {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransferPolicyCreated<T0: MoveType> {
    pub id: ID,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for TransferPolicyCreated<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "transfer_policy";
    const NAME: &'static str = "TransferPolicyCreated";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
impl<T0: MoveType + ::serde::Serialize> MoveArg for TransferPolicyCreated<T0> {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentTransferPolicyCreated<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType + ::serde::Serialize> ArgumentTransferPolicyCreated<T0>
for TransferPolicyCreated<T0> {}
impl<T0: MoveType> ArgumentTransferPolicyCreated<T0> for Argument {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransferPolicyDestroyed<T0: MoveType> {
    pub id: ID,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for TransferPolicyDestroyed<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "transfer_policy";
    const NAME: &'static str = "TransferPolicyDestroyed";
    fn type_params(addrs: &impl PackageAddrs) -> Vec<TypeTag> {
        vec![< T0 as MoveType > ::type_tag(addrs)]
    }
}
impl<T0: MoveType + ::serde::Serialize> MoveArg for TransferPolicyDestroyed<T0> {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentTransferPolicyDestroyed<T0: MoveType>: PTBArgument {
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
impl<T0: MoveType + ::serde::Serialize> ArgumentTransferPolicyDestroyed<T0>
for TransferPolicyDestroyed<T0> {}
impl<T0: MoveType> ArgumentTransferPolicyDestroyed<T0> for Argument {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuleKey<T0: MoveType> {
    pub dummy_field: bool,
    #[serde(skip)]
    pub _phantom_t0: PhantomData<T0>,
}
impl<T0: MoveType> MoveType for RuleKey<T0> {
    type Package = super::Package;
    const MODULE: &'static str = "transfer_policy";
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
/// Returns: `0x2::transfer_policy::TransferRequest<T0>`.
pub async fn new_request<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl PureID,
    arg1: impl PureU64,
    arg2: impl PureID,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "transfer_policy",
        "new_request",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    )
}
/// Returns: `(0x2::transfer_policy::TransferPolicy<T0>, 0x2::transfer_policy::TransferPolicyCap<T0>)`.
pub async fn new<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::package::ArgumentPublisher,
) -> (Argument, Argument) {
    let a0 = arg0.into_argument_ref(b).await;
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "transfer_policy",
            "new",
            vec![< T0 as MoveType > ::type_tag(b)],
            vec![a0],
            2u16,
        );
    (__r[0], __r[1])
}
pub async fn default<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl super::package::ArgumentPublisher,
) {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "transfer_policy",
        "default",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    );
}
/// Returns: `0x2::coin::Coin<0x2::iota::IOTA>`.
pub async fn withdraw<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTransferPolicy<T0>,
    arg1: impl ArgumentTransferPolicyCap<T0>,
    arg2: impl PureOption<u64>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "transfer_policy",
        "withdraw",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    )
}
/// Returns: `0x2::coin::Coin<0x2::iota::IOTA>`.
pub async fn destroy_and_withdraw<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTransferPolicy<T0>,
    arg1: impl ArgumentTransferPolicyCap<T0>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "transfer_policy",
        "destroy_and_withdraw",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Returns: `(0x2::object::ID, u64, 0x2::object::ID)`.
pub async fn confirm_request<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTransferPolicy<T0>,
    arg1: impl ArgumentTransferRequest<T0>,
) -> (Argument, Argument, Argument) {
    let a0 = arg0.into_argument_ref(b).await;
    let a1 = arg1.into_argument(b).await;
    let __r = b
        .move_call_n(
            b.package_id::<super::Package>(),
            "transfer_policy",
            "confirm_request",
            vec![< T0 as MoveType > ::type_tag(b)],
            vec![a0, a1],
            3u16,
        );
    (__r[0], __r[1], __r[2])
}
pub async fn add_rule<T0: MoveType, T1: MoveType, T2: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl ArgumentTransferPolicy<T0>,
    arg2: impl ArgumentTransferPolicyCap<T0>,
    arg3: impl ArgumentObject<()>,
) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument_mut(b).await;
    let a2 = arg2.into_argument_ref(b).await;
    let a3 = arg3.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "transfer_policy",
        "add_rule",
        vec![
            < T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b), < T2 as
            MoveType > ::type_tag(b)
        ],
        vec![a0, a1, a2, a3],
    );
}
/// Returns: `&T2`.
pub async fn get_rule<T0: MoveType, T1: MoveType, T2: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl ArgumentTransferPolicy<T0>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "transfer_policy",
        "get_rule",
        vec![
            < T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b), < T2 as
            MoveType > ::type_tag(b)
        ],
        vec![a0, a1],
    )
}
pub async fn add_to_balance<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl ArgumentTransferPolicy<T0>,
    arg2: impl super::coin::ArgumentCoin<super::iota::IOTA>,
) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument_mut(b).await;
    let a2 = arg2.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "transfer_policy",
        "add_to_balance",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1, a2],
    );
}
pub async fn add_receipt<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentObject<()>,
    arg1: impl ArgumentTransferRequest<T0>,
) {
    let a0 = arg0.into_argument(b).await;
    let a1 = arg1.into_argument_mut(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "transfer_policy",
        "add_receipt",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    );
}
/// Returns: `bool`.
pub async fn has_rule<T0: MoveType, T1: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTransferPolicy<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "transfer_policy",
        "has_rule",
        vec![< T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
pub async fn remove_rule<T0: MoveType, T1: MoveType, T2: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTransferPolicy<T0>,
    arg1: impl ArgumentTransferPolicyCap<T0>,
) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "transfer_policy",
        "remove_rule",
        vec![
            < T0 as MoveType > ::type_tag(b), < T1 as MoveType > ::type_tag(b), < T2 as
            MoveType > ::type_tag(b)
        ],
        vec![a0, a1],
    );
}
/// Returns: `&0x2::object::UID`.
pub async fn uid<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTransferPolicy<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "transfer_policy",
        "uid",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Returns: `&mut 0x2::object::UID`.
pub async fn uid_mut_as_owner<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTransferPolicy<T0>,
    arg1: impl ArgumentTransferPolicyCap<T0>,
) -> Argument {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "transfer_policy",
        "uid_mut_as_owner",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0, a1],
    )
}
/// Returns: `&0x2::vec_set::VecSet<0x1::type_name::TypeName>`.
pub async fn rules<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTransferPolicy<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "transfer_policy",
        "rules",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Returns: `0x2::object::ID`.
pub async fn item<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTransferRequest<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "transfer_policy",
        "item",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Returns: `u64`.
pub async fn paid<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTransferRequest<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "transfer_policy",
        "paid",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
/// Returns: `0x2::object::ID`.
pub async fn from<T0: MoveType>(
    b: &mut PtbBuilder,
    arg0: impl ArgumentTransferRequest<T0>,
) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "transfer_policy",
        "from",
        vec![< T0 as MoveType > ::type_tag(b)],
        vec![a0],
    )
}
