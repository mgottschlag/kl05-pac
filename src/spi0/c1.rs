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
#[doc = "LSB first (shifter direction)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSBFE_A {
    #[doc = "0: SPI serial data transfers start with most significant bit"]
    _0 = 0,
    #[doc = "1: SPI serial data transfers start with least significant bit"]
    _1 = 1,
}
impl From<LSBFE_A> for bool {
    #[inline(always)]
    fn from(variant: LSBFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSBFE` reader - LSB first (shifter direction)"]
pub struct LSBFE_R(crate::FieldReader<bool, LSBFE_A>);
impl LSBFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSBFE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSBFE_A {
        match self.bits {
            false => LSBFE_A::_0,
            true => LSBFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LSBFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LSBFE_A::_1
    }
}
impl core::ops::Deref for LSBFE_R {
    type Target = crate::FieldReader<bool, LSBFE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSBFE` writer - LSB first (shifter direction)"]
pub struct LSBFE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSBFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSBFE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SPI serial data transfers start with most significant bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LSBFE_A::_0)
    }
    #[doc = "SPI serial data transfers start with least significant bit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LSBFE_A::_1)
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
#[doc = "Slave select output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSOE_A {
    #[doc = "0: When MODFEN is 0: In master mode, SS pin function is general-purpose I/O (not SPI). In slave mode, SS pin function is slave select input. When MODFEN is 1: In master mode, SS pin function is SS input for mode fault. In slave mode, SS pin function is slave select input."]
    _0 = 0,
    #[doc = "1: When MODFEN is 0: In master mode, SS pin function is general-purpose I/O (not SPI). In slave mode, SS pin function is slave select input. When MODFEN is 1: In master mode, SS pin function is automatic SS output. In slave mode: SS pin function is slave select input."]
    _1 = 1,
}
impl From<SSOE_A> for bool {
    #[inline(always)]
    fn from(variant: SSOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSOE` reader - Slave select output enable"]
pub struct SSOE_R(crate::FieldReader<bool, SSOE_A>);
impl SSOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSOE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSOE_A {
        match self.bits {
            false => SSOE_A::_0,
            true => SSOE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SSOE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SSOE_A::_1
    }
}
impl core::ops::Deref for SSOE_R {
    type Target = crate::FieldReader<bool, SSOE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSOE` writer - Slave select output enable"]
pub struct SSOE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSOE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "When MODFEN is 0: In master mode, SS pin function is general-purpose I/O (not SPI). In slave mode, SS pin function is slave select input. When MODFEN is 1: In master mode, SS pin function is SS input for mode fault. In slave mode, SS pin function is slave select input."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSOE_A::_0)
    }
    #[doc = "When MODFEN is 0: In master mode, SS pin function is general-purpose I/O (not SPI). In slave mode, SS pin function is slave select input. When MODFEN is 1: In master mode, SS pin function is automatic SS output. In slave mode: SS pin function is slave select input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSOE_A::_1)
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
#[doc = "Clock phase\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHA_A {
    #[doc = "0: First edge on SPSCK occurs at the middle of the first cycle of a data transfer"]
    _0 = 0,
    #[doc = "1: First edge on SPSCK occurs at the start of the first cycle of a data transfer"]
    _1 = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHA` reader - Clock phase"]
pub struct CPHA_R(crate::FieldReader<bool, CPHA_A>);
impl CPHA_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPHA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::_0,
            true => CPHA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CPHA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CPHA_A::_1
    }
}
impl core::ops::Deref for CPHA_R {
    type Target = crate::FieldReader<bool, CPHA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPHA` writer - Clock phase"]
pub struct CPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> CPHA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPHA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "First edge on SPSCK occurs at the middle of the first cycle of a data transfer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPHA_A::_0)
    }
    #[doc = "First edge on SPSCK occurs at the start of the first cycle of a data transfer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPHA_A::_1)
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
#[doc = "Clock polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOL_A {
    #[doc = "0: Active-high SPI clock (idles low)"]
    _0 = 0,
    #[doc = "1: Active-low SPI clock (idles high)"]
    _1 = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOL` reader - Clock polarity"]
pub struct CPOL_R(crate::FieldReader<bool, CPOL_A>);
impl CPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::_0,
            true => CPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CPOL_A::_1
    }
}
impl core::ops::Deref for CPOL_R {
    type Target = crate::FieldReader<bool, CPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPOL` writer - Clock polarity"]
