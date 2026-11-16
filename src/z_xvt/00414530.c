/* SPDX-License-Identifier: MIT */
/* z_xvt 00414530 */
extern int    DAT_00523c2c;
extern int    DAT_00523c30;
extern int   *DAT_00523c34;
extern int    DAT_005254fc;
extern int    DAT_00525730;
extern float  DAT_00536930[];
extern float  DAT_00536960[];
extern float  _DAT_00536950;
extern float  _DAT_00536954;
extern float  _DAT_00536958;
extern float  DAT_005369c0;
extern float  DAT_005369c4;
extern float  DAT_005369c8;
extern float  DAT_0072f460[];
extern float  _DAT_00520058;
extern float  _DAT_0052005c;
extern float  _DAT_00520064;
extern float  _DAT_0052006c;
extern float  _DAT_00520080;
extern float  _DAT_00520084;
extern float  _DAT_00520088;
extern float  _DAT_0052008c;
extern float  _DAT_00520090;
extern float  _DAT_00520094;
extern float  _DAT_006a8a08;
extern unsigned short DAT_006d4984;
extern unsigned short DAT_007091ae;
extern int    DAT_00709ffc;
extern float  _DAT_00732f34;
extern float  _DAT_00732ea0;
extern float  DAT_00732f30;

void __cdecl FUN_0045b420(int param_1);

