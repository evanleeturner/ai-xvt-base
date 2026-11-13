/* SPDX-License-Identifier: MIT */
/* z_bop 0042de40 */
extern unsigned int DAT_00a0adf0;

void __stdcall FUN_0042de40(void)
{
    int iVar1;
    unsigned int *puVar2;
    
    puVar2 = &DAT_00a0adf0;
    for (iVar1 = 0x13a4; iVar1 != 0; iVar1 = iVar1 - 1) {
        *puVar2 = 0;
        puVar2 = puVar2 + 1;
    }
    return;
}
