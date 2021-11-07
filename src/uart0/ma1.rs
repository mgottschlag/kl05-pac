#[doc = "Register `MA1` reader"]
pub struct R(crate::R<MA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MA1` writer"]
pub struct W(crate::W<MA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MA1_SPEC>;
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
impl From<crate::W<MA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MA` reader - Match Address"]
pub struct MA_R(crate::FieldReader<u8, u8>);
impl MA_R {
    pub(crate) fn new(bits: u8) -> Self {
        MA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MA` writer - Match Address"]
pub struct MA_W<'a> {
    w: &'a mut W,
}
impl<'a> MA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u8 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Match Address"]
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Match Address"]
    #[inline(always)]
    pub fn ma(&mut self) -> MA_W {
        MA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Match Address Registers 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ma1](index.html) module"]
pub struct MA1_SPEC;
impl crate::RegisterSpec for MA1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ma1::R](R) reader structure"]
impl crate::Readable for MA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ma1::W](W) writer structure"]
impl crate::Writable for MA1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MA1 to value 0"]
impl crate::Resettable for MA1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
