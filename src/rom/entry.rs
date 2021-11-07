#[doc = "Register `ENTRY%s` reader"]
pub struct R(crate::R<ENTRY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENTRY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENTRY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENTRY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ENTRY` reader - ENTRY"]
pub struct ENTRY_R(crate::FieldReader<u32, u32>);
impl ENTRY_R {
    pub(crate) fn new(bits: u32) -> Self {
        ENTRY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENTRY_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - ENTRY"]
    #[inline(always)]
    pub fn entry(&self) -> ENTRY_R {
        ENTRY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Entry\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [entry](index.html) module"]
pub struct ENTRY_SPEC;
impl crate::RegisterSpec for ENTRY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [entry::R](R) reader structure"]
impl crate::Readable for ENTRY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ENTRY%s to value 0"]
impl crate::Resettable for ENTRY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
