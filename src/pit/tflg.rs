#[doc = "Register `TFLG%s` reader"]
pub struct R(crate::R<TFLG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TFLG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TFLG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TFLG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TFLG%s` writer"]
pub struct W(crate::W<TFLG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TFLG_SPEC>;
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
impl From<crate::W<TFLG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TFLG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Timer Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIF_A {
    #[doc = "0: Timeout has not yet occurred."]
    _0 = 0,
    #[doc = "1: Timeout has occurred."]
    _1 = 1,
}
impl From<TIF_A> for bool {
    #[inline(always)]
    fn from(variant: TIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIF` reader - Timer Interrupt Flag"]
pub struct TIF_R(crate::FieldReader<bool, TIF_A>);
impl TIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIF_A {
        match self.bits {
            false => TIF_A::_0,
            true => TIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TIF_A::_1
    }
}
impl core::ops::Deref for TIF_R {
    type Target = crate::FieldReader<bool, TIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIF` writer - Timer Interrupt Flag"]
pub struct TIF_W<'a> {
    w: &'a mut W,
}
impl<'a> TIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timeout has not yet occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIF_A::_0)
    }
    #[doc = "Timeout has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIF_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Timer Interrupt Flag"]
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer Interrupt Flag"]
    #[inline(always)]
    pub fn tif(&mut self) -> TIF_W {
        TIF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tflg](index.html) module"]
pub struct TFLG_SPEC;
impl crate::RegisterSpec for TFLG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tflg::R](R) reader structure"]
impl crate::Readable for TFLG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tflg::W](W) writer structure"]
impl crate::Writable for TFLG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TFLG%s to value 0"]
impl crate::Resettable for TFLG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
