/* SPDX-License-Identifier: MIT */
/* z_xvt 0045c910 */
/* Prototypes for every callee */
void __cdecl FUN_00414d00(int param_1, unsigned int *param_2, int param_3);
void __cdecl FUN_00414fe0(float *param_1, float *param_2);
void __cdecl FUN_00414070(int *param_1, int param_2, float *param_3, float *param_4, float *param_5);

/* Globals (types inferred from usage) */
extern int DAT_006a885e;            /* 0x6a885e: unaligned, accessed via memcpy */
extern int DAT_006a884a;            /* 0x6a884a: unaligned, accessed via memcpy */
extern int DAT_006a8858;
extern int DAT_006a8874;
extern float DAT_00732f3c;
extern float _DAT_00520194;
extern float _DAT_00520190;
extern float _DAT_005201ac;
extern float _DAT_005201a8;
extern float _DAT_006a8a08;
extern float DAT_006a88b0;
extern unsigned short DAT_007091ae;
extern unsigned short DAT_006d4984;
extern int DAT_00709ffc;

void __cdecl FUN_0045c910(int *param_1)
{
  float fVar1;
  float fVar2;
  int *piVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  float *pfVar7;
  float *pfVar8;
  int *piVar9;
  int *piVar10;
  float *pf10;
  float *pf1;
  float local_50;
  float local_48;
  float local_44;
  float local_40;
  float local_3c;
  int local_38;
  int local_34i;
  float local_34;
  float fStack_30;
  float fStack_2c;
  float fStack_28;
  int local_24;
  int local_20;
  int local_1c;
  float fStack_18;
  float fStack_14;
  float fStack_10;
  float fStack_c;
  float fStack_8;
  float fStack_4;
  int tmp_884a;
  int tmp_885e;
  unsigned int uOff;

  pf1 = (float *)param_1;

  __builtin_memcpy(&tmp_885e, &DAT_006a885e, 4);
  __builtin_memcpy(&tmp_884a, &DAT_006a884a, 4);

  iVar5 = tmp_885e;
  piVar10 = (int *)((unsigned int)param_1[0x2e] * 0x70u + (unsigned int)tmp_884a);
  pf10 = (float *)piVar10;
  param_1[0x2f] = tmp_885e;
  iVar6 = DAT_006a8858;
  iVar4 = 0;
  param_1[0x32] = 0;
  pfVar8 = (float *)((unsigned int)iVar6 + (unsigned int)iVar5 * 0x18u);
  iVar6 = 0;
  if (0 < param_1[0x1e]) {
    do {
      iVar4 = (int)((unsigned int)iVar4 + 4u);
      iVar6 = (int)((unsigned int)iVar6 + 1u);
      *(int *)((unsigned int)DAT_006a8874 - 4u + (unsigned int)iVar4) = -1;
    } while (iVar6 < param_1[0x1e]);
  }
  local_20 = 0;
  if (0 < param_1[0x31]) {
    do {
      FUN_00414d00((int)piVar10,
                   (unsigned int *)((unsigned int)param_1[0x26] + (unsigned int)*piVar10 * 0x18u),
                   (int)(param_1 + 2));
      iVar6 = param_1[0x28];
      piVar10[0x12] = 0;
      local_3c = 0.0f;
      local_24 = 0;
      pf10[0x13] = DAT_00732f3c;
      piVar3 = (int *)((unsigned int)*piVar10 * 0x40u + (unsigned int)iVar6);
      do {
        piVar9 = piVar3;
        iVar6 = *piVar9;
        local_38 = piVar9[8];
        local_34i = piVar9[0xc];
        if (iVar6 == -1) break;
        iVar5 = *(int *)((unsigned int)DAT_006a8874 + (unsigned int)iVar6 * 4u);
        if (iVar5 == -1) {
          *(int *)((unsigned int)DAT_006a8874 + (unsigned int)iVar6 * 4u) = param_1[0x32];
          local_1c = (int)((unsigned int)iVar6 * 0xcu);
          param_1[0x32] = (int)((unsigned int)param_1[0x32] + 1u);
          pfVar7 = (float *)((unsigned int)param_1[0x1f] + (unsigned int)local_1c);
          local_48 = *pfVar7;
          local_44 = pfVar7[1];
          local_40 = pfVar7[2];
          FUN_00414fe0(&local_48, (float *)(param_1 + 5));
          local_48 = pf1[2] + local_48;
          local_44 = pf1[3] + local_44;
          local_40 = pf1[4] + local_40;
          if (_DAT_00520194 <= local_40) {
            pfVar8[2] = DAT_00732f3c / local_40;
            *pfVar8 = pfVar8[2] * local_48;
            pfVar8[1] = pfVar8[2] * local_44;
            *pfVar8 = (float)(DAT_007091ae >> 1) + *pfVar8;
            pfVar8[1] = (float)(int)((unsigned int)(DAT_006d4984 >> 1) + (unsigned int)DAT_00709ffc) + pfVar8[1];
            local_50 = pfVar8[2];
          }
          else {
            pfVar8[2] = local_40 - _DAT_00520194;
            *pfVar8 = local_48;
            pfVar8[1] = local_44;
            piVar10[3] = -1;
            local_50 = DAT_00732f3c;
          }
          FUN_00414070(param_1, (int)pfVar8,
                       (float *)((unsigned int)param_1[0x21] + (unsigned int)local_34i * 0xcu),
                       (float *)((unsigned int)param_1[0x1f] + (unsigned int)local_1c),
                       &DAT_006a88b0);
          uOff = (unsigned int)local_38 * 8u;
          pfVar8[4] = *(float *)((unsigned int)param_1[0x20] + uOff);
          pfVar8[5] = *(float *)((unsigned int)param_1[0x20] + 4u + uOff);
          pfVar8 = pfVar8 + 6;
        }
        else {
          iVar5 = (int)((unsigned int)param_1[0x2f] + (unsigned int)iVar5);
          if (*(unsigned int *)((unsigned int)DAT_006a8858 + 8u + (unsigned int)iVar5 * 0x18u) < 0x80000001u) {
            local_50 = *(float *)((unsigned int)DAT_006a8858 + (unsigned int)iVar5 * 0x18u + 8u);
          }
          else {
            piVar10[3] = -1;
            local_50 = DAT_00732f3c;
          }
        }
        local_3c = local_3c + local_50;
        if (pf10[0x12] < local_50) {
          pf10[0x12] = local_50;
        }
        if (local_50 < pf10[0x13]) {
          pf10[0x13] = local_50;
        }
        local_24 = (int)((unsigned int)local_24 + 1u);
        piVar3 = piVar9 + 1;
      } while (local_24 < 4);
      if (param_1[0x20] != 0) {
        local_38 = piVar9[5];
        pfVar7 = (float *)((unsigned int)param_1[0x1f] + (unsigned int)piVar9[-3] * 0xcu);
        local_48 = *pfVar7;
        local_44 = pfVar7[1];
        local_40 = pfVar7[2];
        FUN_00414fe0(&local_48, (float *)(param_1 + 5));
        local_48 = pf1[2] + local_48;
        local_44 = pf1[3] + local_44;
        local_40 = pf1[4] + local_40;
        uOff = (unsigned int)local_38 * 8u;
        pf10[10] = (local_48 -
                    *(float *)((unsigned int)param_1[0x20] + 4u + uOff) * pf10[7]) -
                   *(float *)((unsigned int)param_1[0x20] + uOff) * pf10[4];
        pf10[0xb] = (local_44 -
                     *(float *)((unsigned int)param_1[0x20] + 4u + uOff) * pf10[8]) -
                    *(float *)((unsigned int)param_1[0x20] + uOff) * pf10[5];
        pf10[0xc] = (local_40 -
                     *(float *)((unsigned int)param_1[0x20] + 4u + uOff) * pf10[9]) -
                    *(float *)((unsigned int)param_1[0x20] + uOff) * pf10[6];
        fStack_18 = pf10[0xc] * pf10[8] - pf10[9] * pf10[0xb];
        fStack_14 = pf10[9] * pf10[10] - pf10[0xc] * pf10[7];
        fStack_10 = pf10[0xb] * pf10[7] - pf10[10] * pf10[8];
        fStack_c = pf10[6] * pf10[0xb] - pf10[0xc] * pf10[5];
        fStack_8 = pf10[0xc] * pf10[4] - pf10[10] * pf10[6];
        fStack_4 = pf10[5] * pf10[10] - pf10[0xb] * pf10[4];
        fStack_30 = pf10[9] * pf10[5] - pf10[6] * pf10[8];
        fStack_2c = pf10[6] * pf10[7] - pf10[9] * pf10[4];
        fStack_28 = pf10[8] * pf10[4] - pf10[5] * pf10[7];
        if (((fStack_30 < 0.0f ? -fStack_30 : fStack_30) == 0.0f) &&
            ((fStack_2c < 0.0f ? -fStack_2c : fStack_2c) == 0.0f) &&
            ((fStack_28 < 0.0f ? -fStack_28 : fStack_28) == 0.0f)) {
          fStack_28 = 1.0f;
        }
        fVar1 = _DAT_00520194 /
                (pf10[0xb] * fStack_2c +
                 pf10[10] * fStack_30 + pf10[0xc] * fStack_28);
        fVar2 = _DAT_006a8a08 * fVar1;
        pf10[4] = fStack_18 * fVar2;
        pf10[5] = fStack_14 * fVar2;
        pf10[6] = fStack_10 * fVar1;
        pf10[7] = fStack_c * fVar2;
        pf10[8] = fStack_8 * fVar2;
        pf10[9] = fStack_4 * fVar1;
        pf10[10] = fStack_30 * fVar2;
        pf10[0xb] = fVar2 * fStack_2c;
        pf10[0xc] = fVar1 * fStack_28;
        pf10[6] = pf10[6] - (float)(DAT_007091ae >> 1) * pf10[4];
        pf10[6] = pf10[6] -
                  (float)(int)((unsigned int)(DAT_006d4984 >> 1) + (unsigned int)DAT_00709ffc) * pf10[5];
        pf10[9] = pf10[9] - (float)(DAT_007091ae >> 1) * pf10[7];
        pf10[9] = pf10[9] -
                  (float)(int)((unsigned int)(DAT_006d4984 >> 1) + (unsigned int)DAT_00709ffc) * pf10[8];
        pf10[0xc] = pf10[0xc] - (float)(DAT_007091ae >> 1) * pf10[10];
        pf10[0xc] = pf10[0xc] -
                    (float)(int)((unsigned int)(DAT_006d4984 >> 1) + (unsigned int)DAT_00709ffc) *
                    pf10[0xb];
        local_34 = pf10[8] * pf10[4] - pf10[5] * pf10[7];
        if (local_34 < _DAT_00520190) {
          local_34 = -local_34;
        }
        fVar1 = _DAT_005201ac;
        if (*piVar9 == -1) {
          fVar1 = _DAT_005201a8;
        }
        local_3c = fVar1 / local_3c;
        piVar10[0x1b] = (int)local_3c;   /* __ftol */
      }
      piVar10 = piVar10 + 0x1c;
      pf10 = (float *)piVar10;
      local_20 = (int)((unsigned int)local_20 + 1u);
    } while (local_20 < param_1[0x31]);
  }
  __builtin_memcpy(&tmp_885e, &DAT_006a885e, 4);
  tmp_885e = (int)((unsigned int)tmp_885e + (unsigned int)param_1[0x32]);
  __builtin_memcpy(&DAT_006a885e, &tmp_885e, 4);
  return;
}
