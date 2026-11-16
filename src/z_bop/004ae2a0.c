/* SPDX-License-Identifier: MIT */
/* z_bop 004ae2a0 */
extern undefined4 __cdecl FUN_004ac720(undefined4 param_1);
extern void __cdecl FUN_00471ff0(int *param_1);
extern undefined4 * __cdecl FUN_004ae1a0(undefined4 param_1, undefined4 param_2);
extern void __cdecl FUN_004ac740(void);
extern short DAT_009a7ed0[];
extern char DAT_00521f19[];

undefined4 __cdecl FUN_004ae2a0(int param_1, int param_2)
{
  int *piVar1;
  undefined4 *puVar2;
  undefined4 uVar3;

  if (DAT_009a7ed0[param_1] == 0) {
    return 0;
  }
  if (param_2 < 0) {
    return 0;
  }
  if ((DAT_00521f19[param_1 * 0x1a] & 1) == 0) {
    return 0;
  }
  piVar1 = (int *)FUN_004ac720((undefined4)(((unsigned int)DAT_009a7ed0[param_1] << 16) | (param_1 & 0xFFFF)));
  if ((int *)*piVar1 != piVar1) {
    FUN_00471ff0(piVar1);
  }
  if (*(int *)(**(int **)((int)piVar1 + 10) + 4) == 0x14) {
    param_2 = param_2 + 1;
  }
  if (*(int *)((int)piVar1 + 6) <= param_2) {
    param_2 = *(int *)((int)piVar1 + 6) + -1;
  }
  uVar3 = 0;
  puVar2 = (undefined4 *)FUN_004ae1a0((*(int **)((int)piVar1 + 10))[param_2], (undefined4)piVar1);
  if (puVar2 != (undefined4 *)0x0) {
    uVar3 = *puVar2;
  }
  FUN_004ac740();
  return uVar3;
}
