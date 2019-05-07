#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::C6 {
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
#[doc = "Possible values of the field `VDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDIVR {
    #[doc = "Multiply Factor is 16"]
    _0,
    #[doc = "Multiply Factor is 17"]
    _1,
    #[doc = "Multiply Factor is 18"]
    _2,
    #[doc = "Multiply Factor is 19"]
    _3,
    #[doc = "Multiply Factor is 20"]
    _4,
    #[doc = "Multiply Factor is 21"]
    _5,
    #[doc = "Multiply Factor is 22"]
    _6,
    #[doc = "Multiply Factor is 23"]
    _7,
    #[doc = "Multiply Factor is 24"]
    _8,
    #[doc = "Multiply Factor is 25"]
    _9,
    #[doc = "Multiply Factor is 26"]
    _10,
    #[doc = "Multiply Factor is 27"]
    _11,
    #[doc = "Multiply Factor is 28"]
    _12,
    #[doc = "Multiply Factor is 29"]
    _13,
    #[doc = "Multiply Factor is 30"]
    _14,
    #[doc = "Multiply Factor is 31"]
    _15,
    #[doc = "Multiply Factor is 32"]
    _16,
    #[doc = "Multiply Factor is 33"]
    _17,
    #[doc = "Multiply Factor is 34"]
    _18,
    #[doc = "Multiply Factor is 35"]
    _19,
    #[doc = "Multiply Factor is 36"]
    _20,
    #[doc = "Multiply Factor is 37"]
    _21,
    #[doc = "Multiply Factor is 38"]
    _22,
    #[doc = "Multiply Factor is 39"]
    _23,
    #[doc = "Multiply Factor is 40"]
    _24,
    #[doc = "Multiply Factor is 41"]
    _25,
    #[doc = "Multiply Factor is 42"]
    _26,
    #[doc = "Multiply Factor is 43"]
    _27,
    #[doc = "Multiply Factor is 44"]
    _28,
    #[doc = "Multiply Factor is 45"]
    _29,
    #[doc = "Multiply Factor is 46"]
    _30,
    #[doc = "Multiply Factor is 47"]
    _31,
}
impl VDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VDIVR::_0 => 0,
            VDIVR::_1 => 1,
            VDIVR::_2 => 2,
            VDIVR::_3 => 3,
            VDIVR::_4 => 4,
            VDIVR::_5 => 5,
            VDIVR::_6 => 6,
            VDIVR::_7 => 7,
            VDIVR::_8 => 8,
            VDIVR::_9 => 9,
            VDIVR::_10 => 10,
            VDIVR::_11 => 11,
            VDIVR::_12 => 12,
            VDIVR::_13 => 13,
            VDIVR::_14 => 14,
            VDIVR::_15 => 15,
            VDIVR::_16 => 16,
            VDIVR::_17 => 17,
            VDIVR::_18 => 18,
            VDIVR::_19 => 19,
            VDIVR::_20 => 20,
            VDIVR::_21 => 21,
            VDIVR::_22 => 22,
            VDIVR::_23 => 23,
            VDIVR::_24 => 24,
            VDIVR::_25 => 25,
            VDIVR::_26 => 26,
            VDIVR::_27 => 27,
            VDIVR::_28 => 28,
            VDIVR::_29 => 29,
            VDIVR::_30 => 30,
            VDIVR::_31 => 31,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VDIVR {
        match value {
            0 => VDIVR::_0,
            1 => VDIVR::_1,
            2 => VDIVR::_2,
            3 => VDIVR::_3,
            4 => VDIVR::_4,
            5 => VDIVR::_5,
            6 => VDIVR::_6,
            7 => VDIVR::_7,
            8 => VDIVR::_8,
            9 => VDIVR::_9,
            10 => VDIVR::_10,
            11 => VDIVR::_11,
            12 => VDIVR::_12,
            13 => VDIVR::_13,
            14 => VDIVR::_14,
            15 => VDIVR::_15,
            16 => VDIVR::_16,
            17 => VDIVR::_17,
            18 => VDIVR::_18,
            19 => VDIVR::_19,
            20 => VDIVR::_20,
            21 => VDIVR::_21,
            22 => VDIVR::_22,
            23 => VDIVR::_23,
            24 => VDIVR::_24,
            25 => VDIVR::_25,
            26 => VDIVR::_26,
            27 => VDIVR::_27,
            28 => VDIVR::_28,
            29 => VDIVR::_29,
            30 => VDIVR::_30,
            31 => VDIVR::_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == VDIVR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == VDIVR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == VDIVR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == VDIVR::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == VDIVR::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline]
    pub fn is_5(&self) -> bool {
        *self == VDIVR::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline]
    pub fn is_6(&self) -> bool {
        *self == VDIVR::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline]
    pub fn is_7(&self) -> bool {
        *self == VDIVR::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == VDIVR::_8
    }
    #[doc = "Checks if the value of the field is `_9`"]
    #[inline]
    pub fn is_9(&self) -> bool {
        *self == VDIVR::_9
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == VDIVR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == VDIVR::_11
    }
    #[doc = "Checks if the value of the field is `_12`"]
    #[inline]
    pub fn is_12(&self) -> bool {
        *self == VDIVR::_12
    }
    #[doc = "Checks if the value of the field is `_13`"]
    #[inline]
    pub fn is_13(&self) -> bool {
        *self == VDIVR::_13
    }
    #[doc = "Checks if the value of the field is `_14`"]
    #[inline]
    pub fn is_14(&self) -> bool {
        *self == VDIVR::_14
    }
    #[doc = "Checks if the value of the field is `_15`"]
    #[inline]
    pub fn is_15(&self) -> bool {
        *self == VDIVR::_15
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == VDIVR::_16
    }
    #[doc = "Checks if the value of the field is `_17`"]
    #[inline]
    pub fn is_17(&self) -> bool {
        *self == VDIVR::_17
    }
    #[doc = "Checks if the value of the field is `_18`"]
    #[inline]
    pub fn is_18(&self) -> bool {
        *self == VDIVR::_18
    }
    #[doc = "Checks if the value of the field is `_19`"]
    #[inline]
    pub fn is_19(&self) -> bool {
        *self == VDIVR::_19
    }
    #[doc = "Checks if the value of the field is `_20`"]
    #[inline]
    pub fn is_20(&self) -> bool {
        *self == VDIVR::_20
    }
    #[doc = "Checks if the value of the field is `_21`"]
    #[inline]
    pub fn is_21(&self) -> bool {
        *self == VDIVR::_21
    }
    #[doc = "Checks if the value of the field is `_22`"]
    #[inline]
    pub fn is_22(&self) -> bool {
        *self == VDIVR::_22
    }
    #[doc = "Checks if the value of the field is `_23`"]
    #[inline]
    pub fn is_23(&self) -> bool {
        *self == VDIVR::_23
    }
    #[doc = "Checks if the value of the field is `_24`"]
    #[inline]
    pub fn is_24(&self) -> bool {
        *self == VDIVR::_24
    }
    #[doc = "Checks if the value of the field is `_25`"]
    #[inline]
    pub fn is_25(&self) -> bool {
        *self == VDIVR::_25
    }
    #[doc = "Checks if the value of the field is `_26`"]
    #[inline]
    pub fn is_26(&self) -> bool {
        *self == VDIVR::_26
    }
    #[doc = "Checks if the value of the field is `_27`"]
    #[inline]
    pub fn is_27(&self) -> bool {
        *self == VDIVR::_27
    }
    #[doc = "Checks if the value of the field is `_28`"]
    #[inline]
    pub fn is_28(&self) -> bool {
        *self == VDIVR::_28
    }
    #[doc = "Checks if the value of the field is `_29`"]
    #[inline]
    pub fn is_29(&self) -> bool {
        *self == VDIVR::_29
    }
    #[doc = "Checks if the value of the field is `_30`"]
    #[inline]
    pub fn is_30(&self) -> bool {
        *self == VDIVR::_30
    }
    #[doc = "Checks if the value of the field is `_31`"]
    #[inline]
    pub fn is_31(&self) -> bool {
        *self == VDIVR::_31
    }
}
#[doc = "Possible values of the field `CME0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CME0R {
    #[doc = "External clock monitor is disabled for OSC0."]
    _0,
    #[doc = "External clock monitor is enabled for OSC0."]
    _1,
}
impl CME0R {
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
            CME0R::_0 => false,
            CME0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CME0R {
        match value {
            false => CME0R::_0,
            true => CME0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CME0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CME0R::_1
    }
}
#[doc = "Possible values of the field `PLLS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLSR {
    #[doc = "FLL is selected."]
    _0,
    #[doc = "PLLCS output clock is selected (PRDIV0 bits of PLL in the C5 register need to be programmed to the correct divider to generate a PLL reference clock in the range specified in the data sheet (fpll_ref) prior to setting the PLLS bit)."]
    _1,
}
impl PLLSR {
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
            PLLSR::_0 => false,
            PLLSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLLSR {
        match value {
            false => PLLSR::_0,
            true => PLLSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PLLSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PLLSR::_1
    }
}
#[doc = "Possible values of the field `LOLIE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOLIE0R {
    #[doc = "No interrupt request is generated on loss of lock."]
    _0,
    #[doc = "Generate an interrupt request on loss of lock."]
    _1,
}
impl LOLIE0R {
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
            LOLIE0R::_0 => false,
            LOLIE0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOLIE0R {
        match value {
            false => LOLIE0R::_0,
            true => LOLIE0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LOLIE0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LOLIE0R::_1
    }
}
#[doc = "Values that can be written to the field `VDIV`"]
pub enum VDIVW {
    #[doc = "Multiply Factor is 16"]
    _0,
    #[doc = "Multiply Factor is 17"]
    _1,
    #[doc = "Multiply Factor is 18"]
    _2,
    #[doc = "Multiply Factor is 19"]
    _3,
    #[doc = "Multiply Factor is 20"]
    _4,
    #[doc = "Multiply Factor is 21"]
    _5,
    #[doc = "Multiply Factor is 22"]
    _6,
    #[doc = "Multiply Factor is 23"]
    _7,
    #[doc = "Multiply Factor is 24"]
    _8,
    #[doc = "Multiply Factor is 25"]
    _9,
    #[doc = "Multiply Factor is 26"]
    _10,
    #[doc = "Multiply Factor is 27"]
    _11,
    #[doc = "Multiply Factor is 28"]
    _12,
    #[doc = "Multiply Factor is 29"]
    _13,
    #[doc = "Multiply Factor is 30"]
    _14,
    #[doc = "Multiply Factor is 31"]
    _15,
    #[doc = "Multiply Factor is 32"]
    _16,
    #[doc = "Multiply Factor is 33"]
    _17,
    #[doc = "Multiply Factor is 34"]
    _18,
    #[doc = "Multiply Factor is 35"]
    _19,
    #[doc = "Multiply Factor is 36"]
    _20,
    #[doc = "Multiply Factor is 37"]
    _21,
    #[doc = "Multiply Factor is 38"]
    _22,
    #[doc = "Multiply Factor is 39"]
    _23,
    #[doc = "Multiply Factor is 40"]
    _24,
    #[doc = "Multiply Factor is 41"]
    _25,
    #[doc = "Multiply Factor is 42"]
    _26,
    #[doc = "Multiply Factor is 43"]
    _27,
    #[doc = "Multiply Factor is 44"]
    _28,
    #[doc = "Multiply Factor is 45"]
    _29,
    #[doc = "Multiply Factor is 46"]
    _30,
    #[doc = "Multiply Factor is 47"]
    _31,
}
impl VDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            VDIVW::_0 => 0,
            VDIVW::_1 => 1,
            VDIVW::_2 => 2,
            VDIVW::_3 => 3,
            VDIVW::_4 => 4,
            VDIVW::_5 => 5,
            VDIVW::_6 => 6,
            VDIVW::_7 => 7,
            VDIVW::_8 => 8,
            VDIVW::_9 => 9,
            VDIVW::_10 => 10,
            VDIVW::_11 => 11,
            VDIVW::_12 => 12,
            VDIVW::_13 => 13,
            VDIVW::_14 => 14,
            VDIVW::_15 => 15,
            VDIVW::_16 => 16,
            VDIVW::_17 => 17,
            VDIVW::_18 => 18,
            VDIVW::_19 => 19,
            VDIVW::_20 => 20,
            VDIVW::_21 => 21,
            VDIVW::_22 => 22,
            VDIVW::_23 => 23,
            VDIVW::_24 => 24,
            VDIVW::_25 => 25,
            VDIVW::_26 => 26,
            VDIVW::_27 => 27,
            VDIVW::_28 => 28,
            VDIVW::_29 => 29,
            VDIVW::_30 => 30,
            VDIVW::_31 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _VDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VDIVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Multiply Factor is 16"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(VDIVW::_0)
    }
    #[doc = "Multiply Factor is 17"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(VDIVW::_1)
    }
    #[doc = "Multiply Factor is 18"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(VDIVW::_2)
    }
    #[doc = "Multiply Factor is 19"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(VDIVW::_3)
    }
    #[doc = "Multiply Factor is 20"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(VDIVW::_4)
    }
    #[doc = "Multiply Factor is 21"]
    #[inline]
    pub fn _5(self) -> &'a mut W {
        self.variant(VDIVW::_5)
    }
    #[doc = "Multiply Factor is 22"]
    #[inline]
    pub fn _6(self) -> &'a mut W {
        self.variant(VDIVW::_6)
    }
    #[doc = "Multiply Factor is 23"]
    #[inline]
    pub fn _7(self) -> &'a mut W {
        self.variant(VDIVW::_7)
    }
    #[doc = "Multiply Factor is 24"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(VDIVW::_8)
    }
    #[doc = "Multiply Factor is 25"]
    #[inline]
    pub fn _9(self) -> &'a mut W {
        self.variant(VDIVW::_9)
    }
    #[doc = "Multiply Factor is 26"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(VDIVW::_10)
    }
    #[doc = "Multiply Factor is 27"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(VDIVW::_11)
    }
    #[doc = "Multiply Factor is 28"]
    #[inline]
    pub fn _12(self) -> &'a mut W {
        self.variant(VDIVW::_12)
    }
    #[doc = "Multiply Factor is 29"]
    #[inline]
    pub fn _13(self) -> &'a mut W {
        self.variant(VDIVW::_13)
    }
    #[doc = "Multiply Factor is 30"]
    #[inline]
    pub fn _14(self) -> &'a mut W {
        self.variant(VDIVW::_14)
    }
    #[doc = "Multiply Factor is 31"]
    #[inline]
    pub fn _15(self) -> &'a mut W {
        self.variant(VDIVW::_15)
    }
    #[doc = "Multiply Factor is 32"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(VDIVW::_16)
    }
    #[doc = "Multiply Factor is 33"]
    #[inline]
    pub fn _17(self) -> &'a mut W {
        self.variant(VDIVW::_17)
    }
    #[doc = "Multiply Factor is 34"]
    #[inline]
    pub fn _18(self) -> &'a mut W {
        self.variant(VDIVW::_18)
    }
    #[doc = "Multiply Factor is 35"]
    #[inline]
    pub fn _19(self) -> &'a mut W {
        self.variant(VDIVW::_19)
    }
    #[doc = "Multiply Factor is 36"]
    #[inline]
    pub fn _20(self) -> &'a mut W {
        self.variant(VDIVW::_20)
    }
    #[doc = "Multiply Factor is 37"]
    #[inline]
    pub fn _21(self) -> &'a mut W {
        self.variant(VDIVW::_21)
    }
    #[doc = "Multiply Factor is 38"]
    #[inline]
    pub fn _22(self) -> &'a mut W {
        self.variant(VDIVW::_22)
    }
    #[doc = "Multiply Factor is 39"]
    #[inline]
    pub fn _23(self) -> &'a mut W {
        self.variant(VDIVW::_23)
    }
    #[doc = "Multiply Factor is 40"]
    #[inline]
    pub fn _24(self) -> &'a mut W {
        self.variant(VDIVW::_24)
    }
    #[doc = "Multiply Factor is 41"]
    #[inline]
    pub fn _25(self) -> &'a mut W {
        self.variant(VDIVW::_25)
    }
    #[doc = "Multiply Factor is 42"]
    #[inline]
    pub fn _26(self) -> &'a mut W {
        self.variant(VDIVW::_26)
    }
    #[doc = "Multiply Factor is 43"]
    #[inline]
    pub fn _27(self) -> &'a mut W {
        self.variant(VDIVW::_27)
    }
    #[doc = "Multiply Factor is 44"]
    #[inline]
    pub fn _28(self) -> &'a mut W {
        self.variant(VDIVW::_28)
    }
    #[doc = "Multiply Factor is 45"]
    #[inline]
    pub fn _29(self) -> &'a mut W {
        self.variant(VDIVW::_29)
    }
    #[doc = "Multiply Factor is 46"]
    #[inline]
    pub fn _30(self) -> &'a mut W {
        self.variant(VDIVW::_30)
    }
    #[doc = "Multiply Factor is 47"]
    #[inline]
    pub fn _31(self) -> &'a mut W {
        self.variant(VDIVW::_31)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CME0`"]
pub enum CME0W {
    #[doc = "External clock monitor is disabled for OSC0."]
    _0,
    #[doc = "External clock monitor is enabled for OSC0."]
    _1,
}
impl CME0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CME0W::_0 => false,
            CME0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CME0W<'a> {
    w: &'a mut W,
}
impl<'a> _CME0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CME0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External clock monitor is disabled for OSC0."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CME0W::_0)
    }
    #[doc = "External clock monitor is enabled for OSC0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CME0W::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLLS`"]
