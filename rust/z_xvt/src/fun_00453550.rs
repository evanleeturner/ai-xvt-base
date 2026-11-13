// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048e480(param_1: i32, param_2: *mut i8) -> *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00453550(
    param_1: *mut core::ffi::c_void,
    param_2: i32,
    param_3: *mut f32,
) {
    if param_1.is_null() {
        return;
    }
    // SAFETY: the pointers describe the game's own node structures and the
    // vertex/offset arrays; reads and writes follow the original's order.
    unsafe {
        let mut p = param_1 as *mut u32;
        while p.add(1).read_unaligned() == 7 {
            let next = FUN_0048e480(param_2, p.add(5).read_unaligned() as *mut i8);
            if next.is_null() {
                return;
            }
            p = next as *mut u32;
        }
        if p.add(1).read_unaligned() == 3 {
            let mut iv2 = p.add(4).read_unaligned() as i32;
            let mut pf = p.add(5).read_unaligned() as *mut f32;
            if 0 < iv2 {
                loop {
                    iv2 = iv2.wrapping_sub(1);
                    let a = param_3.read_unaligned() + pf.read_unaligned();
                    pf.write_unaligned(a);
                    let b = pf.add(1).read_unaligned() + param_3.add(1).read_unaligned();
                    pf.add(1).write_unaligned(b);
                    if iv2 == 0 {
                        break;
                    }
                    let c = pf.add(2).read_unaligned() + param_3.add(2).read_unaligned();
                    pf.add(2).write_unaligned(c);
                    pf = pf.add(3);
                }
                let c = pf.add(2).read_unaligned() + param_3.add(2).read_unaligned();
                pf.add(2).write_unaligned(c);
            }
        }
        let mut off: i32 = 0;
        let mut i: i32 = 0;
        if 0 < p.add(2).read_unaligned() as i32 {
            loop {
                let base = p.add(3).read_unaligned();
                let child = (base.wrapping_add(off as u32) as *const u32).read_unaligned();
                off = off.wrapping_add(4);
                i = i.wrapping_add(1);
                FUN_00453550(child as *mut core::ffi::c_void, param_2, param_3);
                if i >= p.add(2).read_unaligned() as i32 {
                    break;
                }
            }
        }
    }
}
