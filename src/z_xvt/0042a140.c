/* SPDX-License-Identifier: MIT */
/* z_xvt 0042a140 */
extern int DAT_00732a24;
extern int DAT_00732a28;
extern int DAT_00732a2c;

void __cdecl FUN_00421c70(uint param_1, uint param_2);
void __cdecl FUN_00418cd0(int param_1, int param_2, int param_3);

void __cdecl FUN_0042a140(uint param_1, uint param_2)
{
  int iVar1;
  int iVar2;
  int iVar3;
  
  FUN_00421c70(param_2, 0);
  iVar3 = DAT_00732a2c;
  iVar2 = DAT_00732a28;
  iVar1 = DAT_00732a24;
  FUN_00421c70(param_1, 0);
  FUN_00418cd0(iVar3 - DAT_00732a2c, iVar2 - DAT_00732a28, iVar1 - DAT_00732a24);
  return;
}
