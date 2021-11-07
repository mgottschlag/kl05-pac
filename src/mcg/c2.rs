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
#[doc = "Internal Reference Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRCS_A {
    #[doc = "0: Slow internal reference clock selected."]
    _0 = 0,
    #[doc = "1: Fast internal reference clock selected."]
    _1 = 1,
}
impl From<IRCS_A> for bool {
    #[inline(always)]
    fn from(variant: IRCS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRCS` reader - Internal Reference Clock Select"]
pub struct IRCS_R(crate::FieldReader<bool, IRCS_A>);
impl IRCS_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRCS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRCS_A {
        match self.bits {
            false => IRCS_A::_0,
            true => IRCS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IRCS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IRCS_A::_1
    }
}
impl core::ops::Deref for IRCS_R {
    type Target = crate::FieldReader<bool, IRCS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRCS` writer - Internal Reference Clock Select"]
pub struct IRCS_W<'a> {
    w: &'a mut W,
}
impl<'a> IRCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRCS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Slow internal reference clock selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRCS_A::_0)
    }
    #[doc = "Fast internal reference clock selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRCS_A::_1)
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
#[doc = "Low Power Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LP_A {
    #[doc = "0: FLL is not disabled in bypass modes."]
    _0 = 0,
    #[doc = "1: FLL is disabled in bypass modes (lower power)"]
    _1 = 1,
}
impl From<LP_A> for bool {
    #[inline(always)]
    fn from(variant: LP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LP` reader - Low Power Select"]
pub struct LP_R(crate::FieldReader<bool, LP_A>);
impl LP_R {
    pub(crate) fn new(bits: bool) -> Self {
        LP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LP_A {
        match self.bits {
            false => LP_A::_0,
            true => LP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LP_A::_1
    }
}
impl core::ops::Deref for LP_R {
    type Target = crate::FieldReader<bool, LP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LP` writer - Low Power Select"]
pub struct LP_W<'a> {
    w: &'a mut W,
}
impl<'a> LP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FLL is not disabled in bypass modes."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LP_A::_0)
    }
    #[doc = "FLL is disabled in bypass modes (lower power)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LP_A::_1)
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
#[doc = "External Reference Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EREFS0_A {
    #[doc = "0: External reference clock requested."]
    _0 = 0,
    #[doc = "1: Oscillator requested."]
    _1 = 1,
}
impl From<EREFS0_A> for bool {
    #[inline(always)]
    fn from(variant: EREFS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EREFS0` reader - External Reference Select"]
pub struct EREFS0_R(crate::FieldReader<bool, EREFS0_A>);
impl EREFS0_R {
    pub(crate) fn new(bits: bool) -> Self {
        EREFS0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EREFS0_A {
        match self.bits {
            false => EREFS0_A::_0,
            true => EREFS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EREFS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EREFS0_A::_1
    }
}
impl core::ops::Deref for EREFS0_R {
    type Target = crate::FieldReader<bool, EREFS0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EREFS0` writer - External Reference Select"]
pub struct EREFS0_W<'a> {
    w: &'a mut W,
}
impl<'a> EREFS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EREFS0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External reference clock requested."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EREFS0_A::_0)
    }
    #[doc = "Oscillator requested."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EREFS0_A::_1)
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
#[doc = "High Gain Oscillator Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HGO0_A {
    #[doc = "0: Configure crystal oscillator for low-power operation."]
    _0 = 0,
    #[doc = "1: Configure crystal oscillator for high-gain operation."]
    _1 = 1,
}
impl From<HGO0_A> for bool {
    #[inline(always)]
    fn from(variant: HGO0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HGO0` reader - High Gain Oscillator Select"]
pub struct HGO0_R(crate::FieldReader<bool, HGO0_A>);
impl HGO0_R {
    pub(crate) fn new(bits: bool) -> Self {
        HGO0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HGO0_A {
        match self.bits {
            false => HGO0_A::_0,
            true => HGO0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HGO0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HGO0_A::_1
    }
}
impl core::ops::Deref for HGO0_R {
    type Target = crate::FieldReader<bool, HGO0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HGO0` writer - High Gain Oscillator Select"]
pub struct HGO0_W<'a> {
    w: &'a mut W,
}
impl<'a> HGO0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HGO0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Configure crystal oscillator for low-power operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HGO0_A::_0)
    }
    #[doc = "Configure crystal oscillator for high-gain operation."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HGO0_A::_1)
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
#[doc = "Frequency Range Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RANGE0_A {
    #[doc = "0: Encoding 0 - Low frequency range selected for the crystal oscillator ."]
    _00 = 0,
    #[doc = "1: Encoding 1 - High frequency range selected for the crystal oscillator ."]
    _01 = 1,
}
impl From<RANGE0_A> for u8 {
    #[inline(always)]
    fn from(variant: RANGE0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RANGE0` reader - Frequency Range Select"]
pub struct RANGE0_R(crate::FieldReader<u8, RANGE0_A>);
impl RANGE0_R {
    pub(crate) fn new(bits: u8) -> Self {
        RANGE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RANGE0_A> {
        match self.bits {
            0 => Some(RANGE0_A::_00),
            1 => Some(RANGE0_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == RANGE0_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == RANGE0_A::_01
    }
}
impl core::ops::Deref for RANGE0_R {
    type Target = crate::FieldReader<u8, RANGE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RANGE0` writer - Frequency Range Select"]
pub struct RANGE0_W<'a> {
    w: &'a mut W,
}
impl<'a> RANGE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RANGE0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Encoding 0 - Low frequency range selected for the crystal oscillator ."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(RANGE0_A::_00)
    }
    #[doc = "Encoding 1 - High frequency range selected for the crystal oscillator ."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(RANGE0_A::_01)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u8 & 0x03) << 4);
        self.w
    }
}
#[doc = "Loss of Clock Reset Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCRE0_A {
    #[doc = "0: Interrupt request is generated on a loss of OSC0 external reference clock."]
    _0 = 0,
    #[doc = "1: Generate a reset request on a loss of OSC0 external reference clock."]
    _1 = 1,
}
impl From<LOCRE0_A> for bool {
    #[inline(always)]
    fn from(variant: LOCRE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCRE0` reader - Loss of Clock Reset Enable"]
pub struct LOCRE0_R(crate::FieldReader<bool, LOCRE0_A>);
impl LOCRE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCRE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCRE0_A {
        match self.bits {
            false => LOCRE0_A::_0,
            true => LOCRE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LOCRE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LOCRE0_A::_1
    }
}
impl core::ops::Deref for LOCRE0_R {
    type Target = crate::FieldReader<bool, LOCRE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCRE0` writer - Loss of Clock Reset Enable"]
pub struct LOCRE0_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCRE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCRE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request is generated on a loss of OSC0 external reference clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOCRE0_A::_0)
    }
    #[doc = "Generate a reset request on a loss of OSC0 external reference clock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOCRE0_A::_1)
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
    #[doc = "Bit 0 - Internal Reference Clock Select"]
    #[inline(always)]
    pub fn ircs(&self) -> IRCS_R {
        IRCS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Low Power Select"]
    #[inline(always)]
    pub fn lp(&self) -> LP_R {
        LP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External Reference Select"]
    #[inline(always)]
    pub fn erefs0(&self) -> EREFS0_R {
        EREFS0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - High Gain Oscillator Select"]
    #[inline(always)]
    pub fn hgo0(&self) -> HGO0_R {
        HGO0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Frequency Range Select"]
    #[inline(always)]
    pub fn range0(&self) -> RANGE0_R {
        RANGE0_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Loss of Clock Reset Enable"]
    #[inline(always)]
    pub fn locre0(&self) -> LOCRE0_R {
        LOCRE0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Reference Clock Select"]
    #[inline(always)]
    pub fn ircs(&mut self) -> IRCS_W {
        IRCS_W { w: self }
    }
    #[doc = "Bit 1 - Low Power Select"]
    #[inline(always)]
    pub fn lp(&mut self) -> LP_W {
        LP_W { w: self }
    }
    #[doc = "Bit 2 - External Reference Select"]
    #[inline(always)]
    pub fn erefs0(&mut self) -> EREFS0_W {
        EREFS0_W { w: self }
    }
    #[doc = "Bit 3 - High Gain Oscillator Select"]
    #[inline(always)]
    pub fn hgo0(&mut self) -> HGO0_W {
        HGO0_W { w: self }
    }
    #[doc = "Bits 4:5 - Frequency Range Select"]
    #[inline(always)]
    pub fn range0(&mut self) -> RANGE0_W {
        RANGE0_W { w: self }
    }
    #[doc = "Bit 7 - Loss of Clock Reset Enable"]
    #[inline(always)]
    pub fn locre0(&mut self) -> LOCRE0_W {
        LOCRE0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCG Control 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2](index.html) module"]
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
#[doc = "`reset()` method sets C2 to value 0x80"]
impl crate::Resettable for C2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
