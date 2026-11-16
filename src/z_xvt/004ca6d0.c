/* SPDX-License-Identifier: MIT */
/* z_xvt 004ca6d0 */
extern unsigned int __cdecl FUN_004ca780(unsigned int param_1);
extern unsigned int **DAT_007806fa;
extern unsigned int DAT_00780716;
extern unsigned int DAT_0078071a;
extern unsigned int DAT_0078071e;
extern unsigned int DAT_00780722;

unsigned int __cdecl FUN_004ca6d0(unsigned int param_1, unsigned int *param_2)
{
  int iVar1;
  unsigned int *puVar2;
  unsigned int local_50[0x14];
  
  if (param_2 == NULL) {
    param_2 = (unsigned int *)FUN_004ca780(param_1);
    if (param_2 == NULL) {
      return 0;
    }
  }
  puVar2 = local_50;
  for (iVar1 = 0x14; iVar1 != 0; iVar1--) {
    *puVar2 = 0;
    puVar2++;
  }
  local_50[0] = 0x50;
  local_50[1] = 0x40;
  local_50[2] = *param_2;
  local_50[3] = param_2[1];
  local_50[4] = param_2[2];
  local_50[5] = param_2[3];
  iVar1 = (*(int (__stdcall **)(void *, unsigned int *, int))(*DAT_007806fa + 0x60))(DAT_007806fa, local_50, 1);
  if (iVar1 != 0) {
    return 0;
  }
  DAT_00780716 = *param_2;
  DAT_0078071a = param_2[1];
  DAT_0078071e = param_2[2];
  DAT_00780722 = param_2[3];
  return 1;
}
