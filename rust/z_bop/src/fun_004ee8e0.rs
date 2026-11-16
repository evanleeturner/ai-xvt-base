// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa6108: i32;
    static mut DAT_00aa60f8: i32;
    static mut DAT_00aa6104: i32;
    static mut DAT_00aa6100: i32;
    static mut DAT_00a92202: u8;
    static mut DAT_00a92203: u8;
    static mut DAT_00a91d12: u8;
    static mut DAT_0052c564: u32;
    static mut DAT_0052c578: u32;
    static mut DAT_00aa5e10: i32;
    static mut DAT_00aa5cb0: i32;
    static mut DAT_00aa5ac0: i32;
    static mut DAT_00aa615e: u8;
    static mut DAT_00aa6166: u8;
    static mut DAT_00aa616a: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ee8e0(param_1: i32) -> u32 {
    // SAFETY: all globals are the game's own tables; accesses mirror the original.
    unsafe {
        let a92202 = &raw mut DAT_00a92202;
        let a92203 = &raw mut DAT_00a92203;
        let a91d12 = &raw mut DAT_00a91d12;
        let u_var2: u32;
        if param_1 == -1 {
            if DAT_00aa6108 == 0 {
                if DAT_00aa60f8 != 0 {
                    let idx = (DAT_00aa60f8 as u32)
                        .wrapping_add((DAT_00aa6104 as u32).wrapping_mul(0x562));
                    return a92202.add(idx as usize).read() as u32;
                }
                let idx = (DAT_00aa6104 as u32).wrapping_mul(0x562);
                return a91d12.add(idx as usize).read() as u32;
            }
            if DAT_00aa6100 == 0 {
                let idx = (DAT_00aa6104 as u32).wrapping_mul(0x562);
                return a91d12.add(idx as usize).read() as u32;
            }
            let idx = (DAT_00aa6104 as u32).wrapping_mul(0x562);
            let b_var1 = a92202.add(idx as usize).read();
            u_var2 = DAT_00aa6100 as u32;
            if b_var1 != 0 {
                if b_var1 < 3 {
                    return (&raw mut DAT_0052c564)
                        .add(DAT_00aa6100 as u32 as usize)
                        .read();
                }
                if b_var1 == 3 {
                    return (&raw mut DAT_0052c578)
                        .add(DAT_00aa6100 as u32 as usize)
                        .read();
                }
                return DAT_00aa6100 as u32;
            }
            return u_var2;
        }
        let mut local_4: i32 = 0;
        let mut i_var4: i32 = 0;
        let mut pi_var6 = &raw mut DAT_00aa5e10;
        let end = (&raw mut DAT_00aa5e10).add(10);
        let mut local_8 = &raw mut DAT_00aa5cb0;
        let rec = (param_1 as u32).wrapping_mul(0x2a) as usize;
        loop {
            let mut i_var5: i32 = 0;
            if 0 < pi_var6.read() {
                let mut pi_var3 = local_8;
                loop {
                    let tmp = (&raw mut DAT_00aa615e)
                        .add(rec)
                        .cast::<i32>()
                        .read_unaligned();
                    if pi_var3.read() == tmp {
                        local_4 = (&raw mut DAT_00aa5ac0)
                            .offset((i_var5.wrapping_add(i_var4)) as isize)
                            .read();
                        break;
                    }
                    pi_var3 = pi_var3.add(1);
                    i_var5 = i_var5.wrapping_add(1);
                    if i_var5 >= pi_var6.read() {
                        break;
                    }
                }
            }
            i_var4 = i_var4.wrapping_add(8);
            pi_var6 = pi_var6.add(1);
            local_8 = local_8.add(8);
            if pi_var6 >= end {
                break;
            }
        }
        let mut u = (&raw mut DAT_00aa6166)
            .add(rec)
            .cast::<u32>()
            .read_unaligned();
        if u == 0 {
            u = (&raw mut DAT_00aa616a)
                .add(rec)
                .cast::<u32>()
                .read_unaligned();
            if u != 0xffff_ffff && u < 10 {
                let idx = u.wrapping_add((local_4 as u32).wrapping_mul(0x562));
                return a92203.add(idx as usize).read() as u32;
            }
            let idx = (local_4 as u32).wrapping_mul(0x562);
            u = a91d12.add(idx as usize).read() as u32;
        }
        u
    }
}
