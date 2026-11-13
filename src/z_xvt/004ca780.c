/* SPDX-License-Identifier: MIT */
/* z_xvt 004ca780 */
extern unsigned int *DAT_007806fa;
extern unsigned int DAT_00780706;
extern unsigned int DAT_0078070a;
extern unsigned int DAT_0078070e;
extern unsigned int DAT_00780712;
extern void LAB_004ca7f0;

unsigned int __cdecl FUN_004ca780(unsigned int param_1)
{
  int iVar1;
  unsigned int *puVar2;
  unsigned int local_50[0x14];
  
  puVar2 = local_50;
  for (iVar1 = 0x14; iVar1 != 0; iVar1 = iVar1 - 1) {
    *puVar2 = 0;
    puVar2 = puVar2 + 1;
  }
  local_50[0] = 0x50;
  local_50[6] = DAT_00780706;
  local_50[7] = DAT_0078070a;
  local_50[8] = DAT_0078070e;
  local_50[9] = DAT_00780712;
  iVar1 = (**(int (__stdcall **)(unsigned int *, unsigned int *, unsigned int, void *, unsigned int, unsigned int))(*DAT_007806fa + 0x34))(DAT_007806fa, local_50, 0, &LAB_004ca7f0, param_1, 1);
  return -(unsigned int)(iVar1 == 0) & 0x64f048;
}
