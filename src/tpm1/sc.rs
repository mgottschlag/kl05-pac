#[doc = "Register `SC` reader"]
pub struct R(crate::R<SC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SC` writer"]
pub struct W(crate::W<SC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SC_SPEC>;
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
impl From<crate::W<SC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Prescale Factor Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PS_A {
    #[doc = "0: Divide by 1"]
    _000 = 0,
    #[doc = "1: Divide by 2"]
    _001 = 1,
    #[doc = "2: Divide by 4"]
    _010 = 2,
    #[doc = "3: Divide by 8"]
    _011 = 3,
    #[doc = "4: Divide by 16"]
    _100 = 4,
    #[doc = "5: Divide by 32"]
    _101 = 5,
    #[doc = "6: Divide by 64"]
    _110 = 6,
    #[doc = "7: Divide by 128"]
    _111 = 7,
}
impl From<PS_A> for u8 {
    #[inline(always)]
    fn from(variant: PS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PS` reader - Prescale Factor Selection"]
pub struct PS_R(crate::FieldReader<u8, PS_A>);
impl PS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS_A {
        match self.bits {
            0 => PS_A::_000,
            1 => PS_A::_001,
            2 => PS_A::_010,
            3 => PS_A::_011,
            4 => PS_A::_100,
            5 => PS_A::_101,
            6 => PS_A::_110,
            7 => PS_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        **self == PS_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        **self == PS_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        **self == PS_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        **self == PS_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        **self == PS_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        **self == PS_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        **self == PS_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        **self == PS_A::_111
    }
}
impl core::ops::Deref for PS_R {
    type Target = crate::FieldReader<u8, PS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PS` writer - Prescale Factor Selection"]
pub struct PS_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(PS_A::_000)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(PS_A::_001)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(PS_A::_010)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(PS_A::_011)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(PS_A::_100)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(PS_A::_101)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(PS_A::_110)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(PS_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Clock Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMOD_A {
    #[doc = "0: LPTPM counter is disabled"]
    _00 = 0,
    #[doc = "1: LPTPM counter increments on every LPTPM counter clock"]
    _01 = 1,
    #[doc = "2: LPTPM counter increments on rising edge of LPTPM_EXTCLK synchronized to the LPTPM counter clock"]
    _10 = 2,
}
impl From<CMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: CMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMOD` reader - Clock Mode Selection"]
pub struct CMOD_R(crate::FieldReader<u8, CMOD_A>);
impl CMOD_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMOD_A> {
        match self.bits {
            0 => Some(CMOD_A::_00),
            1 => Some(CMOD_A::_01),
            2 => Some(CMOD_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == CMOD_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == CMOD_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == CMOD_A::_10
    }
}
impl core::ops::Deref for CMOD_R {
    type Target = crate::FieldReader<u8, CMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMOD` writer - Clock Mode Selection"]
pub struct CMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMOD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LPTPM counter is disabled"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CMOD_A::_00)
    }
    #[doc = "LPTPM counter increments on every LPTPM counter clock"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CMOD_A::_01)
    }
    #[doc = "LPTPM counter increments on rising edge of LPTPM_EXTCLK synchronized to the LPTPM counter clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CMOD_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Center-aligned PWM Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPWMS_A {
    #[doc = "0: LPTPM counter operates in up counting mode."]
    _0 = 0,
    #[doc = "1: LPTPM counter operates in up-down counting mode."]
    _1 = 1,
}
impl From<CPWMS_A> for bool {
    #[inline(always)]
    fn from(variant: CPWMS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPWMS` reader - Center-aligned PWM Select"]
pub struct CPWMS_R(crate::FieldReader<bool, CPWMS_A>);
impl CPWMS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPWMS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPWMS_A {
        match self.bits {
            false => CPWMS_A::_0,
            true => CPWMS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CPWMS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CPWMS_A::_1
    }
}
impl core::ops::Deref for CPWMS_R {
    type Target = crate::FieldReader<bool, CPWMS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPWMS` writer - Center-aligned PWM Select"]
pub struct CPWMS_W<'a> {
    w: &'a mut W,
}
impl<'a> CPWMS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPWMS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LPTPM counter operates in up counting mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPWMS_A::_0)
    }
    #[doc = "LPTPM counter operates in up-down counting mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPWMS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Timer Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOIE_A {
    #[doc = "0: Disable TOF interrupts. Use software polling or DMA request."]
    _0 = 0,
    #[doc = "1: Enable TOF interrupts. An interrupt is generated when TOF equals one."]
    _1 = 1,
}
impl From<TOIE_A> for bool {
    #[inline(always)]
    fn from(variant: TOIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOIE` reader - Timer Overflow Interrupt Enable"]
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
#[doc = "Field `TOIE` writer - Timer Overflow Interrupt Enable"]
pub struct TOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable TOF interrupts. Use software polling or DMA request."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOIE_A::_0)
    }
    #[doc = "Enable TOF interrupts. An interrupt is generated when TOF equals one."]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Timer Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOF_A {
    #[doc = "0: LPTPM counter has not overflowed."]
    _0 = 0,
    #[doc = "1: LPTPM counter has overflowed."]
    _1 = 1,
}
impl From<TOF_A> for bool {
    #[inline(always)]
    fn from(variant: TOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOF` reader - Timer Overflow Flag"]
pub struct TOF_R(crate::FieldReader<bool, TOF_A>);
impl TOF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOF_A {
        match self.bits {
            false => TOF_A::_0,
            true => TOF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TOF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TOF_A::_1
    }
}
impl core::ops::Deref for TOF_R {
    type Target = crate::FieldReader<bool, TOF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOF` writer - Timer Overflow Flag"]
pub struct TOF_W<'a> {
    w: &'a mut W,
}
impl<'a> TOF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LPTPM counter has not overflowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOF_A::_0)
    }
    #[doc = "LPTPM counter has overflowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOF_A::_1)
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
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_A {
    #[doc = "0: Disables DMA transfers."]
    _0 = 0,
    #[doc = "1: Enables DMA transfers."]
    _1 = 1,
}
impl From<DMA_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA` reader - DMA Enable"]
pub struct DMA_R(crate::FieldReader<bool, DMA_A>);
impl DMA_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_A {
        match self.bits {
            false => DMA_A::_0,
            true => DMA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DMA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DMA_A::_1
    }
}
impl core::ops::Deref for DMA_R {
    type Target = crate::FieldReader<bool, DMA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA` writer - DMA Enable"]
pub struct DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables DMA transfers."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMA_A::_0)
    }
    #[doc = "Enables DMA transfers."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMA_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Prescale Factor Selection"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:4 - Clock Mode Selection"]
    #[inline(always)]
    pub fn cmod(&self) -> CMOD_R {
        CMOD_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Center-aligned PWM Select"]
    #[inline(always)]
    pub fn cpwms(&self) -> CPWMS_R {
        CPWMS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Timer Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn toie(&self) -> TOIE_R {
        TOIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Timer Overflow Flag"]
    #[inline(always)]
    pub fn tof(&self) -> TOF_R {
        TOF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DMA Enable"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Prescale Factor Selection"]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W {
        PS_W { w: self }
    }
    #[doc = "Bits 3:4 - Clock Mode Selection"]
    #[inline(always)]
    pub fn cmod(&mut self) -> CMOD_W {
        CMOD_W { w: self }
    }
    #[doc = "Bit 5 - Center-aligned PWM Select"]
    #[inline(always)]
    pub fn cpwms(&mut self) -> CPWMS_W {
        CPWMS_W { w: self }
    }
    #[doc = "Bit 6 - Timer Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn toie(&mut self) -> TOIE_W {
        TOIE_W { w: self }
    }
    #[doc = "Bit 7 - Timer Overflow Flag"]
    #[inline(always)]
    pub fn tof(&mut self) -> TOF_W {
        TOF_W { w: self }
    }
    #[doc = "Bit 8 - DMA Enable"]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status and Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc](index.html) module"]
pub struct SC_SPEC;
impl crate::RegisterSpec for SC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sc::R](R) reader structure"]
impl crate::Readable for SC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sc::W](W) writer structure"]
impl crate::Writable for SC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SC to value 0"]
impl crate::Resettable for SC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
