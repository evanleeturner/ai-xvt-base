/* SPDX-License-Identifier: MIT */
/* z_bop 004d9ba0 */
extern int DAT_0052c004;
extern int DAT_00665470;

int __cdecl FUN_004d9ba0(int param_1)
{
  unsigned int uVar1;
  int *piVar2;
  
  if (0x1f < DAT_0052c004) {
    return 0;
  }
  uVar1 = 0;
  if (DAT_0052c004 != 0) {
    piVar2 = &DAT_00665470;
    do {
      if (*piVar2 == param_1) {
        return 1;
      }
      piVar2 = piVar2 + 1;
      uVar1 = uVar1 + 1;
    } while (uVar1 < (unsigned int)DAT_0052c004);
  }
  (&DAT_00665470)[DAT_0052c004] = param_1;
  DAT_0052c004 = DAT_0052c004 + 1;
  return 1;
}
