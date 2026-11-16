// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_005254e0: u32;
    static mut DAT_006da850: [u16; 1];
    static mut DAT_0071bb1b: [u8; 1];
    static mut DAT_0071bb1f: [u8; 1];
    static mut DAT_0071bb23: [u8; 1];
    static mut DAT_0071ecf4: i32;
    static mut _DAT_005201ec: f32;
    static mut _DAT_005201f0: f32;

    static mut DAT_0073255c: i32;
    static mut DAT_00732558: i32;
    static mut DAT_00732568: i32;
    static mut DAT_00732578: i32;
    static mut DAT_00732584: i32;
    static mut DAT_0073253c: i32;
    static mut DAT_00732530: i32;
    static mut DAT_00732548: i32;
    static mut DAT_00732538: i32;
    static mut DAT_0070a138: i32;
    static mut DAT_0070a134: i32;
    static mut DAT_0070a130: i32;
    static mut DAT_0070a12c: i32;
    static mut DAT_0070a010: i32;
    static mut DAT_0070a00c: i32;
    static mut _DAT_0070a008: i32;
    static mut _DAT_0070a124: i32;
    static mut _DAT_0070a120: i32;

    static mut DAT_00602678: i32;
    static mut DAT_0052a470: *mut u32;
    static mut DAT_006026c8: u32;
    static mut DAT_006026cc: u32;
    static mut DAT_006026d8: u32;
    static mut _DAT_006026dc: u32;
    static mut DAT_006026e0: u32;
    static mut DAT_0052a478: u32;
    static mut DAT_006026a0: i32;
    static mut _DAT_006026c0: i32;
    static mut DAT_006026a4: i32;
    static mut _DAT_006026a8: i32;
    static mut DAT_00602690: i32;
    static mut DAT_006026bc: i32;
    static mut DAT_0052a46c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048f4f0(param_1: u32) -> *mut i32;
    fn FUN_00486000(param_1: *mut i32, param_2: i32);
    fn FUN_00414fe0(param_1: *mut f32, param_2: *mut f32);
    fn FUN_00486080(param_1: *mut c_void, param_2: *mut c_void, param_3: i32, param_4: i32);
    fn FUN_00486b80(param_1: i32, param_2: *mut u32, param_3: *mut c_void);
    fn FUN_0048f510();
}

