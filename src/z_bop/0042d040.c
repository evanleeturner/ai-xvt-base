/* SPDX-License-Identifier: MIT */
/* z_bop 0042d040 */
int __cdecl FUN_0042dda0(char *param_1);
int __cdecl FUN_0042d600(void *param_1);
void __cdecl FUN_0042dd20(int param_1);
extern char DAT_00a1051c[];
extern char DAT_00a1065c[];

int __cdecl FUN_0042d040(char *param_1)
{
  int *piVar1;
  int iVar3;

  if (*param_1 == '\0') {
    return 0;
  }
  iVar3 = FUN_0042dda0(param_1);
  if (iVar3 == -1) {
    return 0;
  }
  do {
  } while (FUN_0042d600(DAT_00a1051c + iVar3 * 0x145) == 1);
  
  piVar1 = *(int **)(DAT_00a1065c + iVar3 * 0x145);
  (*(void (__stdcall **)(int *))(*piVar1 + 8))(piVar1);
  FUN_0042dd20(iVar3);
  return 1;
}
