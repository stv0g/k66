#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IMASK1 {
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
#[doc = "Possible values of the field `BUFLM0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM0R {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM0R {
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
            BUFLM0R::_0 => false,
            BUFLM0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFLM0R {
        match value {
            false => BUFLM0R::_0,
            true => BUFLM0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUFLM0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUFLM0R::_1
    }
}
#[doc = "Possible values of the field `BUFLM1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM1R {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM1R {
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
            BUFLM1R::_0 => false,
            BUFLM1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFLM1R {
        match value {
            false => BUFLM1R::_0,
            true => BUFLM1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUFLM1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUFLM1R::_1
    }
}
#[doc = "Possible values of the field `BUFLM2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM2R {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM2R {
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
            BUFLM2R::_0 => false,
            BUFLM2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFLM2R {
        match value {
            false => BUFLM2R::_0,
            true => BUFLM2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUFLM2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUFLM2R::_1
    }
}
#[doc = "Possible values of the field `BUFLM3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM3R {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM3R {
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
            BUFLM3R::_0 => false,
            BUFLM3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFLM3R {
        match value {
            false => BUFLM3R::_0,
            true => BUFLM3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUFLM3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUFLM3R::_1
    }
}
#[doc = "Possible values of the field `BUFLM4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM4R {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM4R {
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
            BUFLM4R::_0 => false,
            BUFLM4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFLM4R {
        match value {
            false => BUFLM4R::_0,
            true => BUFLM4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUFLM4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUFLM4R::_1
    }
}
#[doc = "Possible values of the field `BUFLM5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM5R {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM5R {
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
            BUFLM5R::_0 => false,
            BUFLM5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFLM5R {
        match value {
            false => BUFLM5R::_0,
            true => BUFLM5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUFLM5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUFLM5R::_1
    }
}
#[doc = "Possible values of the field `BUFLM6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM6R {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM6R {
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
            BUFLM6R::_0 => false,
            BUFLM6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFLM6R {
        match value {
            false => BUFLM6R::_0,
            true => BUFLM6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUFLM6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUFLM6R::_1
    }
}
#[doc = "Possible values of the field `BUFLM7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM7R {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM7R {
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
            BUFLM7R::_0 => false,
            BUFLM7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFLM7R {
        match value {
            false => BUFLM7R::_0,
            true => BUFLM7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUFLM7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUFLM7R::_1
    }
}
#[doc = "Possible values of the field `BUFLM8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM8R {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM8R {
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
            BUFLM8R::_0 => false,
            BUFLM8R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFLM8R {
        match value {
            false => BUFLM8R::_0,
            true => BUFLM8R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUFLM8R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUFLM8R::_1
    }
}
#[doc = "Possible values of the field `BUFLM9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM9R {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM9R {
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
            BUFLM9R::_0 => false,
            BUFLM9R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFLM9R {
        match value {
            false => BUFLM9R::_0,
            true => BUFLM9R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUFLM9R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUFLM9R::_1
    }
}
#[doc = "Possible values of the field `BUFLM10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM10R {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM10R {
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
            BUFLM10R::_0 => false,
            BUFLM10R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFLM10R {
        match value {
            false => BUFLM10R::_0,
            true => BUFLM10R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUFLM10R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUFLM10R::_1
    }
}
#[doc = "Possible values of the field `BUFLM11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM11R {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM11R {
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
            BUFLM11R::_0 => false,
            BUFLM11R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFLM11R {
        match value {
            false => BUFLM11R::_0,
            true => BUFLM11R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUFLM11R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUFLM11R::_1
    }
}
#[doc = "Possible values of the field `BUFLM12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM12R {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM12R {
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
            BUFLM12R::_0 => false,
            BUFLM12R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFLM12R {
        match value {
            false => BUFLM12R::_0,
            true => BUFLM12R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUFLM12R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUFLM12R::_1
    }
}
#[doc = "Possible values of the field `BUFLM13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM13R {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM13R {
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
            BUFLM13R::_0 => false,
            BUFLM13R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFLM13R {
        match value {
            false => BUFLM13R::_0,
            true => BUFLM13R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUFLM13R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUFLM13R::_1
    }
}
#[doc = "Possible values of the field `BUFLM14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM14R {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM14R {
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
            BUFLM14R::_0 => false,
            BUFLM14R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFLM14R {
        match value {
            false => BUFLM14R::_0,
            true => BUFLM14R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUFLM14R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUFLM14R::_1
    }
}
#[doc = "Possible values of the field `BUFLM15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM15R {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM15R {
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
            BUFLM15R::_0 => false,
            BUFLM15R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFLM15R {
        match value {
            false => BUFLM15R::_0,
            true => BUFLM15R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUFLM15R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUFLM15R::_1
    }
}
#[doc = "Possible values of the field `BUFLM16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM16R {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM16R {
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
            BUFLM16R::_0 => false,
            BUFLM16R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFLM16R {
        match value {
            false => BUFLM16R::_0,
            true => BUFLM16R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUFLM16R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUFLM16R::_1
    }
}
#[doc = "Possible values of the field `BUFLM17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM17R {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM17R {
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
            BUFLM17R::_0 => false,
            BUFLM17R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFLM17R {
        match value {
            false => BUFLM17R::_0,
            true => BUFLM17R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUFLM17R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUFLM17R::_1
    }
}
#[doc = "Possible values of the field `BUFLM18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM18R {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM18R {
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
            BUFLM18R::_0 => false,
            BUFLM18R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFLM18R {
        match value {
            false => BUFLM18R::_0,
            true => BUFLM18R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUFLM18R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUFLM18R::_1
    }
}
#[doc = "Possible values of the field `BUFLM19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM19R {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM19R {
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
            BUFLM19R::_0 => false,
            BUFLM19R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFLM19R {
        match value {
            false => BUFLM19R::_0,
            true => BUFLM19R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUFLM19R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUFLM19R::_1
    }
}
#[doc = "Possible values of the field `BUFLM20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM20R {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM20R {
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
            BUFLM20R::_0 => false,
            BUFLM20R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFLM20R {
        match value {
            false => BUFLM20R::_0,
            true => BUFLM20R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUFLM20R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUFLM20R::_1
    }
}
#[doc = "Possible values of the field `BUFLM21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM21R {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM21R {
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
            BUFLM21R::_0 => false,
            BUFLM21R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFLM21R {
        match value {
            false => BUFLM21R::_0,
            true => BUFLM21R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUFLM21R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUFLM21R::_1
    }
}
#[doc = "Possible values of the field `BUFLM22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM22R {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM22R {
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
            BUFLM22R::_0 => false,
            BUFLM22R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFLM22R {
        match value {
            false => BUFLM22R::_0,
            true => BUFLM22R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUFLM22R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUFLM22R::_1
    }
}
#[doc = "Possible values of the field `BUFLM23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM23R {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM23R {
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
            BUFLM23R::_0 => false,
            BUFLM23R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFLM23R {
        match value {
            false => BUFLM23R::_0,
            true => BUFLM23R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUFLM23R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUFLM23R::_1
    }
}
#[doc = "Possible values of the field `BUFLM24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM24R {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM24R {
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
            BUFLM24R::_0 => false,
            BUFLM24R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFLM24R {
        match value {
            false => BUFLM24R::_0,
            true => BUFLM24R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUFLM24R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUFLM24R::_1
    }
}
#[doc = "Possible values of the field `BUFLM25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM25R {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM25R {
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
            BUFLM25R::_0 => false,
            BUFLM25R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFLM25R {
        match value {
            false => BUFLM25R::_0,
            true => BUFLM25R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUFLM25R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUFLM25R::_1
    }
}
#[doc = "Possible values of the field `BUFLM26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM26R {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM26R {
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
            BUFLM26R::_0 => false,
            BUFLM26R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFLM26R {
        match value {
            false => BUFLM26R::_0,
            true => BUFLM26R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUFLM26R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUFLM26R::_1
    }
}
#[doc = "Possible values of the field `BUFLM27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM27R {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM27R {
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
            BUFLM27R::_0 => false,
            BUFLM27R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFLM27R {
        match value {
            false => BUFLM27R::_0,
            true => BUFLM27R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUFLM27R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUFLM27R::_1
    }
}
#[doc = "Possible values of the field `BUFLM28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM28R {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM28R {
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
            BUFLM28R::_0 => false,
            BUFLM28R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFLM28R {
        match value {
            false => BUFLM28R::_0,
            true => BUFLM28R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUFLM28R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUFLM28R::_1
    }
}
#[doc = "Possible values of the field `BUFLM29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM29R {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM29R {
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
            BUFLM29R::_0 => false,
            BUFLM29R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFLM29R {
        match value {
            false => BUFLM29R::_0,
            true => BUFLM29R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUFLM29R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUFLM29R::_1
    }
}
#[doc = "Possible values of the field `BUFLM30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM30R {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM30R {
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
            BUFLM30R::_0 => false,
            BUFLM30R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFLM30R {
        match value {
            false => BUFLM30R::_0,
            true => BUFLM30R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUFLM30R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUFLM30R::_1
    }
}
#[doc = "Possible values of the field `BUFLM31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFLM31R {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM31R {
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
            BUFLM31R::_0 => false,
            BUFLM31R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFLM31R {
        match value {
            false => BUFLM31R::_0,
            true => BUFLM31R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUFLM31R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUFLM31R::_1
    }
}
#[doc = "Values that can be written to the field `BUFLM0`"]
pub enum BUFLM0W {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFLM0W::_0 => false,
            BUFLM0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFLM0W<'a> {
    w: &'a mut W,
}
impl<'a> _BUFLM0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFLM0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM0W::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM0W::_1)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUFLM1`"]
pub enum BUFLM1W {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFLM1W::_0 => false,
            BUFLM1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFLM1W<'a> {
    w: &'a mut W,
}
impl<'a> _BUFLM1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFLM1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM1W::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM1W::_1)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUFLM2`"]
pub enum BUFLM2W {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFLM2W::_0 => false,
            BUFLM2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFLM2W<'a> {
    w: &'a mut W,
}
impl<'a> _BUFLM2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFLM2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM2W::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM2W::_1)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUFLM3`"]
pub enum BUFLM3W {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFLM3W::_0 => false,
            BUFLM3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFLM3W<'a> {
    w: &'a mut W,
}
impl<'a> _BUFLM3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFLM3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM3W::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM3W::_1)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUFLM4`"]
pub enum BUFLM4W {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFLM4W::_0 => false,
            BUFLM4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFLM4W<'a> {
    w: &'a mut W,
}
impl<'a> _BUFLM4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFLM4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM4W::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM4W::_1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUFLM5`"]
pub enum BUFLM5W {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFLM5W::_0 => false,
            BUFLM5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFLM5W<'a> {
    w: &'a mut W,
}
impl<'a> _BUFLM5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFLM5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM5W::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM5W::_1)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUFLM6`"]
pub enum BUFLM6W {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFLM6W::_0 => false,
            BUFLM6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFLM6W<'a> {
    w: &'a mut W,
}
impl<'a> _BUFLM6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFLM6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM6W::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM6W::_1)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUFLM7`"]
pub enum BUFLM7W {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFLM7W::_0 => false,
            BUFLM7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFLM7W<'a> {
    w: &'a mut W,
}
impl<'a> _BUFLM7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFLM7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM7W::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM7W::_1)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUFLM8`"]
pub enum BUFLM8W {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFLM8W::_0 => false,
            BUFLM8W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFLM8W<'a> {
    w: &'a mut W,
}
impl<'a> _BUFLM8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFLM8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM8W::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM8W::_1)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUFLM9`"]
pub enum BUFLM9W {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFLM9W::_0 => false,
            BUFLM9W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFLM9W<'a> {
    w: &'a mut W,
}
impl<'a> _BUFLM9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFLM9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM9W::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM9W::_1)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUFLM10`"]
pub enum BUFLM10W {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFLM10W::_0 => false,
            BUFLM10W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFLM10W<'a> {
    w: &'a mut W,
}
impl<'a> _BUFLM10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFLM10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM10W::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM10W::_1)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUFLM11`"]
pub enum BUFLM11W {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFLM11W::_0 => false,
            BUFLM11W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFLM11W<'a> {
    w: &'a mut W,
}
impl<'a> _BUFLM11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFLM11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM11W::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM11W::_1)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUFLM12`"]
pub enum BUFLM12W {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFLM12W::_0 => false,
            BUFLM12W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFLM12W<'a> {
    w: &'a mut W,
}
impl<'a> _BUFLM12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFLM12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM12W::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM12W::_1)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUFLM13`"]
pub enum BUFLM13W {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFLM13W::_0 => false,
            BUFLM13W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFLM13W<'a> {
    w: &'a mut W,
}
impl<'a> _BUFLM13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFLM13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM13W::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM13W::_1)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUFLM14`"]
pub enum BUFLM14W {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFLM14W::_0 => false,
            BUFLM14W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFLM14W<'a> {
    w: &'a mut W,
}
impl<'a> _BUFLM14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFLM14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM14W::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM14W::_1)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUFLM15`"]
pub enum BUFLM15W {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFLM15W::_0 => false,
            BUFLM15W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFLM15W<'a> {
    w: &'a mut W,
}
impl<'a> _BUFLM15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFLM15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM15W::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM15W::_1)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUFLM16`"]
pub enum BUFLM16W {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFLM16W::_0 => false,
            BUFLM16W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFLM16W<'a> {
    w: &'a mut W,
}
impl<'a> _BUFLM16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFLM16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM16W::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM16W::_1)
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
#[doc = "Values that can be written to the field `BUFLM17`"]
pub enum BUFLM17W {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFLM17W::_0 => false,
            BUFLM17W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFLM17W<'a> {
    w: &'a mut W,
}
impl<'a> _BUFLM17W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFLM17W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM17W::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM17W::_1)
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
#[doc = "Values that can be written to the field `BUFLM18`"]
pub enum BUFLM18W {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFLM18W::_0 => false,
            BUFLM18W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFLM18W<'a> {
    w: &'a mut W,
}
impl<'a> _BUFLM18W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFLM18W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM18W::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM18W::_1)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUFLM19`"]
pub enum BUFLM19W {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFLM19W::_0 => false,
            BUFLM19W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFLM19W<'a> {
    w: &'a mut W,
}
impl<'a> _BUFLM19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFLM19W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM19W::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM19W::_1)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUFLM20`"]
pub enum BUFLM20W {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFLM20W::_0 => false,
            BUFLM20W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFLM20W<'a> {
    w: &'a mut W,
}
impl<'a> _BUFLM20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFLM20W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM20W::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM20W::_1)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUFLM21`"]
pub enum BUFLM21W {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFLM21W::_0 => false,
            BUFLM21W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFLM21W<'a> {
    w: &'a mut W,
}
impl<'a> _BUFLM21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFLM21W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM21W::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM21W::_1)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUFLM22`"]
pub enum BUFLM22W {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFLM22W::_0 => false,
            BUFLM22W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFLM22W<'a> {
    w: &'a mut W,
}
impl<'a> _BUFLM22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFLM22W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM22W::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM22W::_1)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUFLM23`"]
pub enum BUFLM23W {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFLM23W::_0 => false,
            BUFLM23W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFLM23W<'a> {
    w: &'a mut W,
}
impl<'a> _BUFLM23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFLM23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM23W::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM23W::_1)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUFLM24`"]
pub enum BUFLM24W {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFLM24W::_0 => false,
            BUFLM24W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFLM24W<'a> {
    w: &'a mut W,
}
impl<'a> _BUFLM24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFLM24W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM24W::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM24W::_1)
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
#[doc = "Values that can be written to the field `BUFLM25`"]
pub enum BUFLM25W {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFLM25W::_0 => false,
            BUFLM25W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFLM25W<'a> {
    w: &'a mut W,
}
impl<'a> _BUFLM25W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFLM25W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM25W::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM25W::_1)
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
#[doc = "Values that can be written to the field `BUFLM26`"]
pub enum BUFLM26W {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFLM26W::_0 => false,
            BUFLM26W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFLM26W<'a> {
    w: &'a mut W,
}
impl<'a> _BUFLM26W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFLM26W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM26W::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM26W::_1)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUFLM27`"]
pub enum BUFLM27W {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFLM27W::_0 => false,
            BUFLM27W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFLM27W<'a> {
    w: &'a mut W,
}
impl<'a> _BUFLM27W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFLM27W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM27W::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM27W::_1)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUFLM28`"]
pub enum BUFLM28W {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFLM28W::_0 => false,
            BUFLM28W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFLM28W<'a> {
    w: &'a mut W,
}
impl<'a> _BUFLM28W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFLM28W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM28W::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM28W::_1)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUFLM29`"]
pub enum BUFLM29W {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFLM29W::_0 => false,
            BUFLM29W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFLM29W<'a> {
    w: &'a mut W,
}
impl<'a> _BUFLM29W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFLM29W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM29W::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM29W::_1)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUFLM30`"]
pub enum BUFLM30W {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFLM30W::_0 => false,
            BUFLM30W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFLM30W<'a> {
    w: &'a mut W,
}
impl<'a> _BUFLM30W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFLM30W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM30W::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM30W::_1)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUFLM31`"]
pub enum BUFLM31W {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    _0,
    #[doc = "The corresponding buffer Interrupt is enabled."]
    _1,
}
impl BUFLM31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFLM31W::_0 => false,
            BUFLM31W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFLM31W<'a> {
    w: &'a mut W,
}
impl<'a> _BUFLM31W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFLM31W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM31W::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM31W::_1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm0(&self) -> BUFLM0R {
        BUFLM0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm1(&self) -> BUFLM1R {
        BUFLM1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm2(&self) -> BUFLM2R {
        BUFLM2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm3(&self) -> BUFLM3R {
        BUFLM3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm4(&self) -> BUFLM4R {
        BUFLM4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm5(&self) -> BUFLM5R {
        BUFLM5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm6(&self) -> BUFLM6R {
        BUFLM6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm7(&self) -> BUFLM7R {
        BUFLM7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm8(&self) -> BUFLM8R {
        BUFLM8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm9(&self) -> BUFLM9R {
        BUFLM9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm10(&self) -> BUFLM10R {
        BUFLM10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm11(&self) -> BUFLM11R {
        BUFLM11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm12(&self) -> BUFLM12R {
        BUFLM12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm13(&self) -> BUFLM13R {
        BUFLM13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm14(&self) -> BUFLM14R {
        BUFLM14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm15(&self) -> BUFLM15R {
        BUFLM15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm16(&self) -> BUFLM16R {
        BUFLM16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm17(&self) -> BUFLM17R {
        BUFLM17R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm18(&self) -> BUFLM18R {
        BUFLM18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm19(&self) -> BUFLM19R {
        BUFLM19R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm20(&self) -> BUFLM20R {
        BUFLM20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm21(&self) -> BUFLM21R {
        BUFLM21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm22(&self) -> BUFLM22R {
        BUFLM22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm23(&self) -> BUFLM23R {
        BUFLM23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm24(&self) -> BUFLM24R {
        BUFLM24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm25(&self) -> BUFLM25R {
        BUFLM25R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm26(&self) -> BUFLM26R {
        BUFLM26R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm27(&self) -> BUFLM27R {
        BUFLM27R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm28(&self) -> BUFLM28R {
        BUFLM28R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm29(&self) -> BUFLM29R {
        BUFLM29R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm30(&self) -> BUFLM30R {
        BUFLM30R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm31(&self) -> BUFLM31R {
        BUFLM31R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm0(&mut self) -> _BUFLM0W {
        _BUFLM0W { w: self }
    }
    #[doc = "Bit 1 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm1(&mut self) -> _BUFLM1W {
        _BUFLM1W { w: self }
    }
    #[doc = "Bit 2 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm2(&mut self) -> _BUFLM2W {
        _BUFLM2W { w: self }
    }
    #[doc = "Bit 3 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm3(&mut self) -> _BUFLM3W {
        _BUFLM3W { w: self }
    }
    #[doc = "Bit 4 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm4(&mut self) -> _BUFLM4W {
        _BUFLM4W { w: self }
    }
    #[doc = "Bit 5 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm5(&mut self) -> _BUFLM5W {
        _BUFLM5W { w: self }
    }
    #[doc = "Bit 6 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm6(&mut self) -> _BUFLM6W {
        _BUFLM6W { w: self }
    }
    #[doc = "Bit 7 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm7(&mut self) -> _BUFLM7W {
        _BUFLM7W { w: self }
    }
    #[doc = "Bit 8 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm8(&mut self) -> _BUFLM8W {
        _BUFLM8W { w: self }
    }
    #[doc = "Bit 9 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm9(&mut self) -> _BUFLM9W {
        _BUFLM9W { w: self }
    }
    #[doc = "Bit 10 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm10(&mut self) -> _BUFLM10W {
        _BUFLM10W { w: self }
    }
    #[doc = "Bit 11 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm11(&mut self) -> _BUFLM11W {
        _BUFLM11W { w: self }
    }
    #[doc = "Bit 12 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm12(&mut self) -> _BUFLM12W {
        _BUFLM12W { w: self }
    }
    #[doc = "Bit 13 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm13(&mut self) -> _BUFLM13W {
        _BUFLM13W { w: self }
    }
    #[doc = "Bit 14 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm14(&mut self) -> _BUFLM14W {
        _BUFLM14W { w: self }
    }
    #[doc = "Bit 15 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm15(&mut self) -> _BUFLM15W {
        _BUFLM15W { w: self }
    }
    #[doc = "Bit 16 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm16(&mut self) -> _BUFLM16W {
        _BUFLM16W { w: self }
    }
    #[doc = "Bit 17 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm17(&mut self) -> _BUFLM17W {
        _BUFLM17W { w: self }
    }
    #[doc = "Bit 18 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm18(&mut self) -> _BUFLM18W {
        _BUFLM18W { w: self }
    }
    #[doc = "Bit 19 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm19(&mut self) -> _BUFLM19W {
        _BUFLM19W { w: self }
    }
    #[doc = "Bit 20 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm20(&mut self) -> _BUFLM20W {
        _BUFLM20W { w: self }
    }
    #[doc = "Bit 21 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm21(&mut self) -> _BUFLM21W {
        _BUFLM21W { w: self }
    }
    #[doc = "Bit 22 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm22(&mut self) -> _BUFLM22W {
        _BUFLM22W { w: self }
    }
    #[doc = "Bit 23 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm23(&mut self) -> _BUFLM23W {
        _BUFLM23W { w: self }
    }
    #[doc = "Bit 24 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm24(&mut self) -> _BUFLM24W {
        _BUFLM24W { w: self }
    }
    #[doc = "Bit 25 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm25(&mut self) -> _BUFLM25W {
        _BUFLM25W { w: self }
    }
    #[doc = "Bit 26 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm26(&mut self) -> _BUFLM26W {
        _BUFLM26W { w: self }
    }
    #[doc = "Bit 27 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm27(&mut self) -> _BUFLM27W {
        _BUFLM27W { w: self }
    }
    #[doc = "Bit 28 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm28(&mut self) -> _BUFLM28W {
        _BUFLM28W { w: self }
    }
    #[doc = "Bit 29 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm29(&mut self) -> _BUFLM29W {
        _BUFLM29W { w: self }
    }
    #[doc = "Bit 30 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm30(&mut self) -> _BUFLM30W {
        _BUFLM30W { w: self }
    }
    #[doc = "Bit 31 - Buffer MB i Mask"]
    #[inline]
    pub fn buflm31(&mut self) -> _BUFLM31W {
        _BUFLM31W { w: self }
    }
}
