/* SPDX-License-Identifier: MIT */
/* z_xvt 0044fd90 */
extern unsigned short DAT_006d9d60;
extern unsigned char DAT_0070ad98[];
extern unsigned char DAT_0070ad99[];
extern unsigned char DAT_0070ad9a[];
extern unsigned char DAT_0070ad9b[];
extern unsigned char DAT_0070ad9c[];
extern unsigned char DAT_0070ad9e[];
extern unsigned char DAT_0070ad9f[];
extern unsigned char DAT_0070ada0[];
extern unsigned char DAT_0070ada1[];
extern unsigned char DAT_0070ada2[];

unsigned int __cdecl FUN_0044fe90(unsigned short param_1, unsigned short param_2, unsigned short param_3, unsigned short param_4, unsigned short param_5);

int __cdecl FUN_0044fd90(unsigned int param_1)
{
    int iVar1;
    int iVar2;
    unsigned int uVar3;
    
    iVar2 = (param_1 & 0xffff) * 0x52;
    iVar1 = iVar2 + (unsigned int)DAT_006d9d60 * 0x566;
    uVar3 = FUN_0044fe90((unsigned short)DAT_0070ad9e[iVar1],
                         (unsigned short)DAT_0070ad9f[iVar1],
                         (unsigned short)DAT_0070ada2[iVar1],
                         (unsigned short)DAT_0070ada0[iVar1],
                         (unsigned short)DAT_0070ada1[(unsigned int)DAT_006d9d60 * 0x566 + iVar2]);
    if ((short)uVar3 != 0) {
        return 1;
    }
    iVar1 = iVar2 + (unsigned int)DAT_006d9d60 * 0x566;
    uVar3 = FUN_0044fe90((unsigned short)DAT_0070ad98[iVar1],
                         (unsigned short)DAT_0070ad9a[iVar1],
                         (unsigned short)DAT_0070ad9c[iVar1],
                         (unsigned short)DAT_0070ad99[iVar1],
                         (unsigned short)DAT_0070ad9b[(unsigned int)DAT_006d9d60 * 0x566 + iVar2]);
    return (int)(1 - ((short)uVar3 == 0));
}
