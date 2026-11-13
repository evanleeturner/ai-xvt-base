/* SPDX-License-Identifier: MIT */
/* z_xvt 0040cad0 */
extern unsigned short DAT_0071ed00;
extern unsigned short DAT_006d4a42;
extern uint DAT_00709ffc;

int __cdecl FUN_00413f20(uint param_1, uint param_2);
int __cdecl FUN_00478350(uint param_1, uint param_2);

int __cdecl FUN_0040cad0(uint param_1, uint param_2)
{
    int iVar1;
    uint uVar2;

    if ((int)param_1 < 0) {
        iVar1 = FUN_00413f20(0u - param_1, param_2);
        uVar2 = 0u - (uint)iVar1;
    }
    else {
        uVar2 = (uint)FUN_00413f20(param_1, param_2);
    }
    if (DAT_0071ed00 != 0) {
        if ((int)uVar2 < 0) {
            uVar2 = (uint)FUN_00478350(0u - uVar2, (uint)DAT_0071ed00);
            uVar2 = 0u - uVar2;
        }
        else {
            uVar2 = (uint)FUN_00478350(uVar2, (uint)DAT_0071ed00);
        }
    }
    return (int)(uVar2 + (uint)DAT_006d4a42 + DAT_00709ffc);
}
