/* SPDX-License-Identifier: MIT */
/* z_xvt 00438a10 */
extern unsigned char DAT_0071f2d8[4];  /* only byte 3 (0x0071f2db) is read here; full type unresolved */
extern unsigned char DAT_0071f2dc;
extern unsigned char DAT_0071f2dd;

int __stdcall FUN_00438a10(void)
{
    return (int)((unsigned int)DAT_0071f2dd +
                 ((unsigned int)DAT_0071f2d8[3] * 0x3cU + (unsigned int)DAT_0071f2dc) * 0x3cU);
}
