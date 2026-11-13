/* SPDX-License-Identifier: MIT */
/* z_bop 004d29c0 */
extern unsigned int DAT_00b69a89;
extern int DAT_00b641c1[];
extern int DAT_00665050[];
extern int DAT_00665060[];
extern int DAT_00665410[];
extern unsigned int FUN_004d23b0(int param_1);

int __cdecl FUN_004d29c0(int param_1)
{
  unsigned int uVar1;
  int iVar2;
  int *piVar3;
  int iVar4;
  
  uVar1 = FUN_004d23b0(param_1);
  if ((uVar1 < DAT_00b69a89) && (uVar1 < 0x28)) {
    iVar4 = DAT_00b641c1[uVar1 * 0x8e];
  }
  else {
    iVar4 = 0;
  }
  iVar2 = 0;
  piVar3 = &DAT_00665050[0];
  do {
    if (*piVar3 == param_1) {
      return DAT_00665060[iVar2 * 6] + iVar4;
    }
    piVar3 = piVar3 + 6;
    iVar2 = iVar2 + 1;
  } while (piVar3 < &DAT_00665410[0]);
  return iVar4;
}
