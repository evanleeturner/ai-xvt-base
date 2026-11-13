/* SPDX-License-Identifier: MIT */
/* z_bop 0042d600 */
extern int DAT_00a10514;
extern int DAT_00a606dc;
extern int DAT_00a604d0;
extern int DAT_00a60530;
extern int DAT_00a604d4;
extern int DAT_00a604cc;
extern int DAT_00a606d8;
extern int FUN_0042dda0(char *);

bool __cdecl FUN_0042d600(char *param_1)
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
  iVar5 = DAT_00a606dc + 1;
  iVar4 = 0;
  piVar2 = &DAT_00a604d0;
  do {
    if ((piVar2[-1] == iVar1) && (*piVar2 < iVar5)) {
      iVar3 = iVar4;
      iVar5 = *piVar2;
    }
    piVar2 = piVar2 + 3;
    iVar4 = iVar4 + 1;
  } while (piVar2 < &DAT_00a60530);
  if (iVar3 == -1) {
    return 0;
  }
  piVar2 = (int *)(&DAT_00a604d4)[iVar3 * 3];
  if (piVar2 == (int *)0x0) {
    return 0;
  }
  iVar1 = ((int (*)(int *))(*piVar2 + 0x48))(piVar2);
  ((void (*)(int *))(*piVar2 + 8))(piVar2);
  (&DAT_00a604d4)[iVar3 * 3] = 0;
  (&DAT_00a604cc)[iVar3 * 3] = 0xffffffff;
  DAT_00a606d8 = DAT_00a606d8 + -1;
  return -1 < iVar1;
}
