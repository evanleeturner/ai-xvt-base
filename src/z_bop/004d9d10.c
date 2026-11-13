/* SPDX-License-Identifier: MIT */
/* z_bop 004d9d10 */
extern undefined4 DAT_00665470;
extern undefined4 DAT_006654f0;
extern undefined4 DAT_0052c008;
extern undefined4 DAT_0052c004;

undefined4 __stdcall FUN_004d9d10(void)
{
    undefined4 *puVar2 = &DAT_00665470;
    undefined4 *puVar3 = &DAT_006654f0;
    int iVar1;
    
    for (iVar1 = 0x20; iVar1 != 0; iVar1--) {
        *puVar3 = *puVar2;
        puVar2 = puVar2 + 1;
        puVar3 = puVar3 + 1;
    }
    DAT_0052c008 = DAT_0052c004;
    return 1;
}
