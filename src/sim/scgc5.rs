#[doc = "Register `SCGC5` reader"]
pub struct R(crate::R<SCGC5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCGC5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCGC5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCGC5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCGC5` writer"]
pub struct W(crate::W<SCGC5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCGC5_SPEC>;
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
impl From<crate::W<SCGC5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCGC5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Low Power Timer Access Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPTMR_A {
    #[doc = "0: Access disabled"]
    _0 = 0,
    #[doc = "1: Access enabled"]
    _1 = 1,
}
impl From<LPTMR_A> for bool {
    #[inline(always)]
    fn from(variant: LPTMR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPTMR` reader - Low Power Timer Access Control"]
pub struct LPTMR_R(crate::FieldReader<bool, LPTMR_A>);
impl LPTMR_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTMR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTMR_A {
        match self.bits {
            false => LPTMR_A::_0,
            true => LPTMR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LPTMR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LPTMR_A::_1
    }
}
impl core::ops::Deref for LPTMR_R {
    type Target = crate::FieldReader<bool, LPTMR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTMR` writer - Low Power Timer Access Control"]
pub struct LPTMR_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTMR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTMR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Access disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPTMR_A::_0)
    }
    #[doc = "Access enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPTMR_A::_1)
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
#[doc = "TSI Access Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSI_A {
    #[doc = "0: Access disabled"]
    _0 = 0,
    #[doc = "1: Access enabled"]
    _1 = 1,
}
impl From<TSI_A> for bool {
    #[inline(always)]
    fn from(variant: TSI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSI` reader - TSI Access Control"]
pub struct TSI_R(crate::FieldReader<bool, TSI_A>);
impl TSI_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSI_A {
        match self.bits {
            false => TSI_A::_0,
            true => TSI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TSI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TSI_A::_1
    }
}
impl core::ops::Deref for TSI_R {
    type Target = crate::FieldReader<bool, TSI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSI` writer - TSI Access Control"]
pub struct TSI_W<'a> {
    w: &'a mut W,
}
impl<'a> TSI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Access disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSI_A::_0)
    }
    #[doc = "Access enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSI_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Port A Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORTA_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<PORTA_A> for bool {
    #[inline(always)]
    fn from(variant: PORTA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORTA` reader - Port A Clock Gate Control"]
pub struct PORTA_R(crate::FieldReader<bool, PORTA_A>);
impl PORTA_R {
    pub(crate) fn new(bits: bool) -> Self {
        PORTA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORTA_A {
        match self.bits {
            false => PORTA_A::_0,
            true => PORTA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PORTA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PORTA_A::_1
    }
}
impl core::ops::Deref for PORTA_R {
    type Target = crate::FieldReader<bool, PORTA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORTA` writer - Port A Clock Gate Control"]
pub struct PORTA_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORTA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORTA_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORTA_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Port B Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORTB_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<PORTB_A> for bool {
    #[inline(always)]
    fn from(variant: PORTB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORTB` reader - Port B Clock Gate Control"]
pub struct PORTB_R(crate::FieldReader<bool, PORTB_A>);
impl PORTB_R {
    pub(crate) fn new(bits: bool) -> Self {
        PORTB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORTB_A {
        match self.bits {
            false => PORTB_A::_0,
            true => PORTB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PORTB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PORTB_A::_1
    }
}
impl core::ops::Deref for PORTB_R {
    type Target = crate::FieldReader<bool, PORTB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORTB` writer - Port B Clock Gate Control"]
pub struct PORTB_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORTB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORTB_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORTB_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Low Power Timer Access Control"]
    #[inline(always)]
    pub fn lptmr(&self) -> LPTMR_R {
        LPTMR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 5 - TSI Access Control"]
    #[inline(always)]
    pub fn tsi(&self) -> TSI_R {
        TSI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port A Clock Gate Control"]
    #[inline(always)]
    pub fn porta(&self) -> PORTA_R {
        PORTA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port B Clock Gate Control"]
    #[inline(always)]
    pub fn portb(&self) -> PORTB_R {
        PORTB_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Power Timer Access Control"]
    #[inline(always)]
    pub fn lptmr(&mut self) -> LPTMR_W {
        LPTMR_W { w: self }
    }
    #[doc = "Bit 5 - TSI Access Control"]
    #[inline(always)]
    pub fn tsi(&mut self) -> TSI_W {
        TSI_W { w: self }
    }
    #[doc = "Bit 9 - Port A Clock Gate Control"]
    #[inline(always)]
    pub fn porta(&mut self) -> PORTA_W {
        PORTA_W { w: self }
    }
    #[doc = "Bit 10 - Port B Clock Gate Control"]
    #[inline(always)]
    pub fn portb(&mut self) -> PORTB_W {
        PORTB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Clock Gating Control Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgc5](index.html) module"]
pub struct SCGC5_SPEC;
impl crate::RegisterSpec for SCGC5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scgc5::R](R) reader structure"]
impl crate::Readable for SCGC5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scgc5::W](W) writer structure"]
impl crate::Writable for SCGC5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCGC5 to value 0x0180"]
impl crate::Resettable for SCGC5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0180
    }
}
