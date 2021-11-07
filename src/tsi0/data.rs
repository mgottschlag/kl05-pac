#[doc = "Register `DATA` reader"]
pub struct R(crate::R<DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA` writer"]
pub struct W(crate::W<DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_SPEC>;
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
impl From<crate::W<DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSICNT` reader - TSI Conversion Counter Value"]
pub struct TSICNT_R(crate::FieldReader<u16, u16>);
impl TSICNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        TSICNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSICNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Software Trigger Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWTS_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Start a scan to determine which channel is specified by TSI_DATA\\[TSICH\\]."]
    _1 = 1,
}
impl From<SWTS_AW> for bool {
    #[inline(always)]
    fn from(variant: SWTS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWTS` writer - Software Trigger Start"]
pub struct SWTS_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWTS_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWTS_AW::_0)
    }
    #[doc = "Start a scan to determine which channel is specified by TSI_DATA\\[TSICH\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWTS_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "DMA Transfer Enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN_A {
    #[doc = "0: Interrupt is selected when the interrupt enable bit is set and the corresponding TSI events assert."]
    _0 = 0,
    #[doc = "1: DMA transfer request is selected when the interrupt enable bit is set and the corresponding TSI events assert."]
    _1 = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - DMA Transfer Enabled"]
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
#[doc = "Field `DMAEN` writer - DMA Transfer Enabled"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt is selected when the interrupt enable bit is set and the corresponding TSI events assert."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAEN_A::_0)
    }
    #[doc = "DMA transfer request is selected when the interrupt enable bit is set and the corresponding TSI events assert."]
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "TSICH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSICH_A {
    #[doc = "0: Channel 0."]
    _0000 = 0,
    #[doc = "1: Channel 1."]
    _0001 = 1,
    #[doc = "2: Channel 2."]
    _0010 = 2,
    #[doc = "3: Channel 3."]
    _0011 = 3,
    #[doc = "4: Channel 4."]
    _0100 = 4,
    #[doc = "5: Channel 5."]
    _0101 = 5,
    #[doc = "6: Channel 6."]
    _0110 = 6,
    #[doc = "7: Channel 7."]
    _0111 = 7,
    #[doc = "8: Channel 8."]
    _1000 = 8,
    #[doc = "9: Channel 9."]
    _1001 = 9,
    #[doc = "10: Channel 10."]
    _1010 = 10,
    #[doc = "11: Channel 11."]
    _1011 = 11,
    #[doc = "12: Channel 12."]
    _1100 = 12,
    #[doc = "13: Channel 13."]
    _1101 = 13,
    #[doc = "14: Channel 14."]
    _1110 = 14,
    #[doc = "15: Channel 15."]
    _1111 = 15,
}
impl From<TSICH_A> for u8 {
    #[inline(always)]
    fn from(variant: TSICH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TSICH` reader - TSICH"]
pub struct TSICH_R(crate::FieldReader<u8, TSICH_A>);
impl TSICH_R {
    pub(crate) fn new(bits: u8) -> Self {
        TSICH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSICH_A {
        match self.bits {
            0 => TSICH_A::_0000,
            1 => TSICH_A::_0001,
            2 => TSICH_A::_0010,
            3 => TSICH_A::_0011,
            4 => TSICH_A::_0100,
            5 => TSICH_A::_0101,
            6 => TSICH_A::_0110,
            7 => TSICH_A::_0111,
            8 => TSICH_A::_1000,
            9 => TSICH_A::_1001,
            10 => TSICH_A::_1010,
            11 => TSICH_A::_1011,
            12 => TSICH_A::_1100,
            13 => TSICH_A::_1101,
            14 => TSICH_A::_1110,
            15 => TSICH_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        **self == TSICH_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        **self == TSICH_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        **self == TSICH_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        **self == TSICH_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        **self == TSICH_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        **self == TSICH_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        **self == TSICH_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        **self == TSICH_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        **self == TSICH_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        **self == TSICH_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        **self == TSICH_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        **self == TSICH_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        **self == TSICH_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        **self == TSICH_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        **self == TSICH_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        **self == TSICH_A::_1111
    }
}
impl core::ops::Deref for TSICH_R {
    type Target = crate::FieldReader<u8, TSICH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSICH` writer - TSICH"]
pub struct TSICH_W<'a> {
    w: &'a mut W,
}
impl<'a> TSICH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSICH_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Channel 0."]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(TSICH_A::_0000)
    }
    #[doc = "Channel 1."]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(TSICH_A::_0001)
    }
    #[doc = "Channel 2."]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(TSICH_A::_0010)
    }
    #[doc = "Channel 3."]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(TSICH_A::_0011)
    }
    #[doc = "Channel 4."]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(TSICH_A::_0100)
    }
    #[doc = "Channel 5."]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(TSICH_A::_0101)
    }
    #[doc = "Channel 6."]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(TSICH_A::_0110)
    }
    #[doc = "Channel 7."]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(TSICH_A::_0111)
    }
    #[doc = "Channel 8."]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(TSICH_A::_1000)
    }
    #[doc = "Channel 9."]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(TSICH_A::_1001)
    }
    #[doc = "Channel 10."]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(TSICH_A::_1010)
    }
    #[doc = "Channel 11."]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(TSICH_A::_1011)
    }
    #[doc = "Channel 12."]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(TSICH_A::_1100)
    }
    #[doc = "Channel 13."]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(TSICH_A::_1101)
    }
    #[doc = "Channel 14."]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(TSICH_A::_1110)
    }
    #[doc = "Channel 15."]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(TSICH_A::_1111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - TSI Conversion Counter Value"]
    #[inline(always)]
    pub fn tsicnt(&self) -> TSICNT_R {
        TSICNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 23 - DMA Transfer Enabled"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 28:31 - TSICH"]
    #[inline(always)]
    pub fn tsich(&self) -> TSICH_R {
        TSICH_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 22 - Software Trigger Start"]
    #[inline(always)]
    pub fn swts(&mut self) -> SWTS_W {
        SWTS_W { w: self }
    }
    #[doc = "Bit 23 - DMA Transfer Enabled"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Bits 28:31 - TSICH"]
    #[inline(always)]
    pub fn tsich(&mut self) -> TSICH_W {
        TSICH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TSI DATA Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](index.html) module"]
pub struct DATA_SPEC;
impl crate::RegisterSpec for DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data::R](R) reader structure"]
impl crate::Readable for DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data::W](W) writer structure"]
impl crate::Writable for DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA to value 0"]
impl crate::Resettable for DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
