/* SPDX-License-Identifier: MIT */
/* z_xvt 004c04d0 */
extern void ** __cdecl FUN_004dfc80(char *param_1);
extern void * __cdecl FUN_004be160(char *param_1, char *param_2);
extern int __cdecl FUN_004be320(void *param_1, void *param_2, int param_3);
extern unsigned int __cdecl FUN_004be1f0(void *param_1);
extern int __cdecl __strcmpi(char *param_1, char *param_2);
extern void __cdecl FUN_004dfea0(void *param_1);
extern int __cdecl _sprintf(char *param_1, char *param_2, ...);
extern int __cdecl FUN_004be3e0(void *param_1, void *param_2, int param_3);

extern char s___plt_005320d0[];
extern char s___temp___tmp_005320f4[];
extern char s__s_plt_005320ec[];
extern char DAT_00523570[];
extern char DAT_0081b740[];
extern char DAT_0081bcf0[];
extern char DAT_0052a324[];

int __cdecl FUN_004c04d0(int param_1)
{
  char local_20[16];
  char *pcVar9;
  int iVar5;
  void *pFVar3;
  int iVar4;
  unsigned int uVar6;
  unsigned int uVar7;
  void **puVar8;
  char *pcVar10;
  char cVar1;
  void **puVar2;

  pcVar9 = local_20;
  for (iVar5 = 7; iVar5 != 0; iVar5 = iVar5 - 1) {
    pcVar9[0] = '\0';
    pcVar9[1] = '\0';
    pcVar9[2] = '\0';
    pcVar9[3] = '\0';
    pcVar9 = pcVar9 + 4;
  }
  pcVar9[0] = '\0';
  pcVar9[1] = '\0';

  if (param_1 != 0) {
    *(int *)local_20 = *(int *)s___temp___tmp_005320f4;
    *(int *)(local_20 + 4) = *(int *)(s___temp___tmp_005320f4 + 4);
    *(int *)(local_20 + 8) = *(int *)(s___temp___tmp_005320f4 + 8);
    *(int *)(local_20 + 12) = *(int *)(s___temp___tmp_005320f4 + 12);
    goto LAB_004c05c0;
  }

  puVar2 = FUN_004dfc80(s___plt_005320d0);
  if (puVar2 == (void **)0x0) goto LAB_004c05c0;
  puVar8 = (void **)*puVar2;
  iVar5 = 0;
  if (0 < (int)puVar2[1]) {
    do {
      pFVar3 = FUN_004be160((char *)*puVar8, DAT_00523570);
      if (pFVar3 != (void *)0x0) {
        FUN_004be320(pFVar3, DAT_0081b740, 0xe);
        FUN_004be1f0(pFVar3);
        iVar4 = __strcmpi(DAT_0081b740, DAT_0081bcf0);
        if (iVar4 == 0) {
          uVar6 = 0xffffffff;
          pcVar9 = (char *)*puVar8;
          goto code_r0x004c056c;
        }
      }
      puVar8 = (void **)puVar8[1];
      iVar5 = iVar5 + 1;
    } while (iVar5 < (int)puVar2[1]);
  }
  goto LAB_004c0586;

code_r0x004c056c:
  while (1) {
    uVar6 = uVar6 - 1;
    pcVar10 = pcVar9 + 1;
    cVar1 = *pcVar9;
    pcVar9 = pcVar10;
    if (cVar1 == '\0') break;
    if (uVar6 == 0) break;
  }
  uVar6 = ~uVar6;
  pcVar9 = pcVar10 - uVar6;
  pcVar10 = local_20;
  for (uVar7 = uVar6 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
    *(int *)pcVar10 = *(int *)pcVar9;
    pcVar9 = pcVar9 + 4;
    pcVar10 = pcVar10 + 4;
  }
  for (uVar6 = uVar6 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
    *pcVar10 = *pcVar9;
    pcVar9 = pcVar9 + 1;
    pcVar10 = pcVar10 + 1;
  }

LAB_004c0586:
  FUN_004dfea0(puVar2);

LAB_004c05c0:
  if (DAT_0081bcf0[0] == '\0') {
    return 0;
  }
  if (local_20[0] == '\0') {
    _sprintf(local_20, s__s_plt_005320ec, DAT_0081bcf0);
  }
  pFVar3 = FUN_004be160(local_20, DAT_0052a324);
  if (pFVar3 != (void *)0x0) {
    FUN_004be3e0(pFVar3, DAT_0081bcf0, 0x3df3a);
    FUN_004be1f0(pFVar3);
    return 1;
  }
  return 0;
}
