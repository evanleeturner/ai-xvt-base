/* SPDX-License-Identifier: MIT */
/* z_xvt 0049b200 */
extern int DAT_006db730;
extern int DAT_0071f720;

int * __cdecl FUN_0049b200(int param_1)
{
  int *piVar1;
  int iVar2;
  int *piVar3;
  
  iVar2 = (&DAT_006db730)[param_1];
  piVar3 = &DAT_0071f720 + param_1 * 0x7e9;
  piVar1 = (int *)0x0;
  if (0 < iVar2) {
    do {
      if (*piVar3 != 0) {
        piVar1 = piVar3;
      }
      piVar3 = (int *)((int)piVar3 + 0x12);
      iVar2 = iVar2 + -1;
    } while (iVar2 != 0);
  }
  return piVar1;
}
