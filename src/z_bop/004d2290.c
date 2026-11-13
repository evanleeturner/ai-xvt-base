/* SPDX-License-Identifier: MIT */
/* z_bop 004d2290 */
extern int DAT_00665050;
extern int DAT_00665054;
extern int DAT_00665064;

undefined4 __cdecl FUN_004d2290(int param_1, undefined4 param_2)
{
    int *piVar1;
    int iVar2;
    
    iVar2 = 0;
    piVar1 = &DAT_00665050;
    while (param_1 != *piVar1 && *piVar1 != 0) {
        piVar1 += 6;
        iVar2 += 1;
        if (piVar1 >= (int *)0x665410) {
            return 1;
        }
    }
    (&DAT_00665050)[iVar2 * 6] = param_1;
    (&DAT_00665054)[iVar2 * 6] = param_2;
    (&DAT_00665064)[iVar2 * 6] = 1;
    return 1;
}
