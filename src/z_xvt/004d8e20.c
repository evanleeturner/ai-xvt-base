/* SPDX-License-Identifier: MIT */
/* z_xvt 004d8e20 */
extern int DAT_00532adc;
extern int DAT_0064f688;

int __cdecl FUN_004d8e20(int param_1)
{
  int iVar1;
  int *piVar2;
  
  if (0x1f < DAT_00532adc) {
    return 0;
  }
  iVar1 = 0;
  if (DAT_00532adc != 0) {
    piVar2 = &DAT_0064f688;
    do {
      if (*piVar2 == param_1) {
        return 1;
      }
      piVar2 = piVar2 + 1;
      iVar1 = iVar1 + 1;
    } while (iVar1 < DAT_00532adc);
  }
  (&DAT_0064f688)[DAT_00532adc] = param_1;
  DAT_00532adc = DAT_00532adc + 1;
  return 1;
}
