#[doc = "Register `SMB` reader"]
pub struct R(crate::R<SMB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMB` writer"]
pub struct W(crate::W<SMB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMB_SPEC>;
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
impl From<crate::W<SMB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SHTF2 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHTF2IE_A {
    #[doc = "0: SHTF2 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: SHTF2 interrupt is enabled"]
    _1 = 1,
}
impl From<SHTF2IE_A> for bool {
    #[inline(always)]
    fn from(variant: SHTF2IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHTF2IE` reader - SHTF2 Interrupt Enable"]
pub struct SHTF2IE_R(crate::FieldReader<bool, SHTF2IE_A>);
impl SHTF2IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SHTF2IE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHTF2IE_A {
        match self.bits {
            false => SHTF2IE_A::_0,
            true => SHTF2IE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SHTF2IE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SHTF2IE_A::_1
    }
}
impl core::ops::Deref for SHTF2IE_R {
    type Target = crate::FieldReader<bool, SHTF2IE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHTF2IE` writer - SHTF2 Interrupt Enable"]
pub struct SHTF2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> SHTF2IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SHTF2IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SHTF2 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHTF2IE_A::_0)
    }
    #[doc = "SHTF2 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHTF2IE_A::_1)
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
#[doc = "SCL High Timeout Flag 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHTF2_A {
    #[doc = "0: No SCL high and SDA low timeout occurs"]
    _0 = 0,
    #[doc = "1: SCL high and SDA low timeout occurs"]
    _1 = 1,
}
impl From<SHTF2_A> for bool {
    #[inline(always)]
    fn from(variant: SHTF2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHTF2` reader - SCL High Timeout Flag 2"]
pub struct SHTF2_R(crate::FieldReader<bool, SHTF2_A>);
impl SHTF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SHTF2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHTF2_A {
        match self.bits {
            false => SHTF2_A::_0,
            true => SHTF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SHTF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SHTF2_A::_1
    }
}
impl core::ops::Deref for SHTF2_R {
    type Target = crate::FieldReader<bool, SHTF2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHTF2` writer - SCL High Timeout Flag 2"]
pub struct SHTF2_W<'a> {
    w: &'a mut W,
}
impl<'a> SHTF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SHTF2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No SCL high and SDA low timeout occurs"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHTF2_A::_0)
    }
    #[doc = "SCL high and SDA low timeout occurs"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHTF2_A::_1)
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
#[doc = "SCL High Timeout Flag 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHTF1_A {
    #[doc = "0: No SCL high and SDA high timeout occurs"]
    _0 = 0,
    #[doc = "1: SCL high and SDA high timeout occurs"]
    _1 = 1,
}
impl From<SHTF1_A> for bool {
    #[inline(always)]
    fn from(variant: SHTF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHTF1` reader - SCL High Timeout Flag 1"]
pub struct SHTF1_R(crate::FieldReader<bool, SHTF1_A>);
impl SHTF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SHTF1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHTF1_A {
        match self.bits {
            false => SHTF1_A::_0,
            true => SHTF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SHTF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SHTF1_A::_1
    }
}
impl core::ops::Deref for SHTF1_R {
    type Target = crate::FieldReader<bool, SHTF1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "SCL Low Timeout Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLTF_A {
    #[doc = "0: No low timeout occurs"]
    _0 = 0,
    #[doc = "1: Low timeout occurs"]
    _1 = 1,
}
impl From<SLTF_A> for bool {
    #[inline(always)]
    fn from(variant: SLTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLTF` reader - SCL Low Timeout Flag"]
pub struct SLTF_R(crate::FieldReader<bool, SLTF_A>);
impl SLTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLTF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLTF_A {
        match self.bits {
            false => SLTF_A::_0,
            true => SLTF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SLTF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SLTF_A::_1
    }
}
impl core::ops::Deref for SLTF_R {
    type Target = crate::FieldReader<bool, SLTF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLTF` writer - SCL Low Timeout Flag"]
pub struct SLTF_W<'a> {
    w: &'a mut W,
}
impl<'a> SLTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLTF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No low timeout occurs"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLTF_A::_0)
    }
    #[doc = "Low timeout occurs"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLTF_A::_1)
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
#[doc = "Timeout Counter Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCKSEL_A {
    #[doc = "0: Timeout counter counts at the frequency of the bus clock / 64"]
    _0 = 0,
    #[doc = "1: Timeout counter counts at the frequency of the bus clock"]
    _1 = 1,
}
impl From<TCKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: TCKSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCKSEL` reader - Timeout Counter Clock Select"]
pub struct TCKSEL_R(crate::FieldReader<bool, TCKSEL_A>);
impl TCKSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCKSEL_A {
        match self.bits {
            false => TCKSEL_A::_0,
            true => TCKSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TCKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TCKSEL_A::_1
    }
}
impl core::ops::Deref for TCKSEL_R {
    type Target = crate::FieldReader<bool, TCKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCKSEL` writer - Timeout Counter Clock Select"]
pub struct TCKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TCKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCKSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timeout counter counts at the frequency of the bus clock / 64"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCKSEL_A::_0)
    }
    #[doc = "Timeout counter counts at the frequency of the bus clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCKSEL_A::_1)
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
#[doc = "Second I2C Address Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIICAEN_A {
    #[doc = "0: I2C address register 2 matching is disabled"]
    _0 = 0,
    #[doc = "1: I2C address register 2 matching is enabled"]
    _1 = 1,
}
impl From<SIICAEN_A> for bool {
    #[inline(always)]
    fn from(variant: SIICAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIICAEN` reader - Second I2C Address Enable"]
pub struct SIICAEN_R(crate::FieldReader<bool, SIICAEN_A>);
impl SIICAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SIICAEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIICAEN_A {
        match self.bits {
            false => SIICAEN_A::_0,
            true => SIICAEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SIICAEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SIICAEN_A::_1
    }
}
impl core::ops::Deref for SIICAEN_R {
    type Target = crate::FieldReader<bool, SIICAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIICAEN` writer - Second I2C Address Enable"]
pub struct SIICAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SIICAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIICAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "I2C address register 2 matching is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SIICAEN_A::_0)
    }
    #[doc = "I2C address register 2 matching is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SIICAEN_A::_1)
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
#[doc = "SMBus Alert Response Address Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALERTEN_A {
    #[doc = "0: SMBus alert response address matching is disabled"]
    _0 = 0,
    #[doc = "1: SMBus alert response address matching is enabled"]
    _1 = 1,
}
impl From<ALERTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ALERTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALERTEN` reader - SMBus Alert Response Address Enable"]
pub struct ALERTEN_R(crate::FieldReader<bool, ALERTEN_A>);
impl ALERTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALERTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALERTEN_A {
        match self.bits {
            false => ALERTEN_A::_0,
            true => ALERTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ALERTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ALERTEN_A::_1
    }
}
impl core::ops::Deref for ALERTEN_R {
    type Target = crate::FieldReader<bool, ALERTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALERTEN` writer - SMBus Alert Response Address Enable"]
