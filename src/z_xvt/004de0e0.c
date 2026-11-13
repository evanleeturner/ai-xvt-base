/* SPDX-License-Identifier: MIT */
/* z_xvt 004de0e0 */
int __cdecl FUN_004dee40(char *param_1);
void __stdcall FUN_004d4d90(void);
int __cdecl FUN_004de5d0(char *param_1);
void __cdecl FUN_004dedc0(int param_1);
int __cdecl FUN_004d4d00(void);

extern char DAT_00759645;
extern char *DAT_00759af4;
extern int DAT_00758830;

int __cdecl FUN_004de0e0(char *param_1)
{
  int *piVar1;
  char cVar2;
  int bVar3;
  int iVar4;

  if (*param_1 == '\0') {
    return 0;
  }
  iVar4 = FUN_004dee40(param_1);
  cVar2 = DAT_00759645;
  if (iVar4 == -1) {
    return 0;
  }
  FUN_004d4d90();
  do {
    bVar3 = FUN_004de5d0((char *)(DAT_00759af4 + iVar4 * 0x145));
  } while (bVar3 == 1);
  piVar1 = *(int **)(DAT_00759af4 + 0x140 + iVar4 * 0x145);
  (*(void (__stdcall **)(int *))(*piVar1 + 8))(piVar1);
  FUN_004dedc0(iVar4);
  if (cVar2 != '\0') {
    DAT_00758830 = FUN_004d4d00();
  }
  return 1;
}
