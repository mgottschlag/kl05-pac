#[doc = "Register `S1` reader"]
pub struct R(crate::R<S1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S1` writer"]
pub struct W(crate::W<S1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S1_SPEC>;
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
impl From<crate::W<S1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Parity Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PF_A {
    #[doc = "0: No parity error."]
    _0 = 0,
    #[doc = "1: Parity error."]
    _1 = 1,
}
impl From<PF_A> for bool {
    #[inline(always)]
    fn from(variant: PF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PF` reader - Parity Error Flag"]
pub struct PF_R(crate::FieldReader<bool, PF_A>);
impl PF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PF_A {
        match self.bits {
            false => PF_A::_0,
            true => PF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PF_A::_1
    }
}
impl core::ops::Deref for PF_R {
    type Target = crate::FieldReader<bool, PF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PF` writer - Parity Error Flag"]
pub struct PF_W<'a> {
    w: &'a mut W,
}
impl<'a> PF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No parity error."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PF_A::_0)
    }
    #[doc = "Parity error."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PF_A::_1)
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
#[doc = "Framing Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FE_A {
    #[doc = "0: No framing error detected. This does not guarantee the framing is correct."]
    _0 = 0,
    #[doc = "1: Framing error."]
    _1 = 1,
}
impl From<FE_A> for bool {
    #[inline(always)]
    fn from(variant: FE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE` reader - Framing Error Flag"]
pub struct FE_R(crate::FieldReader<bool, FE_A>);
impl FE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FE_A {
        match self.bits {
            false => FE_A::_0,
            true => FE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FE_A::_1
    }
}
impl core::ops::Deref for FE_R {
    type Target = crate::FieldReader<bool, FE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FE` writer - Framing Error Flag"]
pub struct FE_W<'a> {
    w: &'a mut W,
}
impl<'a> FE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No framing error detected. This does not guarantee the framing is correct."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FE_A::_0)
    }
    #[doc = "Framing error."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FE_A::_1)
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
#[doc = "Noise Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NF_A {
    #[doc = "0: No noise detected."]
    _0 = 0,
    #[doc = "1: Noise detected in the received character in UART _D."]
    _1 = 1,
}
impl From<NF_A> for bool {
    #[inline(always)]
    fn from(variant: NF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NF` reader - Noise Flag"]
pub struct NF_R(crate::FieldReader<bool, NF_A>);
impl NF_R {
    pub(crate) fn new(bits: bool) -> Self {
        NF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NF_A {
        match self.bits {
            false => NF_A::_0,
            true => NF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == NF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == NF_A::_1
    }
}
impl core::ops::Deref for NF_R {
    type Target = crate::FieldReader<bool, NF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NF` writer - Noise Flag"]
pub struct NF_W<'a> {
    w: &'a mut W,
}
impl<'a> NF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No noise detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NF_A::_0)
    }
    #[doc = "Noise detected in the received character in UART _D."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NF_A::_1)
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
#[doc = "Receiver Overrun Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OR_A {
    #[doc = "0: No overrun."]
    _0 = 0,
    #[doc = "1: Receive overrun (new UART data lost)."]
    _1 = 1,
}
impl From<OR_A> for bool {
    #[inline(always)]
    fn from(variant: OR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OR` reader - Receiver Overrun Flag"]
pub struct OR_R(crate::FieldReader<bool, OR_A>);
impl OR_R {
    pub(crate) fn new(bits: bool) -> Self {
        OR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OR_A {
        match self.bits {
            false => OR_A::_0,
            true => OR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OR_A::_1
    }
}
impl core::ops::Deref for OR_R {
    type Target = crate::FieldReader<bool, OR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OR` writer - Receiver Overrun Flag"]
pub struct OR_W<'a> {
    w: &'a mut W,
}
impl<'a> OR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No overrun."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OR_A::_0)
    }
    #[doc = "Receive overrun (new UART data lost)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OR_A::_1)
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
#[doc = "Idle Line Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLE_A {
    #[doc = "0: No idle line detected."]
    _0 = 0,
    #[doc = "1: Idle line was detected."]
    _1 = 1,
}
impl From<IDLE_A> for bool {
    #[inline(always)]
    fn from(variant: IDLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE` reader - Idle Line Flag"]
pub struct IDLE_R(crate::FieldReader<bool, IDLE_A>);
impl IDLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDLE_A {
        match self.bits {
            false => IDLE_A::_0,
            true => IDLE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IDLE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IDLE_A::_1
    }
}
impl core::ops::Deref for IDLE_R {
    type Target = crate::FieldReader<bool, IDLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDLE` writer - Idle Line Flag"]
pub struct IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No idle line detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IDLE_A::_0)
    }
    #[doc = "Idle line was detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IDLE_A::_1)
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
#[doc = "Receive Data Register Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDRF_A {
    #[doc = "0: Receive data buffer empty."]
    _0 = 0,
    #[doc = "1: Receive data buffer full."]
    _1 = 1,
}
impl From<RDRF_A> for bool {
    #[inline(always)]
    fn from(variant: RDRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDRF` reader - Receive Data Register Full Flag"]
pub struct RDRF_R(crate::FieldReader<bool, RDRF_A>);
impl RDRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDRF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDRF_A {
        match self.bits {
            false => RDRF_A::_0,
            true => RDRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RDRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RDRF_A::_1
    }
}
impl core::ops::Deref for RDRF_R {
    type Target = crate::FieldReader<bool, RDRF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmission Complete Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC_A {
    #[doc = "0: Transmitter active (sending data, a preamble, or a break)."]
    _0 = 0,
    #[doc = "1: Transmitter idle (transmission activity complete)."]
    _1 = 1,
}
impl From<TC_A> for bool {
    #[inline(always)]
    fn from(variant: TC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC` reader - Transmission Complete Flag"]
pub struct TC_R(crate::FieldReader<bool, TC_A>);
impl TC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC_A {
        match self.bits {
            false => TC_A::_0,
            true => TC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TC_A::_1
    }
}
impl core::ops::Deref for TC_R {
    type Target = crate::FieldReader<bool, TC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmit Data Register Empty Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDRE_A {
    #[doc = "0: Transmit data buffer full."]
    _0 = 0,
    #[doc = "1: Transmit data buffer empty."]
    _1 = 1,
}
impl From<TDRE_A> for bool {
    #[inline(always)]
    fn from(variant: TDRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDRE` reader - Transmit Data Register Empty Flag"]
pub struct TDRE_R(crate::FieldReader<bool, TDRE_A>);
impl TDRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDRE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDRE_A {
        match self.bits {
            false => TDRE_A::_0,
            true => TDRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TDRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TDRE_A::_1
    }
}
impl core::ops::Deref for TDRE_R {
    type Target = crate::FieldReader<bool, TDRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Parity Error Flag"]
    #[inline(always)]
    pub fn pf(&self) -> PF_R {
        PF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Framing Error Flag"]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Noise Flag"]
    #[inline(always)]
    pub fn nf(&self) -> NF_R {
        NF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receiver Overrun Flag"]
    #[inline(always)]
    pub fn or(&self) -> OR_R {
        OR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Idle Line Flag"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receive Data Register Full Flag"]
    #[inline(always)]
    pub fn rdrf(&self) -> RDRF_R {
        RDRF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmission Complete Flag"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmit Data Register Empty Flag"]
    #[inline(always)]
    pub fn tdre(&self) -> TDRE_R {
        TDRE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Error Flag"]
    #[inline(always)]
    pub fn pf(&mut self) -> PF_W {
        PF_W { w: self }
    }
    #[doc = "Bit 1 - Framing Error Flag"]
    #[inline(always)]
    pub fn fe(&mut self) -> FE_W {
        FE_W { w: self }
    }
    #[doc = "Bit 2 - Noise Flag"]
    #[inline(always)]
    pub fn nf(&mut self) -> NF_W {
        NF_W { w: self }
    }
    #[doc = "Bit 3 - Receiver Overrun Flag"]
    #[inline(always)]
    pub fn or(&mut self) -> OR_W {
        OR_W { w: self }
    }
    #[doc = "Bit 4 - Idle Line Flag"]
    #[inline(always)]
    pub fn idle(&mut self) -> IDLE_W {
        IDLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s1](index.html) module"]
pub struct S1_SPEC;
impl crate::RegisterSpec for S1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [s1::R](R) reader structure"]
impl crate::Readable for S1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s1::W](W) writer structure"]
impl crate::Writable for S1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets S1 to value 0xc0"]
impl crate::Resettable for S1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc0
    }
}
