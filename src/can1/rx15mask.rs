#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RX15MASK {
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
#[doc = "Possible values of the field `RX15M0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M0R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M0R {
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
            RX15M0R::_0 => false,
            RX15M0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX15M0R {
        match value {
            false => RX15M0R::_0,
            true => RX15M0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX15M0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX15M0R::_1
    }
}
#[doc = "Possible values of the field `RX15M1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M1R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M1R {
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
            RX15M1R::_0 => false,
            RX15M1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX15M1R {
        match value {
            false => RX15M1R::_0,
            true => RX15M1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX15M1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX15M1R::_1
    }
}
#[doc = "Possible values of the field `RX15M2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M2R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M2R {
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
            RX15M2R::_0 => false,
            RX15M2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX15M2R {
        match value {
            false => RX15M2R::_0,
            true => RX15M2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX15M2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX15M2R::_1
    }
}
#[doc = "Possible values of the field `RX15M3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M3R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M3R {
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
            RX15M3R::_0 => false,
            RX15M3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX15M3R {
        match value {
            false => RX15M3R::_0,
            true => RX15M3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX15M3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX15M3R::_1
    }
}
#[doc = "Possible values of the field `RX15M4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M4R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M4R {
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
            RX15M4R::_0 => false,
            RX15M4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX15M4R {
        match value {
            false => RX15M4R::_0,
            true => RX15M4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX15M4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX15M4R::_1
    }
}
#[doc = "Possible values of the field `RX15M5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M5R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M5R {
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
            RX15M5R::_0 => false,
            RX15M5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX15M5R {
        match value {
            false => RX15M5R::_0,
            true => RX15M5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX15M5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX15M5R::_1
    }
}
#[doc = "Possible values of the field `RX15M6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M6R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M6R {
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
            RX15M6R::_0 => false,
            RX15M6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX15M6R {
        match value {
            false => RX15M6R::_0,
            true => RX15M6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX15M6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX15M6R::_1
    }
}
#[doc = "Possible values of the field `RX15M7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M7R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M7R {
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
            RX15M7R::_0 => false,
            RX15M7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX15M7R {
        match value {
            false => RX15M7R::_0,
            true => RX15M7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX15M7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX15M7R::_1
    }
}
#[doc = "Possible values of the field `RX15M8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M8R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M8R {
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
            RX15M8R::_0 => false,
            RX15M8R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX15M8R {
        match value {
            false => RX15M8R::_0,
            true => RX15M8R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX15M8R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX15M8R::_1
    }
}
#[doc = "Possible values of the field `RX15M9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M9R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M9R {
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
            RX15M9R::_0 => false,
            RX15M9R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX15M9R {
        match value {
            false => RX15M9R::_0,
            true => RX15M9R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX15M9R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX15M9R::_1
    }
}
#[doc = "Possible values of the field `RX15M10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M10R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M10R {
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
            RX15M10R::_0 => false,
            RX15M10R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX15M10R {
        match value {
            false => RX15M10R::_0,
            true => RX15M10R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX15M10R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX15M10R::_1
    }
}
#[doc = "Possible values of the field `RX15M11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M11R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M11R {
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
            RX15M11R::_0 => false,
            RX15M11R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX15M11R {
        match value {
            false => RX15M11R::_0,
            true => RX15M11R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX15M11R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX15M11R::_1
    }
}
#[doc = "Possible values of the field `RX15M12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M12R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M12R {
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
            RX15M12R::_0 => false,
            RX15M12R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX15M12R {
        match value {
            false => RX15M12R::_0,
            true => RX15M12R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX15M12R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX15M12R::_1
    }
}
#[doc = "Possible values of the field `RX15M13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M13R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M13R {
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
            RX15M13R::_0 => false,
            RX15M13R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX15M13R {
        match value {
            false => RX15M13R::_0,
            true => RX15M13R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX15M13R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX15M13R::_1
    }
}
#[doc = "Possible values of the field `RX15M14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M14R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M14R {
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
            RX15M14R::_0 => false,
            RX15M14R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX15M14R {
        match value {
            false => RX15M14R::_0,
            true => RX15M14R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX15M14R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX15M14R::_1
    }
}
#[doc = "Possible values of the field `RX15M15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M15R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M15R {
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
            RX15M15R::_0 => false,
            RX15M15R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX15M15R {
        match value {
            false => RX15M15R::_0,
            true => RX15M15R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX15M15R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX15M15R::_1
    }
}
#[doc = "Possible values of the field `RX15M16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M16R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M16R {
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
            RX15M16R::_0 => false,
            RX15M16R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX15M16R {
        match value {
            false => RX15M16R::_0,
            true => RX15M16R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX15M16R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX15M16R::_1
    }
}
#[doc = "Possible values of the field `RX15M17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M17R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M17R {
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
            RX15M17R::_0 => false,
            RX15M17R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX15M17R {
        match value {
            false => RX15M17R::_0,
            true => RX15M17R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX15M17R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX15M17R::_1
    }
}
#[doc = "Possible values of the field `RX15M18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M18R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M18R {
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
            RX15M18R::_0 => false,
            RX15M18R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX15M18R {
        match value {
            false => RX15M18R::_0,
            true => RX15M18R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX15M18R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX15M18R::_1
    }
}
#[doc = "Possible values of the field `RX15M19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M19R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M19R {
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
            RX15M19R::_0 => false,
            RX15M19R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX15M19R {
        match value {
            false => RX15M19R::_0,
            true => RX15M19R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX15M19R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX15M19R::_1
    }
}
#[doc = "Possible values of the field `RX15M20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M20R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M20R {
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
            RX15M20R::_0 => false,
            RX15M20R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX15M20R {
        match value {
            false => RX15M20R::_0,
            true => RX15M20R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX15M20R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX15M20R::_1
    }
}
#[doc = "Possible values of the field `RX15M21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M21R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M21R {
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
            RX15M21R::_0 => false,
            RX15M21R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX15M21R {
        match value {
            false => RX15M21R::_0,
            true => RX15M21R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX15M21R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX15M21R::_1
    }
}
#[doc = "Possible values of the field `RX15M22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M22R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M22R {
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
            RX15M22R::_0 => false,
            RX15M22R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX15M22R {
        match value {
            false => RX15M22R::_0,
            true => RX15M22R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX15M22R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX15M22R::_1
    }
}
#[doc = "Possible values of the field `RX15M23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M23R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M23R {
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
            RX15M23R::_0 => false,
            RX15M23R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX15M23R {
        match value {
            false => RX15M23R::_0,
            true => RX15M23R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX15M23R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX15M23R::_1
    }
}
#[doc = "Possible values of the field `RX15M24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M24R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M24R {
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
            RX15M24R::_0 => false,
            RX15M24R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX15M24R {
        match value {
            false => RX15M24R::_0,
            true => RX15M24R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX15M24R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX15M24R::_1
    }
}
#[doc = "Possible values of the field `RX15M25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M25R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M25R {
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
            RX15M25R::_0 => false,
            RX15M25R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX15M25R {
        match value {
            false => RX15M25R::_0,
            true => RX15M25R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX15M25R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX15M25R::_1
    }
}
#[doc = "Possible values of the field `RX15M26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M26R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M26R {
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
            RX15M26R::_0 => false,
            RX15M26R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX15M26R {
        match value {
            false => RX15M26R::_0,
            true => RX15M26R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX15M26R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX15M26R::_1
    }
}
#[doc = "Possible values of the field `RX15M27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M27R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M27R {
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
            RX15M27R::_0 => false,
            RX15M27R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX15M27R {
        match value {
            false => RX15M27R::_0,
            true => RX15M27R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX15M27R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX15M27R::_1
    }
}
#[doc = "Possible values of the field `RX15M28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M28R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M28R {
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
            RX15M28R::_0 => false,
            RX15M28R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX15M28R {
        match value {
            false => RX15M28R::_0,
            true => RX15M28R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX15M28R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX15M28R::_1
    }
}
#[doc = "Possible values of the field `RX15M29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M29R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M29R {
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
            RX15M29R::_0 => false,
            RX15M29R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX15M29R {
        match value {
            false => RX15M29R::_0,
            true => RX15M29R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX15M29R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX15M29R::_1
    }
}
#[doc = "Possible values of the field `RX15M30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M30R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M30R {
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
            RX15M30R::_0 => false,
            RX15M30R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX15M30R {
        match value {
            false => RX15M30R::_0,
            true => RX15M30R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX15M30R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX15M30R::_1
    }
}
#[doc = "Possible values of the field `RX15M31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15M31R {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M31R {
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
            RX15M31R::_0 => false,
            RX15M31R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX15M31R {
        match value {
            false => RX15M31R::_0,
            true => RX15M31R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RX15M31R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RX15M31R::_1
    }
}
#[doc = "Values that can be written to the field `RX15M0`"]
pub enum RX15M0W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX15M0W::_0 => false,
            RX15M0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX15M0W<'a> {
    w: &'a mut W,
}
impl<'a> _RX15M0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX15M0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M0W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M0W::_1)
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
#[doc = "Values that can be written to the field `RX15M1`"]
pub enum RX15M1W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX15M1W::_0 => false,
            RX15M1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX15M1W<'a> {
    w: &'a mut W,
}
impl<'a> _RX15M1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX15M1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M1W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M1W::_1)
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
#[doc = "Values that can be written to the field `RX15M2`"]
pub enum RX15M2W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX15M2W::_0 => false,
            RX15M2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX15M2W<'a> {
    w: &'a mut W,
}
impl<'a> _RX15M2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX15M2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M2W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M2W::_1)
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
#[doc = "Values that can be written to the field `RX15M3`"]
pub enum RX15M3W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX15M3W::_0 => false,
            RX15M3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX15M3W<'a> {
    w: &'a mut W,
}
impl<'a> _RX15M3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX15M3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M3W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M3W::_1)
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
#[doc = "Values that can be written to the field `RX15M4`"]
pub enum RX15M4W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX15M4W::_0 => false,
            RX15M4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX15M4W<'a> {
    w: &'a mut W,
}
impl<'a> _RX15M4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX15M4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M4W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M4W::_1)
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
#[doc = "Values that can be written to the field `RX15M5`"]
pub enum RX15M5W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX15M5W::_0 => false,
            RX15M5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX15M5W<'a> {
    w: &'a mut W,
}
impl<'a> _RX15M5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX15M5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M5W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M5W::_1)
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
#[doc = "Values that can be written to the field `RX15M6`"]
pub enum RX15M6W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX15M6W::_0 => false,
            RX15M6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX15M6W<'a> {
    w: &'a mut W,
}
impl<'a> _RX15M6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX15M6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M6W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M6W::_1)
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
#[doc = "Values that can be written to the field `RX15M7`"]
pub enum RX15M7W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX15M7W::_0 => false,
            RX15M7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX15M7W<'a> {
    w: &'a mut W,
}
impl<'a> _RX15M7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX15M7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M7W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M7W::_1)
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
#[doc = "Values that can be written to the field `RX15M8`"]
pub enum RX15M8W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX15M8W::_0 => false,
            RX15M8W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX15M8W<'a> {
    w: &'a mut W,
}
impl<'a> _RX15M8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX15M8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M8W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M8W::_1)
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
#[doc = "Values that can be written to the field `RX15M9`"]
pub enum RX15M9W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX15M9W::_0 => false,
            RX15M9W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX15M9W<'a> {
    w: &'a mut W,
}
impl<'a> _RX15M9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX15M9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M9W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M9W::_1)
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
#[doc = "Values that can be written to the field `RX15M10`"]
pub enum RX15M10W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX15M10W::_0 => false,
            RX15M10W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX15M10W<'a> {
    w: &'a mut W,
}
impl<'a> _RX15M10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX15M10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M10W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M10W::_1)
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
#[doc = "Values that can be written to the field `RX15M11`"]
pub enum RX15M11W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX15M11W::_0 => false,
            RX15M11W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX15M11W<'a> {
    w: &'a mut W,
}
impl<'a> _RX15M11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX15M11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M11W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M11W::_1)
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
#[doc = "Values that can be written to the field `RX15M12`"]
pub enum RX15M12W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX15M12W::_0 => false,
            RX15M12W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX15M12W<'a> {
    w: &'a mut W,
}
impl<'a> _RX15M12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX15M12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M12W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M12W::_1)
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
#[doc = "Values that can be written to the field `RX15M13`"]
pub enum RX15M13W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX15M13W::_0 => false,
            RX15M13W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX15M13W<'a> {
    w: &'a mut W,
}
impl<'a> _RX15M13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX15M13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M13W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M13W::_1)
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
#[doc = "Values that can be written to the field `RX15M14`"]
pub enum RX15M14W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX15M14W::_0 => false,
            RX15M14W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX15M14W<'a> {
    w: &'a mut W,
}
impl<'a> _RX15M14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX15M14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M14W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M14W::_1)
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
#[doc = "Values that can be written to the field `RX15M15`"]
pub enum RX15M15W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX15M15W::_0 => false,
            RX15M15W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX15M15W<'a> {
    w: &'a mut W,
}
impl<'a> _RX15M15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX15M15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M15W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M15W::_1)
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
#[doc = "Values that can be written to the field `RX15M16`"]
pub enum RX15M16W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX15M16W::_0 => false,
            RX15M16W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX15M16W<'a> {
    w: &'a mut W,
}
impl<'a> _RX15M16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX15M16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M16W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M16W::_1)
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
#[doc = "Values that can be written to the field `RX15M17`"]
pub enum RX15M17W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX15M17W::_0 => false,
            RX15M17W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX15M17W<'a> {
    w: &'a mut W,
}
impl<'a> _RX15M17W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX15M17W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M17W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M17W::_1)
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
#[doc = "Values that can be written to the field `RX15M18`"]
pub enum RX15M18W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX15M18W::_0 => false,
            RX15M18W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX15M18W<'a> {
    w: &'a mut W,
}
impl<'a> _RX15M18W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX15M18W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M18W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M18W::_1)
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
#[doc = "Values that can be written to the field `RX15M19`"]
pub enum RX15M19W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX15M19W::_0 => false,
            RX15M19W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX15M19W<'a> {
    w: &'a mut W,
}
impl<'a> _RX15M19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX15M19W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M19W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M19W::_1)
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
#[doc = "Values that can be written to the field `RX15M20`"]
pub enum RX15M20W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX15M20W::_0 => false,
            RX15M20W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX15M20W<'a> {
    w: &'a mut W,
}
impl<'a> _RX15M20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX15M20W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M20W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M20W::_1)
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
#[doc = "Values that can be written to the field `RX15M21`"]
pub enum RX15M21W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX15M21W::_0 => false,
            RX15M21W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX15M21W<'a> {
    w: &'a mut W,
}
impl<'a> _RX15M21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX15M21W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M21W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M21W::_1)
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
#[doc = "Values that can be written to the field `RX15M22`"]
pub enum RX15M22W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX15M22W::_0 => false,
            RX15M22W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX15M22W<'a> {
    w: &'a mut W,
}
impl<'a> _RX15M22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX15M22W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M22W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M22W::_1)
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
#[doc = "Values that can be written to the field `RX15M23`"]
pub enum RX15M23W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX15M23W::_0 => false,
            RX15M23W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX15M23W<'a> {
    w: &'a mut W,
}
impl<'a> _RX15M23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX15M23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M23W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M23W::_1)
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
#[doc = "Values that can be written to the field `RX15M24`"]
pub enum RX15M24W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX15M24W::_0 => false,
            RX15M24W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX15M24W<'a> {
    w: &'a mut W,
}
impl<'a> _RX15M24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX15M24W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M24W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M24W::_1)
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
#[doc = "Values that can be written to the field `RX15M25`"]
pub enum RX15M25W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX15M25W::_0 => false,
            RX15M25W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX15M25W<'a> {
    w: &'a mut W,
}
impl<'a> _RX15M25W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX15M25W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M25W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M25W::_1)
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
#[doc = "Values that can be written to the field `RX15M26`"]
pub enum RX15M26W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX15M26W::_0 => false,
            RX15M26W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX15M26W<'a> {
    w: &'a mut W,
}
impl<'a> _RX15M26W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX15M26W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M26W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M26W::_1)
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
#[doc = "Values that can be written to the field `RX15M27`"]
pub enum RX15M27W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX15M27W::_0 => false,
            RX15M27W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX15M27W<'a> {
    w: &'a mut W,
}
impl<'a> _RX15M27W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX15M27W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M27W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M27W::_1)
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
#[doc = "Values that can be written to the field `RX15M28`"]
pub enum RX15M28W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX15M28W::_0 => false,
            RX15M28W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX15M28W<'a> {
    w: &'a mut W,
}
impl<'a> _RX15M28W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX15M28W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M28W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M28W::_1)
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
#[doc = "Values that can be written to the field `RX15M29`"]
pub enum RX15M29W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX15M29W::_0 => false,
            RX15M29W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX15M29W<'a> {
    w: &'a mut W,
}
impl<'a> _RX15M29W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX15M29W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M29W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M29W::_1)
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
#[doc = "Values that can be written to the field `RX15M30`"]
pub enum RX15M30W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX15M30W::_0 => false,
            RX15M30W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX15M30W<'a> {
    w: &'a mut W,
}
impl<'a> _RX15M30W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX15M30W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M30W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M30W::_1)
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
#[doc = "Values that can be written to the field `RX15M31`"]
pub enum RX15M31W {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    _0,
    #[doc = "The corresponding bit in the filter is checked."]
    _1,
}
impl RX15M31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX15M31W::_0 => false,
            RX15M31W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX15M31W<'a> {
    w: &'a mut W,
}
impl<'a> _RX15M31W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX15M31W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX15M31W::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX15M31W::_1)
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
    #[doc = "Bit 0 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m0(&self) -> RX15M0R {
        RX15M0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m1(&self) -> RX15M1R {
        RX15M1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m2(&self) -> RX15M2R {
        RX15M2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m3(&self) -> RX15M3R {
        RX15M3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m4(&self) -> RX15M4R {
        RX15M4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m5(&self) -> RX15M5R {
        RX15M5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m6(&self) -> RX15M6R {
        RX15M6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m7(&self) -> RX15M7R {
        RX15M7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m8(&self) -> RX15M8R {
        RX15M8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m9(&self) -> RX15M9R {
        RX15M9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m10(&self) -> RX15M10R {
        RX15M10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m11(&self) -> RX15M11R {
        RX15M11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m12(&self) -> RX15M12R {
        RX15M12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m13(&self) -> RX15M13R {
        RX15M13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m14(&self) -> RX15M14R {
        RX15M14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m15(&self) -> RX15M15R {
        RX15M15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m16(&self) -> RX15M16R {
        RX15M16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m17(&self) -> RX15M17R {
        RX15M17R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m18(&self) -> RX15M18R {
        RX15M18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m19(&self) -> RX15M19R {
        RX15M19R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m20(&self) -> RX15M20R {
        RX15M20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m21(&self) -> RX15M21R {
        RX15M21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m22(&self) -> RX15M22R {
        RX15M22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m23(&self) -> RX15M23R {
        RX15M23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m24(&self) -> RX15M24R {
        RX15M24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m25(&self) -> RX15M25R {
        RX15M25R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m26(&self) -> RX15M26R {
        RX15M26R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m27(&self) -> RX15M27R {
        RX15M27R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m28(&self) -> RX15M28R {
        RX15M28R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m29(&self) -> RX15M29R {
        RX15M29R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m30(&self) -> RX15M30R {
        RX15M30R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m31(&self) -> RX15M31R {
        RX15M31R::_from({
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
    #[doc = "Bit 0 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m0(&mut self) -> _RX15M0W {
        _RX15M0W { w: self }
    }
    #[doc = "Bit 1 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m1(&mut self) -> _RX15M1W {
        _RX15M1W { w: self }
    }
    #[doc = "Bit 2 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m2(&mut self) -> _RX15M2W {
        _RX15M2W { w: self }
    }
    #[doc = "Bit 3 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m3(&mut self) -> _RX15M3W {
        _RX15M3W { w: self }
    }
    #[doc = "Bit 4 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m4(&mut self) -> _RX15M4W {
        _RX15M4W { w: self }
    }
    #[doc = "Bit 5 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m5(&mut self) -> _RX15M5W {
        _RX15M5W { w: self }
    }
    #[doc = "Bit 6 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m6(&mut self) -> _RX15M6W {
        _RX15M6W { w: self }
    }
    #[doc = "Bit 7 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m7(&mut self) -> _RX15M7W {
        _RX15M7W { w: self }
    }
    #[doc = "Bit 8 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m8(&mut self) -> _RX15M8W {
        _RX15M8W { w: self }
    }
    #[doc = "Bit 9 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m9(&mut self) -> _RX15M9W {
        _RX15M9W { w: self }
    }
    #[doc = "Bit 10 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m10(&mut self) -> _RX15M10W {
        _RX15M10W { w: self }
    }
    #[doc = "Bit 11 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m11(&mut self) -> _RX15M11W {
        _RX15M11W { w: self }
    }
    #[doc = "Bit 12 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m12(&mut self) -> _RX15M12W {
        _RX15M12W { w: self }
    }
    #[doc = "Bit 13 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m13(&mut self) -> _RX15M13W {
        _RX15M13W { w: self }
    }
    #[doc = "Bit 14 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m14(&mut self) -> _RX15M14W {
        _RX15M14W { w: self }
    }
    #[doc = "Bit 15 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m15(&mut self) -> _RX15M15W {
        _RX15M15W { w: self }
    }
    #[doc = "Bit 16 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m16(&mut self) -> _RX15M16W {
        _RX15M16W { w: self }
    }
    #[doc = "Bit 17 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m17(&mut self) -> _RX15M17W {
        _RX15M17W { w: self }
    }
    #[doc = "Bit 18 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m18(&mut self) -> _RX15M18W {
        _RX15M18W { w: self }
    }
    #[doc = "Bit 19 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m19(&mut self) -> _RX15M19W {
        _RX15M19W { w: self }
    }
    #[doc = "Bit 20 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m20(&mut self) -> _RX15M20W {
        _RX15M20W { w: self }
    }
    #[doc = "Bit 21 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m21(&mut self) -> _RX15M21W {
        _RX15M21W { w: self }
    }
    #[doc = "Bit 22 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m22(&mut self) -> _RX15M22W {
        _RX15M22W { w: self }
    }
    #[doc = "Bit 23 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m23(&mut self) -> _RX15M23W {
        _RX15M23W { w: self }
    }
    #[doc = "Bit 24 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m24(&mut self) -> _RX15M24W {
        _RX15M24W { w: self }
    }
    #[doc = "Bit 25 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m25(&mut self) -> _RX15M25W {
        _RX15M25W { w: self }
    }
    #[doc = "Bit 26 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m26(&mut self) -> _RX15M26W {
        _RX15M26W { w: self }
    }
    #[doc = "Bit 27 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m27(&mut self) -> _RX15M27W {
        _RX15M27W { w: self }
    }
    #[doc = "Bit 28 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m28(&mut self) -> _RX15M28W {
        _RX15M28W { w: self }
    }
    #[doc = "Bit 29 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m29(&mut self) -> _RX15M29W {
        _RX15M29W { w: self }
    }
    #[doc = "Bit 30 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m30(&mut self) -> _RX15M30W {
        _RX15M30W { w: self }
    }
    #[doc = "Bit 31 - Rx Buffer 15 Mask Bits"]
    #[inline]
    pub fn rx15m31(&mut self) -> _RX15M31W {
        _RX15M31W { w: self }
    }
}
