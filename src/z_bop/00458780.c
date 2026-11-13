/* SPDX-License-Identifier: MIT */
/* z_bop 00458780 */
void __cdecl FUN_00458780(int param_1)
{
    int iVar1;
    int local_val;
    char *pBase;

    __builtin_memcpy(&local_val, (void *)(param_1 + 0x45e), 4);
    if (local_val != 0) {
        *(char *)(local_val + 3) = 0;
        local_val = 0;
        __builtin_memcpy((void *)(param_1 + 0x45e), &local_val, 4);
    }

    pBase = (char *)(param_1 + 0x41e);
    iVar1 = 0x10;
    do {
        __builtin_memcpy(&local_val, pBase, 4);
        if (local_val != 0) {
            *(char *)(local_val + 3) = 0;
            local_val = 0;
            __builtin_memcpy(pBase, &local_val, 4);
        }
        pBase += 4;
        iVar1--;
    } while (iVar1 != 0);
}
