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
#[doc = "Send Break\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBK_A {
    #[doc = "0: Normal transmitter operation."]
    _0 = 0,
    #[doc = "1: Queue break character(s) to be sent."]
    _1 = 1,
}
impl From<SBK_A> for bool {
    #[inline(always)]
    fn from(variant: SBK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBK` reader - Send Break"]
pub struct SBK_R(crate::FieldReader<bool, SBK_A>);
impl SBK_R {
    pub(crate) fn new(bits: bool) -> Self {
        SBK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBK_A {
        match self.bits {
            false => SBK_A::_0,
            true => SBK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SBK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SBK_A::_1
    }
}
impl core::ops::Deref for SBK_R {
    type Target = crate::FieldReader<bool, SBK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SBK` writer - Send Break"]
pub struct SBK_W<'a> {
    w: &'a mut W,
}
impl<'a> SBK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SBK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal transmitter operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBK_A::_0)
    }
    #[doc = "Queue break character(s) to be sent."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBK_A::_1)
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
#[doc = "Receiver Wakeup Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWU_A {
    #[doc = "0: Normal UART receiver operation."]
    _0 = 0,
    #[doc = "1: UART receiver in standby waiting for wakeup condition."]
    _1 = 1,
}
impl From<RWU_A> for bool {
    #[inline(always)]
    fn from(variant: RWU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWU` reader - Receiver Wakeup Control"]
pub struct RWU_R(crate::FieldReader<bool, RWU_A>);
impl RWU_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWU_A {
        match self.bits {
            false => RWU_A::_0,
            true => RWU_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RWU_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RWU_A::_1
    }
}
impl core::ops::Deref for RWU_R {
    type Target = crate::FieldReader<bool, RWU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWU` writer - Receiver Wakeup Control"]
pub struct RWU_W<'a> {
    w: &'a mut W,
}
impl<'a> RWU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal UART receiver operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWU_A::_0)
    }
    #[doc = "UART receiver in standby waiting for wakeup condition."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWU_A::_1)
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
#[doc = "Receiver Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RE_A {
    #[doc = "0: Receiver disabled."]
    _0 = 0,
    #[doc = "1: Receiver enabled."]
    _1 = 1,
}
impl From<RE_A> for bool {
    #[inline(always)]
    fn from(variant: RE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RE` reader - Receiver Enable"]
pub struct RE_R(crate::FieldReader<bool, RE_A>);
impl RE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RE_A {
        match self.bits {
            false => RE_A::_0,
            true => RE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RE_A::_1
    }
}
impl core::ops::Deref for RE_R {
    type Target = crate::FieldReader<bool, RE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RE` writer - Receiver Enable"]
pub struct RE_W<'a> {
    w: &'a mut W,
}
impl<'a> RE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Receiver disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RE_A::_0)
    }
    #[doc = "Receiver enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RE_A::_1)
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
#[doc = "Transmitter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TE_A {
    #[doc = "0: Transmitter disabled."]
    _0 = 0,
    #[doc = "1: Transmitter enabled."]
    _1 = 1,
}
impl From<TE_A> for bool {
    #[inline(always)]
    fn from(variant: TE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE` reader - Transmitter Enable"]
pub struct TE_R(crate::FieldReader<bool, TE_A>);
impl TE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TE_A {
        match self.bits {
            false => TE_A::_0,
            true => TE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TE_A::_1
    }
}
impl core::ops::Deref for TE_R {
    type Target = crate::FieldReader<bool, TE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TE` writer - Transmitter Enable"]
pub struct TE_W<'a> {
    w: &'a mut W,
}
impl<'a> TE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmitter disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TE_A::_0)
    }
    #[doc = "Transmitter enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TE_A::_1)
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
#[doc = "Idle Line Interrupt Enable for IDLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILIE_A {
    #[doc = "0: Hardware interrupts from IDLE disabled; use polling."]
    _0 = 0,
    #[doc = "1: Hardware interrupt requested when IDLE flag is 1."]
    _1 = 1,
}
impl From<ILIE_A> for bool {
    #[inline(always)]
    fn from(variant: ILIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ILIE` reader - Idle Line Interrupt Enable for IDLE"]
pub struct ILIE_R(crate::FieldReader<bool, ILIE_A>);
impl ILIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ILIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILIE_A {
        match self.bits {
            false => ILIE_A::_0,
            true => ILIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ILIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ILIE_A::_1
    }
}
impl core::ops::Deref for ILIE_R {
    type Target = crate::FieldReader<bool, ILIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ILIE` writer - Idle Line Interrupt Enable for IDLE"]
pub struct ILIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ILIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Hardware interrupts from IDLE disabled; use polling."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ILIE_A::_0)
    }
    #[doc = "Hardware interrupt requested when IDLE flag is 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ILIE_A::_1)
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
#[doc = "Receiver Interrupt Enable for RDRF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIE_A {
    #[doc = "0: Hardware interrupts from RDRF disabled; use polling."]
    _0 = 0,
    #[doc = "1: Hardware interrupt requested when RDRF flag is 1."]
    _1 = 1,
}
impl From<RIE_A> for bool {
    #[inline(always)]
    fn from(variant: RIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIE` reader - Receiver Interrupt Enable for RDRF"]
pub struct RIE_R(crate::FieldReader<bool, RIE_A>);
impl RIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RIE_A {
        match self.bits {
            false => RIE_A::_0,
            true => RIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RIE_A::_1
    }
}
impl core::ops::Deref for RIE_R {
    type Target = crate::FieldReader<bool, RIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RIE` writer - Receiver Interrupt Enable for RDRF"]
pub struct RIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Hardware interrupts from RDRF disabled; use polling."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RIE_A::_0)
    }
    #[doc = "Hardware interrupt requested when RDRF flag is 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RIE_A::_1)
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
#[doc = "Transmission Complete Interrupt Enable for TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIE_A {
    #[doc = "0: Hardware interrupts from TC disabled; use polling."]
    _0 = 0,
    #[doc = "1: Hardware interrupt requested when TC flag is 1."]
    _1 = 1,
}
impl From<TCIE_A> for bool {
    #[inline(always)]
    fn from(variant: TCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIE` reader - Transmission Complete Interrupt Enable for TC"]
pub struct TCIE_R(crate::FieldReader<bool, TCIE_A>);
impl TCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIE_A {
        match self.bits {
            false => TCIE_A::_0,
            true => TCIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TCIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TCIE_A::_1
    }
}
impl core::ops::Deref for TCIE_R {
    type Target = crate::FieldReader<bool, TCIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCIE` writer - Transmission Complete Interrupt Enable for TC"]
pub struct TCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Hardware interrupts from TC disabled; use polling."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCIE_A::_0)
    }
    #[doc = "Hardware interrupt requested when TC flag is 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCIE_A::_1)
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
#[doc = "Transmit Interrupt Enable for TDRE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIE_A {
    #[doc = "0: Hardware interrupts from TDRE disabled; use polling."]
    _0 = 0,
    #[doc = "1: Hardware interrupt requested when TDRE flag is 1."]
    _1 = 1,
}
impl From<TIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIE` reader - Transmit Interrupt Enable for TDRE"]
pub struct TIE_R(crate::FieldReader<bool, TIE_A>);
impl TIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIE_A {
        match self.bits {
            false => TIE_A::_0,
            true => TIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TIE_A::_1
    }
}
impl core::ops::Deref for TIE_R {
    type Target = crate::FieldReader<bool, TIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE` writer - Transmit Interrupt Enable for TDRE"]
