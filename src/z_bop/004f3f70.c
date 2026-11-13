/* SPDX-License-Identifier: MIT */
/* z_bop 004f3f70 */
extern int DAT_00b69e30;
extern int DAT_00aa615e;
extern int DAT_00aa5df0;
extern int DAT_00aa5e10;
extern int DAT_00aa5e68;
extern int DAT_00aa5cb4;

int __stdcall FUN_004d1400(void);

int __stdcall FUN_004f3f70(void)
{
  int iVar1;
  int iVar2;
  int iVar3;
  int *piVar4;
  int *piVar5;
  
  iVar1 = 1;
  if (DAT_00b69e30 != 2) {
    iVar1 = FUN_004d1400();
  }
  iVar2 = 0;
  if (0 < iVar1) {
    piVar5 = &DAT_00aa615e;
    do {
      iVar3 = 0;
      piVar4 = &DAT_00aa5df0;
      do {
        if ((*piVar5 != 0) && (*piVar4 == *piVar5)) break;
        piVar4 = piVar4 + 1;
        iVar3 = iVar3 + 1;
      } while (piVar4 < &DAT_00aa5e10);
      if (iVar3 == 8) {
        return 0;
      }
      piVar5 = (int *)((int)piVar5 + 0x2a);
      iVar2 = iVar2 + 1;
    } while (iVar2 < iVar1);
  }
  iVar1 = 0;
  if (0 < DAT_00aa5e68) {
    piVar5 = &DAT_00aa5cb4;
    do {
      iVar2 = 1;
      piVar4 = piVar5;
      do {
        if ((*piVar4 != 0) && (piVar5[-1] == 0)) {
          return 0;
        }
        piVar4 = piVar4 + 1;
        iVar2 = iVar2 + 1;
      } while (iVar2 < 8);
      piVar5 = piVar5 + 8;
      iVar1 = iVar1 + 1;
    } while (iVar1 < DAT_00aa5e68);
  }
  return 1;
}
