/* SPDX-License-Identifier: MIT */
/* z_bop 00404620 */
unsigned int __cdecl FUN_00404620(int param_1)
{
    short *psVar1;
    unsigned short *puVar2;

    psVar1 = *(short **)(param_1 + 0x45e);
    if (psVar1 == NULL) {
        return *(unsigned short *)(param_1 + 9);
    }
    if (*(short *)(param_1 + 0x3b0) != *psVar1) {
        *(unsigned int *)(param_1 + 0x45e) = 0;
        return ((unsigned int)psVar1 & 0xFFFF0000) | *(unsigned short *)(param_1 + 9);
    }
    if (*(int *)((int)psVar1 + 0x1f) == 0) {
        return *(unsigned short *)(param_1 + 9);
    }
    puVar2 = *(unsigned short **)(*(int *)((int)psVar1 + 0x1f) + 0xad);
    if (puVar2 != NULL) {
        return ((unsigned int)puVar2 & 0xFFFF0000) | *puVar2;
    }
    return *(unsigned short *)(param_1 + 9);
}
