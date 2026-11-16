/* SPDX-License-Identifier: MIT */
/* z_bop 00415a70 */
extern unsigned int FUN_004269e0(unsigned int param_1);
extern unsigned int FUN_00415c40(unsigned int param_1, unsigned int param_2, unsigned char *param_3);
extern unsigned int FUN_004a9960(unsigned char *param_1);

extern unsigned char DAT_0051bea0[];
extern unsigned char DAT_00a65ce0[];
extern unsigned char DAT_00a60a60[];
extern unsigned char DAT_00a633c0[];
extern unsigned char DAT_00a686e0[];

unsigned int __cdecl FUN_00415a70(unsigned int param_1, unsigned short param_2, unsigned short param_3, short param_4)
{
    unsigned int uVar1;
    unsigned int uVar2;
    unsigned short extraout_var = 0;
    unsigned int extraout_EDX = 0;
    unsigned char *pbVar3;
    unsigned short uVar4;
    unsigned int uVar5;

    pbVar3 = (unsigned char *)0x0;
    uVar2 = 0;
    uVar4 = 0;
    if (DAT_0051bea0[param_2] != 0x01) {
        uVar2 = (unsigned int)param_3;
    }
    uVar5 = (unsigned int)(unsigned short)(param_2 + param_4);
    uVar1 = FUN_004269e0(param_1);
    if ((short)uVar1 == 0xff) {
        if ((0x45 < (unsigned short)param_1) && ((unsigned short)param_1 < 0x55)) {
            pbVar3 = &DAT_00a65ce0[uVar2 + uVar5 * 0xe];
            uVar2 = FUN_00415c40(0, 0, pbVar3);
            uVar4 = (unsigned short)uVar2;
            uVar1 = FUN_004a9960(pbVar3);
        }
        return ((unsigned int)(short)(uVar1 >> 0x10) << 0x10) | (unsigned int)uVar4;
    }
    uVar1 = (unsigned int)(unsigned char)DAT_00a686e0[uVar1 & 0xffff];
    if (uVar1 == 0) {
        pbVar3 = &DAT_00a65ce0[uVar2 + uVar5 * 0xe];
    }
    else if (uVar1 == 1) {
        pbVar3 = &DAT_00a60a60[uVar2 + uVar5 * 0xe];
    }
    else if (uVar1 == 2) {
        pbVar3 = &DAT_00a633c0[uVar2 + uVar5 * 0xe];
    }
    uVar2 = FUN_00415c40(uVar1, extraout_EDX, pbVar3);
    FUN_004a9960(pbVar3);
    return ((unsigned int)extraout_var << 0x10) | (unsigned int)(short)uVar2;
}
