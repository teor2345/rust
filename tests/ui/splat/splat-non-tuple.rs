//! Test that using `#[splat]` on non-tuple function arguments is an error.
//@ check-pass
#![allow(incomplete_features)]
#![feature(splat)]
#![expect(unused)]

fn primitive_arg(#[splat] x: u32) {} // FIXME(splat): ERROR `#[splat]` attribute must be used on a tuple

enum NotATuple {
    A(u32),
    B(i8),
}

fn enum_arg(#[splat] y: NotATuple) {} // FIXME(splat): ERROR `#[splat]` attribute must be used on a tuple

trait FooTrait {
    fn tuple_1(#[splat] _: (u32,));

    // Ambiguous case, self could be a tuple or a non-tuple
    fn tuple_4(#[splat] self, _: (u32, i8, (), f32));
}

struct Foo;

fn struct_arg(#[splat] z: Foo) {}

impl Foo {
    // FIXME(splat): this should error except when `self` (or any splatted arg) is a tuple.
    fn tuple_2_self(
        #[splat] self, // FIXME(splat): ERROR `#[splat]` attribute must be used on a tuple
        (a, b): (u32, i8),
    ) -> u32 {
        a
    }
}

impl FooTrait for Foo {
    // FIXME(splat): should splat attributes be inherited from traits?
    // Can splat attributes be added on impls?
    fn tuple_1(_: (u32,)) {}

    fn tuple_4(
        #[splat] self, // FIXME(splat): ERROR `#[splat]` attribute must be used on a tuple
        _: (u32, i8, (), f32),
    ) {
    }
}

struct TupleStruct(u32, i8);

// FIXME(splat): tuple structs should error until we have a specific use case for them.
fn tuple_struct_arg(#[splat] z: TupleStruct) {} // FIXME(splat): ERROR `#[splat]` attribute must be used on a tuple

impl TupleStruct {
    fn tuple_2(
        #[splat] self, // FIXME(splat): ERROR `#[splat]` attribute must be used on a tuple
        (a, b): (u32, i8),
    ) -> u32 {
        a
    }
}

impl FooTrait for TupleStruct {
    fn tuple_1(#[splat] _: (u32,)) {}

    fn tuple_4(
        #[splat] self, // FIXME(splat): ERROR `#[splat]` attribute must be used on a tuple
        _: (u32, i8, (), f32),
    ) {
    }
}

fn main() {}
