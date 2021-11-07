#[doc = "Register `TSHD` reader"]
pub struct R(crate::R<TSHD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSHD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSHD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSHD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSHD` writer"]
pub struct W(crate::W<TSHD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSHD_SPEC>;
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
impl From<crate::W<TSHD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSHD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THRESL` reader - TSI Wakeup Channel Low-threshold"]
pub struct THRESL_R(crate::FieldReader<u16, u16>);
impl THRESL_R {
    pub(crate) fn new(bits: u16) -> Self {
        THRESL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THRESL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THRESL` writer - TSI Wakeup Channel Low-threshold"]
pub struct THRESL_W<'a> {
    w: &'a mut W,
}
impl<'a> THRESL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `THRESH` reader - TSI Wakeup Channel High-threshold"]
pub struct THRESH_R(crate::FieldReader<u16, u16>);
impl THRESH_R {
    pub(crate) fn new(bits: u16) -> Self {
        THRESH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THRESH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THRESH` writer - TSI Wakeup Channel High-threshold"]
pub struct THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> THRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - TSI Wakeup Channel Low-threshold"]
    #[inline(always)]
    pub fn thresl(&self) -> THRESL_R {
        THRESL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - TSI Wakeup Channel High-threshold"]
    #[inline(always)]
    pub fn thresh(&self) -> THRESH_R {
        THRESH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TSI Wakeup Channel Low-threshold"]
    #[inline(always)]
    pub fn thresl(&mut self) -> THRESL_W {
        THRESL_W { w: self }
    }
    #[doc = "Bits 16:31 - TSI Wakeup Channel High-threshold"]
    #[inline(always)]
    pub fn thresh(&mut self) -> THRESH_W {
        THRESH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TSI Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tshd](index.html) module"]
pub struct TSHD_SPEC;
impl crate::RegisterSpec for TSHD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tshd::R](R) reader structure"]
impl crate::Readable for TSHD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tshd::W](W) writer structure"]
impl crate::Writable for TSHD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSHD to value 0"]
impl crate::Resettable for TSHD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
