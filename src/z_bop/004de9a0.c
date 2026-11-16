/* SPDX-License-Identifier: MIT */
/* z_bop 004de9a0 */
extern int DAT_00aa7fb7;
extern char DAT_00aa7b0d;
extern int DAT_00aa7fcf;
extern int DAT_00aa7fc3;
extern int DAT_00aa7fcb;
extern int DAT_00aa6cfc;

int __cdecl FUN_004df1b0(char *param_1);
void __stdcall FUN_004d4970(void);
int __cdecl FUN_004d48e0(void);

int __cdecl FUN_004de9a0(char *param_1)
{
  char cVar1;
  int iVar2;
  int *piVar3;
  int iVar4;
  int iVar5;
  int iVar6;

  if (DAT_00aa7fb7 == 0) {
    return 0;
  }
  if (*param_1 == '\0') {
    return 0;
  }
  iVar2 = FUN_004df1b0(param_1);
  cVar1 = DAT_00aa7b0d;
  if (iVar2 == -1) {
    return 0;
  }
  iVar6 = -1;
  iVar5 = DAT_00aa7fcf + 1;
  iVar4 = 0;
  piVar3 = (int *)(DAT_00aa7fc3 + 4);
  do {
    if ((piVar3[-1] == iVar2) && (*piVar3 < iVar5)) {
      iVar5 = *piVar3;
      iVar6 = iVar4;
    }
    piVar3 = piVar3 + 3;
    iVar4 = iVar4 + 1;
  } while (iVar4 < 0xc);
  if (iVar6 == -1) {
    return 0;
  }
  piVar3 = *(int **)(DAT_00aa7fc3 + 8 + iVar6 * 0xc);
  if (piVar3 == (int *)0x0) {
    return 0;
  }
  FUN_004d4970();
  iVar2 = ((int (__cdecl *)(int *))(*(int **)piVar3 + 0x48))(piVar3);
  ((void (__cdecl *)(int *))(*(int **)piVar3 + 8))(piVar3);
  *(int *)(DAT_00aa7fc3 + 8 + iVar6 * 0xc) = 0;
  *(int *)(DAT_00aa7fc3 + iVar6 * 0xc) = 0xffffffff;
  DAT_00aa7fcb = DAT_00aa7fcb + -1;
  if (cVar1 != '\0') {
    DAT_00aa6cfc = FUN_004d48e0();
  }
  return iVar2 > -1;
}
