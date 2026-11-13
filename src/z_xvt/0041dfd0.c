/* SPDX-License-Identifier: MIT */
/* z_xvt 0041dfd0 */
extern unsigned short DAT_0073b374;
extern unsigned char DAT_006d4a60[];
extern unsigned char DAT_006d4a61[];
extern unsigned char DAT_0070ad42[];
extern unsigned char DAT_005247e8[];
extern unsigned char DAT_0052687a[];
extern unsigned char DAT_006d4a02;
extern unsigned char DAT_0070ad51[];

extern unsigned long long __cdecl FUN_0041eea0(unsigned short);
extern void __cdecl FUN_004215c0(unsigned short);

unsigned int __cdecl FUN_0041dfd0(unsigned short param_1)
{
    unsigned int uVar3 = (unsigned int)DAT_0073b374;
    unsigned int iVar1 = uVar3 * 0x126;
    DAT_006d4a60[iVar1] = 1;
    unsigned int iVar2 = uVar3 * 0x566;
    unsigned char idx = DAT_0070ad42[iVar2];
    unsigned char idx2 = DAT_005247e8[idx];
    unsigned char val = DAT_0052687a[idx2 * 0x16];
    if ((val & 0x80) == 0) {
        DAT_006d4a61[iVar1] = DAT_0070ad51[iVar2];
        unsigned long long uVar4 = FUN_0041eea0(param_1);
        return ((unsigned int)(uVar4 >> 16) << 16) | 1;
    }
    if (DAT_006d4a02 == 2) {
        DAT_006d4a61[iVar1] = DAT_0070ad51[iVar2];
    } else {
        DAT_006d4a61[iVar1] = 0;
    }
    FUN_004215c0(param_1);
    return 1;
}
