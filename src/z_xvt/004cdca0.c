/* SPDX-License-Identifier: MIT */
/* z_xvt 004cdca0 */
extern int DAT_00780732;
extern int DAT_0078077a;
extern int DAT_0078077e;

int __cdecl FUN_004cdca0(int param_1)
{
  int iVar1;
  int *piVar2;
  int bVar3;
  
  iVar1 = 0;
  bVar3 = DAT_00780732 == 0;
  if (0 < DAT_00780732) {
    piVar2 = &DAT_0078077a;
    do {
      if (*piVar2 == param_1) break;
      piVar2 = piVar2 + 10;
      iVar1 = iVar1 + 1;
    } while (iVar1 < DAT_00780732);
    bVar3 = iVar1 == DAT_00780732;
  }
  if (bVar3) {
    return 0;
  }
  return (&DAT_0078077e)[iVar1 * 10];
}
