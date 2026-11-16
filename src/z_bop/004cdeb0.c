/* SPDX-License-Identifier: MIT */
/* z_bop 004cdeb0 */
extern unsigned int *DAT_00acebc5;
extern unsigned int DAT_00acebe1;
extern unsigned int DAT_00acebe5;
extern unsigned int DAT_00acebe9;
extern unsigned int DAT_00acebed;

unsigned int __cdecl FUN_004cdf60(unsigned int param_1);

unsigned int __cdecl FUN_004cdeb0(unsigned int param_1, unsigned int *param_2)
{
  int iVar1;
  unsigned int *puVar2;
  unsigned int local_50[4];
  
  if (param_2 == (unsigned int *)0x0) {
    param_2 = (unsigned int *)FUN_004cdf60(param_1);
    if (param_2 == (unsigned int *)0x0) {
      return 0;
    }
  }
  puVar2 = local_50;
  for (iVar1 = 0x14; iVar1 != 0; iVar1 = iVar1 + -1) {
    *puVar2 = 0;
    puVar2 = puVar2 + 1;
  }
  local_50[0] = 0x50;
  local_50[1] = 0x40;
  local_50[2] = *param_2;
  local_50[3] = param_2[1];
  iVar1 = (*(int (__stdcall **)(unsigned int *, unsigned int *, unsigned int))(*DAT_00acebc5 + 0x60))(DAT_00acebc5, local_50, 1);
  if (iVar1 != 0) {
    return 0;
  }
  DAT_00acebe1 = *param_2;
  DAT_00acebe5 = param_2[1];
  DAT_00acebe9 = param_2[2];
  DAT_00acebed = param_2[3];
  return 1;
}
