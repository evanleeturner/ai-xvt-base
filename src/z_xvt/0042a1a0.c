/* SPDX-License-Identifier: MIT */
/* z_xvt 0042a1a0 */
extern int DAT_00732a24;
extern int DAT_00732a28;
extern int DAT_00732a2c;
extern int DAT_006d499c;

void __cdecl FUN_00421c70(uint param_1, uint param_2);

void __cdecl FUN_0042a1a0(uint param_1, uint param_2)
{
  int iVar1;
  int iVar2;
  int iVar3;
  
  FUN_00421c70(param_1, 0);
  iVar2 = DAT_00732a2c;
  iVar1 = DAT_00732a28;
  iVar3 = DAT_00732a24;
  FUN_00421c70(param_2, 0);
  iVar2 = iVar2 - DAT_00732a2c;
  iVar1 = iVar1 - DAT_00732a28;
  iVar3 = iVar3 - DAT_00732a24;
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
    DAT_006d499c = iVar2 + (iVar3 >> 1);
    return;
  }
  DAT_006d499c = (iVar2 >> 1) + iVar3;
  return;
}
