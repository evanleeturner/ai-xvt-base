/* SPDX-License-Identifier: MIT */
/* z_bop 00425d10 */
extern unsigned short DAT_009d77a8;
extern unsigned short DAT_00555c80;

void __stdcall FUN_00425d10(void)
{
    unsigned short uVar1;
    short sVar2;
    
    sVar2 = 0x10;
    do {
        uVar1 = DAT_009d77a8 & 0x8000;
        sVar2 = sVar2 - 1;
        DAT_009d77a8 = (unsigned short)(((DAT_009d77a8 >> 8 ^ (DAT_009d77a8 & 0xff) * 2) & 0x80) != 0) + DAT_009d77a8 * 2;
        DAT_00555c80 = (unsigned short)(uVar1 != 0) + DAT_00555c80 * 2;
    } while (sVar2 != 0);
    return;
}
