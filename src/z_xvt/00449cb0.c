/* SPDX-License-Identifier: MIT */
/* z_xvt 00449cb0 */
extern int FUN_0048f4f0(int param_1);
extern void FUN_00485fa0(int *param_1);
extern int FUN_00449670(int param_1, int *param_2);
extern long long __ftol(void);
extern void FUN_0048f510(int param_1);
extern short DAT_006da850[];

int __cdecl FUN_00449cb0(int param_1, int param_2)
{
  int iVar1;
  int *piVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  long long lVar6;

  if (param_2 < 0) {
    return 0;
  }
  piVar2 = (int *)FUN_0048f4f0(
    ((unsigned int)param_1 & 0xFFFF0000) | (unsigned short)DAT_006da850[param_1]
  );
  if ((int *)*piVar2 != piVar2) {
    FUN_00485fa0(piVar2);
  }
  
  int *ptr_a;
  __builtin_memcpy(&ptr_a, (char *)piVar2 + 10, 4);
  if (*(int *)((char *)ptr_a + 4) == 0x14) {
    param_2 = param_2 + 1;
  }
  
  int field_6;
  __builtin_memcpy(&field_6, (char *)piVar2 + 6, 4);
  if (field_6 <= param_2) {
    param_2 = field_6 - 1;
  }
  
  iVar1 = ptr_a[param_2];
  if ((iVar1 != 0) && (*(int *)((char *)iVar1 + 4) != 0x19)) {
    iVar5 = 0;
    iVar4 = 0;
    if (0 < *(int *)((char *)iVar1 + 8)) {
      do {
        iVar3 = *(int *)((char *)*(int *)((char *)iVar1 + 0xc) + iVar5);
        if ((iVar3 != 0) && (iVar3 = FUN_00449670(iVar3, piVar2), iVar3 != 0)) break;
        iVar5 = iVar5 + 4;
        iVar4 = iVar4 + 1;
      } while (iVar4 < *(int *)((char *)iVar1 + 8));
    }
  }
  lVar6 = __ftol();
  FUN_0048f510(iVar1);
  return (int)lVar6;
}
