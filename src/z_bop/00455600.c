/* SPDX-License-Identifier: MIT */
/* z_bop 00455600 */
extern unsigned short DAT_009a1834;
extern unsigned char DAT_009a20c0[];
extern unsigned char DAT_009d8c82[];
extern unsigned char DAT_00524170[];
extern unsigned char DAT_00521f2e[];
extern unsigned char DAT_009d8c91[];
extern unsigned char DAT_009a2062;

extern unsigned long long FUN_00456190(unsigned short param_1);
extern void FUN_004587c0(unsigned short param_1);

unsigned int FUN_00455600(unsigned short param_1)
{
    unsigned int uVar3 = (unsigned int)DAT_009a1834;
    int iVar1 = uVar3 * 0x126;
    DAT_009a20c0[iVar1] = 1;
    int iVar2 = uVar3 * 0x566;
    if ((DAT_00521f2e[DAT_00524170[DAT_009d8c82[iVar2]] * 0x1a] & 0x80) != 0) {
        DAT_009a20c0[iVar1 + 1] = DAT_009d8c91[iVar2];
        unsigned long long uVar4 = FUN_00456190(param_1);
        return ((unsigned int)(uVar4 >> 0x10) << 0x10) | 1;
    }
    if (DAT_009a2062 == 0x02) {
        DAT_009a20c0[iVar1 + 1] = DAT_009d8c91[iVar2];
    } else {
        DAT_009a20c0[iVar1 + 1] = 0;
    }
    FUN_004587c0(param_1);
    return 1;
}
