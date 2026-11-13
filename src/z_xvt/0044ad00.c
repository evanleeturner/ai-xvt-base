/* SPDX-License-Identifier: MIT */
/* z_xvt 0044ad00 */
extern int DAT_006da850[];
int __cdecl FUN_0048f4f0(int param_1);
void __cdecl FUN_00485fa0(int *param_1);
int __cdecl FUN_00449670(int param_1, int param_2);
void __cdecl FUN_0048f510(void);

int __cdecl FUN_0044ad00(int param_1)
{
  int *piVar3;
  int *piVar4;
  int local_8;
  int local_4;
  int field_6;
  int field_10;
  int iVar1;
  int iVar2;
  int iVar5;
  int iVar6;

  piVar3 = (int *)FUN_0048f4f0(((unsigned int)param_1 & 0xFFFF0000) | (DAT_006da850[param_1] & 0xFFFF));
  if (*piVar3 != (int)piVar3) {
    FUN_00485fa0(piVar3);
  }
  local_4 = 0;
  __builtin_memcpy(&field_6, (char *)piVar3 + 6, 4);
  if (0 < field_6) {
    local_8 = 0;
    __builtin_memcpy(&field_10, (char *)piVar3 + 10, 4);
    do {
      iVar1 = *(int *)(field_10 + local_8);
      if ((iVar1 != 0) && (*(int *)(iVar1 + 4) != 0x14)) {
        if (iVar1 == 0) {
LAB_0044ad8a:
          piVar4 = 0;
        }
        else {
          if (*(int *)(iVar1 + 4) != 0x19) {
            iVar6 = 0;
            iVar5 = 0;
            if (0 < *(int *)(iVar1 + 8)) {
              do {
                iVar2 = *(int *)(*(int *)(iVar1 + 0xc) + iVar6);
                if (iVar2 != 0) {
                  piVar4 = (int *)FUN_00449670(iVar2, (int)piVar3);
                  if (piVar4 != 0) goto LAB_0044ad90;
                }
                iVar6 += 4;
                iVar5 += 1;
              } while (iVar5 < *(int *)(iVar1 + 8));
            }
            goto LAB_0044ad8a;
          }
          piVar4 = *(int **)(iVar1 + 0x14);
        }
        if (piVar4 != 0) {
LAB_0044ad90:
          if (*piVar4 == 3) {
            FUN_0048f510();
            return 1;
          }
        }
      }
      local_8 += 4;
      local_4 += 1;
    } while (local_4 < field_6);
  }
  FUN_0048f510();
  return 0;
}
