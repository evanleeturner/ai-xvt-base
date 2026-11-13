/* SPDX-License-Identifier: MIT */
/* z_bop 004de4f0 */
extern char DAT_00aa7b0d;
extern char *DAT_00aa7fbf;
extern int DAT_00aa6cfc;

int __cdecl FUN_004df1b0(char *param_1);
void __stdcall FUN_004d4970(void);
int __cdecl FUN_004de9a0(char *param_1);
void __cdecl FUN_004df130(int param_1);
int __cdecl FUN_004d48e0(void);

int __cdecl FUN_004de4f0(char *param_1)
{
  int *piVar1;
  char cVar2;
  int bVar3;
  int iVar4;

  if (*param_1 == '\0') {
    return 0;
  }
  iVar4 = FUN_004df1b0(param_1);
  cVar2 = DAT_00aa7b0d;
  if (iVar4 == -1) {
    return 0;
  }
  FUN_004d4970();
  do {
    bVar3 = FUN_004de9a0(DAT_00aa7fbf + iVar4 * 0x145);
  } while (bVar3 == 1);
  piVar1 = *(int **)(DAT_00aa7fbf + 0x140 + iVar4 * 0x145);
  (*(void (__stdcall **)(int *))((char *)*piVar1 + 8))(piVar1);
  FUN_004df130(iVar4);
  if (cVar2 != '\0') {
    DAT_00aa6cfc = FUN_004d48e0();
  }
  return 1;
}
