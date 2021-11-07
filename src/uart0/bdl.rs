#[doc = "Register `BDL` reader"]
pub struct R(crate::R<BDL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BDL` writer"]
pub struct W(crate::W<BDL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDL_SPEC>;
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
impl From<crate::W<BDL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SBR` reader - Baud Rate Modulo Divisor"]
pub struct SBR_R(crate::FieldReader<u8, u8>);
impl SBR_R {
    pub(crate) fn new(bits: u8) -> Self {
        SBR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SBR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SBR` writer - Baud Rate Modulo Divisor"]
pub struct SBR_W<'a> {
    w: &'a mut W,
}
impl<'a> SBR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u8 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Baud Rate Modulo Divisor"]
    #[inline(always)]
    pub fn sbr(&self) -> SBR_R {
        SBR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Baud Rate Modulo Divisor"]
    #[inline(always)]
    pub fn sbr(&mut self) -> SBR_W {
        SBR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Baud Rate Register Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdl](index.html) module"]
pub struct BDL_SPEC;
impl crate::RegisterSpec for BDL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bdl::R](R) reader structure"]
impl crate::Readable for BDL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bdl::W](W) writer structure"]
impl crate::Writable for BDL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BDL to value 0x04"]
impl crate::Resettable for BDL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
