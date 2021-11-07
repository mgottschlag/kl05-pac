#[doc = "Register `SYSACCESS` reader"]
pub struct R(crate::R<SYSACCESS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSACCESS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSACCESS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSACCESS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SYSACCESS` reader - no description available"]
pub struct SYSACCESS_R(crate::FieldReader<u32, u32>);
impl SYSACCESS_R {
    pub(crate) fn new(bits: u32) -> Self {
        SYSACCESS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSACCESS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn sysaccess(&self) -> SYSACCESS_R {
        SYSACCESS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "System Access Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysaccess](index.html) module"]
pub struct SYSACCESS_SPEC;
impl crate::RegisterSpec for SYSACCESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysaccess::R](R) reader structure"]
impl crate::Readable for SYSACCESS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYSACCESS to value 0x01"]
impl crate::Resettable for SYSACCESS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
