// revisions: CHECK-BASE CHECK-OPT
// compile-flags: -C no-prepopulate-passes -Z mir-opt-level=0
//[CHECK-BASE] compile-flags: -Z opt-switch-monomorphizing=off
//[CHECK-OPT] compile-flags: -Z opt-switch-monomorphizing=on

#![crate_type = "lib"]
#![feature(never_type)]

use std::num::NonZeroUsize;

// CHECK-LABEL: @match_never
#[no_mangle]
pub fn match_never(e: !) -> u8 {
    // CHECK-NOT: switch
    // CHECK: unreachable
    match e {}
}

pub enum BothEmpty {
    Left(!),
    Right(!),
}

// CHECK-LABEL: @match_both_empty
#[no_mangle]
pub fn match_both_empty(e: BothEmpty) -> u8 {
    // CHECK: %e =
    // CHECK-BASE-NEXT: switch i[[SIZE:[0-9]+]] undef, label %{{[a-zA-Z0-9_]+}} [
    // CHECK-BASE-NEXT:    i[[SIZE]] 0
    // CHECK-BASE-NEXT:    i[[SIZE]] 1
    // CHECK-BASE-NEXT: ]
    // CHECK-OPT-NEXT: unreachable
    match e {
        BothEmpty::Left(_) => 100,
        BothEmpty::Right(_) => 101,
    }
}

pub enum BothEmptyInts {
    Bool(bool, !),
    Usize(usize, !),
}

// CHECK-LABEL: @match_both_empty_ints
#[no_mangle]
pub fn match_both_empty_ints(e: BothEmptyInts) -> u8 {
    // CHECK: %e = alloca %BothEmptyInts
    // CHECK-BASE-NEXT: switch i[[SIZE:[0-9]+]] undef, label %{{[a-zA-Z0-9_]+}} [
    // CHECK-BASE-NEXT:    i[[SIZE]] 0
    // CHECK-BASE-NEXT:    i[[SIZE]] 1
    // CHECK-BASE-NEXT: ]
    // CHECK-OPT-NEXT: unreachable
    match e {
        BothEmptyInts::Bool(_, _) => 100,
        BothEmptyInts::Usize(_, _) => 101,
    }
}

pub enum BothEmptyWithNiche {
    Unit(!),
    Usize(NonZeroUsize, !),
}

// CHECK-LABEL: @match_both_empty_with_niche
#[no_mangle]
pub fn match_both_empty_with_niche(e: BothEmptyWithNiche) -> u8 {
    // CHECK: %e =
    // CHECK-BASE-NEXT: switch i[[SIZE:[0-9]+]] undef, label %{{[a-zA-Z0-9_]+}} [
    // CHECK-BASE-NEXT:    i[[SIZE]] 0
    // CHECK-BASE-NEXT:    i[[SIZE]] 1
    // CHECK-BASE-NEXT: ]
    // CHECK-OPT-NEXT: unreachable
    match e {
        BothEmptyWithNiche::Unit(_) => 100,
        BothEmptyWithNiche::Usize(_, _) => 101,
    }
}

pub enum EmptyOrBool {
    Empty(!),
    Bool(bool),
}

// CHECK-LABEL: @match_empty_or_bool
#[no_mangle]
pub fn match_empty_or_bool(e: EmptyOrBool) -> u8 {
    // CHECK: %[[REG:[0-9]+]] = zext i1 %0 to i8
    // CHECK-NEXT: store i8 %[[REG]], i8* %e
    // CHECK-BASE-NEXT: switch i[[SIZE:[0-9]+]] 1, label %{{[a-zA-Z0-9_]+}} [
    // CHECK-BASE-NEXT: i[[SIZE]] 0
    // CHECK-BASE-NEXT: i[[SIZE]] 1, label %[[R:[a-zA-Z0-9_]+]]
    // CHECK-BASE-NEXT: ]
    // CHECK-OPT-NEXT: br label %[[R:[a-zA-Z0-9_]+]]
    // CHECK: [[R]]:
    // CHECK-NEXT: store i8 101, i8* %1
    match e {
        EmptyOrBool::Empty(_) => 100,
        EmptyOrBool::Bool(_) => 101,
    }
}

