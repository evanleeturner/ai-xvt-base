/* SPDX-License-Identifier: MIT */
/* z_xvt 00499040 */
extern unsigned short DAT_00645ad4;
extern unsigned int DAT_00645a80;
extern unsigned short DAT_00645a8c;
extern unsigned int DAT_00645778;

extern void __cdecl FUN_0048f4b0(unsigned int param_1);

void __stdcall FUN_00499040(void)
{
    FUN_0048f4b0((unsigned int)DAT_00645ad4);
    DAT_00645ad4 = 0;
    DAT_00645a80 = 0;
    FUN_0048f4b0((unsigned int)DAT_00645a8c);
    DAT_00645a8c = 0;
    DAT_00645778 = 0;
    return;
}
