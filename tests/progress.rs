mod panic_tests;

#[cfg(all(test, not(miri)))]
#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-specifier-types.rs");
    t.pass("tests/02-storage.rs");
    t.pass("tests/03-accessors.rs");
    t.compile_fail("tests/04-multiple-of-8bits.rs");
    t.pass("tests/05-accessor-signatures.rs");
    t.pass("tests/06-enums.rs");
    t.pass("tests/07-optional-discriminant.rs");
    t.compile_fail("tests/08-non-power-of-two.rs");
    t.compile_fail("tests/09-variant-out-of-range.rs");
    t.pass("tests/10-bits-attribute.rs");
    t.compile_fail("tests/11-bits-attribute-wrong.rs");
    t.pass("tests/12-accessors-edge.rs");
    t.pass("tests/13-tuple-structs.rs");
    t.pass("tests/14-checked-setters.rs");
    t.pass("tests/15-manual-reset.rs");
    t.pass("tests/16-u128-specifier.rs");
    t.pass("tests/17-byte-conversions.rs");
    t.pass("tests/18-within-single-byte.rs");
    t.pass("tests/19-get-spanning-data.rs");
    t.compile_fail("tests/20-access-test.rs");
    t.pass("tests/21-raw-identifiers.rs");
    t.pass("tests/22-with-setter.rs");
    t.pass("tests/23-no-implicit-prelude.rs");
    t.pass("tests/24-primitives-as-specifiers.rs");
    t.compile_fail("tests/26-invalid-struct-specifier.rs");
    t.compile_fail("tests/27-invalid-union-specifier.rs");
    t.pass("tests/28-single-bit-enum.rs");
    t.pass("tests/regressions/regression-issue-8.rs");
    t.pass("tests/29-struct-in-struct.rs");
    t.compile_fail("tests/30-out-of-bounds-specifier.rs");
    t.pass("tests/31-unfilled-from-bytes.rs");
    t.pass("tests/regressions/deny_elided_lifetime.rs");
    t.compile_fail("tests/regressions/invalid_bits_field_attr.rs");

    // Tests for `bytes = N` #[bitfield] parameter:
    t.pass("tests/bytes-param/valid-bitfield.rs");
    t.pass("tests/bytes-param/valid-specifier-bitfield.rs");
    t.compile_fail("tests/bytes-param/duplicate-parameters.rs");
    t.compile_fail("tests/bytes-param/fewer-bytes-than-expected.rs");
    t.compile_fail("tests/bytes-param/more-bytes-than-expected.rs");
    t.compile_fail("tests/bytes-param/invalid-int-value.rs");
    t.compile_fail("tests/bytes-param/invalid-type.rs");

    // Tests for `filled: bool` #[bitfield] parameter:
    t.pass("tests/filled-param/valid-bitfield-1.rs");
    t.pass("tests/filled-param/valid-bitfield-2.rs");
    t.pass("tests/filled-param/valid-bitfield-specifier-1.rs");
    t.pass("tests/filled-param/valid-bitfield-specifier-2.rs");
    t.compile_fail("tests/filled-param/duplicate-parameters.rs");
    t.compile_fail("tests/filled-param/invalid-bool-value.rs");
    t.compile_fail("tests/filled-param/invalid-specified-as-filled.rs");
    t.compile_fail("tests/filled-param/invalid-specified-as-unfilled.rs");

    // Tests for `#[repr(uN)]` and `#[cfg_attr(cond, repr(uN))]`:
    t.pass("tests/repr/valid-use.rs");
    t.pass("tests/repr/valid-cond-use.rs");
    t.pass("tests/repr/complex-use.rs");
    t.pass("tests/repr/multiple-valid-reprs-1.rs");
    t.pass("tests/repr/multiple-valid-reprs-2.rs");
    t.compile_fail("tests/repr/duplicate-repr-1.rs");
    t.compile_fail("tests/repr/duplicate-repr-2.rs");
    t.compile_fail("tests/repr/duplicate-repr-3.rs");
    t.compile_fail("tests/repr/invalid-repr-1.rs");
    t.compile_fail("tests/repr/invalid-repr-2.rs");
    t.compile_fail("tests/repr/invalid-repr-width-1.rs");
    t.compile_fail("tests/repr/invalid-repr-width-2.rs");
    t.compile_fail("tests/repr/conflicting-ignored-reprs.rs");

    // Tests for `#[derive(Debug)]`:
    t.pass("tests/derive-debug/valid-use.rs");
    t.pass("tests/derive-debug/valid-use-2.rs");
    t.pass("tests/derive-debug/valid-use-specifier.rs");
    t.pass("tests/derive-debug/print-invalid-bits.rs");
    t.pass("tests/derive-debug/respects-other-derives.rs");
    t.compile_fail("tests/derive-debug/duplicate-derive-debug.rs");
    t.compile_fail("tests/derive-debug/duplicate-derive-debug-2.rs");

    // Tests for `#[skip(..)]`:
    t.pass("tests/skip/skip-default.rs");
    t.pass("tests/skip/skip-getters-and-setters.rs");
    t.pass("tests/skip/skip-with-debug.rs");
    t.pass("tests/skip/double_wildcards-1.rs");
    t.pass("tests/skip/double_wildcards-2.rs");
    t.pass("tests/skip/skip-getters.rs");
    t.pass("tests/skip/skip-setters.rs");
    t.compile_fail("tests/skip/invalid-specifier.rs");
    t.compile_fail("tests/skip/duplicate-attr.rs");
    t.compile_fail("tests/skip/duplicate-specifier.rs");
    t.compile_fail("tests/skip/use-skipped-getter.rs");
    t.compile_fail("tests/skip/use-skipped-setter.rs");

    // Tests for `#[derive(BitfieldSpecifier)] using `#[bitfield]`:
    t.pass("tests/derive-specifier/valid-use.rs");
    t.pass("tests/derive-specifier/struct-in-struct.rs");
    t.pass("tests/derive-specifier/unfilled-from-bytes.rs");
    t.compile_fail("tests/derive-specifier/out-of-bounds.rs");
    t.compile_fail("tests/derive-specifier/duplicate-derive-1.rs");
    t.compile_fail("tests/derive-specifier/duplicate-derive-2.rs");
}
