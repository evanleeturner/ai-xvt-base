/* SPDX-License-Identifier: MIT */
/* z_xvt 00478470 */
extern unsigned short DAT_00703abc;
extern unsigned short DAT_005f2640;

void __stdcall FUN_00478470(void)
{
    unsigned short uVar1;
    short sVar2;

    sVar2 = 0x10;
    do {
        uVar1 = DAT_00703abc & 0x8000;
        sVar2 = (short)(sVar2 + -1);
        DAT_00703abc = (unsigned short)((unsigned short)((((DAT_00703abc >> 8) ^ ((DAT_00703abc & 0xff) * 2)) & 0x80) != 0) +
                                        (unsigned short)(DAT_00703abc * 2));
        DAT_005f2640 = (unsigned short)((unsigned short)(uVar1 != 0) + (unsigned short)(DAT_005f2640 * 2));
    } while (sVar2 != 0);
    return;
}
