#[doc = "Register `M` reader"]
pub struct R(crate::R<M_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M` writer"]
pub struct W(crate::W<M_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M_SPEC>;
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
impl From<crate::W<M_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Bits` reader - Hardware compare value (low byte)"]
pub struct BITS_R(crate::FieldReader<u8, u8>);
impl BITS_R {
    pub(crate) fn new(bits: u8) -> Self {
        BITS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BITS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Bits` writer - Hardware compare value (low byte)"]
pub struct BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> BITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u8 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Hardware compare value (low byte)"]
    #[inline(always)]
    pub fn bits_(&self) -> BITS_R {
        BITS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Hardware compare value (low byte)"]
    #[inline(always)]
    pub fn bits_(&mut self) -> BITS_W {
        BITS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI match register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m](index.html) module"]
pub struct M_SPEC;
impl crate::RegisterSpec for M_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [m::R](R) reader structure"]
impl crate::Readable for M_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m::W](W) writer structure"]
impl crate::Writable for M_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets M to value 0"]
impl crate::Resettable for M_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
