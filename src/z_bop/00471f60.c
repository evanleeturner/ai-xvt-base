/* SPDX-License-Identifier: MIT */
/* z_bop 00471f60 */
void __cdecl FUN_00471f60(int *param_1, int param_2);

void __cdecl FUN_00471f60(int *param_1, int param_2)
{
    int iVar1;
    int iVar2;
    int iVar3;
    
    if (*param_1 != 0) {
        *param_1 = *param_1 + param_2;
    }
    if (param_1[5] != 0) {
        iVar2 = param_2 + param_1[5];
        iVar3 = 0;
        param_1[5] = iVar2;
        if (0 < param_1[4]) {
            do {
                if (*(int *)(iVar2 + 8) != 0) {
                    *(int *)(iVar2 + 8) = *(int *)(iVar2 + 8) + param_2;
                }
                iVar2 = iVar2 + 0xc;
                iVar3 = iVar3 + 1;
            } while (iVar3 < param_1[4]);
        }
    }
    iVar2 = 0;
    if (param_1[3] != 0) {
        iVar3 = 0;
        param_1[3] = param_1[3] + param_2;
        if (0 < param_1[2]) {
            do {
                iVar1 = *(int *)(param_1[3] + iVar2);
                if (iVar1 != 0) {
                    *(int *)(param_1[3] + iVar2) = iVar1 + param_2;
                    FUN_00471f60((int *)iVar1, param_2);
                }
                iVar2 = iVar2 + 4;
                iVar3 = iVar3 + 1;
            } while (iVar3 < param_1[2]);
        }
    }
    return;
}
