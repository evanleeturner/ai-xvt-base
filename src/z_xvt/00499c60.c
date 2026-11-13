/* SPDX-License-Identifier: MIT */
/* z_xvt 00499c60 */
extern int   DAT_00703c88;
extern short DAT_006d49a0;
extern int   DAT_00732a38;
extern int   DAT_0072f4e0;
extern int   DAT_006da530;
extern int   DAT_0071eae8;

int __stdcall FUN_00499c60(void)
{
    unsigned int acc;

    acc = (unsigned int)DAT_00703c88 * 0x24u
        + (unsigned int)(int)DAT_006d49a0 * 0x68cu;
    acc = acc + (unsigned int)DAT_00732a38 * 0xd3u;
    acc = acc + (unsigned int)DAT_0072f4e0 * 8u;
    acc = acc + (unsigned int)DAT_006da530 * 0x4cu;
    acc = acc + 0x924fu;
    acc = acc + (unsigned int)DAT_0071eae8 * 0x436u;
    return (int)acc;
}
