/* SPDX-License-Identifier: MIT */
/* z_xvt 00488b70 */
int __cdecl FUN_00488a90(char *param_1, int param_2, int param_3);

int __cdecl FUN_00488b70(char *param_1, int param_2, int param_3)
{
    int iVar1;
    int iVar2;
    int iVar3;
    
    iVar3 = 0;
    iVar2 = 0;
    if (0 < *(int *)(param_2 + 6)) {
        do {
            iVar1 = *(int *)(*(int *)(param_2 + 10) + iVar3);
            if (param_3 == iVar1) {
                return 0;
            }
            iVar1 = FUN_00488a90(param_1, iVar1, param_3);
            if (iVar1 == 0) {
                return 0;
            }
            iVar3 = iVar3 + 4;
            iVar2 = iVar2 + 1;
        } while (iVar2 < *(int *)(param_2 + 6));
    }
    return 0;
}
