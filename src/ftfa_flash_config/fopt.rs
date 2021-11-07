#[doc = "Register `FOPT` reader"]
pub struct R(crate::R<FOPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FOPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FOPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FOPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPBOOT0_A {
    #[doc = "0: Core and system clock divider (OUTDIV1) is 0x7 (divide by 8) when LPBOOT1=0 or 0x1 (divide by 2) when LPBOOT1=1."]
    _00 = 0,
    #[doc = "1: Core and system clock divider (OUTDIV1) is 0x3 (divide by 4) when LPBOOT1=0 or 0x0 (divide by 1) when LPBOOT1=1."]
    _01 = 1,
}
impl From<LPBOOT0_A> for bool {
    #[inline(always)]
    fn from(variant: LPBOOT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPBOOT0` reader - no description available"]
pub struct LPBOOT0_R(crate::FieldReader<bool, LPBOOT0_A>);
impl LPBOOT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPBOOT0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPBOOT0_A {
        match self.bits {
            false => LPBOOT0_A::_00,
            true => LPBOOT0_A::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == LPBOOT0_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == LPBOOT0_A::_01
    }
}
impl core::ops::Deref for LPBOOT0_R {
    type Target = crate::FieldReader<bool, LPBOOT0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMI_DIS_A {
    #[doc = "0: NMI interrupts are always blocked"]
    _00 = 0,
    #[doc = "1: NMI_b pin/interrupts reset default to enabled"]
    _01 = 1,
}
impl From<NMI_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: NMI_DIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMI_DIS` reader - no description available"]
pub struct NMI_DIS_R(crate::FieldReader<bool, NMI_DIS_A>);
impl NMI_DIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        NMI_DIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NMI_DIS_A {
        match self.bits {
            false => NMI_DIS_A::_00,
            true => NMI_DIS_A::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == NMI_DIS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == NMI_DIS_A::_01
    }
}
impl core::ops::Deref for NMI_DIS_R {
    type Target = crate::FieldReader<bool, NMI_DIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESET_PIN_CFG_A {
    #[doc = "0: RESET pin is disabled following a POR and cannot be enabled as reset function"]
    _00 = 0,
    #[doc = "1: RESET_b pin is dedicated"]
    _01 = 1,
}
impl From<RESET_PIN_CFG_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_PIN_CFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESET_PIN_CFG` reader - no description available"]
pub struct RESET_PIN_CFG_R(crate::FieldReader<bool, RESET_PIN_CFG_A>);
impl RESET_PIN_CFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESET_PIN_CFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESET_PIN_CFG_A {
        match self.bits {
            false => RESET_PIN_CFG_A::_00,
            true => RESET_PIN_CFG_A::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == RESET_PIN_CFG_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == RESET_PIN_CFG_A::_01
    }
}
impl core::ops::Deref for RESET_PIN_CFG_R {
    type Target = crate::FieldReader<bool, RESET_PIN_CFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPBOOT1_A {
    #[doc = "0: Core and system clock divider (OUTDIV1) is 0x7 (divide by 8) when LPBOOT0=0 or 0x3 (divide by 4) when LPBOOT0=1."]
    _00 = 0,
    #[doc = "1: Core and system clock divider (OUTDIV1) is 0x1 (divide by 2) when LPBOOT0=0 or 0x0 (divide by 1) when LPBOOT0=1."]
    _01 = 1,
}
impl From<LPBOOT1_A> for bool {
    #[inline(always)]
    fn from(variant: LPBOOT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPBOOT1` reader - no description available"]
pub struct LPBOOT1_R(crate::FieldReader<bool, LPBOOT1_A>);
impl LPBOOT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPBOOT1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPBOOT1_A {
        match self.bits {
            false => LPBOOT1_A::_00,
            true => LPBOOT1_A::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == LPBOOT1_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == LPBOOT1_A::_01
    }
}
impl core::ops::Deref for LPBOOT1_R {
    type Target = crate::FieldReader<bool, LPBOOT1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAST_INIT_A {
    #[doc = "0: Slower initialization"]
    _00 = 0,
    #[doc = "1: Fast Initialization"]
    _01 = 1,
}
impl From<FAST_INIT_A> for bool {
    #[inline(always)]
    fn from(variant: FAST_INIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAST_INIT` reader - no description available"]
pub struct FAST_INIT_R(crate::FieldReader<bool, FAST_INIT_A>);
impl FAST_INIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        FAST_INIT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAST_INIT_A {
        match self.bits {
            false => FAST_INIT_A::_00,
            true => FAST_INIT_A::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == FAST_INIT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == FAST_INIT_A::_01
    }
}
impl core::ops::Deref for FAST_INIT_R {
    type Target = crate::FieldReader<bool, FAST_INIT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn lpboot0(&self) -> LPBOOT0_R {
        LPBOOT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn nmi_dis(&self) -> NMI_DIS_R {
        NMI_DIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn reset_pin_cfg(&self) -> RESET_PIN_CFG_R {
        RESET_PIN_CFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn lpboot1(&self) -> LPBOOT1_R {
        LPBOOT1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn fast_init(&self) -> FAST_INIT_R {
        FAST_INIT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
#[doc = "Non-volatile Flash Option Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fopt](index.html) module"]
pub struct FOPT_SPEC;
impl crate::RegisterSpec for FOPT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fopt::R](R) reader structure"]
impl crate::Readable for FOPT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FOPT to value 0xff"]
impl crate::Resettable for FOPT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
