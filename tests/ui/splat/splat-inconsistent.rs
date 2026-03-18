//! Test using `#[splat]` incorrectly, in ways not covered by other tests.

#![allow(incomplete_features)]
#![feature(splat)]

fn multisplat_bad_2(#[splat] (_a, _b): (u32, i8), #[splat] (_c, _d): (u32, i8)) {}

extern "C" {
    // FIXME(splat): tuple layouts are unspecified. Should this error in addition to
    // the existing `improper_ctypes` lint?
    #[expect(improper_ctypes)]
    fn bar_2(#[splat] _: (u32, i8));
}

trait FooTrait {
    fn has_splat(#[splat] _: ());

    fn no_splat(_: (u32, f64));
}

struct Foo;

impl FooTrait for Foo {
    fn has_splat(_: ()) {} //~ ERROR method `has_splat` has an incompatible type for trait

    fn no_splat(#[splat] _: (u32, f64)) {} //~ ERROR method `no_splat` has an incompatible type for trait
}

fn main() {}
