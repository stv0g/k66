#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TCR3 {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct WDFLR {
    bits: u8,
}
impl WDFLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TCE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCE0R {
    #[doc = "Transmit data channel N is disabled."]
    _0,
    #[doc = "Transmit data channel N is enabled."]
    _1,
}
impl TCE0R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TCE0R::_0 => false,
            TCE0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCE0R {
        match value {
            false => TCE0R::_0,
            true => TCE0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TCE0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TCE0R::_1
    }
}
#[doc = "Possible values of the field `TCE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCE1R {
    #[doc = "Transmit data channel N is disabled."]
    _0,
    #[doc = "Transmit data channel N is enabled."]
    _1,
}
impl TCE1R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TCE1R::_0 => false,
            TCE1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCE1R {
        match value {
            false => TCE1R::_0,
            true => TCE1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TCE1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TCE1R::_1
    }
}
#[doc = r" Proxy"]
pub struct _WDFLW<'a> {
    w: &'a mut W,
}
impl<'a> _WDFLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TCE0`"]
pub enum TCE0W {
    #[doc = "Transmit data channel N is disabled."]
    _0,
    #[doc = "Transmit data channel N is enabled."]
    _1,
}
impl TCE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCE0W::_0 => false,
            TCE0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCE0W<'a> {
    w: &'a mut W,
}
impl<'a> _TCE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmit data channel N is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCE0W::_0)
    }
    #[doc = "Transmit data channel N is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCE0W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TCE1`"]
pub enum TCE1W {
    #[doc = "Transmit data channel N is disabled."]
    _0,
    #[doc = "Transmit data channel N is enabled."]
    _1,
}
impl TCE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCE1W::_0 => false,
            TCE1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCE1W<'a> {
    w: &'a mut W,
}
impl<'a> _TCE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmit data channel N is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCE1W::_0)
    }
    #[doc = "Transmit data channel N is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCE1W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CFR0`"]
pub enum CFR0W {
    #[doc = "No effect."]
    _0,
    #[doc = "Transmit data channel N FIFO is reset."]
    _1,
}
impl CFR0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CFR0W::_0 => false,
            CFR0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFR0W<'a> {
    w: &'a mut W,
}
impl<'a> _CFR0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFR0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFR0W::_0)
    }
    #[doc = "Transmit data channel N FIFO is reset."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFR0W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CFR1`"]
pub enum CFR1W {
    #[doc = "No effect."]
    _0,
    #[doc = "Transmit data channel N FIFO is reset."]
    _1,
}
impl CFR1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CFR1W::_0 => false,
            CFR1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFR1W<'a> {
    w: &'a mut W,
}
impl<'a> _CFR1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFR1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFR1W::_0)
    }
    #[doc = "Transmit data channel N FIFO is reset."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFR1W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:4 - Word Flag Configuration"]
    #[inline]
    pub fn wdfl(&self) -> WDFLR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WDFLR { bits }
    }
    #[doc = "Bit 16 - Transmit Channel Enable"]
    #[inline]
    pub fn tce0(&self) -> TCE0R {
        TCE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Transmit Channel Enable"]
    #[inline]
    pub fn tce1(&self) -> TCE1R {
        TCE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - Word Flag Configuration"]
    #[inline]
    pub fn wdfl(&mut self) -> _WDFLW {
        _WDFLW { w: self }
    }
    #[doc = "Bit 16 - Transmit Channel Enable"]
    #[inline]
    pub fn tce0(&mut self) -> _TCE0W {
        _TCE0W { w: self }
    }
    #[doc = "Bit 17 - Transmit Channel Enable"]
    #[inline]
    pub fn tce1(&mut self) -> _TCE1W {
        _TCE1W { w: self }
    }
    #[doc = "Bit 24 - Channel FIFO Reset"]
    #[inline]
    pub fn cfr0(&mut self) -> _CFR0W {
        _CFR0W { w: self }
    }
    #[doc = "Bit 25 - Channel FIFO Reset"]
    #[inline]
    pub fn cfr1(&mut self) -> _CFR1W {
        _CFR1W { w: self }
    }
}
