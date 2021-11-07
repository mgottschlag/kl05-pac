#[doc = "Register `F` reader"]
pub struct R(crate::R<F_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<F_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<F_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<F_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `F` writer"]
pub struct W(crate::W<F_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<F_SPEC>;
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
impl From<crate::W<F_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<F_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICR` reader - ClockRate"]
pub struct ICR_R(crate::FieldReader<u8, u8>);
impl ICR_R {
    pub(crate) fn new(bits: u8) -> Self {
        ICR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICR` writer - ClockRate"]
pub struct ICR_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u8 & 0x3f);
        self.w
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MULT_A {
    #[doc = "0: mul = 1"]
    _00 = 0,
    #[doc = "1: mul = 2"]
    _01 = 1,
    #[doc = "2: mul = 4"]
    _10 = 2,
}
impl From<MULT_A> for u8 {
    #[inline(always)]
    fn from(variant: MULT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MULT` reader - no description available"]
pub struct MULT_R(crate::FieldReader<u8, MULT_A>);
impl MULT_R {
    pub(crate) fn new(bits: u8) -> Self {
        MULT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MULT_A> {
        match self.bits {
            0 => Some(MULT_A::_00),
            1 => Some(MULT_A::_01),
            2 => Some(MULT_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == MULT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == MULT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == MULT_A::_10
    }
}
impl core::ops::Deref for MULT_R {
    type Target = crate::FieldReader<u8, MULT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MULT` writer - no description available"]
pub struct MULT_W<'a> {
    w: &'a mut W,
}
impl<'a> MULT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MULT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "mul = 1"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(MULT_A::_00)
    }
    #[doc = "mul = 2"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(MULT_A::_01)
    }
    #[doc = "mul = 4"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(MULT_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u8 & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - ClockRate"]
    #[inline(always)]
    pub fn icr(&self) -> ICR_R {
        ICR_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - no description available"]
    #[inline(always)]
    pub fn mult(&self) -> MULT_R {
        MULT_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - ClockRate"]
    #[inline(always)]
    pub fn icr(&mut self) -> ICR_W {
        ICR_W { w: self }
    }
    #[doc = "Bits 6:7 - no description available"]
    #[inline(always)]
    pub fn mult(&mut self) -> MULT_W {
        MULT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Frequency Divider register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f](index.html) module"]
pub struct F_SPEC;
impl crate::RegisterSpec for F_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [f::R](R) reader structure"]
impl crate::Readable for F_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [f::W](W) writer structure"]
impl crate::Writable for F_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets F to value 0"]
impl crate::Resettable for F_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
