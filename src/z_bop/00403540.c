/* SPDX-License-Identifier: MIT */
/* z_bop 00403540 */
extern int DAT_00a07cd4;
extern int DAT_00a07cd8;
extern int DAT_00a07cdc;
extern int DAT_009a1ff8;

void __cdecl FUN_00458e20(unsigned int param_1, unsigned int param_2);

void __cdecl FUN_00403540(unsigned int param_1, unsigned int param_2)
{
  int iVar1;
  int iVar2;
  int iVar3;
  
  FUN_00458e20(param_1, 0);
  iVar2 = DAT_00a07cdc;
  iVar1 = DAT_00a07cd8;
  iVar3 = DAT_00a07cd4;
  FUN_00458e20(param_2, 0);
  iVar2 = iVar2 - DAT_00a07cdc;
  iVar1 = iVar1 - DAT_00a07cd8;
  iVar3 = iVar3 - DAT_00a07cd4;
  if (iVar2 < 0) {
    iVar2 = -iVar2;
  }
  if (iVar1 < 0) {
    iVar1 = -iVar1;
  }
  if (iVar3 < 0) {
    iVar3 = -iVar3;
  }
  if (iVar1 < iVar2) {
    iVar2 = (iVar1 >> 1) + iVar2;
  }
  else {
    iVar2 = (iVar2 >> 1) + iVar1;
  }
  if (iVar3 < iVar2) {
    DAT_009a1ff8 = iVar2 + (iVar3 >> 1);
    return;
  }
  DAT_009a1ff8 = (iVar2 >> 1) + iVar3;
  return;
}
