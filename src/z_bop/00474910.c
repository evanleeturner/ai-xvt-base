/* SPDX-License-Identifier: MIT */
/* z_bop 00474910 */
int __cdecl FUN_00474830(char *param_1, int param_2, int param_3);

int __cdecl FUN_00474910(char *param_1, int param_2, int param_3)
{
    int iVar1;
    int iVar2;
    int iVar3;
    int count;
    int base;

    __builtin_memcpy(&count, (char *)param_2 + 6, 4);
    __builtin_memcpy(&base, (char *)param_2 + 10, 4);

    iVar2 = 0;
    iVar3 = 0;
    if (0 < count) {
        do {
            iVar1 = *(int *)(base + iVar3);
            if (param_3 == iVar1) {
                return 0;
            }
            iVar1 = FUN_00474830(param_1, iVar1, param_3);
            if (iVar1 == 0) {
                return iVar1;
            }
            iVar3 = iVar3 + 4;
            iVar2 = iVar2 + 1;
        } while (iVar2 < count);
    }
    return 0;
}
