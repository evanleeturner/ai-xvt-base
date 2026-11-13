/* SPDX-License-Identifier: MIT */
/* z_xvt 00498fc0 */
extern unsigned int FUN_00499c60(void);
extern unsigned int __cdecl FUN_0048f3c0(unsigned int, unsigned int);
extern void FUN_0047f010(int);
extern unsigned int FUN_0048f4f0(unsigned int);

extern short DAT_00645ad4;
extern unsigned int DAT_00645a80;
extern short DAT_00645a8c;
extern unsigned int DAT_00645778;

void __stdcall FUN_00498fc0(void)
{
    unsigned int uVar1;
    unsigned int uVar2;

    uVar1 = FUN_00499c60();
    uVar2 = FUN_0048f3c0(uVar1, 0);
    DAT_00645ad4 = (short)uVar2;
    if (DAT_00645ad4 == 0) {
        FUN_0047f010(0);
    }
    DAT_00645a80 = FUN_0048f4f0(uVar2);
    uVar1 = FUN_0048f3c0(uVar1, 0);
    DAT_00645a8c = (short)uVar1;
    if (DAT_00645a8c == 0) {
        FUN_0047f010(0);
    }
    DAT_00645778 = FUN_0048f4f0(uVar1);
    return;
}
