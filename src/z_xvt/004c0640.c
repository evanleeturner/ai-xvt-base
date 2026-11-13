/* SPDX-License-Identifier: MIT */
/* z_xvt 004c0640 */
extern unsigned int *FUN_004dfc80(const char *param_1);
extern void FUN_004dfea0(unsigned int *param_1);
extern void *FUN_004be160(char *param_1, char *param_2);
extern void FUN_004be320(void *param_1, void *param_2, unsigned int param_3);
extern void FUN_004be1f0(void *param_1);
extern int __strcmpi(const char *param_1, const char *param_2);
extern char DAT_00523570[];
extern char DAT_0081b740[];
extern char DAT_0081bcf0[];
extern char s___plt_005320d0[];

unsigned int __cdecl FUN_004c0640(char *param_1)
{
  unsigned int *puVar1;
  void *pFVar2;
  int iVar3;
  unsigned int *puVar4;
  int iVar5;
  unsigned int local_4;
  
  puVar1 = FUN_004dfc80(s___plt_005320d0);
  if (puVar1 == (unsigned int *)0x0) {
    return 0;
  }
  if (param_1 == (char *)0x0) {
    FUN_004dfea0(puVar1);
    return 0;
  }
  if (*param_1 == '\0') {
    FUN_004dfea0(puVar1);
    return 0;
  }
  puVar4 = (unsigned int *)*puVar1;
  iVar5 = 0;
  local_4 = 0;
  if (0 < (int)puVar1[1]) {
    do {
      pFVar2 = FUN_004be160((char *)*puVar4, DAT_00523570);
      if (pFVar2 != (void *)0x0) {
        FUN_004be320(pFVar2, DAT_0081b740, 0xe);
        FUN_004be1f0(pFVar2);
        iVar3 = __strcmpi(DAT_0081b740, param_1);
        if (iVar3 == 0) {
          pFVar2 = FUN_004be160((char *)*puVar4, DAT_00523570);
          FUN_004be320(pFVar2, DAT_0081bcf0, 0x3df3a);
          local_4 = 1;
          FUN_004be1f0(pFVar2);
          break;
        }
      }
      puVar4 = (unsigned int *)puVar4[1];
      iVar5 = iVar5 + 1;
    } while (iVar5 < (int)puVar1[1]);
  }
  FUN_004dfea0(puVar1);
  return local_4;
}
