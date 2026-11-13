// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: i32;
    static mut DAT_0052b08d: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a7aa0(param_1: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040efa0(param_1: u32) -> i32 {
    // SAFETY: the globals, the callee and every pointer dereferenced here
    // belong to the game's own memory; reads happen in the original's order.
    unsafe {
        let i_var2 = ((param_1 & 0xffff) as i32).wrapping_mul(0x23);
        let i_var3 = DAT_006d4988.wrapping_add(i_var2);
        let pc_var1: *const i8 =
            ((i_var3.wrapping_add(0x1f)) as usize as *const *const i8).read_unaligned();
        if pc_var1.is_null() {
            return 0;
        }
        let u_var4: u32;
        if *pc_var1 == 0 {
            let b: u8 = ((i_var3.wrapping_add(3)) as usize as *const u8).read_unaligned();
            let mut arg: u32 = ((i_var3 as u32) >> 8) | (b as u32);
            arg &= 0xffff00ff;
            let r = FUN_004a7aa0(arg);
            let base = (&raw mut DAT_0052b08d) as usize as i32;
            let addr = base.wrapping_add(((r & 0xffff) as i32).wrapping_mul(0xef));
            let v4 = (addr as usize as *const u16).read_unaligned() as u32;
            let v5 = (pc_var1.byte_add(0x79).cast::<u16>()).read_unaligned() as u32;
            if v4 < v5 {
                return (v5 << 0xc) as i32;
            }
            u_var4 = v4;
        } else {
            if *pc_var1 != 1 {
                return 0;
            }
            let ptr: i32 = pc_var1.byte_add(0xa3).cast::<i32>().read_unaligned();
            if ptr != 0 {
                let t = (ptr.wrapping_add(6) as usize as *const u16).read_unaligned();
                return ((t as u32) << 0xc) as i32;
            }
            u_var4 = pc_var1.byte_add(0x79).cast::<u16>().read_unaligned() as u32;
        }
        (u_var4 << 0xc) as i32
    }
}