pub enum BoolOrEmpty {
    Bool(bool),
    Empty(!),
}

// CHECK-LABEL: @match_bool_or_empty
#[no_mangle]
pub fn match_bool_or_empty(e: BoolOrEmpty) -> u8 {
    // CHECK: %[[REG:[0-9]+]] = zext i1 %0 to i8
    // CHECK-NEXT: store i8 %[[REG]], i8* %e
    // CHECK-BASE-NEXT: switch i[[SIZE:[0-9]+]] 0, label %{{[a-zA-Z0-9_]+}} [
    // CHECK-BASE-NEXT: i[[SIZE]] 0, label %[[L:[a-zA-Z0-9_]+]]
    // CHECK-BASE-NEXT: i[[SIZE]] 1
    // CHECK-BASE-NEXT: ]
    // CHECK-OPT-NEXT: br label %[[L:[a-zA-Z0-9_]+]]
    // CHECK: [[L]]:
    // CHECK-NEXT: store i8 100, i8* %1
    match e {
        BoolOrEmpty::Bool(_) => 100,
        BoolOrEmpty::Empty(_) => 101,
    }
}

pub enum UninhabitedUsizeOrBool {
    Usize(usize, !),
    Bool(bool),
}

// CHECK-LABEL: @match_uninhabited_usize_or_bool
#[no_mangle]
pub fn match_uninhabited_usize_or_bool(e: UninhabitedUsizeOrBool) -> u8 {
    // CHECK: %[[TMP:[0-9]+]] = load i8, i8* %{{[0-9]+}}
    // CHECK-NEXT: %_[[REG:[0-9]+]] = zext i8 %[[TMP]] to i[[SIZE:[0-9]+]]
    // CHECK-BASE-NEXT: switch i[[SIZE:[0-9]+]] %_[[REG]], label %{{[a-zA-Z0-9_]+}} [
    // CHECK-BASE-NEXT: i[[SIZE]] 0
    // CHECK-BASE-NEXT: i[[SIZE]] 1, label %[[R:[a-zA-Z0-9_]+]]
    // CHECK-BASE-NEXT: ]
    // CHECK-OPT-NEXT: %[[D:[0-9]+]] = icmp eq i[[SIZE]] %_[[REG]], 1
    // CHECK-OPT-NEXT: br i1 %[[D]], label %[[R:[a-zA-Z0-9_]+]]
    // CHECK: [[R]]:
    // CHECK-NEXT: store i8 101, i8* %1, align 1
    match e {
        UninhabitedUsizeOrBool::Usize(_, _) => 100,
        UninhabitedUsizeOrBool::Bool(_) => 101,
    }
}

pub enum BoolOrUninhabitedUsize {
    Bool(bool),
    Usize(usize, !),
}

// CHECK-LABEL: @match_bool_or_uninhabited_nonzero_usize
#[no_mangle]
pub fn match_bool_or_uninhabited_nonzero_usize(e: BoolOrUninhabitedUsize) -> u8 {
    // CHECK: %[[TMP:[0-9]+]] = load i8, i8* %{{[0-9]+}}
    // CHECK-NEXT: %_[[REG:[0-9]+]] = zext i8 %[[TMP]] to i[[SIZE:[0-9]+]]
    // CHECK-BASE-NEXT: switch i[[SIZE:[0-9]+]] %_[[REG]], label %{{[a-zA-Z0-9_]+}} [
    // CHECK-BASE-NEXT: i[[SIZE]] 0, label %[[L:[a-zA-Z0-9_]+]]
    // CHECK-BASE-NEXT: i[[SIZE]] 1
    // CHECK-BASE-NEXT: ]
    // CHECK-OPT-NEXT: %[[D:[0-9]+]] = icmp eq i[[SIZE]] %_[[REG]], 0
    // CHECK-OPT-NEXT: br i1 %[[D]], label %[[L:[a-zA-Z0-9_]+]]
    // CHECK: [[L]]:
    // CHECK-NEXT: store i8 100, i8* %1, align 1
    match e {
        BoolOrUninhabitedUsize::Bool(_) => 100,
        BoolOrUninhabitedUsize::Usize(_, _) => 101,
    }
}

