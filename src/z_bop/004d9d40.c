/* SPDX-License-Identifier: MIT */
/* z_bop 004d9d40 */
extern unsigned int DAT_006654f0;
extern unsigned int DAT_00665470;
extern unsigned int DAT_0052c004;
extern unsigned int DAT_0052c008;

unsigned int __stdcall FUN_004d9d40(void)
{
    unsigned int *puVar2 = &DAT_006654f0;
    unsigned int *puVar3 = &DAT_00665470;
    int iVar1;
    
    for (iVar1 = 0x20; iVar1 != 0; iVar1 = iVar1 - 1) {
        *puVar3 = *puVar2;
        puVar2 = puVar2 + 1;
        puVar3 = puVar3 + 1;
    }
    DAT_0052c004 = DAT_0052c008;
    return 1;
}
