/* SPDX-License-Identifier: MIT */
/* z_xvt 0049bf30 */
extern int FUN_00431d30(void);
extern int DAT_00525560;
extern int DAT_00645a90;
extern int DAT_00645ad0;
extern int DAT_0052555c;
extern int DAT_0052acc8;
extern int DAT_0052acc4;
extern int DAT_0052accc;

void __cdecl FUN_0049bf30(int param_1)
{
  int bVar1;
  int iVar2;
  int *piVar3;
  int *piVar4;
  
  iVar2 = FUN_00431d30();
  if ((iVar2 == 0) && (*(int *)(param_1 + 4) == DAT_00525560)) {
    piVar4 = (int *)(param_1 + 8);
    bVar1 = 0;
    piVar3 = &DAT_00645a90;
    do {
      if (*piVar4 != *piVar3) {
        bVar1 = 1;
      }
      piVar3 = piVar3 + 1;
      piVar4 = piVar4 + 1;
    } while (piVar3 < &DAT_00645ad0);
    if (!bVar1) {
      DAT_0052555c = 0;
      DAT_0052acc8 = DAT_0052acc4;
      DAT_0052accc = 0;
    }
  }
  return;
}
