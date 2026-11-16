/* SPDX-License-Identifier: MIT */
/* z_bop 004d6dd0 */
void __cdecl FUN_004cc6c0(HFILE param_1, void *param_2, int param_3);

void __cdecl FUN_004d6dd0(HFILE param_1, undefined4 *param_2, int param_3)
{
  int iVar1;
  undefined4 *puVar2;
  undefined1 local_4 = 0;
  undefined1 local_3 = 0;
  undefined1 local_2 = 0;
  
  puVar2 = param_2;
  for (iVar1 = 0x100; iVar1 != 0; iVar1 = iVar1 + -1) {
    *puVar2 = 0;
    puVar2 = puVar2 + 1;
  }
  if (0 < param_3) {
    do {
      FUN_004cc6c0(param_1, &local_4, 4);
      param_3 = param_3 + -1;
      *(undefined1 *)param_2 = local_2;
      *(undefined1 *)((int)param_2 + 1) = local_3;
      *(undefined1 *)((int)param_2 + 2) = local_4;
      *(undefined1 *)((int)param_2 + 3) = 0;
      param_2 = param_2 + 1;
    } while (param_3 != 0);
  }
  return;
}
