use std::convert::TryFrom;
use rust_decimal::Decimal;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Decimal128 {
    data: u128,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unpacked128 {
    Value(bool, u128, i16),
    Inf,
    NegInf,
    NaN,
    SNaN,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    MantissaOverflow,
    ExponentOverflow,
}

impl Decimal128 {
    const EXP_MIN: i16 = -6176i16;
    const EXP_MAX: i16 = 6111i16;

    const EXP_SHIFT: u16 = 113;
    const EXP_HIGH_MASK: u16 = 0x3000u16;

    const EXP_INF_MASK: u16 = 0x3e00u16;
    const EXP_INF_VALUE: u16 = 0x3c00u16;

    const EXP_NAN_MASK: u16 = 0x3f00u16;
    const EXP_NAN_VALUE: u16 = 0x3e00u16;
    const EXP_SNAN_VALUE: u16 = 0x3f00u16;

    const MANTISSA_MAX: u128 = 10_000_000_000_000_000_000_000_000_000_000_000u128;
    const MANTISSA_MASK: u128 = 0x1ffffffffffffffffffffffffffffu128;

    pub fn inf(sign: bool) -> Decimal128 {
        let mut r = (Decimal128::EXP_INF_VALUE as u128) << Decimal128::EXP_SHIFT;
        if sign {
            r |= 1u128 << 127
        }
        Decimal128 { data: r }
    }

    pub fn nan() -> Decimal128 {
        let r = (Decimal128::EXP_NAN_VALUE as u128) << Decimal128::EXP_SHIFT;
        Decimal128 { data: r }
    }

    pub fn snan() -> Decimal128 {
        let r = (Decimal128::EXP_SNAN_VALUE as u128) << Decimal128::EXP_SHIFT;
        Decimal128 { data: r }
    }

    pub fn pack(u: Unpacked128) -> Result<Decimal128, Error> {
        match u {
            Unpacked128::Value(s, m, e) => Decimal128::pack_value(s, m, e),
            Unpacked128::Inf => Ok(Decimal128::inf(false)),
            Unpacked128::NegInf => Ok(Decimal128::inf(true)),
            Unpacked128::NaN => Ok(Decimal128::nan()),
            Unpacked128::SNaN => Ok(Decimal128::snan()),
        }
    }

    pub fn pack_value(sign: bool, mantissa: u128, exponent: i16) -> Result<Decimal128, Error> {
        if exponent > Decimal128::EXP_MAX {
            return Err(Error::ExponentOverflow);
        } else if exponent < Decimal128::EXP_MIN {
            return Err(Error::ExponentOverflow);
        }
        if mantissa > Decimal128::MANTISSA_MAX {
            return Err(Error::MantissaOverflow);
        }
        let mut r = mantissa;
        r |= ((exponent - Decimal128::EXP_MIN) as u128) << Decimal128::EXP_SHIFT;
        if sign {
            r |= 1u128 << 127
        }
        Ok(Decimal128 { data: r })
    }

    pub fn unpack(&self) -> Unpacked128 {
        let combination = (self.data >> Decimal128::EXP_SHIFT) as u16;
        let sign = combination & (0x1u16 << 14) != 0;
        if combination & Decimal128::EXP_HIGH_MASK == Decimal128::EXP_HIGH_MASK {
            if combination & Decimal128::EXP_INF_MASK == Decimal128::EXP_INF_VALUE {
                return if sign {
                    Unpacked128::NegInf
                } else {
                    Unpacked128::Inf
                };
            } else if combination & Decimal128::EXP_NAN_MASK == Decimal128::EXP_NAN_VALUE {
                return Unpacked128::NaN;
            } else if combination & Decimal128::EXP_NAN_MASK == Decimal128::EXP_SNAN_VALUE {
                return Unpacked128::SNaN;
            } else {
                // Overflow, treat as 0
                return Unpacked128::Value(false, 0, 0);
            }
        }
        Unpacked128::Value(
            sign,
            self.data & Decimal128::MANTISSA_MASK,
            (combination & 0x3fff) as i16 + Decimal128::EXP_MIN,
        )
    }

    pub fn to_decimal(&self) -> Result<Decimal, rust_decimal::Error> {
        Decimal::try_from(self)
    }
}

impl std::fmt::Display for Decimal128 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.unpack() {
            Unpacked128::Value(s, m, e) => write!(f, "{}{}.E{}", if s { "-" } else { "" }, m, e),
            Unpacked128::Inf => write!(f, "Inf"),
            Unpacked128::NegInf => write!(f, "-Inf"),
            Unpacked128::NaN => write!(f, "NaN"),
            Unpacked128::SNaN => write!(f, "sNaN"),
        }
    }
}

impl TryFrom<&Decimal> for Decimal128 {
    type Error = Error;

