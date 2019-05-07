#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IFLAG1 {
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
#[doc = "Possible values of the field `BUF0I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF0IR {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _1,
}
impl BUF0IR {
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
            BUF0IR::_0 => false,
            BUF0IR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF0IR {
        match value {
            false => BUF0IR::_0,
            true => BUF0IR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF0IR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF0IR::_1
    }
}
#[doc = "Possible values of the field `BUF4TO1I0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF4TO1I0R {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _1,
}
impl BUF4TO1I0R {
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
            BUF4TO1I0R::_0 => false,
            BUF4TO1I0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF4TO1I0R {
        match value {
            false => BUF4TO1I0R::_0,
            true => BUF4TO1I0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF4TO1I0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF4TO1I0R::_1
    }
}
#[doc = "Possible values of the field `BUF4TO1I1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF4TO1I1R {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _1,
}
impl BUF4TO1I1R {
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
            BUF4TO1I1R::_0 => false,
            BUF4TO1I1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF4TO1I1R {
        match value {
            false => BUF4TO1I1R::_0,
            true => BUF4TO1I1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF4TO1I1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF4TO1I1R::_1
    }
}
#[doc = "Possible values of the field `BUF4TO1I2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF4TO1I2R {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _1,
}
impl BUF4TO1I2R {
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
            BUF4TO1I2R::_0 => false,
            BUF4TO1I2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF4TO1I2R {
        match value {
            false => BUF4TO1I2R::_0,
            true => BUF4TO1I2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF4TO1I2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF4TO1I2R::_1
    }
}
#[doc = "Possible values of the field `BUF4TO1I3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF4TO1I3R {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _1,
}
impl BUF4TO1I3R {
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
            BUF4TO1I3R::_0 => false,
            BUF4TO1I3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF4TO1I3R {
        match value {
            false => BUF4TO1I3R::_0,
            true => BUF4TO1I3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF4TO1I3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF4TO1I3R::_1
    }
}
#[doc = "Possible values of the field `BUF5I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF5IR {
    #[doc = "No occurrence of MB5 completing transmission/reception when MCR\\[RFEN\\]=0, or of frame(s) available in the FIFO, when MCR\\[RFEN\\]=1"]
    _0,
    #[doc = "MB5 completed transmission/reception when MCR\\[RFEN\\]=0, or frame(s) available in the Rx FIFO when MCR\\[RFEN\\]=1"]
    _1,
}
impl BUF5IR {
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
            BUF5IR::_0 => false,
            BUF5IR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF5IR {
        match value {
            false => BUF5IR::_0,
            true => BUF5IR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF5IR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF5IR::_1
    }
}
#[doc = "Possible values of the field `BUF6I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF6IR {
    #[doc = "No occurrence of MB6 completing transmission/reception when MCR\\[RFEN\\]=0, or of Rx FIFO almost full when MCR\\[RFEN\\]=1"]
    _0,
    #[doc = "MB6 completed transmission/reception when MCR\\[RFEN\\]=0, or Rx FIFO almost full when MCR\\[RFEN\\]=1"]
    _1,
}
impl BUF6IR {
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
            BUF6IR::_0 => false,
            BUF6IR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF6IR {
        match value {
            false => BUF6IR::_0,
            true => BUF6IR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF6IR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF6IR::_1
    }
}
#[doc = "Possible values of the field `BUF7I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF7IR {
    #[doc = "No occurrence of MB7 completing transmission/reception when MCR\\[RFEN\\]=0, or of Rx FIFO overflow when MCR\\[RFEN\\]=1"]
    _0,
    #[doc = "MB7 completed transmission/reception when MCR\\[RFEN\\]=0, or Rx FIFO overflow when MCR\\[RFEN\\]=1"]
    _1,
}
impl BUF7IR {
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
            BUF7IR::_0 => false,
            BUF7IR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF7IR {
        match value {
            false => BUF7IR::_0,
            true => BUF7IR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF7IR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF7IR::_1
    }
}
#[doc = "Possible values of the field `BUF31TO8I0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I0R {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I0R {
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
            BUF31TO8I0R::_0 => false,
            BUF31TO8I0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF31TO8I0R {
        match value {
            false => BUF31TO8I0R::_0,
            true => BUF31TO8I0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I0R::_1
    }
}
#[doc = "Possible values of the field `BUF31TO8I1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I1R {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I1R {
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
            BUF31TO8I1R::_0 => false,
            BUF31TO8I1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF31TO8I1R {
        match value {
            false => BUF31TO8I1R::_0,
            true => BUF31TO8I1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I1R::_1
    }
}
#[doc = "Possible values of the field `BUF31TO8I2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I2R {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I2R {
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
            BUF31TO8I2R::_0 => false,
            BUF31TO8I2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF31TO8I2R {
        match value {
            false => BUF31TO8I2R::_0,
            true => BUF31TO8I2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I2R::_1
    }
}
#[doc = "Possible values of the field `BUF31TO8I3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I3R {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I3R {
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
            BUF31TO8I3R::_0 => false,
            BUF31TO8I3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF31TO8I3R {
        match value {
            false => BUF31TO8I3R::_0,
            true => BUF31TO8I3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I3R::_1
    }
}
#[doc = "Possible values of the field `BUF31TO8I4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I4R {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I4R {
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
            BUF31TO8I4R::_0 => false,
            BUF31TO8I4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF31TO8I4R {
        match value {
            false => BUF31TO8I4R::_0,
            true => BUF31TO8I4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I4R::_1
    }
}
#[doc = "Possible values of the field `BUF31TO8I5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I5R {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I5R {
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
            BUF31TO8I5R::_0 => false,
            BUF31TO8I5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF31TO8I5R {
        match value {
            false => BUF31TO8I5R::_0,
            true => BUF31TO8I5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I5R::_1
    }
}
#[doc = "Possible values of the field `BUF31TO8I6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I6R {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I6R {
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
            BUF31TO8I6R::_0 => false,
            BUF31TO8I6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF31TO8I6R {
        match value {
            false => BUF31TO8I6R::_0,
            true => BUF31TO8I6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I6R::_1
    }
}
#[doc = "Possible values of the field `BUF31TO8I7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I7R {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I7R {
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
            BUF31TO8I7R::_0 => false,
            BUF31TO8I7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF31TO8I7R {
        match value {
            false => BUF31TO8I7R::_0,
            true => BUF31TO8I7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I7R::_1
    }
}
#[doc = "Possible values of the field `BUF31TO8I8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I8R {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I8R {
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
            BUF31TO8I8R::_0 => false,
            BUF31TO8I8R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF31TO8I8R {
        match value {
            false => BUF31TO8I8R::_0,
            true => BUF31TO8I8R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I8R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I8R::_1
    }
}
#[doc = "Possible values of the field `BUF31TO8I9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I9R {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I9R {
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
            BUF31TO8I9R::_0 => false,
            BUF31TO8I9R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF31TO8I9R {
        match value {
            false => BUF31TO8I9R::_0,
            true => BUF31TO8I9R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I9R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I9R::_1
    }
}
#[doc = "Possible values of the field `BUF31TO8I10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I10R {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I10R {
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
            BUF31TO8I10R::_0 => false,
            BUF31TO8I10R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF31TO8I10R {
        match value {
            false => BUF31TO8I10R::_0,
            true => BUF31TO8I10R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I10R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I10R::_1
    }
}
#[doc = "Possible values of the field `BUF31TO8I11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I11R {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I11R {
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
            BUF31TO8I11R::_0 => false,
            BUF31TO8I11R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF31TO8I11R {
        match value {
            false => BUF31TO8I11R::_0,
            true => BUF31TO8I11R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I11R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I11R::_1
    }
}
#[doc = "Possible values of the field `BUF31TO8I12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I12R {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I12R {
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
            BUF31TO8I12R::_0 => false,
            BUF31TO8I12R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF31TO8I12R {
        match value {
            false => BUF31TO8I12R::_0,
            true => BUF31TO8I12R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I12R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I12R::_1
    }
}
#[doc = "Possible values of the field `BUF31TO8I13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I13R {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I13R {
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
            BUF31TO8I13R::_0 => false,
            BUF31TO8I13R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF31TO8I13R {
        match value {
            false => BUF31TO8I13R::_0,
            true => BUF31TO8I13R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I13R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I13R::_1
    }
}
#[doc = "Possible values of the field `BUF31TO8I14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I14R {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I14R {
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
            BUF31TO8I14R::_0 => false,
            BUF31TO8I14R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF31TO8I14R {
        match value {
            false => BUF31TO8I14R::_0,
            true => BUF31TO8I14R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I14R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I14R::_1
    }
}
#[doc = "Possible values of the field `BUF31TO8I15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I15R {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I15R {
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
            BUF31TO8I15R::_0 => false,
            BUF31TO8I15R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF31TO8I15R {
        match value {
            false => BUF31TO8I15R::_0,
            true => BUF31TO8I15R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I15R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I15R::_1
    }
}
#[doc = "Possible values of the field `BUF31TO8I16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I16R {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I16R {
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
            BUF31TO8I16R::_0 => false,
            BUF31TO8I16R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF31TO8I16R {
        match value {
            false => BUF31TO8I16R::_0,
            true => BUF31TO8I16R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I16R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I16R::_1
    }
}
#[doc = "Possible values of the field `BUF31TO8I17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I17R {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I17R {
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
            BUF31TO8I17R::_0 => false,
            BUF31TO8I17R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF31TO8I17R {
        match value {
            false => BUF31TO8I17R::_0,
            true => BUF31TO8I17R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I17R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I17R::_1
    }
}
#[doc = "Possible values of the field `BUF31TO8I18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I18R {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I18R {
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
            BUF31TO8I18R::_0 => false,
            BUF31TO8I18R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF31TO8I18R {
        match value {
            false => BUF31TO8I18R::_0,
            true => BUF31TO8I18R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I18R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I18R::_1
    }
}
#[doc = "Possible values of the field `BUF31TO8I19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I19R {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I19R {
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
            BUF31TO8I19R::_0 => false,
            BUF31TO8I19R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF31TO8I19R {
        match value {
            false => BUF31TO8I19R::_0,
            true => BUF31TO8I19R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I19R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I19R::_1
    }
}
#[doc = "Possible values of the field `BUF31TO8I20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I20R {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I20R {
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
            BUF31TO8I20R::_0 => false,
            BUF31TO8I20R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF31TO8I20R {
        match value {
            false => BUF31TO8I20R::_0,
            true => BUF31TO8I20R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I20R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I20R::_1
    }
}
#[doc = "Possible values of the field `BUF31TO8I21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I21R {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I21R {
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
            BUF31TO8I21R::_0 => false,
            BUF31TO8I21R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF31TO8I21R {
        match value {
            false => BUF31TO8I21R::_0,
            true => BUF31TO8I21R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I21R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I21R::_1
    }
}
#[doc = "Possible values of the field `BUF31TO8I22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I22R {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I22R {
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
            BUF31TO8I22R::_0 => false,
            BUF31TO8I22R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF31TO8I22R {
        match value {
            false => BUF31TO8I22R::_0,
            true => BUF31TO8I22R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I22R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I22R::_1
    }
}
#[doc = "Possible values of the field `BUF31TO8I23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I23R {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I23R {
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
            BUF31TO8I23R::_0 => false,
            BUF31TO8I23R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF31TO8I23R {
        match value {
            false => BUF31TO8I23R::_0,
            true => BUF31TO8I23R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I23R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I23R::_1
    }
}
#[doc = "Values that can be written to the field `BUF0I`"]
pub enum BUF0IW {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _1,
}
impl BUF0IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF0IW::_0 => false,
            BUF0IW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF0IW<'a> {
    w: &'a mut W,
}
impl<'a> _BUF0IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF0IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF0IW::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF0IW::_1)
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
#[doc = "Values that can be written to the field `BUF4TO1I0`"]
pub enum BUF4TO1I0W {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _1,
}
impl BUF4TO1I0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF4TO1I0W::_0 => false,
            BUF4TO1I0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF4TO1I0W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF4TO1I0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF4TO1I0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF4TO1I0W::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF4TO1I0W::_1)
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
#[doc = "Values that can be written to the field `BUF4TO1I1`"]
pub enum BUF4TO1I1W {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _1,
}
impl BUF4TO1I1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF4TO1I1W::_0 => false,
            BUF4TO1I1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF4TO1I1W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF4TO1I1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF4TO1I1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF4TO1I1W::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF4TO1I1W::_1)
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
#[doc = "Values that can be written to the field `BUF4TO1I2`"]
pub enum BUF4TO1I2W {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _1,
}
impl BUF4TO1I2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF4TO1I2W::_0 => false,
            BUF4TO1I2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF4TO1I2W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF4TO1I2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF4TO1I2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF4TO1I2W::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF4TO1I2W::_1)
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
#[doc = "Values that can be written to the field `BUF4TO1I3`"]
pub enum BUF4TO1I3W {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _1,
}
impl BUF4TO1I3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF4TO1I3W::_0 => false,
            BUF4TO1I3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF4TO1I3W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF4TO1I3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF4TO1I3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF4TO1I3W::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF4TO1I3W::_1)
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
#[doc = "Values that can be written to the field `BUF5I`"]
pub enum BUF5IW {
    #[doc = "No occurrence of MB5 completing transmission/reception when MCR\\[RFEN\\]=0, or of frame(s) available in the FIFO, when MCR\\[RFEN\\]=1"]
    _0,
    #[doc = "MB5 completed transmission/reception when MCR\\[RFEN\\]=0, or frame(s) available in the Rx FIFO when MCR\\[RFEN\\]=1"]
    _1,
}
impl BUF5IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF5IW::_0 => false,
            BUF5IW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF5IW<'a> {
    w: &'a mut W,
}
impl<'a> _BUF5IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF5IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No occurrence of MB5 completing transmission/reception when MCR\\[RFEN\\]=0, or of frame(s) available in the FIFO, when MCR\\[RFEN\\]=1"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF5IW::_0)
    }
    #[doc = "MB5 completed transmission/reception when MCR\\[RFEN\\]=0, or frame(s) available in the Rx FIFO when MCR\\[RFEN\\]=1"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF5IW::_1)
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
#[doc = "Values that can be written to the field `BUF6I`"]
pub enum BUF6IW {
    #[doc = "No occurrence of MB6 completing transmission/reception when MCR\\[RFEN\\]=0, or of Rx FIFO almost full when MCR\\[RFEN\\]=1"]
    _0,
    #[doc = "MB6 completed transmission/reception when MCR\\[RFEN\\]=0, or Rx FIFO almost full when MCR\\[RFEN\\]=1"]
    _1,
}
impl BUF6IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF6IW::_0 => false,
            BUF6IW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF6IW<'a> {
    w: &'a mut W,
}
impl<'a> _BUF6IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF6IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No occurrence of MB6 completing transmission/reception when MCR\\[RFEN\\]=0, or of Rx FIFO almost full when MCR\\[RFEN\\]=1"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF6IW::_0)
    }
    #[doc = "MB6 completed transmission/reception when MCR\\[RFEN\\]=0, or Rx FIFO almost full when MCR\\[RFEN\\]=1"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF6IW::_1)
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
#[doc = "Values that can be written to the field `BUF7I`"]
pub enum BUF7IW {
    #[doc = "No occurrence of MB7 completing transmission/reception when MCR\\[RFEN\\]=0, or of Rx FIFO overflow when MCR\\[RFEN\\]=1"]
    _0,
    #[doc = "MB7 completed transmission/reception when MCR\\[RFEN\\]=0, or Rx FIFO overflow when MCR\\[RFEN\\]=1"]
    _1,
}
impl BUF7IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF7IW::_0 => false,
            BUF7IW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF7IW<'a> {
    w: &'a mut W,
}
impl<'a> _BUF7IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF7IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No occurrence of MB7 completing transmission/reception when MCR\\[RFEN\\]=0, or of Rx FIFO overflow when MCR\\[RFEN\\]=1"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF7IW::_0)
    }
    #[doc = "MB7 completed transmission/reception when MCR\\[RFEN\\]=0, or Rx FIFO overflow when MCR\\[RFEN\\]=1"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF7IW::_1)
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
#[doc = "Values that can be written to the field `BUF31TO8I0`"]
pub enum BUF31TO8I0W {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF31TO8I0W::_0 => false,
            BUF31TO8I0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF31TO8I0W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF31TO8I0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF31TO8I0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I0W::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I0W::_1)
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
#[doc = "Values that can be written to the field `BUF31TO8I1`"]
pub enum BUF31TO8I1W {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF31TO8I1W::_0 => false,
            BUF31TO8I1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF31TO8I1W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF31TO8I1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF31TO8I1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I1W::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I1W::_1)
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
#[doc = "Values that can be written to the field `BUF31TO8I2`"]
pub enum BUF31TO8I2W {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF31TO8I2W::_0 => false,
            BUF31TO8I2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF31TO8I2W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF31TO8I2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF31TO8I2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I2W::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I2W::_1)
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
#[doc = "Values that can be written to the field `BUF31TO8I3`"]
pub enum BUF31TO8I3W {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF31TO8I3W::_0 => false,
            BUF31TO8I3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF31TO8I3W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF31TO8I3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF31TO8I3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I3W::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I3W::_1)
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
#[doc = "Values that can be written to the field `BUF31TO8I4`"]
pub enum BUF31TO8I4W {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF31TO8I4W::_0 => false,
            BUF31TO8I4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF31TO8I4W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF31TO8I4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF31TO8I4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I4W::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I4W::_1)
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
#[doc = "Values that can be written to the field `BUF31TO8I5`"]
pub enum BUF31TO8I5W {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF31TO8I5W::_0 => false,
            BUF31TO8I5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF31TO8I5W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF31TO8I5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF31TO8I5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I5W::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I5W::_1)
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
#[doc = "Values that can be written to the field `BUF31TO8I6`"]
pub enum BUF31TO8I6W {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF31TO8I6W::_0 => false,
            BUF31TO8I6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF31TO8I6W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF31TO8I6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF31TO8I6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I6W::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I6W::_1)
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
#[doc = "Values that can be written to the field `BUF31TO8I7`"]
pub enum BUF31TO8I7W {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF31TO8I7W::_0 => false,
            BUF31TO8I7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF31TO8I7W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF31TO8I7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF31TO8I7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I7W::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I7W::_1)
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
#[doc = "Values that can be written to the field `BUF31TO8I8`"]
pub enum BUF31TO8I8W {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF31TO8I8W::_0 => false,
            BUF31TO8I8W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF31TO8I8W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF31TO8I8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF31TO8I8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I8W::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I8W::_1)
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
#[doc = "Values that can be written to the field `BUF31TO8I9`"]
pub enum BUF31TO8I9W {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF31TO8I9W::_0 => false,
            BUF31TO8I9W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF31TO8I9W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF31TO8I9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF31TO8I9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I9W::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I9W::_1)
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
#[doc = "Values that can be written to the field `BUF31TO8I10`"]
pub enum BUF31TO8I10W {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF31TO8I10W::_0 => false,
            BUF31TO8I10W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF31TO8I10W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF31TO8I10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF31TO8I10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I10W::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I10W::_1)
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
#[doc = "Values that can be written to the field `BUF31TO8I11`"]
pub enum BUF31TO8I11W {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF31TO8I11W::_0 => false,
            BUF31TO8I11W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF31TO8I11W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF31TO8I11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF31TO8I11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I11W::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I11W::_1)
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
#[doc = "Values that can be written to the field `BUF31TO8I12`"]
pub enum BUF31TO8I12W {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF31TO8I12W::_0 => false,
            BUF31TO8I12W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF31TO8I12W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF31TO8I12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF31TO8I12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I12W::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I12W::_1)
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
#[doc = "Values that can be written to the field `BUF31TO8I13`"]
pub enum BUF31TO8I13W {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF31TO8I13W::_0 => false,
            BUF31TO8I13W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF31TO8I13W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF31TO8I13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF31TO8I13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I13W::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I13W::_1)
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
#[doc = "Values that can be written to the field `BUF31TO8I14`"]
pub enum BUF31TO8I14W {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF31TO8I14W::_0 => false,
            BUF31TO8I14W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF31TO8I14W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF31TO8I14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF31TO8I14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I14W::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I14W::_1)
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
#[doc = "Values that can be written to the field `BUF31TO8I15`"]
pub enum BUF31TO8I15W {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF31TO8I15W::_0 => false,
            BUF31TO8I15W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF31TO8I15W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF31TO8I15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF31TO8I15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I15W::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I15W::_1)
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
#[doc = "Values that can be written to the field `BUF31TO8I16`"]
pub enum BUF31TO8I16W {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF31TO8I16W::_0 => false,
            BUF31TO8I16W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF31TO8I16W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF31TO8I16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF31TO8I16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I16W::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I16W::_1)
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
#[doc = "Values that can be written to the field `BUF31TO8I17`"]
pub enum BUF31TO8I17W {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF31TO8I17W::_0 => false,
            BUF31TO8I17W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF31TO8I17W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF31TO8I17W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF31TO8I17W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I17W::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I17W::_1)
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
#[doc = "Values that can be written to the field `BUF31TO8I18`"]
pub enum BUF31TO8I18W {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF31TO8I18W::_0 => false,
            BUF31TO8I18W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF31TO8I18W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF31TO8I18W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF31TO8I18W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I18W::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I18W::_1)
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
#[doc = "Values that can be written to the field `BUF31TO8I19`"]
pub enum BUF31TO8I19W {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF31TO8I19W::_0 => false,
            BUF31TO8I19W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF31TO8I19W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF31TO8I19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF31TO8I19W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I19W::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I19W::_1)
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
#[doc = "Values that can be written to the field `BUF31TO8I20`"]
pub enum BUF31TO8I20W {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF31TO8I20W::_0 => false,
            BUF31TO8I20W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF31TO8I20W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF31TO8I20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF31TO8I20W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I20W::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I20W::_1)
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
#[doc = "Values that can be written to the field `BUF31TO8I21`"]
pub enum BUF31TO8I21W {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF31TO8I21W::_0 => false,
            BUF31TO8I21W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF31TO8I21W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF31TO8I21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF31TO8I21W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I21W::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I21W::_1)
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
#[doc = "Values that can be written to the field `BUF31TO8I22`"]
pub enum BUF31TO8I22W {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF31TO8I22W::_0 => false,
            BUF31TO8I22W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF31TO8I22W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF31TO8I22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF31TO8I22W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I22W::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I22W::_1)
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
#[doc = "Values that can be written to the field `BUF31TO8I23`"]
pub enum BUF31TO8I23W {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl BUF31TO8I23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF31TO8I23W::_0 => false,
            BUF31TO8I23W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF31TO8I23W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF31TO8I23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF31TO8I23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I23W::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I23W::_1)
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
    #[doc = "Bit 0 - Buffer MB0 Interrupt Or \"reserved\""]
    #[inline]
    pub fn buf0i(&self) -> BUF0IR {
        BUF0IR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Buffer MB i Interrupt Or \"reserved\""]
    #[inline]
    pub fn buf4to1i0(&self) -> BUF4TO1I0R {
        BUF4TO1I0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Buffer MB i Interrupt Or \"reserved\""]
    #[inline]
    pub fn buf4to1i1(&self) -> BUF4TO1I1R {
        BUF4TO1I1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Buffer MB i Interrupt Or \"reserved\""]
    #[inline]
    pub fn buf4to1i2(&self) -> BUF4TO1I2R {
        BUF4TO1I2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Buffer MB i Interrupt Or \"reserved\""]
    #[inline]
    pub fn buf4to1i3(&self) -> BUF4TO1I3R {
        BUF4TO1I3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Buffer MB5 Interrupt Or \"Frames available in Rx FIFO\""]
    #[inline]
    pub fn buf5i(&self) -> BUF5IR {
        BUF5IR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Buffer MB6 Interrupt Or \"Rx FIFO Warning\""]
    #[inline]
    pub fn buf6i(&self) -> BUF6IR {
        BUF6IR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Buffer MB7 Interrupt Or \"Rx FIFO Overflow\""]
    #[inline]
    pub fn buf7i(&self) -> BUF7IR {
        BUF7IR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i0(&self) -> BUF31TO8I0R {
        BUF31TO8I0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i1(&self) -> BUF31TO8I1R {
        BUF31TO8I1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i2(&self) -> BUF31TO8I2R {
        BUF31TO8I2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i3(&self) -> BUF31TO8I3R {
        BUF31TO8I3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i4(&self) -> BUF31TO8I4R {
        BUF31TO8I4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i5(&self) -> BUF31TO8I5R {
        BUF31TO8I5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i6(&self) -> BUF31TO8I6R {
        BUF31TO8I6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i7(&self) -> BUF31TO8I7R {
        BUF31TO8I7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i8(&self) -> BUF31TO8I8R {
        BUF31TO8I8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i9(&self) -> BUF31TO8I9R {
        BUF31TO8I9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i10(&self) -> BUF31TO8I10R {
        BUF31TO8I10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i11(&self) -> BUF31TO8I11R {
        BUF31TO8I11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i12(&self) -> BUF31TO8I12R {
        BUF31TO8I12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i13(&self) -> BUF31TO8I13R {
        BUF31TO8I13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i14(&self) -> BUF31TO8I14R {
        BUF31TO8I14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i15(&self) -> BUF31TO8I15R {
        BUF31TO8I15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i16(&self) -> BUF31TO8I16R {
        BUF31TO8I16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i17(&self) -> BUF31TO8I17R {
        BUF31TO8I17R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i18(&self) -> BUF31TO8I18R {
        BUF31TO8I18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i19(&self) -> BUF31TO8I19R {
        BUF31TO8I19R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i20(&self) -> BUF31TO8I20R {
        BUF31TO8I20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i21(&self) -> BUF31TO8I21R {
        BUF31TO8I21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i22(&self) -> BUF31TO8I22R {
        BUF31TO8I22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i23(&self) -> BUF31TO8I23R {
        BUF31TO8I23R::_from({
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
    #[doc = "Bit 0 - Buffer MB0 Interrupt Or \"reserved\""]
    #[inline]
    pub fn buf0i(&mut self) -> _BUF0IW {
        _BUF0IW { w: self }
    }
    #[doc = "Bit 1 - Buffer MB i Interrupt Or \"reserved\""]
    #[inline]
    pub fn buf4to1i0(&mut self) -> _BUF4TO1I0W {
        _BUF4TO1I0W { w: self }
    }
    #[doc = "Bit 2 - Buffer MB i Interrupt Or \"reserved\""]
    #[inline]
    pub fn buf4to1i1(&mut self) -> _BUF4TO1I1W {
        _BUF4TO1I1W { w: self }
    }
    #[doc = "Bit 3 - Buffer MB i Interrupt Or \"reserved\""]
    #[inline]
    pub fn buf4to1i2(&mut self) -> _BUF4TO1I2W {
        _BUF4TO1I2W { w: self }
    }
    #[doc = "Bit 4 - Buffer MB i Interrupt Or \"reserved\""]
    #[inline]
    pub fn buf4to1i3(&mut self) -> _BUF4TO1I3W {
        _BUF4TO1I3W { w: self }
    }
    #[doc = "Bit 5 - Buffer MB5 Interrupt Or \"Frames available in Rx FIFO\""]
    #[inline]
    pub fn buf5i(&mut self) -> _BUF5IW {
        _BUF5IW { w: self }
    }
    #[doc = "Bit 6 - Buffer MB6 Interrupt Or \"Rx FIFO Warning\""]
    #[inline]
    pub fn buf6i(&mut self) -> _BUF6IW {
        _BUF6IW { w: self }
    }
    #[doc = "Bit 7 - Buffer MB7 Interrupt Or \"Rx FIFO Overflow\""]
    #[inline]
    pub fn buf7i(&mut self) -> _BUF7IW {
        _BUF7IW { w: self }
    }
    #[doc = "Bit 8 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i0(&mut self) -> _BUF31TO8I0W {
        _BUF31TO8I0W { w: self }
    }
    #[doc = "Bit 9 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i1(&mut self) -> _BUF31TO8I1W {
        _BUF31TO8I1W { w: self }
    }
    #[doc = "Bit 10 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i2(&mut self) -> _BUF31TO8I2W {
        _BUF31TO8I2W { w: self }
    }
    #[doc = "Bit 11 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i3(&mut self) -> _BUF31TO8I3W {
        _BUF31TO8I3W { w: self }
    }
    #[doc = "Bit 12 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i4(&mut self) -> _BUF31TO8I4W {
        _BUF31TO8I4W { w: self }
    }
    #[doc = "Bit 13 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i5(&mut self) -> _BUF31TO8I5W {
        _BUF31TO8I5W { w: self }
    }
    #[doc = "Bit 14 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i6(&mut self) -> _BUF31TO8I6W {
        _BUF31TO8I6W { w: self }
    }
    #[doc = "Bit 15 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i7(&mut self) -> _BUF31TO8I7W {
        _BUF31TO8I7W { w: self }
    }
    #[doc = "Bit 16 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i8(&mut self) -> _BUF31TO8I8W {
        _BUF31TO8I8W { w: self }
    }
    #[doc = "Bit 17 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i9(&mut self) -> _BUF31TO8I9W {
        _BUF31TO8I9W { w: self }
    }
    #[doc = "Bit 18 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i10(&mut self) -> _BUF31TO8I10W {
        _BUF31TO8I10W { w: self }
    }
    #[doc = "Bit 19 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i11(&mut self) -> _BUF31TO8I11W {
        _BUF31TO8I11W { w: self }
    }
    #[doc = "Bit 20 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i12(&mut self) -> _BUF31TO8I12W {
        _BUF31TO8I12W { w: self }
    }
    #[doc = "Bit 21 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i13(&mut self) -> _BUF31TO8I13W {
        _BUF31TO8I13W { w: self }
    }
    #[doc = "Bit 22 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i14(&mut self) -> _BUF31TO8I14W {
        _BUF31TO8I14W { w: self }
    }
    #[doc = "Bit 23 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i15(&mut self) -> _BUF31TO8I15W {
        _BUF31TO8I15W { w: self }
    }
    #[doc = "Bit 24 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i16(&mut self) -> _BUF31TO8I16W {
        _BUF31TO8I16W { w: self }
    }
    #[doc = "Bit 25 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i17(&mut self) -> _BUF31TO8I17W {
        _BUF31TO8I17W { w: self }
    }
    #[doc = "Bit 26 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i18(&mut self) -> _BUF31TO8I18W {
        _BUF31TO8I18W { w: self }
    }
    #[doc = "Bit 27 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i19(&mut self) -> _BUF31TO8I19W {
        _BUF31TO8I19W { w: self }
    }
    #[doc = "Bit 28 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i20(&mut self) -> _BUF31TO8I20W {
        _BUF31TO8I20W { w: self }
    }
    #[doc = "Bit 29 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i21(&mut self) -> _BUF31TO8I21W {
        _BUF31TO8I21W { w: self }
    }
    #[doc = "Bit 30 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i22(&mut self) -> _BUF31TO8I22W {
        _BUF31TO8I22W { w: self }
    }
    #[doc = "Bit 31 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i23(&mut self) -> _BUF31TO8I23W {
        _BUF31TO8I23W { w: self }
    }
}
