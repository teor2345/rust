//! Test using `#[splat]` on some "overloading at home" example code.
//! <https://internals.rust-lang.org/t/pre-pre-rfc-splatting-for-named-arguments-and-function-overloading/24012>
//@ check-pass
//@ compile-flags: -Zunpretty=hir,typed
//@ rustc-env: DISABLE_TEOR=1

#![allow(incomplete_features)]
#![feature(splat)]
#![feature(tuple_trait)]

use std::marker::Tuple;

struct Foo;

trait MethodArgs: std::marker::Tuple {
    fn call_method(self, this: &Foo);
}
impl MethodArgs for (i32, String) {
    fn call_method(self, this: &Foo) {}
}
impl MethodArgs for (i32,) {
    fn call_method(self, this: &Foo) {}
}

impl Foo {
    fn method(&self, #[splat] args: impl MethodArgs) {
        args.call_method(self)
    }
}

fn main() {
    let foo = Foo;

    // FIXME(splat): should tuples also be accepted in the caller?
    // Add a tuple test for each call if they are.
    foo.method((42i32,));
    foo.method(42i32);

    foo.method((42,));
    foo.method(42);

    foo.method((42i32, "asdf".to_owned()));
    foo.method(42i32, "asdf".to_owned());

    foo.method((42, "asdf".to_owned()));
    foo.method(42, "asdf".to_owned());
}
