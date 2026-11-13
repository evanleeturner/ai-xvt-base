/* SPDX-License-Identifier: MIT */
/* z_bop 00421ae0 */
extern unsigned short DAT_009a8d4a;
extern unsigned short DAT_009a8d4e;
extern short DAT_009a7b48;
extern short DAT_009a7b40;

void __cdecl FUN_00421ae0(int param_1, unsigned short *param_2)
{
  unsigned short uVar1;
  unsigned short uVar2;
  int iVar3;
  int iVar4;
  unsigned int uVar5;
  unsigned int uVar6;
  
  uVar2 = DAT_009a8d4e;
  uVar1 = DAT_009a8d4a;
  if ((short)DAT_009a8d4a < 0) {
    DAT_009a8d4a = -DAT_009a8d4a;
  }
  if ((short)DAT_009a8d4e < 0) {
    DAT_009a8d4e = -DAT_009a8d4e;
  }
  uVar6 = (int)((unsigned int)*param_2 * (int)(short)DAT_009a8d4a + 0x80) >> 8 & 0xffff;
  iVar3 = *(unsigned short *)(param_1 + 6) * uVar6;
  if (((*(unsigned short *)(param_1 + 8) ^ uVar1) & 0x8000) != 0) {
    iVar3 = -iVar3;
  }
  uVar5 = (int)(((int)((unsigned int)param_2[0x205] * (int)(short)DAT_009a8d4e + 0x80) >> 8) *
                (unsigned int)*param_2 + 0x80) >> 8 & 0xffff;
  iVar4 = *(unsigned short *)(param_1 + 2) * uVar5;
  if (((*(unsigned short *)(param_1 + 4) ^ uVar2) & 0x8000) != 0) {
    iVar4 = -iVar4;
  }
  DAT_009a7b48 = (short)((unsigned int)(iVar3 + 0x8000 + iVar4) >> 0x10);
  iVar3 = *(unsigned short *)(param_1 + 2) * uVar6;
  if (((*(unsigned short *)(param_1 + 4) ^ uVar1) & 0x8000) != 0) {
    iVar3 = -iVar3;
  }
  iVar4 = *(unsigned short *)(param_1 + 6) * uVar5;
  if (((*(unsigned short *)(param_1 + 8) ^ uVar2) & 0x8000) == 0) {
    iVar4 = -iVar4;
  }
  iVar4 = iVar3 + 0x8000 + iVar4;
  DAT_009a7b40 = (short)((unsigned int)iVar4 >> 0x10);
  if (iVar4 < 0) {
    DAT_009a7b40 = -DAT_009a7b40;
  }
  DAT_009a7b40 = (short)(((unsigned int)param_2[0x204] * (int)DAT_009a7b40 + 0x80) >> 8);
  if (iVar4 < 0) {
    DAT_009a7b40 = -DAT_009a7b40;
  }
  return;
}
