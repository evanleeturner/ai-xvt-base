/* SPDX-License-Identifier: MIT */
/* z_xvt 004310c0 */
extern int DAT_007330ac;
extern int DAT_00733120;
void __cdecl FUN_00431110(int param_1, unsigned int *param_2, unsigned int param_3);

void __cdecl FUN_004310c0(unsigned int *param_1, unsigned int param_2)
{
    int *piVar1;
    int iVar2;
    
    iVar2 = 0;
    if (0 < DAT_007330ac) {
        piVar1 = &DAT_00733120;
        do {
            if ((*piVar1 != 0) && (piVar1[1] != 0)) {
                FUN_00431110(*piVar1, param_1, param_2);
            }
            piVar1 = piVar1 + 10;
            iVar2 = iVar2 + 1;
        } while (iVar2 < DAT_007330ac);
    }
    return;
}
