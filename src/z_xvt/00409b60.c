/* SPDX-License-Identifier: MIT */
/* z_xvt 00409b60 */
extern unsigned int DAT_0073ba30[];

void __stdcall FUN_00409b60(void)
{
    unsigned int *puVar2;
    int iVar1;
    
    puVar2 = DAT_0073ba30;
    for (iVar1 = 0x1392; iVar1 != 0; iVar1 = iVar1 - 1) {
        *puVar2 = 0;
        puVar2 = puVar2 + 1;
    }
    return;
}
