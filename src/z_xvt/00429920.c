/* SPDX-License-Identifier: MIT */
/* z_xvt 00429920 */
extern int __cdecl FUN_00429a20(unsigned int param_2);
extern unsigned int __cdecl FUN_00478320(unsigned int param_1, unsigned int param_2);

extern unsigned short DAT_00524778[];
extern unsigned int DAT_006d9d70;
extern int DAT_006d4988;
extern int DAT_006d9d64;
extern int DAT_006d9d68;
extern int DAT_006d9d6c;
extern int DAT_006d499c;

int __cdecl FUN_00429920(unsigned int param_1, unsigned int param_2, int param_3)
{
    int ret;
    unsigned int uVar2;
    unsigned int uVar3;
    int iVar4;
    int iVar5;
    int iVar6;

    ret = FUN_00429a20(param_2);
    if (ret != 0) {
        uVar2 = FUN_00478320(0x500, DAT_00524778[DAT_006d9d70]);
        uVar2 = (uVar2 & 0xFFFF) + 0xA00;
        if (param_3 != 0) {
            uVar3 = FUN_00478320(uVar2, 0x5555);
            uVar2 = uVar2 + (uVar3 & 0xFFFF);
        }
        iVar6 = param_2 * 0x23 + DAT_006d4988;
        iVar4 = DAT_006d9d64 - *(int *)(iVar6 + 4);
        iVar5 = DAT_006d9d68 - *(int *)(iVar6 + 8);
        iVar6 = DAT_006d9d6c - *(int *)(iVar6 + 0xC);
        if (iVar4 < 0) {
            iVar4 = -iVar4;
        }
        if (iVar5 < 0) {
            iVar5 = -iVar5;
        }
        if (iVar6 < 0) {
            iVar6 = -iVar6;
        }
        if (iVar5 < iVar4) {
            DAT_006d499c = (iVar5 >> 1) + iVar4;
        } else {
            DAT_006d499c = (iVar4 >> 1) + iVar5;
        }
        if (iVar6 < DAT_006d499c) {
            iVar6 = iVar6 >> 1;
        } else {
            DAT_006d499c = DAT_006d499c >> 1;
        }
        DAT_006d499c = DAT_006d499c + iVar6;
        if (DAT_006d499c < (int)(uVar2 * 0x100)) {
            return 1;
        }
    }
    return 0;
}
