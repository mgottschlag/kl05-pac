#[doc = "Register `BASE` reader"]
pub struct R(crate::R<BASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BASEADDR` reader - no description available"]
pub struct BASEADDR_R(crate::FieldReader<u32, u32>);
impl BASEADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        BASEADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BASEADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn baseaddr(&self) -> BASEADDR_R {
        BASEADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "MTB Base Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [base](index.html) module"]
pub struct BASE_SPEC;
impl crate::RegisterSpec for BASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [base::R](R) reader structure"]
impl crate::Readable for BASE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BASE to value 0"]
impl crate::Resettable for BASE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
