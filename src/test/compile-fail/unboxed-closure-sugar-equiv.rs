// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Test that the unboxed closure sugar can be used with an arbitrary
// struct type and that it is equivalent to the same syntax using
// angle brackets. This test covers only simple types and in
// particular doesn't test bound regions.

#![allow(dead_code)]

struct Foo<T,U> {
    t: T, u: U
}

trait Eq<X> { }
impl<X> Eq<X> for X { }
fn eq<A,B:Eq<A>>() { }

fn test<'a,'b>() {
    // No errors expected:
    eq::< Foo<(),()>,                   Foo()                         >();
    eq::< Foo<(int,),()>,               Foo(int)                      >();
    eq::< Foo<(int,uint),()>,           Foo(int,uint)                 >();
    eq::< Foo<(int,uint),uint>,         Foo(int,uint) -> uint         >();
    eq::< Foo<(&'a int,&'b uint),uint>, Foo(&'a int,&'b uint) -> uint >();

    // Errors expected:
    eq::< Foo<(),()>,                   Foo(char)                     >();
    //~^ ERROR not implemented
}

fn main() { }
