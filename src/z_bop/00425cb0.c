/* SPDX-License-Identifier: MIT */
/* z_bop 00425cb0 */
extern unsigned short DAT_009d113c;
extern unsigned short DAT_00555c84;

void __stdcall FUN_00425cb0(void)
{
    unsigned short uVar1;
    short sVar2;
    
    sVar2 = 0x10;
    while (sVar2 != 0) {
        sVar2 = sVar2 - 1;
        if (sVar2 == 0) break;
        uVar1 = DAT_009d113c & 0x8000;
        DAT_009d113c = (unsigned short)(((DAT_009d113c >> 8 ^ (DAT_009d113c & 0xff) * 2) & 0x80) != 0) + (unsigned short)(DAT_009d113c * 2);
        DAT_00555c84 = (unsigned short)(uVar1 != 0) + (unsigned short)(DAT_00555c84 * 2);
    }
    return;
}