    fn try_from(v: &Decimal) -> Result<Decimal128, Error> {
        let m = v.mantissa();
        Decimal128::pack_value(m < 0, if m > 0 { m as u128 } else { (-m) as u128 }, v.scale() as i16) // scale is in range [0, 28]
    }
}

impl TryFrom<&Decimal128> for Decimal {
    type Error = rust_decimal::Error;

    fn try_from(v: &Decimal128) -> Result<Decimal, Self::Error> {
        match v.unpack() {
            Unpacked128::Inf => Err(Self::Error::ExceedsMaximumPossibleValue),
            Unpacked128::NegInf => Err(Self::Error::LessThanMinimumPossibleValue),
            Unpacked128::NaN => Err(Self::Error::ErrorString("Cannot convert from NaN".to_string())),
            Unpacked128::SNaN => Err(Self::Error::ErrorString("Cannot convert from sNaN".to_string())),
            Unpacked128::Value(s, um, e) => {
                let im = um as i128; // Always possible, mantissa uses ~110 bits
                let m = if s { -im } else { im };

                if e < 0 {
                    if m == 0 { return Ok(Decimal::ZERO); }
                    Err(Self::Error::ErrorString("Negative exponent".to_string()))
                } else {
                    Decimal::try_from_i128_with_scale(m, e as u32)
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn check_d128(s: &str, bin: &[u8; 16], u: Unpacked128) {
        let r = Decimal128::pack(u);
        assert!(r.is_ok());
        let d = r.unwrap();
        assert_eq!(*bin, d.data.to_ne_bytes());
        let u1 = d.unpack();
        assert_eq!(u, u1);
        assert_eq!(s, format!("{}", d));

        match u {
            Unpacked128::Value(s, m, e) => {
                if e > 28 || e < 0 { return; }
                let r1 = Decimal::try_from(&d);
                assert!(r1.is_ok());
                let d1 = r1.unwrap();
                assert_eq!(d1.scale(), e as u32);
                assert_eq!(d1.mantissa(), if s { -(m as i128) } else { m as i128 });
            },
            _ => {},
        }
    }

    #[test]
    fn test() {
        check_d128(
            "Inf",
            b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x78",
            Unpacked128::Inf,
        );
        check_d128(
            "-Inf",
            b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xf8",
            Unpacked128::NegInf,
        );
        check_d128(
            "NaN",
            b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x7c",
            Unpacked128::NaN,
        );
        check_d128(
            "sNaN",
            b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x7e",
            Unpacked128::SNaN,
        );
        check_d128(
            "9999999999999999999999999999999999.E6111",
            b"\xff\xff\xff\xff\x63\x8e\x8d\x37\xc0\x87\xad\xbe\x09\xed\xff\x5f",
            Unpacked128::Value(false, 9999999999999999999999999999999999u128, 6111i16),
        );
        check_d128(
            "-9999999999999999999999999999999999.E6111",
            b"\xff\xff\xff\xff\x63\x8e\x8d\x37\xc0\x87\xad\xbe\x09\xed\xff\xdf",
            Unpacked128::Value(true, 9999999999999999999999999999999999u128, 6111i16),
        );
        check_d128(
            "1.E-6176",
            b"\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
            Unpacked128::Value(false, 1u128, -6176i16),
        );
        check_d128(
            "18446744073709551616.E0",
            b"\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x40\x30",
            Unpacked128::Value(false, 18446744073709551616u128, 0i16),
        );
        check_d128(
            "9223372036854775808.E0",
            b"\x00\x00\x00\x00\x00\x00\x00\x80\x00\x00\x00\x00\x00\x00\x40\x30",
            Unpacked128::Value(false, 9223372036854775808u128, 0i16),
        );
        check_d128(
            "-9223372036854775809.E0",
            b"\x01\x00\x00\x00\x00\x00\x00\x80\x00\x00\x00\x00\x00\x00\x40\xb0",
            Unpacked128::Value(true, 9223372036854775809u128, 0i16),
        );
        check_d128(
            "1234567890.E0",
            b"\xd2\x02\x96\x49\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x40\x30",
            Unpacked128::Value(false, 1234567890u128, 0i16),
        );
        check_d128(
            "1234567890.E5",
            b"\xd2\x02\x96\x49\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x4a\x30",
            Unpacked128::Value(false, 1234567890u128, 5i16),
        );
        check_d128(
            "-1234567890.E0",
            b"\xd2\x02\x96\x49\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x40\xb0",
            Unpacked128::Value(true, 1234567890u128, 0i16),
        );
        check_d128(
            "-1234567890.E5",
            b"\xd2\x02\x96\x49\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x4a\xb0",
            Unpacked128::Value(true, 1234567890u128, 5i16),
        );
    }
}
