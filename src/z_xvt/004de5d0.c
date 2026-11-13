/* SPDX-License-Identifier: MIT */
/* z_xvt 004de5d0 */
extern int DAT_00759aec;
extern char DAT_00759645;
extern int DAT_00759b04;
extern void *DAT_00759af8;
extern int DAT_00759b00;
extern int DAT_00758830;

extern int __cdecl FUN_004dee40(char *param_1);
extern void __stdcall FUN_004d4d90(void);
extern int __stdcall FUN_004d4d00(void);

bool __cdecl FUN_004de5d0(char *param_1)
{
  char cVar1;
  int iVar2;
  int *piVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  
  if (DAT_00759aec == 0) {
    return 0;
  }
  if (*param_1 == '\0') {
    return 0;
  }
  iVar2 = FUN_004dee40(param_1);
  cVar1 = DAT_00759645;
  if (iVar2 == -1) {
    return 0;
  }
  iVar4 = -1;
  iVar6 = DAT_00759b04 + 1;
  iVar5 = 0;
  piVar3 = (int *)(DAT_00759af8 + 4);
  do {
    if ((piVar3[-1] == iVar2) && (*piVar3 < iVar6)) {
      iVar4 = iVar5;
      iVar6 = *piVar3;
    }
    piVar3 = piVar3 + 3;
    iVar5 = iVar5 + 1;
  } while (iVar5 < 0xc);
  if (iVar4 == -1) {
    return 0;
  }
  piVar3 = *(int **)(DAT_00759af8 + 8 + iVar4 * 0xc);
  if (piVar3 == (int *)0x0) {
    return 0;
  }
  FUN_004d4d90();
  iVar2 = (**(int (**)(void *))( *piVar3 + 0x48))(piVar3);
  (**(void (**)(void *))( *piVar3 + 8))(piVar3);
  *(int *)(DAT_00759af8 + 8 + iVar4 * 0xc) = 0;
  *(int *)(DAT_00759af8 + iVar4 * 0xc) = 0xffffffff;
  DAT_00759b00 = DAT_00759b00 + -1;
  if (cVar1 != '\0') {
    DAT_00758830 = FUN_004d4d00();
  }
  return -1 < iVar2;
}
