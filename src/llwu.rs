#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LLWU Pin Enable 1 register"]
    pub pe1: crate::Reg<pe1::PE1_SPEC>,
    #[doc = "0x01 - LLWU Pin Enable 2 register"]
    pub pe2: crate::Reg<pe2::PE2_SPEC>,
    #[doc = "0x02 - LLWU Module Enable register"]
    pub me: crate::Reg<me::ME_SPEC>,
    #[doc = "0x03 - LLWU Flag 1 register"]
    pub f1: crate::Reg<f1::F1_SPEC>,
    #[doc = "0x04 - LLWU Flag 3 register"]
    pub f3: crate::Reg<f3::F3_SPEC>,
    #[doc = "0x05 - LLWU Pin Filter 1 register"]
    pub filt1: crate::Reg<filt1::FILT1_SPEC>,
    #[doc = "0x06 - LLWU Pin Filter 2 register"]
    pub filt2: crate::Reg<filt2::FILT2_SPEC>,
}
#[doc = "PE1 register accessor: an alias for `Reg<PE1_SPEC>`"]
pub type PE1 = crate::Reg<pe1::PE1_SPEC>;
#[doc = "LLWU Pin Enable 1 register"]
pub mod pe1;
#[doc = "PE2 register accessor: an alias for `Reg<PE2_SPEC>`"]
pub type PE2 = crate::Reg<pe2::PE2_SPEC>;
#[doc = "LLWU Pin Enable 2 register"]
pub mod pe2;
#[doc = "ME register accessor: an alias for `Reg<ME_SPEC>`"]
pub type ME = crate::Reg<me::ME_SPEC>;
#[doc = "LLWU Module Enable register"]
pub mod me;
#[doc = "F1 register accessor: an alias for `Reg<F1_SPEC>`"]
pub type F1 = crate::Reg<f1::F1_SPEC>;
#[doc = "LLWU Flag 1 register"]
pub mod f1;
#[doc = "F3 register accessor: an alias for `Reg<F3_SPEC>`"]
pub type F3 = crate::Reg<f3::F3_SPEC>;
#[doc = "LLWU Flag 3 register"]
pub mod f3;
#[doc = "FILT1 register accessor: an alias for `Reg<FILT1_SPEC>`"]
pub type FILT1 = crate::Reg<filt1::FILT1_SPEC>;
#[doc = "LLWU Pin Filter 1 register"]
pub mod filt1;
#[doc = "FILT2 register accessor: an alias for `Reg<FILT2_SPEC>`"]
pub type FILT2 = crate::Reg<filt2::FILT2_SPEC>;
#[doc = "LLWU Pin Filter 2 register"]
pub mod filt2;
