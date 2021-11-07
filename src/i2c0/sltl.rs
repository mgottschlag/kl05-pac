#[doc = "Register `SLTL` reader"]
pub struct R(crate::R<SLTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLTL` writer"]
pub struct W(crate::W<SLTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SLTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSLT` reader - no description available"]
pub struct SSLT_R(crate::FieldReader<u8, u8>);
impl SSLT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SSLT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSLT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSLT` writer - no description available"]
pub struct SSLT_W<'a> {
    w: &'a mut W,
}
impl<'a> SSLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u8 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - no description available"]
    #[inline(always)]
    pub fn sslt(&self) -> SSLT_R {
        SSLT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - no description available"]
    #[inline(always)]
    pub fn sslt(&mut self) -> SSLT_W {
        SSLT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C SCL Low Timeout Register Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sltl](index.html) module"]
pub struct SLTL_SPEC;
impl crate::RegisterSpec for SLTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sltl::R](R) reader structure"]
impl crate::Readable for SLTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sltl::W](W) writer structure"]
impl crate::Writable for SLTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLTL to value 0"]
impl crate::Resettable for SLTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
