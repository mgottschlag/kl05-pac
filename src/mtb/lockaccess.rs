#[doc = "Register `LOCKACCESS` reader"]
pub struct R(crate::R<LOCKACCESS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOCKACCESS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOCKACCESS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOCKACCESS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LOCKACCESS` reader - no description available"]
pub struct LOCKACCESS_R(crate::FieldReader<u32, u32>);
impl LOCKACCESS_R {
    pub(crate) fn new(bits: u32) -> Self {
        LOCKACCESS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCKACCESS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn lockaccess(&self) -> LOCKACCESS_R {
        LOCKACCESS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Lock Access Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockaccess](index.html) module"]
pub struct LOCKACCESS_SPEC;
impl crate::RegisterSpec for LOCKACCESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lockaccess::R](R) reader structure"]
impl crate::Readable for LOCKACCESS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LOCKACCESS to value 0"]
impl crate::Resettable for LOCKACCESS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
