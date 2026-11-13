// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071ed02: u16;
    static mut DAT_0071ed04: u16;
    static mut DAT_0071ed0c: u16;
    static mut _DAT_006db794: u32;
    static mut _DAT_0071b70c: u32;
    static mut DAT_007325d0: u32;
    static mut _DAT_00732e4e: u16;
    static mut DAT_006db580: u32;
    static mut DAT_007327f0: u32;
    static mut DAT_00524570: [u16; 0];
    static mut DAT_006da9f0: u16;
    static mut DAT_00732e40: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00419000(param_1: u32, param_2: u32, param_3: *mut u16, param_4: *mut u16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00418cd0(param_1: u32, param_2: u32, param_3: u32) {
    let mut param_1 = param_1;
    let mut param_2 = param_2;
    let mut param_3 = param_3;
    let mut local_4: u32 = 0;
    let hi: *mut u16;
    let lo: *mut u16;
    // SAFETY: the globals and callee are the game's own; hi/lo point into
    // the local u32, which the callee fills through u16 pointers.
    unsafe {
        let base = (&raw mut local_4).cast::<u16>();
        lo = base;
        hi = base.add(1);

        let b = (param_1 as i32) < 0;
        if b {
            param_1 = 0u32.wrapping_sub(param_1);
        }
        DAT_0071ed02 = b as u16;
        let b = (param_2 as i32) < 0;
        if b {
            param_2 = 0u32.wrapping_sub(param_2);
        }
        DAT_0071ed04 = b as u16;
        let b = (param_3 as i32) < 0;
        if b {
            param_3 = 0u32.wrapping_sub(param_3);
        }
        DAT_0071ed0c = b as u16;
        _DAT_006db794 = param_1;
        _DAT_0071b70c = param_2;
        DAT_007325d0 = param_3;

        let tab = (&raw mut DAT_00524570).cast::<u16>();

        FUN_00419000(param_1, param_2, hi, lo);
        _DAT_00732e4e = hi.read();
        let t = u32::from(tab.add(lo.read() as usize).read());
        let d = DAT_007327f0;
        DAT_006db580 = (((d & 0xffff).wrapping_mul(t).wrapping_add(0x8000)) >> 16)
            .wrapping_add((d >> 16).wrapping_mul(t))
            .wrapping_add(d);
        let mut s = hi.read() as i16;
        if DAT_0071ed04 != 0 {
            s = (0u32.wrapping_sub(s as i32 as u32)) as i16;
        }
        if DAT_0071ed02 != 0 {
            s = (0xffff_8000u32.wrapping_sub(s as i32 as u32)) as i16;
        }
        DAT_006da9f0 = (0x4000u32.wrapping_sub(s as i32 as u32)) as u16;

        FUN_00419000(DAT_006db580, DAT_007325d0, hi, lo);
        _DAT_00732e4e = hi.read();
        let t = u32::from(tab.add(lo.read() as usize).read());
        let d = DAT_007327f0;
        DAT_006db580 = d
            .wrapping_add(((d & 0xffff).wrapping_mul(t).wrapping_add(0x8000)) >> 16)
            .wrapping_add((d >> 16).wrapping_mul(t));
        if DAT_0071ed0c != 0 {
            hi.write((0u32.wrapping_sub(u32::from(hi.read()))) as u16);
        }
        DAT_00732e40 = (0x4000u32.wrapping_sub(u32::from(hi.read()))) as u16;
    }
}
