/* SPDX-License-Identifier: MIT */
/* z_xvt 004eca20 */
extern unsigned int __cdecl FUN_004ecce0(int param_1);
extern int DAT_00757c90;
extern int DAT_00757cb0;
extern char DAT_00743db7[];
extern char DAT_007438d6[];
extern int DAT_007579d0[];
extern int DAT_00757870[];
extern int DAT_00757cee[];
extern int DAT_00757680[];
extern int DAT_00757d02[];

char __cdecl FUN_004eca20(int param_1)
{
  unsigned int uVar1;
  int iVar2;
  int iVar3;
  int *piVar4;
  int *piVar5;
  int local_8;
  int *local_4;

  if (param_1 == -1) {
    uVar1 = FUN_004ecce0(-1);
    if ((0 < (int)uVar1) && (((int)uVar1 < 4) || (uVar1 == 0xe))) {
      return '\0';
    }
    if (DAT_00757c90 != 0) {
      return DAT_00743db7[DAT_00757c90 + DAT_00757cb0 * 0x562];
    }
    if (DAT_007438d6[DAT_00757cb0 * 0x562] != '\0') {
      return DAT_007438d6[DAT_00757cb0 * 0x562];
    }
    return '\0';
  }

  iVar3 = 0;
  piVar4 = DAT_007579d0;
  local_4 = DAT_00757870;
  do {
    iVar2 = 0;
    if (0 < *piVar4) {
      piVar5 = local_4;
      do {
        if (*piVar5 == *(int *)((int)DAT_00757cee + param_1 * 0x2a)) {
          local_8 = *(int *)((int)DAT_00757680 + (iVar2 + iVar3) * 4);
          break;
        }
        piVar5 = piVar5 + 1;
        iVar2 = iVar2 + 1;
      } while (iVar2 < *piVar4);
    }
    iVar3 = iVar3 + 8;
    local_4 = local_4 + 8;
    piVar4 = piVar4 + 1;
    if ((int *)0x7579f7 < piVar4) {
      uVar1 = FUN_004ecce0(param_1);
      if ((0 < (int)uVar1) && (((int)uVar1 < 4) || (uVar1 == 0xe))) {
        return '\0';
      }
      iVar3 = *(int *)((int)DAT_00757d02 + param_1 * 0x2a);
      if (iVar3 != 0) {
        return DAT_00743db7[iVar3 + local_8 * 0x562];
      }
      if (DAT_007438d6[local_8 * 0x562] != '\0') {
        return DAT_007438d6[local_8 * 0x562];
      }
      return '\0';
    }
  } while (1);
}
