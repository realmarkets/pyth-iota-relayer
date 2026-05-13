#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// Information about the transaction currently being executed.
/// This cannot be constructed by a transaction. It is a privileged object created by
/// the VM and passed in to the entrypoint of the transaction as `&mut TxContext`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TxContext {
    /// The address of the user that signed the current transaction
    pub sender: Address,
    /// Hash of the current transaction
    pub tx_hash: Vec<u8>,
    /// The current epoch number
    pub epoch: u64,
    /// Timestamp that the epoch started at
    pub epoch_timestamp_ms: u64,
    /// Counter recording the number of fresh id's created while executing
    /// this transaction. Always 0 at the start of a transaction
    pub ids_created: u64,
}
impl MoveType for TxContext {
    type Package = super::Package;
    const MODULE: &'static str = "tx_context";
    const NAME: &'static str = "TxContext";
}
impl MoveArg for TxContext {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentTxContext: PTBArgument {
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
impl ArgumentTxContext for TxContext {}
impl ArgumentTxContext for Argument {}
/// Return the address of the user that signed the current
/// transaction
///
/// Returns: `address`.
pub async fn sender(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "tx_context",
        "sender",
        Vec::new(),
        vec![],
    )
}
/// Return the transaction digest (hash of transaction inputs).
/// Please do not use as a source of randomness.
///
/// Returns: `&vector<u8>`.
pub async fn digest(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "tx_context",
        "digest",
        Vec::new(),
        vec![],
    )
}
/// Return the current epoch
///
/// Returns: `u64`.
pub async fn epoch(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "tx_context",
        "epoch",
        Vec::new(),
        vec![],
    )
}
/// Return the epoch start time as a unix timestamp in milliseconds.
///
/// Returns: `u64`.
pub async fn epoch_timestamp_ms(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "tx_context",
        "epoch_timestamp_ms",
        Vec::new(),
        vec![],
    )
}
/// Create an `address` that has not been used. As it is an object address, it will never
/// occur as the address for a user.
/// In other words, the generated address is a globally unique object ID.
///
/// Returns: `address`.
pub async fn fresh_object_address(b: &mut PtbBuilder) -> Argument {
    b.move_call(
        b.package_id::<super::Package>(),
        "tx_context",
        "fresh_object_address",
        Vec::new(),
        vec![],
    )
}
/// Read-only handle bound to a runtime package address. Use
/// via `super::Package::at(addr).<module>()` and chain
/// `<type>_tag()` to build a [`TypeTag`] without spinning up
/// a `PtbBuilder`.
pub struct ModuleAt {
    pub(crate) package: Address,
}
impl ModuleAt {
    pub fn txcontext_tag(&self) -> TypeTag {
        <TxContext as MoveType>::type_tag_at(self.package)
    }
}
