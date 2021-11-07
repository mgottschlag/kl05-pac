#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DAC Buffer Read Pointer Bottom Position Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACBFRPBF_A {
    #[doc = "0: The DAC buffer read pointer is not equal to C2\\[DACBFUP\\]."]
    _0 = 0,
    #[doc = "1: The DAC buffer read pointer is equal to C2\\[DACBFUP\\]."]
    _1 = 1,
}
impl From<DACBFRPBF_A> for bool {
    #[inline(always)]
    fn from(variant: DACBFRPBF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACBFRPBF` reader - DAC Buffer Read Pointer Bottom Position Flag"]
pub struct DACBFRPBF_R(crate::FieldReader<bool, DACBFRPBF_A>);
impl DACBFRPBF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DACBFRPBF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACBFRPBF_A {
        match self.bits {
            false => DACBFRPBF_A::_0,
            true => DACBFRPBF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DACBFRPBF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DACBFRPBF_A::_1
    }
}
impl core::ops::Deref for DACBFRPBF_R {
    type Target = crate::FieldReader<bool, DACBFRPBF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACBFRPBF` writer - DAC Buffer Read Pointer Bottom Position Flag"]
pub struct DACBFRPBF_W<'a> {
    w: &'a mut W,
}
impl<'a> DACBFRPBF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACBFRPBF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DAC buffer read pointer is not equal to C2\\[DACBFUP\\]."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACBFRPBF_A::_0)
    }
    #[doc = "The DAC buffer read pointer is equal to C2\\[DACBFUP\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACBFRPBF_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "DAC Buffer Read Pointer Top Position Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACBFRPTF_A {
    #[doc = "0: The DAC buffer read pointer is not zero."]
    _0 = 0,
    #[doc = "1: The DAC buffer read pointer is zero."]
    _1 = 1,
}
impl From<DACBFRPTF_A> for bool {
    #[inline(always)]
    fn from(variant: DACBFRPTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACBFRPTF` reader - DAC Buffer Read Pointer Top Position Flag"]
pub struct DACBFRPTF_R(crate::FieldReader<bool, DACBFRPTF_A>);
impl DACBFRPTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DACBFRPTF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACBFRPTF_A {
        match self.bits {
            false => DACBFRPTF_A::_0,
            true => DACBFRPTF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DACBFRPTF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DACBFRPTF_A::_1
    }
}
impl core::ops::Deref for DACBFRPTF_R {
    type Target = crate::FieldReader<bool, DACBFRPTF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACBFRPTF` writer - DAC Buffer Read Pointer Top Position Flag"]
pub struct DACBFRPTF_W<'a> {
    w: &'a mut W,
}
impl<'a> DACBFRPTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACBFRPTF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DAC buffer read pointer is not zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACBFRPTF_A::_0)
    }
    #[doc = "The DAC buffer read pointer is zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACBFRPTF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DAC Buffer Read Pointer Bottom Position Flag"]
    #[inline(always)]
    pub fn dacbfrpbf(&self) -> DACBFRPBF_R {
        DACBFRPBF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DAC Buffer Read Pointer Top Position Flag"]
    #[inline(always)]
    pub fn dacbfrptf(&self) -> DACBFRPTF_R {
        DACBFRPTF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC Buffer Read Pointer Bottom Position Flag"]
    #[inline(always)]
    pub fn dacbfrpbf(&mut self) -> DACBFRPBF_W {
        DACBFRPBF_W { w: self }
    }
    #[doc = "Bit 1 - DAC Buffer Read Pointer Top Position Flag"]
    #[inline(always)]
    pub fn dacbfrptf(&mut self) -> DACBFRPTF_W {
        DACBFRPTF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR to value 0x02"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
