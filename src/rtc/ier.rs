#[doc = "Register `IER` reader"]
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Time Invalid Interrupt Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIIE_A {
    #[doc = "0: Time invalid flag does not generate an interrupt."]
    _0 = 0,
    #[doc = "1: Time invalid flag does generate an interrupt."]
    _1 = 1,
}
impl From<TIIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIIE` reader - Time Invalid Interrupt Enable"]
pub struct TIIE_R(crate::FieldReader<bool, TIIE_A>);
impl TIIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIIE_A {
        match self.bits {
            false => TIIE_A::_0,
            true => TIIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TIIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TIIE_A::_1
    }
}
impl core::ops::Deref for TIIE_R {
    type Target = crate::FieldReader<bool, TIIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIIE` writer - Time Invalid Interrupt Enable"]
pub struct TIIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Time invalid flag does not generate an interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIIE_A::_0)
    }
    #[doc = "Time invalid flag does generate an interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIIE_A::_1)
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
#[doc = "Time Overflow Interrupt Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOIE_A {
    #[doc = "0: Time overflow flag does not generate an interrupt."]
    _0 = 0,
    #[doc = "1: Time overflow flag does generate an interrupt."]
    _1 = 1,
}
impl From<TOIE_A> for bool {
    #[inline(always)]
    fn from(variant: TOIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOIE` reader - Time Overflow Interrupt Enable"]
pub struct TOIE_R(crate::FieldReader<bool, TOIE_A>);
impl TOIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOIE_A {
        match self.bits {
            false => TOIE_A::_0,
            true => TOIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TOIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TOIE_A::_1
    }
}
impl core::ops::Deref for TOIE_R {
    type Target = crate::FieldReader<bool, TOIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOIE` writer - Time Overflow Interrupt Enable"]
pub struct TOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Time overflow flag does not generate an interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOIE_A::_0)
    }
    #[doc = "Time overflow flag does generate an interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Time Alarm Interrupt Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAIE_A {
    #[doc = "0: Time alarm flag does not generate an interrupt."]
    _0 = 0,
    #[doc = "1: Time alarm flag does generate an interrupt."]
    _1 = 1,
}
impl From<TAIE_A> for bool {
    #[inline(always)]
    fn from(variant: TAIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAIE` reader - Time Alarm Interrupt Enable"]
pub struct TAIE_R(crate::FieldReader<bool, TAIE_A>);
impl TAIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAIE_A {
        match self.bits {
            false => TAIE_A::_0,
            true => TAIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TAIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TAIE_A::_1
    }
}
impl core::ops::Deref for TAIE_R {
    type Target = crate::FieldReader<bool, TAIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAIE` writer - Time Alarm Interrupt Enable"]
pub struct TAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Time alarm flag does not generate an interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TAIE_A::_0)
    }
    #[doc = "Time alarm flag does generate an interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TAIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Time Seconds Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSIE_A {
    #[doc = "0: Seconds interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Seconds interrupt is enabled."]
    _1 = 1,
}
impl From<TSIE_A> for bool {
    #[inline(always)]
    fn from(variant: TSIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSIE` reader - Time Seconds Interrupt Enable"]
pub struct TSIE_R(crate::FieldReader<bool, TSIE_A>);
impl TSIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSIE_A {
        match self.bits {
            false => TSIE_A::_0,
            true => TSIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TSIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TSIE_A::_1
    }
}
impl core::ops::Deref for TSIE_R {
    type Target = crate::FieldReader<bool, TSIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSIE` writer - Time Seconds Interrupt Enable"]
pub struct TSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Seconds interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSIE_A::_0)
    }
    #[doc = "Seconds interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Wakeup Pin On\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPON_A {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: If the wakeup pin is enabled, then the wakeup pin will assert."]
    _1 = 1,
}
impl From<WPON_A> for bool {
    #[inline(always)]
    fn from(variant: WPON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPON` reader - Wakeup Pin On"]
pub struct WPON_R(crate::FieldReader<bool, WPON_A>);
impl WPON_R {
    pub(crate) fn new(bits: bool) -> Self {
        WPON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPON_A {
        match self.bits {
            false => WPON_A::_0,
            true => WPON_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WPON_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WPON_A::_1
    }
}
impl core::ops::Deref for WPON_R {
    type Target = crate::FieldReader<bool, WPON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WPON` writer - Wakeup Pin On"]
pub struct WPON_W<'a> {
    w: &'a mut W,
}
impl<'a> WPON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WPON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WPON_A::_0)
    }
    #[doc = "If the wakeup pin is enabled, then the wakeup pin will assert."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WPON_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Time Invalid Interrupt Enable"]
    #[inline(always)]
    pub fn tiie(&self) -> TIIE_R {
        TIIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Time Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn toie(&self) -> TOIE_R {
        TOIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Time Alarm Interrupt Enable"]
    #[inline(always)]
    pub fn taie(&self) -> TAIE_R {
        TAIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Time Seconds Interrupt Enable"]
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Wakeup Pin On"]
    #[inline(always)]
    pub fn wpon(&self) -> WPON_R {
        WPON_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Time Invalid Interrupt Enable"]
    #[inline(always)]
    pub fn tiie(&mut self) -> TIIE_W {
        TIIE_W { w: self }
    }
    #[doc = "Bit 1 - Time Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn toie(&mut self) -> TOIE_W {
        TOIE_W { w: self }
    }
    #[doc = "Bit 2 - Time Alarm Interrupt Enable"]
    #[inline(always)]
    pub fn taie(&mut self) -> TAIE_W {
        TAIE_W { w: self }
    }
    #[doc = "Bit 4 - Time Seconds Interrupt Enable"]
    #[inline(always)]
    pub fn tsie(&mut self) -> TSIE_W {
        TSIE_W { w: self }
    }
    #[doc = "Bit 7 - Wakeup Pin On"]
    #[inline(always)]
    pub fn wpon(&mut self) -> WPON_W {
        WPON_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier::R](R) reader structure"]
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IER to value 0x07"]
impl crate::Resettable for IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
