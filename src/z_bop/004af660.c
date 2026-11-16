/* SPDX-License-Identifier: MIT */
/* z_bop 004af660 */
int * FUN_004ac720(int param_1);
void FUN_00471ff0(int *param_1);
int * FUN_004ae1a0(int param_1, int *param_2);
void FUN_004ac740(void);

extern unsigned char DAT_00521f19[];
extern short DAT_009a7ed0[];

int __cdecl FUN_004af660(int param_1, int param_2, int param_3, int param_4)
{
  int iVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  int *piVar6;
  int *piVar7;
  float *pfVar7;
  int iVar8;
  float local_1c;
  int local_18;
  float local_14;
  float local_4;
  int local_20;
  int local_24;
  int local_28;

  fVar2 = (float)param_2;
  fVar3 = (float)param_3;
  fVar4 = (float)param_4;
  local_4 = 2.1474836e+09f;
  if ((DAT_00521f19[(int)((unsigned int)param_1 * 0x1aU)] & 1) == 0) {
    return 0;
  }
  piVar6 = FUN_004ac720((int)((((unsigned int)param_1 * 5u) >> 0x10) << 0x10 |
                              (unsigned int)(unsigned short)DAT_009a7ed0[param_1]));
  if ((int *)*piVar6 != piVar6) {
    FUN_00471ff0(piVar6);
  }
  iVar8 = 0;
  __builtin_memcpy(&local_20, (char *)piVar6 + 6, 4);
  if (0 < local_20) {
    do {
      __builtin_memcpy(&local_24, (char *)piVar6 + 10, 4);
      iVar1 = *(int *)(unsigned int)((unsigned int)local_24 + (unsigned int)iVar8 * 4U);
      if ((*(int *)(unsigned int)((unsigned int)iVar1 + 4U) != 0x14) &&
          ((piVar7 = FUN_004ae1a0(iVar1, piVar6)), piVar7 != (int *)0x0) && (*piVar7 == 1)) {
        pfVar7 = (float *)piVar7;
        if (fVar2 <= pfVar7[0xb]) {
          if (pfVar7[8] <= fVar2) {
            local_1c = 0.0f;
          }
          else {
            local_1c = pfVar7[8] - fVar2;
          }
        }
        else {
          local_1c = fVar2 - pfVar7[0xb];
        }
        fVar5 = local_1c;
        local_14 = local_1c;
        if (fVar3 <= pfVar7[0xc]) {
          if (pfVar7[9] <= fVar3) {
            local_1c = 0.0f;
          }
          else {
            local_1c = pfVar7[9] - fVar3;
          }
        }
        else {
          local_1c = fVar3 - pfVar7[0xc];
        }
        if (fVar5 < local_1c) {
          local_14 = local_1c;
        }
        if (fVar4 <= pfVar7[0xd]) {
          if (pfVar7[10] <= fVar4) {
            local_1c = 0.0f;
          }
          else {
            local_1c = pfVar7[10] - fVar4;
          }
        }
        else {
          local_1c = fVar4 - pfVar7[0xd];
        }
        if (local_14 < local_1c) {
          local_14 = local_1c;
        }
        if ((local_14 < local_4) && (local_4 = local_14, local_18 = iVar8, local_14 == 0.0f))
          break;
      }
      iVar8 = (int)((unsigned int)iVar8 + 1U);
      __builtin_memcpy(&local_20, (char *)piVar6 + 6, 4);
    } while (iVar8 < local_20);
  }
  __builtin_memcpy(&local_24, (char *)piVar6 + 10, 4);
  local_28 = *(int *)(unsigned int)local_24;
  if (*(int *)(unsigned int)((unsigned int)local_28 + 4U) == 0x14) {
    local_18 = (int)((unsigned int)local_18 - 1U);
  }
  FUN_004ac740();
  return local_18;
}
