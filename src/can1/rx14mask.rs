#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RX14MASK {
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
#[doc = "Possible values of the field `RX14M0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX14M0R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M0R {
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
            RX14M0R::_0 => false,
            RX14M0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX14M0R {
        match value {
            false => RX14M0R::_0,
            true => RX14M0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX14M0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX14M0R::_1
    }
}
#[doc = "Possible values of the field `RX14M1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX14M1R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M1R {
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
            RX14M1R::_0 => false,
            RX14M1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX14M1R {
        match value {
            false => RX14M1R::_0,
            true => RX14M1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX14M1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX14M1R::_1
    }
}
#[doc = "Possible values of the field `RX14M2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX14M2R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M2R {
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
            RX14M2R::_0 => false,
            RX14M2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX14M2R {
        match value {
            false => RX14M2R::_0,
            true => RX14M2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX14M2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX14M2R::_1
    }
}
#[doc = "Possible values of the field `RX14M3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX14M3R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M3R {
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
            RX14M3R::_0 => false,
            RX14M3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX14M3R {
        match value {
            false => RX14M3R::_0,
            true => RX14M3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX14M3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX14M3R::_1
    }
}
#[doc = "Possible values of the field `RX14M4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX14M4R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M4R {
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
            RX14M4R::_0 => false,
            RX14M4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX14M4R {
        match value {
            false => RX14M4R::_0,
            true => RX14M4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX14M4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX14M4R::_1
    }
}
#[doc = "Possible values of the field `RX14M5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX14M5R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M5R {
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
            RX14M5R::_0 => false,
            RX14M5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX14M5R {
        match value {
            false => RX14M5R::_0,
            true => RX14M5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX14M5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX14M5R::_1
    }
}
#[doc = "Possible values of the field `RX14M6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX14M6R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M6R {
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
            RX14M6R::_0 => false,
            RX14M6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX14M6R {
        match value {
            false => RX14M6R::_0,
            true => RX14M6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX14M6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX14M6R::_1
    }
}
#[doc = "Possible values of the field `RX14M7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX14M7R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M7R {
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
            RX14M7R::_0 => false,
            RX14M7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX14M7R {
        match value {
            false => RX14M7R::_0,
            true => RX14M7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX14M7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX14M7R::_1
    }
}
#[doc = "Possible values of the field `RX14M8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX14M8R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M8R {
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
            RX14M8R::_0 => false,
            RX14M8R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX14M8R {
        match value {
            false => RX14M8R::_0,
            true => RX14M8R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX14M8R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX14M8R::_1
    }
}
#[doc = "Possible values of the field `RX14M9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX14M9R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M9R {
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
            RX14M9R::_0 => false,
            RX14M9R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX14M9R {
        match value {
            false => RX14M9R::_0,
            true => RX14M9R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX14M9R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX14M9R::_1
    }
}
#[doc = "Possible values of the field `RX14M10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX14M10R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M10R {
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
            RX14M10R::_0 => false,
            RX14M10R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX14M10R {
        match value {
            false => RX14M10R::_0,
            true => RX14M10R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX14M10R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX14M10R::_1
    }
}
#[doc = "Possible values of the field `RX14M11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX14M11R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M11R {
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
            RX14M11R::_0 => false,
            RX14M11R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX14M11R {
        match value {
            false => RX14M11R::_0,
            true => RX14M11R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX14M11R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX14M11R::_1
    }
}
#[doc = "Possible values of the field `RX14M12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX14M12R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M12R {
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
            RX14M12R::_0 => false,
            RX14M12R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX14M12R {
        match value {
            false => RX14M12R::_0,
            true => RX14M12R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX14M12R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX14M12R::_1
    }
}
#[doc = "Possible values of the field `RX14M13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX14M13R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M13R {
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
            RX14M13R::_0 => false,
            RX14M13R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX14M13R {
        match value {
            false => RX14M13R::_0,
            true => RX14M13R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX14M13R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX14M13R::_1
    }
}
#[doc = "Possible values of the field `RX14M14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX14M14R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M14R {
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
            RX14M14R::_0 => false,
            RX14M14R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX14M14R {
        match value {
            false => RX14M14R::_0,
            true => RX14M14R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX14M14R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX14M14R::_1
    }
}
#[doc = "Possible values of the field `RX14M15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX14M15R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M15R {
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
            RX14M15R::_0 => false,
            RX14M15R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX14M15R {
        match value {
            false => RX14M15R::_0,
            true => RX14M15R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX14M15R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX14M15R::_1
    }
}
#[doc = "Possible values of the field `RX14M16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX14M16R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M16R {
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
            RX14M16R::_0 => false,
            RX14M16R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX14M16R {
        match value {
            false => RX14M16R::_0,
            true => RX14M16R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX14M16R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX14M16R::_1
    }
}
#[doc = "Possible values of the field `RX14M17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX14M17R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M17R {
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
            RX14M17R::_0 => false,
            RX14M17R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX14M17R {
        match value {
            false => RX14M17R::_0,
            true => RX14M17R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX14M17R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX14M17R::_1
    }
}
#[doc = "Possible values of the field `RX14M18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX14M18R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M18R {
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
            RX14M18R::_0 => false,
            RX14M18R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX14M18R {
        match value {
            false => RX14M18R::_0,
            true => RX14M18R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX14M18R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX14M18R::_1
    }
}
#[doc = "Possible values of the field `RX14M19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX14M19R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M19R {
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
            RX14M19R::_0 => false,
            RX14M19R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX14M19R {
        match value {
            false => RX14M19R::_0,
            true => RX14M19R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX14M19R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX14M19R::_1
    }
}
#[doc = "Possible values of the field `RX14M20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX14M20R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M20R {
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
            RX14M20R::_0 => false,
            RX14M20R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX14M20R {
        match value {
            false => RX14M20R::_0,
            true => RX14M20R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX14M20R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX14M20R::_1
    }
}
#[doc = "Possible values of the field `RX14M21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX14M21R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M21R {
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
            RX14M21R::_0 => false,
            RX14M21R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX14M21R {
        match value {
            false => RX14M21R::_0,
            true => RX14M21R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX14M21R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX14M21R::_1
    }
}
#[doc = "Possible values of the field `RX14M22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX14M22R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M22R {
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
            RX14M22R::_0 => false,
            RX14M22R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX14M22R {
        match value {
            false => RX14M22R::_0,
            true => RX14M22R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX14M22R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX14M22R::_1
    }
}
#[doc = "Possible values of the field `RX14M23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX14M23R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M23R {
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
            RX14M23R::_0 => false,
            RX14M23R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX14M23R {
        match value {
            false => RX14M23R::_0,
            true => RX14M23R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX14M23R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX14M23R::_1
    }
}
#[doc = "Possible values of the field `RX14M24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX14M24R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M24R {
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
            RX14M24R::_0 => false,
            RX14M24R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX14M24R {
        match value {
            false => RX14M24R::_0,
            true => RX14M24R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX14M24R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX14M24R::_1
    }
}
#[doc = "Possible values of the field `RX14M25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX14M25R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M25R {
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
            RX14M25R::_0 => false,
            RX14M25R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX14M25R {
        match value {
            false => RX14M25R::_0,
            true => RX14M25R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX14M25R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX14M25R::_1
    }
}
#[doc = "Possible values of the field `RX14M26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX14M26R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M26R {
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
            RX14M26R::_0 => false,
            RX14M26R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX14M26R {
        match value {
            false => RX14M26R::_0,
            true => RX14M26R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX14M26R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX14M26R::_1
    }
}
#[doc = "Possible values of the field `RX14M27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX14M27R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M27R {
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
            RX14M27R::_0 => false,
            RX14M27R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX14M27R {
        match value {
            false => RX14M27R::_0,
            true => RX14M27R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX14M27R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX14M27R::_1
    }
}
#[doc = "Possible values of the field `RX14M28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX14M28R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M28R {
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
            RX14M28R::_0 => false,
            RX14M28R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX14M28R {
        match value {
            false => RX14M28R::_0,
            true => RX14M28R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX14M28R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX14M28R::_1
    }
}
#[doc = "Possible values of the field `RX14M29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX14M29R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M29R {
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
            RX14M29R::_0 => false,
            RX14M29R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX14M29R {
        match value {
            false => RX14M29R::_0,
            true => RX14M29R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX14M29R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX14M29R::_1
    }
}
#[doc = "Possible values of the field `RX14M30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX14M30R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M30R {
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
            RX14M30R::_0 => false,
            RX14M30R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX14M30R {
        match value {
            false => RX14M30R::_0,
            true => RX14M30R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX14M30R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX14M30R::_1
    }
}
#[doc = "Possible values of the field `RX14M31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX14M31R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M31R {
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
            RX14M31R::_0 => false,
            RX14M31R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX14M31R {
        match value {
            false => RX14M31R::_0,
            true => RX14M31R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX14M31R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX14M31R::_1
    }
}
#[doc = "Values that can be written to the field `RX14M0`"]
pub enum RX14M0W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX14M0W::_0 => false,
            RX14M0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX14M0W<'a> {
    w: &'a mut W,
}
impl<'a> _RX14M0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX14M0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M0W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M0W::_1)
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
#[doc = "Values that can be written to the field `RX14M1`"]
pub enum RX14M1W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX14M1W::_0 => false,
            RX14M1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX14M1W<'a> {
    w: &'a mut W,
}
impl<'a> _RX14M1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX14M1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M1W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M1W::_1)
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
#[doc = "Values that can be written to the field `RX14M2`"]
pub enum RX14M2W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX14M2W::_0 => false,
            RX14M2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX14M2W<'a> {
    w: &'a mut W,
}
impl<'a> _RX14M2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX14M2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M2W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M2W::_1)
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
#[doc = "Values that can be written to the field `RX14M3`"]
pub enum RX14M3W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX14M3W::_0 => false,
            RX14M3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX14M3W<'a> {
    w: &'a mut W,
}
impl<'a> _RX14M3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX14M3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M3W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M3W::_1)
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
#[doc = "Values that can be written to the field `RX14M4`"]
pub enum RX14M4W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX14M4W::_0 => false,
            RX14M4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX14M4W<'a> {
    w: &'a mut W,
}
impl<'a> _RX14M4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX14M4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M4W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M4W::_1)
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
#[doc = "Values that can be written to the field `RX14M5`"]
pub enum RX14M5W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX14M5W::_0 => false,
            RX14M5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX14M5W<'a> {
    w: &'a mut W,
}
impl<'a> _RX14M5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX14M5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M5W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M5W::_1)
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
#[doc = "Values that can be written to the field `RX14M6`"]
pub enum RX14M6W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX14M6W::_0 => false,
            RX14M6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX14M6W<'a> {
    w: &'a mut W,
}
impl<'a> _RX14M6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX14M6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M6W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M6W::_1)
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
#[doc = "Values that can be written to the field `RX14M7`"]
pub enum RX14M7W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX14M7W::_0 => false,
            RX14M7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX14M7W<'a> {
    w: &'a mut W,
}
impl<'a> _RX14M7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX14M7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M7W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M7W::_1)
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
#[doc = "Values that can be written to the field `RX14M8`"]
pub enum RX14M8W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX14M8W::_0 => false,
            RX14M8W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX14M8W<'a> {
    w: &'a mut W,
}
impl<'a> _RX14M8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX14M8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M8W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M8W::_1)
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
#[doc = "Values that can be written to the field `RX14M9`"]
pub enum RX14M9W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX14M9W::_0 => false,
            RX14M9W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX14M9W<'a> {
    w: &'a mut W,
}
impl<'a> _RX14M9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX14M9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M9W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M9W::_1)
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
#[doc = "Values that can be written to the field `RX14M10`"]
pub enum RX14M10W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX14M10W::_0 => false,
            RX14M10W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX14M10W<'a> {
    w: &'a mut W,
}
impl<'a> _RX14M10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX14M10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M10W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M10W::_1)
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
#[doc = "Values that can be written to the field `RX14M11`"]
pub enum RX14M11W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX14M11W::_0 => false,
            RX14M11W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX14M11W<'a> {
    w: &'a mut W,
}
impl<'a> _RX14M11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX14M11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M11W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M11W::_1)
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
#[doc = "Values that can be written to the field `RX14M12`"]
pub enum RX14M12W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX14M12W::_0 => false,
            RX14M12W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX14M12W<'a> {
    w: &'a mut W,
}
impl<'a> _RX14M12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX14M12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M12W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M12W::_1)
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
#[doc = "Values that can be written to the field `RX14M13`"]
pub enum RX14M13W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX14M13W::_0 => false,
            RX14M13W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX14M13W<'a> {
    w: &'a mut W,
}
impl<'a> _RX14M13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX14M13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M13W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M13W::_1)
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
#[doc = "Values that can be written to the field `RX14M14`"]
pub enum RX14M14W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX14M14W::_0 => false,
            RX14M14W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX14M14W<'a> {
    w: &'a mut W,
}
impl<'a> _RX14M14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX14M14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M14W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M14W::_1)
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
#[doc = "Values that can be written to the field `RX14M15`"]
pub enum RX14M15W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX14M15W::_0 => false,
            RX14M15W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX14M15W<'a> {
    w: &'a mut W,
}
impl<'a> _RX14M15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX14M15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M15W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M15W::_1)
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
#[doc = "Values that can be written to the field `RX14M16`"]
pub enum RX14M16W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX14M16W::_0 => false,
            RX14M16W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX14M16W<'a> {
    w: &'a mut W,
}
impl<'a> _RX14M16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX14M16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M16W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M16W::_1)
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
#[doc = "Values that can be written to the field `RX14M17`"]
pub enum RX14M17W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX14M17W::_0 => false,
            RX14M17W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX14M17W<'a> {
    w: &'a mut W,
}
impl<'a> _RX14M17W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX14M17W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M17W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M17W::_1)
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
#[doc = "Values that can be written to the field `RX14M18`"]
pub enum RX14M18W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX14M18W::_0 => false,
            RX14M18W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX14M18W<'a> {
    w: &'a mut W,
}
impl<'a> _RX14M18W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX14M18W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M18W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M18W::_1)
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
#[doc = "Values that can be written to the field `RX14M19`"]
pub enum RX14M19W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX14M19W::_0 => false,
            RX14M19W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX14M19W<'a> {
    w: &'a mut W,
}
impl<'a> _RX14M19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX14M19W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M19W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M19W::_1)
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
#[doc = "Values that can be written to the field `RX14M20`"]
pub enum RX14M20W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX14M20W::_0 => false,
            RX14M20W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX14M20W<'a> {
    w: &'a mut W,
}
impl<'a> _RX14M20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX14M20W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M20W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M20W::_1)
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
#[doc = "Values that can be written to the field `RX14M21`"]
pub enum RX14M21W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX14M21W::_0 => false,
            RX14M21W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX14M21W<'a> {
    w: &'a mut W,
}
impl<'a> _RX14M21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX14M21W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M21W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M21W::_1)
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
#[doc = "Values that can be written to the field `RX14M22`"]
pub enum RX14M22W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX14M22W::_0 => false,
            RX14M22W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX14M22W<'a> {
    w: &'a mut W,
}
impl<'a> _RX14M22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX14M22W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M22W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M22W::_1)
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
#[doc = "Values that can be written to the field `RX14M23`"]
pub enum RX14M23W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX14M23W::_0 => false,
            RX14M23W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX14M23W<'a> {
    w: &'a mut W,
}
impl<'a> _RX14M23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX14M23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M23W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M23W::_1)
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
#[doc = "Values that can be written to the field `RX14M24`"]
pub enum RX14M24W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX14M24W::_0 => false,
            RX14M24W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX14M24W<'a> {
    w: &'a mut W,
}
impl<'a> _RX14M24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX14M24W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M24W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M24W::_1)
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
#[doc = "Values that can be written to the field `RX14M25`"]
pub enum RX14M25W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX14M25W::_0 => false,
            RX14M25W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX14M25W<'a> {
    w: &'a mut W,
}
impl<'a> _RX14M25W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX14M25W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M25W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M25W::_1)
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
#[doc = "Values that can be written to the field `RX14M26`"]
pub enum RX14M26W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX14M26W::_0 => false,
            RX14M26W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX14M26W<'a> {
    w: &'a mut W,
}
impl<'a> _RX14M26W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX14M26W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M26W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M26W::_1)
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
#[doc = "Values that can be written to the field `RX14M27`"]
pub enum RX14M27W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX14M27W::_0 => false,
            RX14M27W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX14M27W<'a> {
    w: &'a mut W,
}
impl<'a> _RX14M27W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX14M27W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M27W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M27W::_1)
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
#[doc = "Values that can be written to the field `RX14M28`"]
pub enum RX14M28W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX14M28W::_0 => false,
            RX14M28W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX14M28W<'a> {
    w: &'a mut W,
}
impl<'a> _RX14M28W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX14M28W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M28W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M28W::_1)
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
#[doc = "Values that can be written to the field `RX14M29`"]
pub enum RX14M29W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX14M29W::_0 => false,
            RX14M29W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX14M29W<'a> {
    w: &'a mut W,
}
impl<'a> _RX14M29W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX14M29W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M29W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M29W::_1)
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
#[doc = "Values that can be written to the field `RX14M30`"]
pub enum RX14M30W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX14M30W::_0 => false,
            RX14M30W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX14M30W<'a> {
    w: &'a mut W,
}
impl<'a> _RX14M30W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX14M30W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M30W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M30W::_1)
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
#[doc = "Values that can be written to the field `RX14M31`"]
pub enum RX14M31W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX14M31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX14M31W::_0 => false,
            RX14M31W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX14M31W<'a> {
    w: &'a mut W,
}
impl<'a> _RX14M31W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX14M31W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M31W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M31W::_1)
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
    #[doc = "Bit 0 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m0(&self) -> RX14M0R {
        RX14M0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m1(&self) -> RX14M1R {
        RX14M1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m2(&self) -> RX14M2R {
        RX14M2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m3(&self) -> RX14M3R {
        RX14M3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m4(&self) -> RX14M4R {
        RX14M4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m5(&self) -> RX14M5R {
        RX14M5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m6(&self) -> RX14M6R {
        RX14M6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m7(&self) -> RX14M7R {
        RX14M7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m8(&self) -> RX14M8R {
        RX14M8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m9(&self) -> RX14M9R {
        RX14M9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m10(&self) -> RX14M10R {
        RX14M10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m11(&self) -> RX14M11R {
        RX14M11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m12(&self) -> RX14M12R {
        RX14M12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m13(&self) -> RX14M13R {
        RX14M13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m14(&self) -> RX14M14R {
        RX14M14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m15(&self) -> RX14M15R {
        RX14M15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m16(&self) -> RX14M16R {
        RX14M16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m17(&self) -> RX14M17R {
        RX14M17R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m18(&self) -> RX14M18R {
        RX14M18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m19(&self) -> RX14M19R {
        RX14M19R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m20(&self) -> RX14M20R {
        RX14M20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m21(&self) -> RX14M21R {
        RX14M21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m22(&self) -> RX14M22R {
        RX14M22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m23(&self) -> RX14M23R {
        RX14M23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m24(&self) -> RX14M24R {
        RX14M24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m25(&self) -> RX14M25R {
        RX14M25R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m26(&self) -> RX14M26R {
        RX14M26R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m27(&self) -> RX14M27R {
        RX14M27R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m28(&self) -> RX14M28R {
        RX14M28R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m29(&self) -> RX14M29R {
        RX14M29R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m30(&self) -> RX14M30R {
        RX14M30R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m31(&self) -> RX14M31R {
        RX14M31R::_from({
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
    #[doc = "Bit 0 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m0(&mut self) -> _RX14M0W {
        _RX14M0W { w: self }
    }
    #[doc = "Bit 1 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m1(&mut self) -> _RX14M1W {
        _RX14M1W { w: self }
    }
    #[doc = "Bit 2 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m2(&mut self) -> _RX14M2W {
        _RX14M2W { w: self }
    }
    #[doc = "Bit 3 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m3(&mut self) -> _RX14M3W {
        _RX14M3W { w: self }
    }
    #[doc = "Bit 4 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m4(&mut self) -> _RX14M4W {
        _RX14M4W { w: self }
    }
    #[doc = "Bit 5 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m5(&mut self) -> _RX14M5W {
        _RX14M5W { w: self }
    }
    #[doc = "Bit 6 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m6(&mut self) -> _RX14M6W {
        _RX14M6W { w: self }
    }
    #[doc = "Bit 7 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m7(&mut self) -> _RX14M7W {
        _RX14M7W { w: self }
    }
    #[doc = "Bit 8 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m8(&mut self) -> _RX14M8W {
        _RX14M8W { w: self }
    }
    #[doc = "Bit 9 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m9(&mut self) -> _RX14M9W {
        _RX14M9W { w: self }
    }
    #[doc = "Bit 10 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m10(&mut self) -> _RX14M10W {
        _RX14M10W { w: self }
    }
    #[doc = "Bit 11 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m11(&mut self) -> _RX14M11W {
        _RX14M11W { w: self }
    }
    #[doc = "Bit 12 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m12(&mut self) -> _RX14M12W {
        _RX14M12W { w: self }
    }
    #[doc = "Bit 13 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m13(&mut self) -> _RX14M13W {
        _RX14M13W { w: self }
    }
    #[doc = "Bit 14 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m14(&mut self) -> _RX14M14W {
        _RX14M14W { w: self }
    }
    #[doc = "Bit 15 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m15(&mut self) -> _RX14M15W {
        _RX14M15W { w: self }
    }
    #[doc = "Bit 16 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m16(&mut self) -> _RX14M16W {
        _RX14M16W { w: self }
    }
    #[doc = "Bit 17 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m17(&mut self) -> _RX14M17W {
        _RX14M17W { w: self }
    }
    #[doc = "Bit 18 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m18(&mut self) -> _RX14M18W {
        _RX14M18W { w: self }
    }
    #[doc = "Bit 19 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m19(&mut self) -> _RX14M19W {
        _RX14M19W { w: self }
    }
    #[doc = "Bit 20 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m20(&mut self) -> _RX14M20W {
        _RX14M20W { w: self }
    }
    #[doc = "Bit 21 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m21(&mut self) -> _RX14M21W {
        _RX14M21W { w: self }
    }
    #[doc = "Bit 22 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m22(&mut self) -> _RX14M22W {
        _RX14M22W { w: self }
    }
    #[doc = "Bit 23 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m23(&mut self) -> _RX14M23W {
        _RX14M23W { w: self }
    }
    #[doc = "Bit 24 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m24(&mut self) -> _RX14M24W {
        _RX14M24W { w: self }
    }
    #[doc = "Bit 25 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m25(&mut self) -> _RX14M25W {
        _RX14M25W { w: self }
    }
    #[doc = "Bit 26 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m26(&mut self) -> _RX14M26W {
        _RX14M26W { w: self }
    }
    #[doc = "Bit 27 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m27(&mut self) -> _RX14M27W {
        _RX14M27W { w: self }
    }
    #[doc = "Bit 28 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m28(&mut self) -> _RX14M28W {
        _RX14M28W { w: self }
    }
    #[doc = "Bit 29 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m29(&mut self) -> _RX14M29W {
        _RX14M29W { w: self }
    }
    #[doc = "Bit 30 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m30(&mut self) -> _RX14M30W {
        _RX14M30W { w: self }
    }
    #[doc = "Bit 31 - Rx Buffer 14 Mask Bits"]
    #[inline]
    pub fn rx14m31(&mut self) -> _RX14M31W {
        _RX14M31W { w: self }
    }
}
