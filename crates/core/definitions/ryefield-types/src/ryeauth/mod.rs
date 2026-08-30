//! # RyeSlab: A Data Sharing Platform
//! 
//! RyeSlab allows you to share information only that is required. It protects privacy and security.
//! 
//! It uses the "SLAB" format to do this.


pub mod RyeSlab {
    
    use fixedstr::str256;
    use fixedstr::str128;
    /// # RyeSlabOwner:
    pub struct RyeSlabOwner(pub str256);
    pub struct RyeSlabProperties(pub str128);
}