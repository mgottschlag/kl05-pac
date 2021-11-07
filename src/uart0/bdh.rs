#[doc = "Register `BDH` reader"]
pub struct R(crate::R<BDH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BDH` writer"]
pub struct W(crate::W<BDH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDH_SPEC>;
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
impl From<crate::W<BDH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SBR` reader - Baud Rate Modulo Divisor."]
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
#[doc = "Field `SBR` writer - Baud Rate Modulo Divisor."]
pub struct SBR_W<'a> {
    w: &'a mut W,
}
impl<'a> SBR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u8 & 0x1f);
        self.w
    }
}
#[doc = "Stop Bit Number Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBNS_A {
    #[doc = "0: One stop bit."]
    _0 = 0,
    #[doc = "1: Two stop bit."]
    _1 = 1,
}
impl From<SBNS_A> for bool {
    #[inline(always)]
    fn from(variant: SBNS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBNS` reader - Stop Bit Number Select"]
pub struct SBNS_R(crate::FieldReader<bool, SBNS_A>);
impl SBNS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SBNS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBNS_A {
        match self.bits {
            false => SBNS_A::_0,
            true => SBNS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SBNS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SBNS_A::_1
    }
}
impl core::ops::Deref for SBNS_R {
    type Target = crate::FieldReader<bool, SBNS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SBNS` writer - Stop Bit Number Select"]
pub struct SBNS_W<'a> {
    w: &'a mut W,
}
impl<'a> SBNS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SBNS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "One stop bit."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBNS_A::_0)
    }
    #[doc = "Two stop bit."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBNS_A::_1)
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
#[doc = "RX Input Active Edge Interrupt Enable (for RXEDGIF)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEDGIE_A {
    #[doc = "0: Hardware interrupts from UART _S2\\[RXEDGIF\\]
disabled (use polling)."]
    _0 = 0,
    #[doc = "1: Hardware interrupt requested when UART _S2\\[RXEDGIF\\]
flag is 1."]
    _1 = 1,
}
impl From<RXEDGIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXEDGIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXEDGIE` reader - RX Input Active Edge Interrupt Enable (for RXEDGIF)"]
pub struct RXEDGIE_R(crate::FieldReader<bool, RXEDGIE_A>);
impl RXEDGIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXEDGIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEDGIE_A {
        match self.bits {
            false => RXEDGIE_A::_0,
            true => RXEDGIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXEDGIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXEDGIE_A::_1
    }
}
impl core::ops::Deref for RXEDGIE_R {
    type Target = crate::FieldReader<bool, RXEDGIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXEDGIE` writer - RX Input Active Edge Interrupt Enable (for RXEDGIF)"]
pub struct RXEDGIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXEDGIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXEDGIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Hardware interrupts from UART _S2\\[RXEDGIF\\]
disabled (use polling)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXEDGIE_A::_0)
    }
    #[doc = "Hardware interrupt requested when UART _S2\\[RXEDGIF\\]
flag is 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXEDGIE_A::_1)
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
#[doc = "LIN Break Detect Interrupt Enable (for LBKDIF)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBKDIE_A {
    #[doc = "0: Hardware interrupts from UART _S2\\[LBKDIF\\]
disabled (use polling)."]
    _0 = 0,
    #[doc = "1: Hardware interrupt requested when UART _S2\\[LBKDIF\\]
flag is 1."]
    _1 = 1,
}
impl From<LBKDIE_A> for bool {
    #[inline(always)]
    fn from(variant: LBKDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBKDIE` reader - LIN Break Detect Interrupt Enable (for LBKDIF)"]
pub struct LBKDIE_R(crate::FieldReader<bool, LBKDIE_A>);
impl LBKDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LBKDIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBKDIE_A {
        match self.bits {
            false => LBKDIE_A::_0,
            true => LBKDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LBKDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LBKDIE_A::_1
    }
}
impl core::ops::Deref for LBKDIE_R {
    type Target = crate::FieldReader<bool, LBKDIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LBKDIE` writer - LIN Break Detect Interrupt Enable (for LBKDIF)"]
pub struct LBKDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LBKDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LBKDIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Hardware interrupts from UART _S2\\[LBKDIF\\]
disabled (use polling)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LBKDIE_A::_0)
    }
    #[doc = "Hardware interrupt requested when UART _S2\\[LBKDIF\\]
flag is 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LBKDIE_A::_1)
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
    #[doc = "Bits 0:4 - Baud Rate Modulo Divisor."]
    #[inline(always)]
    pub fn sbr(&self) -> SBR_R {
        SBR_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Stop Bit Number Select"]
    #[inline(always)]
    pub fn sbns(&self) -> SBNS_R {
        SBNS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RX Input Active Edge Interrupt Enable (for RXEDGIF)"]
    #[inline(always)]
    pub fn rxedgie(&self) -> RXEDGIE_R {
        RXEDGIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LIN Break Detect Interrupt Enable (for LBKDIF)"]
    #[inline(always)]
    pub fn lbkdie(&self) -> LBKDIE_R {
        LBKDIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Baud Rate Modulo Divisor."]
    #[inline(always)]
    pub fn sbr(&mut self) -> SBR_W {
        SBR_W { w: self }
    }
    #[doc = "Bit 5 - Stop Bit Number Select"]
    #[inline(always)]
    pub fn sbns(&mut self) -> SBNS_W {
        SBNS_W { w: self }
    }
    #[doc = "Bit 6 - RX Input Active Edge Interrupt Enable (for RXEDGIF)"]
    #[inline(always)]
    pub fn rxedgie(&mut self) -> RXEDGIE_W {
        RXEDGIE_W { w: self }
    }
    #[doc = "Bit 7 - LIN Break Detect Interrupt Enable (for LBKDIF)"]
    #[inline(always)]
    pub fn lbkdie(&mut self) -> LBKDIE_W {
        LBKDIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Baud Rate Register High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdh](index.html) module"]
pub struct BDH_SPEC;
impl crate::RegisterSpec for BDH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bdh::R](R) reader structure"]
impl crate::Readable for BDH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bdh::W](W) writer structure"]
impl crate::Writable for BDH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BDH to value 0"]
impl crate::Resettable for BDH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