pub struct ALERTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALERTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALERTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SMBus alert response address matching is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALERTEN_A::_0)
    }
    #[doc = "SMBus alert response address matching is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALERTEN_A::_1)
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
#[doc = "Fast NACK/ACK Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FACK_A {
    #[doc = "0: An ACK or NACK is sent on the following receiving data byte"]
    _0 = 0,
    #[doc = "1: Writing 0 to TXAK after receiving a data byte generates an ACK. Writing 1 to TXAK after receiving a data byte generates a NACK."]
    _1 = 1,
}
impl From<FACK_A> for bool {
    #[inline(always)]
    fn from(variant: FACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FACK` reader - Fast NACK/ACK Enable"]
pub struct FACK_R(crate::FieldReader<bool, FACK_A>);
impl FACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        FACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FACK_A {
        match self.bits {
            false => FACK_A::_0,
            true => FACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FACK_A::_1
    }
}
impl core::ops::Deref for FACK_R {
    type Target = crate::FieldReader<bool, FACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FACK` writer - Fast NACK/ACK Enable"]
pub struct FACK_W<'a> {
    w: &'a mut W,
}
impl<'a> FACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FACK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "An ACK or NACK is sent on the following receiving data byte"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FACK_A::_0)
    }
    #[doc = "Writing 0 to TXAK after receiving a data byte generates an ACK. Writing 1 to TXAK after receiving a data byte generates a NACK."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FACK_A::_1)
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
    #[doc = "Bit 0 - SHTF2 Interrupt Enable"]
    #[inline(always)]
    pub fn shtf2ie(&self) -> SHTF2IE_R {
        SHTF2IE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SCL High Timeout Flag 2"]
    #[inline(always)]
    pub fn shtf2(&self) -> SHTF2_R {
        SHTF2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SCL High Timeout Flag 1"]
    #[inline(always)]
    pub fn shtf1(&self) -> SHTF1_R {
        SHTF1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SCL Low Timeout Flag"]
    #[inline(always)]
    pub fn sltf(&self) -> SLTF_R {
        SLTF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Timeout Counter Clock Select"]
    #[inline(always)]
    pub fn tcksel(&self) -> TCKSEL_R {
        TCKSEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Second I2C Address Enable"]
    #[inline(always)]
    pub fn siicaen(&self) -> SIICAEN_R {
        SIICAEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SMBus Alert Response Address Enable"]
    #[inline(always)]
    pub fn alerten(&self) -> ALERTEN_R {
        ALERTEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Fast NACK/ACK Enable"]
    #[inline(always)]
    pub fn fack(&self) -> FACK_R {
        FACK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SHTF2 Interrupt Enable"]
    #[inline(always)]
    pub fn shtf2ie(&mut self) -> SHTF2IE_W {
        SHTF2IE_W { w: self }
    }
    #[doc = "Bit 1 - SCL High Timeout Flag 2"]
    #[inline(always)]
    pub fn shtf2(&mut self) -> SHTF2_W {
        SHTF2_W { w: self }
    }
    #[doc = "Bit 3 - SCL Low Timeout Flag"]
    #[inline(always)]
    pub fn sltf(&mut self) -> SLTF_W {
        SLTF_W { w: self }
    }
    #[doc = "Bit 4 - Timeout Counter Clock Select"]
    #[inline(always)]
    pub fn tcksel(&mut self) -> TCKSEL_W {
        TCKSEL_W { w: self }
    }
    #[doc = "Bit 5 - Second I2C Address Enable"]
    #[inline(always)]
    pub fn siicaen(&mut self) -> SIICAEN_W {
        SIICAEN_W { w: self }
    }
    #[doc = "Bit 6 - SMBus Alert Response Address Enable"]
    #[inline(always)]
    pub fn alerten(&mut self) -> ALERTEN_W {
        ALERTEN_W { w: self }
    }
    #[doc = "Bit 7 - Fast NACK/ACK Enable"]
    #[inline(always)]
    pub fn fack(&mut self) -> FACK_W {
        FACK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C SMBus Control and Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smb](index.html) module"]
pub struct SMB_SPEC;
impl crate::RegisterSpec for SMB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [smb::R](R) reader structure"]
impl crate::Readable for SMB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smb::W](W) writer structure"]
impl crate::Writable for SMB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMB to value 0"]
impl crate::Resettable for SMB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
