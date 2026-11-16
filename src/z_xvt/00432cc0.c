/* SPDX-License-Identifier: MIT */
/* z_xvt 00432cc0 */
extern int DAT_007330ac;
extern int DAT_007330e8;
extern int DAT_00733120;
extern int DAT_00733260;
extern int DAT_00733110;

int * __cdecl FUN_00432cc0(int param_1)
{
  int *piVar1;
  int *piVar2;
  int iVar3;
  int iVar4;

  if (DAT_007330ac == 1) {
    return &DAT_007330e8;
  }
  iVar3 = 0;
  if (DAT_007330ac < 1) {
    return (int *)DAT_007330ac;
  }
  piVar1 = &DAT_00733120;
  do {
    iVar4 = 0;
    piVar2 = &DAT_00733120;
    do {
      if (*piVar2 == *piVar1) {
        break;
      }
      piVar2 += 10;
      iVar4++;
    } while (piVar2 < &DAT_00733260);
    if (param_1 == iVar4) {
      return &DAT_00733110 + iVar3 * 10;
    }
    piVar1 += 10;
    iVar3++;
    if (DAT_007330ac <= iVar3) {
      return piVar1;
    }
  } while (1);
}
