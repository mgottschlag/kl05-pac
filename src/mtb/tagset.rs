#[doc = "Register `TAGSET` reader"]
pub struct R(crate::R<TAGSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAGSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAGSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAGSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TAGSET` reader - no description available"]
pub struct TAGSET_R(crate::FieldReader<u32, u32>);
impl TAGSET_R {
    pub(crate) fn new(bits: u32) -> Self {
        TAGSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAGSET_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn tagset(&self) -> TAGSET_R {
        TAGSET_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Claim TAG Set Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tagset](index.html) module"]
pub struct TAGSET_SPEC;
impl crate::RegisterSpec for TAGSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tagset::R](R) reader structure"]
impl crate::Readable for TAGSET_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TAGSET to value 0"]
impl crate::Resettable for TAGSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
