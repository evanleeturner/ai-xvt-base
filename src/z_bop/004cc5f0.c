/* SPDX-License-Identifier: MIT */
/* z_bop 004cc5f0 */
undefined4 __cdecl FUN_004cc5e0(HFILE param_1);
undefined __cdecl FUN_004cc5c0(HFILE param_1, long param_2, short param_3);

undefined4 __cdecl FUN_004cc5f0(HFILE param_1)
{
  long lVar1;
  undefined4 uVar2;
  
  lVar1 = FUN_004cc5e0(param_1);
  FUN_004cc5c0(param_1, 0, 2);
  uVar2 = FUN_004cc5e0(param_1);
  FUN_004cc5c0(param_1, lVar1, 0);
  return uVar2;
}
