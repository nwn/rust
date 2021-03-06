// compile-flags: -O

#![feature(llvm_asm)]
#![crate_type = "lib"]

// Check that inline assembly expressions without any outputs
// are marked as having side effects / being volatile

// CHECK-LABEL: @assembly
#[no_mangle]
pub fn assembly() {
    unsafe { llvm_asm!("") }
// CHECK: tail call void asm sideeffect "", {{.*}}
}
