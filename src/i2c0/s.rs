#[doc = "Register `S` reader"]
pub struct R(crate::R<S_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S` writer"]
pub struct W(crate::W<S_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S_SPEC>;
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
impl From<crate::W<S_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Receive Acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXAK_A {
    #[doc = "0: Acknowledge signal was received after the completion of one byte of data transmission on the bus"]
    _0 = 0,
    #[doc = "1: No acknowledge signal detected"]
    _1 = 1,
}
impl From<RXAK_A> for bool {
    #[inline(always)]
    fn from(variant: RXAK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXAK` reader - Receive Acknowledge"]
pub struct RXAK_R(crate::FieldReader<bool, RXAK_A>);
impl RXAK_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXAK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXAK_A {
        match self.bits {
            false => RXAK_A::_0,
            true => RXAK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXAK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXAK_A::_1
    }
}
impl core::ops::Deref for RXAK_R {
    type Target = crate::FieldReader<bool, RXAK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IICIF_A {
    #[doc = "0: No interrupt pending"]
    _0 = 0,
    #[doc = "1: Interrupt pending"]
    _1 = 1,
}
impl From<IICIF_A> for bool {
    #[inline(always)]
    fn from(variant: IICIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IICIF` reader - Interrupt Flag"]
pub struct IICIF_R(crate::FieldReader<bool, IICIF_A>);
impl IICIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        IICIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IICIF_A {
        match self.bits {
            false => IICIF_A::_0,
            true => IICIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IICIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IICIF_A::_1
    }
}
impl core::ops::Deref for IICIF_R {
    type Target = crate::FieldReader<bool, IICIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IICIF` writer - Interrupt Flag"]
pub struct IICIF_W<'a> {
    w: &'a mut W,
}
impl<'a> IICIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IICIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IICIF_A::_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IICIF_A::_1)
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
#[doc = "Slave Read/Write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRW_A {
    #[doc = "0: Slave receive, master writing to slave"]
    _0 = 0,
    #[doc = "1: Slave transmit, master reading from slave"]
    _1 = 1,
}
impl From<SRW_A> for bool {
    #[inline(always)]
    fn from(variant: SRW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRW` reader - Slave Read/Write"]
pub struct SRW_R(crate::FieldReader<bool, SRW_A>);
impl SRW_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRW_A {
        match self.bits {
            false => SRW_A::_0,
            true => SRW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SRW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SRW_A::_1
    }
}
impl core::ops::Deref for SRW_R {
    type Target = crate::FieldReader<bool, SRW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Range Address Match\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_A {
    #[doc = "0: Not addressed"]
    _0 = 0,
    #[doc = "1: Addressed as a slave"]
    _1 = 1,
}
impl From<RAM_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM` reader - Range Address Match"]
pub struct RAM_R(crate::FieldReader<bool, RAM_A>);
impl RAM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_A {
        match self.bits {
            false => RAM_A::_0,
            true => RAM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RAM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RAM_A::_1
    }
}
impl core::ops::Deref for RAM_R {
    type Target = crate::FieldReader<bool, RAM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM` writer - Range Address Match"]
pub struct RAM_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not addressed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RAM_A::_0)
    }
    #[doc = "Addressed as a slave"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RAM_A::_1)
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
#[doc = "Arbitration Lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARBL_A {
    #[doc = "0: Standard bus operation."]
    _0 = 0,
    #[doc = "1: Loss of arbitration."]
    _1 = 1,
}
impl From<ARBL_A> for bool {
    #[inline(always)]
    fn from(variant: ARBL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARBL` reader - Arbitration Lost"]
pub struct ARBL_R(crate::FieldReader<bool, ARBL_A>);
impl ARBL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ARBL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARBL_A {
        match self.bits {
            false => ARBL_A::_0,
            true => ARBL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ARBL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ARBL_A::_1
    }
}
impl core::ops::Deref for ARBL_R {
    type Target = crate::FieldReader<bool, ARBL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARBL` writer - Arbitration Lost"]
pub struct ARBL_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARBL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Standard bus operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ARBL_A::_0)
    }
    #[doc = "Loss of arbitration."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ARBL_A::_1)
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
#[doc = "Bus Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: Bus is idle"]
    _0 = 0,
    #[doc = "1: Bus is busy"]
    _1 = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Bus Busy"]
pub struct BUSY_R(crate::FieldReader<bool, BUSY_A>);
impl BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::_0,
            true => BUSY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BUSY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BUSY_A::_1
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, BUSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Addressed As A Slave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IAAS_A {
    #[doc = "0: Not addressed"]
    _0 = 0,
    #[doc = "1: Addressed as a slave"]
    _1 = 1,
}
impl From<IAAS_A> for bool {
    #[inline(always)]
    fn from(variant: IAAS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IAAS` reader - Addressed As A Slave"]
pub struct IAAS_R(crate::FieldReader<bool, IAAS_A>);
impl IAAS_R {
    pub(crate) fn new(bits: bool) -> Self {
        IAAS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IAAS_A {
        match self.bits {
            false => IAAS_A::_0,
            true => IAAS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IAAS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IAAS_A::_1
    }
}
impl core::ops::Deref for IAAS_R {
    type Target = crate::FieldReader<bool, IAAS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IAAS` writer - Addressed As A Slave"]
pub struct IAAS_W<'a> {
    w: &'a mut W,
}
impl<'a> IAAS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IAAS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not addressed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IAAS_A::_0)
    }
    #[doc = "Addressed as a slave"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IAAS_A::_1)
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
#[doc = "Transfer Complete Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCF_A {
    #[doc = "0: Transfer in progress"]
    _0 = 0,
    #[doc = "1: Transfer complete"]
    _1 = 1,
}
impl From<TCF_A> for bool {
    #[inline(always)]
    fn from(variant: TCF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCF` reader - Transfer Complete Flag"]
pub struct TCF_R(crate::FieldReader<bool, TCF_A>);
impl TCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCF_A {
        match self.bits {
            false => TCF_A::_0,
            true => TCF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TCF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TCF_A::_1
    }
}
impl core::ops::Deref for TCF_R {
    type Target = crate::FieldReader<bool, TCF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Receive Acknowledge"]
    #[inline(always)]
    pub fn rxak(&self) -> RXAK_R {
        RXAK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt Flag"]
    #[inline(always)]
    pub fn iicif(&self) -> IICIF_R {
        IICIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Slave Read/Write"]
    #[inline(always)]
    pub fn srw(&self) -> SRW_R {
        SRW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Range Address Match"]
    #[inline(always)]
    pub fn ram(&self) -> RAM_R {
        RAM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Arbitration Lost"]
    #[inline(always)]
    pub fn arbl(&self) -> ARBL_R {
        ARBL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Bus Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Addressed As A Slave"]
    #[inline(always)]
    pub fn iaas(&self) -> IAAS_R {
        IAAS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transfer Complete Flag"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Interrupt Flag"]
    #[inline(always)]
    pub fn iicif(&mut self) -> IICIF_W {
        IICIF_W { w: self }
    }
    #[doc = "Bit 3 - Range Address Match"]
    #[inline(always)]
    pub fn ram(&mut self) -> RAM_W {
        RAM_W { w: self }
    }
    #[doc = "Bit 4 - Arbitration Lost"]
    #[inline(always)]
    pub fn arbl(&mut self) -> ARBL_W {
        ARBL_W { w: self }
    }
    #[doc = "Bit 6 - Addressed As A Slave"]
    #[inline(always)]
    pub fn iaas(&mut self) -> IAAS_W {
        IAAS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s](index.html) module"]
pub struct S_SPEC;
impl crate::RegisterSpec for S_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [s::R](R) reader structure"]
impl crate::Readable for S_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s::W](W) writer structure"]
impl crate::Writable for S_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets S to value 0x80"]
impl crate::Resettable for S_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
