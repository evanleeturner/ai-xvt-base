/* SPDX-License-Identifier: MIT */
/* z_bop 00419510 */
extern int FUN_0046e040(void);
extern void FUN_004197b0(void);
extern int DAT_00523420;
extern int DAT_0052341c;
extern int DAT_00550b98;
extern int DAT_00550bd8;

void __cdecl FUN_00419510(int param_1)
{
  int iVar2;
  int *piVar3;
  int *piVar4;
  int bVar1;
  
  iVar2 = FUN_0046e040();
  if ((iVar2 == 0) && (*(int *)(param_1 + 4) == DAT_00523420)) {
    piVar4 = (int *)(param_1 + 8);
    bVar1 = 0;
    piVar3 = &DAT_00550b98;
    do {
      if (*piVar4 != *piVar3) {
        bVar1 = 1;
      }
      piVar3 = piVar3 + 1;
      piVar4 = piVar4 + 1;
    } while (piVar3 < &DAT_00550bd8);
    if (!bVar1) {
      DAT_0052341c = 0;
      FUN_004197b0();
    }
  }
  return;
}
