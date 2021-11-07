#[doc = "Register `C5` reader"]
pub struct R(crate::R<C5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "MCG Control 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c5](index.html) module"]
pub struct C5_SPEC;
impl crate::RegisterSpec for C5_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c5::R](R) reader structure"]
impl crate::Readable for C5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets C5 to value 0"]
impl crate::Resettable for C5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
