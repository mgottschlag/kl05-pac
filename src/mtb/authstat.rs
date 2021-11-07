#[doc = "Register `AUTHSTAT` reader"]
pub struct R(crate::R<AUTHSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUTHSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUTHSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUTHSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BIT0` reader - no description available"]
pub struct BIT0_R(crate::FieldReader<bool, bool>);
impl BIT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        BIT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIT1` reader - no description available"]
pub struct BIT1_R(crate::FieldReader<bool, bool>);
impl BIT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        BIT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIT2` reader - no description available"]
pub struct BIT2_R(crate::FieldReader<bool, bool>);
impl BIT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        BIT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIT3` reader - no description available"]
pub struct BIT3_R(crate::FieldReader<bool, bool>);
impl BIT3_R {
    pub(crate) fn new(bits: bool) -> Self {
        BIT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bit0(&self) -> BIT0_R {
        BIT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bit1(&self) -> BIT1_R {
        BIT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bit2(&self) -> BIT2_R {
        BIT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bit3(&self) -> BIT3_R {
        BIT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
#[doc = "Authentication Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [authstat](index.html) module"]
pub struct AUTHSTAT_SPEC;
impl crate::RegisterSpec for AUTHSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [authstat::R](R) reader structure"]
impl crate::Readable for AUTHSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AUTHSTAT to value 0"]
impl crate::Resettable for AUTHSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