pub struct TIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Hardware interrupts from TDRE disabled; use polling."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIE_A::_0)
    }
    #[doc = "Hardware interrupt requested when TDRE flag is 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIE_A::_1)
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
    #[doc = "Bit 0 - Send Break"]
    #[inline(always)]
    pub fn sbk(&self) -> SBK_R {
        SBK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receiver Wakeup Control"]
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receiver Enable"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Idle Line Interrupt Enable for IDLE"]
    #[inline(always)]
    pub fn ilie(&self) -> ILIE_R {
        ILIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receiver Interrupt Enable for RDRF"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmission Complete Interrupt Enable for TC"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmit Interrupt Enable for TDRE"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Send Break"]
    #[inline(always)]
    pub fn sbk(&mut self) -> SBK_W {
        SBK_W { w: self }
    }
    #[doc = "Bit 1 - Receiver Wakeup Control"]
    #[inline(always)]
    pub fn rwu(&mut self) -> RWU_W {
        RWU_W { w: self }
    }
    #[doc = "Bit 2 - Receiver Enable"]
    #[inline(always)]
    pub fn re(&mut self) -> RE_W {
        RE_W { w: self }
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    pub fn te(&mut self) -> TE_W {
        TE_W { w: self }
    }
    #[doc = "Bit 4 - Idle Line Interrupt Enable for IDLE"]
    #[inline(always)]
    pub fn ilie(&mut self) -> ILIE_W {
        ILIE_W { w: self }
    }
    #[doc = "Bit 5 - Receiver Interrupt Enable for RDRF"]
    #[inline(always)]
    pub fn rie(&mut self) -> RIE_W {
        RIE_W { w: self }
    }
    #[doc = "Bit 6 - Transmission Complete Interrupt Enable for TC"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W {
        TCIE_W { w: self }
    }
    #[doc = "Bit 7 - Transmit Interrupt Enable for TDRE"]
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W {
        TIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2](index.html) module"]
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
