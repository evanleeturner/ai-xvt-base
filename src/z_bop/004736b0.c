/* SPDX-License-Identifier: MIT */
/* z_bop 004736b0 */
extern float _DAT_005181c0;
extern unsigned int DAT_0060f1c0;
extern unsigned int DAT_0060f1e8;
extern float *_DAT_0060f208;
extern unsigned int _DAT_0060f1f0;
extern unsigned int DAT_0060f1d8;
extern unsigned int DAT_0060f204;

unsigned int *__cdecl FUN_00479de0(int param_1, char *param_2);
int __cdecl FUN_00473ad0(int param_1, float *param_2, float *param_3);
void __cdecl FUN_004210a0(float *param_1, float *param_2);
void __cdecl FUN_00420fe0(float *param_1, float *param_2);
void __cdecl FUN_004211d0(float *param_1, float *param_2);
long double __cdecl FUN_00421040(float *param_1, float *param_2);
long double __cdecl FUN_00421060(float *param_1, int param_2);
long double __cdecl FUN_00421080(float *param_1, int param_2);

unsigned int __cdecl
FUN_004736b0(int param_1, unsigned int *param_2, unsigned int *param_3, float *param_4, float *param_5)
{
  float *pfVar1;
  float *pfVar2;
  float fVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  float *pfVar7;
  unsigned int *puVar8;
  long double fVar9;
  unsigned int local_d0[52];

  if (param_2 == (unsigned int *)0x0) {
    return 0;
  }
  while (param_2[1] == 7) {
    param_2 = FUN_00479de0(param_1, (char *)param_2[5]);
    if (param_2 == (unsigned int *)0x0) {
      return 0;
    }
  }
  pfVar2 = (float *)param_2[5];
  if (pfVar2 != (float *)0x0) {
    switch (param_2[1]) {
    case 1:
    case 0xf:
    case 0x10:
    case 0x11:
      pfVar7 = pfVar2 + 1;
      param_3[0x23] = param_2[4];
      param_3[0x24] = *(unsigned int *)pfVar2;
      param_3[0x28] = (unsigned int)pfVar7;
      pfVar7 = pfVar7 + (param_2[4] * 0x10u);
      param_3[0x25] = (unsigned int)pfVar7;
      iVar6 = (int)param_2[4];
      param_3[0x26] = (unsigned int)(pfVar7 + (unsigned int)iVar6 * 3u);
      if (param_3[0x21] == 0) {
        param_3[0x21] = (unsigned int)(pfVar7 + (unsigned int)iVar6 * 3u + param_2[4] * 6u);
        iVar6 = FUN_00473ad0((int)param_3, param_4, param_5);
        if (iVar6 != 0) {
          return 1;
        }
        param_3[0x21] = 0;
      } else {
        iVar6 = FUN_00473ad0((int)param_3, param_4, param_5);
        if (iVar6 != 0) {
          return 1;
        }
      }
      break;
    case 2:
      pfVar7 = (float *)(param_3 + 2);
      FUN_004210a0((float *)(param_3 + 5), pfVar2 + 3);
      FUN_00420fe0(pfVar7, pfVar2 + 3);
      *pfVar7 = *pfVar7 + *pfVar2;
      pfVar7 = (float *)(param_3 + 0x11);
      ((float *)param_3)[3] = pfVar2[1] + ((float *)param_3)[3];
      ((float *)param_3)[4] = pfVar2[2] + ((float *)param_3)[4];
      FUN_004211d0(pfVar7, pfVar2 + 3);
      fVar9 = FUN_00421040(pfVar2, pfVar7);
      ((float *)param_3)[0xe] = (float)((long double)((float *)param_3)[0xe] - fVar9);
      fVar9 = FUN_00421060(pfVar2, (int)pfVar7);
      ((float *)param_3)[0xf] = (float)((long double)((float *)param_3)[0xf] - fVar9);
      fVar9 = FUN_00421080(pfVar2, (int)pfVar7);
      ((float *)param_3)[0x10] = (float)((long double)((float *)param_3)[0x10] - fVar9);
      break;
    case 3:
      param_3[0x1e] = param_2[4];
      param_3[0x1f] = (unsigned int)pfVar2;
      break;
    case 4:
      ((float *)param_3)[2] = ((float *)param_3)[2] + *pfVar2;
      pfVar7 = (float *)(param_3 + 0x11);
      ((float *)param_3)[3] = pfVar2[1] + ((float *)param_3)[3];
      ((float *)param_3)[4] = pfVar2[2] + ((float *)param_3)[4];
      fVar9 = FUN_00421040(pfVar2, pfVar7);
      ((float *)param_3)[0xe] = (float)((long double)((float *)param_3)[0xe] - fVar9);
      fVar9 = FUN_00421060(pfVar2, (int)pfVar7);
      ((float *)param_3)[0xf] = (float)((long double)((float *)param_3)[0xf] - fVar9);
      fVar9 = FUN_00421080(pfVar2, (int)pfVar7);
      ((float *)param_3)[0x10] = (float)((long double)((float *)param_3)[0x10] - fVar9);
      break;
    case 5:
      FUN_004210a0((float *)(param_3 + 5), pfVar2);
      FUN_00420fe0((float *)(param_3 + 2), pfVar2);
      FUN_004211d0((float *)(param_3 + 0x11), pfVar2);
      break;
    case 6:
      pfVar7 = pfVar2 + 1;
      pfVar1 = pfVar2 + 2;
      ((float *)param_3)[5] = ((float *)param_3)[5] * *pfVar2;
      ((float *)param_3)[6] = *pfVar7 * ((float *)param_3)[6];
      ((float *)param_3)[7] = *pfVar1 * ((float *)param_3)[7];
      ((float *)param_3)[8] = ((float *)param_3)[8] * *pfVar2;
      ((float *)param_3)[9] = *pfVar7 * ((float *)param_3)[9];
      ((float *)param_3)[10] = *pfVar1 * ((float *)param_3)[10];
      ((float *)param_3)[0xb] = ((float *)param_3)[0xb] * *pfVar2;
      ((float *)param_3)[0xc] = *pfVar7 * ((float *)param_3)[0xc];
      ((float *)param_3)[0xd] = *pfVar1 * ((float *)param_3)[0xd];
      ((float *)param_3)[2] = ((float *)param_3)[2] * *pfVar2;
      ((float *)param_3)[3] = *pfVar7 * ((float *)param_3)[3];
      ((float *)param_3)[4] = *pfVar1 * ((float *)param_3)[4];
      fVar3 = _DAT_005181c0 / *pfVar2;
      ((float *)param_3)[0x11] = ((float *)param_3)[0x11] * fVar3;
      ((float *)param_3)[0x12] = ((float *)param_3)[0x12] * fVar3;
      ((float *)param_3)[0x13] = fVar3 * ((float *)param_3)[0x13];
      fVar3 = _DAT_005181c0 / *pfVar7;
      ((float *)param_3)[0x14] = ((float *)param_3)[0x14] * fVar3;
      ((float *)param_3)[0x15] = ((float *)param_3)[0x15] * fVar3;
      ((float *)param_3)[0x16] = fVar3 * ((float *)param_3)[0x16];
      fVar3 = _DAT_005181c0 / *pfVar1;
      ((float *)param_3)[0x17] = ((float *)param_3)[0x17] * fVar3;
      ((float *)param_3)[0x18] = ((float *)param_3)[0x18] * fVar3;
      ((float *)param_3)[0x19] = fVar3 * ((float *)param_3)[0x19];
      break;
    default:
      break;
    case 0xb:
      _DAT_0060f208 = pfVar2;
      param_3[0x21] = (unsigned int)pfVar2;
    }
  }
  iVar6 = 0;
  if (param_2[2] != 0) {
    puVar8 = local_d0;
    for (iVar5 = 0x34; iVar5 != 0; iVar5 = iVar5 + -1) {
      *puVar8 = *param_3;
      param_3 = param_3 + 1;
      puVar8 = puVar8 + 1;
    }
    DAT_0060f1c0 = 0;
    DAT_0060f1e8 = 0;
    iVar5 = 0;
    _DAT_0060f208 = (float *)0x0;
    _DAT_0060f1f0 = 0;
    DAT_0060f1d8 = 0;
    DAT_0060f204 = 0;
    if (0 < (int)param_2[2]) {
      do {
        iVar4 = FUN_004736b0(param_1, *(unsigned int **)(param_2[3] + iVar6), local_d0, param_4, param_5);
        if (iVar4 != 0) {
          return 1;
        }
        iVar6 = iVar6 + 4;
        iVar5 = iVar5 + 1;
      } while (iVar5 < (int)param_2[2]);
    }
  }
  return 0;
}
