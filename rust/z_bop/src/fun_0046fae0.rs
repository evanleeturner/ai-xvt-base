// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00999548: u32;
    static mut DAT_0099f6e0: u32;
    static mut DAT_00999e0c: [u32; 1];
    static mut DAT_005597a0: u32;
    static mut DAT_005597a4: u32;
    static mut DAT_005597a8: u32;
    static mut DAT_005597ac: u32;
    static mut DAT_005597b0: u32;
    static mut DAT_00999bf8: [u32; 1];
    static mut DAT_00999c00: [u32; 1];
    static mut DAT_00999bf0: [u32; 1];
    static mut DAT_00999be8: [u32; 1];
    static s_Sending_RRA_0052709c: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0046dfa0(param_1: *mut u32) -> *mut u32;
    fn FUN_0046fee0(param_1: i32) -> u32;
    fn FUN_0046c220(param_1: *const i8);
    fn FUN_0046f470(param_1: u32, param_2: *mut u32, param_3: u32, param_4: u32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn timeGetTime() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0046fae0() -> u32 {
    let mut local_4: u32 = 0;
    let mut u_var5: u32 = 0;
    // SAFETY: all callees and globals are the game's own memory; the pointer
    // returned by FUN_0046dfa0 addresses the entries read as the original does,
    // and the indexed tables are bounds-limited by the checks on u_var3.
    unsafe {
        let p_var1 = FUN_0046dfa0(&raw mut local_4);
        if local_4 != 0 {
            let mut pi_var4 = p_var1.add(8) as *mut i32;
            loop {
                let d_var2 = timeGetTime();
                let cur = pi_var4.read_unaligned();
                if DAT_00999548 != cur as u32 {
                    let u_var3 = FUN_0046fee0(cur);
                    if u_var3 < DAT_0099f6e0 && u_var3 < 8 {
                        let idx = (u_var3 as usize).wrapping_mul(0x8e);
                        let t = (&raw mut DAT_00999e0c).cast::<u32>().add(idx);
                        if d_var2.wrapping_sub(t.read_unaligned()) > 3000 {
                            t.write_unaligned(d_var2);
                            FUN_0046c220(&raw const s_Sending_RRA_0052709c);
                            DAT_005597a0 = 0x37;
                            DAT_005597a4 = (&raw mut DAT_00999bf8)
                                .cast::<u32>()
                                .add(idx)
                                .read_unaligned()
                                .wrapping_add(1);
                            if 0x7f < DAT_005597a4 {
                                DAT_005597a4 = 0;
                            }
                            DAT_005597a8 = (&raw mut DAT_00999c00)
                                .cast::<u32>()
                                .add(idx)
                                .read_unaligned()
                                .wrapping_add(1);
                            if 0x7f < DAT_005597a8 {
                                DAT_005597a8 = 0;
                            }
                            DAT_005597ac = (&raw mut DAT_00999bf0)
                                .cast::<u32>()
                                .add(idx)
                                .read_unaligned()
                                .wrapping_add(1);
                            if 0x7f < DAT_005597ac {
                                DAT_005597ac = 0;
                            }
                            DAT_005597b0 = timeGetTime();
                            FUN_0046f470(
                                (&raw mut DAT_00999be8)
                                    .cast::<u32>()
                                    .add(idx)
                                    .read_unaligned(),
                                &raw mut DAT_005597a0,
                                0x14,
                                DAT_005597b0,
                            );
                        }
                    }
                }
                pi_var4 = pi_var4.add(10);
                u_var5 = u_var5.wrapping_add(1);
                if !(u_var5 < local_4) {
                    break;
                }
            }
        }
    }
    1
}
