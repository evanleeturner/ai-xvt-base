/* SPDX-License-Identifier: MIT */
/* z_bop 0042acd0 */
void __cdecl FUN_0042aba0(int, int, float *);

void __cdecl FUN_0042acd0(int param_1, float *param_2)
{
    int count;
    __builtin_memcpy(&count, (char *)param_1 + 6, 4);
    if (count > 0) {
        int iVar1 = 0;
        int iVar2 = 0;
        do {
            iVar2 += 4;
            iVar1++;
            int base_addr;
            __builtin_memcpy(&base_addr, (char *)param_1 + 10, 4);
            int elem;
            __builtin_memcpy(&elem, (char *)base_addr - 4 + iVar2, 4);
            FUN_0042aba0(elem, param_1, param_2);
        } while (iVar1 < count);
    }
}
