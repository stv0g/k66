#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RXFGMASK {
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
#[doc = "Possible values of the field `FGM0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM0R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM0R {
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
            FGM0R::_0 => false,
            FGM0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FGM0R {
        match value {
            false => FGM0R::_0,
            true => FGM0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FGM0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FGM0R::_1
    }
}
#[doc = "Possible values of the field `FGM1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM1R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM1R {
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
            FGM1R::_0 => false,
            FGM1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FGM1R {
        match value {
            false => FGM1R::_0,
            true => FGM1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FGM1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FGM1R::_1
    }
}
#[doc = "Possible values of the field `FGM2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM2R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM2R {
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
            FGM2R::_0 => false,
            FGM2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FGM2R {
        match value {
            false => FGM2R::_0,
            true => FGM2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FGM2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FGM2R::_1
    }
}
#[doc = "Possible values of the field `FGM3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM3R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM3R {
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
            FGM3R::_0 => false,
            FGM3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FGM3R {
        match value {
            false => FGM3R::_0,
            true => FGM3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FGM3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FGM3R::_1
    }
}
#[doc = "Possible values of the field `FGM4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM4R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM4R {
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
            FGM4R::_0 => false,
            FGM4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FGM4R {
        match value {
            false => FGM4R::_0,
            true => FGM4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FGM4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FGM4R::_1
    }
}
#[doc = "Possible values of the field `FGM5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM5R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM5R {
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
            FGM5R::_0 => false,
            FGM5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FGM5R {
        match value {
            false => FGM5R::_0,
            true => FGM5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FGM5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FGM5R::_1
    }
}
#[doc = "Possible values of the field `FGM6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM6R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM6R {
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
            FGM6R::_0 => false,
            FGM6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FGM6R {
        match value {
            false => FGM6R::_0,
            true => FGM6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FGM6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FGM6R::_1
    }
}
#[doc = "Possible values of the field `FGM7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM7R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM7R {
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
            FGM7R::_0 => false,
            FGM7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FGM7R {
        match value {
            false => FGM7R::_0,
            true => FGM7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FGM7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FGM7R::_1
    }
}
#[doc = "Possible values of the field `FGM8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM8R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM8R {
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
            FGM8R::_0 => false,
            FGM8R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FGM8R {
        match value {
            false => FGM8R::_0,
            true => FGM8R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FGM8R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FGM8R::_1
    }
}
#[doc = "Possible values of the field `FGM9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM9R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM9R {
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
            FGM9R::_0 => false,
            FGM9R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FGM9R {
        match value {
            false => FGM9R::_0,
            true => FGM9R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FGM9R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FGM9R::_1
    }
}
#[doc = "Possible values of the field `FGM10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM10R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM10R {
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
            FGM10R::_0 => false,
            FGM10R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FGM10R {
        match value {
            false => FGM10R::_0,
            true => FGM10R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FGM10R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FGM10R::_1
    }
}
#[doc = "Possible values of the field `FGM11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM11R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM11R {
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
            FGM11R::_0 => false,
            FGM11R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FGM11R {
        match value {
            false => FGM11R::_0,
            true => FGM11R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FGM11R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FGM11R::_1
    }
}
#[doc = "Possible values of the field `FGM12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM12R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM12R {
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
            FGM12R::_0 => false,
            FGM12R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FGM12R {
        match value {
            false => FGM12R::_0,
            true => FGM12R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FGM12R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FGM12R::_1
    }
}
#[doc = "Possible values of the field `FGM13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM13R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM13R {
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
            FGM13R::_0 => false,
            FGM13R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FGM13R {
        match value {
            false => FGM13R::_0,
            true => FGM13R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FGM13R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FGM13R::_1
    }
}
#[doc = "Possible values of the field `FGM14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM14R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM14R {
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
            FGM14R::_0 => false,
            FGM14R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FGM14R {
        match value {
            false => FGM14R::_0,
            true => FGM14R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FGM14R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FGM14R::_1
    }
}
#[doc = "Possible values of the field `FGM15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM15R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM15R {
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
            FGM15R::_0 => false,
            FGM15R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FGM15R {
        match value {
            false => FGM15R::_0,
            true => FGM15R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FGM15R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FGM15R::_1
    }
}
#[doc = "Possible values of the field `FGM16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM16R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM16R {
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
            FGM16R::_0 => false,
            FGM16R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FGM16R {
        match value {
            false => FGM16R::_0,
            true => FGM16R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FGM16R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FGM16R::_1
    }
}
#[doc = "Possible values of the field `FGM17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM17R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM17R {
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
            FGM17R::_0 => false,
            FGM17R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FGM17R {
        match value {
            false => FGM17R::_0,
            true => FGM17R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FGM17R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FGM17R::_1
    }
}
#[doc = "Possible values of the field `FGM18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM18R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM18R {
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
            FGM18R::_0 => false,
            FGM18R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FGM18R {
        match value {
            false => FGM18R::_0,
            true => FGM18R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FGM18R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FGM18R::_1
    }
}
#[doc = "Possible values of the field `FGM19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM19R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM19R {
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
            FGM19R::_0 => false,
            FGM19R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FGM19R {
        match value {
            false => FGM19R::_0,
            true => FGM19R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FGM19R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FGM19R::_1
    }
}
#[doc = "Possible values of the field `FGM20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM20R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM20R {
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
            FGM20R::_0 => false,
            FGM20R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FGM20R {
        match value {
            false => FGM20R::_0,
            true => FGM20R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FGM20R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FGM20R::_1
    }
}
#[doc = "Possible values of the field `FGM21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM21R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM21R {
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
            FGM21R::_0 => false,
            FGM21R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FGM21R {
        match value {
            false => FGM21R::_0,
            true => FGM21R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FGM21R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FGM21R::_1
    }
}
#[doc = "Possible values of the field `FGM22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM22R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM22R {
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
            FGM22R::_0 => false,
            FGM22R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FGM22R {
        match value {
            false => FGM22R::_0,
            true => FGM22R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FGM22R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FGM22R::_1
    }
}
#[doc = "Possible values of the field `FGM23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM23R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM23R {
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
            FGM23R::_0 => false,
            FGM23R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FGM23R {
        match value {
            false => FGM23R::_0,
            true => FGM23R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FGM23R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FGM23R::_1
    }
}
#[doc = "Possible values of the field `FGM24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM24R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM24R {
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
            FGM24R::_0 => false,
            FGM24R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FGM24R {
        match value {
            false => FGM24R::_0,
            true => FGM24R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FGM24R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FGM24R::_1
    }
}
#[doc = "Possible values of the field `FGM25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM25R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM25R {
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
            FGM25R::_0 => false,
            FGM25R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FGM25R {
        match value {
            false => FGM25R::_0,
            true => FGM25R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FGM25R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FGM25R::_1
    }
}
#[doc = "Possible values of the field `FGM26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM26R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM26R {
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
            FGM26R::_0 => false,
            FGM26R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FGM26R {
        match value {
            false => FGM26R::_0,
            true => FGM26R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FGM26R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FGM26R::_1
    }
}
#[doc = "Possible values of the field `FGM27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM27R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM27R {
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
            FGM27R::_0 => false,
            FGM27R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FGM27R {
        match value {
            false => FGM27R::_0,
            true => FGM27R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FGM27R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FGM27R::_1
    }
}
#[doc = "Possible values of the field `FGM28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM28R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM28R {
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
            FGM28R::_0 => false,
            FGM28R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FGM28R {
        match value {
            false => FGM28R::_0,
            true => FGM28R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FGM28R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FGM28R::_1
    }
}
#[doc = "Possible values of the field `FGM29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM29R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM29R {
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
            FGM29R::_0 => false,
            FGM29R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FGM29R {
        match value {
            false => FGM29R::_0,
            true => FGM29R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FGM29R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FGM29R::_1
    }
}
#[doc = "Possible values of the field `FGM30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM30R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM30R {
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
            FGM30R::_0 => false,
            FGM30R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FGM30R {
        match value {
            false => FGM30R::_0,
            true => FGM30R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FGM30R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FGM30R::_1
    }
}
#[doc = "Possible values of the field `FGM31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FGM31R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM31R {
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
            FGM31R::_0 => false,
            FGM31R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FGM31R {
        match value {
            false => FGM31R::_0,
            true => FGM31R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FGM31R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FGM31R::_1
    }
}
#[doc = "Values that can be written to the field `FGM0`"]
pub enum FGM0W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FGM0W::_0 => false,
            FGM0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FGM0W<'a> {
    w: &'a mut W,
}
impl<'a> _FGM0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FGM0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM0W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM0W::_1)
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
#[doc = "Values that can be written to the field `FGM1`"]
pub enum FGM1W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FGM1W::_0 => false,
            FGM1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FGM1W<'a> {
    w: &'a mut W,
}
impl<'a> _FGM1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FGM1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM1W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM1W::_1)
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
#[doc = "Values that can be written to the field `FGM2`"]
pub enum FGM2W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FGM2W::_0 => false,
            FGM2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FGM2W<'a> {
    w: &'a mut W,
}
impl<'a> _FGM2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FGM2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM2W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM2W::_1)
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
#[doc = "Values that can be written to the field `FGM3`"]
pub enum FGM3W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FGM3W::_0 => false,
            FGM3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FGM3W<'a> {
    w: &'a mut W,
}
impl<'a> _FGM3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FGM3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM3W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM3W::_1)
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
#[doc = "Values that can be written to the field `FGM4`"]
pub enum FGM4W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FGM4W::_0 => false,
            FGM4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FGM4W<'a> {
    w: &'a mut W,
}
impl<'a> _FGM4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FGM4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM4W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM4W::_1)
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
#[doc = "Values that can be written to the field `FGM5`"]
pub enum FGM5W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FGM5W::_0 => false,
            FGM5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FGM5W<'a> {
    w: &'a mut W,
}
impl<'a> _FGM5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FGM5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM5W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM5W::_1)
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
#[doc = "Values that can be written to the field `FGM6`"]
pub enum FGM6W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FGM6W::_0 => false,
            FGM6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FGM6W<'a> {
    w: &'a mut W,
}
impl<'a> _FGM6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FGM6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM6W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM6W::_1)
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
#[doc = "Values that can be written to the field `FGM7`"]
pub enum FGM7W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FGM7W::_0 => false,
            FGM7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FGM7W<'a> {
    w: &'a mut W,
}
impl<'a> _FGM7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FGM7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM7W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM7W::_1)
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
#[doc = "Values that can be written to the field `FGM8`"]
pub enum FGM8W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FGM8W::_0 => false,
            FGM8W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FGM8W<'a> {
    w: &'a mut W,
}
impl<'a> _FGM8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FGM8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM8W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM8W::_1)
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
#[doc = "Values that can be written to the field `FGM9`"]
pub enum FGM9W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FGM9W::_0 => false,
            FGM9W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FGM9W<'a> {
    w: &'a mut W,
}
impl<'a> _FGM9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FGM9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM9W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM9W::_1)
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
#[doc = "Values that can be written to the field `FGM10`"]
pub enum FGM10W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FGM10W::_0 => false,
            FGM10W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FGM10W<'a> {
    w: &'a mut W,
}
impl<'a> _FGM10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FGM10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM10W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM10W::_1)
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
#[doc = "Values that can be written to the field `FGM11`"]
pub enum FGM11W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FGM11W::_0 => false,
            FGM11W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FGM11W<'a> {
    w: &'a mut W,
}
impl<'a> _FGM11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FGM11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM11W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM11W::_1)
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
#[doc = "Values that can be written to the field `FGM12`"]
pub enum FGM12W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FGM12W::_0 => false,
            FGM12W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FGM12W<'a> {
    w: &'a mut W,
}
impl<'a> _FGM12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FGM12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM12W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM12W::_1)
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
#[doc = "Values that can be written to the field `FGM13`"]
pub enum FGM13W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FGM13W::_0 => false,
            FGM13W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FGM13W<'a> {
    w: &'a mut W,
}
impl<'a> _FGM13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FGM13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM13W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM13W::_1)
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
#[doc = "Values that can be written to the field `FGM14`"]
pub enum FGM14W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FGM14W::_0 => false,
            FGM14W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FGM14W<'a> {
    w: &'a mut W,
}
impl<'a> _FGM14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FGM14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM14W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM14W::_1)
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
#[doc = "Values that can be written to the field `FGM15`"]
pub enum FGM15W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FGM15W::_0 => false,
            FGM15W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FGM15W<'a> {
    w: &'a mut W,
}
impl<'a> _FGM15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FGM15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM15W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM15W::_1)
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
#[doc = "Values that can be written to the field `FGM16`"]
pub enum FGM16W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FGM16W::_0 => false,
            FGM16W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FGM16W<'a> {
    w: &'a mut W,
}
impl<'a> _FGM16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FGM16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM16W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM16W::_1)
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
#[doc = "Values that can be written to the field `FGM17`"]
pub enum FGM17W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FGM17W::_0 => false,
            FGM17W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FGM17W<'a> {
    w: &'a mut W,
}
impl<'a> _FGM17W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FGM17W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM17W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM17W::_1)
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
#[doc = "Values that can be written to the field `FGM18`"]
pub enum FGM18W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FGM18W::_0 => false,
            FGM18W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FGM18W<'a> {
    w: &'a mut W,
}
impl<'a> _FGM18W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FGM18W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM18W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM18W::_1)
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
#[doc = "Values that can be written to the field `FGM19`"]
pub enum FGM19W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FGM19W::_0 => false,
            FGM19W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FGM19W<'a> {
    w: &'a mut W,
}
impl<'a> _FGM19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FGM19W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM19W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM19W::_1)
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
#[doc = "Values that can be written to the field `FGM20`"]
pub enum FGM20W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FGM20W::_0 => false,
            FGM20W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FGM20W<'a> {
    w: &'a mut W,
}
impl<'a> _FGM20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FGM20W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM20W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM20W::_1)
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
#[doc = "Values that can be written to the field `FGM21`"]
pub enum FGM21W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FGM21W::_0 => false,
            FGM21W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FGM21W<'a> {
    w: &'a mut W,
}
impl<'a> _FGM21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FGM21W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM21W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM21W::_1)
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
#[doc = "Values that can be written to the field `FGM22`"]
pub enum FGM22W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FGM22W::_0 => false,
            FGM22W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FGM22W<'a> {
    w: &'a mut W,
}
impl<'a> _FGM22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FGM22W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM22W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM22W::_1)
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
#[doc = "Values that can be written to the field `FGM23`"]
pub enum FGM23W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FGM23W::_0 => false,
            FGM23W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FGM23W<'a> {
    w: &'a mut W,
}
impl<'a> _FGM23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FGM23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM23W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM23W::_1)
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
#[doc = "Values that can be written to the field `FGM24`"]
pub enum FGM24W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FGM24W::_0 => false,
            FGM24W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FGM24W<'a> {
    w: &'a mut W,
}
impl<'a> _FGM24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FGM24W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM24W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM24W::_1)
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
#[doc = "Values that can be written to the field `FGM25`"]
pub enum FGM25W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FGM25W::_0 => false,
            FGM25W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FGM25W<'a> {
    w: &'a mut W,
}
impl<'a> _FGM25W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FGM25W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM25W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM25W::_1)
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
#[doc = "Values that can be written to the field `FGM26`"]
pub enum FGM26W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FGM26W::_0 => false,
            FGM26W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FGM26W<'a> {
    w: &'a mut W,
}
impl<'a> _FGM26W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FGM26W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM26W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM26W::_1)
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
#[doc = "Values that can be written to the field `FGM27`"]
pub enum FGM27W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FGM27W::_0 => false,
            FGM27W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FGM27W<'a> {
    w: &'a mut W,
}
impl<'a> _FGM27W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FGM27W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM27W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM27W::_1)
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
#[doc = "Values that can be written to the field `FGM28`"]
pub enum FGM28W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FGM28W::_0 => false,
            FGM28W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FGM28W<'a> {
    w: &'a mut W,
}
impl<'a> _FGM28W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FGM28W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM28W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM28W::_1)
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
#[doc = "Values that can be written to the field `FGM29`"]
pub enum FGM29W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FGM29W::_0 => false,
            FGM29W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FGM29W<'a> {
    w: &'a mut W,
}
impl<'a> _FGM29W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FGM29W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM29W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM29W::_1)
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
#[doc = "Values that can be written to the field `FGM30`"]
pub enum FGM30W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FGM30W::_0 => false,
            FGM30W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FGM30W<'a> {
    w: &'a mut W,
}
impl<'a> _FGM30W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FGM30W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM30W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM30W::_1)
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
#[doc = "Values that can be written to the field `FGM31`"]
pub enum FGM31W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl FGM31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FGM31W::_0 => false,
            FGM31W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FGM31W<'a> {
    w: &'a mut W,
}
impl<'a> _FGM31W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FGM31W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FGM31W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FGM31W::_1)
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
    #[doc = "Bit 0 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm0(&self) -> FGM0R {
        FGM0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm1(&self) -> FGM1R {
        FGM1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm2(&self) -> FGM2R {
        FGM2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm3(&self) -> FGM3R {
        FGM3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm4(&self) -> FGM4R {
        FGM4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm5(&self) -> FGM5R {
        FGM5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm6(&self) -> FGM6R {
        FGM6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm7(&self) -> FGM7R {
        FGM7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm8(&self) -> FGM8R {
        FGM8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm9(&self) -> FGM9R {
        FGM9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm10(&self) -> FGM10R {
        FGM10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm11(&self) -> FGM11R {
        FGM11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm12(&self) -> FGM12R {
        FGM12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm13(&self) -> FGM13R {
        FGM13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm14(&self) -> FGM14R {
        FGM14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm15(&self) -> FGM15R {
        FGM15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm16(&self) -> FGM16R {
        FGM16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm17(&self) -> FGM17R {
        FGM17R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm18(&self) -> FGM18R {
        FGM18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm19(&self) -> FGM19R {
        FGM19R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm20(&self) -> FGM20R {
        FGM20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm21(&self) -> FGM21R {
        FGM21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm22(&self) -> FGM22R {
        FGM22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm23(&self) -> FGM23R {
        FGM23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm24(&self) -> FGM24R {
        FGM24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm25(&self) -> FGM25R {
        FGM25R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm26(&self) -> FGM26R {
        FGM26R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm27(&self) -> FGM27R {
        FGM27R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm28(&self) -> FGM28R {
        FGM28R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm29(&self) -> FGM29R {
        FGM29R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm30(&self) -> FGM30R {
        FGM30R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm31(&self) -> FGM31R {
        FGM31R::_from({
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
        W { bits: 4294967295 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm0(&mut self) -> _FGM0W {
        _FGM0W { w: self }
    }
    #[doc = "Bit 1 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm1(&mut self) -> _FGM1W {
        _FGM1W { w: self }
    }
    #[doc = "Bit 2 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm2(&mut self) -> _FGM2W {
        _FGM2W { w: self }
    }
    #[doc = "Bit 3 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm3(&mut self) -> _FGM3W {
        _FGM3W { w: self }
    }
    #[doc = "Bit 4 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm4(&mut self) -> _FGM4W {
        _FGM4W { w: self }
    }
    #[doc = "Bit 5 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm5(&mut self) -> _FGM5W {
        _FGM5W { w: self }
    }
    #[doc = "Bit 6 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm6(&mut self) -> _FGM6W {
        _FGM6W { w: self }
    }
    #[doc = "Bit 7 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm7(&mut self) -> _FGM7W {
        _FGM7W { w: self }
    }
    #[doc = "Bit 8 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm8(&mut self) -> _FGM8W {
        _FGM8W { w: self }
    }
    #[doc = "Bit 9 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm9(&mut self) -> _FGM9W {
        _FGM9W { w: self }
    }
    #[doc = "Bit 10 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm10(&mut self) -> _FGM10W {
        _FGM10W { w: self }
    }
    #[doc = "Bit 11 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm11(&mut self) -> _FGM11W {
        _FGM11W { w: self }
    }
    #[doc = "Bit 12 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm12(&mut self) -> _FGM12W {
        _FGM12W { w: self }
    }
    #[doc = "Bit 13 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm13(&mut self) -> _FGM13W {
        _FGM13W { w: self }
    }
    #[doc = "Bit 14 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm14(&mut self) -> _FGM14W {
        _FGM14W { w: self }
    }
    #[doc = "Bit 15 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm15(&mut self) -> _FGM15W {
        _FGM15W { w: self }
    }
    #[doc = "Bit 16 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm16(&mut self) -> _FGM16W {
        _FGM16W { w: self }
    }
    #[doc = "Bit 17 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm17(&mut self) -> _FGM17W {
        _FGM17W { w: self }
    }
    #[doc = "Bit 18 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm18(&mut self) -> _FGM18W {
        _FGM18W { w: self }
    }
    #[doc = "Bit 19 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm19(&mut self) -> _FGM19W {
        _FGM19W { w: self }
    }
    #[doc = "Bit 20 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm20(&mut self) -> _FGM20W {
        _FGM20W { w: self }
    }
    #[doc = "Bit 21 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm21(&mut self) -> _FGM21W {
        _FGM21W { w: self }
    }
    #[doc = "Bit 22 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm22(&mut self) -> _FGM22W {
        _FGM22W { w: self }
    }
    #[doc = "Bit 23 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm23(&mut self) -> _FGM23W {
        _FGM23W { w: self }
    }
    #[doc = "Bit 24 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm24(&mut self) -> _FGM24W {
        _FGM24W { w: self }
    }
    #[doc = "Bit 25 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm25(&mut self) -> _FGM25W {
        _FGM25W { w: self }
    }
    #[doc = "Bit 26 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm26(&mut self) -> _FGM26W {
        _FGM26W { w: self }
    }
    #[doc = "Bit 27 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm27(&mut self) -> _FGM27W {
        _FGM27W { w: self }
    }
    #[doc = "Bit 28 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm28(&mut self) -> _FGM28W {
        _FGM28W { w: self }
    }
    #[doc = "Bit 29 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm29(&mut self) -> _FGM29W {
        _FGM29W { w: self }
    }
    #[doc = "Bit 30 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm30(&mut self) -> _FGM30W {
        _FGM30W { w: self }
    }
    #[doc = "Bit 31 - Rx FIFO Global Mask Bits"]
    #[inline]
    pub fn fgm31(&mut self) -> _FGM31W {
        _FGM31W { w: self }
    }
}