long double __cdecl FUN_00414530(int *param_1, int param_2, int param_3, float param_4)
{
  float fVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  int *piVar7;
  float *pfVar7;
  int iVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  float *pfVar12;
  int iVar13;
  float *pfVar14;
  float *pfVar15;
  float local_44;
  float local_40;
  float local_3c;
  float local_38;
  float local_4;
  unsigned int uBits;
  int iBits;
  float *pfTable;

  piVar7 = (int *)param_1[1];
  if (piVar7 == (int *)0x0) {
    return (long double)_DAT_00520064;
  }
  pfVar7 = (float *)piVar7;
  if (DAT_00523c2c != *piVar7) {
    FUN_0045b420(*piVar7);
    iVar13 = 0;
    DAT_00523c30 = DAT_00525730;
    DAT_00523c2c = *piVar7;
    if (0 < DAT_00525730) {
      pfVar14 = DAT_00536930;
      pfVar12 = DAT_0072f460;
      pfVar15 = DAT_00536960;
      while (1) {
        iVar13 = (int)((unsigned int)iVar13 + 1u);
        *pfVar15 = pfVar12[0];
        pfVar15[1] = pfVar12[1];
        pfVar15[2] = pfVar12[2];
        fVar1 = *pfVar15;
        fVar2 = pfVar15[1];
        fVar3 = pfVar15[2];
        *pfVar15 = pfVar7[5] * fVar1 + pfVar7[8] * fVar2 + pfVar7[0xb] * fVar3;
        pfVar15[1] = pfVar7[6] * fVar1 +
                     pfVar7[9] * fVar2 + pfVar7[0xc] * fVar3;
        pfVar15[2] = fVar3 * pfVar7[0xd] + pfVar7[10] * fVar2 +
                     pfVar7[7] * fVar1;
        *pfVar15 = *pfVar15 + pfVar7[2];
        pfVar15[1] = pfVar15[1] + pfVar7[3];
        pfVar15[2] = pfVar15[2] + pfVar7[4];
        if (DAT_00525730 <= iVar13) break;
        *pfVar14 = pfVar12[3];
        pfVar14 = pfVar14 + 1;
        pfVar12 = pfVar12 + 4;
        pfVar15 = pfVar15 + 3;
      }
      *pfVar14 = pfVar12[3];
    }
    fVar1 = _DAT_00732f34 * _DAT_0052005c;
    fVar2 = _DAT_00732ea0 * _DAT_0052005c;
    fVar3 = DAT_00732f30 * _DAT_0052005c;
    _DAT_00536950 = pfVar7[5] * fVar3 + pfVar7[0xb] * fVar2 + pfVar7[8] * fVar1;
    _DAT_00536954 = pfVar7[0xc] * fVar2 + pfVar7[6] * fVar3 + pfVar7[9] * fVar1;
    _DAT_00536958 =
         pfVar7[10] * fVar1 + fVar3 * pfVar7[7] + pfVar7[0xd] * fVar2;
  }
  fVar1 = _DAT_00520058 / param_4;
  fVar2 = (float)(int)((unsigned int)param_2 - (unsigned int)(DAT_007091ae >> 1)) * fVar1 * _DAT_006a8a08;
  fVar3 = (float)(int)(((unsigned int)param_3 - (unsigned int)(DAT_006d4984 >> 1)) -
                       (unsigned int)DAT_00709ffc) * fVar1 * _DAT_006a8a08;
  if (DAT_00523c34 != param_1) {
    DAT_00523c34 = param_1;
    pfTable = (float *)piVar7[0x25];
    iVar8 = *param_1;
    fVar4 = ((float *)((char *)pfTable + (unsigned int)iVar8 * 0xcu))[0];
    fVar5 = ((float *)((char *)pfTable + (unsigned int)iVar8 * 0xcu))[1];
    fVar6 = ((float *)((char *)pfTable + (unsigned int)iVar8 * 0xcu))[2];
    DAT_005369c0 = pfVar7[5] * fVar4 + pfVar7[0xb] * fVar6 + pfVar7[8] * fVar5;
    DAT_005369c4 = pfVar7[6] * fVar4 + pfVar7[0xc] * fVar6 + pfVar7[9] * fVar5;
    DAT_005369c8 = fVar6 * pfVar7[0xd] + pfVar7[10] * fVar5 + pfVar7[7] * fVar4;
  }
  local_4 = 0.0f;
  if (DAT_005254fc != 0) {
    fVar4 = -fVar2;
    fVar5 = -fVar3;
    fVar6 = -fVar1;
    local_3c = fVar4;
    __builtin_memcpy(&uBits, &fVar4, 4);
    if (0x80000000u < uBits) {
      local_3c = -fVar4;
    }
    local_40 = fVar5;
    __builtin_memcpy(&uBits, &fVar5, 4);
    if (0x80000000u < uBits) {
      local_40 = -fVar5;
    }
    local_44 = fVar6;
    __builtin_memcpy(&uBits, &fVar6, 4);
    if (0x80000000u < uBits) {
      local_44 = -fVar6;
    }
    if ((local_3c < local_40) || (local_3c < local_44)) {
      if ((local_40 < local_3c) || (local_40 < local_44)) {
        fVar9 = (local_40 + local_3c) * _DAT_0052008c;
        local_3c = local_44;
      }
      else {
        fVar9 = (local_44 + local_3c) * _DAT_0052008c;
        local_3c = local_40;
      }
    }
    else {
      fVar9 = (local_44 + local_40) * _DAT_0052008c;
    }
    fVar9 = _DAT_00520058 / (local_3c * _DAT_00520090 + fVar9);
    local_38 = (_DAT_00536958 + fVar9 * fVar6) * DAT_005369c8 +
               (_DAT_00536954 + fVar5 * fVar9) * DAT_005369c4 +
               (_DAT_00536950 + fVar4 * fVar9) * DAT_005369c0;
    if (local_38 <= _DAT_00520064) {
      local_38 = 0.0f;
    }
    else {
      local_38 = local_38 * _DAT_00520080;
      local_38 = local_38 * local_38 * local_38;
      local_38 = local_38 * local_38;
      local_38 = local_38 * local_38;
      local_38 = local_38 * local_38;
      local_38 = local_38 * local_38;
    }
    local_38 = local_38 * _DAT_00520094;
    if (_DAT_00520064 < local_38) {
      local_4 = local_38;
      __builtin_memcpy(&iBits, &local_38, 4);
      if (0x3f7fffff < iBits) {
        return (long double)_DAT_00520058;
      }
    }
  }
  pfVar14 = DAT_00536960;
  iVar13 = 0;
  if (0 < DAT_00523c30) {
    pfVar15 = DAT_00536930;
    do {
      fVar4 = *pfVar14 - fVar2;
      fVar5 = pfVar14[1] - fVar3;
      fVar6 = pfVar14[2] - fVar1;
      fVar9 = fVar6 * DAT_005369c8 + fVar5 * DAT_005369c4 + fVar4 * DAT_005369c0;
      if (_DAT_00520064 < fVar9) {
        local_3c = fVar4;
        __builtin_memcpy(&uBits, &fVar4, 4);
        if (0x80000000u < uBits) {
          local_3c = -fVar4;
        }
        local_40 = fVar5;
        __builtin_memcpy(&uBits, &fVar5, 4);
        if (0x80000000u < uBits) {
          local_40 = -fVar5;
        }
        local_44 = fVar6;
        __builtin_memcpy(&uBits, &fVar6, 4);
        if (0x80000000u < uBits) {
          local_44 = -fVar6;
        }
        if ((local_3c < local_40) || (local_3c < local_44)) {
          if ((local_40 < local_3c) || (local_40 < local_44)) {
            fVar10 = (local_40 + local_3c) * _DAT_0052006c + local_44;
          }
          else {
            fVar10 = (local_44 + local_3c) * _DAT_0052006c + local_40;
          }
        }
        else {
          fVar10 = (local_44 + local_40) * _DAT_0052006c + local_3c;
        }
        if (DAT_005254fc == 0) {
LAB_00414c93:
          local_38 = 0.0f;
        }
        else {
          local_3c = fVar4 - fVar2;
          local_40 = fVar5 - fVar3;
          local_44 = fVar6 - fVar1;
          fVar4 = local_3c * DAT_005369c0;
          fVar6 = local_40 * DAT_005369c4;
          fVar5 = local_44 * DAT_005369c8;
          __builtin_memcpy(&uBits, &local_3c, 4);
          if (0x80000000u < uBits) {
            local_3c = -local_3c;
          }
          __builtin_memcpy(&uBits, &local_40, 4);
          if (0x80000000u < uBits) {
            local_40 = -local_40;
          }
          __builtin_memcpy(&uBits, &local_44, 4);
          if (0x80000000u < uBits) {
            local_44 = -local_44;
          }
          if ((local_3c < local_40) || (local_3c < local_44)) {
            if ((local_40 < local_3c) || (local_40 < local_44)) {
              fVar11 = (local_40 + local_3c) * _DAT_00520084;
              local_3c = local_44;
            }
            else {
              fVar11 = (local_44 + local_3c) * _DAT_00520084;
              local_3c = local_40;
            }
          }
          else {
            fVar11 = (local_44 + local_40) * _DAT_00520084;
          }
          local_38 = _DAT_00520058 / (local_3c * _DAT_00520088 + fVar11);
          fVar4 = local_38 * (fVar5 + fVar6 + fVar4) * _DAT_00520080;
          if (fVar4 < _DAT_00520064) goto LAB_00414c93;
          fVar4 = fVar4 * fVar4 * fVar4;
          fVar4 = fVar4 * fVar4;
          fVar4 = fVar4 * fVar4;
          fVar4 = fVar4 * fVar4;
          local_38 = fVar4 * fVar4 * local_38;
        }
        local_38 = local_38 + fVar9 / (fVar10 * fVar10);
        if (_DAT_00520064 < local_38) {
          local_4 = *pfVar15 * local_38 + local_4;
          if (_DAT_00520058 <= local_4) {
            local_4 = 1.0f;
            break;
          }
        }
      }
      pfVar15 = pfVar15 + 1;
      iVar13 = (int)((unsigned int)iVar13 + 1u);
      pfVar14 = pfVar14 + 3;
    } while (iVar13 < DAT_00523c30);
  }
  return (long double)local_4;
}
