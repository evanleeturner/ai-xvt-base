/* SPDX-License-Identifier: MIT */
/* z_bop 004d2950 */
extern unsigned int DAT_00b69a89;
extern int DAT_00b641bd;
extern int DAT_00665050;
extern int DAT_0066505c;
extern int DAT_00665410;

extern int FUN_004d23b0(int);

int __cdecl FUN_004d2950(int param_1)
{
  unsigned int uVar1;
  int *piVar2;
  int iVar3;
  int iVar4;
  
  uVar1 = FUN_004d23b0(param_1);
  if ((uVar1 < DAT_00b69a89) && (uVar1 < 0x28)) {
    iVar4 = (&DAT_00b641bd)[uVar1 * 0x8e];
  }
  else {
    iVar4 = 0;
  }
  iVar3 = 0;
  piVar2 = &DAT_00665050;
  do {
    if (*piVar2 == param_1) {
      FUN_004d23b0(param_1);
      return (&DAT_0066505c)[iVar3 * 6] + iVar4;
    }
    piVar2 = piVar2 + 6;
    iVar3 = iVar3 + 1;
  } while (piVar2 < &DAT_00665410);
  return iVar4;
}
