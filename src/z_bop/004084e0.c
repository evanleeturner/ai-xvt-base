/* SPDX-License-Identifier: MIT */
/* z_bop 004084e0 */
/* Globals; types inferred from the decompile and P-code sizes. */
extern unsigned char DAT_00999462[4];   /* int, unaligned (0x999462) */
extern unsigned char DAT_0099944a[4];   /* int, unaligned (0x99944a) */
extern int DAT_0099945c;
extern int DAT_00999478;
extern float DAT_00a081ec;              /* UNCONFIRMED: assumed float (4 bytes; Ghidra prints (float)DAT_00a081ec). Not verified against the binary. */
extern unsigned short DAT_009d682c;
extern unsigned short DAT_009a1fe4;
extern int DAT_009d7680;
extern float _DAT_00518060;
extern float _DAT_00518054;
extern float _DAT_00518074;
extern float _DAT_00518078;
extern float _DAT_0099940c;

void FUN_00420db0(int param_1, float *param_2, int param_3);
void FUN_00420fe0(float *param_1, float *param_2);
void FUN_004201f0(int *param_1, int param_2, float *param_3, float *param_4);

#define U(x) ((unsigned int)(x))
/* float view of the int-typed record piVar10 (aliasing, as in the original) */
#define F10(k) (((float *)piVar10)[k])

