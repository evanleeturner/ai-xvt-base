// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn FUN_004b88a0(a: u32, b: u32, c: i32) -> *mut i32;
    fn _memset(p: *mut core::ffi::c_void, v: i32, n: u32) -> *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b634d(param_1: i32, param_2: *mut u32) -> *mut i32 {
    let mut local_70: [u32; 0x1b] = [0; 0x1b];
    // SAFETY: the callees are the game's own; param_1 and param_2 point at the
    // caller's structures; the object at piVar1[3] has a vtable with a method
    // at +0x64 that fills local_70, whose words 2, 4 and 9 are the C's
    // local_68, local_60 and local_4c.
    unsafe {
        let pi_var1 = FUN_004b88a0(
            param_2.add(1).read_unaligned(),
            param_2.add(2).read_unaligned(),
            param_1,
        );
        _memset(local_70.as_mut_ptr().cast(), 0, 0x6c);
        local_70[0] = 0x6c;
        let obj = pi_var1.add(3).read_unaligned();
        let vtable = (obj as usize as *const i32).read_unaligned();
        let fnp = ((vtable as usize).wrapping_add(0x64) as *const i32).read_unaligned();
        let f: unsafe extern "C" fn(i32, i32, *mut u32, i32, i32) =
            core::mem::transmute(fnp as usize as *const core::ffi::c_void);
        f(obj, 0, local_70.as_mut_ptr(), 1, 0);

        let p = local_70.as_ptr();
        let local_68 = p.add(2).read_volatile();
        let local_60 = p.add(4).read_volatile() as i32;
        let local_4c = p.add(9).read_volatile();

        let divisor = ((param_1 as u32).wrapping_add(4) as usize as *const u32).read_unaligned();
        let n = (local_60.wrapping_shl(3)) as u32;
        let q: u32;
        core::arch::asm!(
            "div {d}",
            d = in(reg) divisor,
            inout("eax") n => q,
            inout("edx") 0u32 => _,
        );
        param_2.add(1).write_unaligned(q);
        param_2.add(2).write_unaligned(local_68);
        let d2 = ((param_1 as u32).wrapping_add(4) as usize as *const u32).read_unaligned();
        param_2.add(3).write_unaligned(d2 >> 3);
        param_2.write_unaligned(local_4c);
        pi_var1
    }
}
