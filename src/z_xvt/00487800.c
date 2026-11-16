/* SPDX-License-Identifier: MIT */
/* z_xvt 00487800 */
extern float _DAT_005201dc;
extern int DAT_00602678;
extern int DAT_006026a0;
extern int _DAT_006026a8;
extern int DAT_00602690;
extern int DAT_006026bc;
extern float *_DAT_006026c0;

int __cdecl __strcmpi(const char *, const char *);
undefined4 *__cdecl FUN_0048e520(undefined4 *, char *);
int __cdecl FUN_00487cb0(int, float *, float *);
void __cdecl FUN_004150a0(float *, float *);
void __cdecl FUN_00414fe0(float *, float *);
void __cdecl FUN_004151d0(float *, float *);
double __cdecl FUN_00415040(float *, float *);
double __cdecl FUN_00415060(float *, int);
double __cdecl FUN_00415080(float *, int);
undefined4 __cdecl FUN_00487800(int param_1, undefined4 *param_2, undefined4 *param_3, float *param_4, float *param_5);

undefined4 __cdecl
FUN_00487800(int param_1, undefined4 *param_2, undefined4 *param_3, float *param_4, float *param_5)
{
  float *pfVar1;
  char *_Str2;
  float *pfVar2;
  float fVar3;
  int iVar4;
  undefined4 *puVar5;
  int iVar6;
  int iVar7;
  float *pfVar8;
  double fVar9;
  int local_d8;
  int local_d4;
  undefined4 local_d0[52];
  float *f3;
  int cnt;
  int base;

  if (param_2 == (undefined4 *)0x0) {
    return 0;
  }
  while (param_2[1] == 7) {
    _Str2 = (char *)param_2[5];
    local_d4 = 0;
    __builtin_memcpy(&cnt, (char *)param_1 + 6, 4);
    if (0 < cnt) {
      local_d8 = 0;
      do {
        __builtin_memcpy(&base, (char *)param_1 + 10, 4);
        puVar5 = *(undefined4 **)(base + local_d8);
        if (puVar5 == (undefined4 *)0x0) {
LAB_0048789a:
          puVar5 = (undefined4 *)0x0;
        }
        else if (((char *)*puVar5 == (char *)0x0) ||
                 (iVar4 = __strcmpi((char *)*puVar5, _Str2), iVar4 != 0)) {
          iVar4 = 0;
          iVar7 = 0;
          if (0 < (int)puVar5[2]) {
            do {
              param_2 = FUN_0048e520(*(undefined4 **)(puVar5[3] + iVar4), _Str2);
              if (param_2 != (undefined4 *)0x0) goto LAB_004878c4;
              iVar4 = iVar4 + 4;
              iVar7 = iVar7 + 1;
            } while (iVar7 < (int)puVar5[2]);
          }
          goto LAB_0048789a;
        }
        param_2 = puVar5;
        if (puVar5 != (undefined4 *)0x0) goto LAB_004878c4;
        local_d8 = local_d8 + 4;
        local_d4 = local_d4 + 1;
        __builtin_memcpy(&cnt, (char *)param_1 + 6, 4);
      } while (local_d4 < cnt);
    }
    param_2 = (undefined4 *)0x0;
LAB_004878c4:
    if (param_2 == (undefined4 *)0x0) {
      return 0;
    }
  }
  f3 = (float *)param_3;
  pfVar2 = (float *)param_2[5];
  if (pfVar2 != (float *)0x0) {
    switch (param_2[1]) {
    case 1:
    case 0xf:
    case 0x10:
    case 0x11:
      pfVar8 = pfVar2 + 1;
      param_3[0x23] = param_2[4];
      param_3[0x24] = *(undefined4 *)pfVar2;
      param_3[0x28] = (undefined4)pfVar8;
      pfVar8 = pfVar8 + (int)(param_2[4] * 0x10u);
      param_3[0x25] = (undefined4)pfVar8;
      iVar4 = (int)param_2[4];
      param_3[0x26] = (undefined4)(pfVar8 + (int)((unsigned)iVar4 * 3u));
      if (param_3[0x21] == 0) {
        param_3[0x21] = (undefined4)(pfVar8 + (int)((unsigned)iVar4 * 3u) + (int)(param_2[4] * 6u));
        iVar4 = FUN_00487cb0((int)param_3, param_4, param_5);
        if (iVar4 != 0) {
          return 1;
        }
        param_3[0x21] = 0;
      }
      else {
        iVar4 = FUN_00487cb0((int)param_3, param_4, param_5);
        if (iVar4 != 0) {
          return 1;
        }
      }
      break;
    case 2:
      pfVar8 = (float *)(param_3 + 2);
      FUN_004150a0((float *)(param_3 + 5), pfVar2 + 3);
      FUN_00414fe0(pfVar8, pfVar2 + 3);
      *pfVar8 = *pfVar2 + *pfVar8;
      pfVar8 = (float *)(param_3 + 0x11);
      f3[3] = pfVar2[1] + f3[3];
      f3[4] = pfVar2[2] + f3[4];
      FUN_004151d0(pfVar8, pfVar2 + 3);
      fVar9 = FUN_00415040(pfVar2, pfVar8);
      f3[0xe] = (float)((double)f3[0xe] - fVar9);
      fVar9 = FUN_00415060(pfVar2, (int)pfVar8);
      f3[0xf] = (float)((double)f3[0xf] - fVar9);
      fVar9 = FUN_00415080(pfVar2, (int)pfVar8);
      f3[0x10] = (float)((double)f3[0x10] - fVar9);
      break;
    case 3:
      param_3[0x1e] = param_2[4];
      param_3[0x1f] = (undefined4)pfVar2;
      break;
    case 4:
      f3[2] = f3[2] + *pfVar2;
      pfVar8 = (float *)(param_3 + 0x11);
      f3[3] = f3[3] + pfVar2[1];
      f3[4] = f3[4] + pfVar2[2];
      fVar9 = FUN_00415040(pfVar2, pfVar8);
      f3[0xe] = (float)((double)f3[0xe] - fVar9);
      fVar9 = FUN_00415060(pfVar2, (int)pfVar8);
      f3[0xf] = (float)((double)f3[0xf] - fVar9);
      fVar9 = FUN_00415080(pfVar2, (int)pfVar8);
      f3[0x10] = (float)((double)f3[0x10] - fVar9);
      break;
    case 5:
      FUN_004150a0((float *)(param_3 + 5), pfVar2);
      FUN_00414fe0((float *)(param_3 + 2), pfVar2);
      FUN_004151d0((float *)(param_3 + 0x11), pfVar2);
      break;
    case 6:
      pfVar8 = pfVar2 + 1;
      pfVar1 = pfVar2 + 2;
      f3[5] = f3[5] * *pfVar2;
      f3[6] = f3[6] * *pfVar8;
      f3[7] = f3[7] * *pfVar1;
      f3[8] = f3[8] * *pfVar2;
      f3[9] = f3[9] * *pfVar8;
      f3[10] = f3[10] * *pfVar1;
      f3[0xb] = f3[0xb] * *pfVar2;
      f3[0xc] = *pfVar8 * f3[0xc];
      f3[0xd] = *pfVar1 * f3[0xd];
      f3[2] = f3[2] * *pfVar2;
      f3[3] = f3[3] * *pfVar8;
      f3[4] = f3[4] * *pfVar1;
      fVar3 = _DAT_005201dc / *pfVar2;
      f3[0x11] = f3[0x11] * fVar3;
      f3[0x12] = f3[0x12] * fVar3;
      f3[0x13] = fVar3 * f3[0x13];
      fVar3 = _DAT_005201dc / *pfVar8;
      f3[0x14] = f3[0x14] * fVar3;
      f3[0x15] = f3[0x15] * fVar3;
      f3[0x16] = fVar3 * f3[0x16];
      fVar3 = _DAT_005201dc / *pfVar1;
      f3[0x17] = f3[0x17] * fVar3;
      f3[0x18] = f3[0x18] * fVar3;
      f3[0x19] = fVar3 * f3[0x19];
      break;
    default:
      break;
    case 0xb:
      _DAT_006026c0 = pfVar2;
      param_3[0x21] = (undefined4)pfVar2;
    }
  }
  iVar4 = 0;
  if (param_2[2] != 0) {
    puVar5 = local_d0;
    for (iVar7 = 0x34; iVar7 != 0; iVar7 = iVar7 + -1) {
      *puVar5 = *param_3;
      param_3 = param_3 + 1;
      puVar5 = puVar5 + 1;
    }
    DAT_00602678 = 0;
    DAT_006026a0 = 0;
    iVar7 = 0;
    _DAT_006026c0 = (float *)0x0;
    _DAT_006026a8 = 0;
    DAT_00602690 = 0;
    DAT_006026bc = 0;
    if (0 < (int)param_2[2]) {
      do {
        iVar6 = FUN_00487800(param_1, *(undefined4 **)(param_2[3] + iVar4), local_d0, param_4, param_5);
        if (iVar6 != 0) {
          return 1;
        }
        iVar4 = iVar4 + 4;
        iVar7 = iVar7 + 1;
      } while (iVar7 < (int)param_2[2]);
    }
  }
  return 0;
}
