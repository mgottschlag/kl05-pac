#[doc = "Register `S` reader"]
pub struct R(crate::R<S_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Master mode fault flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODF_A {
    #[doc = "0: No mode fault error"]
    _0 = 0,
    #[doc = "1: Mode fault error detected"]
    _1 = 1,
}
impl From<MODF_A> for bool {
    #[inline(always)]
    fn from(variant: MODF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODF` reader - Master mode fault flag"]
pub struct MODF_R(crate::FieldReader<bool, MODF_A>);
impl MODF_R {
    pub(crate) fn new(bits: bool) -> Self {
        MODF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODF_A {
        match self.bits {
            false => MODF_A::_0,
            true => MODF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MODF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MODF_A::_1
    }
}
impl core::ops::Deref for MODF_R {
    type Target = crate::FieldReader<bool, MODF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "SPI transmit buffer empty flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPTEF_A {
    #[doc = "0: SPI transmit buffer not empty"]
    _0 = 0,
    #[doc = "1: SPI transmit buffer empty"]
    _1 = 1,
}
impl From<SPTEF_A> for bool {
    #[inline(always)]
    fn from(variant: SPTEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPTEF` reader - SPI transmit buffer empty flag"]
pub struct SPTEF_R(crate::FieldReader<bool, SPTEF_A>);
impl SPTEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPTEF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPTEF_A {
        match self.bits {
            false => SPTEF_A::_0,
            true => SPTEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPTEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPTEF_A::_1
    }
}
impl core::ops::Deref for SPTEF_R {
    type Target = crate::FieldReader<bool, SPTEF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "SPI match flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPMF_A {
    #[doc = "0: Value in the receive data buffer does not match the value in the M register"]
    _0 = 0,
    #[doc = "1: Value in the receive data buffer matches the value in the M register"]
    _1 = 1,
}
impl From<SPMF_A> for bool {
    #[inline(always)]
    fn from(variant: SPMF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPMF` reader - SPI match flag"]
pub struct SPMF_R(crate::FieldReader<bool, SPMF_A>);
impl SPMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPMF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPMF_A {
        match self.bits {
            false => SPMF_A::_0,
            true => SPMF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPMF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPMF_A::_1
    }
}
impl core::ops::Deref for SPMF_R {
    type Target = crate::FieldReader<bool, SPMF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "SPI read buffer full flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPRF_A {
    #[doc = "0: No data available in the receive data buffer"]
    _0 = 0,
    #[doc = "1: Data available in the receive data buffer"]
    _1 = 1,
}
impl From<SPRF_A> for bool {
    #[inline(always)]
    fn from(variant: SPRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPRF` reader - SPI read buffer full flag"]
pub struct SPRF_R(crate::FieldReader<bool, SPRF_A>);
impl SPRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPRF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPRF_A {
        match self.bits {
            false => SPRF_A::_0,
            true => SPRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPRF_A::_1
    }
}
impl core::ops::Deref for SPRF_R {
    type Target = crate::FieldReader<bool, SPRF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 4 - Master mode fault flag"]
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SPI transmit buffer empty flag"]
    #[inline(always)]
    pub fn sptef(&self) -> SPTEF_R {
        SPTEF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SPI match flag"]
    #[inline(always)]
    pub fn spmf(&self) -> SPMF_R {
        SPMF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SPI read buffer full flag"]
    #[inline(always)]
    pub fn sprf(&self) -> SPRF_R {
        SPRF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "SPI status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s](index.html) module"]
pub struct S_SPEC;
impl crate::RegisterSpec for S_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [s::R](R) reader structure"]
impl crate::Readable for S_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets S to value 0x20"]
impl crate::Resettable for S_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
