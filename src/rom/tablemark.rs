#[doc = "Register `TABLEMARK` reader"]
pub struct R(crate::R<TABLEMARK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TABLEMARK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TABLEMARK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TABLEMARK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MARK` reader - no description available"]
pub struct MARK_R(crate::FieldReader<u32, u32>);
impl MARK_R {
    pub(crate) fn new(bits: u32) -> Self {
        MARK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MARK_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn mark(&self) -> MARK_R {
        MARK_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "End of Table Marker Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tablemark](index.html) module"]
pub struct TABLEMARK_SPEC;
impl crate::RegisterSpec for TABLEMARK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tablemark::R](R) reader structure"]
impl crate::Readable for TABLEMARK_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TABLEMARK to value 0"]
impl crate::Resettable for TABLEMARK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
