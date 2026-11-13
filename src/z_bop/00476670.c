/* SPDX-License-Identifier: MIT */
/* z_bop 00476670 */
int __cdecl FUN_004765b0(int, int, int);

int __cdecl FUN_00476670(int param_1, int param_2, int param_3)
{
    int iVar1;
    int iVar2;
    int iVar3;
    int v6;
    int v10_2;
    int v10_1;

    __builtin_memcpy(&v6, (void *)(param_2 + 6), 4);
    __builtin_memcpy(&v10_2, (void *)(param_2 + 10), 4);
    __builtin_memcpy(&v10_1, (void *)(param_1 + 10), 4);

    iVar2 = 0;
    iVar3 = 0;
    if (v6 > 0) {
        do {
            iVar1 = FUN_004765b0(*(int *)(v10_2 + iVar2),
                                 *(int *)(v10_1 + iVar2),
                                 param_3);
            if (iVar1 != 0) {
                return iVar1;
            }
            iVar2 += 4;
            iVar3 += 1;
        } while (iVar3 < v6);
    }
    return 0;
}
