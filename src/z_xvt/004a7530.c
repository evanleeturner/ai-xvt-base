/* SPDX-License-Identifier: MIT */
/* z_xvt 004a7530 */
extern int DAT_0052b050;
extern int DAT_00645be8;
extern int DAT_00646868;

unsigned int __stdcall FUN_004a7530(void)
{
    int *piVar1;

    if (DAT_0052b050 == 0) {
        return 0;
    }
    piVar1 = &DAT_00645be8;
    do {
        if (*piVar1 != 0) {
            return 1;
        }
        piVar1 = piVar1 + 8;
    } while (piVar1 < &DAT_00646868);
    return 0;
}
