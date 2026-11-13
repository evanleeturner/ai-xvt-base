/* SPDX-License-Identifier: MIT */
/* z_bop 004d1240 */
extern int DAT_00acebfd;
extern int DAT_00acec45;
extern int DAT_00acec49;

void __cdecl FUN_004d1240(int param_1)
{
  int iVar1;
  int *piVar2;
  int bVar3;
  
  iVar1 = 0;
  bVar3 = (DAT_00acebfd == 0);
  if (DAT_00acebfd > 0) {
    piVar2 = &DAT_00acec45;
    do {
      if (*piVar2 == param_1) {
        break;
      }
      piVar2 += 10;
      iVar1 += 1;
    } while (iVar1 < DAT_00acebfd);
    bVar3 = (DAT_00acebfd == iVar1);
  }
  if (!bVar3) {
    (&DAT_00acec49)[iVar1 * 10] = 1;
  }
  return;
}
