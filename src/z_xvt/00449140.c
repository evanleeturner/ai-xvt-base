/* SPDX-License-Identifier: MIT */
/* z_xvt 00449140 */
int __cdecl FUN_00449140(int param_1);

int __cdecl FUN_00449140(int param_1)
{
    int iVar1;
    int iVar2;
    int iVar3;

    if (param_1 == 0) {
        return 0;
    }
    if (*(int *)(param_1 + 4) != 3) {
        iVar2 = 0;
        iVar3 = 0;
        if (0 < *(int *)(param_1 + 8)) {
            do {
                iVar1 = *(int *)(*(int *)(param_1 + 0xc) + iVar2);
                if (iVar1 != 0) {
                    iVar1 = FUN_00449140(iVar1);
                    if (iVar1 != 0) {
                        return iVar1;
                    }
                }
                iVar2 = iVar2 + 4;
                iVar3 = iVar3 + 1;
            } while (iVar3 < *(int *)(param_1 + 8));
        }
        return 0;
    }
    return param_1;
}
