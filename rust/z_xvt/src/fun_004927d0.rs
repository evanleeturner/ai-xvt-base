// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d9d56: *mut u8;
    static mut DAT_006d9d60: u16;
    static mut DAT_006d9d62: u16;
    static mut DAT_0070b1da: [u16; 1];
    static mut DAT_00703ca0: [i8; 1];
    static mut DAT_005250b4: i8;
    static mut DAT_005250a0: [i8; 1];
    static mut DAT_00525090: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_0042a060();
}

unsafe fn same(a: *const i8, b: *const i8, n: usize) -> bool {
    let mut i = 0;
    while i < n {
        // SAFETY: both pointers address the game's own buffers, at least n bytes long.
        if unsafe { a.add(i).read() != b.add(i).read() } {
            return false;
        }
        i += 1;
    }
    true
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004927d0() {
    // SAFETY: all globals and the callee are the game's own memory; the
    // pointers are read and written where the original does.
    unsafe {
        let base = DAT_006d9d56;
        let b_var1 = base.byte_add(0xb).read();
        base.byte_add(0xc)
            .write(base.byte_add(0xc).read().wrapping_add(1));
        let b_var2 = base.byte_add(0xc).read();
        let empty = if b_var2 > 0xb {
            true
        } else {
            let idx = (DAT_006d9d60 as i32)
                .wrapping_mul(0x566)
                .wrapping_add((b_var2 as i32).wrapping_mul(2));
            (&raw const DAT_0070b1da)
                .cast::<u16>()
                .offset(idx as isize)
                .read()
                == 0
        };
        if empty {
            DAT_006d9d56.byte_add(0xc).write(4);
            let p5 = (&raw const DAT_00703ca0)
                .cast::<i8>()
                .offset((b_var1 as isize) * 0x55);
            let matched = same(p5, &raw const DAT_005250b4, 0xc)
                || same(p5, (&raw const DAT_005250a0).cast::<i8>(), 0x11)
                || same(p5, (&raw const DAT_00525090).cast::<i8>(), 0x10);
            if matched {
                let p = DAT_006d9d56.byte_add(DAT_006d9d62 as usize).byte_add(5);
                p.write(p.read().wrapping_add(1));
            }
        }
        let base = DAT_006d9d56;
        let v = (base.byte_add(0xc).read() as i32).wrapping_add(0x8000) as u16;
        base.byte_add(0x18).cast::<u16>().write_unaligned(v);
        base.byte_add(0x1a).cast::<u16>().write_unaligned(0);
        base.byte_add(0x1e).write(0);
        FUN_0042a060();
    }
}
