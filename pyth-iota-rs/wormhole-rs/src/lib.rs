#![allow(unused_imports, non_snake_case, clippy::too_many_arguments)]
use move_bindgen_runtime::Address;
pub mod cursor;
pub mod bytes;
pub mod bytes20;
pub mod bytes32;
pub mod set;
pub mod consumed_vaas;
pub mod version_control;
pub mod package_utils;
pub mod guardian_signature;
pub mod guardian;
pub mod guardian_set;
pub mod fee_collector;
pub mod external_address;
pub mod state;
pub mod emitter;
pub mod vaa;
pub mod governance_message;
pub mod upgrade_contract;
pub mod migrate;
pub mod publish_message;
pub mod set_fee;
pub mod setup;
pub mod transfer_fee;
pub mod update_guardian_set;
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
    pub fn bytes20(&self) -> bytes20::ModuleAt {
        bytes20::ModuleAt {
            package: self.addr,
        }
    }
    pub fn bytes32(&self) -> bytes32::ModuleAt {
        bytes32::ModuleAt {
            package: self.addr,
        }
    }
    pub fn set(&self) -> set::ModuleAt {
        set::ModuleAt {
            package: self.addr,
        }
    }
    pub fn consumed_vaas(&self) -> consumed_vaas::ModuleAt {
        consumed_vaas::ModuleAt {
            package: self.addr,
        }
    }
    pub fn version_control(&self) -> version_control::ModuleAt {
        version_control::ModuleAt {
            package: self.addr,
        }
    }
    pub fn package_utils(&self) -> package_utils::ModuleAt {
        package_utils::ModuleAt {
            package: self.addr,
        }
    }
    pub fn guardian_signature(&self) -> guardian_signature::ModuleAt {
        guardian_signature::ModuleAt {
            package: self.addr,
        }
    }
    pub fn guardian(&self) -> guardian::ModuleAt {
        guardian::ModuleAt {
            package: self.addr,
        }
    }
    pub fn guardian_set(&self) -> guardian_set::ModuleAt {
        guardian_set::ModuleAt {
            package: self.addr,
        }
    }
    pub fn fee_collector(&self) -> fee_collector::ModuleAt {
        fee_collector::ModuleAt {
            package: self.addr,
        }
    }
    pub fn external_address(&self) -> external_address::ModuleAt {
        external_address::ModuleAt {
            package: self.addr,
        }
    }
    pub fn state(&self) -> state::ModuleAt {
        state::ModuleAt {
            package: self.addr,
        }
    }
    pub fn emitter(&self) -> emitter::ModuleAt {
        emitter::ModuleAt {
            package: self.addr,
        }
    }
    pub fn vaa(&self) -> vaa::ModuleAt {
        vaa::ModuleAt {
            package: self.addr,
        }
    }
    pub fn upgrade_contract(&self) -> upgrade_contract::ModuleAt {
        upgrade_contract::ModuleAt {
            package: self.addr,
        }
    }
    pub fn migrate(&self) -> migrate::ModuleAt {
        migrate::ModuleAt {
            package: self.addr,
        }
    }
    pub fn publish_message(&self) -> publish_message::ModuleAt {
        publish_message::ModuleAt {
            package: self.addr,
        }
    }
    pub fn set_fee(&self) -> set_fee::ModuleAt {
        set_fee::ModuleAt {
            package: self.addr,
        }
    }
    pub fn setup(&self) -> setup::ModuleAt {
        setup::ModuleAt {
            package: self.addr,
        }
    }
    pub fn transfer_fee(&self) -> transfer_fee::ModuleAt {
        transfer_fee::ModuleAt {
            package: self.addr,
        }
    }
    pub fn update_guardian_set(&self) -> update_guardian_set::ModuleAt {
        update_guardian_set::ModuleAt {
            package: self.addr,
        }
    }
}
