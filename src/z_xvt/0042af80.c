/* SPDX-License-Identifier: MIT */
/* z_xvt 0042af80 */
extern unsigned int FUN_0049f010(unsigned int, unsigned short, unsigned short);
extern unsigned short DAT_006d9d60;
extern unsigned short DAT_006d9d62;
extern unsigned char DAT_0070ad9e[];
extern unsigned char DAT_0070ad9f[];
extern unsigned char DAT_0070ada0[];
extern unsigned char DAT_0070ada1[];
extern unsigned char DAT_0070ada2[];
extern unsigned char DAT_0070ad98[];
extern unsigned char DAT_0070ad9a[];
extern unsigned char DAT_0070ad99[];
extern unsigned char DAT_0070ad9b[];
extern unsigned char DAT_0070ad9c[];
extern unsigned int unaff_EDI;

unsigned int __cdecl FUN_0042af80(unsigned short param_1)
{
    unsigned int uVar6;
    unsigned int ret;
    unsigned short uVar5;
    unsigned short uVar4;
    unsigned short uVar3;
    unsigned int idx;

    uVar6 = ((unsigned int)(short)(unaff_EDI >> 16) << 16) | (unsigned int)param_1;
    idx = (unsigned int)DAT_006d9d62 * 0x52 + (unsigned int)DAT_006d9d60 * 0x566;

    ret = FUN_0049f010(uVar6, (unsigned short)DAT_0070ad9e[idx], (unsigned short)DAT_0070ad9f[idx]);
    uVar5 = (unsigned short)ret;

    ret = FUN_0049f010(uVar6, (unsigned short)DAT_0070ada0[idx], (unsigned short)DAT_0070ada1[idx]);
    uVar4 = (unsigned short)ret;

    if (DAT_0070ada2[idx] == 1) {
        uVar5 = uVar5 | uVar4;
    }
    else {
        uVar5 = uVar5 & uVar4;
    }

    ret = FUN_0049f010(uVar6, (unsigned short)DAT_0070ad98[idx], (unsigned short)DAT_0070ad9a[idx]);
    uVar4 = (unsigned short)ret;

    ret = FUN_0049f010(uVar6, (unsigned short)DAT_0070ad99[idx], (unsigned short)DAT_0070ad9b[idx]);
    uVar3 = (unsigned short)ret;

    if (DAT_0070ad9c[idx] == 1) {
        uVar4 = uVar4 | uVar3;
    }
    else {
        uVar4 = uVar4 & uVar3;
    }

    uVar3 = (unsigned short)(ret >> 16);

    if ((uVar5 == 0) && (uVar4 == 0)) {
        return (unsigned int)uVar3 << 16;
    }
    return ((unsigned int)uVar3 << 16) | 1;
}
