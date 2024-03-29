#[doc = "Register `C3` reader"]
pub struct R(crate::R<C3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C3` writer"]
pub struct W(crate::W<C3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C3_SPEC>;
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
impl From<crate::W<C3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCTRIM` reader - Slow Internal Reference Clock Trim Setting"]
pub struct SCTRIM_R(crate::FieldReader<u8, u8>);
impl SCTRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCTRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCTRIM` writer - Slow Internal Reference Clock Trim Setting"]
pub struct SCTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SCTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u8 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Slow Internal Reference Clock Trim Setting"]
    #[inline(always)]
    pub fn sctrim(&self) -> SCTRIM_R {
        SCTRIM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Slow Internal Reference Clock Trim Setting"]
    #[inline(always)]
    pub fn sctrim(&mut self) -> SCTRIM_W {
        SCTRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCG Control 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c3](index.html) module"]
pub struct C3_SPEC;
impl crate::RegisterSpec for C3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c3::R](R) reader structure"]
impl crate::Readable for C3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c3::W](W) writer structure"]
impl crate::Writable for C3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C3 to value 0"]
impl crate::Resettable for C3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
