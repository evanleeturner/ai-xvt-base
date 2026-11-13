/* SPDX-License-Identifier: MIT */
/* z_xvt 00429fc0 */
extern int DAT_006d4988;
extern int DAT_006d9d64;
extern int DAT_006d9d68;
extern int DAT_006d9d6c;
extern int DAT_006d499c;

int __cdecl FUN_00429fc0(int param_1, int param_2)
{
  int iVar1;
  int iVar2;
  int iVar3;
  
  iVar3 = param_1 * 0x23 + DAT_006d4988;
  iVar1 = DAT_006d9d64 - *(int *)(iVar3 + 4);
  iVar2 = DAT_006d9d68 - *(int *)(iVar3 + 8);
  iVar3 = DAT_006d9d6c - *(int *)(iVar3 + 0xc);
  if (iVar1 < 0) {
    iVar1 = -iVar1;
  }
  if (iVar2 < 0) {
    iVar2 = -iVar2;
  }
  if (iVar3 < 0) {
    iVar3 = -iVar3;
  }
  if (iVar2 < iVar1) {
    DAT_006d499c = (iVar2 >> 1) + iVar1;
  }
  else {
    DAT_006d499c = (iVar1 >> 1) + iVar2;
  }
  if (iVar3 < DAT_006d499c) {
    iVar3 = iVar3 >> 1;
  }
  else {
    DAT_006d499c = DAT_006d499c >> 1;
  }
  DAT_006d499c = DAT_006d499c + iVar3;
  return DAT_006d499c < param_2;
}
