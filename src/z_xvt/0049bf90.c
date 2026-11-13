/* SPDX-License-Identifier: MIT */
/* z_xvt 0049bf90 */
extern int DAT_00645778;

void __cdecl FUN_0049bf90(undefined4 *param_1, int param_2, uint param_3)
{
  uint uVar1;
  undefined4 *puVar2;
  
  puVar2 = (undefined4 *)(param_2 + DAT_00645778);
  for (uVar1 = param_3 >> 2; uVar1 != 0; uVar1 = uVar1 - 1) {
    *puVar2 = *param_1;
    param_1 = param_1 + 1;
    puVar2 = puVar2 + 1;
  }
  for (uVar1 = param_3 & 3; uVar1 != 0; uVar1 = uVar1 - 1) {
    *(undefined1 *)puVar2 = *(undefined1 *)param_1;
    param_1 = (undefined4 *)((int)param_1 + 1);
    puVar2 = (undefined4 *)((int)puVar2 + 1);
  }
  return;
}
