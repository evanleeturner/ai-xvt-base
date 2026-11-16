/* SPDX-License-Identifier: MIT */
/* z_bop 004defd0 */
extern int DAT_00aa7fb7;
extern char DAT_00aa7b0d;
extern int DAT_00aa7fc3;
extern int DAT_00aa6cfc;

int __cdecl FUN_004df1b0(char *param_1);
void __stdcall FUN_004d4970(void);
int __stdcall FUN_004d48e0(void);

int __cdecl FUN_004defd0(char *param_1)
{
  int *piVar1;
  char cVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  unsigned char local_4[4];

  if (DAT_00aa7fb7 == 0) {
    return 0;
  }
  if (*param_1 == '\0') {
    return 0;
  }
  iVar3 = FUN_004df1b0(param_1);
  cVar2 = DAT_00aa7b0d;
  if (iVar3 == -1) {
    return 0;
  }
  iVar6 = 0;
  iVar5 = 0;
  FUN_004d4970();
  do {
    if (*(int *)(DAT_00aa7fc3 + iVar6) == iVar3) {
      piVar1 = (int *)((int *)(DAT_00aa7fc3 + iVar6))[2];
      iVar4 = ((int (__stdcall *)(int *, unsigned char *))(*(void **)(*piVar1 + 0x24)))(piVar1, local_4);
      if ((iVar4 == 0) && ((local_4[0] & 5) != 0)) {
        iVar5 = iVar5 + 1;
      }
    }
    iVar6 = iVar6 + 0xc;
  } while (iVar6 < 0x90);
  if (cVar2 != '\0') {
    DAT_00aa6cfc = FUN_004d48e0();
  }
  return iVar5;
}
