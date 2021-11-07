#[doc = "Register `SOPT1CFG` reader"]
pub struct R(crate::R<SOPT1CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOPT1CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOPT1CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOPT1CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "SOPT1 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sopt1cfg](index.html) module"]
pub struct SOPT1CFG_SPEC;
impl crate::RegisterSpec for SOPT1CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sopt1cfg::R](R) reader structure"]
impl crate::Readable for SOPT1CFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SOPT1CFG to value 0"]
impl crate::Resettable for SOPT1CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
