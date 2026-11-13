/* SPDX-License-Identifier: MIT */
/* z_bop 004deca0 */
extern int DAT_00aa7fb7;
extern char DAT_00aa7b0d;
extern int DAT_00aa7fc3;
extern int DAT_00aa6cfc;
extern int unaff_ESI;

extern int __cdecl FUN_004df1b0(char *param_1);
extern void __stdcall FUN_004d4970(void);
extern int __stdcall FUN_004d48e0(void);

int __cdecl FUN_004deca0(char *param_1)
{
  char cVar1;
  int iVar2;
  int *piVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  char local_4[4];
  
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
  iVar4 = 0;
  iVar5 = -1;
  piVar3 = (int *)(DAT_00aa7fc3 + 4);
  do {
    if ((piVar3[-1] == iVar2) && (iVar6 < *piVar3)) {
      iVar5 = iVar4;
      iVar6 = *piVar3;
    }
    piVar3 = piVar3 + 3;
    iVar4 = iVar4 + 1;
  } while (iVar4 < 0xc);
  if (iVar5 == -1) {
    return 0;
  }
  FUN_004d4970();
  piVar3 = *(int **)(DAT_00aa7fc3 + 8 + iVar5 * 0xc);
  iVar2 = (**(int (__stdcall **)(int *, char *))(*piVar3 + 0x18))(piVar3, local_4);
  if (iVar2 != 0) {
    return 0;
  }
  if (cVar1 != '\0') {
    DAT_00aa6cfc = FUN_004d48e0();
  }
  return (unaff_ESI * 0x7f) / 2000 + 0x7f;
}
