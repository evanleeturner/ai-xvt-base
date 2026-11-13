/* SPDX-License-Identifier: MIT */
/* z_bop 004bf8c0 */
void __cdecl FUN_004ccf50(void);
unsigned int * __cdecl FUN_004dfa10(char *);
void __cdecl FUN_004ccf70(void);
void __cdecl FUN_004dfc30(unsigned int *);
void * __cdecl FUN_004cc4a0(char *, char *);
void __cdecl FUN_004cc6c0(void *, void *, unsigned int);
void __cdecl FUN_004cc590(void *);
int __cdecl __strcmpi(char *, char *);
int __cdecl FUN_004cb0c0(char *);

extern char s___plt_0052b35c[];
extern char DAT_0051a854;
extern char DAT_00b69d20;

unsigned int __cdecl FUN_004bf8c0(char *param_1)
{
  unsigned int *puVar1;
  void *pFVar2;
  int iVar3;
  int iVar4;
  unsigned int *puVar5;
  unsigned int local_4;
  
  FUN_004ccf50();
  puVar1 = FUN_004dfa10(s___plt_0052b35c);
  FUN_004ccf70();
  if (puVar1 == (unsigned int *)0x0) {
    return 0;
  }
  if (param_1 == (char *)0x0) {
    FUN_004dfc30(puVar1);
    return 0;
  }
  if (*param_1 == '\0') {
    FUN_004dfc30(puVar1);
    return 0;
  }
  puVar5 = (unsigned int *)*puVar1;
  iVar4 = 0;
  local_4 = 0;
  if (0 < (int)puVar1[1]) {
    do {
      pFVar2 = FUN_004cc4a0((char *)*puVar5, &DAT_0051a854);
      if (pFVar2 != (void *)0x0) {
        FUN_004cc6c0(pFVar2, &DAT_00b69d20, 0xe);
        FUN_004cc590(pFVar2);
        iVar3 = __strcmpi((char *)&DAT_00b69d20, param_1);
        if (iVar3 == 0) {
          iVar4 = FUN_004cb0c0((char *)*puVar5);
          if (iVar4 != 0) {
            local_4 = 1;
          }
          break;
        }
      }
      puVar5 = (unsigned int *)puVar5[1];
      iVar4 = iVar4 + 1;
    } while (iVar4 < (int)puVar1[1]);
  }
  FUN_004dfc30(puVar1);
  return local_4;
}
