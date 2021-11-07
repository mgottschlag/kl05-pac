#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DAC Data Low Register"]
    pub dat0l: crate::Reg<datl::DATL_SPEC>,
    #[doc = "0x01 - DAC Data High Register"]
    pub dat0h: crate::Reg<dath::DATH_SPEC>,
    #[doc = "0x02 - DAC Data Low Register"]
    pub dat1l: crate::Reg<datl::DATL_SPEC>,
    #[doc = "0x03 - DAC Data High Register"]
    pub dat1h: crate::Reg<dath::DATH_SPEC>,
    _reserved4: [u8; 0x1c],
    #[doc = "0x20 - DAC Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x21 - DAC Control Register"]
    pub c0: crate::Reg<c0::C0_SPEC>,
    #[doc = "0x22 - DAC Control Register 1"]
    pub c1: crate::Reg<c1::C1_SPEC>,
    #[doc = "0x23 - DAC Control Register 2"]
    pub c2: crate::Reg<c2::C2_SPEC>,
}
#[doc = "DATL register accessor: an alias for `Reg<DATL_SPEC>`"]
pub type DATL = crate::Reg<datl::DATL_SPEC>;
#[doc = "DAC Data Low Register"]
pub mod datl;
#[doc = "DATH register accessor: an alias for `Reg<DATH_SPEC>`"]
pub type DATH = crate::Reg<dath::DATH_SPEC>;
#[doc = "DAC Data High Register"]
pub mod dath;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "DAC Status Register"]
pub mod sr;
#[doc = "C0 register accessor: an alias for `Reg<C0_SPEC>`"]
pub type C0 = crate::Reg<c0::C0_SPEC>;
#[doc = "DAC Control Register"]
pub mod c0;
#[doc = "C1 register accessor: an alias for `Reg<C1_SPEC>`"]
pub type C1 = crate::Reg<c1::C1_SPEC>;
#[doc = "DAC Control Register 1"]
pub mod c1;
#[doc = "C2 register accessor: an alias for `Reg<C2_SPEC>`"]
pub type C2 = crate::Reg<c2::C2_SPEC>;
#[doc = "DAC Control Register 2"]
pub mod c2;
