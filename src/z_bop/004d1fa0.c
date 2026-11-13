/* SPDX-License-Identifier: MIT */
/* z_bop 004d1fa0 */
extern int DAT_00b69a89;
extern int DAT_00b63f99[];
extern int DAT_00b63fa1[];
extern int DAT_00b63fa9[];

int __cdecl FUN_004d23b0(int param_1);

int __cdecl FUN_004d1fa0(int param_1, int param_2, int param_3, int param_4)
{
    int iVar2;
    unsigned int uVar1;

    iVar2 = DAT_00b69a89;
    uVar1 = FUN_004d23b0(param_1);
    if ((DAT_00b69a89 != iVar2) || (0x27 < uVar1)) {
        return 0;
    }
    if (param_3 == 0) {
        if (param_4 == 0) {
            iVar2 = DAT_00b63f99[uVar1 * 0x8e];
        }
        else {
            iVar2 = DAT_00b63fa9[uVar1 * 0x8e];
        }
    }
    else {
        iVar2 = DAT_00b63fa1[uVar1 * 0x8e];
    }
    iVar2 = param_2 - iVar2;
    if ((-0x41 < iVar2) && ((iVar2 < 1) || (0x3f < iVar2))) {
        return 1;
    }
    if (param_3 != 0) {
        DAT_00b63fa1[uVar1 * 0x8e] = param_2;
        return 0;
    }
    if (param_4 != 0) {
        DAT_00b63fa9[uVar1 * 0x8e] = param_2;
        return 0;
    }
    DAT_00b63f99[uVar1 * 0x8e] = param_2;
    return 0;
}
