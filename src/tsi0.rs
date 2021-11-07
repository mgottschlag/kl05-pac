#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TSI General Control and Status Register"]
    pub gencs: crate::Reg<gencs::GENCS_SPEC>,
    #[doc = "0x04 - TSI DATA Register"]
    pub data: crate::Reg<data::DATA_SPEC>,
    #[doc = "0x08 - TSI Threshold Register"]
    pub tshd: crate::Reg<tshd::TSHD_SPEC>,
}
#[doc = "GENCS register accessor: an alias for `Reg<GENCS_SPEC>`"]
pub type GENCS = crate::Reg<gencs::GENCS_SPEC>;
#[doc = "TSI General Control and Status Register"]
pub mod gencs;
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "TSI DATA Register"]
pub mod data;
#[doc = "TSHD register accessor: an alias for `Reg<TSHD_SPEC>`"]
pub type TSHD = crate::Reg<tshd::TSHD_SPEC>;
#[doc = "TSI Threshold Register"]
pub mod tshd;
