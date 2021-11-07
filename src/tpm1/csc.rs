#[doc = "Register `C%sSC` reader"]
pub struct R(crate::R<CSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C%sSC` writer"]
pub struct W(crate::W<CSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSC_SPEC>;
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
impl From<crate::W<CSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_A {
    #[doc = "0: Disable DMA transfers."]
    _0 = 0,
    #[doc = "1: Enable DMA transfers."]
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
    #[doc = "Disable DMA transfers."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMA_A::_0)
    }
    #[doc = "Enable DMA transfers."]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `ELSA` reader - Edge or Level Select"]
pub struct ELSA_R(crate::FieldReader<bool, bool>);
impl ELSA_R {
    pub(crate) fn new(bits: bool) -> Self {
        ELSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ELSA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ELSA` writer - Edge or Level Select"]
pub struct ELSA_W<'a> {
    w: &'a mut W,
}
impl<'a> ELSA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `ELSB` reader - Edge or Level Select"]
pub struct ELSB_R(crate::FieldReader<bool, bool>);
impl ELSB_R {
    pub(crate) fn new(bits: bool) -> Self {
        ELSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ELSB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ELSB` writer - Edge or Level Select"]
pub struct ELSB_W<'a> {
    w: &'a mut W,
}
impl<'a> ELSB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `MSA` reader - Channel Mode Select"]
pub struct MSA_R(crate::FieldReader<bool, bool>);
impl MSA_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSA` writer - Channel Mode Select"]
pub struct MSA_W<'a> {
    w: &'a mut W,
}
impl<'a> MSA_W<'a> {
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
#[doc = "Field `MSB` reader - Channel Mode Select"]
pub struct MSB_R(crate::FieldReader<bool, bool>);
impl MSB_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSB` writer - Channel Mode Select"]
pub struct MSB_W<'a> {
    w: &'a mut W,
}
impl<'a> MSB_W<'a> {
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
#[doc = "Channel Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHIE_A {
    #[doc = "0: Disable channel interrupts."]
    _0 = 0,
    #[doc = "1: Enable channel interrupts."]
    _1 = 1,
}
impl From<CHIE_A> for bool {
    #[inline(always)]
    fn from(variant: CHIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHIE` reader - Channel Interrupt Enable"]
pub struct CHIE_R(crate::FieldReader<bool, CHIE_A>);
impl CHIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHIE_A {
        match self.bits {
            false => CHIE_A::_0,
            true => CHIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CHIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CHIE_A::_1
    }
}
impl core::ops::Deref for CHIE_R {
    type Target = crate::FieldReader<bool, CHIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHIE` writer - Channel Interrupt Enable"]
pub struct CHIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable channel interrupts."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHIE_A::_0)
    }
    #[doc = "Enable channel interrupts."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHIE_A::_1)
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
#[doc = "Channel Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHF_A {
    #[doc = "0: No channel event has occurred."]
    _0 = 0,
    #[doc = "1: A channel event has occurred."]
    _1 = 1,
}
impl From<CHF_A> for bool {
    #[inline(always)]
    fn from(variant: CHF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHF` reader - Channel Flag"]
pub struct CHF_R(crate::FieldReader<bool, CHF_A>);
impl CHF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHF_A {
        match self.bits {
            false => CHF_A::_0,
            true => CHF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CHF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CHF_A::_1
    }
}
impl core::ops::Deref for CHF_R {
    type Target = crate::FieldReader<bool, CHF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHF` writer - Channel Flag"]
pub struct CHF_W<'a> {
    w: &'a mut W,
}
impl<'a> CHF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHF_A::_0)
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHF_A::_1)
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
impl R {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Edge or Level Select"]
    #[inline(always)]
    pub fn elsa(&self) -> ELSA_R {
        ELSA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Edge or Level Select"]
    #[inline(always)]
    pub fn elsb(&self) -> ELSB_R {
        ELSB_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel Mode Select"]
    #[inline(always)]
    pub fn msa(&self) -> MSA_R {
        MSA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel Mode Select"]
    #[inline(always)]
    pub fn msb(&self) -> MSB_R {
        MSB_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel Interrupt Enable"]
    #[inline(always)]
    pub fn chie(&self) -> CHIE_R {
        CHIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel Flag"]
    #[inline(always)]
    pub fn chf(&self) -> CHF_R {
        CHF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W { w: self }
    }
    #[doc = "Bit 2 - Edge or Level Select"]
    #[inline(always)]
    pub fn elsa(&mut self) -> ELSA_W {
        ELSA_W { w: self }
    }
    #[doc = "Bit 3 - Edge or Level Select"]
    #[inline(always)]
    pub fn elsb(&mut self) -> ELSB_W {
        ELSB_W { w: self }
    }
    #[doc = "Bit 4 - Channel Mode Select"]
    #[inline(always)]
    pub fn msa(&mut self) -> MSA_W {
        MSA_W { w: self }
    }
    #[doc = "Bit 5 - Channel Mode Select"]
    #[inline(always)]
    pub fn msb(&mut self) -> MSB_W {
        MSB_W { w: self }
    }
    #[doc = "Bit 6 - Channel Interrupt Enable"]
    #[inline(always)]
    pub fn chie(&mut self) -> CHIE_W {
        CHIE_W { w: self }
    }
    #[doc = "Bit 7 - Channel Flag"]
    #[inline(always)]
    pub fn chf(&mut self) -> CHF_W {
        CHF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel (n) Status and Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csc](index.html) module"]
pub struct CSC_SPEC;
impl crate::RegisterSpec for CSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csc::R](R) reader structure"]
impl crate::Readable for CSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csc::W](W) writer structure"]
impl crate::Writable for CSC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C%sSC to value 0"]
impl crate::Resettable for CSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
