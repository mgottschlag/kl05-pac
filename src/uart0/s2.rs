#[doc = "Register `S2` reader"]
pub struct R(crate::R<S2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S2` writer"]
pub struct W(crate::W<S2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S2_SPEC>;
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
impl From<crate::W<S2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Receiver Active Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAF_A {
    #[doc = "0: UART receiver idle waiting for a start bit."]
    _0 = 0,
    #[doc = "1: UART receiver active ( UART _RXD input not idle)."]
    _1 = 1,
}
impl From<RAF_A> for bool {
    #[inline(always)]
    fn from(variant: RAF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAF` reader - Receiver Active Flag"]
pub struct RAF_R(crate::FieldReader<bool, RAF_A>);
impl RAF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAF_A {
        match self.bits {
            false => RAF_A::_0,
            true => RAF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RAF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RAF_A::_1
    }
}
impl core::ops::Deref for RAF_R {
    type Target = crate::FieldReader<bool, RAF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "LIN Break Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBKDE_A {
    #[doc = "0: Break character is detected at length 10 bit times (if M = 0, SBNS = 0) or 11 (if M = 1, SBNS = 0 or M = 0, SBNS = 1) or 12 (if M = 1, SBNS = 1 or M10 = 1, SNBS = 0) or 13 (if M10 = 1, SNBS = 1)."]
    _0 = 0,
    #[doc = "1: Break character is detected at length of 11 bit times (if M = 0, SBNS = 0) or 12 (if M = 1, SBNS = 0 or M = 0, SBNS = 1) or 14 (if M = 1, SBNS = 1 or M10 = 1, SNBS = 0) or 15 (if M10 = 1, SNBS = 1)."]
    _1 = 1,
}
impl From<LBKDE_A> for bool {
    #[inline(always)]
    fn from(variant: LBKDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBKDE` reader - LIN Break Detection Enable"]
pub struct LBKDE_R(crate::FieldReader<bool, LBKDE_A>);
impl LBKDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LBKDE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBKDE_A {
        match self.bits {
            false => LBKDE_A::_0,
            true => LBKDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LBKDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LBKDE_A::_1
    }
}
impl core::ops::Deref for LBKDE_R {
    type Target = crate::FieldReader<bool, LBKDE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LBKDE` writer - LIN Break Detection Enable"]
pub struct LBKDE_W<'a> {
    w: &'a mut W,
}
impl<'a> LBKDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LBKDE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Break character is detected at length 10 bit times (if M = 0, SBNS = 0) or 11 (if M = 1, SBNS = 0 or M = 0, SBNS = 1) or 12 (if M = 1, SBNS = 1 or M10 = 1, SNBS = 0) or 13 (if M10 = 1, SNBS = 1)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LBKDE_A::_0)
    }
    #[doc = "Break character is detected at length of 11 bit times (if M = 0, SBNS = 0) or 12 (if M = 1, SBNS = 0 or M = 0, SBNS = 1) or 14 (if M = 1, SBNS = 1 or M10 = 1, SNBS = 0) or 15 (if M10 = 1, SNBS = 1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LBKDE_A::_1)
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
#[doc = "Break Character Generation Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRK13_A {
    #[doc = "0: Break character is transmitted with length of 10 bit times (if M = 0, SBNS = 0) or 11 (if M = 1, SBNS = 0 or M = 0, SBNS = 1) or 12 (if M = 1, SBNS = 1 or M10 = 1, SNBS = 0) or 13 (if M10 = 1, SNBS = 1)."]
    _0 = 0,
    #[doc = "1: Break character is transmitted with length of 13 bit times (if M = 0, SBNS = 0) or 14 (if M = 1, SBNS = 0 or M = 0, SBNS = 1) or 15 (if M = 1, SBNS = 1 or M10 = 1, SNBS = 0) or 16 (if M10 = 1, SNBS = 1)."]
    _1 = 1,
}
impl From<BRK13_A> for bool {
    #[inline(always)]
    fn from(variant: BRK13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRK13` reader - Break Character Generation Length"]
pub struct BRK13_R(crate::FieldReader<bool, BRK13_A>);
impl BRK13_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRK13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRK13_A {
        match self.bits {
            false => BRK13_A::_0,
            true => BRK13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BRK13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BRK13_A::_1
    }
}
impl core::ops::Deref for BRK13_R {
    type Target = crate::FieldReader<bool, BRK13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRK13` writer - Break Character Generation Length"]
pub struct BRK13_W<'a> {
    w: &'a mut W,
}
impl<'a> BRK13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRK13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Break character is transmitted with length of 10 bit times (if M = 0, SBNS = 0) or 11 (if M = 1, SBNS = 0 or M = 0, SBNS = 1) or 12 (if M = 1, SBNS = 1 or M10 = 1, SNBS = 0) or 13 (if M10 = 1, SNBS = 1)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRK13_A::_0)
    }
    #[doc = "Break character is transmitted with length of 13 bit times (if M = 0, SBNS = 0) or 14 (if M = 1, SBNS = 0 or M = 0, SBNS = 1) or 15 (if M = 1, SBNS = 1 or M10 = 1, SNBS = 0) or 16 (if M10 = 1, SNBS = 1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRK13_A::_1)
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
#[doc = "Receive Wake Up Idle Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWUID_A {
    #[doc = "0: During receive standby state (RWU = 1), the IDLE bit does not get set upon detection of an idle character."]
    _0 = 0,
    #[doc = "1: During receive standby state (RWU = 1), the IDLE bit gets set upon detection of an idle character."]
    _1 = 1,
}
impl From<RWUID_A> for bool {
    #[inline(always)]
    fn from(variant: RWUID_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWUID` reader - Receive Wake Up Idle Detect"]
pub struct RWUID_R(crate::FieldReader<bool, RWUID_A>);
impl RWUID_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWUID_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWUID_A {
        match self.bits {
            false => RWUID_A::_0,
            true => RWUID_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RWUID_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RWUID_A::_1
    }
}
impl core::ops::Deref for RWUID_R {
    type Target = crate::FieldReader<bool, RWUID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWUID` writer - Receive Wake Up Idle Detect"]
pub struct RWUID_W<'a> {
    w: &'a mut W,
}
impl<'a> RWUID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWUID_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "During receive standby state (RWU = 1), the IDLE bit does not get set upon detection of an idle character."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWUID_A::_0)
    }
    #[doc = "During receive standby state (RWU = 1), the IDLE bit gets set upon detection of an idle character."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWUID_A::_1)
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
#[doc = "Receive Data Inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXINV_A {
    #[doc = "0: Receive data not inverted."]
    _0 = 0,
    #[doc = "1: Receive data inverted."]
    _1 = 1,
}
impl From<RXINV_A> for bool {
    #[inline(always)]
    fn from(variant: RXINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXINV` reader - Receive Data Inversion"]
pub struct RXINV_R(crate::FieldReader<bool, RXINV_A>);
impl RXINV_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXINV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXINV_A {
        match self.bits {
            false => RXINV_A::_0,
            true => RXINV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXINV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXINV_A::_1
    }
}
impl core::ops::Deref for RXINV_R {
    type Target = crate::FieldReader<bool, RXINV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXINV` writer - Receive Data Inversion"]
pub struct RXINV_W<'a> {
    w: &'a mut W,
}
impl<'a> RXINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXINV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Receive data not inverted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXINV_A::_0)
    }
    #[doc = "Receive data inverted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXINV_A::_1)
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
#[doc = "MSB First\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSBF_A {
    #[doc = "0: LSB (bit0) is the first bit that is transmitted following the start bit. Further, the first bit received after the start bit is identified as bit0."]
    _0 = 0,
    #[doc = "1: MSB (bit9, bit8, bit7 or bit6) is the first bit that is transmitted following the start bit depending on the setting of C1\\[M\\], C1\\[PE\\]
