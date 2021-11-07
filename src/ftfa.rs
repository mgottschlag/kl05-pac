#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Status Register"]
    pub fstat: crate::Reg<fstat::FSTAT_SPEC>,
    #[doc = "0x01 - Flash Configuration Register"]
    pub fcnfg: crate::Reg<fcnfg::FCNFG_SPEC>,
    #[doc = "0x02 - Flash Security Register"]
    pub fsec: crate::Reg<fsec::FSEC_SPEC>,
    #[doc = "0x03 - Flash Option Register"]
    pub fopt: crate::Reg<fopt::FOPT_SPEC>,
    #[doc = "0x04 - Flash Common Command Object Registers"]
    pub fccob3: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x05 - Flash Common Command Object Registers"]
    pub fccob2: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x06 - Flash Common Command Object Registers"]
    pub fccob1: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x07 - Flash Common Command Object Registers"]
    pub fccob0: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x08 - Flash Common Command Object Registers"]
    pub fccob7: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x09 - Flash Common Command Object Registers"]
    pub fccob6: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x0a - Flash Common Command Object Registers"]
    pub fccob5: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x0b - Flash Common Command Object Registers"]
    pub fccob4: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x0c - Flash Common Command Object Registers"]
    pub fccobb: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x0d - Flash Common Command Object Registers"]
    pub fccoba: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x0e - Flash Common Command Object Registers"]
    pub fccob9: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x0f - Flash Common Command Object Registers"]
    pub fccob8: crate::Reg<fccob::FCCOB_SPEC>,
    #[doc = "0x10 - Program Flash Protection Registers"]
    pub fprot3: crate::Reg<fprot::FPROT_SPEC>,
    #[doc = "0x11 - Program Flash Protection Registers"]
    pub fprot2: crate::Reg<fprot::FPROT_SPEC>,
    #[doc = "0x12 - Program Flash Protection Registers"]
    pub fprot1: crate::Reg<fprot::FPROT_SPEC>,
    #[doc = "0x13 - Program Flash Protection Registers"]
    pub fprot0: crate::Reg<fprot::FPROT_SPEC>,
}
#[doc = "FSTAT register accessor: an alias for `Reg<FSTAT_SPEC>`"]
pub type FSTAT = crate::Reg<fstat::FSTAT_SPEC>;
#[doc = "Flash Status Register"]
pub mod fstat;
#[doc = "FCNFG register accessor: an alias for `Reg<FCNFG_SPEC>`"]
pub type FCNFG = crate::Reg<fcnfg::FCNFG_SPEC>;
#[doc = "Flash Configuration Register"]
pub mod fcnfg;
#[doc = "FSEC register accessor: an alias for `Reg<FSEC_SPEC>`"]
pub type FSEC = crate::Reg<fsec::FSEC_SPEC>;
#[doc = "Flash Security Register"]
pub mod fsec;
#[doc = "FOPT register accessor: an alias for `Reg<FOPT_SPEC>`"]
pub type FOPT = crate::Reg<fopt::FOPT_SPEC>;
#[doc = "Flash Option Register"]
pub mod fopt;
#[doc = "FCCOB register accessor: an alias for `Reg<FCCOB_SPEC>`"]
pub type FCCOB = crate::Reg<fccob::FCCOB_SPEC>;
#[doc = "Flash Common Command Object Registers"]
pub mod fccob;
#[doc = "FPROT register accessor: an alias for `Reg<FPROT_SPEC>`"]
pub type FPROT = crate::Reg<fprot::FPROT_SPEC>;
#[doc = "Program Flash Protection Registers"]
pub mod fprot;
