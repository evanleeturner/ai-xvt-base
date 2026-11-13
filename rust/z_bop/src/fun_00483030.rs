// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: i32;
    static mut DAT_00521f1c: [i8; 0];
    static mut DAT_0051c659: [i8; 0];
    static mut DAT_0051c65b: [i8; 0];
    static mut DAT_0051c65d: [i8; 0];
    static mut DAT_0051c65f: [i8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00483030(param_1: i32) -> i32 {
    // SAFETY: the globals and the pointers derived from them are the game's
    // own memory, read where the original reads.
    unsafe {
        let i_var4 = param_1.wrapping_mul(0x23).wrapping_add(DAT_009a1fe8);
        let local_1f = ((i_var4.wrapping_add(0x1f)) as *const i32).read_unaligned();
        if local_1f == 0 {
            let idx = ((i_var4.wrapping_add(3)) as *const u8).read() as usize * 0x1a;
            return (&raw mut DAT_00521f1c)
                .cast::<u8>()
                .add(idx)
                .cast::<i32>()
                .read_unaligned();
        }
        let local_a9 = ((local_1f.wrapping_add(0xa9)) as *const i32).read_unaligned();
        if local_a9 != 0 {
            let u_var1 = ((local_a9.wrapping_add(4)) as *const u8).read() as u32;
            let u_var3 = (u_var1 | (u_var1 << 8)) as u16 as usize;
            let a = (&raw mut DAT_0051c65b)
                .cast::<u8>()
                .add(u_var3)
                .cast::<i16>()
                .read_unaligned();
            let b = (&raw mut DAT_0051c65d)
                .cast::<u8>()
                .add(u_var3)
                .cast::<i16>()
                .read_unaligned();
            let c = (&raw mut DAT_0051c65f)
                .cast::<u8>()
                .add(u_var3)
                .cast::<i16>()
                .read_unaligned();
            let d = (&raw mut DAT_0051c659).cast::<u8>().add(u_var3).read();
            let sum = (a as i32).wrapping_add(b as i32).wrapping_add(c as i32);
            return (sum / 3).wrapping_shl((d & 0x1f) as u32);
        }
        let idx = ((i_var4.wrapping_add(3)) as *const u8).read() as usize * 0x1a;
        (&raw mut DAT_00521f1c)
            .cast::<u8>()
            .add(idx)
            .cast::<i32>()
            .read_unaligned()
    }
}
