#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RXIMR {
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
#[doc = "Possible values of the field `MI0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI0R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI0R {
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
            MI0R::_0 => false,
            MI0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MI0R {
        match value {
            false => MI0R::_0,
            true => MI0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MI0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MI0R::_1
    }
}
#[doc = "Possible values of the field `MI1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI1R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI1R {
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
            MI1R::_0 => false,
            MI1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MI1R {
        match value {
            false => MI1R::_0,
            true => MI1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MI1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MI1R::_1
    }
}
#[doc = "Possible values of the field `MI2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI2R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI2R {
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
            MI2R::_0 => false,
            MI2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MI2R {
        match value {
            false => MI2R::_0,
            true => MI2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MI2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MI2R::_1
    }
}
#[doc = "Possible values of the field `MI3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI3R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI3R {
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
            MI3R::_0 => false,
            MI3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MI3R {
        match value {
            false => MI3R::_0,
            true => MI3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MI3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MI3R::_1
    }
}
#[doc = "Possible values of the field `MI4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI4R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI4R {
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
            MI4R::_0 => false,
            MI4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MI4R {
        match value {
            false => MI4R::_0,
            true => MI4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MI4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MI4R::_1
    }
}
#[doc = "Possible values of the field `MI5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI5R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI5R {
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
            MI5R::_0 => false,
            MI5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MI5R {
        match value {
            false => MI5R::_0,
            true => MI5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MI5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MI5R::_1
    }
}
#[doc = "Possible values of the field `MI6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI6R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI6R {
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
            MI6R::_0 => false,
            MI6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MI6R {
        match value {
            false => MI6R::_0,
            true => MI6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MI6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MI6R::_1
    }
}
#[doc = "Possible values of the field `MI7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI7R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI7R {
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
            MI7R::_0 => false,
            MI7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MI7R {
        match value {
            false => MI7R::_0,
            true => MI7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MI7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MI7R::_1
    }
}
#[doc = "Possible values of the field `MI8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI8R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI8R {
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
            MI8R::_0 => false,
            MI8R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MI8R {
        match value {
            false => MI8R::_0,
            true => MI8R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MI8R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MI8R::_1
    }
}
#[doc = "Possible values of the field `MI9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI9R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI9R {
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
            MI9R::_0 => false,
            MI9R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MI9R {
        match value {
            false => MI9R::_0,
            true => MI9R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MI9R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MI9R::_1
    }
}
#[doc = "Possible values of the field `MI10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI10R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI10R {
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
            MI10R::_0 => false,
            MI10R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MI10R {
        match value {
            false => MI10R::_0,
            true => MI10R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MI10R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MI10R::_1
    }
}
#[doc = "Possible values of the field `MI11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI11R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI11R {
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
            MI11R::_0 => false,
            MI11R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MI11R {
        match value {
            false => MI11R::_0,
            true => MI11R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MI11R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MI11R::_1
    }
}
#[doc = "Possible values of the field `MI12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI12R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI12R {
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
            MI12R::_0 => false,
            MI12R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MI12R {
        match value {
            false => MI12R::_0,
            true => MI12R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MI12R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MI12R::_1
    }
}
#[doc = "Possible values of the field `MI13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI13R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI13R {
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
            MI13R::_0 => false,
            MI13R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MI13R {
        match value {
            false => MI13R::_0,
            true => MI13R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MI13R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MI13R::_1
    }
}
#[doc = "Possible values of the field `MI14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI14R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI14R {
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
            MI14R::_0 => false,
            MI14R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MI14R {
        match value {
            false => MI14R::_0,
            true => MI14R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MI14R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MI14R::_1
    }
}
#[doc = "Possible values of the field `MI15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI15R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI15R {
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
            MI15R::_0 => false,
            MI15R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MI15R {
        match value {
            false => MI15R::_0,
            true => MI15R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MI15R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MI15R::_1
    }
}
#[doc = "Possible values of the field `MI16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI16R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI16R {
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
            MI16R::_0 => false,
            MI16R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MI16R {
        match value {
            false => MI16R::_0,
            true => MI16R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MI16R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MI16R::_1
    }
}
#[doc = "Possible values of the field `MI17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI17R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI17R {
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
            MI17R::_0 => false,
            MI17R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MI17R {
        match value {
            false => MI17R::_0,
            true => MI17R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MI17R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MI17R::_1
    }
}
#[doc = "Possible values of the field `MI18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI18R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI18R {
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
            MI18R::_0 => false,
            MI18R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MI18R {
        match value {
            false => MI18R::_0,
            true => MI18R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MI18R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MI18R::_1
    }
}
#[doc = "Possible values of the field `MI19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI19R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI19R {
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
            MI19R::_0 => false,
            MI19R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MI19R {
        match value {
            false => MI19R::_0,
            true => MI19R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MI19R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MI19R::_1
    }
}
#[doc = "Possible values of the field `MI20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI20R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI20R {
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
            MI20R::_0 => false,
            MI20R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MI20R {
        match value {
            false => MI20R::_0,
            true => MI20R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MI20R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MI20R::_1
    }
}
#[doc = "Possible values of the field `MI21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI21R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI21R {
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
            MI21R::_0 => false,
            MI21R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MI21R {
        match value {
            false => MI21R::_0,
            true => MI21R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MI21R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MI21R::_1
    }
}
#[doc = "Possible values of the field `MI22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI22R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI22R {
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
            MI22R::_0 => false,
            MI22R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MI22R {
        match value {
            false => MI22R::_0,
            true => MI22R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MI22R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MI22R::_1
    }
}
#[doc = "Possible values of the field `MI23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI23R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI23R {
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
            MI23R::_0 => false,
            MI23R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MI23R {
        match value {
            false => MI23R::_0,
            true => MI23R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MI23R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MI23R::_1
    }
}
#[doc = "Possible values of the field `MI24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI24R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI24R {
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
            MI24R::_0 => false,
            MI24R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MI24R {
        match value {
            false => MI24R::_0,
            true => MI24R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MI24R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MI24R::_1
    }
}
#[doc = "Possible values of the field `MI25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI25R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI25R {
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
            MI25R::_0 => false,
            MI25R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MI25R {
        match value {
            false => MI25R::_0,
            true => MI25R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MI25R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MI25R::_1
    }
}
#[doc = "Possible values of the field `MI26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI26R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI26R {
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
            MI26R::_0 => false,
            MI26R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MI26R {
        match value {
            false => MI26R::_0,
            true => MI26R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MI26R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MI26R::_1
    }
}
#[doc = "Possible values of the field `MI27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI27R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI27R {
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
            MI27R::_0 => false,
            MI27R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MI27R {
        match value {
            false => MI27R::_0,
            true => MI27R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MI27R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MI27R::_1
    }
}
#[doc = "Possible values of the field `MI28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI28R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI28R {
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
            MI28R::_0 => false,
            MI28R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MI28R {
        match value {
            false => MI28R::_0,
            true => MI28R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MI28R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MI28R::_1
    }
}
#[doc = "Possible values of the field `MI29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI29R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI29R {
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
            MI29R::_0 => false,
            MI29R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MI29R {
        match value {
            false => MI29R::_0,
            true => MI29R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MI29R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MI29R::_1
    }
}
#[doc = "Possible values of the field `MI30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI30R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI30R {
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
            MI30R::_0 => false,
            MI30R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MI30R {
        match value {
            false => MI30R::_0,
            true => MI30R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MI30R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MI30R::_1
    }
}
#[doc = "Possible values of the field `MI31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI31R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI31R {
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
            MI31R::_0 => false,
            MI31R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MI31R {
        match value {
            false => MI31R::_0,
            true => MI31R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MI31R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MI31R::_1
    }
}
#[doc = "Values that can be written to the field `MI0`"]
pub enum MI0W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MI0W::_0 => false,
            MI0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MI0W<'a> {
    w: &'a mut W,
}
impl<'a> _MI0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MI0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI0W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI0W::_1)
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
#[doc = "Values that can be written to the field `MI1`"]
pub enum MI1W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MI1W::_0 => false,
            MI1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MI1W<'a> {
    w: &'a mut W,
}
impl<'a> _MI1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MI1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI1W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI1W::_1)
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
#[doc = "Values that can be written to the field `MI2`"]
pub enum MI2W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MI2W::_0 => false,
            MI2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MI2W<'a> {
    w: &'a mut W,
}
impl<'a> _MI2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MI2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI2W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI2W::_1)
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
#[doc = "Values that can be written to the field `MI3`"]
pub enum MI3W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MI3W::_0 => false,
            MI3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MI3W<'a> {
    w: &'a mut W,
}
impl<'a> _MI3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MI3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI3W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI3W::_1)
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
#[doc = "Values that can be written to the field `MI4`"]
pub enum MI4W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MI4W::_0 => false,
            MI4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MI4W<'a> {
    w: &'a mut W,
}
impl<'a> _MI4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MI4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI4W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI4W::_1)
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
#[doc = "Values that can be written to the field `MI5`"]
pub enum MI5W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MI5W::_0 => false,
            MI5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MI5W<'a> {
    w: &'a mut W,
}
impl<'a> _MI5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MI5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI5W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI5W::_1)
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
#[doc = "Values that can be written to the field `MI6`"]
pub enum MI6W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MI6W::_0 => false,
            MI6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MI6W<'a> {
    w: &'a mut W,
}
impl<'a> _MI6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MI6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI6W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI6W::_1)
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
#[doc = "Values that can be written to the field `MI7`"]
pub enum MI7W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MI7W::_0 => false,
            MI7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MI7W<'a> {
    w: &'a mut W,
}
impl<'a> _MI7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MI7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI7W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI7W::_1)
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
#[doc = "Values that can be written to the field `MI8`"]
pub enum MI8W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MI8W::_0 => false,
            MI8W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MI8W<'a> {
    w: &'a mut W,
}
impl<'a> _MI8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MI8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI8W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI8W::_1)
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
#[doc = "Values that can be written to the field `MI9`"]
pub enum MI9W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MI9W::_0 => false,
            MI9W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MI9W<'a> {
    w: &'a mut W,
}
impl<'a> _MI9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MI9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI9W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI9W::_1)
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
#[doc = "Values that can be written to the field `MI10`"]
pub enum MI10W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MI10W::_0 => false,
            MI10W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MI10W<'a> {
    w: &'a mut W,
}
impl<'a> _MI10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MI10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI10W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI10W::_1)
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
#[doc = "Values that can be written to the field `MI11`"]
pub enum MI11W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MI11W::_0 => false,
            MI11W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MI11W<'a> {
    w: &'a mut W,
}
impl<'a> _MI11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MI11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI11W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI11W::_1)
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
#[doc = "Values that can be written to the field `MI12`"]
pub enum MI12W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MI12W::_0 => false,
            MI12W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MI12W<'a> {
    w: &'a mut W,
}
impl<'a> _MI12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MI12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI12W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI12W::_1)
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
#[doc = "Values that can be written to the field `MI13`"]
pub enum MI13W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MI13W::_0 => false,
            MI13W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MI13W<'a> {
    w: &'a mut W,
}
impl<'a> _MI13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MI13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI13W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI13W::_1)
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
#[doc = "Values that can be written to the field `MI14`"]
pub enum MI14W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MI14W::_0 => false,
            MI14W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MI14W<'a> {
    w: &'a mut W,
}
impl<'a> _MI14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MI14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI14W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI14W::_1)
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
#[doc = "Values that can be written to the field `MI15`"]
pub enum MI15W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MI15W::_0 => false,
            MI15W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MI15W<'a> {
    w: &'a mut W,
}
impl<'a> _MI15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MI15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI15W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI15W::_1)
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
#[doc = "Values that can be written to the field `MI16`"]
pub enum MI16W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MI16W::_0 => false,
            MI16W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MI16W<'a> {
    w: &'a mut W,
}
impl<'a> _MI16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MI16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI16W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI16W::_1)
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
#[doc = "Values that can be written to the field `MI17`"]
pub enum MI17W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MI17W::_0 => false,
            MI17W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MI17W<'a> {
    w: &'a mut W,
}
impl<'a> _MI17W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MI17W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI17W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI17W::_1)
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
#[doc = "Values that can be written to the field `MI18`"]
pub enum MI18W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MI18W::_0 => false,
            MI18W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MI18W<'a> {
    w: &'a mut W,
}
impl<'a> _MI18W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MI18W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI18W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI18W::_1)
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
#[doc = "Values that can be written to the field `MI19`"]
pub enum MI19W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MI19W::_0 => false,
            MI19W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MI19W<'a> {
    w: &'a mut W,
}
impl<'a> _MI19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MI19W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI19W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI19W::_1)
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
#[doc = "Values that can be written to the field `MI20`"]
pub enum MI20W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MI20W::_0 => false,
            MI20W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MI20W<'a> {
    w: &'a mut W,
}
impl<'a> _MI20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MI20W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI20W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI20W::_1)
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
#[doc = "Values that can be written to the field `MI21`"]
pub enum MI21W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MI21W::_0 => false,
            MI21W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MI21W<'a> {
    w: &'a mut W,
}
impl<'a> _MI21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MI21W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI21W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI21W::_1)
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
#[doc = "Values that can be written to the field `MI22`"]
pub enum MI22W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MI22W::_0 => false,
            MI22W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MI22W<'a> {
    w: &'a mut W,
}
impl<'a> _MI22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MI22W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI22W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI22W::_1)
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
#[doc = "Values that can be written to the field `MI23`"]
pub enum MI23W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MI23W::_0 => false,
            MI23W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MI23W<'a> {
    w: &'a mut W,
}
impl<'a> _MI23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MI23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI23W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI23W::_1)
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
#[doc = "Values that can be written to the field `MI24`"]
pub enum MI24W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MI24W::_0 => false,
            MI24W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MI24W<'a> {
    w: &'a mut W,
}
impl<'a> _MI24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MI24W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI24W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI24W::_1)
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
#[doc = "Values that can be written to the field `MI25`"]
pub enum MI25W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MI25W::_0 => false,
            MI25W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MI25W<'a> {
    w: &'a mut W,
}
impl<'a> _MI25W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MI25W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI25W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI25W::_1)
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
#[doc = "Values that can be written to the field `MI26`"]
pub enum MI26W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MI26W::_0 => false,
            MI26W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MI26W<'a> {
    w: &'a mut W,
}
impl<'a> _MI26W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MI26W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI26W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI26W::_1)
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
#[doc = "Values that can be written to the field `MI27`"]
pub enum MI27W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MI27W::_0 => false,
            MI27W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MI27W<'a> {
    w: &'a mut W,
}
impl<'a> _MI27W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MI27W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI27W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI27W::_1)
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
#[doc = "Values that can be written to the field `MI28`"]
pub enum MI28W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MI28W::_0 => false,
            MI28W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MI28W<'a> {
    w: &'a mut W,
}
impl<'a> _MI28W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MI28W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI28W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI28W::_1)
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
#[doc = "Values that can be written to the field `MI29`"]
pub enum MI29W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MI29W::_0 => false,
            MI29W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MI29W<'a> {
    w: &'a mut W,
}
impl<'a> _MI29W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MI29W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI29W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI29W::_1)
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
#[doc = "Values that can be written to the field `MI30`"]
pub enum MI30W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MI30W::_0 => false,
            MI30W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MI30W<'a> {
    w: &'a mut W,
}
impl<'a> _MI30W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MI30W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI30W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI30W::_1)
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
#[doc = "Values that can be written to the field `MI31`"]
pub enum MI31W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MI31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MI31W::_0 => false,
            MI31W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MI31W<'a> {
    w: &'a mut W,
}
impl<'a> _MI31W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MI31W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI31W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI31W::_1)
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
    #[doc = "Bit 0 - Individual Mask Bits"]
    #[inline]
    pub fn mi0(&self) -> MI0R {
        MI0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Individual Mask Bits"]
    #[inline]
    pub fn mi1(&self) -> MI1R {
        MI1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Individual Mask Bits"]
    #[inline]
    pub fn mi2(&self) -> MI2R {
        MI2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Individual Mask Bits"]
    #[inline]
    pub fn mi3(&self) -> MI3R {
        MI3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Individual Mask Bits"]
    #[inline]
    pub fn mi4(&self) -> MI4R {
        MI4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Individual Mask Bits"]
    #[inline]
    pub fn mi5(&self) -> MI5R {
        MI5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Individual Mask Bits"]
    #[inline]
    pub fn mi6(&self) -> MI6R {
        MI6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Individual Mask Bits"]
    #[inline]
    pub fn mi7(&self) -> MI7R {
        MI7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Individual Mask Bits"]
    #[inline]
    pub fn mi8(&self) -> MI8R {
        MI8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Individual Mask Bits"]
    #[inline]
    pub fn mi9(&self) -> MI9R {
        MI9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Individual Mask Bits"]
    #[inline]
    pub fn mi10(&self) -> MI10R {
        MI10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Individual Mask Bits"]
    #[inline]
    pub fn mi11(&self) -> MI11R {
        MI11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Individual Mask Bits"]
    #[inline]
    pub fn mi12(&self) -> MI12R {
        MI12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Individual Mask Bits"]
    #[inline]
    pub fn mi13(&self) -> MI13R {
        MI13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Individual Mask Bits"]
    #[inline]
    pub fn mi14(&self) -> MI14R {
        MI14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Individual Mask Bits"]
    #[inline]
    pub fn mi15(&self) -> MI15R {
        MI15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Individual Mask Bits"]
    #[inline]
    pub fn mi16(&self) -> MI16R {
        MI16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Individual Mask Bits"]
    #[inline]
    pub fn mi17(&self) -> MI17R {
        MI17R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Individual Mask Bits"]
    #[inline]
    pub fn mi18(&self) -> MI18R {
        MI18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Individual Mask Bits"]
    #[inline]
    pub fn mi19(&self) -> MI19R {
        MI19R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Individual Mask Bits"]
    #[inline]
    pub fn mi20(&self) -> MI20R {
        MI20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Individual Mask Bits"]
    #[inline]
    pub fn mi21(&self) -> MI21R {
        MI21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Individual Mask Bits"]
    #[inline]
    pub fn mi22(&self) -> MI22R {
        MI22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Individual Mask Bits"]
    #[inline]
    pub fn mi23(&self) -> MI23R {
        MI23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Individual Mask Bits"]
    #[inline]
    pub fn mi24(&self) -> MI24R {
        MI24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Individual Mask Bits"]
    #[inline]
    pub fn mi25(&self) -> MI25R {
        MI25R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Individual Mask Bits"]
    #[inline]
    pub fn mi26(&self) -> MI26R {
        MI26R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Individual Mask Bits"]
    #[inline]
    pub fn mi27(&self) -> MI27R {
        MI27R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Individual Mask Bits"]
    #[inline]
    pub fn mi28(&self) -> MI28R {
        MI28R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Individual Mask Bits"]
    #[inline]
    pub fn mi29(&self) -> MI29R {
        MI29R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Individual Mask Bits"]
    #[inline]
    pub fn mi30(&self) -> MI30R {
        MI30R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Individual Mask Bits"]
    #[inline]
    pub fn mi31(&self) -> MI31R {
        MI31R::_from({
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
    #[doc = "Bit 0 - Individual Mask Bits"]
    #[inline]
    pub fn mi0(&mut self) -> _MI0W {
        _MI0W { w: self }
    }
    #[doc = "Bit 1 - Individual Mask Bits"]
    #[inline]
    pub fn mi1(&mut self) -> _MI1W {
        _MI1W { w: self }
    }
    #[doc = "Bit 2 - Individual Mask Bits"]
    #[inline]
    pub fn mi2(&mut self) -> _MI2W {
        _MI2W { w: self }
    }
    #[doc = "Bit 3 - Individual Mask Bits"]
    #[inline]
    pub fn mi3(&mut self) -> _MI3W {
        _MI3W { w: self }
    }
    #[doc = "Bit 4 - Individual Mask Bits"]
    #[inline]
    pub fn mi4(&mut self) -> _MI4W {
        _MI4W { w: self }
    }
    #[doc = "Bit 5 - Individual Mask Bits"]
    #[inline]
    pub fn mi5(&mut self) -> _MI5W {
        _MI5W { w: self }
    }
    #[doc = "Bit 6 - Individual Mask Bits"]
    #[inline]
    pub fn mi6(&mut self) -> _MI6W {
        _MI6W { w: self }
    }
    #[doc = "Bit 7 - Individual Mask Bits"]
    #[inline]
    pub fn mi7(&mut self) -> _MI7W {
        _MI7W { w: self }
    }
    #[doc = "Bit 8 - Individual Mask Bits"]
    #[inline]
    pub fn mi8(&mut self) -> _MI8W {
        _MI8W { w: self }
    }
    #[doc = "Bit 9 - Individual Mask Bits"]
    #[inline]
    pub fn mi9(&mut self) -> _MI9W {
        _MI9W { w: self }
    }
    #[doc = "Bit 10 - Individual Mask Bits"]
    #[inline]
    pub fn mi10(&mut self) -> _MI10W {
        _MI10W { w: self }
    }
    #[doc = "Bit 11 - Individual Mask Bits"]
    #[inline]
    pub fn mi11(&mut self) -> _MI11W {
        _MI11W { w: self }
    }
    #[doc = "Bit 12 - Individual Mask Bits"]
    #[inline]
    pub fn mi12(&mut self) -> _MI12W {
        _MI12W { w: self }
    }
    #[doc = "Bit 13 - Individual Mask Bits"]
    #[inline]
    pub fn mi13(&mut self) -> _MI13W {
        _MI13W { w: self }
    }
    #[doc = "Bit 14 - Individual Mask Bits"]
    #[inline]
    pub fn mi14(&mut self) -> _MI14W {
        _MI14W { w: self }
    }
    #[doc = "Bit 15 - Individual Mask Bits"]
    #[inline]
    pub fn mi15(&mut self) -> _MI15W {
        _MI15W { w: self }
    }
    #[doc = "Bit 16 - Individual Mask Bits"]
    #[inline]
    pub fn mi16(&mut self) -> _MI16W {
        _MI16W { w: self }
    }
    #[doc = "Bit 17 - Individual Mask Bits"]
    #[inline]
    pub fn mi17(&mut self) -> _MI17W {
        _MI17W { w: self }
    }
    #[doc = "Bit 18 - Individual Mask Bits"]
    #[inline]
    pub fn mi18(&mut self) -> _MI18W {
        _MI18W { w: self }
    }
    #[doc = "Bit 19 - Individual Mask Bits"]
    #[inline]
    pub fn mi19(&mut self) -> _MI19W {
        _MI19W { w: self }
    }
    #[doc = "Bit 20 - Individual Mask Bits"]
    #[inline]
    pub fn mi20(&mut self) -> _MI20W {
        _MI20W { w: self }
    }
    #[doc = "Bit 21 - Individual Mask Bits"]
    #[inline]
    pub fn mi21(&mut self) -> _MI21W {
        _MI21W { w: self }
    }
    #[doc = "Bit 22 - Individual Mask Bits"]
    #[inline]
    pub fn mi22(&mut self) -> _MI22W {
        _MI22W { w: self }
    }
    #[doc = "Bit 23 - Individual Mask Bits"]
    #[inline]
    pub fn mi23(&mut self) -> _MI23W {
        _MI23W { w: self }
    }
    #[doc = "Bit 24 - Individual Mask Bits"]
    #[inline]
    pub fn mi24(&mut self) -> _MI24W {
        _MI24W { w: self }
    }
    #[doc = "Bit 25 - Individual Mask Bits"]
    #[inline]
    pub fn mi25(&mut self) -> _MI25W {
        _MI25W { w: self }
    }
    #[doc = "Bit 26 - Individual Mask Bits"]
    #[inline]
    pub fn mi26(&mut self) -> _MI26W {
        _MI26W { w: self }
    }
    #[doc = "Bit 27 - Individual Mask Bits"]
    #[inline]
    pub fn mi27(&mut self) -> _MI27W {
        _MI27W { w: self }
    }
    #[doc = "Bit 28 - Individual Mask Bits"]
    #[inline]
    pub fn mi28(&mut self) -> _MI28W {
        _MI28W { w: self }
    }
    #[doc = "Bit 29 - Individual Mask Bits"]
    #[inline]
    pub fn mi29(&mut self) -> _MI29W {
        _MI29W { w: self }
    }
    #[doc = "Bit 30 - Individual Mask Bits"]
    #[inline]
    pub fn mi30(&mut self) -> _MI30W {
        _MI30W { w: self }
    }
    #[doc = "Bit 31 - Individual Mask Bits"]
    #[inline]
    pub fn mi31(&mut self) -> _MI31W {
        _MI31W { w: self }
    }
}
