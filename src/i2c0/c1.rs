#[doc = "Register `C1` reader"]
pub struct R(crate::R<C1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1` writer"]
pub struct W(crate::W<C1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1_SPEC>;
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
impl From<crate::W<C1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN_A {
    #[doc = "0: All DMA signalling disabled."]
    _0 = 0,
    #[doc = "1: DMA transfer is enabled and the following conditions trigger the DMA request: While FACK = 0, a data byte is received, either address or data is transmitted. (ACK/NACK automatic) While FACK = 0, the first byte received matches the A1 register or is general call address. If any address matching occurs, IAAS and TCF are set. If the direction of transfer is known from master to slave, then it is not required to check the SRW. With this assumption, DMA can also be used in this case. In other cases, if the master reads data from the slave, then it is required to rewrite the C1 register operation. With this assumption, DMA cannot be used. When FACK = 1, an address or a data byte is transmitted."]
    _1 = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - DMA Enable"]
pub struct DMAEN_R(crate::FieldReader<bool, DMAEN_A>);
impl DMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::_0,
            true => DMAEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DMAEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DMAEN_A::_1
    }
}
impl core::ops::Deref for DMAEN_R {
    type Target = crate::FieldReader<bool, DMAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAEN` writer - DMA Enable"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "All DMA signalling disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAEN_A::_0)
    }
    #[doc = "DMA transfer is enabled and the following conditions trigger the DMA request: While FACK = 0, a data byte is received, either address or data is transmitted. (ACK/NACK automatic) While FACK = 0, the first byte received matches the A1 register or is general call address. If any address matching occurs, IAAS and TCF are set. If the direction of transfer is known from master to slave, then it is not required to check the SRW. With this assumption, DMA can also be used in this case. In other cases, if the master reads data from the slave, then it is required to rewrite the C1 register operation. With this assumption, DMA cannot be used. When FACK = 1, an address or a data byte is transmitted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAEN_A::_1)
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
#[doc = "Wakeup Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUEN_A {
    #[doc = "0: Normal operation. No interrupt generated when address matching in low power mode."]
    _0 = 0,
    #[doc = "1: Enables the wakeup function in low power mode."]
    _1 = 1,
}
impl From<WUEN_A> for bool {
    #[inline(always)]
    fn from(variant: WUEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUEN` reader - Wakeup Enable"]
pub struct WUEN_R(crate::FieldReader<bool, WUEN_A>);
impl WUEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUEN_A {
        match self.bits {
            false => WUEN_A::_0,
            true => WUEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WUEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WUEN_A::_1
    }
}
impl core::ops::Deref for WUEN_R {
    type Target = crate::FieldReader<bool, WUEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUEN` writer - Wakeup Enable"]
pub struct WUEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WUEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal operation. No interrupt generated when address matching in low power mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUEN_A::_0)
    }
    #[doc = "Enables the wakeup function in low power mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUEN_A::_1)
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
#[doc = "Field `RSTA` writer - Repeat START"]
pub struct RSTA_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTA_W<'a> {
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
#[doc = "Transmit Acknowledge Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXAK_A {
    #[doc = "0: An acknowledge signal is sent to the bus on the following receiving byte (if FACK is cleared) or the current receiving byte (if FACK is set)."]
    _0 = 0,
    #[doc = "1: No acknowledge signal is sent to the bus on the following receiving data byte (if FACK is cleared) or the current receiving data byte (if FACK is set)."]
    _1 = 1,
}
impl From<TXAK_A> for bool {
    #[inline(always)]
    fn from(variant: TXAK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXAK` reader - Transmit Acknowledge Enable"]
pub struct TXAK_R(crate::FieldReader<bool, TXAK_A>);
impl TXAK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXAK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXAK_A {
        match self.bits {
            false => TXAK_A::_0,
            true => TXAK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXAK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXAK_A::_1
    }
}
impl core::ops::Deref for TXAK_R {
    type Target = crate::FieldReader<bool, TXAK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXAK` writer - Transmit Acknowledge Enable"]
pub struct TXAK_W<'a> {
    w: &'a mut W,
}
impl<'a> TXAK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXAK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "An acknowledge signal is sent to the bus on the following receiving byte (if FACK is cleared) or the current receiving byte (if FACK is set)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXAK_A::_0)
    }
    #[doc = "No acknowledge signal is sent to the bus on the following receiving data byte (if FACK is cleared) or the current receiving data byte (if FACK is set)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXAK_A::_1)
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
#[doc = "Transmit Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_A {
    #[doc = "0: Receive"]
    _0 = 0,
    #[doc = "1: Transmit"]
    _1 = 1,
}
impl From<TX_A> for bool {
    #[inline(always)]
    fn from(variant: TX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX` reader - Transmit Mode Select"]
pub struct TX_R(crate::FieldReader<bool, TX_A>);
impl TX_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_A {
        match self.bits {
            false => TX_A::_0,
            true => TX_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TX_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TX_A::_1
    }
}
impl core::ops::Deref for TX_R {
    type Target = crate::FieldReader<bool, TX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX` writer - Transmit Mode Select"]
pub struct TX_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Receive"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TX_A::_0)
    }
    #[doc = "Transmit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TX_A::_1)
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
#[doc = "Master Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MST_A {
    #[doc = "0: Slave mode"]
    _0 = 0,
    #[doc = "1: Master mode"]
    _1 = 1,
}
impl From<MST_A> for bool {
    #[inline(always)]
    fn from(variant: MST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MST` reader - Master Mode Select"]
pub struct MST_R(crate::FieldReader<bool, MST_A>);
impl MST_R {
    pub(crate) fn new(bits: bool) -> Self {
        MST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MST_A {
        match self.bits {
            false => MST_A::_0,
            true => MST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MST_A::_1
    }
}
impl core::ops::Deref for MST_R {
    type Target = crate::FieldReader<bool, MST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MST` writer - Master Mode Select"]
pub struct MST_W<'a> {
    w: &'a mut W,
}
impl<'a> MST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Slave mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MST_A::_0)
    }
    #[doc = "Master mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MST_A::_1)
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
#[doc = "I2C Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IICIE_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<IICIE_A> for bool {
    #[inline(always)]
    fn from(variant: IICIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IICIE` reader - I2C Interrupt Enable"]
pub struct IICIE_R(crate::FieldReader<bool, IICIE_A>);
impl IICIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IICIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IICIE_A {
        match self.bits {
            false => IICIE_A::_0,
            true => IICIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IICIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IICIE_A::_1
    }
}
impl core::ops::Deref for IICIE_R {
    type Target = crate::FieldReader<bool, IICIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IICIE` writer - I2C Interrupt Enable"]
pub struct IICIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IICIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IICIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IICIE_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IICIE_A::_1)
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
#[doc = "I2C Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IICEN_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<IICEN_A> for bool {
    #[inline(always)]
    fn from(variant: IICEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IICEN` reader - I2C Enable"]
pub struct IICEN_R(crate::FieldReader<bool, IICEN_A>);
impl IICEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IICEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IICEN_A {
        match self.bits {
            false => IICEN_A::_0,
            true => IICEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IICEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IICEN_A::_1
    }
}
impl core::ops::Deref for IICEN_R {
    type Target = crate::FieldReader<bool, IICEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IICEN` writer - I2C Enable"]
pub struct IICEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IICEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IICEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IICEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IICEN_A::_1)
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
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wakeup Enable"]
    #[inline(always)]
    pub fn wuen(&self) -> WUEN_R {
        WUEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit Acknowledge Enable"]
    #[inline(always)]
    pub fn txak(&self) -> TXAK_R {
        TXAK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmit Mode Select"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Master Mode Select"]
    #[inline(always)]
    pub fn mst(&self) -> MST_R {
        MST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - I2C Interrupt Enable"]
    #[inline(always)]
    pub fn iicie(&self) -> IICIE_R {
        IICIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - I2C Enable"]
    #[inline(always)]
    pub fn iicen(&self) -> IICEN_R {
        IICEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Bit 1 - Wakeup Enable"]
    #[inline(always)]
    pub fn wuen(&mut self) -> WUEN_W {
        WUEN_W { w: self }
    }
    #[doc = "Bit 2 - Repeat START"]
    #[inline(always)]
    pub fn rsta(&mut self) -> RSTA_W {
        RSTA_W { w: self }
    }
    #[doc = "Bit 3 - Transmit Acknowledge Enable"]
    #[inline(always)]
    pub fn txak(&mut self) -> TXAK_W {
        TXAK_W { w: self }
    }
    #[doc = "Bit 4 - Transmit Mode Select"]
    #[inline(always)]
    pub fn tx(&mut self) -> TX_W {
        TX_W { w: self }
    }
    #[doc = "Bit 5 - Master Mode Select"]
    #[inline(always)]
    pub fn mst(&mut self) -> MST_W {
        MST_W { w: self }
    }
    #[doc = "Bit 6 - I2C Interrupt Enable"]
    #[inline(always)]
    pub fn iicie(&mut self) -> IICIE_W {
        IICIE_W { w: self }
    }
    #[doc = "Bit 7 - I2C Enable"]
    #[inline(always)]
    pub fn iicen(&mut self) -> IICEN_W {
        IICEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1](index.html) module"]
pub struct C1_SPEC;
impl crate::RegisterSpec for C1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c1::R](R) reader structure"]
impl crate::Readable for C1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1::W](W) writer structure"]
impl crate::Writable for C1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C1 to value 0"]
impl crate::Resettable for C1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
