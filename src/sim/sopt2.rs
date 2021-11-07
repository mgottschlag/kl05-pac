#[doc = "Register `SOPT2` reader"]
pub struct R(crate::R<SOPT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOPT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOPT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOPT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOPT2` writer"]
pub struct W(crate::W<SOPT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOPT2_SPEC>;
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
impl From<crate::W<SOPT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOPT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "RTC clock out select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCCLKOUTSEL_A {
    #[doc = "0: RTC 1 Hz clock is output on the RTC_CLKOUT pin."]
    _0 = 0,
    #[doc = "1: OSCERCLK clock is output on the RTC_CLKOUT pin."]
    _1 = 1,
}
impl From<RTCCLKOUTSEL_A> for bool {
    #[inline(always)]
    fn from(variant: RTCCLKOUTSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCCLKOUTSEL` reader - RTC clock out select"]
pub struct RTCCLKOUTSEL_R(crate::FieldReader<bool, RTCCLKOUTSEL_A>);
impl RTCCLKOUTSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCCLKOUTSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCCLKOUTSEL_A {
        match self.bits {
            false => RTCCLKOUTSEL_A::_0,
            true => RTCCLKOUTSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RTCCLKOUTSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RTCCLKOUTSEL_A::_1
    }
}
impl core::ops::Deref for RTCCLKOUTSEL_R {
    type Target = crate::FieldReader<bool, RTCCLKOUTSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCCLKOUTSEL` writer - RTC clock out select"]
pub struct RTCCLKOUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCLKOUTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCCLKOUTSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RTC 1 Hz clock is output on the RTC_CLKOUT pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTCCLKOUTSEL_A::_0)
    }
    #[doc = "OSCERCLK clock is output on the RTC_CLKOUT pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTCCLKOUTSEL_A::_1)
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
#[doc = "CLKOUT select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKOUTSEL_A {
    #[doc = "2: Bus clock"]
    _010 = 2,
    #[doc = "3: LPO clock (1 kHz)"]
    _011 = 3,
    #[doc = "4: MCGIRCLK"]
    _100 = 4,
    #[doc = "6: OSCERCLK"]
    _110 = 6,
}
impl From<CLKOUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLKOUTSEL` reader - CLKOUT select"]
pub struct CLKOUTSEL_R(crate::FieldReader<u8, CLKOUTSEL_A>);
impl CLKOUTSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKOUTSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKOUTSEL_A> {
        match self.bits {
            2 => Some(CLKOUTSEL_A::_010),
            3 => Some(CLKOUTSEL_A::_011),
            4 => Some(CLKOUTSEL_A::_100),
            6 => Some(CLKOUTSEL_A::_110),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        **self == CLKOUTSEL_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        **self == CLKOUTSEL_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        **self == CLKOUTSEL_A::_100
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        **self == CLKOUTSEL_A::_110
    }
}
impl core::ops::Deref for CLKOUTSEL_R {
    type Target = crate::FieldReader<u8, CLKOUTSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKOUTSEL` writer - CLKOUT select"]
pub struct CLKOUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKOUTSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Bus clock"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_010)
    }
    #[doc = "LPO clock (1 kHz)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_011)
    }
    #[doc = "MCGIRCLK"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_100)
    }
    #[doc = "OSCERCLK"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_110)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "TPM clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TPMSRC_A {
    #[doc = "0: Clock disabled"]
    _00 = 0,
    #[doc = "1: MCGFLLCLK clock"]
    _01 = 1,
    #[doc = "2: OSCERCLK clock"]
    _10 = 2,
    #[doc = "3: MCGIRCLK clock"]
    _11 = 3,
}
impl From<TPMSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: TPMSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TPMSRC` reader - TPM clock source select"]
pub struct TPMSRC_R(crate::FieldReader<u8, TPMSRC_A>);
impl TPMSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        TPMSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPMSRC_A {
        match self.bits {
            0 => TPMSRC_A::_00,
            1 => TPMSRC_A::_01,
            2 => TPMSRC_A::_10,
            3 => TPMSRC_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == TPMSRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == TPMSRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == TPMSRC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == TPMSRC_A::_11
    }
}
impl core::ops::Deref for TPMSRC_R {
    type Target = crate::FieldReader<u8, TPMSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPMSRC` writer - TPM clock source select"]
pub struct TPMSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TPMSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPMSRC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TPMSRC_A::_00)
    }
    #[doc = "MCGFLLCLK clock"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TPMSRC_A::_01)
    }
    #[doc = "OSCERCLK clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TPMSRC_A::_10)
    }
    #[doc = "MCGIRCLK clock"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TPMSRC_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "UART0 clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UART0SRC_A {
    #[doc = "0: Clock disabled"]
    _00 = 0,
    #[doc = "1: MCGFLLCLK clock"]
    _01 = 1,
    #[doc = "2: OSCERCLK clock"]
    _10 = 2,
    #[doc = "3: MCGIRCLK clock"]
    _11 = 3,
}
impl From<UART0SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: UART0SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `UART0SRC` reader - UART0 clock source select"]
pub struct UART0SRC_R(crate::FieldReader<u8, UART0SRC_A>);
impl UART0SRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART0SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART0SRC_A {
        match self.bits {
            0 => UART0SRC_A::_00,
            1 => UART0SRC_A::_01,
            2 => UART0SRC_A::_10,
            3 => UART0SRC_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == UART0SRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == UART0SRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == UART0SRC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == UART0SRC_A::_11
    }
}
impl core::ops::Deref for UART0SRC_R {
    type Target = crate::FieldReader<u8, UART0SRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART0SRC` writer - UART0 clock source select"]
pub struct UART0SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART0SRC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(UART0SRC_A::_00)
    }
    #[doc = "MCGFLLCLK clock"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(UART0SRC_A::_01)
    }
    #[doc = "OSCERCLK clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(UART0SRC_A::_10)
    }
    #[doc = "MCGIRCLK clock"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(UART0SRC_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - RTC clock out select"]
    #[inline(always)]
    pub fn rtcclkoutsel(&self) -> RTCCLKOUTSEL_R {
        RTCCLKOUTSEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - CLKOUT select"]
    #[inline(always)]
    pub fn clkoutsel(&self) -> CLKOUTSEL_R {
        CLKOUTSEL_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 24:25 - TPM clock source select"]
    #[inline(always)]
    pub fn tpmsrc(&self) -> TPMSRC_R {
        TPMSRC_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - UART0 clock source select"]
    #[inline(always)]
    pub fn uart0src(&self) -> UART0SRC_R {
        UART0SRC_R::new(((self.bits >> 26) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - RTC clock out select"]
    #[inline(always)]
    pub fn rtcclkoutsel(&mut self) -> RTCCLKOUTSEL_W {
        RTCCLKOUTSEL_W { w: self }
    }
    #[doc = "Bits 5:7 - CLKOUT select"]
    #[inline(always)]
    pub fn clkoutsel(&mut self) -> CLKOUTSEL_W {
        CLKOUTSEL_W { w: self }
    }
    #[doc = "Bits 24:25 - TPM clock source select"]
    #[inline(always)]
    pub fn tpmsrc(&mut self) -> TPMSRC_W {
        TPMSRC_W { w: self }
    }
    #[doc = "Bits 26:27 - UART0 clock source select"]
    #[inline(always)]
    pub fn uart0src(&mut self) -> UART0SRC_W {
        UART0SRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Options Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sopt2](index.html) module"]
pub struct SOPT2_SPEC;
impl crate::RegisterSpec for SOPT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sopt2::R](R) reader structure"]
impl crate::Readable for SOPT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sopt2::W](W) writer structure"]
impl crate::Writable for SOPT2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SOPT2 to value 0"]
impl crate::Resettable for SOPT2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
