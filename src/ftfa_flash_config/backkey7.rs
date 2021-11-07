#[doc = "Register `BACKKEY7` reader"]
pub struct R(crate::R<BACKKEY7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BACKKEY7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BACKKEY7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BACKKEY7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `KEY` reader - Backdoor Comparison Key."]
pub struct KEY_R(crate::FieldReader<u8, u8>);
impl KEY_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Backdoor Comparison Key."]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Backdoor Comparison Key 7.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [backkey7](index.html) module"]
pub struct BACKKEY7_SPEC;
impl crate::RegisterSpec for BACKKEY7_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [backkey7::R](R) reader structure"]
impl crate::Readable for BACKKEY7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BACKKEY7 to value 0xff"]
impl crate::Resettable for BACKKEY7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}