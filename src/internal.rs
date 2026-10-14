macro_rules! randi_wide {
    (i 32) => {
        i64
    };
    (i 64) => {
        i128
    };
    (u 32) => {
        u64
    };
    (u 64) => {
        u128
    };
}
pub(crate) use randi_wide;

macro_rules! i2f_bits {
    (32 bits) => {
        0x3F800000
    };
    (32 bias) => {
        9
    };
    (64 bits) => {
        0x3FF0000000000000
    };
    (64 bias) => {
        12
    };
}
pub(crate) use i2f_bits;

macro_rules! u2f_01 {
    ($ft:ty, $bits:tt, $x:expr) => {{
        <$ft>::from_bits(($x >> $crate::i2f_bits!($bits bias)) | $crate::i2f_bits!($bits bits)) - 1.0
    }};
}
pub(crate) use u2f_01;
