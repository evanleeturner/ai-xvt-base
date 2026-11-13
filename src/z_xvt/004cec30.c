/* SPDX-License-Identifier: MIT */
/* z_xvt 004cec30 */
extern unsigned int DAT_008050aa;
extern unsigned int DAT_008050ae;
extern unsigned int DAT_008050a6;
extern unsigned int DAT_007810a6[];

unsigned int __cdecl FUN_004cec30(unsigned int param_1)
{
    int iVar1;
    unsigned int uVar2;
    unsigned int uVar3;
    unsigned int uVar4;
    unsigned int *puVar5;
    unsigned int *puVar6;

    if (DAT_008050aa == param_1) {
        DAT_008050aa = DAT_008050aa + 1;
        DAT_008050ae = DAT_008050ae - 1;
        if (0x3ff < DAT_008050aa) {
            DAT_008050aa = 0;
        }
        return 1;
    }
    uVar3 = param_1 + 1;
    if (0x3ff < uVar3) {
        uVar3 = 0;
    }
    uVar4 = DAT_008050ae + DAT_008050aa;
    if (0x3ff < uVar4) {
        uVar4 = uVar4 - 0x400;
    }
    while (uVar4 != uVar3) {
        uVar2 = param_1 + 1;
        puVar5 = DAT_007810a6 + uVar3 * 0x84;
        puVar6 = DAT_007810a6 + param_1 * 0x84;
        for (iVar1 = 0x84; iVar1 != 0; iVar1 = iVar1 - 1) {
            *puVar6 = *puVar5;
            puVar5 = puVar5 + 1;
            puVar6 = puVar6 + 1;
        }
        if (0x3ff < uVar2) {
            uVar2 = 0;
        }
        uVar3 = uVar3 + 1;
        param_1 = uVar2;
        if (0x3ff < uVar3) {
            uVar3 = 0;
        }
    }
    DAT_008050ae = DAT_008050ae - 1;
    if (DAT_008050a6 == 0) {
        DAT_008050a6 = 0x3ff;
        return 0;
    }
    DAT_008050a6 = DAT_008050a6 - 1;
    return 0;
}
