#[doc = "Register `LOCKSTAT` reader"]
pub struct R(crate::R<LOCKSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOCKSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOCKSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOCKSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LOCKSTAT` reader - no description available"]
pub struct LOCKSTAT_R(crate::FieldReader<u32, u32>);
impl LOCKSTAT_R {
    pub(crate) fn new(bits: u32) -> Self {
        LOCKSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCKSTAT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn lockstat(&self) -> LOCKSTAT_R {
        LOCKSTAT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Lock Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockstat](index.html) module"]
pub struct LOCKSTAT_SPEC;
impl crate::RegisterSpec for LOCKSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lockstat::R](R) reader structure"]
impl crate::Readable for LOCKSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LOCKSTAT to value 0"]
impl crate::Resettable for LOCKSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
