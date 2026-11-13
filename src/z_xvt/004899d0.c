/* SPDX-License-Identifier: MIT */
/* z_xvt 004899d0 */
typedef unsigned int undefined4;

int __cdecl __strcmpi(const char *s1, const char *s2);
undefined4 * __cdecl FUN_0048e520(undefined4 *param_1, char *param_2);
void __cdecl FUN_004899d0(int param_1, undefined4 *param_2, int param_3, undefined4 param_4);

void __cdecl FUN_004899d0(int param_1, undefined4 *param_2, int param_3, undefined4 param_4)
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
  int _p3_6;
  int _p3_10;

  if (param_2 != (undefined4 *)0x0) {
    __builtin_memcpy(&_p3_6, (char *)param_3 + 6, 4);
    __builtin_memcpy(&_p3_10, (char *)param_3 + 10, 4);
    
    while (param_2[1] == 7) {
      _Str2 = (char *)param_2[5];
      local_4 = 0;
      if (0 < _p3_6) {
        local_8 = 0;
        do {
          puVar2 = *(undefined4 **)(_p3_10 + local_8);
          if (puVar2 == (undefined4 *)0x0) {
            puVar2 = (undefined4 *)0x0;
          }
          else if (((char *)*puVar2 == (char *)0x0) ||
                  (iVar1 = __strcmpi((char *)*puVar2, _Str2), iVar1 != 0)) {
            iVar1 = 0;
            iVar5 = 0;
            if (0 < (int)puVar2[2]) {
              do {
                param_2 = FUN_0048e520(*(undefined4 **)(puVar2[3] + iVar1), _Str2);
                if (param_2 != (undefined4 *)0x0) goto LAB_00489a7c;
                iVar1 = iVar1 + 4;
                iVar5 = iVar5 + 1;
              } while (iVar5 < (int)puVar2[2]);
            }
            puVar2 = (undefined4 *)0x0;
          }
          param_2 = puVar2;
          if (puVar2 != (undefined4 *)0x0) goto LAB_00489a7c;
          local_8 = local_8 + 4;
          local_4 = local_4 + 1;
        } while (local_4 < _p3_6);
      }
      param_2 = (undefined4 *)0x0;
LAB_00489a7c:
      if (param_2 == (undefined4 *)0x0) {
        return;
      }
    }
    if (param_2[1] == 0xd) {
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
              if ((*pfVar4 == *pfVar3) && (pfVar4[1] == pfVar3[1])) break;
              pfVar4 = pfVar4 + 2;
              iVar5 = iVar5 + 1;
            } while (iVar5 < iVar1);
            bVar6 = iVar1 == iVar5;
          }
          if (bVar6) {
            *pfVar4 = *pfVar3;
            pfVar4[1] = pfVar3[1];
            *(int *)(param_1 + 0x10) = *(int *)(param_1 + 0x10) + 1;
          }
          pfVar3 = pfVar3 + 2;
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
        FUN_004899d0(param_1, *(undefined4 **)(param_2[3] + -4 + iVar1), param_3, param_4);
      } while (iVar5 < (int)param_2[2]);
    }
  }
  return;
}
