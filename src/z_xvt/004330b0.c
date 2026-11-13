/* SPDX-License-Identifier: MIT */
/* z_xvt 004330b0 */
extern int DAT_00733124;
extern int DAT_00733264;

int __stdcall FUN_004330b0(void)
{
    int iVar1;
    int *piVar2;

    iVar1 = 0;
    piVar2 = &DAT_00733124;
    do {
        if (*piVar2 == 1) {
            iVar1 = iVar1 + 1;
        }
        piVar2 = piVar2 + 10;
    } while (piVar2 < &DAT_00733264);
    return iVar1;
}