void __cdecl FUN_004084e0(int *param_1)
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
  float local_50;
  float local_48;
  float local_44;
  float local_40;
  float local_3c;
  int local_38;
  float local_34;   /* int bits first, float later; reinterpreted via memcpy (int view in iVar4) */
  float local_30;
  float local_2c;
  float local_28;
  int local_24;
  int local_20;
  int local_1c;
  float local_18;
  float local_14;
  float local_10;
  float local_c;
  float local_8;
  float local_4;

  __builtin_memcpy(&iVar5, DAT_00999462, 4);
  __builtin_memcpy(&iVar4, DAT_0099944a, 4);   /* temporarily holds DAT_0099944a */
  piVar10 = (int *)(U(param_1[0x2e]) * 0x70u + U(iVar4));
  param_1[0x2f] = iVar5;
  iVar6 = DAT_0099945c;
  iVar4 = 0;
  param_1[0x32] = 0;
  pfVar8 = (float *)(U(iVar6) + U(iVar5) * 0x18u);
  iVar6 = 0;
  if (0 < param_1[0x1e]) {
    do {
      iVar4 = (int)(U(iVar4) + 4u);
      iVar6 = (int)(U(iVar6) + 1u);
      *(int *)(U(DAT_00999478) + 0xFFFFFFFCu + U(iVar4)) = -1;
    } while (iVar6 < param_1[0x1e]);
  }
  local_20 = 0;
  if (0 < param_1[0x31]) {
    do {
      FUN_00420db0((int)piVar10, (float *)(U(param_1[0x26]) + U(*piVar10) * 0x18u), (int)(param_1 + 2));
      iVar6 = param_1[0x28];
      piVar10[0x12] = 0;
      local_3c = 0.0f;
      local_24 = 0;
      F10(0x13) = DAT_00a081ec;
      piVar3 = (int *)(U(*piVar10) * 0x40u + U(iVar6));
      do {
        piVar9 = piVar3;
        iVar6 = *piVar9;
        local_38 = piVar9[8];
        __builtin_memcpy(&local_34, &piVar9[0xc], 4);
        if (iVar6 == -1) break;
        iVar5 = *(int *)(U(DAT_00999478) + U(iVar6) * 4u);
        if (iVar5 == -1) {
          *(int *)(U(DAT_00999478) + U(iVar6) * 4u) = param_1[0x32];
          local_1c = (int)(U(iVar6) * 0xcu);
          param_1[0x32] = (int)(U(param_1[0x32]) + 1u);
          pfVar7 = (float *)(U(param_1[0x1f]) + U(local_1c));
          local_48 = *pfVar7;
          local_44 = pfVar7[1];
          local_40 = pfVar7[2];
          FUN_00420fe0(&local_48, (float *)(param_1 + 5));
          local_48 = *(float *)&param_1[2] + local_48;
          local_44 = *(float *)&param_1[3] + local_44;
          local_40 = *(float *)&param_1[4] + local_40;
          if (_DAT_00518060 <= local_40) {
            pfVar8[2] = DAT_00a081ec / local_40;
            *pfVar8 = pfVar8[2] * local_48;
            pfVar8[1] = pfVar8[2] * local_44;
            *pfVar8 = (float)(int)(DAT_009d682c >> 1) + *pfVar8;
            pfVar8[1] = (float)(int)((unsigned int)(DAT_009a1fe4 >> 1) + (unsigned int)DAT_009d7680) + pfVar8[1];
            local_50 = pfVar8[2];
          }
          else {
            pfVar8[2] = local_40 - _DAT_00518060;
            *pfVar8 = local_48;
            pfVar8[1] = local_44;
            piVar10[3] = -1;
            local_50 = DAT_00a081ec;
          }
          __builtin_memcpy(&iVar4, &local_34, 4);   /* (int)local_34 as index */
          FUN_004201f0(param_1, (int)pfVar8, (float *)(U(param_1[0x21]) + U(iVar4) * 0xcu),
                       (float *)(U(param_1[0x1f]) + U(local_1c)));
          pfVar8[4] = *(float *)(U(param_1[0x20]) + U(local_38) * 8u);
          pfVar8[5] = *(float *)(U(param_1[0x20]) + 4u + U(local_38) * 8u);
          pfVar8 = pfVar8 + 6;
        }
        else {
          iVar5 = (int)(U(param_1[0x2f]) + U(iVar5));
          if (*(unsigned int *)(U(DAT_0099945c) + 8u + U(iVar5) * 0x18u) < 0x80000001u) {
            local_50 = *(float *)(U(DAT_0099945c) + U(iVar5) * 0x18u + 8u);
          }
          else {
            piVar10[3] = -1;
            local_50 = DAT_00a081ec;
          }
        }
        local_3c = local_3c + local_50;
        if (F10(0x12) < local_50) {
          F10(0x12) = local_50;
        }
        if (local_50 < F10(0x13)) {
          F10(0x13) = local_50;
        }
        local_24 = (int)(U(local_24) + 1u);
        piVar3 = piVar9 + 1;
      } while (local_24 < 4);
      if (param_1[0x20] != 0) {
        local_38 = piVar9[5];
        pfVar7 = (float *)(U(param_1[0x1f]) + U(piVar9[-3]) * 0xcu);
        local_48 = *pfVar7;
        local_44 = pfVar7[1];
        local_40 = pfVar7[2];
        FUN_00420fe0(&local_48, (float *)(param_1 + 5));
        local_48 = *(float *)&param_1[2] + local_48;
        local_44 = *(float *)&param_1[3] + local_44;
        local_40 = *(float *)&param_1[4] + local_40;
        F10(10) = (local_48 -
                   *(float *)(U(param_1[0x20]) + 4u + U(local_38) * 8u) * F10(7)) -
                  *(float *)(U(param_1[0x20]) + U(local_38) * 8u) * F10(4);
        F10(0xb) = (local_44 -
                    *(float *)(U(param_1[0x20]) + 4u + U(local_38) * 8u) * F10(8)) -
                   *(float *)(U(param_1[0x20]) + U(local_38) * 8u) * F10(5);
        F10(0xc) = (local_40 -
                    *(float *)(U(param_1[0x20]) + 4u + U(local_38) * 8u) * F10(9)) -
                   *(float *)(U(param_1[0x20]) + U(local_38) * 8u) * F10(6);
        local_18 = F10(0xc) * F10(8) - F10(9) * F10(0xb);
        local_14 = F10(9) * F10(10) - F10(0xc) * F10(7);
        local_10 = F10(0xb) * F10(7) - F10(10) * F10(8);
        local_c = F10(6) * F10(0xb) - F10(0xc) * F10(5);
        local_8 = F10(0xc) * F10(4) - F10(10) * F10(6);
        local_4 = F10(5) * F10(10) - F10(0xb) * F10(4);
        local_30 = F10(9) * F10(5) - F10(6) * F10(8);
        local_2c = F10(6) * F10(7) - F10(9) * F10(4);
        local_28 = F10(8) * F10(4) - F10(5) * F10(7);
        if ((((local_30 < 0.0f ? -local_30 : local_30) == 0.0f) &&
             ((local_2c < 0.0f ? -local_2c : local_2c) == 0.0f)) &&
            ((local_28 < 0.0f ? -local_28 : local_28) == 0.0f)) {
          local_28 = 1.0f;
        }
        fVar1 = _DAT_00518060 /
                (F10(10) * local_30 +
                 F10(0xb) * local_2c + F10(0xc) * local_28);
        fVar2 = _DAT_0099940c * fVar1;
        F10(4) = local_18 * fVar2;
        F10(5) = local_14 * fVar2;
        F10(6) = local_10 * fVar1;
        F10(7) = local_c * fVar2;
        F10(8) = local_8 * fVar2;
        F10(9) = local_4 * fVar1;
        F10(10) = local_30 * fVar2;
        F10(0xb) = fVar2 * local_2c;
        F10(0xc) = fVar1 * local_28;
        F10(6) = F10(6) - (float)(int)(DAT_009d682c >> 1) * F10(4);
        F10(6) = F10(6) -
                 (float)(int)((unsigned int)(DAT_009a1fe4 >> 1) + (unsigned int)DAT_009d7680) * F10(5);
        F10(9) = F10(9) - (float)(int)(DAT_009d682c >> 1) * F10(7);
        F10(9) = F10(9) -
                 (float)(int)((unsigned int)(DAT_009a1fe4 >> 1) + (unsigned int)DAT_009d7680) * F10(8);
        F10(0xc) = F10(0xc) - (float)(int)(DAT_009d682c >> 1) * F10(10);
        F10(0xc) = F10(0xc) -
                   (float)(int)((unsigned int)(DAT_009a1fe4 >> 1) + (unsigned int)DAT_009d7680) *
                   F10(0xb);
        local_34 = F10(8) * F10(4) - F10(5) * F10(7);
        if (local_34 < _DAT_00518054) {
          local_34 = -local_34;
        }
        fVar1 = _DAT_00518078;
        if (*piVar9 == -1) {
          fVar1 = _DAT_00518074;
        }
        local_3c = fVar1 / local_3c;
        piVar10[0x1b] = (int)local_3c;   /* __ftol: truncating float-to-int, low 32 bits stored */
      }
      piVar10 = piVar10 + 0x1c;
      local_20 = (int)(U(local_20) + 1u);
    } while (local_20 < param_1[0x31]);
  }
  __builtin_memcpy(&iVar5, DAT_00999462, 4);
  iVar5 = (int)(U(iVar5) + U(param_1[0x32]));
  __builtin_memcpy(DAT_00999462, &iVar5, 4);
  return;
}
