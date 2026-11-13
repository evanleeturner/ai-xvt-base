/* SPDX-License-Identifier: MIT */
/* z_xvt 00449190 */
int __cdecl FUN_00449190(int param_1);

int __cdecl FUN_00449190(int param_1)
{
    int iVar1;
    int iVar2;
    int iVar3;
    
    if (param_1 == 0) {
        return 0;
    }
    if (*(int *)(param_1 + 4) != 0x17) {
        iVar3 = 0;
        iVar2 = 0;
        if (0 < *(int *)(param_1 + 8)) {
            do {
                if ((*(int *)(iVar3 + *(int *)(param_1 + 0xc)) != 0) &&
                    (iVar1 = FUN_00449190(*(int *)(*(int *)(param_1 + 0xc) + iVar3)), iVar1 != 0)) {
                    return iVar1;
                }
                iVar3 = iVar3 + 4;
                iVar2 = iVar2 + 1;
            } while (iVar2 < *(int *)(param_1 + 8));
        }
        return 0;
    }
    return param_1;
}
