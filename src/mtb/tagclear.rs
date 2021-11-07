#[doc = "Register `TAGCLEAR` reader"]
pub struct R(crate::R<TAGCLEAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAGCLEAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAGCLEAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAGCLEAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TAGCLEAR` reader - no description available"]
pub struct TAGCLEAR_R(crate::FieldReader<u32, u32>);
impl TAGCLEAR_R {
    pub(crate) fn new(bits: u32) -> Self {
        TAGCLEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAGCLEAR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn tagclear(&self) -> TAGCLEAR_R {
        TAGCLEAR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Claim TAG Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tagclear](index.html) module"]
pub struct TAGCLEAR_SPEC;
impl crate::RegisterSpec for TAGCLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tagclear::R](R) reader structure"]
impl crate::Readable for TAGCLEAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TAGCLEAR to value 0"]
impl crate::Resettable for TAGCLEAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
