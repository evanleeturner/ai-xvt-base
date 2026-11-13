/* SPDX-License-Identifier: MIT */
/* z_bop 00451560 */
extern unsigned char DAT_009d77f8;
extern unsigned short DAT_009a6ff0;
extern unsigned char DAT_009d8b86;
extern unsigned int DAT_009ecc34;

void __stdcall FUN_00451560(void)
{
    DAT_009d77f8 = 0;
    DAT_009a6ff0 = 0xffff;
    *(unsigned short *)(&DAT_009d8b86 + DAT_009ecc34 * 0x10) = 0;
    return;
}
