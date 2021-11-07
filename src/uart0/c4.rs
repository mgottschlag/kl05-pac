#[doc = "Register `C4` reader"]
pub struct R(crate::R<C4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C4` writer"]
pub struct W(crate::W<C4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C4_SPEC>;
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
impl From<crate::W<C4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSR` reader - Over Sampling Ratio"]
pub struct OSR_R(crate::FieldReader<u8, u8>);
impl OSR_R {
    pub(crate) fn new(bits: u8) -> Self {
        OSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSR` writer - Over Sampling Ratio"]
pub struct OSR_W<'a> {
    w: &'a mut W,
}
impl<'a> OSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u8 & 0x1f);
        self.w
    }
}
#[doc = "10-bit Mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M10_A {
    #[doc = "0: Receiver and transmitter use 8-bit or 9-bit data characters."]
    _0 = 0,
    #[doc = "1: Receiver and transmitter use 10-bit data characters."]
    _1 = 1,
}
impl From<M10_A> for bool {
    #[inline(always)]
    fn from(variant: M10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `M10` reader - 10-bit Mode select"]
pub struct M10_R(crate::FieldReader<bool, M10_A>);
impl M10_R {
    pub(crate) fn new(bits: bool) -> Self {
        M10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M10_A {
        match self.bits {
            false => M10_A::_0,
            true => M10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == M10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == M10_A::_1
    }
}
impl core::ops::Deref for M10_R {
    type Target = crate::FieldReader<bool, M10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M10` writer - 10-bit Mode select"]
pub struct M10_W<'a> {
    w: &'a mut W,
}
impl<'a> M10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Receiver and transmitter use 8-bit or 9-bit data characters."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M10_A::_0)
    }
    #[doc = "Receiver and transmitter use 10-bit data characters."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M10_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u8 & 0x01) << 5);
        self.w
    }
}
#[doc = "Match Address Mode Enable 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAEN2_A {
    #[doc = "0: All data received is transferred to the data buffer if MAEN1 is cleared."]
    _0 = 0,
    #[doc = "1: All data received with the most significant bit cleared, is discarded. All data received with the most significant bit set, is compared with contents of MA2 register. If no match occurs, the data is discarded. If match occurs, data is transferred to the data buffer."]
    _1 = 1,
}
impl From<MAEN2_A> for bool {
    #[inline(always)]
    fn from(variant: MAEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAEN2` reader - Match Address Mode Enable 2"]
pub struct MAEN2_R(crate::FieldReader<bool, MAEN2_A>);
impl MAEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        MAEN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAEN2_A {
        match self.bits {
            false => MAEN2_A::_0,
            true => MAEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MAEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MAEN2_A::_1
    }
}
impl core::ops::Deref for MAEN2_R {
    type Target = crate::FieldReader<bool, MAEN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAEN2` writer - Match Address Mode Enable 2"]
pub struct MAEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> MAEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "All data received is transferred to the data buffer if MAEN1 is cleared."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MAEN2_A::_0)
    }
    #[doc = "All data received with the most significant bit cleared, is discarded. All data received with the most significant bit set, is compared with contents of MA2 register. If no match occurs, the data is discarded. If match occurs, data is transferred to the data buffer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MAEN2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
#[doc = "Match Address Mode Enable 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAEN1_A {
    #[doc = "0: All data received is transferred to the data buffer if MAEN2 is cleared."]
    _0 = 0,
    #[doc = "1: All data received with the most significant bit cleared, is discarded. All data received with the most significant bit set, is compared with contents of MA1 register. If no match occurs, the data is discarded. If match occurs, data is transferred to the data buffer."]
    _1 = 1,
}
impl From<MAEN1_A> for bool {
    #[inline(always)]
    fn from(variant: MAEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAEN1` reader - Match Address Mode Enable 1"]
pub struct MAEN1_R(crate::FieldReader<bool, MAEN1_A>);
impl MAEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        MAEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAEN1_A {
        match self.bits {
            false => MAEN1_A::_0,
            true => MAEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MAEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MAEN1_A::_1
    }
}
impl core::ops::Deref for MAEN1_R {
    type Target = crate::FieldReader<bool, MAEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAEN1` writer - Match Address Mode Enable 1"]
pub struct MAEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> MAEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "All data received is transferred to the data buffer if MAEN2 is cleared."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MAEN1_A::_0)
    }
    #[doc = "All data received with the most significant bit cleared, is discarded. All data received with the most significant bit set, is compared with contents of MA1 register. If no match occurs, the data is discarded. If match occurs, data is transferred to the data buffer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MAEN1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Over Sampling Ratio"]
    #[inline(always)]
    pub fn osr(&self) -> OSR_R {
        OSR_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - 10-bit Mode select"]
    #[inline(always)]
    pub fn m10(&self) -> M10_R {
        M10_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Match Address Mode Enable 2"]
    #[inline(always)]
    pub fn maen2(&self) -> MAEN2_R {
        MAEN2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Match Address Mode Enable 1"]
    #[inline(always)]
    pub fn maen1(&self) -> MAEN1_R {
        MAEN1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Over Sampling Ratio"]
    #[inline(always)]
    pub fn osr(&mut self) -> OSR_W {
        OSR_W { w: self }
    }
    #[doc = "Bit 5 - 10-bit Mode select"]
    #[inline(always)]
    pub fn m10(&mut self) -> M10_W {
        M10_W { w: self }
    }
    #[doc = "Bit 6 - Match Address Mode Enable 2"]
    #[inline(always)]
    pub fn maen2(&mut self) -> MAEN2_W {
        MAEN2_W { w: self }
    }
    #[doc = "Bit 7 - Match Address Mode Enable 1"]
    #[inline(always)]
    pub fn maen1(&mut self) -> MAEN1_W {
        MAEN1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Control Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c4](index.html) module"]
pub struct C4_SPEC;
impl crate::RegisterSpec for C4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c4::R](R) reader structure"]
impl crate::Readable for C4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c4::W](W) writer structure"]
impl crate::Writable for C4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C4 to value 0x0f"]
impl crate::Resettable for C4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
