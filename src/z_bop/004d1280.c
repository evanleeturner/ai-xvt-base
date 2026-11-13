/* SPDX-License-Identifier: MIT */
/* z_bop 004d1280 */
extern int DAT_00acec45;
extern int DAT_00acf145;
extern int DAT_00acec49;

void __cdecl FUN_004d1280(int param_1)
{
  int iVar1;
  int *piVar2;
  
  iVar1 = 0;
  piVar2 = &DAT_00acec45;
  do {
    if (*piVar2 == param_1) break;
    piVar2 = piVar2 + 10;
    iVar1 = iVar1 + 1;
  } while (piVar2 < &DAT_00acf145);
  if (iVar1 != 0x20) {
    (&DAT_00acec49)[iVar1 * 10] = 0;
  }
  return;
}
