#[doc = "Register `C2` reader"]
pub struct R(crate::R<C2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2` writer"]
pub struct W(crate::W<C2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2_SPEC>;
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
impl From<crate::W<C2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SPI pin control 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPC0_A {
    #[doc = "0: SPI uses separate pins for data input and data output (pin mode is normal). In master mode of operation: MISO is master in and MOSI is master out. In slave mode of operation: MISO is slave out and MOSI is slave in."]
    _0 = 0,
    #[doc = "1: SPI configured for single-wire bidirectional operation (pin mode is bidirectional). In master mode of operation: MISO is not used by SPI; MOSI is master in when BIDIROE is 0 or master I/O when BIDIROE is 1. In slave mode of operation: MISO is slave in when BIDIROE is 0 or slave I/O when BIDIROE is 1; MOSI is not used by SPI."]
    _1 = 1,
}
impl From<SPC0_A> for bool {
    #[inline(always)]
    fn from(variant: SPC0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPC0` reader - SPI pin control 0"]
pub struct SPC0_R(crate::FieldReader<bool, SPC0_A>);
impl SPC0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPC0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPC0_A {
        match self.bits {
            false => SPC0_A::_0,
            true => SPC0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPC0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPC0_A::_1
    }
}
impl core::ops::Deref for SPC0_R {
    type Target = crate::FieldReader<bool, SPC0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPC0` writer - SPI pin control 0"]
pub struct SPC0_W<'a> {
    w: &'a mut W,
}
impl<'a> SPC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPC0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SPI uses separate pins for data input and data output (pin mode is normal). In master mode of operation: MISO is master in and MOSI is master out. In slave mode of operation: MISO is slave out and MOSI is slave in."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPC0_A::_0)
    }
    #[doc = "SPI configured for single-wire bidirectional operation (pin mode is bidirectional). In master mode of operation: MISO is not used by SPI; MOSI is master in when BIDIROE is 0 or master I/O when BIDIROE is 1. In slave mode of operation: MISO is slave in when BIDIROE is 0 or slave I/O when BIDIROE is 1; MOSI is not used by SPI."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPC0_A::_1)
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
#[doc = "SPI stop in wait mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPISWAI_A {
    #[doc = "0: SPI clocks continue to operate in wait mode"]
    _0 = 0,
    #[doc = "1: SPI clocks stop when the MCU enters wait mode"]
    _1 = 1,
}
impl From<SPISWAI_A> for bool {
    #[inline(always)]
    fn from(variant: SPISWAI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPISWAI` reader - SPI stop in wait mode"]
pub struct SPISWAI_R(crate::FieldReader<bool, SPISWAI_A>);
impl SPISWAI_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPISWAI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPISWAI_A {
        match self.bits {
            false => SPISWAI_A::_0,
            true => SPISWAI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPISWAI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPISWAI_A::_1
    }
}
impl core::ops::Deref for SPISWAI_R {
    type Target = crate::FieldReader<bool, SPISWAI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPISWAI` writer - SPI stop in wait mode"]
pub struct SPISWAI_W<'a> {
    w: &'a mut W,
}
impl<'a> SPISWAI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPISWAI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SPI clocks continue to operate in wait mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPISWAI_A::_0)
    }
    #[doc = "SPI clocks stop when the MCU enters wait mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPISWAI_A::_1)
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
#[doc = "Receive DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDMAE_A {
    #[doc = "0: DMA request for receive is disabled and interrupt from SPRF is allowed"]
    _0 = 0,
    #[doc = "1: DMA request for receive is enabled and interrupt from SPRF is disabled"]
    _1 = 1,
}
impl From<RXDMAE_A> for bool {
    #[inline(always)]
    fn from(variant: RXDMAE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDMAE` reader - Receive DMA enable"]
pub struct RXDMAE_R(crate::FieldReader<bool, RXDMAE_A>);
impl RXDMAE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXDMAE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDMAE_A {
        match self.bits {
            false => RXDMAE_A::_0,
            true => RXDMAE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXDMAE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXDMAE_A::_1
    }
}
impl core::ops::Deref for RXDMAE_R {
    type Target = crate::FieldReader<bool, RXDMAE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDMAE` writer - Receive DMA enable"]
pub struct RXDMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDMAE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXDMAE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA request for receive is disabled and interrupt from SPRF is allowed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXDMAE_A::_0)
    }
    #[doc = "DMA request for receive is enabled and interrupt from SPRF is disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXDMAE_A::_1)
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
#[doc = "Bidirectional mode output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIDIROE_A {
    #[doc = "0: Output driver disabled so SPI data I/O pin acts as an input"]
    _0 = 0,
    #[doc = "1: SPI I/O pin enabled as an output"]
    _1 = 1,
}
impl From<BIDIROE_A> for bool {
    #[inline(always)]
    fn from(variant: BIDIROE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIDIROE` reader - Bidirectional mode output enable"]
pub struct BIDIROE_R(crate::FieldReader<bool, BIDIROE_A>);
impl BIDIROE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BIDIROE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIDIROE_A {
        match self.bits {
            false => BIDIROE_A::_0,
            true => BIDIROE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BIDIROE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BIDIROE_A::_1
    }
}
impl core::ops::Deref for BIDIROE_R {
    type Target = crate::FieldReader<bool, BIDIROE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIDIROE` writer - Bidirectional mode output enable"]
pub struct BIDIROE_W<'a> {
    w: &'a mut W,
}
impl<'a> BIDIROE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIDIROE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Output driver disabled so SPI data I/O pin acts as an input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BIDIROE_A::_0)
    }
    #[doc = "SPI I/O pin enabled as an output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BIDIROE_A::_1)
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
#[doc = "Master mode-fault function enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODFEN_A {
    #[doc = "0: Mode fault function disabled, master SS pin reverts to general-purpose I/O not controlled by SPI"]
    _0 = 0,
    #[doc = "1: Mode fault function enabled, master SS pin acts as the mode fault input or the slave select output"]
    _1 = 1,
}
impl From<MODFEN_A> for bool {
    #[inline(always)]
    fn from(variant: MODFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODFEN` reader - Master mode-fault function enable"]
pub struct MODFEN_R(crate::FieldReader<bool, MODFEN_A>);
impl MODFEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MODFEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODFEN_A {
        match self.bits {
            false => MODFEN_A::_0,
            true => MODFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MODFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MODFEN_A::_1
    }
}
impl core::ops::Deref for MODFEN_R {
    type Target = crate::FieldReader<bool, MODFEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODFEN` writer - Master mode-fault function enable"]
pub struct MODFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MODFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODFEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mode fault function disabled, master SS pin reverts to general-purpose I/O not controlled by SPI"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODFEN_A::_0)
    }
    #[doc = "Mode fault function enabled, master SS pin acts as the mode fault input or the slave select output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODFEN_A::_1)
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
#[doc = "Transmit DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDMAE_A {
    #[doc = "0: DMA request for transmit is disabled and interrupt from SPTEF is allowed"]
    _0 = 0,
    #[doc = "1: DMA request for transmit is enabled and interrupt from SPTEF is disabled"]
    _1 = 1,
}
impl From<TXDMAE_A> for bool {
    #[inline(always)]
    fn from(variant: TXDMAE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDMAE` reader - Transmit DMA enable"]
pub struct TXDMAE_R(crate::FieldReader<bool, TXDMAE_A>);
impl TXDMAE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXDMAE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDMAE_A {
        match self.bits {
            false => TXDMAE_A::_0,
            true => TXDMAE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXDMAE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXDMAE_A::_1
    }
}
impl core::ops::Deref for TXDMAE_R {
    type Target = crate::FieldReader<bool, TXDMAE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDMAE` writer - Transmit DMA enable"]
pub struct TXDMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDMAE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXDMAE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA request for transmit is disabled and interrupt from SPTEF is allowed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXDMAE_A::_0)
    }
    #[doc = "DMA request for transmit is enabled and interrupt from SPTEF is disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXDMAE_A::_1)
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
#[doc = "SPI match interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPMIE_A {
    #[doc = "0: Interrupts from SPMF inhibited (use polling)"]
    _0 = 0,
    #[doc = "1: When SPMF is 1, requests a hardware interrupt"]
    _1 = 1,
}
impl From<SPMIE_A> for bool {
    #[inline(always)]
    fn from(variant: SPMIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPMIE` reader - SPI match interrupt enable"]
pub struct SPMIE_R(crate::FieldReader<bool, SPMIE_A>);
impl SPMIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPMIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPMIE_A {
        match self.bits {
            false => SPMIE_A::_0,
            true => SPMIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPMIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPMIE_A::_1
    }
}
impl core::ops::Deref for SPMIE_R {
    type Target = crate::FieldReader<bool, SPMIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPMIE` writer - SPI match interrupt enable"]
pub struct SPMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPMIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPMIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupts from SPMF inhibited (use polling)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPMIE_A::_0)
    }
    #[doc = "When SPMF is 1, requests a hardware interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPMIE_A::_1)
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
    #[doc = "Bit 0 - SPI pin control 0"]
    #[inline(always)]
    pub fn spc0(&self) -> SPC0_R {
        SPC0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SPI stop in wait mode"]
    #[inline(always)]
    pub fn spiswai(&self) -> SPISWAI_R {
        SPISWAI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive DMA enable"]
    #[inline(always)]
    pub fn rxdmae(&self) -> RXDMAE_R {
        RXDMAE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Bidirectional mode output enable"]
    #[inline(always)]
    pub fn bidiroe(&self) -> BIDIROE_R {
        BIDIROE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Master mode-fault function enable"]
    #[inline(always)]
    pub fn modfen(&self) -> MODFEN_R {
        MODFEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit DMA enable"]
    #[inline(always)]
    pub fn txdmae(&self) -> TXDMAE_R {
        TXDMAE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SPI match interrupt enable"]
    #[inline(always)]
    pub fn spmie(&self) -> SPMIE_R {
        SPMIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI pin control 0"]
    #[inline(always)]
    pub fn spc0(&mut self) -> SPC0_W {
        SPC0_W { w: self }
    }
    #[doc = "Bit 1 - SPI stop in wait mode"]
    #[inline(always)]
    pub fn spiswai(&mut self) -> SPISWAI_W {
        SPISWAI_W { w: self }
    }
    #[doc = "Bit 2 - Receive DMA enable"]
    #[inline(always)]
    pub fn rxdmae(&mut self) -> RXDMAE_W {
        RXDMAE_W { w: self }
    }
    #[doc = "Bit 3 - Bidirectional mode output enable"]
    #[inline(always)]
    pub fn bidiroe(&mut self) -> BIDIROE_W {
        BIDIROE_W { w: self }
    }
    #[doc = "Bit 4 - Master mode-fault function enable"]
    #[inline(always)]
    pub fn modfen(&mut self) -> MODFEN_W {
        MODFEN_W { w: self }
    }
    #[doc = "Bit 5 - Transmit DMA enable"]
    #[inline(always)]
    pub fn txdmae(&mut self) -> TXDMAE_W {
        TXDMAE_W { w: self }
    }
    #[doc = "Bit 7 - SPI match interrupt enable"]
    #[inline(always)]
    pub fn spmie(&mut self) -> SPMIE_W {
        SPMIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2](index.html) module"]
pub struct C2_SPEC;
impl crate::RegisterSpec for C2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c2::R](R) reader structure"]
impl crate::Readable for C2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2::W](W) writer structure"]
impl crate::Writable for C2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2 to value 0"]
impl crate::Resettable for C2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
