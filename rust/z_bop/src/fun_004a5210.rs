// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00622c4c: u32;
    static mut DAT_00622c48: u32;
    static mut _DAT_00622bc0: u32;
    static mut DAT_00622bc8: u32;
    static mut DAT_00622bd0: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn mciSendCommandA(param_1: u32, param_2: u32, param_3: u32, param_4: u32) -> u32;
    fn auxGetNumDevs() -> u32;
    fn auxGetDevCapsA(device_id: u32, caps: *mut core::ffi::c_void, size: u32) -> u32;
    fn auxSetVolume(device_id: u32, volume: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004a5210() -> u32 {
    let mut au_stack_34 = [0u8; 28];
    let mut aux_caps = [0u8; 0x30];
    // SAFETY: the globals and the winmm/MCI functions are the game's own;
    // the local buffers outlive the calls that receive their addresses, and
    // the zeroed region is the game's 0x1e-dword block at DAT_00622bd0.
    unsafe {
        if DAT_00622c4c == 0 {
            return 0;
        }
        if DAT_00622c48 != 0 {
            mciSendCommandA(DAT_00622c4c, 0x808, 0, au_stack_34.as_mut_ptr() as u32);
            DAT_00622c48 = 0;
            _DAT_00622bc0 = 0;
        }
        let mut u_device_id: u32 = 0;
        mciSendCommandA(DAT_00622c4c, 0x804, 0, 0);
        DAT_00622c4c = 0;
        let mut p = &raw mut DAT_00622bd0;
        let mut i: i32 = 0x1e;
        while i != 0 {
            p.write_unaligned(0);
            p = p.add(1);
            i -= 1;
        }
        DAT_00622c48 = 0;
        _DAT_00622bc0 = 0;
        let u_num_devs = auxGetNumDevs();
        let mut mmr_result = DAT_00622bc8;
        if DAT_00622bc8 != 0xffff_ffff {
            let dw_volume = DAT_00622bc8.wrapping_mul(0x10001);
            if 0 < (u_num_devs as i32) {
                loop {
                    aux_caps = [0u8; 0x30];
                    mmr_result = auxGetDevCapsA(
                        u_device_id,
                        aux_caps.as_mut_ptr().cast::<core::ffi::c_void>(),
                        0x30,
                    );
                    if aux_caps
                        .as_ptr()
                        .byte_add(0x18)
                        .cast::<u16>()
                        .read_unaligned()
                        == 1
                    {
                        let support = aux_caps
                            .as_ptr()
                            .byte_add(0x14)
                            .cast::<u32>()
                            .read_unaligned();
                        mmr_result = support;
                        if (support & 1) != 0 {
                            mmr_result = auxSetVolume(u_device_id, dw_volume);
                        }
                    }
                    u_device_id = u_device_id.wrapping_add(1);
                    if !((u_device_id as i32) < (u_num_devs as i32)) {
                        break;
                    }
                }
            }
        }
        DAT_00622bc8 = 0xffff_ffff;
        mmr_result
    }
}
