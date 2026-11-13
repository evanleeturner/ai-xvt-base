/* SPDX-License-Identifier: MIT */
/* z_xvt 004dec50 */
extern int DAT_00759aec;
extern char DAT_00759645;
extern void *DAT_00759af8;
extern int DAT_00758830;

int __cdecl FUN_004dee40(char *param_1);
void __stdcall FUN_004d4d90(void);
int __stdcall FUN_004d4d00(void);

int __cdecl FUN_004dec50(char *param_1)
{
  int *piVar1;
  char cVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  unsigned int uStack_4;
  
  if (DAT_00759aec == 0) {
    return 0;
  }
  if (*param_1 == '\0') {
    return 0;
  }
  iVar3 = FUN_004dee40(param_1);
  cVar2 = DAT_00759645;
  if (iVar3 == -1) {
    return 0;
  }
  iVar6 = 0;
  iVar5 = 0;
  FUN_004d4d90();
  do {
    if (*(int *)((char *)DAT_00759af8 + iVar6) == iVar3) {
      piVar1 = (int *)((int *)((char *)DAT_00759af8 + iVar6))[2];
      iVar4 = ((int (__stdcall *)(int *, unsigned int *))(*(int *)((char *)*piVar1 + 0x24)))(piVar1, &uStack_4);
      if ((iVar4 == 0) && ((uStack_4 & 5) != 0)) {
        iVar5 = iVar5 + 1;
      }
    }
    iVar6 = iVar6 + 0xc;
  } while (iVar6 < 0x90);
  if (cVar2 != '\0') {
    DAT_00758830 = FUN_004d4d00();
  }
  return iVar5;
}
