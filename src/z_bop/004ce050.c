/* SPDX-License-Identifier: MIT */
/* z_bop 004ce050 */
extern undefined4 DAT_005182a0;
extern undefined4 DAT_005182a4;
extern undefined4 DAT_005182a8;
extern undefined4 DAT_005182ac;
extern undefined4 DAT_005182b0;
extern undefined4 DAT_005182b4;
extern undefined4 DAT_005182b8;
extern undefined4 DAT_005182bc;
extern undefined4 DAT_005182c0;
extern undefined4 DAT_005182c4;
extern undefined4 DAT_005182c8;
extern undefined4 DAT_005182cc;
extern undefined4 DAT_005182d0;
extern undefined4 DAT_005182d4;
extern undefined4 DAT_005182d8;
extern undefined4 DAT_005182dc;

extern undefined4 DAT_00665040;
extern undefined4 DAT_00665044;
extern undefined4 DAT_00665048;
extern undefined4 DAT_0066504c;

undefined * __cdecl FUN_004ce050(undefined4 param_1)
{
    switch (param_1) {
        case 0:
            DAT_00665040 = DAT_005182a0;
            DAT_00665044 = DAT_005182a4;
            DAT_00665048 = DAT_005182a8;
            DAT_0066504c = DAT_005182ac;
            return (undefined *)&DAT_00665040;
        case 1:
            DAT_00665040 = DAT_005182c0;
            DAT_00665044 = DAT_005182c4;
            DAT_00665048 = DAT_005182c8;
            DAT_0066504c = DAT_005182cc;
            return (undefined *)&DAT_00665040;
        case 2:
            DAT_00665040 = DAT_005182b0;
            DAT_00665044 = DAT_005182b4;
            DAT_00665048 = DAT_005182b8;
            DAT_0066504c = DAT_005182bc;
            return (undefined *)&DAT_00665040;
        case 3:
            DAT_00665040 = DAT_005182d0;
            DAT_00665044 = DAT_005182d4;
            DAT_00665048 = DAT_005182d8;
            DAT_0066504c = DAT_005182dc;
            return (undefined *)&DAT_00665040;
        default:
            return (undefined *)0x0;
    }
}
