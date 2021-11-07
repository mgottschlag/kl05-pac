#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x80 - Pin Control Register n"]
    pub pcr: [crate::Reg<pcr::PCR_SPEC>; 32],
    #[doc = "0x80 - Global Pin Control Low Register"]
    pub gpclr: crate::Reg<gpclr::GPCLR_SPEC>,
    #[doc = "0x84 - Global Pin Control High Register"]
    pub gpchr: crate::Reg<gpchr::GPCHR_SPEC>,
    _reserved3: [u8; 0x18],
    #[doc = "0xa0 - Interrupt Status Flag Register"]
    pub isfr: crate::Reg<isfr::ISFR_SPEC>,
}
#[doc = "PCR register accessor: an alias for `Reg<PCR_SPEC>`"]
pub type PCR = crate::Reg<pcr::PCR_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr;
#[doc = "GPCLR register accessor: an alias for `Reg<GPCLR_SPEC>`"]
pub type GPCLR = crate::Reg<gpclr::GPCLR_SPEC>;
#[doc = "Global Pin Control Low Register"]
pub mod gpclr;
#[doc = "GPCHR register accessor: an alias for `Reg<GPCHR_SPEC>`"]
pub type GPCHR = crate::Reg<gpchr::GPCHR_SPEC>;
#[doc = "Global Pin Control High Register"]
pub mod gpchr;
#[doc = "ISFR register accessor: an alias for `Reg<ISFR_SPEC>`"]
pub type ISFR = crate::Reg<isfr::ISFR_SPEC>;
#[doc = "Interrupt Status Flag Register"]
pub mod isfr;
