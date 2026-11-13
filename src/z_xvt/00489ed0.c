/* SPDX-License-Identifier: MIT */
/* z_xvt 00489ed0 */
extern int DAT_006026b4;

int __cdecl FUN_00489ed0(int param_1, int param_2, int param_3)
{
  float *pfVar1;
  float *pfVar2;
  
  if (param_3 < 0) {
    return -1;
  }
  pfVar1 = (float *)(param_2 + param_3 * 8);
  DAT_006026b4 = DAT_006026b4 - (param_3 >> 1);
  if ((DAT_006026b4 < 0) || (*(int *)(param_1 + 0x10) < DAT_006026b4)) {
    DAT_006026b4 = 0;
  }
  pfVar2 = (float *)(*(int *)(param_1 + 0x14) + DAT_006026b4 * 8);
  if (DAT_006026b4 < *(int *)(param_1 + 0x10)) {
    do {
      if ((*pfVar2 == *pfVar1) && (pfVar2[1] == pfVar1[1])) {
        return DAT_006026b4;
      }
      pfVar2 = pfVar2 + 2;
      DAT_006026b4 = DAT_006026b4 + 1;
    } while (DAT_006026b4 < *(int *)(param_1 + 0x10));
  }
  DAT_006026b4 = 0;
  pfVar2 = *(float **)(param_1 + 0x14);
  if (0 < *(int *)(param_1 + 0x10)) {
    do {
      if ((*pfVar2 == *pfVar1) && (pfVar2[1] == pfVar1[1])) {
        return DAT_006026b4;
      }
      pfVar2 = pfVar2 + 2;
      DAT_006026b4 = DAT_006026b4 + 1;
    } while (DAT_006026b4 < *(int *)(param_1 + 0x10));
  }
  return 0;
}
