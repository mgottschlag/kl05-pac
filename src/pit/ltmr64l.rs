#[doc = "Register `LTMR64L` reader"]
pub struct R(crate::R<LTMR64L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTMR64L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTMR64L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTMR64L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LTL` reader - Life Timer value"]
pub struct LTL_R(crate::FieldReader<u32, u32>);
impl LTL_R {
    pub(crate) fn new(bits: u32) -> Self {
        LTL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LTL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Life Timer value"]
    #[inline(always)]
    pub fn ltl(&self) -> LTL_R {
        LTL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "PIT Lower Lifetime Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltmr64l](index.html) module"]
pub struct LTMR64L_SPEC;
impl crate::RegisterSpec for LTMR64L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltmr64l::R](R) reader structure"]
impl crate::Readable for LTMR64L_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LTMR64L to value 0"]
impl crate::Resettable for LTMR64L_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
