/* SPDX-License-Identifier: MIT */
/* z_xvt 004cdce0 */
extern int DAT_00780732;
extern int DAT_0078075a;
extern int DAT_0078077a;

int * __cdecl FUN_004cdce0(int param_1)
{
  int iVar2 = 0;
  int bVar3 = (DAT_00780732 == 0);
  if (DAT_00780732 > 0) {
    int *piVar1 = &DAT_0078077a;
    do {
      if (*piVar1 == param_1) break;
      piVar1 = piVar1 + 10;
      iVar2 = iVar2 + 1;
    } while (iVar2 < DAT_00780732);
    bVar3 = DAT_00780732 == iVar2;
  }
  if (bVar3) {
    return (int *)0x0;
  }
  return &DAT_0078075a + iVar2 * 10;
}
