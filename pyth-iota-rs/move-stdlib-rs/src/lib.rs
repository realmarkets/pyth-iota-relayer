#![allow(unused_imports, non_snake_case, clippy::too_many_arguments)]
use move_bindgen_runtime::Address;
pub mod bcs;
pub mod debug;
pub mod fixed_point32;
pub mod hash;
pub mod u128;
pub mod u16;
pub mod u256;
pub mod u32;
pub mod u64;
pub mod u8;
pub mod vector;
pub mod bit_vector;
pub mod address;
pub mod type_name;
/// Marker type identifying this package. Register the package's
/// on-chain address before issuing any PTB call:
///
/// ```ignore
/// let mut b = PtbBuilder::new(sender);
/// b.with_package::<Package>(my_published_address);
/// // ...generated calls now resolve `my_published_address`.
/// ```
///
/// Same marker is accepted by the free-standing
/// `PackageRegistry::at::<Package>(addr)` for non-PTB callers
/// (event decoders, BCS deserialization, etc.).
pub struct Package;
impl Package {
    /// Bind the package to a runtime address and get a chainable
    /// handle:
    ///
    /// ```ignore
    /// let pkg = Package::at(my_addr);
    /// let tag = pkg.counter().counter_tag();
    /// ```
    pub fn at(addr: Address) -> PackageAt {
        PackageAt { addr }
    }
}
/// Read-only handle bound to a runtime package address. Use
/// the per-module accessors to navigate to a [`TypeTag`] without
/// spinning up a `PtbBuilder`.
pub struct PackageAt {
    addr: Address,
}
impl PackageAt {
    pub fn fixed_point32(&self) -> fixed_point32::ModuleAt {
        fixed_point32::ModuleAt {
            package: self.addr,
        }
    }
    pub fn bit_vector(&self) -> bit_vector::ModuleAt {
        bit_vector::ModuleAt {
            package: self.addr,
        }
    }
    pub fn type_name(&self) -> type_name::ModuleAt {
        type_name::ModuleAt {
            package: self.addr,
        }
    }
}
