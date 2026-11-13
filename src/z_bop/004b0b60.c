/* SPDX-License-Identifier: MIT */
/* z_bop 004b0b60 */
extern undefined4 DAT_00664770;

void __cdecl FUN_004b0b60(undefined4 *param_1, uint param_2)
{
  uint uVar1;
  undefined4 *puVar2;
  
  puVar2 = &DAT_00664770;
  for (uVar1 = (param_2 & 0x7fffffff) >> 1; uVar1 != 0; uVar1 = uVar1 - 1) {
    *puVar2 = *param_1;
    param_1 = param_1 + 1;
    puVar2 = puVar2 + 1;
  }
  for (uVar1 = param_2 * 2 & 3; uVar1 != 0; uVar1 = uVar1 - 1) {
    *(undefined1 *)puVar2 = *(undefined1 *)param_1;
    param_1 = (undefined4 *)((int)param_1 + 1);
    puVar2 = (undefined4 *)((int)puVar2 + 1);
  }
  return;
}
