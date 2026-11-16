/* SPDX-License-Identifier: MIT */
/* z_bop 0042d940 */
extern int DAT_00a10514;
extern int DAT_00a604d0;
extern int DAT_00a604d4;
extern int DAT_00a60530;
extern int FUN_0042dda0(char *param_1);

bool __cdecl FUN_0042d940(char *param_1, int param_2)
{
  int iVar1;
  int *piVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  
  if (DAT_00a10514 == 0) {
    return 0;
  }
  if (*param_1 == '\0') {
    return 0;
  }
  iVar1 = FUN_0042dda0(param_1);
  if (iVar1 == -1) {
    return 0;
  }
  iVar3 = -1;
  iVar5 = 0;
  iVar4 = -1;
  piVar2 = &DAT_00a604d0;
  do {
    if ((piVar2[-1] == iVar1) && (iVar3 < *piVar2)) {
      iVar3 = *piVar2;
      iVar4 = iVar5;
    }
    piVar2 = piVar2 + 3;
    iVar5 = iVar5 + 1;
  } while (piVar2 < &DAT_00a60530);
  if (iVar4 == -1) {
    return 0;
  }
  if (0x7f < param_2) {
    param_2 = 0x7f;
  }
  if (param_2 < 0) {
    param_2 = 0;
  }
  iVar1 = ((int (__stdcall *)(int *, int))(*(int **)(((int *)&DAT_00a604d4)[iVar4 * 3] + 0x40)))
                    ((int *)((int *)&DAT_00a604d4)[iVar4 * 3], ((param_2 * 5 + -0x13b) * 400) / 0x3f);
  return iVar1 == 0;
}
