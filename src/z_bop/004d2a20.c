/* SPDX-License-Identifier: MIT */
/* z_bop 004d2a20 */
extern int DAT_00665050[];

int __cdecl FUN_004d2a20(int param_1, int param_2)
{
    int *piVar1;
    int iVar2;
    
    iVar2 = 0;
    piVar1 = &DAT_00665050[0];
    do {
        if (*piVar1 == param_1) {
            DAT_00665050[iVar2 * 6] = param_1;
            DAT_00665050[iVar2 * 6 + 2] = param_2;
            break;
        }
        piVar1 += 6;
        iVar2++;
    } while (piVar1 < (int *)0x665410);
    
    if (iVar2 == 0x28) {
        iVar2 = 0;
        piVar1 = &DAT_00665050[0];
        while (*piVar1 != 0) {
            piVar1 += 6;
            iVar2++;
            if (piVar1 >= (int *)0x665410) {
                return 1;
            }
        }
        DAT_00665050[iVar2 * 6] = param_1;
        DAT_00665050[iVar2 * 6 + 1] = 1;
        DAT_00665050[iVar2 * 6 + 2] = param_2;
        DAT_00665050[iVar2 * 6 + 3] = 0;
        DAT_00665050[iVar2 * 6 + 4] = 0;
    }
    return 1;
}