#[repr(C)]
struct Frame {
    local_d0: i32,
    local_cc: f32,
    local_c8: f32,
    local_c4: f32,
    local_c0: f32,
    local_bc: f32,
    local_b8: f32,
    local_b4: f32,
    local_b0: f32,
    local_ac: f32,
    local_a8: f32,
    local_a4: f32,
    local_a0: f32,
    local_9c: f32,
    local_98: f32,
    local_94: f32,
    local_90: f32,
    local_8c: f32,
    local_88: f32,
    local_84: f32,
    local_80: f32,
    local_7c: f32,
    local_78: f32,
    local_74: f32,
    local_70: f32,
    local_6c: f32,
    pad: [u32; 26],
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00486310(param_1: i32) {
    // SAFETY: all globals, callees and the pointers derived from param_1 are
    // the game's own memory, accessed exactly as the original does.
    unsafe {
        let p1 = param_1 as u32 as usize as *mut u8;

        let fld = p1.byte_add(0x1f).cast::<i32>().read_unaligned();
        if fld == 0 {
            _DAT_005254e0 = 0;
        } else {
            let pb = fld.wrapping_add(0x88) as u32 as usize as *const u8;
            _DAT_005254e0 = u32::from(pb.read());
        }

        let idx = p1.byte_add(3).read() as usize;
        let piVar3 = FUN_0048f4f0(u32::from(
            (&raw const DAT_006da850).cast::<u16>().add(idx).read(),
        ));
        let piVar4 = piVar3.read() as u32 as usize as *mut i32;
        if piVar3 != piVar4 {
            let b3 = piVar3 as *mut u8;
            let mut iVar7: i32 = 0;
            let iVar6: i32 = (piVar3 as u32).wrapping_sub(piVar4 as u32) as i32;
            piVar3.write((piVar4 as u32).wrapping_add(iVar6 as u32) as i32);
            let mut tmp = b3.byte_add(10).cast::<i32>().read_unaligned();
            if tmp != 0 {
                let mut iVar5: i32 = 0;
                tmp = tmp.wrapping_add(iVar6);
                b3.byte_add(10).cast::<i32>().write_unaligned(tmp);
                let mut tmp2 = b3.byte_add(6).cast::<i32>().read_unaligned();
                if 0 < tmp2 {
                    loop {
                        let t = b3.byte_add(10).cast::<i32>().read_unaligned();
                        let pp = t.wrapping_add(iVar7) as u32 as usize as *mut i32;
                        let iVar1 = pp.read_unaligned();
                        if iVar1 != 0 {
                            pp.write_unaligned(iVar1.wrapping_add(iVar6));
                            let t2 = b3.byte_add(10).cast::<i32>().read_unaligned();
                            let q = (t2.wrapping_add(iVar7) as u32 as usize as *const i32)
                                .read_unaligned() as u32
                                as usize as *mut i32;
                            FUN_00486000(q, iVar6);
                        }
                        iVar7 = iVar7.wrapping_add(4);
                        iVar5 = iVar5.wrapping_add(1);
                        tmp2 = b3.byte_add(6).cast::<i32>().read_unaligned();
                        if !(iVar5 < tmp2) {
                            break;
                        }
                    }
                }
            }
        }

        let mut frame: Frame = core::mem::zeroed();
        frame.local_d0 = param_1;

        let cur = (DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize;

        {
            let a = p1.byte_add(4).cast::<i32>().read_unaligned();
            let b = (&raw mut DAT_0071bb1b)
                .cast::<u8>()
                .add(cur)
                .cast::<i32>()
                .read_unaligned();
            frame.local_c8 = a.wrapping_sub(b) as f32;
            let a = p1.byte_add(8).cast::<i32>().read_unaligned();
            let b = (&raw mut DAT_0071bb1f)
                .cast::<u8>()
                .add(cur)
                .cast::<i32>()
                .read_unaligned();
            frame.local_c4 = a.wrapping_sub(b) as f32;
        }
        frame.local_bc = DAT_0073255c as f32 * _DAT_005201ec;
        frame.local_b8 = DAT_00732558 as f32 * _DAT_005201ec;
        frame.local_b4 = DAT_00732568 as f32 * _DAT_005201ec;
        frame.local_ac = DAT_00732578 as f32 * _DAT_005201ec;
        frame.local_b0 = DAT_00732584 as f32 * _DAT_005201ec;
        frame.local_a4 = DAT_0073253c as f32 * _DAT_005201ec;
        frame.local_a8 = DAT_00732530 as f32 * _DAT_005201ec;
        frame.local_9c = DAT_00732548 as f32 * _DAT_005201ec;
        frame.local_a0 = DAT_00732538 as f32 * _DAT_005201ec;
        {
            let a = p1.byte_add(0xc).cast::<i32>().read_unaligned();
            let b = (&raw mut DAT_0071bb23)
                .cast::<u8>()
                .add(cur)
                .cast::<i32>()
                .read_unaligned();
            frame.local_c0 = a.wrapping_sub(b) as f32;
        }
        FUN_00414fe0(&raw mut frame.local_c8, &raw mut frame.local_bc);

        frame.local_b8 = DAT_0070a138 as f32 * _DAT_005201ec;
        frame.local_b4 = DAT_0070a134 as f32 * _DAT_005201ec;
        frame.local_b0 = DAT_0070a130 as f32 * _DAT_005201ec;
        frame.local_bc = DAT_0070a12c as f32 * _DAT_005201ec;
        frame.local_ac = DAT_0070a010 as f32 * _DAT_005201ec;
        frame.local_a8 = DAT_0070a00c as f32 * _DAT_005201ec;
        frame.local_a4 = _DAT_0070a008 as f32 * _DAT_005201ec;
        frame.local_a0 = _DAT_0070a124 as f32 * _DAT_005201ec;
        frame.local_9c = _DAT_0070a120 as f32 * _DAT_005201ec;
        frame.local_98 = -frame.local_c8;
        frame.local_94 = -frame.local_c4;
        frame.local_90 = -frame.local_c0;
        frame.local_8c = frame.local_bc;
        frame.local_88 = frame.local_b0;
        frame.local_84 = frame.local_a4;
        frame.local_80 = frame.local_b8;
        frame.local_7c = frame.local_ac;
        frame.local_78 = frame.local_a0;
        frame.local_74 = frame.local_b4;
        frame.local_70 = frame.local_a8;
        frame.local_6c = frame.local_9c;
        FUN_00414fe0(&raw mut frame.local_98, &raw mut frame.local_8c);

        DAT_00602678 = 0;
        if DAT_0052a470.is_null() {
            DAT_0052a470 = &raw mut DAT_006026c8;
            _DAT_006026dc = 8;
            DAT_006026d8 = 8;
            DAT_006026cc = 0x10;
            DAT_006026c8 = 0x100;
            FUN_00486080(
                (&raw mut DAT_006026e0).cast::<c_void>(),
                (&raw mut DAT_0052a478).cast::<c_void>(),
                8,
                8,
            );
        }
        DAT_006026a0 = 0;
        _DAT_006026c0 = 0;
        let mut iVar7: i32 = 0;
        let mut iVar6: i32 = 0;
        DAT_006026a4 = DAT_0052a470 as usize as i32;
        _DAT_006026a8 = 0;
        DAT_00602690 = 0;
        DAT_006026bc = 0;

        let b3 = piVar3 as *mut u8;
        let mut tmp2 = b3.byte_add(6).cast::<i32>().read_unaligned();
        if 0 < tmp2 {
            let mut iVar5: i32 = 0;
            loop {
                frame.local_cc = 0.0;
                let tmp = b3.byte_add(10).cast::<i32>().read_unaligned();
                let puVar2 = (tmp.wrapping_add(iVar5) as u32 as usize as *const i32)
                    .read_unaligned() as u32 as usize as *mut u32;
                'done: {
                    'lab: {
                        if puVar2.add(1).read() == 0x14 {
                            break 'lab;
                        }
                        iVar7 = iVar7.wrapping_add(1);
                        let fld = p1.byte_add(0x1f).cast::<i32>().read_unaligned();
                        if fld == 0 {
                            break 'lab;
                        }
                        let iVar1 = (fld as u32 as usize as *const u8)
                            .byte_add(0xa7)
                            .cast::<i32>()
                            .read_unaligned();
                        if iVar1 == 0 {
                            break 'lab;
                        }
                        let a1 = iVar1.wrapping_add(0x2c9).wrapping_add(iVar7) as u32 as usize
                            as *const i8;
                        if a1.read() == 0 {
                            let a2 = iVar1.wrapping_add(0x2f1).wrapping_add(iVar7) as u32 as usize
                                as *const u8;
                            frame.local_cc = f32::from(a2.read()) * _DAT_005201f0;
                            break 'lab;
                        }
                        break 'done;
                    }
                    DAT_0052a46c = DAT_0052a46c.wrapping_add(1);
                    FUN_00486b80(
                        piVar3 as usize as i32,
                        puVar2,
                        (&raw mut frame).cast::<c_void>(),
                    );
                }
                iVar5 = iVar5.wrapping_add(4);
                iVar6 = iVar6.wrapping_add(1);
                tmp2 = b3.byte_add(6).cast::<i32>().read_unaligned();
                if !(iVar6 < tmp2) {
                    break;
                }
            }
        }
        FUN_0048f510();
    }
}
