#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CMP Control Register 0"]
    pub cr0: crate::Reg<cr0::CR0_SPEC>,
    #[doc = "0x01 - CMP Control Register 1"]
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    #[doc = "0x02 - CMP Filter Period Register"]
    pub fpr: crate::Reg<fpr::FPR_SPEC>,
    #[doc = "0x03 - CMP Status and Control Register"]
    pub scr: crate::Reg<scr::SCR_SPEC>,
    #[doc = "0x04 - DAC Control Register"]
    pub daccr: crate::Reg<daccr::DACCR_SPEC>,
    #[doc = "0x05 - MUX Control Register"]
    pub muxcr: crate::Reg<muxcr::MUXCR_SPEC>,
}
#[doc = "CR0 register accessor: an alias for `Reg<CR0_SPEC>`"]
pub type CR0 = crate::Reg<cr0::CR0_SPEC>;
#[doc = "CMP Control Register 0"]
pub mod cr0;
#[doc = "CR1 register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "CMP Control Register 1"]
pub mod cr1;
#[doc = "FPR register accessor: an alias for `Reg<FPR_SPEC>`"]
pub type FPR = crate::Reg<fpr::FPR_SPEC>;
#[doc = "CMP Filter Period Register"]
pub mod fpr;
#[doc = "SCR register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "CMP Status and Control Register"]
pub mod scr;
#[doc = "DACCR register accessor: an alias for `Reg<DACCR_SPEC>`"]
pub type DACCR = crate::Reg<daccr::DACCR_SPEC>;
#[doc = "DAC Control Register"]
pub mod daccr;
#[doc = "MUXCR register accessor: an alias for `Reg<MUXCR_SPEC>`"]
pub type MUXCR = crate::Reg<muxcr::MUXCR_SPEC>;
#[doc = "MUX Control Register"]
pub mod muxcr;
