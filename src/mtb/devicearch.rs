#[doc = "Register `DEVICEARCH` reader"]
pub struct R(crate::R<DEVICEARCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVICEARCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVICEARCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVICEARCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DEVICEARCH` reader - no description available"]
pub struct DEVICEARCH_R(crate::FieldReader<u32, u32>);
impl DEVICEARCH_R {
    pub(crate) fn new(bits: u32) -> Self {
        DEVICEARCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEVICEARCH_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn devicearch(&self) -> DEVICEARCH_R {
        DEVICEARCH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Device Architecture Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devicearch](index.html) module"]
pub struct DEVICEARCH_SPEC;
impl crate::RegisterSpec for DEVICEARCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devicearch::R](R) reader structure"]
impl crate::Readable for DEVICEARCH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEVICEARCH to value 0x4770_0a31"]
impl crate::Resettable for DEVICEARCH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4770_0a31
    }
}
