extern crate piston_meta;
extern crate dyon;

use dyon::*;

pub fn test_src(source: &str) {
    let mut module = Module::new();
    load(source, &mut module).unwrap_or_else(|err| {
        panic!("{}", err);
    });
}

pub fn debug_src(source: &str) {
    let mut module = Module::new();
    load(source, &mut module).unwrap_or_else(|err| {
        panic!("{}", err);
    });
    panic!("{:?}", module.functions);
}

#[test]
fn test_syntax() {
    test_src("source/syntax/main.rs");
    test_src("source/syntax/args.rs");
    test_src("source/syntax/id.rs");
    test_src("source/syntax/call.rs");
    test_src("source/syntax/array.rs");
    test_src("source/syntax/prop.rs");
    test_src("source/syntax/for.rs");
    test_src("source/syntax/compare.rs");
    test_src("source/syntax/add.rs");
    test_src("source/syntax/mul.rs");
    test_src("source/syntax/pow.rs");
    test_src("source/syntax/add_mul.rs");
    test_src("source/syntax/mul_add.rs");
    test_src("source/syntax/pos_len.rs");
    test_src("source/syntax/if.rs");
    test_src("source/syntax/else_if.rs");
    test_src("source/syntax/assign_if.rs");
    test_src("source/syntax/new_pos.rs");
    test_src("source/syntax/lifetime.rs");
    test_src("source/syntax/lifetime_6.rs");
    test_src("source/syntax/insert.rs");
    test_src("source/syntax/named_call.rs");
    test_src("source/syntax/max_min.rs");
    test_src("source/syntax/return_void.rs");
    test_src("source/syntax/typeof.rs");
    test_src("source/syntax/load_module.rs");
    test_src("source/syntax/println_colon.rs");
    test_src("source/syntax/neg.rs");
    test_src("source/syntax/some.rs");
    test_src("source/syntax/pop.rs");
    test_src("source/syntax/accessor.rs");
    test_src("source/syntax/sum.rs");
    test_src("source/syntax/min_max.rs");
}

#[test]
fn test_functions() {
    test_src("source/functions/functions.rs");
}

#[test]
fn test_error() {
    test_src("source/error/propagate.rs");
    test_src("source/error/call.rs");
    test_src("source/error/named_call.rs");
    test_src("source/error/if.rs");
    test_src("source/error/trace.rs");
    test_src("source/error/unwrap_err.rs");
    test_src("source/error/option.rs");
}
