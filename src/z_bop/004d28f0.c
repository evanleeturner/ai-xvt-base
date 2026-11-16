/* SPDX-License-Identifier: MIT */
/* z_bop 004d28f0 */
extern int DAT_00b69a89;
extern int DAT_00b641b9;
extern int DAT_00665050;
extern int DAT_00665058;
extern int DAT_00665410;
extern unsigned int __cdecl FUN_004d23b0(int);

int __cdecl FUN_004d28f0(int param_1)
{
  unsigned int uVar1;
  int iVar2;
  int *piVar3;
  int iVar4;
  
  uVar1 = FUN_004d23b0(param_1);
  if (uVar1 < (unsigned int)DAT_00b69a89 && uVar1 < 0x28) {
    iVar4 = (&DAT_00b641b9)[uVar1 * 0x8e];
  }
  else {
    iVar4 = 0;
  }
  iVar2 = 0;
  piVar3 = &DAT_00665050;
  do {
    if (*piVar3 == param_1) {
      return (&DAT_00665058)[iVar2 * 6] + iVar4;
    }
    piVar3 = piVar3 + 6;
    iVar2 = iVar2 + 1;
  } while (piVar3 < &DAT_00665410);
  return iVar4;
}
