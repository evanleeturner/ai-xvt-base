// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0044bad0(param_1: *mut i32, param_2: *mut u32, param_3: i32) -> u32;
}

#[inline(always)]
unsafe fn vfn(obj: *mut i32, off: usize) -> *const c_void {
    // SAFETY: obj points at a COM-like object whose first word is a vtable pointer.
    unsafe {
        let vt = (obj.read_unaligned() as u32 as usize) as *const u8;
        vt.byte_add(off).cast::<*const c_void>().read_unaligned()
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044b960(param_1: *mut u32) -> *mut i32 {
    if param_1.is_null() {
        return core::ptr::null_mut();
    }
    // SAFETY: param_1 points at the game's own structure; the objects and their
    // vtables are the game's own, called exactly as the original does.
    unsafe {
        let mut local_4: [u8; 4] = [0; 4];
        let mut unaff_esi: u32 = 0;
        let idx = param_1.add(3).read_unaligned() as i32;
        let mut p_var2 = param_1
            .offset((idx as isize).wrapping_add(4))
            .read_unaligned() as usize as *mut i32;
        if !p_var2.is_null() {
            let f: unsafe extern "C" fn(*mut i32, *mut u8) -> i32 =
                core::mem::transmute(vfn(p_var2, 0x24));
            let i_var1 = f(p_var2, local_4.as_mut_ptr());
            unaff_esi = i_var1 as u32;
            if i_var1 < 0 {
                unaff_esi = 0;
            }
            if (unaff_esi & 1) != 0 {
                if (param_1.add(2).read_unaligned() as i32) < 2 {
                    p_var2 = core::ptr::null_mut();
                } else {
                    let i_var1 = param_1.add(3).read_unaligned() as i32;
                    let next = i_var1.wrapping_add(1);
                    param_1.add(3).write_unaligned(next as u32);
                    if (param_1.add(2).read_unaligned() as i32) <= next {
                        param_1.add(3).write_unaligned(0);
                    }
                    let idx2 = param_1.add(3).read_unaligned() as i32;
                    p_var2 = param_1
                        .offset((idx2 as isize).wrapping_add(4))
                        .read_unaligned() as usize as *mut i32;
                    let f: unsafe extern "C" fn(*mut i32, *mut u8) -> i32 =
                        core::mem::transmute(vfn(p_var2, 0x24));
                    let i_var1 = f(p_var2, local_4.as_mut_ptr());
                    if -1 < i_var1 && (unaff_esi & 1) != 0 {
                        let g: unsafe extern "C" fn(*mut i32) =
                            core::mem::transmute(vfn(p_var2, 0x48));
                        g(p_var2);
                        let h: unsafe extern "C" fn(*mut i32, i32) =
                            core::mem::transmute(vfn(p_var2, 0x34));
                        h(p_var2, 0);
                    }
                }
            }
            if !p_var2.is_null() && (unaff_esi & 2) != 0 {
                let f: unsafe extern "C" fn(*mut i32) -> i32 =
                    core::mem::transmute(vfn(p_var2, 0x50));
                let i_var1 = f(p_var2);
                if -1 < i_var1 {
                    let a = param_1.read_unaligned() as usize as *mut u32;
                    let b = param_1.add(1).read_unaligned() as i32;
                    if FUN_0044bad0(p_var2, a, b) != 0 {
                        return p_var2;
                    }
                }
                p_var2 = core::ptr::null_mut();
            }
        }
        p_var2
    }
}
