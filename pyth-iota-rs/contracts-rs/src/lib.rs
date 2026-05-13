#![allow(unused_imports, non_snake_case, clippy::too_many_arguments)]
use move_bindgen_runtime::Address;
pub mod price_identifier;
pub mod i64;
pub mod price;
pub mod price_feed;
pub mod price_info;
pub mod deserialize;
pub mod merkle_tree;
pub mod accumulator;
pub mod price_status;
pub mod batch_price_attestation;
pub mod version_control;
pub mod event;
pub mod set;
pub mod data_source;
pub mod state;
pub mod governance_action;
pub mod governance_instruction;
pub mod set_update_fee;
pub mod set_stale_price_threshold;
pub mod set_governance_data_source;
pub mod set_fee_recipient;
pub mod set_data_sources;
pub mod governance;
pub mod contract_upgrade;
pub mod hot_potato_vector;
pub mod migrate;
pub mod setup;
pub mod pyth;
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
    pub fn price_identifier(&self) -> price_identifier::ModuleAt {
        price_identifier::ModuleAt {
            package: self.addr,
        }
    }
    pub fn i64(&self) -> i64::ModuleAt {
        i64::ModuleAt {
            package: self.addr,
        }
    }
    pub fn price(&self) -> price::ModuleAt {
        price::ModuleAt {
            package: self.addr,
        }
    }
    pub fn price_feed(&self) -> price_feed::ModuleAt {
        price_feed::ModuleAt {
            package: self.addr,
        }
    }
    pub fn price_info(&self) -> price_info::ModuleAt {
        price_info::ModuleAt {
            package: self.addr,
        }
    }
    pub fn price_status(&self) -> price_status::ModuleAt {
        price_status::ModuleAt {
            package: self.addr,
        }
    }
    pub fn batch_price_attestation(&self) -> batch_price_attestation::ModuleAt {
        batch_price_attestation::ModuleAt {
            package: self.addr,
        }
    }
    pub fn version_control(&self) -> version_control::ModuleAt {
        version_control::ModuleAt {
            package: self.addr,
        }
    }
    pub fn event(&self) -> event::ModuleAt {
        event::ModuleAt {
            package: self.addr,
        }
    }
    pub fn set(&self) -> set::ModuleAt {
        set::ModuleAt {
            package: self.addr,
        }
    }
    pub fn data_source(&self) -> data_source::ModuleAt {
        data_source::ModuleAt {
            package: self.addr,
        }
    }
    pub fn state(&self) -> state::ModuleAt {
        state::ModuleAt {
            package: self.addr,
        }
    }
    pub fn governance_action(&self) -> governance_action::ModuleAt {
        governance_action::ModuleAt {
            package: self.addr,
        }
    }
    pub fn governance_instruction(&self) -> governance_instruction::ModuleAt {
        governance_instruction::ModuleAt {
            package: self.addr,
        }
    }
    pub fn set_update_fee(&self) -> set_update_fee::ModuleAt {
        set_update_fee::ModuleAt {
            package: self.addr,
        }
    }
    pub fn set_stale_price_threshold(&self) -> set_stale_price_threshold::ModuleAt {
        set_stale_price_threshold::ModuleAt {
            package: self.addr,
        }
    }
    pub fn set_governance_data_source(&self) -> set_governance_data_source::ModuleAt {
        set_governance_data_source::ModuleAt {
            package: self.addr,
        }
    }
    pub fn set_fee_recipient(&self) -> set_fee_recipient::ModuleAt {
        set_fee_recipient::ModuleAt {
            package: self.addr,
        }
    }
    pub fn set_data_sources(&self) -> set_data_sources::ModuleAt {
        set_data_sources::ModuleAt {
            package: self.addr,
        }
    }
    pub fn governance(&self) -> governance::ModuleAt {
        governance::ModuleAt {
            package: self.addr,
        }
    }
    pub fn contract_upgrade(&self) -> contract_upgrade::ModuleAt {
        contract_upgrade::ModuleAt {
            package: self.addr,
        }
    }
    pub fn migrate(&self) -> migrate::ModuleAt {
        migrate::ModuleAt {
            package: self.addr,
        }
    }
    pub fn setup(&self) -> setup::ModuleAt {
        setup::ModuleAt {
            package: self.addr,
        }
    }
}
