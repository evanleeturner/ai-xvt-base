/* SPDX-License-Identifier: MIT */
/* z_xvt 004f0960 */
extern int DAT_0081b850;
extern int DAT_00757cee;
extern int DAT_007579b0;
extern int DAT_007579d0;
extern int DAT_00757a28;
extern int DAT_00757874;

extern int __stdcall FUN_004cde00(void);

int __stdcall FUN_004f0960(void)
{
  int iVar1;
  int *piVar2;
  int iVar3;
  int *piVar4;
  int iVar5;
  
  iVar1 = 1;
  if (DAT_0081b850 != 2) {
    iVar1 = FUN_004cde00();
  }
  iVar3 = 0;
  if (0 < iVar1) {
    piVar2 = &DAT_00757cee;
    do {
      iVar5 = 0;
      piVar4 = &DAT_007579b0;
      do {
        if ((*piVar2 != 0) && (*piVar4 == *piVar2)) break;
        piVar4 = piVar4 + 1;
        iVar5 = iVar5 + 1;
      } while (piVar4 < &DAT_007579d0);
      if (iVar5 == 8) {
        return 0;
      }
      piVar2 = (int *)((int)piVar2 + 0x2a);
      iVar3 = iVar3 + 1;
    } while (iVar3 < iVar1);
  }
  iVar1 = 0;
  if (0 < DAT_00757a28) {
    piVar2 = &DAT_00757874;
    do {
      iVar3 = 1;
      piVar4 = piVar2;
      do {
        if ((*piVar4 != 0) && (piVar2[-1] == 0)) {
          return 0;
        }
        piVar4 = piVar4 + 1;
        iVar3 = iVar3 + 1;
      } while (iVar3 < 8);
      piVar2 = piVar2 + 8;
      iVar1 = iVar1 + 1;
    } while (iVar1 < DAT_00757a28);
  }
  return 1;
}
