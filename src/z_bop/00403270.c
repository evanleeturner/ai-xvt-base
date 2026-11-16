/* SPDX-License-Identifier: MIT */
/* z_bop 00403270 */
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

unsigned int __cdecl FUN_004039e0(unsigned short, unsigned short, unsigned short, unsigned short, unsigned short);

void __cdecl FUN_00403270(unsigned int param_1)
{
    unsigned int iVar1;
    unsigned int iVar2;
    unsigned int uVar3;
    
    iVar2 = (param_1 & 0xffff) * 0x52;
    iVar1 = iVar2 + (unsigned int)DAT_009a73c0 * 0x566;
    uVar3 = FUN_004039e0((unsigned short)DAT_009d8cde[iVar1],
                         (unsigned short)DAT_009d8cdf[iVar1],
                         (unsigned short)DAT_009d8ce2[iVar1],
                         (unsigned short)DAT_009d8ce0[iVar1],
                         (unsigned short)DAT_009d8ce1[iVar1]);
    if ((short)uVar3 == -1) {
        iVar1 = iVar2 + (unsigned int)DAT_009a73c0 * 0x566;
        FUN_004039e0((unsigned short)DAT_009d8cd8[iVar1],
                     (unsigned short)DAT_009d8cda[iVar1],
                     (unsigned short)DAT_009d8cdc[iVar1],
                     (unsigned short)DAT_009d8cd9[iVar1],
                     (unsigned short)DAT_009d8cdb[iVar1]);
    }
    return;
}
