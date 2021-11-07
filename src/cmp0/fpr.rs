#[doc = "Register `FPR` reader"]
pub struct R(crate::R<FPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPR` writer"]
pub struct W(crate::W<FPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPR_SPEC>;
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
impl From<crate::W<FPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FILT_PER` reader - Filter Sample Period"]
pub struct FILT_PER_R(crate::FieldReader<u8, u8>);
impl FILT_PER_R {
    pub(crate) fn new(bits: u8) -> Self {
        FILT_PER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILT_PER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILT_PER` writer - Filter Sample Period"]
pub struct FILT_PER_W<'a> {
    w: &'a mut W,
}
impl<'a> FILT_PER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u8 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Filter Sample Period"]
    #[inline(always)]
    pub fn filt_per(&self) -> FILT_PER_R {
        FILT_PER_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Filter Sample Period"]
    #[inline(always)]
    pub fn filt_per(&mut self) -> FILT_PER_W {
        FILT_PER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMP Filter Period Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpr](index.html) module"]
pub struct FPR_SPEC;
impl crate::RegisterSpec for FPR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fpr::R](R) reader structure"]
impl crate::Readable for FPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpr::W](W) writer structure"]
impl crate::Writable for FPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FPR to value 0"]
impl crate::Resettable for FPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
