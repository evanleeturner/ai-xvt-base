/* SPDX-License-Identifier: MIT */
/* z_bop 00403360 */
extern int DAT_009a1fe8;
extern int DAT_009a73c4;
extern int DAT_009a73c8;
extern int DAT_009a73cc;
extern int DAT_009a1ff8;

bool __cdecl FUN_00403360(int param_1, int param_2)
{
  int iVar1;
  int iVar2;
  int iVar3;
  
  iVar3 = param_1 * 0x23 + DAT_009a1fe8;
  iVar1 = DAT_009a73c4 - *(int *)(iVar3 + 4);
  iVar2 = DAT_009a73c8 - *(int *)(iVar3 + 8);
  iVar3 = DAT_009a73cc - *(int *)(iVar3 + 0xc);
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
    DAT_009a1ff8 = (iVar2 >> 1) + iVar1;
  }
  else {
    DAT_009a1ff8 = (iVar1 >> 1) + iVar2;
  }
  if (iVar3 < DAT_009a1ff8) {
    iVar3 = iVar3 >> 1;
  }
  else {
    DAT_009a1ff8 = DAT_009a1ff8 >> 1;
  }
  DAT_009a1ff8 = DAT_009a1ff8 + iVar3;
  return DAT_009a1ff8 < param_2;
}
