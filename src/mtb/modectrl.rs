#[doc = "Register `MODECTRL` reader"]
pub struct R(crate::R<MODECTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODECTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODECTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODECTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MODECTRL` reader - no description available"]
pub struct MODECTRL_R(crate::FieldReader<u32, u32>);
impl MODECTRL_R {
    pub(crate) fn new(bits: u32) -> Self {
        MODECTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODECTRL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn modectrl(&self) -> MODECTRL_R {
        MODECTRL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Integration Mode Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modectrl](index.html) module"]
pub struct MODECTRL_SPEC;
impl crate::RegisterSpec for MODECTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [modectrl::R](R) reader structure"]
impl crate::Readable for MODECTRL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MODECTRL to value 0"]
impl crate::Resettable for MODECTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
