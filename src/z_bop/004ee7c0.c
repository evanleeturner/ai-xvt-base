/* SPDX-License-Identifier: MIT */
/* z_bop 004ee7c0 */
extern int DAT_00aa610c;
extern int DAT_00aa6104;
extern char DAT_00a921fd;
extern char DAT_00a921e3;
extern int DAT_00aa5e10;
extern int DAT_00aa5cb0;
extern int DAT_00aa615e;
extern int DAT_00aa5ac0;
extern int DAT_00aa6176;

char __cdecl FUN_004ee7c0(int param_1)
{
  int iVar1;
  int *piVar2;
  int iVar3;
  int *piVar4;
  int *local_8;
  int local_4;
  
  if (param_1 == -1) {
    if (DAT_00aa610c != 0) {
      return (&DAT_00a921fd)[DAT_00aa610c + DAT_00aa6104 * 0x562];
    }
    if ((&DAT_00a921e3)[DAT_00aa6104 * 0x562] != '\0') {
      return (&DAT_00a921e3)[DAT_00aa6104 * 0x562];
    }
    return '\0';
  }
  iVar3 = 0;
  piVar2 = &DAT_00aa5e10;
  local_8 = &DAT_00aa5cb0;
  do {
    iVar1 = 0;
    if (0 < *piVar2) {
      piVar4 = local_8;
      do {
        if (*piVar4 == *(int *)((int)&DAT_00aa615e + param_1 * 0x2a)) {
          local_4 = (&DAT_00aa5ac0)[iVar1 + iVar3];
          break;
        }
        piVar4 = piVar4 + 1;
        iVar1 = iVar1 + 1;
      } while (iVar1 < *piVar2);
    }
    iVar3 = iVar3 + 8;
    piVar2 = piVar2 + 1;
    local_8 = local_8 + 8;
    if ((int *)0xaa5e37 < piVar2) {
      iVar3 = *(int *)((int)&DAT_00aa6176 + param_1 * 0x2a);
      if (iVar3 != 0) {
        return (&DAT_00a921fd)[iVar3 + local_4 * 0x562];
      }
      if ((&DAT_00a921e3)[local_4 * 0x562] != '\0') {
        return (&DAT_00a921e3)[local_4 * 0x562];
      }
      return '\0';
    }
  } while (1);
}
