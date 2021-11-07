#[doc = "Register `PLACR` reader"]
pub struct R(crate::R<PLACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLACR` writer"]
pub struct W(crate::W<PLACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLACR_SPEC>;
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
impl From<crate::W<PLACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Arbitration select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARB_A {
    #[doc = "0: Fixed-priority arbitration for the crossbar masters"]
    _0 = 0,
    #[doc = "1: Round-robin arbitration for the crossbar masters"]
    _1 = 1,
}
impl From<ARB_A> for bool {
    #[inline(always)]
    fn from(variant: ARB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARB` reader - Arbitration select"]
pub struct ARB_R(crate::FieldReader<bool, ARB_A>);
impl ARB_R {
    pub(crate) fn new(bits: bool) -> Self {
        ARB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARB_A {
        match self.bits {
            false => ARB_A::_0,
            true => ARB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ARB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ARB_A::_1
    }
}
impl core::ops::Deref for ARB_R {
    type Target = crate::FieldReader<bool, ARB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARB` writer - Arbitration select"]
pub struct ARB_W<'a> {
    w: &'a mut W,
}
impl<'a> ARB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fixed-priority arbitration for the crossbar masters"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ARB_A::_0)
    }
    #[doc = "Round-robin arbitration for the crossbar masters"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ARB_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `CFCC` writer - Clear Flash Controller Cache"]
pub struct CFCC_W<'a> {
    w: &'a mut W,
}
impl<'a> CFCC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Disable Flash Controller Data Caching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFCDA_A {
    #[doc = "0: Enable flash controller data caching"]
    _0 = 0,
    #[doc = "1: Disable flash controller data caching."]
    _1 = 1,
}
impl From<DFCDA_A> for bool {
    #[inline(always)]
    fn from(variant: DFCDA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFCDA` reader - Disable Flash Controller Data Caching"]
pub struct DFCDA_R(crate::FieldReader<bool, DFCDA_A>);
impl DFCDA_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFCDA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFCDA_A {
        match self.bits {
            false => DFCDA_A::_0,
            true => DFCDA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DFCDA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DFCDA_A::_1
    }
}
impl core::ops::Deref for DFCDA_R {
    type Target = crate::FieldReader<bool, DFCDA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFCDA` writer - Disable Flash Controller Data Caching"]
pub struct DFCDA_W<'a> {
    w: &'a mut W,
}
impl<'a> DFCDA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFCDA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable flash controller data caching"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFCDA_A::_0)
    }
    #[doc = "Disable flash controller data caching."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFCDA_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Disable Flash Controller Instruction Caching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFCIC_A {
    #[doc = "0: Enable flash controller instruction caching."]
    _0 = 0,
    #[doc = "1: Disable flash controller instruction caching."]
    _1 = 1,
}
impl From<DFCIC_A> for bool {
    #[inline(always)]
    fn from(variant: DFCIC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFCIC` reader - Disable Flash Controller Instruction Caching"]
pub struct DFCIC_R(crate::FieldReader<bool, DFCIC_A>);
impl DFCIC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFCIC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFCIC_A {
        match self.bits {
            false => DFCIC_A::_0,
            true => DFCIC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DFCIC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DFCIC_A::_1
    }
}
impl core::ops::Deref for DFCIC_R {
    type Target = crate::FieldReader<bool, DFCIC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFCIC` writer - Disable Flash Controller Instruction Caching"]
pub struct DFCIC_W<'a> {
    w: &'a mut W,
}
impl<'a> DFCIC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFCIC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable flash controller instruction caching."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFCIC_A::_0)
    }
    #[doc = "Disable flash controller instruction caching."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFCIC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Disable Flash Controller Cache\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFCC_A {
    #[doc = "0: Enable flash controller cache."]
    _0 = 0,
    #[doc = "1: Disable flash controller cache."]
    _1 = 1,
}
impl From<DFCC_A> for bool {
    #[inline(always)]
    fn from(variant: DFCC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFCC` reader - Disable Flash Controller Cache"]
pub struct DFCC_R(crate::FieldReader<bool, DFCC_A>);
impl DFCC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFCC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFCC_A {
        match self.bits {
            false => DFCC_A::_0,
            true => DFCC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DFCC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DFCC_A::_1
    }
}
impl core::ops::Deref for DFCC_R {
    type Target = crate::FieldReader<bool, DFCC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFCC` writer - Disable Flash Controller Cache"]
pub struct DFCC_W<'a> {
    w: &'a mut W,
}
impl<'a> DFCC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFCC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable flash controller cache."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFCC_A::_0)
    }
    #[doc = "Disable flash controller cache."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFCC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Enable Flash Data Speculation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EFDS_A {
    #[doc = "0: Disable flash data speculation."]
    _0 = 0,
    #[doc = "1: Enable flash data speculation."]
    _1 = 1,
}
impl From<EFDS_A> for bool {
    #[inline(always)]
    fn from(variant: EFDS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EFDS` reader - Enable Flash Data Speculation"]
pub struct EFDS_R(crate::FieldReader<bool, EFDS_A>);
impl EFDS_R {
    pub(crate) fn new(bits: bool) -> Self {
        EFDS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EFDS_A {
        match self.bits {
            false => EFDS_A::_0,
            true => EFDS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EFDS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EFDS_A::_1
    }
}
impl core::ops::Deref for EFDS_R {
    type Target = crate::FieldReader<bool, EFDS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFDS` writer - Enable Flash Data Speculation"]
pub struct EFDS_W<'a> {
    w: &'a mut W,
}
impl<'a> EFDS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EFDS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable flash data speculation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EFDS_A::_0)
    }
    #[doc = "Enable flash data speculation."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EFDS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Disable Flash Controller Speculation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFCS_A {
    #[doc = "0: Enable flash controller speculation."]
    _0 = 0,
    #[doc = "1: Disable flash controller speculation."]
    _1 = 1,
}
impl From<DFCS_A> for bool {
    #[inline(always)]
    fn from(variant: DFCS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFCS` reader - Disable Flash Controller Speculation"]
pub struct DFCS_R(crate::FieldReader<bool, DFCS_A>);
impl DFCS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFCS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFCS_A {
        match self.bits {
            false => DFCS_A::_0,
            true => DFCS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DFCS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DFCS_A::_1
    }
}
impl core::ops::Deref for DFCS_R {
    type Target = crate::FieldReader<bool, DFCS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFCS` writer - Disable Flash Controller Speculation"]
pub struct DFCS_W<'a> {
    w: &'a mut W,
}
impl<'a> DFCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFCS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable flash controller speculation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFCS_A::_0)
    }
    #[doc = "Disable flash controller speculation."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFCS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Enable Stalling Flash Controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESFC_A {
    #[doc = "0: Disable stalling flash controller when flash is busy."]
    _0 = 0,
    #[doc = "1: Enable stalling flash controller when flash is busy."]
    _1 = 1,
}
impl From<ESFC_A> for bool {
    #[inline(always)]
    fn from(variant: ESFC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESFC` reader - Enable Stalling Flash Controller"]
pub struct ESFC_R(crate::FieldReader<bool, ESFC_A>);
impl ESFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ESFC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ESFC_A {
        match self.bits {
            false => ESFC_A::_0,
            true => ESFC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ESFC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ESFC_A::_1
    }
}
impl core::ops::Deref for ESFC_R {
    type Target = crate::FieldReader<bool, ESFC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESFC` writer - Enable Stalling Flash Controller"]
pub struct ESFC_W<'a> {
    w: &'a mut W,
}
impl<'a> ESFC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ESFC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable stalling flash controller when flash is busy."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ESFC_A::_0)
    }
    #[doc = "Enable stalling flash controller when flash is busy."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ESFC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 9 - Arbitration select"]
    #[inline(always)]
    pub fn arb(&self) -> ARB_R {
        ARB_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Disable Flash Controller Data Caching"]
    #[inline(always)]
    pub fn dfcda(&self) -> DFCDA_R {
        DFCDA_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Disable Flash Controller Instruction Caching"]
    #[inline(always)]
    pub fn dfcic(&self) -> DFCIC_R {
        DFCIC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Disable Flash Controller Cache"]
    #[inline(always)]
    pub fn dfcc(&self) -> DFCC_R {
        DFCC_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable Flash Data Speculation"]
    #[inline(always)]
    pub fn efds(&self) -> EFDS_R {
        EFDS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Disable Flash Controller Speculation"]
    #[inline(always)]
    pub fn dfcs(&self) -> DFCS_R {
        DFCS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable Stalling Flash Controller"]
    #[inline(always)]
    pub fn esfc(&self) -> ESFC_R {
        ESFC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Arbitration select"]
    #[inline(always)]
    pub fn arb(&mut self) -> ARB_W {
        ARB_W { w: self }
    }
    #[doc = "Bit 10 - Clear Flash Controller Cache"]
    #[inline(always)]
    pub fn cfcc(&mut self) -> CFCC_W {
        CFCC_W { w: self }
    }
    #[doc = "Bit 11 - Disable Flash Controller Data Caching"]
    #[inline(always)]
    pub fn dfcda(&mut self) -> DFCDA_W {
        DFCDA_W { w: self }
    }
    #[doc = "Bit 12 - Disable Flash Controller Instruction Caching"]
    #[inline(always)]
    pub fn dfcic(&mut self) -> DFCIC_W {
        DFCIC_W { w: self }
    }
    #[doc = "Bit 13 - Disable Flash Controller Cache"]
    #[inline(always)]
    pub fn dfcc(&mut self) -> DFCC_W {
        DFCC_W { w: self }
    }
    #[doc = "Bit 14 - Enable Flash Data Speculation"]
    #[inline(always)]
    pub fn efds(&mut self) -> EFDS_W {
        EFDS_W { w: self }
    }
    #[doc = "Bit 15 - Disable Flash Controller Speculation"]
    #[inline(always)]
    pub fn dfcs(&mut self) -> DFCS_W {
        DFCS_W { w: self }
    }
    #[doc = "Bit 16 - Enable Stalling Flash Controller"]
    #[inline(always)]
    pub fn esfc(&mut self) -> ESFC_W {
        ESFC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Platform Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [placr](index.html) module"]
pub struct PLACR_SPEC;
impl crate::RegisterSpec for PLACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [placr::R](R) reader structure"]
impl crate::Readable for PLACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [placr::W](W) writer structure"]
impl crate::Writable for PLACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLACR to value 0"]
impl crate::Resettable for PLACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
