// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006da1a0: i32;
    static mut DAT_006a89e0: *mut *mut i8;
    static mut DAT_0070ab20: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0047ea40(param_1: *mut i8);
    fn FUN_0048fce0(param_1: *mut i8, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0047f010(param_1: u16) {
    let mut local_80: core::mem::MaybeUninit<[i8; 128]> = core::mem::MaybeUninit::uninit();
    let buf: *mut i8 = local_80.as_mut_ptr().cast::<i8>();

    // SAFETY: globals and callees are the game's own; buf is a local 128-byte
    // buffer, written where the original writes (including its unchecked
    // indices); the source strings are the game's NUL-terminated data.
    unsafe {
        if param_1 < 4 {
            if DAT_006da1a0 == 0 {
                buf.write(0);
            } else {
                let pc_var2: *mut i8 = (&raw mut DAT_006a89e0)
                    .cast::<*mut i8>()
                    .add(param_1 as usize)
                    .read_unaligned();
                FUN_0047ea40(pc_var2);
                let mut u_var5: u32 = 0;
                loop {
                    let c_var1 = pc_var2.add(u_var5 as usize).read();
                    buf.add(u_var5 as usize).write(c_var1);
                    if c_var1 == 0 {
                        break;
                    }
                    let u_var3: u16 = (u_var5 as u16).wrapping_add(1);
                    u_var5 = u_var3 as u32;
                    if !(u_var3 < 0x80) {
                        break;
                    }
                }
                if param_1 == 1 {
                    let mut u_var4: u16 = u_var5 as u16;
                    let mut u_var3: u16 = 0;
                    while u_var4 < 0x80 {
                        let c_var1 = (&raw mut DAT_0070ab20)
                            .cast::<i8>()
                            .byte_add(u_var3 as usize)
                            .read();
                        buf.add(u_var5 as usize).write(c_var1);
                        if c_var1 == 0 {
                            break;
                        }
                        u_var4 = (u_var5 as u16).wrapping_add(1);
                        u_var5 = u_var4 as u32;
                        u_var3 = u_var3.wrapping_add(1);
                    }
                    buf.add(u_var5 as usize).write(b'\n' as i8);
                    buf.add((u_var5 as u16).wrapping_add(1) as usize).write(0);
                }
            }
        }
        FUN_0048fce0(buf, (-0xffi32).wrapping_sub(param_1 as i32));
    }
}
