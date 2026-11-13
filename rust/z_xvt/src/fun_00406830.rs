// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525518: i32;
    static mut DAT_007091b0: [u8; 0];
    static mut DAT_00703ae0: [u8; 0];
    static mut DAT_006db5c0: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00406830(param_1: i32) {
    // SAFETY: param_1 is the address of the caller's buffer structure; the
    // globals are the game's own arrays, written where the original does.
    unsafe {
        let i_var2 = ((param_1 as usize).wrapping_add(0x28) as *const i32).read_unaligned();
        let off = ((param_1 as usize).wrapping_add(0xc) as *const i32).read_unaligned();
        let mut p = off.wrapping_add(param_1) as usize as *const u8;
        if DAT_00525518 == 2 {
            let mut i: i32 = 0;
            if 0 < i_var2 {
                loop {
                    let a = p.read();
                    (&raw mut DAT_007091b0)
                        .cast::<u8>()
                        .offset(i as isize)
                        .write(a);
                    let i3 = i.wrapping_add(1);
                    let b = p.add(1).read();
                    (&raw mut DAT_00703ae0)
                        .cast::<u8>()
                        .offset(i as isize)
                        .write(b);
                    i = i3;
                    p = p.add(2);
                    if i3 >= i_var2 {
                        break;
                    }
                }
                return;
            }
        } else {
            let mut i: i32 = 0;
            if 0 < i_var2 {
                loop {
                    let v = p.read();
                    p = p.add(1);
                    (&raw mut DAT_006db5c0)
                        .cast::<u8>()
                        .offset(i as isize)
                        .write(v);
                    i = i.wrapping_add(1);
                    if i >= i_var2 {
                        break;
                    }
                }
            }
        }
    }
}
