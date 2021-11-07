#[doc = "Register `SCGC6` reader"]
pub struct R(crate::R<SCGC6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCGC6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCGC6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCGC6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCGC6` writer"]
pub struct W(crate::W<SCGC6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCGC6_SPEC>;
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
impl From<crate::W<SCGC6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCGC6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Flash Memory Clock Gate Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTF_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<FTF_A> for bool {
    #[inline(always)]
    fn from(variant: FTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTF` reader - Flash Memory Clock Gate Control"]
pub struct FTF_R(crate::FieldReader<bool, FTF_A>);
impl FTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FTF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTF_A {
        match self.bits {
            false => FTF_A::_0,
            true => FTF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FTF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FTF_A::_1
    }
}
impl core::ops::Deref for FTF_R {
    type Target = crate::FieldReader<bool, FTF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTF` writer - Flash Memory Clock Gate Control"]
pub struct FTF_W<'a> {
    w: &'a mut W,
}
impl<'a> FTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTF_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTF_A::_1)
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
#[doc = "DMA Mux Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAMUX_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<DMAMUX_A> for bool {
    #[inline(always)]
    fn from(variant: DMAMUX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMUX` reader - DMA Mux Clock Gate Control"]
pub struct DMAMUX_R(crate::FieldReader<bool, DMAMUX_A>);
impl DMAMUX_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAMUX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAMUX_A {
        match self.bits {
            false => DMAMUX_A::_0,
            true => DMAMUX_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DMAMUX_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DMAMUX_A::_1
    }
}
impl core::ops::Deref for DMAMUX_R {
    type Target = crate::FieldReader<bool, DMAMUX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAMUX` writer - DMA Mux Clock Gate Control"]
pub struct DMAMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAMUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAMUX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAMUX_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAMUX_A::_1)
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
#[doc = "PIT Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIT_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<PIT_A> for bool {
    #[inline(always)]
    fn from(variant: PIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIT` reader - PIT Clock Gate Control"]
pub struct PIT_R(crate::FieldReader<bool, PIT_A>);
impl PIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIT_A {
        match self.bits {
            false => PIT_A::_0,
            true => PIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PIT_A::_1
    }
}
impl core::ops::Deref for PIT_R {
    type Target = crate::FieldReader<bool, PIT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIT` writer - PIT Clock Gate Control"]
pub struct PIT_W<'a> {
    w: &'a mut W,
}
impl<'a> PIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIT_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "TPM0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPM0_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<TPM0_A> for bool {
    #[inline(always)]
    fn from(variant: TPM0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPM0` reader - TPM0 Clock Gate Control"]
pub struct TPM0_R(crate::FieldReader<bool, TPM0_A>);
impl TPM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TPM0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPM0_A {
        match self.bits {
            false => TPM0_A::_0,
            true => TPM0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TPM0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TPM0_A::_1
    }
}
impl core::ops::Deref for TPM0_R {
    type Target = crate::FieldReader<bool, TPM0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPM0` writer - TPM0 Clock Gate Control"]
pub struct TPM0_W<'a> {
    w: &'a mut W,
}
impl<'a> TPM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPM0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPM0_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPM0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "TPM1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPM1_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<TPM1_A> for bool {
    #[inline(always)]
    fn from(variant: TPM1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPM1` reader - TPM1 Clock Gate Control"]
pub struct TPM1_R(crate::FieldReader<bool, TPM1_A>);
impl TPM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TPM1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPM1_A {
        match self.bits {
            false => TPM1_A::_0,
            true => TPM1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TPM1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TPM1_A::_1
    }
}
impl core::ops::Deref for TPM1_R {
    type Target = crate::FieldReader<bool, TPM1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPM1` writer - TPM1 Clock Gate Control"]
pub struct TPM1_W<'a> {
    w: &'a mut W,
}
impl<'a> TPM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPM1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPM1_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPM1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "ADC0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC0_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<ADC0_A> for bool {
    #[inline(always)]
    fn from(variant: ADC0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC0` reader - ADC0 Clock Gate Control"]
pub struct ADC0_R(crate::FieldReader<bool, ADC0_A>);
impl ADC0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC0_A {
        match self.bits {
            false => ADC0_A::_0,
            true => ADC0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADC0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADC0_A::_1
    }
}
impl core::ops::Deref for ADC0_R {
    type Target = crate::FieldReader<bool, ADC0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC0` writer - ADC0 Clock Gate Control"]
pub struct ADC0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADC0_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADC0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "RTC Access Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_A {
    #[doc = "0: Access and interrupts disabled"]
    _0 = 0,
    #[doc = "1: Access and interrupts enabled"]
    _1 = 1,
}
impl From<RTC_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC` reader - RTC Access Control"]
pub struct RTC_R(crate::FieldReader<bool, RTC_A>);
impl RTC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_A {
        match self.bits {
            false => RTC_A::_0,
            true => RTC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RTC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RTC_A::_1
    }
}
impl core::ops::Deref for RTC_R {
    type Target = crate::FieldReader<bool, RTC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC` writer - RTC Access Control"]
pub struct RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Access and interrupts disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTC_A::_0)
    }
    #[doc = "Access and interrupts enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "DAC0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAC0_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<DAC0_A> for bool {
    #[inline(always)]
    fn from(variant: DAC0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC0` reader - DAC0 Clock Gate Control"]
pub struct DAC0_R(crate::FieldReader<bool, DAC0_A>);
impl DAC0_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAC0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC0_A {
        match self.bits {
            false => DAC0_A::_0,
            true => DAC0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DAC0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DAC0_A::_1
    }
}
impl core::ops::Deref for DAC0_R {
    type Target = crate::FieldReader<bool, DAC0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC0` writer - DAC0 Clock Gate Control"]
pub struct DAC0_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAC0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DAC0_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DAC0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Flash Memory Clock Gate Control"]
    #[inline(always)]
    pub fn ftf(&self) -> FTF_R {
        FTF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA Mux Clock Gate Control"]
    #[inline(always)]
    pub fn dmamux(&self) -> DMAMUX_R {
        DMAMUX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 23 - PIT Clock Gate Control"]
    #[inline(always)]
    pub fn pit(&self) -> PIT_R {
        PIT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - TPM0 Clock Gate Control"]
    #[inline(always)]
    pub fn tpm0(&self) -> TPM0_R {
        TPM0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - TPM1 Clock Gate Control"]
    #[inline(always)]
    pub fn tpm1(&self) -> TPM1_R {
        TPM1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 27 - ADC0 Clock Gate Control"]
    #[inline(always)]
    pub fn adc0(&self) -> ADC0_R {
        ADC0_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 29 - RTC Access Control"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 31 - DAC0 Clock Gate Control"]
    #[inline(always)]
    pub fn dac0(&self) -> DAC0_R {
        DAC0_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Memory Clock Gate Control"]
    #[inline(always)]
    pub fn ftf(&mut self) -> FTF_W {
        FTF_W { w: self }
    }
    #[doc = "Bit 1 - DMA Mux Clock Gate Control"]
    #[inline(always)]
    pub fn dmamux(&mut self) -> DMAMUX_W {
        DMAMUX_W { w: self }
    }
    #[doc = "Bit 23 - PIT Clock Gate Control"]
    #[inline(always)]
    pub fn pit(&mut self) -> PIT_W {
        PIT_W { w: self }
    }
    #[doc = "Bit 24 - TPM0 Clock Gate Control"]
    #[inline(always)]
    pub fn tpm0(&mut self) -> TPM0_W {
        TPM0_W { w: self }
    }
    #[doc = "Bit 25 - TPM1 Clock Gate Control"]
    #[inline(always)]
    pub fn tpm1(&mut self) -> TPM1_W {
        TPM1_W { w: self }
    }
    #[doc = "Bit 27 - ADC0 Clock Gate Control"]
    #[inline(always)]
    pub fn adc0(&mut self) -> ADC0_W {
        ADC0_W { w: self }
    }
    #[doc = "Bit 29 - RTC Access Control"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W {
        RTC_W { w: self }
    }
    #[doc = "Bit 31 - DAC0 Clock Gate Control"]
    #[inline(always)]
    pub fn dac0(&mut self) -> DAC0_W {
        DAC0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Clock Gating Control Register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgc6](index.html) module"]
pub struct SCGC6_SPEC;
impl crate::RegisterSpec for SCGC6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scgc6::R](R) reader structure"]
impl crate::Readable for SCGC6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scgc6::W](W) writer structure"]
impl crate::Writable for SCGC6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCGC6 to value 0x01"]
impl crate::Resettable for SCGC6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
