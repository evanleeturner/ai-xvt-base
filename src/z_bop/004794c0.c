/* SPDX-License-Identifier: MIT */
/* z_bop 004794c0 */
extern int DAT_0060f1d4;
extern void FUN_00479510(float *param_1, int param_2, int param_3, int param_4);

float * __cdecl FUN_004794c0(int param_1, int param_2, int param_3)
{
  float *pfVar1;
  
  pfVar1 = (float *)(param_2 + *(int *)(param_1 + 0x10) * 0x40);
  FUN_00479510(pfVar1, *(int *)(param_1 + 0x14), *(int *)(param_1 + 0x10), param_3);
  return pfVar1 + *(int *)(param_1 + 0x10) * 9 + DAT_0060f1d4 * 3;
}
