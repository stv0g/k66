#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RXMGMASK {
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
#[doc = "Possible values of the field `MG0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG0R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG0R {
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
            MG0R::_0 => false,
            MG0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MG0R {
        match value {
            false => MG0R::_0,
            true => MG0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MG0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MG0R::_1
    }
}
#[doc = "Possible values of the field `MG1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG1R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG1R {
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
            MG1R::_0 => false,
            MG1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MG1R {
        match value {
            false => MG1R::_0,
            true => MG1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MG1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MG1R::_1
    }
}
#[doc = "Possible values of the field `MG2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG2R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG2R {
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
            MG2R::_0 => false,
            MG2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MG2R {
        match value {
            false => MG2R::_0,
            true => MG2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MG2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MG2R::_1
    }
}
#[doc = "Possible values of the field `MG3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG3R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG3R {
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
            MG3R::_0 => false,
            MG3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MG3R {
        match value {
            false => MG3R::_0,
            true => MG3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MG3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MG3R::_1
    }
}
#[doc = "Possible values of the field `MG4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG4R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG4R {
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
            MG4R::_0 => false,
            MG4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MG4R {
        match value {
            false => MG4R::_0,
            true => MG4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MG4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MG4R::_1
    }
}
#[doc = "Possible values of the field `MG5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG5R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG5R {
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
            MG5R::_0 => false,
            MG5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MG5R {
        match value {
            false => MG5R::_0,
            true => MG5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MG5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MG5R::_1
    }
}
#[doc = "Possible values of the field `MG6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG6R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG6R {
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
            MG6R::_0 => false,
            MG6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MG6R {
        match value {
            false => MG6R::_0,
            true => MG6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MG6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MG6R::_1
    }
}
#[doc = "Possible values of the field `MG7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG7R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG7R {
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
            MG7R::_0 => false,
            MG7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MG7R {
        match value {
            false => MG7R::_0,
            true => MG7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MG7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MG7R::_1
    }
}
#[doc = "Possible values of the field `MG8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG8R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG8R {
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
            MG8R::_0 => false,
            MG8R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MG8R {
        match value {
            false => MG8R::_0,
            true => MG8R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MG8R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MG8R::_1
    }
}
#[doc = "Possible values of the field `MG9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG9R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG9R {
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
            MG9R::_0 => false,
            MG9R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MG9R {
        match value {
            false => MG9R::_0,
            true => MG9R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MG9R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MG9R::_1
    }
}
#[doc = "Possible values of the field `MG10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG10R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG10R {
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
            MG10R::_0 => false,
            MG10R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MG10R {
        match value {
            false => MG10R::_0,
            true => MG10R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MG10R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MG10R::_1
    }
}
#[doc = "Possible values of the field `MG11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG11R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG11R {
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
            MG11R::_0 => false,
            MG11R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MG11R {
        match value {
            false => MG11R::_0,
            true => MG11R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MG11R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MG11R::_1
    }
}
#[doc = "Possible values of the field `MG12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG12R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG12R {
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
            MG12R::_0 => false,
            MG12R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MG12R {
        match value {
            false => MG12R::_0,
            true => MG12R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MG12R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MG12R::_1
    }
}
#[doc = "Possible values of the field `MG13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG13R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG13R {
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
            MG13R::_0 => false,
            MG13R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MG13R {
        match value {
            false => MG13R::_0,
            true => MG13R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MG13R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MG13R::_1
    }
}
#[doc = "Possible values of the field `MG14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG14R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG14R {
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
            MG14R::_0 => false,
            MG14R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MG14R {
        match value {
            false => MG14R::_0,
            true => MG14R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MG14R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MG14R::_1
    }
}
#[doc = "Possible values of the field `MG15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG15R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG15R {
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
            MG15R::_0 => false,
            MG15R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MG15R {
        match value {
            false => MG15R::_0,
            true => MG15R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MG15R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MG15R::_1
    }
}
#[doc = "Possible values of the field `MG16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG16R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG16R {
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
            MG16R::_0 => false,
            MG16R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MG16R {
        match value {
            false => MG16R::_0,
            true => MG16R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MG16R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MG16R::_1
    }
}
#[doc = "Possible values of the field `MG17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG17R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG17R {
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
            MG17R::_0 => false,
            MG17R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MG17R {
        match value {
            false => MG17R::_0,
            true => MG17R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MG17R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MG17R::_1
    }
}
#[doc = "Possible values of the field `MG18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG18R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG18R {
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
            MG18R::_0 => false,
            MG18R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MG18R {
        match value {
            false => MG18R::_0,
            true => MG18R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MG18R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MG18R::_1
    }
}
#[doc = "Possible values of the field `MG19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG19R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG19R {
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
            MG19R::_0 => false,
            MG19R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MG19R {
        match value {
            false => MG19R::_0,
            true => MG19R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MG19R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MG19R::_1
    }
}
#[doc = "Possible values of the field `MG20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG20R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG20R {
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
            MG20R::_0 => false,
            MG20R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MG20R {
        match value {
            false => MG20R::_0,
            true => MG20R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MG20R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MG20R::_1
    }
}
#[doc = "Possible values of the field `MG21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG21R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG21R {
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
            MG21R::_0 => false,
            MG21R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MG21R {
        match value {
            false => MG21R::_0,
            true => MG21R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MG21R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MG21R::_1
    }
}
#[doc = "Possible values of the field `MG22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG22R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG22R {
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
            MG22R::_0 => false,
            MG22R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MG22R {
        match value {
            false => MG22R::_0,
            true => MG22R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MG22R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MG22R::_1
    }
}
#[doc = "Possible values of the field `MG23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG23R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG23R {
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
            MG23R::_0 => false,
            MG23R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MG23R {
        match value {
            false => MG23R::_0,
            true => MG23R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MG23R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MG23R::_1
    }
}
#[doc = "Possible values of the field `MG24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG24R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG24R {
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
            MG24R::_0 => false,
            MG24R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MG24R {
        match value {
            false => MG24R::_0,
            true => MG24R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MG24R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MG24R::_1
    }
}
#[doc = "Possible values of the field `MG25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG25R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG25R {
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
            MG25R::_0 => false,
            MG25R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MG25R {
        match value {
            false => MG25R::_0,
            true => MG25R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MG25R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MG25R::_1
    }
}
#[doc = "Possible values of the field `MG26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG26R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG26R {
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
            MG26R::_0 => false,
            MG26R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MG26R {
        match value {
            false => MG26R::_0,
            true => MG26R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MG26R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MG26R::_1
    }
}
#[doc = "Possible values of the field `MG27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG27R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG27R {
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
            MG27R::_0 => false,
            MG27R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MG27R {
        match value {
            false => MG27R::_0,
            true => MG27R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MG27R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MG27R::_1
    }
}
#[doc = "Possible values of the field `MG28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG28R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG28R {
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
            MG28R::_0 => false,
            MG28R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MG28R {
        match value {
            false => MG28R::_0,
            true => MG28R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MG28R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MG28R::_1
    }
}
#[doc = "Possible values of the field `MG29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG29R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG29R {
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
            MG29R::_0 => false,
            MG29R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MG29R {
        match value {
            false => MG29R::_0,
            true => MG29R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MG29R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MG29R::_1
    }
}
#[doc = "Possible values of the field `MG30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG30R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG30R {
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
            MG30R::_0 => false,
            MG30R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MG30R {
        match value {
            false => MG30R::_0,
            true => MG30R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MG30R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MG30R::_1
    }
}
#[doc = "Possible values of the field `MG31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MG31R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG31R {
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
            MG31R::_0 => false,
            MG31R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MG31R {
        match value {
            false => MG31R::_0,
            true => MG31R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MG31R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MG31R::_1
    }
}
#[doc = "Values that can be written to the field `MG0`"]
pub enum MG0W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MG0W::_0 => false,
            MG0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MG0W<'a> {
    w: &'a mut W,
}
impl<'a> _MG0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MG0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG0W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG0W::_1)
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
#[doc = "Values that can be written to the field `MG1`"]
pub enum MG1W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MG1W::_0 => false,
            MG1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MG1W<'a> {
    w: &'a mut W,
}
impl<'a> _MG1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MG1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG1W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG1W::_1)
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
#[doc = "Values that can be written to the field `MG2`"]
pub enum MG2W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MG2W::_0 => false,
            MG2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MG2W<'a> {
    w: &'a mut W,
}
impl<'a> _MG2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MG2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG2W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG2W::_1)
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
#[doc = "Values that can be written to the field `MG3`"]
pub enum MG3W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MG3W::_0 => false,
            MG3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MG3W<'a> {
    w: &'a mut W,
}
impl<'a> _MG3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MG3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG3W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG3W::_1)
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
#[doc = "Values that can be written to the field `MG4`"]
pub enum MG4W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MG4W::_0 => false,
            MG4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MG4W<'a> {
    w: &'a mut W,
}
impl<'a> _MG4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MG4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG4W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG4W::_1)
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
#[doc = "Values that can be written to the field `MG5`"]
pub enum MG5W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MG5W::_0 => false,
            MG5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MG5W<'a> {
    w: &'a mut W,
}
impl<'a> _MG5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MG5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG5W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG5W::_1)
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
#[doc = "Values that can be written to the field `MG6`"]
pub enum MG6W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MG6W::_0 => false,
            MG6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MG6W<'a> {
    w: &'a mut W,
}
impl<'a> _MG6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MG6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG6W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG6W::_1)
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
#[doc = "Values that can be written to the field `MG7`"]
pub enum MG7W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MG7W::_0 => false,
            MG7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MG7W<'a> {
    w: &'a mut W,
}
impl<'a> _MG7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MG7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG7W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG7W::_1)
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
#[doc = "Values that can be written to the field `MG8`"]
pub enum MG8W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MG8W::_0 => false,
            MG8W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MG8W<'a> {
    w: &'a mut W,
}
impl<'a> _MG8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MG8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG8W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG8W::_1)
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
#[doc = "Values that can be written to the field `MG9`"]
pub enum MG9W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MG9W::_0 => false,
            MG9W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MG9W<'a> {
    w: &'a mut W,
}
impl<'a> _MG9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MG9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG9W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG9W::_1)
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
#[doc = "Values that can be written to the field `MG10`"]
pub enum MG10W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MG10W::_0 => false,
            MG10W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MG10W<'a> {
    w: &'a mut W,
}
impl<'a> _MG10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MG10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG10W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG10W::_1)
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
#[doc = "Values that can be written to the field `MG11`"]
pub enum MG11W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MG11W::_0 => false,
            MG11W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MG11W<'a> {
    w: &'a mut W,
}
impl<'a> _MG11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MG11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG11W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG11W::_1)
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
#[doc = "Values that can be written to the field `MG12`"]
pub enum MG12W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MG12W::_0 => false,
            MG12W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MG12W<'a> {
    w: &'a mut W,
}
impl<'a> _MG12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MG12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG12W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG12W::_1)
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
#[doc = "Values that can be written to the field `MG13`"]
pub enum MG13W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MG13W::_0 => false,
            MG13W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MG13W<'a> {
    w: &'a mut W,
}
impl<'a> _MG13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MG13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG13W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG13W::_1)
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
#[doc = "Values that can be written to the field `MG14`"]
pub enum MG14W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MG14W::_0 => false,
            MG14W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MG14W<'a> {
    w: &'a mut W,
}
impl<'a> _MG14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MG14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG14W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG14W::_1)
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
#[doc = "Values that can be written to the field `MG15`"]
pub enum MG15W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MG15W::_0 => false,
            MG15W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MG15W<'a> {
    w: &'a mut W,
}
impl<'a> _MG15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MG15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG15W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG15W::_1)
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
#[doc = "Values that can be written to the field `MG16`"]
pub enum MG16W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MG16W::_0 => false,
            MG16W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MG16W<'a> {
    w: &'a mut W,
}
impl<'a> _MG16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MG16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG16W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG16W::_1)
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
#[doc = "Values that can be written to the field `MG17`"]
pub enum MG17W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MG17W::_0 => false,
            MG17W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MG17W<'a> {
    w: &'a mut W,
}
impl<'a> _MG17W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MG17W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG17W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG17W::_1)
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
#[doc = "Values that can be written to the field `MG18`"]
pub enum MG18W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MG18W::_0 => false,
            MG18W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MG18W<'a> {
    w: &'a mut W,
}
impl<'a> _MG18W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MG18W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG18W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG18W::_1)
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
#[doc = "Values that can be written to the field `MG19`"]
pub enum MG19W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MG19W::_0 => false,
            MG19W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MG19W<'a> {
    w: &'a mut W,
}
impl<'a> _MG19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MG19W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG19W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG19W::_1)
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
#[doc = "Values that can be written to the field `MG20`"]
pub enum MG20W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MG20W::_0 => false,
            MG20W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MG20W<'a> {
    w: &'a mut W,
}
impl<'a> _MG20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MG20W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG20W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG20W::_1)
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
#[doc = "Values that can be written to the field `MG21`"]
pub enum MG21W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MG21W::_0 => false,
            MG21W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MG21W<'a> {
    w: &'a mut W,
}
impl<'a> _MG21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MG21W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG21W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG21W::_1)
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
#[doc = "Values that can be written to the field `MG22`"]
pub enum MG22W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MG22W::_0 => false,
            MG22W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MG22W<'a> {
    w: &'a mut W,
}
impl<'a> _MG22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MG22W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG22W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG22W::_1)
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
#[doc = "Values that can be written to the field `MG23`"]
pub enum MG23W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MG23W::_0 => false,
            MG23W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MG23W<'a> {
    w: &'a mut W,
}
impl<'a> _MG23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MG23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG23W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG23W::_1)
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
#[doc = "Values that can be written to the field `MG24`"]
pub enum MG24W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MG24W::_0 => false,
            MG24W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MG24W<'a> {
    w: &'a mut W,
}
impl<'a> _MG24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MG24W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG24W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG24W::_1)
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
#[doc = "Values that can be written to the field `MG25`"]
pub enum MG25W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MG25W::_0 => false,
            MG25W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MG25W<'a> {
    w: &'a mut W,
}
impl<'a> _MG25W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MG25W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG25W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG25W::_1)
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
#[doc = "Values that can be written to the field `MG26`"]
pub enum MG26W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MG26W::_0 => false,
            MG26W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MG26W<'a> {
    w: &'a mut W,
}
impl<'a> _MG26W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MG26W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG26W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG26W::_1)
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
#[doc = "Values that can be written to the field `MG27`"]
pub enum MG27W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MG27W::_0 => false,
            MG27W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MG27W<'a> {
    w: &'a mut W,
}
impl<'a> _MG27W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MG27W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG27W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG27W::_1)
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
#[doc = "Values that can be written to the field `MG28`"]
pub enum MG28W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MG28W::_0 => false,
            MG28W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MG28W<'a> {
    w: &'a mut W,
}
impl<'a> _MG28W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MG28W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG28W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG28W::_1)
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
#[doc = "Values that can be written to the field `MG29`"]
pub enum MG29W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MG29W::_0 => false,
            MG29W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MG29W<'a> {
    w: &'a mut W,
}
impl<'a> _MG29W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MG29W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG29W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG29W::_1)
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
#[doc = "Values that can be written to the field `MG30`"]
pub enum MG30W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MG30W::_0 => false,
            MG30W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MG30W<'a> {
    w: &'a mut W,
}
impl<'a> _MG30W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MG30W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG30W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG30W::_1)
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
#[doc = "Values that can be written to the field `MG31`"]
pub enum MG31W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl MG31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MG31W::_0 => false,
            MG31W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MG31W<'a> {
    w: &'a mut W,
}
impl<'a> _MG31W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MG31W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG31W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG31W::_1)
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
    #[doc = "Bit 0 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg0(&self) -> MG0R {
        MG0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg1(&self) -> MG1R {
        MG1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg2(&self) -> MG2R {
        MG2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg3(&self) -> MG3R {
        MG3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg4(&self) -> MG4R {
        MG4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg5(&self) -> MG5R {
        MG5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg6(&self) -> MG6R {
        MG6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg7(&self) -> MG7R {
        MG7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg8(&self) -> MG8R {
        MG8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg9(&self) -> MG9R {
        MG9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg10(&self) -> MG10R {
        MG10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg11(&self) -> MG11R {
        MG11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg12(&self) -> MG12R {
        MG12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg13(&self) -> MG13R {
        MG13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg14(&self) -> MG14R {
        MG14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg15(&self) -> MG15R {
        MG15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg16(&self) -> MG16R {
        MG16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg17(&self) -> MG17R {
        MG17R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg18(&self) -> MG18R {
        MG18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg19(&self) -> MG19R {
        MG19R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg20(&self) -> MG20R {
        MG20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg21(&self) -> MG21R {
        MG21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg22(&self) -> MG22R {
        MG22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg23(&self) -> MG23R {
        MG23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg24(&self) -> MG24R {
        MG24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg25(&self) -> MG25R {
        MG25R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg26(&self) -> MG26R {
        MG26R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg27(&self) -> MG27R {
        MG27R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg28(&self) -> MG28R {
        MG28R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg29(&self) -> MG29R {
        MG29R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg30(&self) -> MG30R {
        MG30R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg31(&self) -> MG31R {
        MG31R::_from({
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
    #[doc = "Bit 0 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg0(&mut self) -> _MG0W {
        _MG0W { w: self }
    }
    #[doc = "Bit 1 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg1(&mut self) -> _MG1W {
        _MG1W { w: self }
    }
    #[doc = "Bit 2 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg2(&mut self) -> _MG2W {
        _MG2W { w: self }
    }
    #[doc = "Bit 3 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg3(&mut self) -> _MG3W {
        _MG3W { w: self }
    }
    #[doc = "Bit 4 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg4(&mut self) -> _MG4W {
        _MG4W { w: self }
    }
    #[doc = "Bit 5 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg5(&mut self) -> _MG5W {
        _MG5W { w: self }
    }
    #[doc = "Bit 6 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg6(&mut self) -> _MG6W {
        _MG6W { w: self }
    }
    #[doc = "Bit 7 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg7(&mut self) -> _MG7W {
        _MG7W { w: self }
    }
    #[doc = "Bit 8 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg8(&mut self) -> _MG8W {
        _MG8W { w: self }
    }
    #[doc = "Bit 9 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg9(&mut self) -> _MG9W {
        _MG9W { w: self }
    }
    #[doc = "Bit 10 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg10(&mut self) -> _MG10W {
        _MG10W { w: self }
    }
    #[doc = "Bit 11 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg11(&mut self) -> _MG11W {
        _MG11W { w: self }
    }
    #[doc = "Bit 12 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg12(&mut self) -> _MG12W {
        _MG12W { w: self }
    }
    #[doc = "Bit 13 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg13(&mut self) -> _MG13W {
        _MG13W { w: self }
    }
    #[doc = "Bit 14 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg14(&mut self) -> _MG14W {
        _MG14W { w: self }
    }
    #[doc = "Bit 15 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg15(&mut self) -> _MG15W {
        _MG15W { w: self }
    }
    #[doc = "Bit 16 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg16(&mut self) -> _MG16W {
        _MG16W { w: self }
    }
    #[doc = "Bit 17 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg17(&mut self) -> _MG17W {
        _MG17W { w: self }
    }
    #[doc = "Bit 18 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg18(&mut self) -> _MG18W {
        _MG18W { w: self }
    }
    #[doc = "Bit 19 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg19(&mut self) -> _MG19W {
        _MG19W { w: self }
    }
    #[doc = "Bit 20 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg20(&mut self) -> _MG20W {
        _MG20W { w: self }
    }
    #[doc = "Bit 21 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg21(&mut self) -> _MG21W {
        _MG21W { w: self }
    }
    #[doc = "Bit 22 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg22(&mut self) -> _MG22W {
        _MG22W { w: self }
    }
    #[doc = "Bit 23 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg23(&mut self) -> _MG23W {
        _MG23W { w: self }
    }
    #[doc = "Bit 24 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg24(&mut self) -> _MG24W {
        _MG24W { w: self }
    }
    #[doc = "Bit 25 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg25(&mut self) -> _MG25W {
        _MG25W { w: self }
    }
    #[doc = "Bit 26 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg26(&mut self) -> _MG26W {
        _MG26W { w: self }
    }
    #[doc = "Bit 27 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg27(&mut self) -> _MG27W {
        _MG27W { w: self }
    }
    #[doc = "Bit 28 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg28(&mut self) -> _MG28W {
        _MG28W { w: self }
    }
    #[doc = "Bit 29 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg29(&mut self) -> _MG29W {
        _MG29W { w: self }
    }
    #[doc = "Bit 30 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg30(&mut self) -> _MG30W {
        _MG30W { w: self }
    }
    #[doc = "Bit 31 - Rx Mailboxes Global Mask Bits"]
    #[inline]
    pub fn mg31(&mut self) -> _MG31W {
        _MG31W { w: self }
    }
}
