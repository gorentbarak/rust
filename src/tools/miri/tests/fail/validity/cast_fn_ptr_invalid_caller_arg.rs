#![allow(internal_features)]
#![feature(core_intrinsics, custom_mir)]

use std::intrinsics::mir::*;
use std::num::NonZeroU32;
use std::ptr;

fn f(c: u32) {
    println!("{c}");
}

// Call that function in a bad way, with an invalid NonZeroU32, but without
// ever materializing this as a NonZeroU32 value outside the call itself.
#[custom_mir(dialect = "runtime", phase = "optimized")]
fn call(f: fn(NonZeroU32)) {
    mir! {
        let _res: ();
        {
            let c = 0;
            let tmp = ptr::addr_of!(c);
            let ptr = tmp as *const NonZeroU32;
            // The call site now is a NonZeroU32-to-u32 transmute.
            Call(_res = f(*ptr), retblock, UnwindContinue()) //~ERROR: expected something greater or equal to 1
        }
        retblock = {
            Return()
        }
    }
}

fn main() {
    let f: fn(NonZeroU32) = unsafe { std::mem::transmute(f as fn(u32)) };
    call(f);
}
