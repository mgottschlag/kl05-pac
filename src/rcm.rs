#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Reset Status Register 0"]
    pub srs0: crate::Reg<srs0::SRS0_SPEC>,
    #[doc = "0x01 - System Reset Status Register 1"]
    pub srs1: crate::Reg<srs1::SRS1_SPEC>,
    _reserved2: [u8; 0x02],
    #[doc = "0x04 - Reset Pin Filter Control register"]
    pub rpfc: crate::Reg<rpfc::RPFC_SPEC>,
    #[doc = "0x05 - Reset Pin Filter Width register"]
    pub rpfw: crate::Reg<rpfw::RPFW_SPEC>,
}
#[doc = "SRS0 register accessor: an alias for `Reg<SRS0_SPEC>`"]
pub type SRS0 = crate::Reg<srs0::SRS0_SPEC>;
#[doc = "System Reset Status Register 0"]
pub mod srs0;
#[doc = "SRS1 register accessor: an alias for `Reg<SRS1_SPEC>`"]
pub type SRS1 = crate::Reg<srs1::SRS1_SPEC>;
#[doc = "System Reset Status Register 1"]
pub mod srs1;
#[doc = "RPFC register accessor: an alias for `Reg<RPFC_SPEC>`"]
pub type RPFC = crate::Reg<rpfc::RPFC_SPEC>;
#[doc = "Reset Pin Filter Control register"]
pub mod rpfc;
#[doc = "RPFW register accessor: an alias for `Reg<RPFW_SPEC>`"]
pub type RPFW = crate::Reg<rpfw::RPFW_SPEC>;
#[doc = "Reset Pin Filter Width register"]
pub mod rpfw;
