/* SPDX-License-Identifier: MIT */
/* z_bop 004d2ab0 */
extern int DAT_00665050[];
extern int DAT_00665410[];

undefined4 __cdecl FUN_004d2ab0(int param_1, undefined4 param_2)
{
  int *piVar1;
  int iVar2;
  
  piVar1 = DAT_00665050;
  iVar2 = 0;
  do {
    if (*piVar1 == param_1) {
      DAT_00665050[iVar2 * 6] = param_1;
      DAT_00665050[iVar2 * 6 + 3] = param_2;
      break;
    }
    piVar1 = piVar1 + 6;
    iVar2 = iVar2 + 1;
  } while (piVar1 < DAT_00665410);
  
  if (iVar2 == 0x28) {
    iVar2 = 0;
    piVar1 = DAT_00665050;
    while (*piVar1 != 0) {
      piVar1 = piVar1 + 6;
      iVar2 = iVar2 + 1;
      if (piVar1 > (int *)0x66540f) {
        return 1;
      }
    }
    DAT_00665050[iVar2 * 6] = param_1;
    DAT_00665050[iVar2 * 6 + 1] = 1;
    DAT_00665050[iVar2 * 6 + 2] = 0;
    DAT_00665050[iVar2 * 6 + 3] = param_2;
    DAT_00665050[iVar2 * 6 + 4] = 0;
  }
  return 1;
}
