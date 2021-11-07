#[doc = "Register `C0` reader"]
pub struct R(crate::R<C0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C0` writer"]
pub struct W(crate::W<C0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C0_SPEC>;
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
impl From<crate::W<C0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DAC Buffer Read Pointer Bottom Flag Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACBBIEN_A {
    #[doc = "0: The DAC buffer read pointer bottom flag interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The DAC buffer read pointer bottom flag interrupt is enabled."]
    _1 = 1,
}
impl From<DACBBIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DACBBIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACBBIEN` reader - DAC Buffer Read Pointer Bottom Flag Interrupt Enable"]
pub struct DACBBIEN_R(crate::FieldReader<bool, DACBBIEN_A>);
impl DACBBIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DACBBIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACBBIEN_A {
        match self.bits {
            false => DACBBIEN_A::_0,
            true => DACBBIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DACBBIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DACBBIEN_A::_1
    }
}
impl core::ops::Deref for DACBBIEN_R {
    type Target = crate::FieldReader<bool, DACBBIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACBBIEN` writer - DAC Buffer Read Pointer Bottom Flag Interrupt Enable"]
pub struct DACBBIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DACBBIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACBBIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DAC buffer read pointer bottom flag interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACBBIEN_A::_0)
    }
    #[doc = "The DAC buffer read pointer bottom flag interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACBBIEN_A::_1)
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
#[doc = "DAC Buffer Read Pointer Top Flag Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACBTIEN_A {
    #[doc = "0: The DAC buffer read pointer top flag interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The DAC buffer read pointer top flag interrupt is enabled."]
    _1 = 1,
}
impl From<DACBTIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DACBTIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACBTIEN` reader - DAC Buffer Read Pointer Top Flag Interrupt Enable"]
pub struct DACBTIEN_R(crate::FieldReader<bool, DACBTIEN_A>);
impl DACBTIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DACBTIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACBTIEN_A {
        match self.bits {
            false => DACBTIEN_A::_0,
            true => DACBTIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DACBTIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DACBTIEN_A::_1
    }
}
impl core::ops::Deref for DACBTIEN_R {
    type Target = crate::FieldReader<bool, DACBTIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACBTIEN` writer - DAC Buffer Read Pointer Top Flag Interrupt Enable"]
pub struct DACBTIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DACBTIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACBTIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DAC buffer read pointer top flag interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACBTIEN_A::_0)
    }
    #[doc = "The DAC buffer read pointer top flag interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACBTIEN_A::_1)
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
#[doc = "DAC Low Power Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPEN_A {
    #[doc = "0: High-Power mode"]
    _0 = 0,
    #[doc = "1: Low-Power mode"]
    _1 = 1,
}
impl From<LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPEN` reader - DAC Low Power Control"]
pub struct LPEN_R(crate::FieldReader<bool, LPEN_A>);
impl LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPEN_A {
        match self.bits {
            false => LPEN_A::_0,
            true => LPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LPEN_A::_1
    }
}
impl core::ops::Deref for LPEN_R {
    type Target = crate::FieldReader<bool, LPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPEN` writer - DAC Low Power Control"]
pub struct LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "High-Power mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPEN_A::_0)
    }
    #[doc = "Low-Power mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPEN_A::_1)
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
#[doc = "DAC Software Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACSWTRG_AW {
    #[doc = "0: The DAC soft trigger is not valid."]
    _0 = 0,
    #[doc = "1: The DAC soft trigger is valid."]
    _1 = 1,
}
impl From<DACSWTRG_AW> for bool {
    #[inline(always)]
    fn from(variant: DACSWTRG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACSWTRG` writer - DAC Software Trigger"]
pub struct DACSWTRG_W<'a> {
    w: &'a mut W,
}
impl<'a> DACSWTRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACSWTRG_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DAC soft trigger is not valid."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACSWTRG_AW::_0)
    }
    #[doc = "The DAC soft trigger is valid."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACSWTRG_AW::_1)
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
#[doc = "DAC Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACTRGSEL_A {
    #[doc = "0: The DAC hardware trigger is selected."]
    _0 = 0,
    #[doc = "1: The DAC software trigger is selected."]
    _1 = 1,
}
impl From<DACTRGSEL_A> for bool {
    #[inline(always)]
    fn from(variant: DACTRGSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACTRGSEL` reader - DAC Trigger Select"]
pub struct DACTRGSEL_R(crate::FieldReader<bool, DACTRGSEL_A>);
impl DACTRGSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DACTRGSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACTRGSEL_A {
        match self.bits {
            false => DACTRGSEL_A::_0,
            true => DACTRGSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DACTRGSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DACTRGSEL_A::_1
    }
}
impl core::ops::Deref for DACTRGSEL_R {
    type Target = crate::FieldReader<bool, DACTRGSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACTRGSEL` writer - DAC Trigger Select"]
pub struct DACTRGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DACTRGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACTRGSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DAC hardware trigger is selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACTRGSEL_A::_0)
    }
    #[doc = "The DAC software trigger is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACTRGSEL_A::_1)
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
#[doc = "DAC Reference Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACRFS_A {
    #[doc = "0: The DAC selects DACREF_1 as the reference voltage."]
    _0 = 0,
    #[doc = "1: The DAC selects DACREF_2 as the reference voltage."]
    _1 = 1,
}
impl From<DACRFS_A> for bool {
    #[inline(always)]
    fn from(variant: DACRFS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACRFS` reader - DAC Reference Select"]
pub struct DACRFS_R(crate::FieldReader<bool, DACRFS_A>);
impl DACRFS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DACRFS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACRFS_A {
        match self.bits {
            false => DACRFS_A::_0,
            true => DACRFS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DACRFS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DACRFS_A::_1
    }
}
impl core::ops::Deref for DACRFS_R {
    type Target = crate::FieldReader<bool, DACRFS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACRFS` writer - DAC Reference Select"]
pub struct DACRFS_W<'a> {
    w: &'a mut W,
}
impl<'a> DACRFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACRFS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DAC selects DACREF_1 as the reference voltage."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACRFS_A::_0)
    }
    #[doc = "The DAC selects DACREF_2 as the reference voltage."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACRFS_A::_1)
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
#[doc = "DAC Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACEN_A {
    #[doc = "0: The DAC system is disabled."]
    _0 = 0,
    #[doc = "1: The DAC system is enabled."]
    _1 = 1,
}
impl From<DACEN_A> for bool {
    #[inline(always)]
    fn from(variant: DACEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACEN` reader - DAC Enable"]
pub struct DACEN_R(crate::FieldReader<bool, DACEN_A>);
impl DACEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DACEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACEN_A {
        match self.bits {
            false => DACEN_A::_0,
            true => DACEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DACEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DACEN_A::_1
    }
}
impl core::ops::Deref for DACEN_R {
    type Target = crate::FieldReader<bool, DACEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACEN` writer - DAC Enable"]
pub struct DACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DACEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DAC system is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACEN_A::_0)
    }
    #[doc = "The DAC system is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACEN_A::_1)
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
    #[doc = "Bit 0 - DAC Buffer Read Pointer Bottom Flag Interrupt Enable"]
    #[inline(always)]
    pub fn dacbbien(&self) -> DACBBIEN_R {
        DACBBIEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DAC Buffer Read Pointer Top Flag Interrupt Enable"]
    #[inline(always)]
    pub fn dacbtien(&self) -> DACBTIEN_R {
        DACBTIEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DAC Low Power Control"]
    #[inline(always)]
    pub fn lpen(&self) -> LPEN_R {
        LPEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DAC Trigger Select"]
    #[inline(always)]
    pub fn dactrgsel(&self) -> DACTRGSEL_R {
        DACTRGSEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DAC Reference Select"]
    #[inline(always)]
    pub fn dacrfs(&self) -> DACRFS_R {
        DACRFS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DAC Enable"]
    #[inline(always)]
    pub fn dacen(&self) -> DACEN_R {
        DACEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC Buffer Read Pointer Bottom Flag Interrupt Enable"]
    #[inline(always)]
    pub fn dacbbien(&mut self) -> DACBBIEN_W {
        DACBBIEN_W { w: self }
    }
    #[doc = "Bit 1 - DAC Buffer Read Pointer Top Flag Interrupt Enable"]
    #[inline(always)]
    pub fn dacbtien(&mut self) -> DACBTIEN_W {
        DACBTIEN_W { w: self }
    }
    #[doc = "Bit 3 - DAC Low Power Control"]
    #[inline(always)]
    pub fn lpen(&mut self) -> LPEN_W {
        LPEN_W { w: self }
    }
    #[doc = "Bit 4 - DAC Software Trigger"]
    #[inline(always)]
    pub fn dacswtrg(&mut self) -> DACSWTRG_W {
        DACSWTRG_W { w: self }
    }
    #[doc = "Bit 5 - DAC Trigger Select"]
    #[inline(always)]
    pub fn dactrgsel(&mut self) -> DACTRGSEL_W {
        DACTRGSEL_W { w: self }
    }
    #[doc = "Bit 6 - DAC Reference Select"]
    #[inline(always)]
    pub fn dacrfs(&mut self) -> DACRFS_W {
        DACRFS_W { w: self }
    }
    #[doc = "Bit 7 - DAC Enable"]
    #[inline(always)]
    pub fn dacen(&mut self) -> DACEN_W {
        DACEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c0](index.html) module"]
pub struct C0_SPEC;
impl crate::RegisterSpec for C0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c0::R](R) reader structure"]
impl crate::Readable for C0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c0::W](W) writer structure"]
impl crate::Writable for C0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C0 to value 0"]
impl crate::Resettable for C0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
