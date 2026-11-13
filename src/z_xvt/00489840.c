/* SPDX-License-Identifier: MIT */
/* z_xvt 00489840 */
undefined4 * __cdecl FUN_0048e520(undefined4 * param_1, char * param_2);
int __cdecl __strcmpi(const char * s1, const char * s2);

void __cdecl FUN_00489840(int param_1, undefined4 * param_2, int param_3, undefined4 param_4)
{
  char *_Str2;
  int iVar1;
  undefined4 *puVar2;
  float *pfVar3;
  float *pfVar4;
  int iVar5;
  int bVar6;
  int local_8;
  int local_4;

  if (param_2 != (undefined4 *)0x0) {
    while (param_2[1] == 7) {
      _Str2 = (char *)param_2[5];
      local_4 = 0;
      if (0 < *(int *)(param_3 + 6)) {
        local_8 = 0;
        do {
          puVar2 = *(undefined4 **)(*(int *)(param_3 + 10) + local_8);
          if (puVar2 == (undefined4 *)0x0) {
            puVar2 = (undefined4 *)0x0;
          }
          else if (((char *)*puVar2 == (char *)0x0) ||
                  (iVar1 = __strcmpi((char *)*puVar2, _Str2), iVar1 != 0)) {
            iVar5 = 0;
            iVar1 = 0;
            if (0 < (int)puVar2[2]) {
              do {
                param_2 = FUN_0048e520(*(undefined4 **)(puVar2[3] + iVar5), _Str2);
                if (param_2 != (undefined4 *)0x0) goto LAB_004898ec;
                iVar5 = iVar5 + 4;
                iVar1 = iVar1 + 1;
              } while (iVar1 < (int)puVar2[2]);
            }
            puVar2 = (undefined4 *)0x0;
          }
          param_2 = puVar2;
          if (puVar2 != (undefined4 *)0x0) goto LAB_004898ec;
          local_8 = local_8 + 4;
          local_4 = local_4 + 1;
        } while (local_4 < *(int *)(param_3 + 6));
      }
      param_2 = (undefined4 *)0x0;
LAB_004898ec:
      if (param_2 == (undefined4 *)0x0) {
        return;
      }
    }
    if (param_2[1] == 3) {
      pfVar3 = (float *)param_2[5];
      local_8 = 0;
      if (0 < (int)param_2[4]) {
        do {
          pfVar4 = *(float **)(param_1 + 0x14);
          iVar5 = 0;
          iVar1 = *(int *)(param_1 + 0x10);
          bVar6 = iVar1 == 0;
          if (0 < iVar1) {
            do {
              if (((*pfVar3 == *pfVar4) && (pfVar3[1] == pfVar4[1])) && (pfVar3[2] == pfVar4[2]))
                break;
              pfVar4 = pfVar4 + 3;
              iVar5 = iVar5 + 1;
            } while (iVar5 < iVar1);
            bVar6 = iVar5 == iVar1;
          }
          if (bVar6) {
            *pfVar4 = *pfVar3;
            pfVar4[1] = pfVar3[1];
            pfVar4[2] = pfVar3[2];
            *(int *)(param_1 + 0x10) = *(int *)(param_1 + 0x10) + 1;
          }
          pfVar3 = pfVar3 + 3;
          local_8 = local_8 + 1;
        } while (local_8 < (int)param_2[4]);
      }
    }
    iVar1 = 0;
    iVar5 = 0;
    if (0 < (int)param_2[2]) {
      do {
        iVar1 = iVar1 + 4;
        iVar5 = iVar5 + 1;
        FUN_00489840(param_1, *(undefined4 **)(param_2[3] + -4 + iVar1), param_3, param_4);
      } while (iVar5 < (int)param_2[2]);
    }
  }
  return;
}