and C4\\[M10\\]. Further, the first bit received after the start bit is identified as bit9, bit8, bit7 or bit6 depending on the setting of C1\\[M\\]
and C1\\[PE\\]."]
    _1 = 1,
}
impl From<MSBF_A> for bool {
    #[inline(always)]
    fn from(variant: MSBF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSBF` reader - MSB First"]
pub struct MSBF_R(crate::FieldReader<bool, MSBF_A>);
impl MSBF_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSBF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSBF_A {
        match self.bits {
            false => MSBF_A::_0,
            true => MSBF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MSBF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MSBF_A::_1
    }
}
impl core::ops::Deref for MSBF_R {
    type Target = crate::FieldReader<bool, MSBF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSBF` writer - MSB First"]
pub struct MSBF_W<'a> {
    w: &'a mut W,
}
impl<'a> MSBF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSBF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LSB (bit0) is the first bit that is transmitted following the start bit. Further, the first bit received after the start bit is identified as bit0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSBF_A::_0)
    }
    #[doc = "MSB (bit9, bit8, bit7 or bit6) is the first bit that is transmitted following the start bit depending on the setting of C1\\[M\\], C1\\[PE\\]
and C4\\[M10\\]. Further, the first bit received after the start bit is identified as bit9, bit8, bit7 or bit6 depending on the setting of C1\\[M\\]
and C1\\[PE\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSBF_A::_1)
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
#[doc = "UART _RX Pin Active Edge Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEDGIF_A {
    #[doc = "0: No active edge on the receive pin has occurred."]
    _0 = 0,
    #[doc = "1: An active edge on the receive pin has occurred."]
    _1 = 1,
}
impl From<RXEDGIF_A> for bool {
    #[inline(always)]
    fn from(variant: RXEDGIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXEDGIF` reader - UART _RX Pin Active Edge Interrupt Flag"]
pub struct RXEDGIF_R(crate::FieldReader<bool, RXEDGIF_A>);
impl RXEDGIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXEDGIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEDGIF_A {
        match self.bits {
            false => RXEDGIF_A::_0,
            true => RXEDGIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXEDGIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXEDGIF_A::_1
    }
}
impl core::ops::Deref for RXEDGIF_R {
    type Target = crate::FieldReader<bool, RXEDGIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXEDGIF` writer - UART _RX Pin Active Edge Interrupt Flag"]
pub struct RXEDGIF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXEDGIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXEDGIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No active edge on the receive pin has occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXEDGIF_A::_0)
    }
    #[doc = "An active edge on the receive pin has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXEDGIF_A::_1)
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
#[doc = "LIN Break Detect Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBKDIF_A {
    #[doc = "0: No LIN break character has been detected."]
    _0 = 0,
    #[doc = "1: LIN break character has been detected."]
    _1 = 1,
}
impl From<LBKDIF_A> for bool {
    #[inline(always)]
    fn from(variant: LBKDIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBKDIF` reader - LIN Break Detect Interrupt Flag"]
pub struct LBKDIF_R(crate::FieldReader<bool, LBKDIF_A>);
impl LBKDIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LBKDIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBKDIF_A {
        match self.bits {
            false => LBKDIF_A::_0,
            true => LBKDIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LBKDIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LBKDIF_A::_1
    }
}
impl core::ops::Deref for LBKDIF_R {
    type Target = crate::FieldReader<bool, LBKDIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LBKDIF` writer - LIN Break Detect Interrupt Flag"]
pub struct LBKDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> LBKDIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LBKDIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No LIN break character has been detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LBKDIF_A::_0)
    }
    #[doc = "LIN break character has been detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LBKDIF_A::_1)
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
    #[doc = "Bit 0 - Receiver Active Flag"]
    #[inline(always)]
    pub fn raf(&self) -> RAF_R {
        RAF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LIN Break Detection Enable"]
    #[inline(always)]
    pub fn lbkde(&self) -> LBKDE_R {
        LBKDE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Break Character Generation Length"]
    #[inline(always)]
    pub fn brk13(&self) -> BRK13_R {
        BRK13_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive Wake Up Idle Detect"]
    #[inline(always)]
    pub fn rwuid(&self) -> RWUID_R {
        RWUID_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive Data Inversion"]
    #[inline(always)]
    pub fn rxinv(&self) -> RXINV_R {
        RXINV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MSB First"]
    #[inline(always)]
    pub fn msbf(&self) -> MSBF_R {
        MSBF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - UART _RX Pin Active Edge Interrupt Flag"]
    #[inline(always)]
    pub fn rxedgif(&self) -> RXEDGIF_R {
        RXEDGIF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LIN Break Detect Interrupt Flag"]
    #[inline(always)]
    pub fn lbkdif(&self) -> LBKDIF_R {
        LBKDIF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - LIN Break Detection Enable"]
    #[inline(always)]
    pub fn lbkde(&mut self) -> LBKDE_W {
        LBKDE_W { w: self }
    }
    #[doc = "Bit 2 - Break Character Generation Length"]
    #[inline(always)]
    pub fn brk13(&mut self) -> BRK13_W {
        BRK13_W { w: self }
    }
    #[doc = "Bit 3 - Receive Wake Up Idle Detect"]
    #[inline(always)]
    pub fn rwuid(&mut self) -> RWUID_W {
        RWUID_W { w: self }
    }
    #[doc = "Bit 4 - Receive Data Inversion"]
    #[inline(always)]
    pub fn rxinv(&mut self) -> RXINV_W {
        RXINV_W { w: self }
    }
    #[doc = "Bit 5 - MSB First"]
    #[inline(always)]
    pub fn msbf(&mut self) -> MSBF_W {
        MSBF_W { w: self }
    }
    #[doc = "Bit 6 - UART _RX Pin Active Edge Interrupt Flag"]
    #[inline(always)]
    pub fn rxedgif(&mut self) -> RXEDGIF_W {
        RXEDGIF_W { w: self }
    }
    #[doc = "Bit 7 - LIN Break Detect Interrupt Flag"]
    #[inline(always)]
    pub fn lbkdif(&mut self) -> LBKDIF_W {
        LBKDIF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Status Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s2](index.html) module"]
pub struct S2_SPEC;
impl crate::RegisterSpec for S2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [s2::R](R) reader structure"]
impl crate::Readable for S2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s2::W](W) writer structure"]
impl crate::Writable for S2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets S2 to value 0"]
impl crate::Resettable for S2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
