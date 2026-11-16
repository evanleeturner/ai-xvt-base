/* SPDX-License-Identifier: MIT */
/* z_bop 0046fc00 */
extern int DAT_0099f6e0;
extern int DAT_00999bf0[];
extern int DAT_00999c00[];
extern int DAT_00999bf8[];
extern unsigned int FUN_0046fee0(int param_1);

int __cdecl FUN_0046fc00(int param_1, int param_2, int param_3, int param_4)
{
    int iVar2;
    unsigned int uVar1;

    iVar2 = DAT_0099f6e0;
    uVar1 = FUN_0046fee0(param_1);
    if (DAT_0099f6e0 != iVar2 || 0x27 < uVar1) {
        return 0;
    }
    if (param_3 == 0) {
        if (param_4 == 0) {
            iVar2 = DAT_00999bf0[uVar1 * 0x8e];
        }
        else {
            iVar2 = DAT_00999c00[uVar1 * 0x8e];
        }
    }
    else {
        iVar2 = DAT_00999bf8[uVar1 * 0x8e];
    }
    iVar2 = param_2 - iVar2;
    if (-0x41 < iVar2 && (iVar2 < 1 || 0x3f < iVar2)) {
        return 1;
    }
    if (param_3 != 0) {
        DAT_00999bf8[uVar1 * 0x8e] = param_2;
        return 0;
    }
    if (param_4 != 0) {
        DAT_00999c00[uVar1 * 0x8e] = param_2;
        return 0;
    }
    DAT_00999bf0[uVar1 * 0x8e] = param_2;
    return 0;
}