pub struct CPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Active-high SPI clock (idles low)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPOL_A::_0)
    }
    #[doc = "Active-low SPI clock (idles high)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPOL_A::_1)
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
#[doc = "Master/slave mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTR_A {
    #[doc = "0: SPI module configured as a slave SPI device"]
    _0 = 0,
    #[doc = "1: SPI module configured as a master SPI device"]
    _1 = 1,
}
impl From<MSTR_A> for bool {
    #[inline(always)]
    fn from(variant: MSTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTR` reader - Master/slave mode select"]
pub struct MSTR_R(crate::FieldReader<bool, MSTR_A>);
impl MSTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSTR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTR_A {
        match self.bits {
            false => MSTR_A::_0,
            true => MSTR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MSTR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MSTR_A::_1
    }
}
impl core::ops::Deref for MSTR_R {
    type Target = crate::FieldReader<bool, MSTR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTR` writer - Master/slave mode select"]
pub struct MSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SPI module configured as a slave SPI device"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTR_A::_0)
    }
    #[doc = "SPI module configured as a master SPI device"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTR_A::_1)
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
#[doc = "SPI transmit interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPTIE_A {
    #[doc = "0: Interrupts from SPTEF inhibited (use polling)"]
    _0 = 0,
    #[doc = "1: When SPTEF is 1, hardware interrupt requested"]
    _1 = 1,
}
impl From<SPTIE_A> for bool {
    #[inline(always)]
    fn from(variant: SPTIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPTIE` reader - SPI transmit interrupt enable"]
pub struct SPTIE_R(crate::FieldReader<bool, SPTIE_A>);
impl SPTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPTIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPTIE_A {
        match self.bits {
            false => SPTIE_A::_0,
            true => SPTIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPTIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPTIE_A::_1
    }
}
impl core::ops::Deref for SPTIE_R {
    type Target = crate::FieldReader<bool, SPTIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPTIE` writer - SPI transmit interrupt enable"]
pub struct SPTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPTIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPTIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupts from SPTEF inhibited (use polling)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPTIE_A::_0)
    }
    #[doc = "When SPTEF is 1, hardware interrupt requested"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPTIE_A::_1)
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
#[doc = "SPI system enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPE_A {
    #[doc = "0: SPI system inactive"]
    _0 = 0,
    #[doc = "1: SPI system enabled"]
    _1 = 1,
}
impl From<SPE_A> for bool {
    #[inline(always)]
    fn from(variant: SPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPE` reader - SPI system enable"]
pub struct SPE_R(crate::FieldReader<bool, SPE_A>);
impl SPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPE_A {
        match self.bits {
            false => SPE_A::_0,
            true => SPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPE_A::_1
    }
}
impl core::ops::Deref for SPE_R {
    type Target = crate::FieldReader<bool, SPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPE` writer - SPI system enable"]
pub struct SPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SPI system inactive"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPE_A::_0)
    }
    #[doc = "SPI system enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPE_A::_1)
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
#[doc = "SPI interrupt enable: for SPRF and MODF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIE_A {
    #[doc = "0: Interrupts from SPRF and MODF are inhibited-use polling"]
    _0 = 0,
    #[doc = "1: Request a hardware interrupt when SPRF or MODF is 1"]
    _1 = 1,
}
impl From<SPIE_A> for bool {
    #[inline(always)]
    fn from(variant: SPIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIE` reader - SPI interrupt enable: for SPRF and MODF"]
pub struct SPIE_R(crate::FieldReader<bool, SPIE_A>);
impl SPIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIE_A {
        match self.bits {
            false => SPIE_A::_0,
            true => SPIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPIE_A::_1
    }
}
impl core::ops::Deref for SPIE_R {
    type Target = crate::FieldReader<bool, SPIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPIE` writer - SPI interrupt enable: for SPRF and MODF"]
pub struct SPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupts from SPRF and MODF are inhibited-use polling"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPIE_A::_0)
    }
    #[doc = "Request a hardware interrupt when SPRF or MODF is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPIE_A::_1)
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
    #[doc = "Bit 0 - LSB first (shifter direction)"]
    #[inline(always)]
    pub fn lsbfe(&self) -> LSBFE_R {
        LSBFE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Slave select output enable"]
    #[inline(always)]
    pub fn ssoe(&self) -> SSOE_R {
        SSOE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Clock phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Clock polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Master/slave mode select"]
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SPI transmit interrupt enable"]
    #[inline(always)]
    pub fn sptie(&self) -> SPTIE_R {
        SPTIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SPI system enable"]
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SPI interrupt enable: for SPRF and MODF"]
    #[inline(always)]
    pub fn spie(&self) -> SPIE_R {
        SPIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSB first (shifter direction)"]
    #[inline(always)]
    pub fn lsbfe(&mut self) -> LSBFE_W {
        LSBFE_W { w: self }
    }
    #[doc = "Bit 1 - Slave select output enable"]
    #[inline(always)]
    pub fn ssoe(&mut self) -> SSOE_W {
        SSOE_W { w: self }
    }
    #[doc = "Bit 2 - Clock phase"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W {
        CPHA_W { w: self }
    }
    #[doc = "Bit 3 - Clock polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W { w: self }
    }
    #[doc = "Bit 4 - Master/slave mode select"]
    #[inline(always)]
    pub fn mstr(&mut self) -> MSTR_W {
        MSTR_W { w: self }
    }
    #[doc = "Bit 5 - SPI transmit interrupt enable"]
    #[inline(always)]
    pub fn sptie(&mut self) -> SPTIE_W {
        SPTIE_W { w: self }
    }
    #[doc = "Bit 6 - SPI system enable"]
    #[inline(always)]
    pub fn spe(&mut self) -> SPE_W {
        SPE_W { w: self }
    }
    #[doc = "Bit 7 - SPI interrupt enable: for SPRF and MODF"]
    #[inline(always)]
    pub fn spie(&mut self) -> SPIE_W {
        SPIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1](index.html) module"]
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
#[doc = "`reset()` method sets C1 to value 0x04"]
impl crate::Resettable for C1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
