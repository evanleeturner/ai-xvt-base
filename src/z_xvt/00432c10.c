/* SPDX-License-Identifier: MIT */
/* z_xvt 00432c10 */
extern int DAT_00733120;
extern int DAT_00733260;

int __cdecl FUN_00432c10(int param_1)
{
    int iVar1;
    int *piVar2;

    iVar1 = 0;
    piVar2 = &DAT_00733120;
    do {
        if (*piVar2 == param_1) {
            return iVar1;
        }
        piVar2 = piVar2 + 10;
        iVar1 = iVar1 + 1;
    } while (piVar2 < &DAT_00733260);
    return iVar1;
}