pub enum PLLSW {
    #[doc = "FLL is selected."]
    _0,
    #[doc = "PLLCS output clock is selected (PRDIV0 bits of PLL in the C5 register need to be programmed to the correct divider to generate a PLL reference clock in the range specified in the data sheet (fpll_ref) prior to setting the PLLS bit)."]
    _1,
}
impl PLLSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLLSW::_0 => false,
            PLLSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLLSW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FLL is selected."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLLSW::_0)
    }
    #[doc = "PLLCS output clock is selected (PRDIV0 bits of PLL in the C5 register need to be programmed to the correct divider to generate a PLL reference clock in the range specified in the data sheet (fpll_ref) prior to setting the PLLS bit)."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLLSW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LOLIE0`"]
pub enum LOLIE0W {
    #[doc = "No interrupt request is generated on loss of lock."]
    _0,
    #[doc = "Generate an interrupt request on loss of lock."]
    _1,
}
impl LOLIE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOLIE0W::_0 => false,
            LOLIE0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOLIE0W<'a> {
    w: &'a mut W,
}
impl<'a> _LOLIE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOLIE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt request is generated on loss of lock."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOLIE0W::_0)
    }
    #[doc = "Generate an interrupt request on loss of lock."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOLIE0W::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:4 - VCO Divider"]
    #[inline]
    pub fn vdiv(&self) -> VDIVR {
        VDIVR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 5 - Clock Monitor Enable"]
    #[inline]
    pub fn cme0(&self) -> CME0R {
        CME0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - PLL Select"]
    #[inline]
    pub fn plls(&self) -> PLLSR {
        PLLSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Loss of Lock Interrrupt Enable"]
    #[inline]
    pub fn lolie0(&self) -> LOLIE0R {
        LOLIE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - VCO Divider"]
    #[inline]
    pub fn vdiv(&mut self) -> _VDIVW {
        _VDIVW { w: self }
    }
    #[doc = "Bit 5 - Clock Monitor Enable"]
    #[inline]
    pub fn cme0(&mut self) -> _CME0W {
        _CME0W { w: self }
    }
    #[doc = "Bit 6 - PLL Select"]
    #[inline]
    pub fn plls(&mut self) -> _PLLSW {
        _PLLSW { w: self }
    }
    #[doc = "Bit 7 - Loss of Lock Interrrupt Enable"]
    #[inline]
    pub fn lolie0(&mut self) -> _LOLIE0W {
        _LOLIE0W { w: self }
    }
}
