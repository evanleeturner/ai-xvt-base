/* SPDX-License-Identifier: MIT */
/* z_bop 0045d5f0 */
extern unsigned short DAT_009a73c0;
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

extern unsigned short __cdecl FUN_0045d6e0(unsigned short, unsigned short, unsigned short, unsigned short, unsigned short);

void __cdecl FUN_0045d5f0(unsigned int param_1)
{
    unsigned int iVar2;
    unsigned int iVar1;
    unsigned short uVar3;

    iVar2 = (param_1 & 0xffff) * 0x52;
    iVar1 = iVar2 + (unsigned int)DAT_009a73c0 * 0x566;
    uVar3 = FUN_0045d6e0((unsigned short)DAT_009d8cde[iVar1],
                         (unsigned short)DAT_009d8cdf[iVar1],
                         (unsigned short)DAT_009d8ce2[iVar1],
                         (unsigned short)DAT_009d8ce0[iVar1],
                         (unsigned short)DAT_009d8ce1[iVar1]);
    if (uVar3 == 0xffff) {
        iVar1 = iVar2 + (unsigned int)DAT_009a73c0 * 0x566;
        FUN_0045d6e0((unsigned short)DAT_009d8cd8[iVar1],
                     (unsigned short)DAT_009d8cda[iVar1],
                     (unsigned short)DAT_009d8cdc[iVar1],
                     (unsigned short)DAT_009d8cd9[iVar1],
                     (unsigned short)DAT_009d8cdb[iVar1]);
    }
    return;
}
