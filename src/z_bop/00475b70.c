/* SPDX-License-Identifier: MIT */
/* z_bop 00475b70 */
extern int DAT_0060f1e4;

int __cdecl FUN_00475b70(int param_1, int param_2, int param_3)
{
  float *pfVar1;
  float *pfVar2;
  
  if (param_3 < 0) {
    return -1;
  }
  pfVar1 = (float *)(param_2 + param_3 * 0xc);
  DAT_0060f1e4 = DAT_0060f1e4 - (param_3 >> 1);
  if ((DAT_0060f1e4 < 0) || (*(int *)(param_1 + 0x10) < DAT_0060f1e4)) {
    DAT_0060f1e4 = 0;
  }
  pfVar2 = (float *)(*(int *)(param_1 + 0x14) + DAT_0060f1e4 * 0xc);
  if (DAT_0060f1e4 < *(int *)(param_1 + 0x10)) {
    do {
      if (((*pfVar2 == *pfVar1) && (pfVar2[1] == pfVar1[1])) && (pfVar1[2] == pfVar2[2])) {
        return DAT_0060f1e4;
      }
      pfVar2 = pfVar2 + 3;
      DAT_0060f1e4 = DAT_0060f1e4 + 1;
    } while (DAT_0060f1e4 < *(int *)(param_1 + 0x10));
  }
  DAT_0060f1e4 = 0;
  pfVar2 = *(float **)(param_1 + 0x14);
  if (0 < *(int *)(param_1 + 0x10)) {
    do {
      if (((*pfVar2 == *pfVar1) && (pfVar2[1] == pfVar1[1])) && (pfVar1[2] == pfVar2[2])) {
        return DAT_0060f1e4;
      }
      pfVar2 = pfVar2 + 3;
      DAT_0060f1e4 = DAT_0060f1e4 + 1;
    } while (DAT_0060f1e4 < *(int *)(param_1 + 0x10));
  }
  return 0;
}
