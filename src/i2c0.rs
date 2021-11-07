#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C Address Register 1"]
    pub a1: crate::Reg<a1::A1_SPEC>,
    #[doc = "0x01 - I2C Frequency Divider register"]
    pub f: crate::Reg<f::F_SPEC>,
    #[doc = "0x02 - I2C Control Register 1"]
    pub c1: crate::Reg<c1::C1_SPEC>,
    #[doc = "0x03 - I2C Status register"]
    pub s: crate::Reg<s::S_SPEC>,
    #[doc = "0x04 - I2C Data I/O register"]
    pub d: crate::Reg<d::D_SPEC>,
    #[doc = "0x05 - I2C Control Register 2"]
    pub c2: crate::Reg<c2::C2_SPEC>,
    #[doc = "0x06 - I2C Programmable Input Glitch Filter register"]
    pub flt: crate::Reg<flt::FLT_SPEC>,
    #[doc = "0x07 - I2C Range Address register"]
    pub ra: crate::Reg<ra::RA_SPEC>,
    #[doc = "0x08 - I2C SMBus Control and Status register"]
    pub smb: crate::Reg<smb::SMB_SPEC>,
    #[doc = "0x09 - I2C Address Register 2"]
    pub a2: crate::Reg<a2::A2_SPEC>,
    #[doc = "0x0a - I2C SCL Low Timeout Register High"]
    pub slth: crate::Reg<slth::SLTH_SPEC>,
    #[doc = "0x0b - I2C SCL Low Timeout Register Low"]
    pub sltl: crate::Reg<sltl::SLTL_SPEC>,
}
#[doc = "A1 register accessor: an alias for `Reg<A1_SPEC>`"]
pub type A1 = crate::Reg<a1::A1_SPEC>;
#[doc = "I2C Address Register 1"]
pub mod a1;
#[doc = "F register accessor: an alias for `Reg<F_SPEC>`"]
pub type F = crate::Reg<f::F_SPEC>;
#[doc = "I2C Frequency Divider register"]
pub mod f;
#[doc = "C1 register accessor: an alias for `Reg<C1_SPEC>`"]
pub type C1 = crate::Reg<c1::C1_SPEC>;
#[doc = "I2C Control Register 1"]
pub mod c1;
#[doc = "S register accessor: an alias for `Reg<S_SPEC>`"]
pub type S = crate::Reg<s::S_SPEC>;
#[doc = "I2C Status register"]
pub mod s;
#[doc = "D register accessor: an alias for `Reg<D_SPEC>`"]
pub type D = crate::Reg<d::D_SPEC>;
#[doc = "I2C Data I/O register"]
pub mod d;
#[doc = "C2 register accessor: an alias for `Reg<C2_SPEC>`"]
pub type C2 = crate::Reg<c2::C2_SPEC>;
#[doc = "I2C Control Register 2"]
pub mod c2;
#[doc = "FLT register accessor: an alias for `Reg<FLT_SPEC>`"]
pub type FLT = crate::Reg<flt::FLT_SPEC>;
#[doc = "I2C Programmable Input Glitch Filter register"]
pub mod flt;
#[doc = "RA register accessor: an alias for `Reg<RA_SPEC>`"]
pub type RA = crate::Reg<ra::RA_SPEC>;
#[doc = "I2C Range Address register"]
pub mod ra;
#[doc = "SMB register accessor: an alias for `Reg<SMB_SPEC>`"]
pub type SMB = crate::Reg<smb::SMB_SPEC>;
#[doc = "I2C SMBus Control and Status register"]
pub mod smb;
#[doc = "A2 register accessor: an alias for `Reg<A2_SPEC>`"]
pub type A2 = crate::Reg<a2::A2_SPEC>;
#[doc = "I2C Address Register 2"]
pub mod a2;
#[doc = "SLTH register accessor: an alias for `Reg<SLTH_SPEC>`"]
pub type SLTH = crate::Reg<slth::SLTH_SPEC>;
#[doc = "I2C SCL Low Timeout Register High"]
pub mod slth;
#[doc = "SLTL register accessor: an alias for `Reg<SLTL_SPEC>`"]
pub type SLTL = crate::Reg<sltl::SLTL_SPEC>;
#[doc = "I2C SCL Low Timeout Register Low"]
pub mod sltl;
