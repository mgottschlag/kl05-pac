#[doc = "Register `C1` reader"]
pub struct R(crate::R<C1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1` writer"]
pub struct W(crate::W<C1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1_SPEC>;
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
impl From<crate::W<C1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DAC Buffer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACBFEN_A {
    #[doc = "0: Buffer read pointer is disabled. The converted data is always the first word of the buffer."]
    _0 = 0,
    #[doc = "1: Buffer read pointer is enabled. The converted data is the word that the read pointer points to. It means converted data can be from any word of the buffer."]
    _1 = 1,
}
impl From<DACBFEN_A> for bool {
    #[inline(always)]
    fn from(variant: DACBFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACBFEN` reader - DAC Buffer Enable"]
pub struct DACBFEN_R(crate::FieldReader<bool, DACBFEN_A>);
impl DACBFEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DACBFEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACBFEN_A {
        match self.bits {
            false => DACBFEN_A::_0,
            true => DACBFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DACBFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DACBFEN_A::_1
    }
}
impl core::ops::Deref for DACBFEN_R {
    type Target = crate::FieldReader<bool, DACBFEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACBFEN` writer - DAC Buffer Enable"]
pub struct DACBFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DACBFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACBFEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Buffer read pointer is disabled. The converted data is always the first word of the buffer."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACBFEN_A::_0)
    }
    #[doc = "Buffer read pointer is enabled. The converted data is the word that the read pointer points to. It means converted data can be from any word of the buffer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACBFEN_A::_1)
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
#[doc = "DAC Buffer Work Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACBFMD_A {
    #[doc = "0: Normal mode"]
    _0 = 0,
    #[doc = "1: One-Time Scan mode"]
    _1 = 1,
}
impl From<DACBFMD_A> for bool {
    #[inline(always)]
    fn from(variant: DACBFMD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACBFMD` reader - DAC Buffer Work Mode Select"]
pub struct DACBFMD_R(crate::FieldReader<bool, DACBFMD_A>);
impl DACBFMD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DACBFMD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACBFMD_A {
        match self.bits {
            false => DACBFMD_A::_0,
            true => DACBFMD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DACBFMD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DACBFMD_A::_1
    }
}
impl core::ops::Deref for DACBFMD_R {
    type Target = crate::FieldReader<bool, DACBFMD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACBFMD` writer - DAC Buffer Work Mode Select"]
pub struct DACBFMD_W<'a> {
    w: &'a mut W,
}
impl<'a> DACBFMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACBFMD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACBFMD_A::_0)
    }
    #[doc = "One-Time Scan mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACBFMD_A::_1)
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
#[doc = "DMA Enable Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN_A {
    #[doc = "0: DMA is disabled."]
    _0 = 0,
    #[doc = "1: DMA is enabled. When DMA is enabled, the DMA request will be generated by original interrupts. The interrupts will not be presented on this module at the same time."]
    _1 = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - DMA Enable Select"]
pub struct DMAEN_R(crate::FieldReader<bool, DMAEN_A>);
impl DMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::_0,
            true => DMAEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DMAEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DMAEN_A::_1
    }
}
impl core::ops::Deref for DMAEN_R {
    type Target = crate::FieldReader<bool, DMAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAEN` writer - DMA Enable Select"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAEN_A::_0)
    }
    #[doc = "DMA is enabled. When DMA is enabled, the DMA request will be generated by original interrupts. The interrupts will not be presented on this module at the same time."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAEN_A::_1)
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
    #[doc = "Bit 0 - DAC Buffer Enable"]
    #[inline(always)]
    pub fn dacbfen(&self) -> DACBFEN_R {
        DACBFEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - DAC Buffer Work Mode Select"]
    #[inline(always)]
    pub fn dacbfmd(&self) -> DACBFMD_R {
        DACBFMD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DMA Enable Select"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC Buffer Enable"]
    #[inline(always)]
    pub fn dacbfen(&mut self) -> DACBFEN_W {
        DACBFEN_W { w: self }
    }
    #[doc = "Bit 2 - DAC Buffer Work Mode Select"]
    #[inline(always)]
    pub fn dacbfmd(&mut self) -> DACBFMD_W {
        DACBFMD_W { w: self }
    }
    #[doc = "Bit 7 - DMA Enable Select"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1](index.html) module"]
pub struct C1_SPEC;
impl crate::RegisterSpec for C1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c1::R](R) reader structure"]
impl crate::Readable for C1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1::W](W) writer structure"]
impl crate::Writable for C1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C1 to value 0"]
impl crate::Resettable for C1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
