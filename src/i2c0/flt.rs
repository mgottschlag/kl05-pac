#[doc = "Register `FLT` reader"]
pub struct R(crate::R<FLT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLT` writer"]
pub struct W(crate::W<FLT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLT_SPEC>;
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
impl From<crate::W<FLT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "I2C Programmable Filter Factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLT_A {
    #[doc = "0: No filter/bypass"]
    _0 = 0,
}
impl From<FLT_A> for u8 {
    #[inline(always)]
    fn from(variant: FLT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLT` reader - I2C Programmable Filter Factor"]
pub struct FLT_R(crate::FieldReader<u8, FLT_A>);
impl FLT_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLT_A> {
        match self.bits {
            0 => Some(FLT_A::_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FLT_A::_0
    }
}
impl core::ops::Deref for FLT_R {
    type Target = crate::FieldReader<u8, FLT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT` writer - I2C Programmable Filter Factor"]
pub struct FLT_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No filter/bypass"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLT_A::_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u8 & 0x0f);
        self.w
    }
}
#[doc = "I2C Bus Start Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTF_A {
    #[doc = "0: No start happens on I2C bus"]
    _0 = 0,
    #[doc = "1: Start detected on I2C bus"]
    _1 = 1,
}
impl From<STARTF_A> for bool {
    #[inline(always)]
    fn from(variant: STARTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STARTF` reader - I2C Bus Start Detect Flag"]
pub struct STARTF_R(crate::FieldReader<bool, STARTF_A>);
impl STARTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        STARTF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STARTF_A {
        match self.bits {
            false => STARTF_A::_0,
            true => STARTF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STARTF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STARTF_A::_1
    }
}
impl core::ops::Deref for STARTF_R {
    type Target = crate::FieldReader<bool, STARTF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STARTF` writer - I2C Bus Start Detect Flag"]
pub struct STARTF_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STARTF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No start happens on I2C bus"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STARTF_A::_0)
    }
    #[doc = "Start detected on I2C bus"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STARTF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u8 & 0x01) << 4);
        self.w
    }
}
#[doc = "I2C Bus Stop or Start Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSIE_A {
    #[doc = "0: Stop or start detection interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Stop or start detection interrupt is enabled"]
    _1 = 1,
}
impl From<SSIE_A> for bool {
    #[inline(always)]
    fn from(variant: SSIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSIE` reader - I2C Bus Stop or Start Interrupt Enable"]
pub struct SSIE_R(crate::FieldReader<bool, SSIE_A>);
impl SSIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSIE_A {
        match self.bits {
            false => SSIE_A::_0,
            true => SSIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SSIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SSIE_A::_1
    }
}
impl core::ops::Deref for SSIE_R {
    type Target = crate::FieldReader<bool, SSIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSIE` writer - I2C Bus Stop or Start Interrupt Enable"]
pub struct SSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Stop or start detection interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSIE_A::_0)
    }
    #[doc = "Stop or start detection interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSIE_A::_1)
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
#[doc = "I2C Bus Stop Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPF_A {
    #[doc = "0: No stop happens on I2C bus"]
    _0 = 0,
    #[doc = "1: Stop detected on I2C bus"]
    _1 = 1,
}
impl From<STOPF_A> for bool {
    #[inline(always)]
    fn from(variant: STOPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPF` reader - I2C Bus Stop Detect Flag"]
pub struct STOPF_R(crate::FieldReader<bool, STOPF_A>);
impl STOPF_R {
    pub(crate) fn new(bits: bool) -> Self {
        STOPF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPF_A {
        match self.bits {
            false => STOPF_A::_0,
            true => STOPF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STOPF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STOPF_A::_1
    }
}
impl core::ops::Deref for STOPF_R {
    type Target = crate::FieldReader<bool, STOPF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOPF` writer - I2C Bus Stop Detect Flag"]
pub struct STOPF_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOPF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No stop happens on I2C bus"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STOPF_A::_0)
    }
    #[doc = "Stop detected on I2C bus"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STOPF_A::_1)
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
#[doc = "Stop Hold Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHEN_A {
    #[doc = "0: Stop holdoff is disabled. The MCU's entry to stop mode is not gated."]
    _0 = 0,
    #[doc = "1: Stop holdoff is enabled."]
    _1 = 1,
}
impl From<SHEN_A> for bool {
    #[inline(always)]
    fn from(variant: SHEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHEN` reader - Stop Hold Enable"]
pub struct SHEN_R(crate::FieldReader<bool, SHEN_A>);
impl SHEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SHEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHEN_A {
        match self.bits {
            false => SHEN_A::_0,
            true => SHEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SHEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SHEN_A::_1
    }
}
impl core::ops::Deref for SHEN_R {
    type Target = crate::FieldReader<bool, SHEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHEN` writer - Stop Hold Enable"]
pub struct SHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SHEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SHEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Stop holdoff is disabled. The MCU's entry to stop mode is not gated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHEN_A::_0)
    }
    #[doc = "Stop holdoff is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHEN_A::_1)
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
    #[doc = "Bits 0:3 - I2C Programmable Filter Factor"]
    #[inline(always)]
    pub fn flt(&self) -> FLT_R {
        FLT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - I2C Bus Start Detect Flag"]
    #[inline(always)]
    pub fn startf(&self) -> STARTF_R {
        STARTF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - I2C Bus Stop or Start Interrupt Enable"]
    #[inline(always)]
    pub fn ssie(&self) -> SSIE_R {
        SSIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - I2C Bus Stop Detect Flag"]
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Stop Hold Enable"]
    #[inline(always)]
    pub fn shen(&self) -> SHEN_R {
        SHEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - I2C Programmable Filter Factor"]
    #[inline(always)]
    pub fn flt(&mut self) -> FLT_W {
        FLT_W { w: self }
    }
    #[doc = "Bit 4 - I2C Bus Start Detect Flag"]
    #[inline(always)]
    pub fn startf(&mut self) -> STARTF_W {
        STARTF_W { w: self }
    }
    #[doc = "Bit 5 - I2C Bus Stop or Start Interrupt Enable"]
    #[inline(always)]
    pub fn ssie(&mut self) -> SSIE_W {
        SSIE_W { w: self }
    }
    #[doc = "Bit 6 - I2C Bus Stop Detect Flag"]
    #[inline(always)]
    pub fn stopf(&mut self) -> STOPF_W {
        STOPF_W { w: self }
    }
    #[doc = "Bit 7 - Stop Hold Enable"]
    #[inline(always)]
    pub fn shen(&mut self) -> SHEN_W {
        SHEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Programmable Input Glitch Filter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt](index.html) module"]
pub struct FLT_SPEC;
impl crate::RegisterSpec for FLT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [flt::R](R) reader structure"]
impl crate::Readable for FLT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flt::W](W) writer structure"]
impl crate::Writable for FLT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLT to value 0"]
impl crate::Resettable for FLT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
