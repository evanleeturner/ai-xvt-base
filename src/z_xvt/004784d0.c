/* SPDX-License-Identifier: MIT */
/* z_xvt 004784d0 */
extern unsigned short DAT_0070a128;
extern unsigned short DAT_005f263c;

void __stdcall FUN_004784d0(void)
{
    unsigned short uVar1;
    short sVar2;

    sVar2 = 0x10;
    do {
        uVar1 = DAT_0070a128 & 0x8000;
        sVar2 = (short)(sVar2 + -1);
        DAT_0070a128 = (unsigned short)((unsigned short)((((DAT_0070a128 >> 8) ^ ((DAT_0070a128 & 0xff) * 2)) & 0x80) != 0) + (unsigned short)(DAT_0070a128 * 2));
        DAT_005f263c = (unsigned short)((unsigned short)(uVar1 != 0) + (unsigned short)(DAT_005f263c * 2));
    } while (sVar2 != 0);
    return;
}
