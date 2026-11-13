/* SPDX-License-Identifier: MIT */
/* z_bop 0046fa30 */
extern int DAT_00999574;
extern int DAT_009996b4;

int __stdcall FUN_0046fa30(void)
{
    int iVar1 = 0;
    int *piVar2 = &DAT_00999574;

    do {
        if (*piVar2 == 1) {
            iVar1 = iVar1 + 1;
        }
        piVar2 = piVar2 + 10;
    } while (piVar2 < &DAT_009996b4);

    return iVar1;
}
