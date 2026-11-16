/* SPDX-License-Identifier: MIT */
/* z_bop 00479510 */
extern int DAT_0060f1c0;
extern int DAT_0060f1e8;
extern int DAT_005271d0;
extern int DAT_0060f1d4;
extern int DAT_0060f204;
extern float _DAT_005181b8;
extern float _DAT_005181c0;

double __cdecl sqrt(double);
void __cdecl FUN_00479ce0(float *param_1, int param_2, int param_3);

/* vertex (stride 0xc) and uv (stride 8) accessors; index math wraps on unsigned */
#define VTX(i, off) (*(float *)((unsigned int)DAT_0060f1c0 + (unsigned int)(i) * 0xc + (off)))
#define VTXP(i)     ((float *)((unsigned int)DAT_0060f1c0 + (unsigned int)(i) * 0xc))
#define UV(i, off)  (*(float *)((unsigned int)DAT_0060f1e8 + (unsigned int)(i) * 8 + (off)))
#define UVP(i)      ((float *)((unsigned int)DAT_0060f1e8 + (unsigned int)(i) * 8))
#define FABS_(x)    (((x) < 0.0f) ? -(x) : (x))

void __cdecl FUN_00479510(float *param_1, int param_2, int param_3, int param_4)
{
  int iVar1;
  int iVar2;
  int iVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float *pfVar10;
  int *piVar11;
  int *piVar12;
  float *pfVar13;
  int iVar14;
  float fStack_1c;
  float fStack_18;
  float fStack_14;
  float fStack_10;
  float fStack_c;
  float fStack_8;
  int iStack_4;

  if (DAT_0060f1c0 != 0) {
    piVar11 = (int *)(param_2 + 4);
    piVar12 = piVar11;
    iVar14 = param_3;
    if (0 < param_3) {
      do {
        iVar2 = piVar12[1];
        iVar3 = *piVar12;
        fVar6 = VTX(iVar2, 8) - VTX(iVar3, 8);
        fVar4 = VTX(iVar2, 0) - VTX(iVar3, 0);
        fVar9 = VTX(iVar2, 4) - VTX(iVar3, 4);
        fVar5 = VTX(iVar2, 0) - VTX(piVar12[2], 0);
        fVar7 = VTX(iVar2, 4) - VTX(piVar12[2], 4);
        fVar8 = VTX(iVar2, 8) - VTX(piVar12[2], 8);
        *param_1 = fVar8 * fVar9 - fVar7 * fVar6;
        param_1[1] = fVar5 * fVar6 - fVar8 * fVar4;
        param_1[2] = fVar7 * fVar4 - fVar5 * fVar9;
        fVar4 = param_1[1] * param_1[1] + *param_1 * *param_1 + param_1[2] * param_1[2];
        if (fVar4 == _DAT_005181b8) {
          iVar1 = piVar12[3];
          if (iVar1 != -1) {
            iVar3 = *piVar12;
            fVar6 = VTX(iVar1, 8) - VTX(iVar3, 8);
            fVar4 = VTX(iVar1, 0) - VTX(iVar3, 0);
            fVar9 = VTX(iVar1, 4) - VTX(iVar3, 4);
            fVar5 = VTX(iVar1, 0) - VTX(piVar12[2], 0);
            fVar7 = VTX(iVar1, 4) - VTX(piVar12[2], 4);
            fVar8 = VTX(iVar1, 8) - VTX(piVar12[2], 8);
            *param_1 = fVar8 * fVar9 - fVar7 * fVar6;
            param_1[1] = fVar5 * fVar6 - fVar8 * fVar4;
            param_1[2] = fVar7 * fVar4 - fVar5 * fVar9;
            fVar4 = param_1[1] * param_1[1] + *param_1 * *param_1 + param_1[2] * param_1[2];
            if (fVar4 != _DAT_005181b8) goto LAB_004796b9;
          }
        }
        else {
LAB_004796b9:
          fVar4 = _DAT_005181c0 / (float)sqrt((double)fVar4);
          *param_1 = *param_1 * fVar4;
          param_1[1] = param_1[1] * fVar4;
          param_1[2] = fVar4 * param_1[2];
        }
        if (DAT_005271d0 != 0) {
          *param_1 = -*param_1;
          param_1[1] = -param_1[1];
          param_1[2] = -param_1[2];
        }
        param_1 = param_1 + 3;
        iVar14 = iVar14 + -1;
        piVar12 = piVar12 + 0x10;
      } while (iVar14 != 0);
    }
    if (DAT_0060f1e8 == 0) {
      param_1 = param_1 + (int)((unsigned int)param_3 * 0x18);
    }
    else if (0 < param_3) {
      iStack_4 = param_3;
      do {
        iVar14 = *piVar11;
        iVar1 = piVar11[1];
        fVar4 = VTX(iVar14, 0) - VTX(iVar1, 0);
        fVar7 = VTX(iVar14, 4) - VTX(iVar1, 4);
        fVar6 = VTX(iVar14, 8) - VTX(iVar1, 8);
        iVar1 = piVar11[8];
        pfVar10 = VTXP(piVar11[2]);
        fStack_1c = VTX(iVar14, 8) - pfVar10[2];
        fStack_c = VTX(iVar14, 0) - *pfVar10;
        fStack_8 = VTX(iVar14, 4) - pfVar10[1];
        fStack_18 = UV(iVar1, 0) - UV(piVar11[9], 0);
        fVar5 = UV(iVar1, 4) - UV(piVar11[9], 4);
        pfVar10 = UVP(piVar11[10]);
        fVar8 = UV(iVar1, 4) - pfVar10[1];
        fStack_14 = UV(iVar1, 0) - *pfVar10;
        fStack_10 = fStack_14 * fVar5 - fStack_18 * fVar8;
        *param_1 = fStack_c * fVar5 - fVar8 * fVar4;
        param_1[1] = fStack_8 * fVar5 - fVar8 * fVar7;
        param_1[2] = fVar5 * fStack_1c - fVar8 * fVar6;
        if ((FABS_(fStack_10) == 0.0f) ||
            (param_1[1] * param_1[1] + *param_1 * *param_1 + param_1[2] * param_1[2] == _DAT_005181b8)) {
          if (piVar11[3] == -1) {
LAB_00479bcb:
            fStack_14 = 1.0f;
            fStack_10 = 1.0f;
            *param_1 = fStack_c;
            fStack_18 = 0.0f;
            param_1[1] = fStack_8;
            param_1[2] = fStack_1c;
          }
          else {
            iVar1 = *piVar11;
            pfVar10 = VTXP(piVar11[3]);
            fStack_c = VTX(iVar1, 0) - *pfVar10;
            fStack_8 = VTX(iVar1, 4) - pfVar10[1];
            pfVar13 = VTXP(piVar11[1]);
            fVar7 = VTX(iVar1, 4) - pfVar13[1];
            fVar4 = VTX(iVar1, 0) - *pfVar13;
            fVar6 = VTX(iVar1, 8) - pfVar13[2];
            fStack_1c = VTX(iVar1, 8) - pfVar10[2];
            iVar14 = piVar11[8];
            fStack_18 = UV(iVar14, 0) - UV(piVar11[9], 0);
            fVar5 = UV(iVar14, 4) - UV(piVar11[9], 4);
            pfVar10 = UVP(piVar11[0xb]);
            fVar8 = UV(iVar14, 4) - pfVar10[1];
            fStack_14 = UV(iVar14, 0) - *pfVar10;
            fStack_10 = fStack_14 * fVar5 - fStack_18 * fVar8;
            *param_1 = fStack_c * fVar5 - fVar8 * fVar4;
            param_1[1] = fStack_8 * fVar5 - fVar8 * fVar7;
            param_1[2] = fVar5 * fStack_1c - fVar8 * fVar6;
            if ((FABS_(fStack_10) == 0.0f) ||
                (param_1[1] * param_1[1] + *param_1 * *param_1 + param_1[2] * param_1[2] ==
                 _DAT_005181b8)) {
              iVar1 = *piVar11;
              iVar2 = piVar11[2];
              fVar7 = VTX(iVar1, 4) - VTX(iVar2, 4);
              fVar4 = VTX(iVar1, 0) - VTX(iVar2, 0);
              fVar6 = VTX(iVar1, 8) - VTX(iVar2, 8);
              iVar2 = piVar11[8];
              pfVar10 = VTXP(piVar11[3]);
              fStack_c = VTX(iVar1, 0) - *pfVar10;
              fStack_8 = VTX(iVar1, 4) - pfVar10[1];
              fStack_1c = VTX(iVar1, 8) - pfVar10[2];
              fStack_18 = UV(iVar2, 0) - UV(piVar11[10], 0);
              fVar5 = UV(iVar2, 4) - UV(piVar11[10], 4);
              pfVar10 = UVP(piVar11[0xb]);
              fVar8 = UV(iVar2, 4) - pfVar10[1];
              fStack_14 = UV(iVar2, 0) - *pfVar10;
              fStack_10 = fStack_14 * fVar5 - fStack_18 * fVar8;
              *param_1 = fStack_c * fVar5 - fVar8 * fVar4;
              param_1[1] = fStack_8 * fVar5 - fVar8 * fVar7;
              param_1[2] = fVar5 * fStack_1c - fVar8 * fVar6;
              if ((FABS_(fStack_10) == 0.0f) ||
                  (param_1[1] * param_1[1] + *param_1 * *param_1 + param_1[2] * param_1[2] ==
                   _DAT_005181b8)) {
                iVar14 = piVar11[1];
                iVar1 = piVar11[2];
                fVar4 = VTX(iVar14, 0) - VTX(iVar1, 0);
                fVar7 = VTX(iVar14, 4) - VTX(iVar1, 4);
                fVar6 = VTX(iVar14, 8) - VTX(iVar1, 8);
                pfVar13 = VTXP(piVar11[3]);
                pfVar10 = UVP(piVar11[9]);
                fStack_1c = VTX(iVar14, 8) - pfVar13[2];
                fStack_c = VTX(iVar14, 0) - *pfVar13;
                fStack_8 = VTX(iVar14, 4) - pfVar13[1];
                fVar5 = pfVar10[1] - UV(piVar11[10], 4);
                fStack_18 = *pfVar10 - UV(piVar11[10], 0);
                pfVar13 = UVP(piVar11[0xb]);
                fStack_14 = *pfVar10 - *pfVar13;
                fVar8 = pfVar10[1] - pfVar13[1];
                fStack_10 = fStack_14 * fVar5 - fStack_18 * fVar8;
                *param_1 = fStack_c * fVar5 - fVar8 * fVar4;
                param_1[1] = fStack_8 * fVar5 - fVar8 * fVar7;
                param_1[2] = fVar5 * fStack_1c - fVar8 * fVar6;
                if ((FABS_(fStack_10) == 0.0f) ||
                    (param_1[1] * param_1[1] + *param_1 * *param_1 + param_1[2] * param_1[2] ==
                     _DAT_005181b8)) goto LAB_00479bcb;
              }
            }
          }
        }
        fStack_10 = _DAT_005181c0 / fStack_10;
        *param_1 = *param_1 * fStack_10;
        param_1[1] = param_1[1] * fStack_10;
        fVar5 = -fStack_10;
        param_1[2] = param_1[2] * fStack_10;
        param_1[3] = (fStack_18 * fStack_c - fStack_14 * fVar4) * fVar5;
        param_1[4] = (fStack_18 * fStack_8 - fStack_14 * fVar7) * fVar5;
        param_1[5] = fVar5 * (fStack_18 * fStack_1c - fStack_14 * fVar6);
        if ((FABS_(fStack_18) == 0.0f) && (FABS_(fStack_14) == 0.0f)) {
          param_1[4] = 1.0f;
        }
        param_1 = param_1 + 6;
        piVar11 = piVar11 + 0x10;
        iStack_4 = iStack_4 + -1;
      } while (iStack_4 != 0);
    }
    if (*(int *)(param_4 + 0x84) != 0) {
      DAT_0060f1d4 = 0;
      return;
    }
    DAT_0060f1d4 = DAT_0060f204;
    FUN_00479ce0(param_1, param_2, param_3);
  }
  return;
}
