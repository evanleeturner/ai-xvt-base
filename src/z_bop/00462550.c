/* SPDX-License-Identifier: MIT */
/* z_bop 00462550 */
extern unsigned int DAT_009a73b6;
extern unsigned short DAT_009a73c0;
extern unsigned short DAT_009a73c2;
extern unsigned char DAT_009a1fdc;
extern unsigned char DAT_009d8cde[];
extern unsigned char DAT_009d8cdf[];
extern unsigned char DAT_009d8ce2[];
extern unsigned char DAT_009d8ce0[];
extern unsigned char DAT_009d8ce1[];
extern unsigned char DAT_009d8cd8[];
extern unsigned char DAT_009d8cda[];
extern unsigned char DAT_009d8cdc[];
extern unsigned char DAT_009d8cd9[];
extern unsigned char DAT_009d8cdb[];

extern unsigned short __cdecl FUN_00462690(unsigned short, unsigned short, unsigned short, unsigned short, unsigned short);

unsigned int __stdcall FUN_00462550(void)
{
    unsigned int iVar1;
    unsigned int uVar2;
    unsigned short uVar3;

    *(unsigned char *)(DAT_009a73b6 + 0x2d) = 0xff;
    iVar1 = (unsigned int)DAT_009a73c2 * 0x52 + (unsigned int)DAT_009a73c0 * 0x566;
    uVar3 = FUN_00462690((unsigned short)DAT_009d8cde[iVar1],
                         (unsigned short)DAT_009d8cdf[iVar1],
                         (unsigned short)DAT_009d8ce2[iVar1],
                         (unsigned short)DAT_009d8ce0[iVar1],
                         (unsigned short)DAT_009d8ce1[iVar1]);
    uVar2 = DAT_009a73b6;
    if (uVar3 != 0xffff) {
        *(unsigned char *)(DAT_009a73b6 + 0x2d) = DAT_009a1fdc;
        return uVar2 & 0xffff0000;
    }
    iVar1 = (unsigned int)DAT_009a73c2 * 0x52 + (unsigned int)DAT_009a73c0 * 0x566;
    uVar3 = FUN_00462690((unsigned short)DAT_009d8cd8[iVar1],
                         (unsigned short)DAT_009d8cda[iVar1],
                         (unsigned short)DAT_009d8cdc[iVar1],
                         (unsigned short)DAT_009d8cd9[iVar1],
                         (unsigned short)DAT_009d8cdb[iVar1]);
    uVar2 = DAT_009a73b6;
    if (uVar3 != 0xffff) {
        *(unsigned char *)(DAT_009a73b6 + 0x2d) = DAT_009a1fdc;
        return uVar2 & 0xffff0000;
    }
    return 0;
}
