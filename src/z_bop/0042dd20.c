/* SPDX-License-Identifier: MIT */
/* z_bop 0042dd20 */
extern int DAT_00a1051c;
extern int DAT_00a606d4;
extern int DAT_00a604cc;
extern int DAT_00a6052c;

void __cdecl FUN_0042dd20(int param_1)
{
  int iVar1;
  int *piVar2;
  int iVar3;
  int *puVar4;
  int *puVar5;
  int *puVar6;
  
  if (param_1 > -1 && param_1 < DAT_00a606d4) {
    if (param_1 < DAT_00a606d4 - 1) {
      iVar3 = param_1;
      puVar4 = (int *)((char *)&DAT_00a1051c + param_1 * 0x145);
      do {
        puVar5 = (int *)((char *)puVar4 + 0x145);
        puVar6 = puVar4;
        for (iVar1 = 0x51; iVar1 != 0; iVar1--) {
          *puVar6 = *puVar5;
          puVar5++;
          puVar6++;
        }
        *(char *)puVar6 = *(char *)puVar5;
        iVar3++;
        puVar4 = (int *)((char *)puVar4 + 0x145);
      } while (iVar3 < DAT_00a606d4 - 1);
    }
    piVar2 = &DAT_00a604cc;
    DAT_00a606d4--;
    do {
      if (param_1 < *piVar2) {
        (*piVar2)--;
      }
      piVar2 += 3;
    } while (piVar2 < &DAT_00a6052c);
  }
  return;
}