pub enum UninhabitedNonZeroUsizeOrUnit {
    Usize(NonZeroUsize, !),
    Unit,
}

// CHECK-LABEL: @match_uninhabited_non_zero_usize_or_unit
#[no_mangle]
pub fn match_uninhabited_non_zero_usize_or_unit(e: UninhabitedNonZeroUsizeOrUnit) -> u8 {
    // CHECK: %[[TMP1:[0-9]+]] = load i[[SIZE:[0-9]+]], i[[SIZE]]* %{{[0-9]+}}
    // CHECK-NEXT: %[[TMP2:[0-9]+]] = icmp eq i[[SIZE]] %[[TMP1]], 0
    // CHECK-NEXT: %_[[REG:[0-9]+]] = select i1 %[[TMP2]], i[[SIZE]] 1, i[[SIZE]] 0
    // CHECK-BASE-NEXT: switch i[[SIZE:[0-9]+]] %_[[REG]], label %{{[a-zA-Z0-9_]+}} [
    // CHECK-BASE-NEXT: i[[SIZE]] 0
    // CHECK-BASE-NEXT: i[[SIZE]] 1, label %[[R:[a-zA-Z0-9_]+]]
    // CHECK-BASE-NEXT: ]
    // CHECK-OPT-NEXT: %[[D:[0-9]+]] = icmp eq i[[SIZE]] %_[[REG]], 1
    // CHECK-OPT-NEXT: br i1 %[[D]], label %[[R:[a-zA-Z0-9_]+]]
    // CHECK: [[R]]:
    // CHECK-NEXT: store i8 101, i8* %1, align 1
    match e {
        UninhabitedNonZeroUsizeOrUnit::Usize(_, _) => 100,
        UninhabitedNonZeroUsizeOrUnit::Unit => 101,
    }
}

pub enum UnitOrUninhabitedNonZeroUsize {
    Unit,
    Usize(NonZeroUsize, !),
}

// CHECK-LABEL: @match_unit_or_uninhabited_non_zero_usize
#[no_mangle]
pub fn match_unit_or_uninhabited_non_zero_usize(e: UnitOrUninhabitedNonZeroUsize) -> u8 {
    // CHECK: %[[TMP1:[0-9]+]] = load i[[SIZE:[0-9]+]], i[[SIZE]]* %{{[0-9]+}}
    // CHECK-NEXT: %[[TMP2:[0-9]+]] = icmp eq i[[SIZE]] %[[TMP1]], 0
    // CHECK-NEXT: %_[[REG:[0-9]+]] = select i1 %[[TMP2]], i[[SIZE]] 0, i[[SIZE]] 1
    // CHECK-BASE-NEXT: switch i[[SIZE:[0-9]+]] %_[[REG]], label %{{[a-zA-Z0-9_]+}} [
    // CHECK-BASE-NEXT: i[[SIZE]] 0, label %[[L:[a-zA-Z0-9_]+]]
    // CHECK-BASE-NEXT: i[[SIZE]] 1
    // CHECK-BASE-NEXT: ]
    // CHECK-OPT-NEXT: %[[D:[0-9]+]] = icmp eq i[[SIZE]] %_[[REG]], 0
    // CHECK-OPT-NEXT: br i1 %[[D]], label %[[L:[a-zA-Z0-9_]+]]
    // CHECK: [[L]]:
    // CHECK-NEXT: store i8 100, i8* %1, align 1
    match e {
        UnitOrUninhabitedNonZeroUsize::Unit => 100,
        UnitOrUninhabitedNonZeroUsize::Usize(_, _) => 101,
    }
}
