#![allow(unused_imports, non_snake_case, non_upper_case_globals)]
//! URL: standard Uniform Resource Locator string
use move_bindgen_runtime::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
/// Standard Uniform Resource Locator (URL) string.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Url {
    pub url: String,
}
impl MoveType for Url {
    type Package = super::Package;
    const MODULE: &'static str = "url";
    const NAME: &'static str = "Url";
}
impl MoveArg for Url {
    fn pure_bytes(self) -> PureBytes {
        PureBytes(::bcs::to_bytes(&self).expect("bcs serialization failed"))
    }
}
pub trait ArgumentUrl: PTBArgument {
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
impl ArgumentUrl for Url {}
impl ArgumentUrl for Argument {}
/// Create a `Url`, with no validation
///
/// Returns: `0x2::url::Url`.
pub async fn new_unsafe(b: &mut PtbBuilder, arg0: impl PureString) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "url",
        "new_unsafe",
        Vec::new(),
        vec![a0],
    )
}
/// Create a `Url` with no validation from bytes
/// Note: this will abort if `bytes` is not valid ASCII
///
/// Returns: `0x2::url::Url`.
pub async fn new_unsafe_from_bytes(
    b: &mut PtbBuilder,
    arg0: impl PureVec<u8>,
) -> Argument {
    let a0 = arg0.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "url",
        "new_unsafe_from_bytes",
        Vec::new(),
        vec![a0],
    )
}
/// Get inner URL
///
/// Returns: `0x1::ascii::String`.
pub async fn inner_url(b: &mut PtbBuilder, arg0: impl ArgumentUrl) -> Argument {
    let a0 = arg0.into_argument_ref(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "url",
        "inner_url",
        Vec::new(),
        vec![a0],
    )
}
/// Update the inner URL
pub async fn update(b: &mut PtbBuilder, arg0: impl ArgumentUrl, arg1: impl PureString) {
    let a0 = arg0.into_argument_mut(b).await;
    let a1 = arg1.into_argument(b).await;
    b.move_call(
        b.package_id::<super::Package>(),
        "url",
        "update",
        Vec::new(),
        vec![a0, a1],
    );
}
/// Read-only handle bound to a runtime package address. Use
/// via `super::Package::at(addr).<module>()` and chain
/// `<type>_tag()` to build a [`TypeTag`] without spinning up
/// a `PtbBuilder`.
pub struct ModuleAt {
    pub(crate) package: Address,
}
impl ModuleAt {
    pub fn url_tag(&self) -> TypeTag {
        <Url as MoveType>::type_tag_at(self.package)
    }
}
