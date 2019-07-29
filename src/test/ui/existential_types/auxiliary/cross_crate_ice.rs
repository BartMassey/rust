// Crate that exports an existential type. Used for testing cross-crate.

#![crate_type="rlib"]

#![feature(type_alias_impl_trait)]

pub type Foo = impl std::fmt::Debug;

pub fn foo() -> Foo {
    5
}
