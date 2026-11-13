/* SPDX-License-Identifier: MIT */
/* z_bop 004bf100 */
extern void *DAT_00b69cc0;
extern void **DAT_00b69e28;
extern const char s___plt_0052b35c[];
extern int DAT_0051a854;
extern const char DAT_00b6a2e0[];

void __cdecl _free(void *);
void __cdecl FUN_004dfc30(void *);
void __stdcall FUN_004ccf50(void);
void * __cdecl FUN_004dfa10(const char *);
void __stdcall FUN_004ccf70(void);
void * __cdecl _malloc(size_t);
HFILE __cdecl FUN_004cc4a0(const char *, void *);
int __cdecl FUN_004cc6c0(HFILE, void *, size_t);
int __cdecl __strcmpi(const char *, const char *);
void __cdecl FUN_004cc590(HFILE);

int __cdecl FUN_004bf100(int *param_1)
{
  HFILE pFVar1;
  int iVar2;
  unsigned int uVar3;
  int iVar4;
  void **puVar5;
  void **puVar6;
  int iVar7;

  if (DAT_00b69cc0 != NULL) {
    _free(DAT_00b69cc0);
    DAT_00b69cc0 = NULL;
  }
  if (DAT_00b69e28 != NULL) {
    FUN_004dfc30(DAT_00b69e28);
    DAT_00b69e28 = NULL;
  }
  FUN_004ccf50();
  DAT_00b69e28 = FUN_004dfa10(s___plt_0052b35c);
  FUN_004ccf70();
  if (DAT_00b69e28 == NULL) {
    return 0;
  }
  puVar5 = (void **)*DAT_00b69e28;
  if ((int)DAT_00b69e28[1] < 1) {
    DAT_00b69cc0 = NULL;
  }
  else {
    DAT_00b69cc0 = _malloc((size_t)DAT_00b69e28[1] * 0xe);
    iVar4 = (int)DAT_00b69e28[1];
    puVar6 = DAT_00b69cc0;
    for (uVar3 = (unsigned int)(iVar4 * 0xe) >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
      *(int *)puVar6 = 0;
      puVar6 = puVar6 + 1;
    }
    for (uVar3 = iVar4 * 0xe & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
      *(char *)puVar6 = 0;
      puVar6 = (void **)((int)puVar6 + 1);
    }
  }
  if (DAT_00b69cc0 == NULL) {
    return 0;
  }
  iVar4 = 0;
  if (puVar5 != NULL) {
    iVar7 = 0;
    do {
      pFVar1 = FUN_004cc4a0((char *)*puVar5, &DAT_0051a854);
      if (pFVar1 != 0) {
        FUN_004cc6c0(pFVar1, (char *)((int)DAT_00b69cc0 + iVar7), 0xc);
        *(char *)((int)DAT_00b69cc0 + iVar7 + 0xc) = 0;
        iVar2 = __strcmpi((char *)((int)DAT_00b69cc0 + iVar7), (char *)&DAT_00b6a2e0);
        if (iVar2 == 0) {
          *param_1 = iVar4;
        }
        iVar7 = iVar7 + 0xe;
        iVar4 = iVar4 + 1;
        FUN_004cc590(pFVar1);
      }
      puVar5 = (void **)puVar5[1];
    } while (puVar5 != NULL);
  }
  return 1;
}
