/* SPDX-License-Identifier: MIT */
/* z_xvt 0043fdf0 */
extern unsigned short DAT_006559b0[];
extern short DAT_0052f0e0;

unsigned int __stdcall FUN_0043fdf0(void)
{
    unsigned short uVar1;
    unsigned int uVar2;

    uVar2 = 0;
    while (DAT_006559b0[uVar2] == 0 || DAT_0052f0e0 == (short)uVar2) {
        uVar1 = (unsigned short)((short)uVar2 + 1);
        uVar2 = (unsigned int)uVar1;
        if (7 < uVar1) {
            return 0xffff;
        }
    }
    return uVar2;
}
