/* SPDX-License-Identifier: MIT */
/* z_bop 004d24c0 */
extern int DAT_00acebf9;
extern int DAT_00665050;
extern int DAT_00665058;
extern int DAT_00665060;
extern int DAT_0066505c;
extern int DAT_00665410;
extern int DAT_00b69a89;
extern int DAT_00b641b9;
extern int DAT_00b641c1;
extern int DAT_00b641bd;

uint __cdecl FUN_004d23b0(int);

uint __cdecl FUN_004d24c0(int param_1)
{
  uint uVar1;
  int *piVar2;
  int iVar3;
  int iVar4;
  
  if (DAT_00acebf9 == 0) {
    iVar4 = 0;
    piVar2 = &DAT_00665050;
    do {
      if (*piVar2 == param_1) {
        uVar1 = (&DAT_00665058)[iVar4 * 6];
        if (uVar1 == 0) {
          uVar1 = 1;
        }
        uVar1 = (uint)(((&DAT_00665060)[iVar4 * 6] * 2 + (&DAT_0066505c)[iVar4 * 6]) * 10000) / uVar1;
        if ((int)uVar1 < 0x2711) {
          return uVar1;
        }
        return 10000;
      }
      piVar2 = piVar2 + 6;
      iVar4 = iVar4 + 1;
    } while (piVar2 < &DAT_00665410);
    uVar1 = 0;
  }
  else {
    uVar1 = FUN_004d23b0(param_1);
    if ((uVar1 < (uint)DAT_00b69a89) && (uVar1 < 0x28)) {
      iVar4 = (&DAT_00b641b9)[uVar1 * 0x8e];
      iVar3 = (&DAT_00b641c1)[uVar1 * 0x8e] * 2 + (&DAT_00b641bd)[uVar1 * 0x8e];
    }
    else {
      iVar4 = 0;
      iVar3 = 0;
    }
    piVar2 = &DAT_00665050;
    do {
      if (*piVar2 == param_1) {
        iVar4 = iVar4 + piVar2[2];
        iVar3 = iVar3 + piVar2[4] * 2 + piVar2[3];
      }
      piVar2 = piVar2 + 6;
    } while (piVar2 < &DAT_00665410);
    if (iVar4 == 0) {
      iVar4 = 1;
    }
    uVar1 = (iVar3 * 10000) / iVar4;
    if (10000 < (int)uVar1) {
      return 10000;
    }
  }
  return uVar1;
}
