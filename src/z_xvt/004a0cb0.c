/* SPDX-License-Identifier: MIT */
/* z_xvt 004a0cb0 */
extern unsigned char *DAT_006d4988;
extern unsigned char DAT_0070ad48[];
extern unsigned char DAT_0070ad42[];
extern unsigned char DAT_0072f508[];
extern unsigned char DAT_005247e8[];
extern unsigned char DAT_0052686a[];
extern char DAT_006d4a02;
extern char DAT_007092d4;
extern int DAT_00709360[];
extern int DAT_0070934c[];
extern int DAT_00709338[];
extern int DAT_00709310[];

int __cdecl FUN_004a1980(unsigned int param_1);
int __cdecl FUN_004a1590(int param_1, unsigned int param_2, int param_3, int param_4, int param_5, int param_6);

void __cdecl FUN_004a0cb0(unsigned int param_1, int param_2, int param_3, int param_4)
{
    unsigned char bVar1;
    unsigned int uVar2;
    int iVar3;
    int iVar4;
    unsigned int uVar5;
    unsigned short uVar6;

    uVar5 = param_1 & 0xffff;
    uVar6 = 1;
    bVar1 = *(unsigned char *)(uVar5 * 0x23 + 0x16 + DAT_006d4988);
    if (DAT_0072f508[param_4 * 0x1e5 + (unsigned int)DAT_0070ad48[(unsigned int)bVar1 * 0x566]] == 0) {
        uVar2 = (unsigned int)DAT_005247e8[DAT_0070ad42[(unsigned int)*(unsigned char *)(uVar5 * 0x23 + 0x16 + DAT_006d4988) * 0x566]];
        if (DAT_0052686a[uVar2 * 0x16] == 0) {
            iVar3 = FUN_004a1980(uVar5);
        }
        else {
            if (((uVar2 < 0x46) || (0x4a < uVar2)) && ((uVar2 < 0x50 || (0x54 < uVar2)))) {
                iVar3 = 0x28;
            }
            else {
                iVar3 = 500;
            }
            if (DAT_006d4a02 == 2) {
                iVar3 = (int)((unsigned int)iVar3 * 3u);
            }
        }
        if (param_3 == 1) {
            DAT_00709360[param_4] = (int)((unsigned int)DAT_00709360[param_4] + 1u);
            uVar6 = 10;
            iVar3 = iVar3 / 10;
        }
        else if (param_3 == 2) {
            DAT_0070934c[param_4] = (int)((unsigned int)DAT_0070934c[param_4] + 1u);
            uVar6 = 6;
            iVar3 = iVar3 / 2;
        }
        else if (param_3 == 3) {
            DAT_00709338[param_4] = (int)((unsigned int)DAT_00709338[param_4] + 1u);
            uVar6 = 1;
        }
        if (DAT_007092d4 == 2) {
            iVar3 = (int)((unsigned int)iVar3 * 2u);
        }
        else if (DAT_007092d4 == 0) {
            iVar3 = iVar3 / 2;
        }
        /* NOTE: the original's 2nd arg is (EBX & 0xffff0000) | bVar1; the upper 16 bits
           are an unresolved caller-register leftover (unaff_EBX), not reproducible in C. */
        iVar4 = FUN_004a1590(2, (unsigned int)bVar1, -1, (int)uVar6, param_2, param_4);
        if (-1 < iVar4) {
            DAT_00709310[param_4] = (int)((unsigned int)DAT_00709310[param_4] + (unsigned int)iVar3);
            return;
        }
    }
    else if ((param_3 == 2) || (param_3 == 3)) {
        uVar2 = (unsigned int)DAT_005247e8[DAT_0070ad42[(unsigned int)*(unsigned char *)(uVar5 * 0x23 + 0x16 + DAT_006d4988) * 0x566]];
        if (DAT_0052686a[uVar2 * 0x16] == 0) {
            iVar3 = FUN_004a1980(uVar5);
        }
        else {
            if (((uVar2 < 0x46) || (0x4a < uVar2)) && ((uVar2 < 0x50 || (0x54 < uVar2)))) {
                iVar3 = 0x28;
            }
            else {
                iVar3 = 500;
            }
            if (DAT_006d4a02 == 2) {
                iVar3 = (int)((unsigned int)iVar3 * 3u);
            }
        }
        DAT_00709310[param_4] = (int)((unsigned int)DAT_00709310[param_4] - (unsigned int)iVar3);
    }
    return;
}
