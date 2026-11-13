/* SPDX-License-Identifier: MIT */
/* z_bop 0042dc60 */
extern int DAT_00a606d4;
extern char DAT_00a1051c;
extern int DAT_00a604cc;
extern int DAT_00a6052c;
int __cdecl _strncmp(const char *, const char *, unsigned int);

void __cdecl FUN_0042dc60(char *param_1)
{
  int iVar1;
  int iVar2;
  int *piVar3;
  int *puVar4;
  int iVar5;
  char *pcVar6;
  int *puVar7;
  int *puVar8;
  
  iVar5 = 0;
  if (0 < DAT_00a606d4) {
    pcVar6 = &DAT_00a1051c;
    do {
      iVar1 = _strncmp(param_1, pcVar6, 0x40);
      if (iVar1 < 0) break;
      pcVar6 = pcVar6 + 0x145;
      iVar5 = iVar5 + 1;
    } while (iVar5 < DAT_00a606d4);
  }
  if (iVar5 < DAT_00a606d4) {
    iVar1 = DAT_00a606d4 - iVar5;
    puVar4 = (int *)(&DAT_00a1051c + DAT_00a606d4 * 0x145);
    do {
      puVar7 = (int *)((int)puVar4 + -0x145);
      puVar8 = puVar4;
      for (iVar2 = 0x51; iVar2 != 0; iVar2 = iVar2 + -1) {
        *puVar8 = *puVar7;
        puVar7 = puVar7 + 1;
        puVar8 = puVar8 + 1;
      }
      *(char *)puVar8 = *(char *)puVar7;
      iVar1 = iVar1 + -1;
      puVar4 = (int *)((int)puVar4 + -0x145);
    } while (iVar1 != 0);
  }
  pcVar6 = &DAT_00a1051c + iVar5 * 0x145;
  for (iVar1 = 0x51; iVar1 != 0; iVar1 = iVar1 + -1) {
    *(int *)pcVar6 = *(int *)param_1;
    param_1 = param_1 + 4;
    pcVar6 = pcVar6 + 4;
  }
  *pcVar6 = *param_1;
  piVar3 = &DAT_00a604cc;
  DAT_00a606d4 = DAT_00a606d4 + 1;
  do {
    if (iVar5 <= *piVar3) {
      *piVar3 = *piVar3 + 1;
    }
    piVar3 = piVar3 + 3;
  } while (piVar3 < &DAT_00a6052c);
  return;
}
