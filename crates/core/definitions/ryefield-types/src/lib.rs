//! # RyeFieldTypes
//! 
//! ## Description
//! 
//! The `RyeFieldTypes` package contains types for all different type of required and optional type systems. These are also called primitives.
//! 
//! ## Overview
//! 
//! - [ ]**RyeAddress:** A BLAKE2S/BLAKE2B
//!     - [ ] Generate From Input
//!     - [ ] Variable Digest
//! - [ ]**RyeAuth** (Using RyeSlab)
//!     - [ ] Allows easy sharing of data across services
//! - [ ] **RyeCID:** A BLAKE3 or BLAKE2B Content-Identifier for data
//! - [ ] **RyePublicKey:** A Public Key
//! 
//! ---
//! 
//! ## 1: RyeAddress
//! 
//! A RyeAddress is either a default address or a network extended address which specifies the network it is interacting with.
//! 
//! ### Extensions
//! 
//! - [ ] Default (RyeAddress-Only)
//! - [ ] Default-With-Network (RyeAddress With Network Info)
//! - [ ] Default-With-OTP
//! - [ ] RegisterAsService: Registers a RyeAddress as a service
//! - [ ] RegisterAsDAO: Registers as a DAO
//! 
//! ---

use fixedstr::str256;


pub mod rye_address {
    use fixedstr::str256;
    use crate::RyeNetwork::RyeNetworkType;

    /// # Rye Address
    /// 
    /// ## Developer Notes
    /// 
    /// - Contains a fixedstr of 256.
    #[derive(Debug,Clone,Copy,Serialize,Deserialize,PartialEq,PartialOrd,Hash)]
    pub struct RyeAddress(pub str256);

    #[derive(Debug,Clone,Copy,Serialize,Deserialize,PartialEq,PartialOrd,Hash)]
    pub struct RyeAddressWithNetwork {
        addr: RyeAddress,
        network: RyeNetworkType,
    }

    pub struct RyeAddressWithNonce {
        addr: RyeAddress,
        nonce: u64,
    }
}

pub mod RyeNetwork {
    use fixedstr::str64;

    
    /// # Enumerate Rye Networks
    /// 
    /// This special type provides an easy wrapper for enumerating different Networks.
    /// 
    /// It can be seen as a wrapper/api for accessing new networks.
    /// 
    /// ## TODO:
    /// 
    /// - Add Enumeration of Networks
    pub struct RyeNetworkEnumerater;

    /// # RyeNetworkType
    /// 
    /// A specificed Network Identifier
    #[derive(Debug,Clone,Copy,Serialize,Deserialize,PartialEq,PartialOrd,Hash)]
    pub struct RyeNetworkType {
        pub network: str64,
    }
}

pub mod AccountLedgerManager {
    
    use fixedstr::str256;
    use crate::rye_address::RyeAddress;

    pub struct AccountLedgerManager {
        addr: RyeAddress
    }
}

pub mod registration;
pub mod ryeauth;
pub mod traits;