#![no_main]

icu_benchmark_macros::static_setup!();

use fixed_decimal::FixedDecimal;
use writeable::Writeable;

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    icu_benchmark_macros::main_setup!();
    let monetary_int = 19_9500;
    let fixed_decimal = FixedDecimal::from(monetary_int)
        .multiplied_pow10(-4)
        .expect("-4 is well in range");

    let mut output = String::with_capacity(fixed_decimal.write_len().capacity());
    fixed_decimal
        .write_to(&mut output)
        .expect("Writing to a string is infallible");

    debug_assert_eq!("19.9500", fixed_decimal.to_string());

    0
}
