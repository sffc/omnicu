// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_pattern::DoublePlaceholderPattern;
use writeable::assert_try_writeable_eq;

#[test]
fn test_try_writeable() {
	let pattern = DoublePlaceholderPattern::try_from_str("{0}-{1}", Default::default()).unwrap();
	assert_try_writeable_eq!(
		pattern.try_interpolate((Ok("xxx"), Err("yyy"))),
		"xxx-yyy",
		Err("yyy")
	);
}
