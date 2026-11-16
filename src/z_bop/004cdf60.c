/* SPDX-License-Identifier: MIT */
/* z_bop 004cdf60 */
extern undefined4 DAT_00acebc5;
extern undefined4 DAT_00acebd1;
extern undefined4 DAT_00acebd5;
extern undefined4 DAT_00acebd9;
extern undefined4 DAT_00acebdd;
extern void LAB_004cdfd0;

uint __cdecl FUN_004cdf60(undefined4 param_1)
{
  undefined4 *puVar1;
  int iVar2;
  undefined4 local_50[0x14];

  puVar1 = local_50;
  for (iVar2 = 0x14; iVar2 != 0; iVar2 = iVar2 + -1) {
    *puVar1 = 0;
    puVar1 = puVar1 + 1;
  }
  local_50[0] = 0x50;
  local_50[6] = DAT_00acebd1;
  local_50[7] = DAT_00acebd5;
  local_50[8] = DAT_00acebd9;
  local_50[9] = DAT_00acebdd;
  iVar2 = ((int (__stdcall *)(undefined4, undefined4 *, uint, void *, undefined4, uint))(*(undefined4 **)((char *)DAT_00acebc5 + 0x34)))(DAT_00acebc5, local_50, 0, &LAB_004cdfd0, param_1, 1);
  return -(uint)(iVar2 == 0) & 0x665028;
}
