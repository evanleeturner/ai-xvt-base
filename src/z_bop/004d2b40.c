/* SPDX-License-Identifier: MIT */
/* z_bop 004d2b40 */
extern int DAT_00665050;
extern int DAT_00665054;
extern int DAT_00665058;
extern int DAT_0066505c;
extern int DAT_00665060;
extern int DAT_00665410;

int __cdecl FUN_004d2b40(int param_1, unsigned int param_2)
{
  int *piVar1;
  int iVar2;
  
  piVar1 = &DAT_00665050;
  iVar2 = 0;
  do {
    if (*piVar1 == param_1) {
      (&DAT_00665050)[iVar2 * 6] = param_1;
      (&DAT_00665060)[iVar2 * 6] = param_2;
      break;
    }
    piVar1 = piVar1 + 6;
    iVar2 = iVar2 + 1;
  } while (piVar1 < &DAT_00665410);
  
  if (iVar2 != 0x28) {
    iVar2 = 0;
    piVar1 = &DAT_00665050;
    while (*piVar1 != 0) {
      piVar1 = piVar1 + 6;
      iVar2 = iVar2 + 1;
      if ((int *)0x66540f < piVar1) {
        return 1;
      }
    }
    (&DAT_00665050)[iVar2 * 6] = param_1;
    (&DAT_00665054)[iVar2 * 6] = 1;
    (&DAT_00665058)[iVar2 * 6] = 0;
    (&DAT_0066505c)[iVar2 * 6] = 0;
    (&DAT_00665060)[iVar2 * 6] = param_2;
  }
  return 1;
}
