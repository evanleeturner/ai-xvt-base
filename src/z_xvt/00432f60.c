/* SPDX-License-Identifier: MIT */
/* z_xvt 00432f60 */
DWORD __stdcall timeGetTime(void);
void * FUN_00431d70(int *, undefined4 *);
void FUN_00430980(int, int);
extern int DAT_007330ac;
extern int DAT_00733100;

undefined4 __stdcall FUN_00432f60(void)
{
  DWORD DVar1;
  DWORD DVar2;
  int *piVar3;
  int *piVar4;
  int iVar5;
  int iVar6;
  int *piVar7;
  int local_8;
  undefined4 local_4;
  
  DVar1 = timeGetTime();
  do {
    DVar2 = timeGetTime();
    if (60000 < DVar2 - DVar1) {
      piVar3 = (int *)0x0;
      goto LAB_00432fb2;
    }
    while (piVar3 = (int *)FUN_00431d70(&local_8, &local_4), piVar3 != (int *)0x0) {
      if (local_8 != 0) {
        if (piVar3 != (int *)0x0) goto LAB_00432fb2;
        break;
      }
      FUN_00430980(*piVar3, (int)piVar3);
    }
  } while (1);

LAB_00432fb2:
  if (piVar3 == (int *)0x0) {
    return 0;
  }
  DAT_007330ac = *piVar3;
  iVar6 = 0;
  if (DAT_007330ac < 1) {
    return 1;
  }
  piVar3 = &DAT_00733100;
  do {
    DVar1 = timeGetTime();
LAB_00432fd9:
    DVar2 = timeGetTime();
    if (DVar2 - DVar1 < 0xea61) {
      while (piVar4 = (int *)FUN_00431d70(&local_8, &local_4), piVar4 != (int *)0x0) {
        if (local_8 != 0) {
          if (piVar4 != (int *)0x0) goto LAB_0043301d;
          break;
        }
        FUN_00430980(*piVar4, (int)piVar4);
      }
      goto LAB_00432fd9;
    }
    piVar4 = (int *)0x0;
LAB_0043301d:
    if (piVar4 == (int *)0x0) {
      return 0;
    }
    piVar7 = piVar3;
    for (iVar5 = 10; iVar5 != 0; iVar5 = iVar5 + -1) {
      *piVar7 = *piVar4;
      piVar4 = piVar4 + 1;
      piVar7 = piVar7 + 1;
    }
    iVar6 = iVar6 + 1;
    piVar3 = piVar3 + 10;
    if (DAT_007330ac <= iVar6) {
      return 1;
    }
  } while (1);
}
