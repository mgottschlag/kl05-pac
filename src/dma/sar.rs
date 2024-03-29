#[doc = "Register `SAR%s` reader"]
pub struct R(crate::R<SAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR%s` writer"]
pub struct W(crate::W<SAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_SPEC>;
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
impl From<crate::W<SAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR` reader - no description available"]
pub struct SAR_R(crate::FieldReader<u32, u32>);
impl SAR_R {
    pub(crate) fn new(bits: u32) -> Self {
        SAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR` writer - no description available"]
pub struct SAR_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn sar(&self) -> SAR_R {
        SAR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn sar(&mut self) -> SAR_W {
        SAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar](index.html) module"]
pub struct SAR_SPEC;
impl crate::RegisterSpec for SAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar::R](R) reader structure"]
impl crate::Readable for SAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar::W](W) writer structure"]
impl crate::Writable for SAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR%s to value 0"]
impl crate::Resettable for SAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
