#![allow(unused_imports, non_snake_case, clippy::too_many_arguments)]
use move_bindgen_runtime::Address;
pub mod ecdsa_k1;
pub mod ecdsa_r1;
pub mod ecvrf;
pub mod ed25519;
pub mod groth16;
pub mod hash;
pub mod vdf;
pub mod hex;
pub mod address;
pub mod tx_context;
pub mod transfer;
pub mod dynamic_field;
pub mod authenticator_state;
pub mod bag;
pub mod balance;
pub mod bcs;
pub mod group_ops;
pub mod bls12381;
pub mod borrow;
pub mod clock;
pub mod url;
pub mod types;
pub mod event;
pub mod dynamic_object_field;
pub mod config;
pub mod deny_list;
pub mod coin;
pub mod coin_manager;
pub mod vec_map;
pub mod package;
pub mod display;
pub mod iota;
pub mod vec_set;
pub mod transfer_policy;
pub mod kiosk;
pub mod kiosk_extension;
pub mod labeler;
pub mod linked_table;
pub mod object_bag;
pub mod object_table;
pub mod pay;
pub mod poseidon;
pub mod priority_queue;
pub mod versioned;
pub mod hmac;
pub mod random;
pub mod system_admin_cap;
pub mod table;
pub mod table_vec;
pub mod timelock;
pub mod token;
pub mod zklogin_verified_id;
pub mod zklogin_verified_issuer;
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
    pub fn groth16(&self) -> groth16::ModuleAt {
        groth16::ModuleAt {
            package: self.addr,
        }
    }
    pub fn tx_context(&self) -> tx_context::ModuleAt {
        tx_context::ModuleAt {
            package: self.addr,
        }
    }
    pub fn authenticator_state(&self) -> authenticator_state::ModuleAt {
        authenticator_state::ModuleAt {
            package: self.addr,
        }
    }
    pub fn bag(&self) -> bag::ModuleAt {
        bag::ModuleAt {
            package: self.addr,
        }
    }
    pub fn bcs(&self) -> bcs::ModuleAt {
        bcs::ModuleAt {
            package: self.addr,
        }
    }
    pub fn bls12381(&self) -> bls12381::ModuleAt {
        bls12381::ModuleAt {
            package: self.addr,
        }
    }
    pub fn borrow(&self) -> borrow::ModuleAt {
        borrow::ModuleAt {
            package: self.addr,
        }
    }
    pub fn clock(&self) -> clock::ModuleAt {
        clock::ModuleAt {
            package: self.addr,
        }
    }
    pub fn url(&self) -> url::ModuleAt {
        url::ModuleAt {
            package: self.addr,
        }
    }
    pub fn deny_list(&self) -> deny_list::ModuleAt {
        deny_list::ModuleAt {
            package: self.addr,
        }
    }
    pub fn coin_manager(&self) -> coin_manager::ModuleAt {
        coin_manager::ModuleAt {
            package: self.addr,
        }
    }
    pub fn package(&self) -> package::ModuleAt {
        package::ModuleAt {
            package: self.addr,
        }
    }
    pub fn iota(&self) -> iota::ModuleAt {
        iota::ModuleAt {
            package: self.addr,
        }
    }
    pub fn kiosk(&self) -> kiosk::ModuleAt {
        kiosk::ModuleAt {
            package: self.addr,
        }
    }
    pub fn kiosk_extension(&self) -> kiosk_extension::ModuleAt {
        kiosk_extension::ModuleAt {
            package: self.addr,
        }
    }
    pub fn object_bag(&self) -> object_bag::ModuleAt {
        object_bag::ModuleAt {
            package: self.addr,
        }
    }
    pub fn versioned(&self) -> versioned::ModuleAt {
        versioned::ModuleAt {
            package: self.addr,
        }
    }
    pub fn random(&self) -> random::ModuleAt {
        random::ModuleAt {
            package: self.addr,
        }
    }
    pub fn system_admin_cap(&self) -> system_admin_cap::ModuleAt {
        system_admin_cap::ModuleAt {
            package: self.addr,
        }
    }
    pub fn zklogin_verified_id(&self) -> zklogin_verified_id::ModuleAt {
        zklogin_verified_id::ModuleAt {
            package: self.addr,
        }
    }
    pub fn zklogin_verified_issuer(&self) -> zklogin_verified_issuer::ModuleAt {
        zklogin_verified_issuer::ModuleAt {
            package: self.addr,
        }
    }
}
