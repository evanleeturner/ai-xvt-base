/* SPDX-License-Identifier: MIT */
/* z_xvt 00429c40 */
extern uint DAT_006d9d70;
extern ushort DAT_00524778;
extern int DAT_006d4988;
extern int DAT_006d9d64;
extern int DAT_006d9d68;
extern int DAT_006d9d6c;
extern int DAT_006d499c;

uint __cdecl FUN_00478320(uint param_1, ushort param_2);

bool __cdecl FUN_00429c40(uint param_1)
{
  uint uVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  
  uVar1 = FUN_00478320(0x500, *(ushort *)(&DAT_00524778 + (uint)DAT_006d9d70 * 2));
  iVar4 = (param_1 & 0xffff) * 0x23 + DAT_006d4988;
  iVar3 = DAT_006d9d64 - *(int *)(iVar4 + 4);
  iVar2 = DAT_006d9d68 - *(int *)(iVar4 + 8);
  iVar4 = DAT_006d9d6c - *(int *)(iVar4 + 0xc);
  if (iVar3 < 0) {
    iVar3 = -iVar3;
  }
  if (iVar2 < 0) {
    iVar2 = -iVar2;
  }
  if (iVar4 < 0) {
    iVar4 = -iVar4;
  }
  if (iVar2 < iVar3) {
    DAT_006d499c = (iVar2 >> 1) + iVar3;
  }
  else {
    DAT_006d499c = (iVar3 >> 1) + iVar2;
  }
  if (iVar4 < DAT_006d499c) {
    iVar4 = iVar4 >> 1;
  }
  else {
    DAT_006d499c = DAT_006d499c >> 1;
  }
  DAT_006d499c = DAT_006d499c + iVar4;
  return DAT_006d499c < (int)(((uVar1 & 0xffff) + 0xa00) * 0x100);
}
