#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::CHCFG {
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
#[doc = "Possible values of the field `SOURCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOURCER {
    #[doc = "Disable_Signal"]
    _0,
    #[doc = "TSI0_Signal"]
    _1,
    #[doc = "UART0_Rx_Signal"]
    _2,
    #[doc = "UART0_Tx_Signal"]
    _3,
    #[doc = "UART1_Rx_Signal"]
    _4,
    #[doc = "UART1_Tx_Signal"]
    _5,
    #[doc = "UART2_Rx_Signal"]
    _6,
    #[doc = "UART2_Tx_Signal"]
    _7,
    #[doc = "UART3_Rx_Signal"]
    _8,
    #[doc = "UART3_Tx_Signal"]
    _9,
    #[doc = "UART4_Signal"]
    _10,
    #[doc = "I2S0_Rx_Signal"]
    _12,
    #[doc = "I2S0_Tx_Signal"]
    _13,
    #[doc = "SPI0_Rx_Signal"]
    _14,
    #[doc = "SPI0_Tx_Signal"]
    _15,
    #[doc = "SPI1_Rx_Signal"]
    _16,
    #[doc = "SPI1_Tx_Signal"]
    _17,
    #[doc = "I2C0_I2C3_Signal"]
    _18,
    #[doc = "I2C1_I2C2_Signal"]
    _19,
    #[doc = "FTM0_Channel0_Signal"]
    _20,
    #[doc = "FTM0_Channel1_Signal"]
    _21,
    #[doc = "FTM0_Channel2_Signal"]
    _22,
    #[doc = "FTM0_Channel3_Signal"]
    _23,
    #[doc = "FTM0_Channel4_Signal"]
    _24,
    #[doc = "FTM0_Channel5_Signal"]
    _25,
    #[doc = "FTM0_Channel6_Signal"]
    _26,
    #[doc = "FTM0_Channel7_Signal"]
    _27,
    #[doc = "FTM1_TPM1_Channel0_Signal"]
    _28,
    #[doc = "FTM1_TPM1_Channel1_Signal"]
    _29,
    #[doc = "FTM2_TPM2_Channel0_Signal"]
    _30,
    #[doc = "FTM2_TPM2_Channel1_Signal"]
    _31,
    #[doc = "FTM3_Channel0_Signal"]
    _32,
    #[doc = "FTM3_Channel1_Signal"]
    _33,
    #[doc = "FTM3_Channel2_Signal"]
    _34,
    #[doc = "FTM3_Channel3_Signal"]
    _35,
    #[doc = "FTM3_Channel4_Signal"]
    _36,
    #[doc = "FTM3_Channel5_Signal"]
    _37,
    #[doc = "FTM3_Channel6_SPI2_Rx_Signal"]
    _38,
    #[doc = "FTM3_Channel7_SPI2_Tx_Signal"]
    _39,
    #[doc = "ADC0_Signal"]
    _40,
    #[doc = "ADC1_Signal"]
    _41,
    #[doc = "CMP0_Signal"]
    _42,
    #[doc = "CMP1_Signal"]
    _43,
    #[doc = "CMP2_CMP3_Signal"]
    _44,
    #[doc = "DAC0_Signal"]
    _45,
    #[doc = "DAC1_Signal"]
    _46,
    #[doc = "CMT_Signal"]
    _47,
    #[doc = "PDB_Signal"]
    _48,
    #[doc = "PortA_Signal"]
    _49,
    #[doc = "PortB_Signal"]
    _50,
    #[doc = "PortC_Signal"]
    _51,
    #[doc = "PortD_Signal"]
    _52,
    #[doc = "PortE_Signal"]
    _53,
    #[doc = "IEEE1588Timer0_Signal"]
    _54,
    #[doc = "IEEE1588Timer1_TPM1_Overflow_Signal"]
    _55,
    #[doc = "IEEE1588Timer2_TPM2_Overflow_Signal"]
    _56,
    #[doc = "IEEE1588Timer3_Signal"]
    _57,
    #[doc = "LPUART0_Rx_Signal"]
    _58,
    #[doc = "LPUART0_Tx_Signal"]
    _59,
    #[doc = "AlwaysOn60_Signal"]
    _60,
    #[doc = "AlwaysOn61_Signal"]
    _61,
    #[doc = "AlwaysOn62_Signal"]
    _62,
    #[doc = "AlwaysOn63_Signal"]
    _63,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SOURCER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SOURCER::_0 => 0,
            SOURCER::_1 => 1,
            SOURCER::_2 => 2,
            SOURCER::_3 => 3,
            SOURCER::_4 => 4,
            SOURCER::_5 => 5,
            SOURCER::_6 => 6,
            SOURCER::_7 => 7,
            SOURCER::_8 => 8,
            SOURCER::_9 => 9,
            SOURCER::_10 => 10,
            SOURCER::_12 => 12,
            SOURCER::_13 => 13,
            SOURCER::_14 => 14,
            SOURCER::_15 => 15,
            SOURCER::_16 => 16,
            SOURCER::_17 => 17,
            SOURCER::_18 => 18,
            SOURCER::_19 => 19,
            SOURCER::_20 => 20,
            SOURCER::_21 => 21,
            SOURCER::_22 => 22,
            SOURCER::_23 => 23,
            SOURCER::_24 => 24,
            SOURCER::_25 => 25,
            SOURCER::_26 => 26,
            SOURCER::_27 => 27,
            SOURCER::_28 => 28,
            SOURCER::_29 => 29,
            SOURCER::_30 => 30,
            SOURCER::_31 => 31,
            SOURCER::_32 => 32,
            SOURCER::_33 => 33,
            SOURCER::_34 => 34,
            SOURCER::_35 => 35,
            SOURCER::_36 => 36,
            SOURCER::_37 => 37,
            SOURCER::_38 => 38,
            SOURCER::_39 => 39,
            SOURCER::_40 => 40,
            SOURCER::_41 => 41,
            SOURCER::_42 => 42,
            SOURCER::_43 => 43,
            SOURCER::_44 => 44,
            SOURCER::_45 => 45,
            SOURCER::_46 => 46,
            SOURCER::_47 => 47,
            SOURCER::_48 => 48,
            SOURCER::_49 => 49,
            SOURCER::_50 => 50,
            SOURCER::_51 => 51,
            SOURCER::_52 => 52,
            SOURCER::_53 => 53,
            SOURCER::_54 => 54,
            SOURCER::_55 => 55,
            SOURCER::_56 => 56,
            SOURCER::_57 => 57,
            SOURCER::_58 => 58,
            SOURCER::_59 => 59,
            SOURCER::_60 => 60,
            SOURCER::_61 => 61,
            SOURCER::_62 => 62,
            SOURCER::_63 => 63,
            SOURCER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SOURCER {
        match value {
            0 => SOURCER::_0,
            1 => SOURCER::_1,
            2 => SOURCER::_2,
            3 => SOURCER::_3,
            4 => SOURCER::_4,
            5 => SOURCER::_5,
            6 => SOURCER::_6,
            7 => SOURCER::_7,
            8 => SOURCER::_8,
            9 => SOURCER::_9,
            10 => SOURCER::_10,
            12 => SOURCER::_12,
            13 => SOURCER::_13,
            14 => SOURCER::_14,
            15 => SOURCER::_15,
            16 => SOURCER::_16,
            17 => SOURCER::_17,
            18 => SOURCER::_18,
            19 => SOURCER::_19,
            20 => SOURCER::_20,
            21 => SOURCER::_21,
            22 => SOURCER::_22,
            23 => SOURCER::_23,
            24 => SOURCER::_24,
            25 => SOURCER::_25,
            26 => SOURCER::_26,
            27 => SOURCER::_27,
            28 => SOURCER::_28,
            29 => SOURCER::_29,
            30 => SOURCER::_30,
            31 => SOURCER::_31,
            32 => SOURCER::_32,
            33 => SOURCER::_33,
            34 => SOURCER::_34,
            35 => SOURCER::_35,
            36 => SOURCER::_36,
            37 => SOURCER::_37,
            38 => SOURCER::_38,
            39 => SOURCER::_39,
            40 => SOURCER::_40,
            41 => SOURCER::_41,
            42 => SOURCER::_42,
            43 => SOURCER::_43,
            44 => SOURCER::_44,
            45 => SOURCER::_45,
            46 => SOURCER::_46,
            47 => SOURCER::_47,
            48 => SOURCER::_48,
            49 => SOURCER::_49,
            50 => SOURCER::_50,
            51 => SOURCER::_51,
            52 => SOURCER::_52,
            53 => SOURCER::_53,
            54 => SOURCER::_54,
            55 => SOURCER::_55,
            56 => SOURCER::_56,
            57 => SOURCER::_57,
            58 => SOURCER::_58,
            59 => SOURCER::_59,
            60 => SOURCER::_60,
            61 => SOURCER::_61,
            62 => SOURCER::_62,
            63 => SOURCER::_63,
            i => SOURCER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SOURCER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SOURCER::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == SOURCER::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == SOURCER::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == SOURCER::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline]
    pub fn is_5(&self) -> bool {
        *self == SOURCER::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline]
    pub fn is_6(&self) -> bool {
        *self == SOURCER::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline]
    pub fn is_7(&self) -> bool {
        *self == SOURCER::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == SOURCER::_8
    }
    #[doc = "Checks if the value of the field is `_9`"]
    #[inline]
    pub fn is_9(&self) -> bool {
        *self == SOURCER::_9
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == SOURCER::_10
    }
    #[doc = "Checks if the value of the field is `_12`"]
    #[inline]
    pub fn is_12(&self) -> bool {
        *self == SOURCER::_12
    }
    #[doc = "Checks if the value of the field is `_13`"]
    #[inline]
    pub fn is_13(&self) -> bool {
        *self == SOURCER::_13
    }
    #[doc = "Checks if the value of the field is `_14`"]
    #[inline]
    pub fn is_14(&self) -> bool {
        *self == SOURCER::_14
    }
    #[doc = "Checks if the value of the field is `_15`"]
    #[inline]
    pub fn is_15(&self) -> bool {
        *self == SOURCER::_15
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == SOURCER::_16
    }
    #[doc = "Checks if the value of the field is `_17`"]
    #[inline]
    pub fn is_17(&self) -> bool {
        *self == SOURCER::_17
    }
    #[doc = "Checks if the value of the field is `_18`"]
    #[inline]
    pub fn is_18(&self) -> bool {
        *self == SOURCER::_18
    }
    #[doc = "Checks if the value of the field is `_19`"]
    #[inline]
    pub fn is_19(&self) -> bool {
        *self == SOURCER::_19
    }
    #[doc = "Checks if the value of the field is `_20`"]
    #[inline]
    pub fn is_20(&self) -> bool {
        *self == SOURCER::_20
    }
    #[doc = "Checks if the value of the field is `_21`"]
    #[inline]
    pub fn is_21(&self) -> bool {
        *self == SOURCER::_21
    }
    #[doc = "Checks if the value of the field is `_22`"]
    #[inline]
    pub fn is_22(&self) -> bool {
        *self == SOURCER::_22
    }
    #[doc = "Checks if the value of the field is `_23`"]
    #[inline]
    pub fn is_23(&self) -> bool {
        *self == SOURCER::_23
    }
    #[doc = "Checks if the value of the field is `_24`"]
    #[inline]
    pub fn is_24(&self) -> bool {
        *self == SOURCER::_24
    }
    #[doc = "Checks if the value of the field is `_25`"]
    #[inline]
    pub fn is_25(&self) -> bool {
        *self == SOURCER::_25
    }
    #[doc = "Checks if the value of the field is `_26`"]
    #[inline]
    pub fn is_26(&self) -> bool {
        *self == SOURCER::_26
    }
    #[doc = "Checks if the value of the field is `_27`"]
    #[inline]
    pub fn is_27(&self) -> bool {
        *self == SOURCER::_27
    }
    #[doc = "Checks if the value of the field is `_28`"]
    #[inline]
    pub fn is_28(&self) -> bool {
        *self == SOURCER::_28
    }
    #[doc = "Checks if the value of the field is `_29`"]
    #[inline]
    pub fn is_29(&self) -> bool {
        *self == SOURCER::_29
    }
    #[doc = "Checks if the value of the field is `_30`"]
    #[inline]
    pub fn is_30(&self) -> bool {
        *self == SOURCER::_30
    }
    #[doc = "Checks if the value of the field is `_31`"]
    #[inline]
    pub fn is_31(&self) -> bool {
        *self == SOURCER::_31
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline]
    pub fn is_32(&self) -> bool {
        *self == SOURCER::_32
    }
    #[doc = "Checks if the value of the field is `_33`"]
    #[inline]
    pub fn is_33(&self) -> bool {
        *self == SOURCER::_33
    }
    #[doc = "Checks if the value of the field is `_34`"]
    #[inline]
    pub fn is_34(&self) -> bool {
        *self == SOURCER::_34
    }
    #[doc = "Checks if the value of the field is `_35`"]
    #[inline]
    pub fn is_35(&self) -> bool {
        *self == SOURCER::_35
    }
    #[doc = "Checks if the value of the field is `_36`"]
    #[inline]
    pub fn is_36(&self) -> bool {
        *self == SOURCER::_36
    }
    #[doc = "Checks if the value of the field is `_37`"]
    #[inline]
    pub fn is_37(&self) -> bool {
        *self == SOURCER::_37
    }
    #[doc = "Checks if the value of the field is `_38`"]
    #[inline]
    pub fn is_38(&self) -> bool {
        *self == SOURCER::_38
    }
    #[doc = "Checks if the value of the field is `_39`"]
    #[inline]
    pub fn is_39(&self) -> bool {
        *self == SOURCER::_39
    }
    #[doc = "Checks if the value of the field is `_40`"]
    #[inline]
    pub fn is_40(&self) -> bool {
        *self == SOURCER::_40
    }
    #[doc = "Checks if the value of the field is `_41`"]
    #[inline]
    pub fn is_41(&self) -> bool {
        *self == SOURCER::_41
    }
    #[doc = "Checks if the value of the field is `_42`"]
    #[inline]
    pub fn is_42(&self) -> bool {
        *self == SOURCER::_42
    }
    #[doc = "Checks if the value of the field is `_43`"]
    #[inline]
    pub fn is_43(&self) -> bool {
        *self == SOURCER::_43
    }
    #[doc = "Checks if the value of the field is `_44`"]
    #[inline]
    pub fn is_44(&self) -> bool {
        *self == SOURCER::_44
    }
    #[doc = "Checks if the value of the field is `_45`"]
    #[inline]
    pub fn is_45(&self) -> bool {
        *self == SOURCER::_45
    }
    #[doc = "Checks if the value of the field is `_46`"]
    #[inline]
    pub fn is_46(&self) -> bool {
        *self == SOURCER::_46
    }
    #[doc = "Checks if the value of the field is `_47`"]
    #[inline]
    pub fn is_47(&self) -> bool {
        *self == SOURCER::_47
    }
    #[doc = "Checks if the value of the field is `_48`"]
    #[inline]
    pub fn is_48(&self) -> bool {
        *self == SOURCER::_48
    }
    #[doc = "Checks if the value of the field is `_49`"]
    #[inline]
    pub fn is_49(&self) -> bool {
        *self == SOURCER::_49
    }
    #[doc = "Checks if the value of the field is `_50`"]
    #[inline]
    pub fn is_50(&self) -> bool {
        *self == SOURCER::_50
    }
    #[doc = "Checks if the value of the field is `_51`"]
    #[inline]
    pub fn is_51(&self) -> bool {
        *self == SOURCER::_51
    }
    #[doc = "Checks if the value of the field is `_52`"]
    #[inline]
    pub fn is_52(&self) -> bool {
        *self == SOURCER::_52
    }
    #[doc = "Checks if the value of the field is `_53`"]
    #[inline]
    pub fn is_53(&self) -> bool {
        *self == SOURCER::_53
    }
    #[doc = "Checks if the value of the field is `_54`"]
    #[inline]
    pub fn is_54(&self) -> bool {
        *self == SOURCER::_54
    }
    #[doc = "Checks if the value of the field is `_55`"]
    #[inline]
    pub fn is_55(&self) -> bool {
        *self == SOURCER::_55
    }
    #[doc = "Checks if the value of the field is `_56`"]
    #[inline]
    pub fn is_56(&self) -> bool {
        *self == SOURCER::_56
    }
    #[doc = "Checks if the value of the field is `_57`"]
    #[inline]
    pub fn is_57(&self) -> bool {
        *self == SOURCER::_57
    }
    #[doc = "Checks if the value of the field is `_58`"]
    #[inline]
    pub fn is_58(&self) -> bool {
        *self == SOURCER::_58
    }
    #[doc = "Checks if the value of the field is `_59`"]
    #[inline]
    pub fn is_59(&self) -> bool {
        *self == SOURCER::_59
    }
    #[doc = "Checks if the value of the field is `_60`"]
    #[inline]
    pub fn is_60(&self) -> bool {
        *self == SOURCER::_60
    }
    #[doc = "Checks if the value of the field is `_61`"]
    #[inline]
    pub fn is_61(&self) -> bool {
        *self == SOURCER::_61
    }
    #[doc = "Checks if the value of the field is `_62`"]
    #[inline]
    pub fn is_62(&self) -> bool {
        *self == SOURCER::_62
    }
    #[doc = "Checks if the value of the field is `_63`"]
    #[inline]
    pub fn is_63(&self) -> bool {
        *self == SOURCER::_63
    }
}
#[doc = "Possible values of the field `TRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGR {
    #[doc = "Triggering is disabled. If triggering is disabled and ENBL is set, the DMA Channel will simply route the specified source to the DMA channel. (Normal mode)"]
    _0,
    #[doc = "Triggering is enabled. If triggering is enabled and ENBL is set, the DMAMUX is in Periodic Trigger mode."]
    _1,
}
impl TRIGR {
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
            TRIGR::_0 => false,
            TRIGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIGR {
        match value {
            false => TRIGR::_0,
            true => TRIGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TRIGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TRIGR::_1
    }
}
#[doc = "Possible values of the field `ENBL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENBLR {
    #[doc = "DMA channel is disabled. This mode is primarily used during configuration of the DMAMux. The DMA has separate channel enables/disables, which should be used to disable or reconfigure a DMA channel."]
    _0,
    #[doc = "DMA channel is enabled"]
    _1,
}
impl ENBLR {
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
            ENBLR::_0 => false,
            ENBLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENBLR {
        match value {
            false => ENBLR::_0,
            true => ENBLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ENBLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ENBLR::_1
    }
}
#[doc = "Values that can be written to the field `SOURCE`"]
pub enum SOURCEW {
    #[doc = "Disable_Signal"]
    _0,
    #[doc = "TSI0_Signal"]
    _1,
    #[doc = "UART0_Rx_Signal"]
    _2,
    #[doc = "UART0_Tx_Signal"]
    _3,
    #[doc = "UART1_Rx_Signal"]
    _4,
    #[doc = "UART1_Tx_Signal"]
    _5,
    #[doc = "UART2_Rx_Signal"]
    _6,
    #[doc = "UART2_Tx_Signal"]
    _7,
    #[doc = "UART3_Rx_Signal"]
    _8,
    #[doc = "UART3_Tx_Signal"]
    _9,
    #[doc = "UART4_Signal"]
    _10,
    #[doc = "I2S0_Rx_Signal"]
    _12,
    #[doc = "I2S0_Tx_Signal"]
    _13,
    #[doc = "SPI0_Rx_Signal"]
    _14,
    #[doc = "SPI0_Tx_Signal"]
    _15,
    #[doc = "SPI1_Rx_Signal"]
    _16,
    #[doc = "SPI1_Tx_Signal"]
    _17,
    #[doc = "I2C0_I2C3_Signal"]
    _18,
    #[doc = "I2C1_I2C2_Signal"]
    _19,
    #[doc = "FTM0_Channel0_Signal"]
    _20,
    #[doc = "FTM0_Channel1_Signal"]
    _21,
    #[doc = "FTM0_Channel2_Signal"]
    _22,
    #[doc = "FTM0_Channel3_Signal"]
    _23,
    #[doc = "FTM0_Channel4_Signal"]
    _24,
    #[doc = "FTM0_Channel5_Signal"]
    _25,
    #[doc = "FTM0_Channel6_Signal"]
    _26,
    #[doc = "FTM0_Channel7_Signal"]
    _27,
    #[doc = "FTM1_TPM1_Channel0_Signal"]
    _28,
    #[doc = "FTM1_TPM1_Channel1_Signal"]
    _29,
    #[doc = "FTM2_TPM2_Channel0_Signal"]
    _30,
    #[doc = "FTM2_TPM2_Channel1_Signal"]
    _31,
    #[doc = "FTM3_Channel0_Signal"]
    _32,
    #[doc = "FTM3_Channel1_Signal"]
    _33,
    #[doc = "FTM3_Channel2_Signal"]
    _34,
    #[doc = "FTM3_Channel3_Signal"]
    _35,
    #[doc = "FTM3_Channel4_Signal"]
    _36,
    #[doc = "FTM3_Channel5_Signal"]
    _37,
    #[doc = "FTM3_Channel6_SPI2_Rx_Signal"]
    _38,
    #[doc = "FTM3_Channel7_SPI2_Tx_Signal"]
    _39,
    #[doc = "ADC0_Signal"]
    _40,
    #[doc = "ADC1_Signal"]
    _41,
    #[doc = "CMP0_Signal"]
    _42,
    #[doc = "CMP1_Signal"]
    _43,
    #[doc = "CMP2_CMP3_Signal"]
    _44,
    #[doc = "DAC0_Signal"]
    _45,
    #[doc = "DAC1_Signal"]
    _46,
    #[doc = "CMT_Signal"]
    _47,
    #[doc = "PDB_Signal"]
    _48,
    #[doc = "PortA_Signal"]
    _49,
    #[doc = "PortB_Signal"]
    _50,
    #[doc = "PortC_Signal"]
    _51,
    #[doc = "PortD_Signal"]
    _52,
    #[doc = "PortE_Signal"]
    _53,
    #[doc = "IEEE1588Timer0_Signal"]
    _54,
    #[doc = "IEEE1588Timer1_TPM1_Overflow_Signal"]
    _55,
    #[doc = "IEEE1588Timer2_TPM2_Overflow_Signal"]
    _56,
    #[doc = "IEEE1588Timer3_Signal"]
    _57,
    #[doc = "LPUART0_Rx_Signal"]
    _58,
    #[doc = "LPUART0_Tx_Signal"]
    _59,
    #[doc = "AlwaysOn60_Signal"]
    _60,
    #[doc = "AlwaysOn61_Signal"]
    _61,
    #[doc = "AlwaysOn62_Signal"]
    _62,
    #[doc = "AlwaysOn63_Signal"]
    _63,
}
impl SOURCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SOURCEW::_0 => 0,
            SOURCEW::_1 => 1,
            SOURCEW::_2 => 2,
            SOURCEW::_3 => 3,
            SOURCEW::_4 => 4,
            SOURCEW::_5 => 5,
            SOURCEW::_6 => 6,
            SOURCEW::_7 => 7,
            SOURCEW::_8 => 8,
            SOURCEW::_9 => 9,
            SOURCEW::_10 => 10,
            SOURCEW::_12 => 12,
            SOURCEW::_13 => 13,
            SOURCEW::_14 => 14,
            SOURCEW::_15 => 15,
            SOURCEW::_16 => 16,
            SOURCEW::_17 => 17,
            SOURCEW::_18 => 18,
            SOURCEW::_19 => 19,
            SOURCEW::_20 => 20,
            SOURCEW::_21 => 21,
            SOURCEW::_22 => 22,
            SOURCEW::_23 => 23,
            SOURCEW::_24 => 24,
            SOURCEW::_25 => 25,
            SOURCEW::_26 => 26,
            SOURCEW::_27 => 27,
            SOURCEW::_28 => 28,
            SOURCEW::_29 => 29,
            SOURCEW::_30 => 30,
            SOURCEW::_31 => 31,
            SOURCEW::_32 => 32,
            SOURCEW::_33 => 33,
            SOURCEW::_34 => 34,
            SOURCEW::_35 => 35,
            SOURCEW::_36 => 36,
            SOURCEW::_37 => 37,
            SOURCEW::_38 => 38,
            SOURCEW::_39 => 39,
            SOURCEW::_40 => 40,
            SOURCEW::_41 => 41,
            SOURCEW::_42 => 42,
            SOURCEW::_43 => 43,
            SOURCEW::_44 => 44,
            SOURCEW::_45 => 45,
            SOURCEW::_46 => 46,
            SOURCEW::_47 => 47,
            SOURCEW::_48 => 48,
            SOURCEW::_49 => 49,
            SOURCEW::_50 => 50,
            SOURCEW::_51 => 51,
            SOURCEW::_52 => 52,
            SOURCEW::_53 => 53,
            SOURCEW::_54 => 54,
            SOURCEW::_55 => 55,
            SOURCEW::_56 => 56,
            SOURCEW::_57 => 57,
            SOURCEW::_58 => 58,
            SOURCEW::_59 => 59,
            SOURCEW::_60 => 60,
            SOURCEW::_61 => 61,
            SOURCEW::_62 => 62,
            SOURCEW::_63 => 63,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SOURCEW<'a> {
    w: &'a mut W,
}
impl<'a> _SOURCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SOURCEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disable_Signal"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOURCEW::_0)
    }
    #[doc = "TSI0_Signal"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOURCEW::_1)
    }
    #[doc = "UART0_Rx_Signal"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(SOURCEW::_2)
    }
    #[doc = "UART0_Tx_Signal"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(SOURCEW::_3)
    }
    #[doc = "UART1_Rx_Signal"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(SOURCEW::_4)
    }
    #[doc = "UART1_Tx_Signal"]
    #[inline]
    pub fn _5(self) -> &'a mut W {
        self.variant(SOURCEW::_5)
    }
    #[doc = "UART2_Rx_Signal"]
    #[inline]
    pub fn _6(self) -> &'a mut W {
        self.variant(SOURCEW::_6)
    }
    #[doc = "UART2_Tx_Signal"]
    #[inline]
    pub fn _7(self) -> &'a mut W {
        self.variant(SOURCEW::_7)
    }
    #[doc = "UART3_Rx_Signal"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(SOURCEW::_8)
    }
    #[doc = "UART3_Tx_Signal"]
    #[inline]
    pub fn _9(self) -> &'a mut W {
        self.variant(SOURCEW::_9)
    }
    #[doc = "UART4_Signal"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(SOURCEW::_10)
    }
    #[doc = "I2S0_Rx_Signal"]
    #[inline]
    pub fn _12(self) -> &'a mut W {
        self.variant(SOURCEW::_12)
    }
    #[doc = "I2S0_Tx_Signal"]
    #[inline]
    pub fn _13(self) -> &'a mut W {
        self.variant(SOURCEW::_13)
    }
    #[doc = "SPI0_Rx_Signal"]
    #[inline]
    pub fn _14(self) -> &'a mut W {
        self.variant(SOURCEW::_14)
    }
    #[doc = "SPI0_Tx_Signal"]
    #[inline]
    pub fn _15(self) -> &'a mut W {
        self.variant(SOURCEW::_15)
    }
    #[doc = "SPI1_Rx_Signal"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(SOURCEW::_16)
    }
    #[doc = "SPI1_Tx_Signal"]
    #[inline]
    pub fn _17(self) -> &'a mut W {
        self.variant(SOURCEW::_17)
    }
    #[doc = "I2C0_I2C3_Signal"]
    #[inline]
    pub fn _18(self) -> &'a mut W {
        self.variant(SOURCEW::_18)
    }
    #[doc = "I2C1_I2C2_Signal"]
    #[inline]
    pub fn _19(self) -> &'a mut W {
        self.variant(SOURCEW::_19)
    }
    #[doc = "FTM0_Channel0_Signal"]
    #[inline]
    pub fn _20(self) -> &'a mut W {
        self.variant(SOURCEW::_20)
    }
    #[doc = "FTM0_Channel1_Signal"]
    #[inline]
    pub fn _21(self) -> &'a mut W {
        self.variant(SOURCEW::_21)
    }
    #[doc = "FTM0_Channel2_Signal"]
    #[inline]
    pub fn _22(self) -> &'a mut W {
        self.variant(SOURCEW::_22)
    }
    #[doc = "FTM0_Channel3_Signal"]
    #[inline]
    pub fn _23(self) -> &'a mut W {
        self.variant(SOURCEW::_23)
    }
    #[doc = "FTM0_Channel4_Signal"]
    #[inline]
    pub fn _24(self) -> &'a mut W {
        self.variant(SOURCEW::_24)
    }
    #[doc = "FTM0_Channel5_Signal"]
    #[inline]
    pub fn _25(self) -> &'a mut W {
        self.variant(SOURCEW::_25)
    }
    #[doc = "FTM0_Channel6_Signal"]
    #[inline]
    pub fn _26(self) -> &'a mut W {
        self.variant(SOURCEW::_26)
    }
    #[doc = "FTM0_Channel7_Signal"]
    #[inline]
    pub fn _27(self) -> &'a mut W {
        self.variant(SOURCEW::_27)
    }
    #[doc = "FTM1_TPM1_Channel0_Signal"]
    #[inline]
    pub fn _28(self) -> &'a mut W {
        self.variant(SOURCEW::_28)
    }
    #[doc = "FTM1_TPM1_Channel1_Signal"]
    #[inline]
    pub fn _29(self) -> &'a mut W {
        self.variant(SOURCEW::_29)
    }
    #[doc = "FTM2_TPM2_Channel0_Signal"]
    #[inline]
    pub fn _30(self) -> &'a mut W {
        self.variant(SOURCEW::_30)
    }
    #[doc = "FTM2_TPM2_Channel1_Signal"]
    #[inline]
    pub fn _31(self) -> &'a mut W {
        self.variant(SOURCEW::_31)
    }
    #[doc = "FTM3_Channel0_Signal"]
    #[inline]
    pub fn _32(self) -> &'a mut W {
        self.variant(SOURCEW::_32)
    }
    #[doc = "FTM3_Channel1_Signal"]
    #[inline]
    pub fn _33(self) -> &'a mut W {
        self.variant(SOURCEW::_33)
    }
    #[doc = "FTM3_Channel2_Signal"]
    #[inline]
    pub fn _34(self) -> &'a mut W {
        self.variant(SOURCEW::_34)
    }
    #[doc = "FTM3_Channel3_Signal"]
    #[inline]
    pub fn _35(self) -> &'a mut W {
        self.variant(SOURCEW::_35)
    }
    #[doc = "FTM3_Channel4_Signal"]
    #[inline]
    pub fn _36(self) -> &'a mut W {
        self.variant(SOURCEW::_36)
    }
    #[doc = "FTM3_Channel5_Signal"]
    #[inline]
    pub fn _37(self) -> &'a mut W {
        self.variant(SOURCEW::_37)
    }
    #[doc = "FTM3_Channel6_SPI2_Rx_Signal"]
    #[inline]
    pub fn _38(self) -> &'a mut W {
        self.variant(SOURCEW::_38)
    }
    #[doc = "FTM3_Channel7_SPI2_Tx_Signal"]
    #[inline]
    pub fn _39(self) -> &'a mut W {
        self.variant(SOURCEW::_39)
    }
    #[doc = "ADC0_Signal"]
    #[inline]
    pub fn _40(self) -> &'a mut W {
        self.variant(SOURCEW::_40)
    }
    #[doc = "ADC1_Signal"]
    #[inline]
    pub fn _41(self) -> &'a mut W {
        self.variant(SOURCEW::_41)
    }
    #[doc = "CMP0_Signal"]
    #[inline]
    pub fn _42(self) -> &'a mut W {
        self.variant(SOURCEW::_42)
    }
    #[doc = "CMP1_Signal"]
    #[inline]
    pub fn _43(self) -> &'a mut W {
        self.variant(SOURCEW::_43)
    }
    #[doc = "CMP2_CMP3_Signal"]
    #[inline]
    pub fn _44(self) -> &'a mut W {
        self.variant(SOURCEW::_44)
    }
    #[doc = "DAC0_Signal"]
    #[inline]
    pub fn _45(self) -> &'a mut W {
        self.variant(SOURCEW::_45)
    }
    #[doc = "DAC1_Signal"]
    #[inline]
    pub fn _46(self) -> &'a mut W {
        self.variant(SOURCEW::_46)
    }
    #[doc = "CMT_Signal"]
    #[inline]
    pub fn _47(self) -> &'a mut W {
        self.variant(SOURCEW::_47)
    }
    #[doc = "PDB_Signal"]
    #[inline]
    pub fn _48(self) -> &'a mut W {
        self.variant(SOURCEW::_48)
    }
    #[doc = "PortA_Signal"]
    #[inline]
    pub fn _49(self) -> &'a mut W {
        self.variant(SOURCEW::_49)
    }
    #[doc = "PortB_Signal"]
    #[inline]
    pub fn _50(self) -> &'a mut W {
        self.variant(SOURCEW::_50)
    }
    #[doc = "PortC_Signal"]
    #[inline]
    pub fn _51(self) -> &'a mut W {
        self.variant(SOURCEW::_51)
    }
    #[doc = "PortD_Signal"]
    #[inline]
    pub fn _52(self) -> &'a mut W {
        self.variant(SOURCEW::_52)
    }
    #[doc = "PortE_Signal"]
    #[inline]
    pub fn _53(self) -> &'a mut W {
        self.variant(SOURCEW::_53)
    }
    #[doc = "IEEE1588Timer0_Signal"]
    #[inline]
    pub fn _54(self) -> &'a mut W {
        self.variant(SOURCEW::_54)
    }
    #[doc = "IEEE1588Timer1_TPM1_Overflow_Signal"]
    #[inline]
    pub fn _55(self) -> &'a mut W {
        self.variant(SOURCEW::_55)
    }
    #[doc = "IEEE1588Timer2_TPM2_Overflow_Signal"]
    #[inline]
    pub fn _56(self) -> &'a mut W {
        self.variant(SOURCEW::_56)
    }
    #[doc = "IEEE1588Timer3_Signal"]
    #[inline]
    pub fn _57(self) -> &'a mut W {
        self.variant(SOURCEW::_57)
    }
    #[doc = "LPUART0_Rx_Signal"]
    #[inline]
    pub fn _58(self) -> &'a mut W {
        self.variant(SOURCEW::_58)
    }
    #[doc = "LPUART0_Tx_Signal"]
    #[inline]
    pub fn _59(self) -> &'a mut W {
        self.variant(SOURCEW::_59)
    }
    #[doc = "AlwaysOn60_Signal"]
    #[inline]
    pub fn _60(self) -> &'a mut W {
        self.variant(SOURCEW::_60)
    }
    #[doc = "AlwaysOn61_Signal"]
    #[inline]
    pub fn _61(self) -> &'a mut W {
        self.variant(SOURCEW::_61)
    }
    #[doc = "AlwaysOn62_Signal"]
    #[inline]
    pub fn _62(self) -> &'a mut W {
        self.variant(SOURCEW::_62)
    }
    #[doc = "AlwaysOn63_Signal"]
    #[inline]
    pub fn _63(self) -> &'a mut W {
        self.variant(SOURCEW::_63)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRIG`"]
pub enum TRIGW {
    #[doc = "Triggering is disabled. If triggering is disabled and ENBL is set, the DMA Channel will simply route the specified source to the DMA channel. (Normal mode)"]
    _0,
    #[doc = "Triggering is enabled. If triggering is enabled and ENBL is set, the DMAMUX is in Periodic Trigger mode."]
    _1,
}
impl TRIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIGW::_0 => false,
            TRIGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Triggering is disabled. If triggering is disabled and ENBL is set, the DMA Channel will simply route the specified source to the DMA channel. (Normal mode)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRIGW::_0)
    }
    #[doc = "Triggering is enabled. If triggering is enabled and ENBL is set, the DMAMUX is in Periodic Trigger mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRIGW::_1)
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
#[doc = "Values that can be written to the field `ENBL`"]
pub enum ENBLW {
    #[doc = "DMA channel is disabled. This mode is primarily used during configuration of the DMAMux. The DMA has separate channel enables/disables, which should be used to disable or reconfigure a DMA channel."]
    _0,
    #[doc = "DMA channel is enabled"]
    _1,
}
impl ENBLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENBLW::_0 => false,
            ENBLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENBLW<'a> {
    w: &'a mut W,
}
impl<'a> _ENBLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENBLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA channel is disabled. This mode is primarily used during configuration of the DMAMux. The DMA has separate channel enables/disables, which should be used to disable or reconfigure a DMA channel."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENBLW::_0)
    }
    #[doc = "DMA channel is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENBLW::_1)
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
    #[doc = "Bits 0:5 - DMA Channel Source (Slot)"]
    #[inline]
    pub fn source(&self) -> SOURCER {
        SOURCER::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 6 - DMA Channel Trigger Enable"]
    #[inline]
    pub fn trig(&self) -> TRIGR {
        TRIGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - DMA Channel Enable"]
    #[inline]
    pub fn enbl(&self) -> ENBLR {
        ENBLR::_from({
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
    #[doc = "Bits 0:5 - DMA Channel Source (Slot)"]
    #[inline]
    pub fn source(&mut self) -> _SOURCEW {
        _SOURCEW { w: self }
    }
    #[doc = "Bit 6 - DMA Channel Trigger Enable"]
    #[inline]
    pub fn trig(&mut self) -> _TRIGW {
        _TRIGW { w: self }
    }
    #[doc = "Bit 7 - DMA Channel Enable"]
    #[inline]
    pub fn enbl(&mut self) -> _ENBLW {
        _ENBLW { w: self }
    }
}
