#[doc = "Register `FPROT3` reader"]
pub struct R(crate::R<FPROT3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPROT3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPROT3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPROT3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PROT` reader - P-Flash Region Protect"]
pub struct PROT_R(crate::FieldReader<u8, u8>);
impl PROT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PROT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - P-Flash Region Protect"]
    #[inline(always)]
    pub fn prot(&self) -> PROT_R {
        PROT_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Non-volatile P-Flash Protection 1 - Low Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fprot3](index.html) module"]
pub struct FPROT3_SPEC;
impl crate::RegisterSpec for FPROT3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fprot3::R](R) reader structure"]
impl crate::Readable for FPROT3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FPROT3 to value 0xff"]
impl crate::Resettable for FPROT3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
