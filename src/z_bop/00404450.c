/* SPDX-License-Identifier: MIT */
/* z_bop 00404450 */
extern unsigned short DAT_009a73c0;
extern unsigned short DAT_009a73c2;
extern unsigned char DAT_009d8cde[];
extern unsigned char DAT_009d8cdf[];
extern unsigned char DAT_009d8ce0[];
extern unsigned char DAT_009d8ce1[];
extern unsigned char DAT_009d8ce2[];
extern unsigned char DAT_009d8cd8[];
extern unsigned char DAT_009d8cda[];
extern unsigned char DAT_009d8cd9[];
extern unsigned char DAT_009d8cdb[];
extern unsigned char DAT_009d8cdc[];
extern unsigned int unaff_EDI;
extern unsigned int FUN_00433390(unsigned int, unsigned short, unsigned short);

unsigned int __cdecl FUN_00404450(unsigned short param_1)
{
  unsigned int idx;
  unsigned int uVar6;
  unsigned int ret;
  unsigned short uVar3;
  unsigned short uVar4;
  unsigned short uVar5;
  
  uVar6 = (unaff_EDI & 0xFFFF0000) | (unsigned int)param_1;
  idx = (unsigned int)DAT_009a73c2 * 0x52 + (unsigned int)DAT_009a73c0 * 0x566;
  
  ret = FUN_00433390(uVar6, (unsigned short)DAT_009d8cde[idx], (unsigned short)DAT_009d8cdf[idx]);
  uVar5 = (unsigned short)ret;
  
  ret = FUN_00433390(uVar6, (unsigned short)DAT_009d8ce0[idx], (unsigned short)DAT_009d8ce1[idx]);
  uVar4 = (unsigned short)ret;
  
  if (DAT_009d8ce2[idx] == 1) {
    uVar5 |= uVar4;
  } else {
    uVar5 &= uVar4;
  }
  
  ret = FUN_00433390(uVar6, (unsigned short)DAT_009d8cd8[idx], (unsigned short)(DAT_009d8cda[idx] & 0xFF));
  uVar4 = (unsigned short)ret;
  
  ret = FUN_00433390(uVar6, (unsigned short)DAT_009d8cd9[idx], (unsigned short)DAT_009d8cdb[idx]);
  uVar3 = (unsigned short)(ret >> 16);
  
  if (DAT_009d8cdc[idx] == 1) {
    uVar4 |= uVar3;
  } else {
    uVar4 &= uVar3;
  }
  
  if (uVar5 == 0 && uVar4 == 0) {
    return (unsigned int)uVar3 << 16;
  }
  return ((unsigned int)uVar3 << 16) | 1;
}
