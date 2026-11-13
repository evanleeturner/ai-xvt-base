/* SPDX-License-Identifier: MIT */
/* z_xvt 0049b0d0 */
extern int DAT_006db730[];
extern int DAT_0071f720[];

int * __cdecl FUN_0049b0d0(int param_1, int param_2, int *param_3)
{
  int iVar1;
  int *piVar2;
  int iVar3;
  int iVar4;
  int *piVar5;

  iVar1 = DAT_006db730[param_1];
  iVar4 = 0;
  piVar2 = &DAT_0071f720[param_1 * 0x7e9];
  piVar5 = (int *)(iVar1 * 0x12 + (int)piVar2);
  if (0 < iVar1) {
    do {
      iVar3 = piVar2[2];
      if (param_2 <= iVar3) goto LAB_0049b122;
      iVar4 = iVar4 + 1;
      piVar2 = (int *)((int)piVar2 + 0x12);
    } while (iVar4 < iVar1);
  }
  iVar3 = piVar2[2];
LAB_0049b122:
  if ((param_2 < iVar3) || (iVar1 == iVar4)) {
    if (iVar1 == 0x1c2) {
      return (int *)0x0;
    }
    DAT_006db730[param_1] = iVar1 + 1;
    for (; piVar2 < piVar5; piVar5 = (int *)((int)piVar5 - 0x12)) {
      *piVar5 = *(int *)((int)piVar5 - 0x12);
      piVar5[1] = *(int *)((int)piVar5 - 0xe);
      piVar5[2] = *(int *)((int)piVar5 - 10);
      piVar5[3] = *(int *)((int)piVar5 - 6);
      *(unsigned short *)(piVar5 + 4) = *(unsigned short *)((int)piVar5 - 2);
    }
  }
  else if (param_2 == iVar3) {
    if (piVar2[1] == 0) {
      return (int *)0x0;
    }
    if (*piVar2 == 1) {
      return (int *)0x0;
    }
  }
  piVar2[2] = param_2;
  piVar2[1] = 1;
  *piVar2 = 0;
  piVar2[3] = *param_3;
  *(short *)(piVar2 + 4) = (short)param_3[1];
  return piVar2;
}
