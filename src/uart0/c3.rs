#[doc = "Register `C3` reader"]
pub struct R(crate::R<C3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C3` writer"]
pub struct W(crate::W<C3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C3_SPEC>;
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
impl From<crate::W<C3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Parity Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEIE_A {
    #[doc = "0: PF interrupts disabled; use polling)."]
    _0 = 0,
    #[doc = "1: Hardware interrupt requested when PF is set."]
    _1 = 1,
}
impl From<PEIE_A> for bool {
    #[inline(always)]
    fn from(variant: PEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEIE` reader - Parity Error Interrupt Enable"]
pub struct PEIE_R(crate::FieldReader<bool, PEIE_A>);
impl PEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEIE_A {
        match self.bits {
            false => PEIE_A::_0,
            true => PEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PEIE_A::_1
    }
}
impl core::ops::Deref for PEIE_R {
    type Target = crate::FieldReader<bool, PEIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEIE` writer - Parity Error Interrupt Enable"]
pub struct PEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PF interrupts disabled; use polling)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEIE_A::_0)
    }
    #[doc = "Hardware interrupt requested when PF is set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEIE_A::_1)
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
#[doc = "Framing Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIE_A {
    #[doc = "0: FE interrupts disabled; use polling."]
    _0 = 0,
    #[doc = "1: Hardware interrupt requested when FE is set."]
    _1 = 1,
}
impl From<FEIE_A> for bool {
    #[inline(always)]
    fn from(variant: FEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEIE` reader - Framing Error Interrupt Enable"]
pub struct FEIE_R(crate::FieldReader<bool, FEIE_A>);
impl FEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FEIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEIE_A {
        match self.bits {
            false => FEIE_A::_0,
            true => FEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FEIE_A::_1
    }
}
impl core::ops::Deref for FEIE_R {
    type Target = crate::FieldReader<bool, FEIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEIE` writer - Framing Error Interrupt Enable"]
pub struct FEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FEIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FE interrupts disabled; use polling."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FEIE_A::_0)
    }
    #[doc = "Hardware interrupt requested when FE is set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FEIE_A::_1)
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
#[doc = "Noise Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEIE_A {
    #[doc = "0: NF interrupts disabled; use polling."]
    _0 = 0,
    #[doc = "1: Hardware interrupt requested when NF is set."]
    _1 = 1,
}
impl From<NEIE_A> for bool {
    #[inline(always)]
    fn from(variant: NEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NEIE` reader - Noise Error Interrupt Enable"]
pub struct NEIE_R(crate::FieldReader<bool, NEIE_A>);
impl NEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        NEIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NEIE_A {
        match self.bits {
            false => NEIE_A::_0,
            true => NEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == NEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == NEIE_A::_1
    }
}
impl core::ops::Deref for NEIE_R {
    type Target = crate::FieldReader<bool, NEIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NEIE` writer - Noise Error Interrupt Enable"]
pub struct NEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> NEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NEIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "NF interrupts disabled; use polling."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NEIE_A::_0)
    }
    #[doc = "Hardware interrupt requested when NF is set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NEIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "Overrun Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ORIE_A {
    #[doc = "0: OR interrupts disabled; use polling."]
    _0 = 0,
    #[doc = "1: Hardware interrupt requested when OR is set."]
    _1 = 1,
}
impl From<ORIE_A> for bool {
    #[inline(always)]
    fn from(variant: ORIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ORIE` reader - Overrun Interrupt Enable"]
pub struct ORIE_R(crate::FieldReader<bool, ORIE_A>);
impl ORIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ORIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ORIE_A {
        match self.bits {
            false => ORIE_A::_0,
            true => ORIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ORIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ORIE_A::_1
    }
}
impl core::ops::Deref for ORIE_R {
    type Target = crate::FieldReader<bool, ORIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ORIE` writer - Overrun Interrupt Enable"]
pub struct ORIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ORIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ORIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "OR interrupts disabled; use polling."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ORIE_A::_0)
    }
    #[doc = "Hardware interrupt requested when OR is set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ORIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u8 & 0x01) << 3);
        self.w
    }
}
#[doc = "Transmit Data Inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXINV_A {
    #[doc = "0: Transmit data not inverted."]
    _0 = 0,
    #[doc = "1: Transmit data inverted."]
    _1 = 1,
}
impl From<TXINV_A> for bool {
    #[inline(always)]
    fn from(variant: TXINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXINV` reader - Transmit Data Inversion"]
pub struct TXINV_R(crate::FieldReader<bool, TXINV_A>);
impl TXINV_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXINV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXINV_A {
        match self.bits {
            false => TXINV_A::_0,
            true => TXINV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXINV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXINV_A::_1
    }
}
impl core::ops::Deref for TXINV_R {
    type Target = crate::FieldReader<bool, TXINV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXINV` writer - Transmit Data Inversion"]
pub struct TXINV_W<'a> {
    w: &'a mut W,
}
impl<'a> TXINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXINV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmit data not inverted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXINV_A::_0)
    }
    #[doc = "Transmit data inverted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXINV_A::_1)
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
#[doc = "UART _TX Pin Direction in Single-Wire Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDIR_A {
    #[doc = "0: UART _TXD pin is an input in single-wire mode."]
    _0 = 0,
    #[doc = "1: UART _TXD pin is an output in single-wire mode."]
    _1 = 1,
}
impl From<TXDIR_A> for bool {
    #[inline(always)]
    fn from(variant: TXDIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDIR` reader - UART _TX Pin Direction in Single-Wire Mode"]
pub struct TXDIR_R(crate::FieldReader<bool, TXDIR_A>);
impl TXDIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXDIR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDIR_A {
        match self.bits {
            false => TXDIR_A::_0,
            true => TXDIR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXDIR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXDIR_A::_1
    }
}
impl core::ops::Deref for TXDIR_R {
    type Target = crate::FieldReader<bool, TXDIR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDIR` writer - UART _TX Pin Direction in Single-Wire Mode"]
pub struct TXDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXDIR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "UART _TXD pin is an input in single-wire mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXDIR_A::_0)
    }
    #[doc = "UART _TXD pin is an output in single-wire mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXDIR_A::_1)
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
#[doc = "Field `R9T8` reader - Receive Bit 9 / Transmit Bit 8"]
pub struct R9T8_R(crate::FieldReader<bool, bool>);
impl R9T8_R {
    pub(crate) fn new(bits: bool) -> Self {
        R9T8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R9T8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R9T8` writer - Receive Bit 9 / Transmit Bit 8"]
pub struct R9T8_W<'a> {
    w: &'a mut W,
}
impl<'a> R9T8_W<'a> {
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
#[doc = "Field `R8T9` reader - Receive Bit 8 / Transmit Bit 9"]
pub struct R8T9_R(crate::FieldReader<bool, bool>);
impl R8T9_R {
    pub(crate) fn new(bits: bool) -> Self {
        R8T9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R8T9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R8T9` writer - Receive Bit 8 / Transmit Bit 9"]
pub struct R8T9_W<'a> {
    w: &'a mut W,
}
impl<'a> R8T9_W<'a> {
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
    #[doc = "Bit 0 - Parity Error Interrupt Enable"]
    #[inline(always)]
    pub fn peie(&self) -> PEIE_R {
        PEIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Framing Error Interrupt Enable"]
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Noise Error Interrupt Enable"]
    #[inline(always)]
    pub fn neie(&self) -> NEIE_R {
        NEIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn orie(&self) -> ORIE_R {
        ORIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmit Data Inversion"]
    #[inline(always)]
    pub fn txinv(&self) -> TXINV_R {
        TXINV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - UART _TX Pin Direction in Single-Wire Mode"]
    #[inline(always)]
    pub fn txdir(&self) -> TXDIR_R {
        TXDIR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive Bit 9 / Transmit Bit 8"]
    #[inline(always)]
    pub fn r9t8(&self) -> R9T8_R {
        R9T8_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Receive Bit 8 / Transmit Bit 9"]
    #[inline(always)]
    pub fn r8t9(&self) -> R8T9_R {
        R8T9_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Error Interrupt Enable"]
    #[inline(always)]
    pub fn peie(&mut self) -> PEIE_W {
        PEIE_W { w: self }
    }
    #[doc = "Bit 1 - Framing Error Interrupt Enable"]
    #[inline(always)]
    pub fn feie(&mut self) -> FEIE_W {
        FEIE_W { w: self }
    }
    #[doc = "Bit 2 - Noise Error Interrupt Enable"]
    #[inline(always)]
    pub fn neie(&mut self) -> NEIE_W {
        NEIE_W { w: self }
    }
    #[doc = "Bit 3 - Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn orie(&mut self) -> ORIE_W {
        ORIE_W { w: self }
    }
    #[doc = "Bit 4 - Transmit Data Inversion"]
    #[inline(always)]
    pub fn txinv(&mut self) -> TXINV_W {
        TXINV_W { w: self }
    }
    #[doc = "Bit 5 - UART _TX Pin Direction in Single-Wire Mode"]
    #[inline(always)]
    pub fn txdir(&mut self) -> TXDIR_W {
        TXDIR_W { w: self }
    }
    #[doc = "Bit 6 - Receive Bit 9 / Transmit Bit 8"]
    #[inline(always)]
    pub fn r9t8(&mut self) -> R9T8_W {
        R9T8_W { w: self }
    }
    #[doc = "Bit 7 - Receive Bit 8 / Transmit Bit 9"]
    #[inline(always)]
    pub fn r8t9(&mut self) -> R8T9_W {
        R8T9_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c3](index.html) module"]
pub struct C3_SPEC;
impl crate::RegisterSpec for C3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c3::R](R) reader structure"]
impl crate::Readable for C3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c3::W](W) writer structure"]
impl crate::Writable for C3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C3 to value 0"]
impl crate::Resettable for C3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
