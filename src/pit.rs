#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PIT Module Control Register"]
    pub mcr: crate::Reg<mcr::MCR_SPEC>,
    _reserved1: [u8; 0xdc],
    #[doc = "0xe0 - PIT Upper Lifetime Timer Register"]
    pub ltmr64h: crate::Reg<ltmr64h::LTMR64H_SPEC>,
    #[doc = "0xe4 - PIT Lower Lifetime Timer Register"]
    pub ltmr64l: crate::Reg<ltmr64l::LTMR64L_SPEC>,
    _reserved3: [u8; 0x18],
    #[doc = "0x100 - Timer Load Value Register"]
    pub ldval0: crate::Reg<ldval::LDVAL_SPEC>,
    #[doc = "0x104 - Current Timer Value Register"]
    pub cval0: crate::Reg<cval::CVAL_SPEC>,
    #[doc = "0x108 - Timer Control Register"]
    pub tctrl0: crate::Reg<tctrl::TCTRL_SPEC>,
    #[doc = "0x10c - Timer Flag Register"]
    pub tflg0: crate::Reg<tflg::TFLG_SPEC>,
    #[doc = "0x110 - Timer Load Value Register"]
    pub ldval1: crate::Reg<ldval::LDVAL_SPEC>,
    #[doc = "0x114 - Current Timer Value Register"]
    pub cval1: crate::Reg<cval::CVAL_SPEC>,
    #[doc = "0x118 - Timer Control Register"]
    pub tctrl1: crate::Reg<tctrl::TCTRL_SPEC>,
    #[doc = "0x11c - Timer Flag Register"]
    pub tflg1: crate::Reg<tflg::TFLG_SPEC>,
}
#[doc = "MCR register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "PIT Module Control Register"]
pub mod mcr;
#[doc = "LTMR64H register accessor: an alias for `Reg<LTMR64H_SPEC>`"]
pub type LTMR64H = crate::Reg<ltmr64h::LTMR64H_SPEC>;
#[doc = "PIT Upper Lifetime Timer Register"]
pub mod ltmr64h;
#[doc = "LTMR64L register accessor: an alias for `Reg<LTMR64L_SPEC>`"]
pub type LTMR64L = crate::Reg<ltmr64l::LTMR64L_SPEC>;
#[doc = "PIT Lower Lifetime Timer Register"]
pub mod ltmr64l;
#[doc = "LDVAL register accessor: an alias for `Reg<LDVAL_SPEC>`"]
pub type LDVAL = crate::Reg<ldval::LDVAL_SPEC>;
#[doc = "Timer Load Value Register"]
pub mod ldval;
#[doc = "CVAL register accessor: an alias for `Reg<CVAL_SPEC>`"]
pub type CVAL = crate::Reg<cval::CVAL_SPEC>;
#[doc = "Current Timer Value Register"]
pub mod cval;
#[doc = "TCTRL register accessor: an alias for `Reg<TCTRL_SPEC>`"]
pub type TCTRL = crate::Reg<tctrl::TCTRL_SPEC>;
#[doc = "Timer Control Register"]
pub mod tctrl;
#[doc = "TFLG register accessor: an alias for `Reg<TFLG_SPEC>`"]
pub type TFLG = crate::Reg<tflg::TFLG_SPEC>;
#[doc = "Timer Flag Register"]
pub mod tflg;
