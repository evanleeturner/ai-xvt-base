/* SPDX-License-Identifier: MIT */
/* z_xvt 0048dba0 */
extern int DAT_00602678;
extern int DAT_006026a0;
extern int DAT_0052a468;
extern int DAT_0060268c;
extern int DAT_006026bc;
extern float _DAT_005201d4;
extern float _DAT_005201dc;

void __cdecl FUN_0048e380(float *param_1, int param_2, int param_3);

/* vertex (12-byte stride) and UV (8-byte stride) element accessors */
#define VX(i, o) (*(float *)((unsigned)DAT_00602678 + (unsigned)(o) + (unsigned)(i) * 0xcu))
#define UV(i, o) (*(float *)((unsigned)DAT_006026a0 + (unsigned)(o) + (unsigned)(i) * 8u))

void __cdecl FUN_0048dba0(float *param_1, int param_2, int param_3, int param_4)
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
  int *piVar11;
  int iVar13;
  int *piVar14;
  float fStack_1c;
  float fStack_18;
  float fStack_14;
  float fStack_10;
  float fStack_c;
  float fStack_8;
  int iStack_4;

  if (DAT_00602678 != 0) {
    piVar11 = (int *)(param_2 + 4);
    iVar13 = param_3;
    piVar14 = piVar11;
    if (0 < param_3) {
      do {
        iVar2 = piVar14[1];
        iVar3 = *piVar14;
        fVar6 = VX(iVar2, 8) - VX(iVar3, 8);
        fVar4 = VX(iVar2, 0) - VX(iVar3, 0);
        fVar9 = VX(iVar2, 4) - VX(iVar3, 4);
        fVar5 = VX(iVar2, 0) - VX(piVar14[2], 0);
        fVar7 = VX(iVar2, 4) - VX(piVar14[2], 4);
        fVar8 = VX(iVar2, 8) - VX(piVar14[2], 8);
        *param_1 = fVar8 * fVar9 - fVar7 * fVar6;
        param_1[1] = fVar5 * fVar6 - fVar8 * fVar4;
        param_1[2] = fVar4 * fVar7 - fVar5 * fVar9;
        fVar4 = param_1[1] * param_1[1] + *param_1 * *param_1 + param_1[2] * param_1[2];
        if (fVar4 == _DAT_005201d4) {
          iVar1 = piVar14[3];
          if (iVar1 != -1) {
            iVar3 = *piVar14;
            fVar6 = VX(iVar1, 8) - VX(iVar3, 8);
            fVar4 = VX(iVar1, 0) - VX(iVar3, 0);
            fVar9 = VX(iVar1, 4) - VX(iVar3, 4);
            fVar5 = VX(iVar1, 0) - VX(piVar14[2], 0);
            fVar7 = VX(iVar1, 4) - VX(piVar14[2], 4);
            fVar8 = VX(iVar1, 8) - VX(piVar14[2], 8);
            *param_1 = fVar8 * fVar9 - fVar7 * fVar6;
            param_1[1] = fVar5 * fVar6 - fVar8 * fVar4;
            param_1[2] = fVar4 * fVar7 - fVar5 * fVar9;
            fVar4 = param_1[1] * param_1[1] + *param_1 * *param_1 + param_1[2] * param_1[2];
            if (fVar4 != _DAT_005201d4) goto LAB_0048dd49;
          }
        }
        else {
LAB_0048dd49:
          fVar4 = _DAT_005201dc / __builtin_sqrtf(fVar4);
          *param_1 = *param_1 * fVar4;
          param_1[1] = param_1[1] * fVar4;
          param_1[2] = fVar4 * param_1[2];
        }
        if (DAT_0052a468 != 0) {
          *param_1 = -*param_1;
          param_1[1] = -param_1[1];
          param_1[2] = -param_1[2];
        }
        param_1 = param_1 + 3;
        iVar13 = iVar13 + -1;
        piVar14 = piVar14 + 0x10;
      } while (iVar13 != 0);
    }
    if (DAT_006026a0 == 0) {
      param_1 = param_1 + (unsigned)param_3 * 0x18u;
    }
    else if (0 < param_3) {
      iStack_4 = param_3;
      do {
        iVar13 = *piVar11;
        iVar1 = piVar11[1];
        fVar4 = VX(iVar13, 0) - VX(iVar1, 0);
        fVar5 = VX(iVar13, 4) - VX(iVar1, 4);
        fVar6 = VX(iVar13, 8) - VX(iVar1, 8);
        iVar1 = piVar11[8];
        fStack_1c = VX(iVar13, 8) - VX(piVar11[2], 8);
        fStack_c = VX(iVar13, 0) - VX(piVar11[2], 0);
        fStack_8 = VX(iVar13, 4) - VX(piVar11[2], 4);
        fStack_18 = UV(iVar1, 0) - UV(piVar11[9], 0);
        fVar7 = UV(iVar1, 4) - UV(piVar11[9], 4);
        fVar8 = UV(iVar1, 4) - UV(piVar11[10], 4);
        fStack_14 = UV(iVar1, 0) - UV(piVar11[10], 0);
        fStack_10 = fStack_14 * fVar7 - fStack_18 * fVar8;
        *param_1 = fStack_c * fVar7 - fVar4 * fVar8;
        param_1[1] = fStack_8 * fVar7 - fVar8 * fVar5;
        param_1[2] = fVar7 * fStack_1c - fVar8 * fVar6;
        if ((fStack_10 == 0.0f) ||
            (param_1[1] * param_1[1] + *param_1 * *param_1 + param_1[2] * param_1[2] == _DAT_005201d4)) {
          if (piVar11[3] == -1) {
LAB_0048e265:
            fStack_14 = 1.0f;
            fStack_10 = 1.0f;
            *param_1 = fStack_c;
            fStack_18 = 0.0f;
            param_1[1] = fStack_8;
            param_1[2] = fStack_1c;
          }
          else {
            iVar1 = *piVar11;
            fStack_c = VX(iVar1, 0) - VX(piVar11[3], 0);
            fStack_8 = VX(iVar1, 4) - VX(piVar11[3], 4);
            fVar5 = VX(iVar1, 4) - VX(piVar11[1], 4);
            fVar4 = VX(iVar1, 0) - VX(piVar11[1], 0);
            fVar6 = VX(iVar1, 8) - VX(piVar11[1], 8);
            fStack_1c = VX(iVar1, 8) - VX(piVar11[3], 8);
            iVar13 = piVar11[8];
            fVar7 = UV(iVar13, 4) - UV(piVar11[9], 4);
            fStack_18 = UV(iVar13, 0) - UV(piVar11[9], 0);
            fStack_14 = UV(iVar13, 0) - UV(piVar11[0xb], 0);
            fVar8 = UV(iVar13, 4) - UV(piVar11[0xb], 4);
            fStack_10 = fStack_14 * fVar7 - fStack_18 * fVar8;
            *param_1 = fStack_c * fVar7 - fVar4 * fVar8;
            param_1[1] = fStack_8 * fVar7 - fVar8 * fVar5;
            param_1[2] = fVar7 * fStack_1c - fVar8 * fVar6;
            if ((fStack_10 == 0.0f) ||
                (param_1[1] * param_1[1] + *param_1 * *param_1 + param_1[2] * param_1[2] == _DAT_005201d4)) {
              iVar1 = *piVar11;
              iVar2 = piVar11[2];
              fVar5 = VX(iVar1, 4) - VX(iVar2, 4);
              fVar4 = VX(iVar1, 0) - VX(iVar2, 0);
              fVar6 = VX(iVar1, 8) - VX(iVar2, 8);
              fStack_c = VX(iVar1, 0) - VX(piVar11[3], 0);
              fStack_8 = VX(iVar1, 4) - VX(piVar11[3], 4);
              fStack_1c = VX(iVar1, 8) - VX(piVar11[3], 8);
              iVar13 = piVar11[8];
              fStack_18 = UV(iVar13, 0) - UV(piVar11[10], 0);
              fVar7 = UV(iVar13, 4) - UV(piVar11[10], 4);
              fVar8 = UV(iVar13, 4) - UV(piVar11[0xb], 4);
              fStack_14 = UV(iVar13, 0) - UV(piVar11[0xb], 0);
              fStack_10 = fStack_14 * fVar7 - fStack_18 * fVar8;
              *param_1 = fStack_c * fVar7 - fVar4 * fVar8;
              param_1[1] = fStack_8 * fVar7 - fVar8 * fVar5;
              param_1[2] = fVar7 * fStack_1c - fVar8 * fVar6;
              if ((fStack_10 == 0.0f) ||
                  (param_1[1] * param_1[1] + *param_1 * *param_1 + param_1[2] * param_1[2] == _DAT_005201d4)) {
                iVar13 = piVar11[1];
                iVar1 = piVar11[2];
                fVar4 = VX(iVar13, 0) - VX(iVar1, 0);
                fVar5 = VX(iVar13, 4) - VX(iVar1, 4);
                fVar6 = VX(iVar13, 8) - VX(iVar1, 8);
                iVar1 = piVar11[9];
                fStack_1c = VX(iVar13, 8) - VX(piVar11[3], 8);
                fStack_c = VX(iVar13, 0) - VX(piVar11[3], 0);
                fStack_8 = VX(iVar13, 4) - VX(piVar11[3], 4);
                fStack_18 = UV(iVar1, 0) - UV(piVar11[10], 0);
                fVar7 = UV(iVar1, 4) - UV(piVar11[10], 4);
                fVar8 = UV(iVar1, 4) - UV(piVar11[0xb], 4);
                fStack_14 = UV(iVar1, 0) - UV(piVar11[0xb], 0);
                fStack_10 = fStack_14 * fVar7 - fStack_18 * fVar8;
                *param_1 = fStack_c * fVar7 - fVar4 * fVar8;
                param_1[1] = fStack_8 * fVar7 - fVar8 * fVar5;
                param_1[2] = fVar7 * fStack_1c - fVar8 * fVar6;
                if ((fStack_10 == 0.0f) ||
                    (param_1[1] * param_1[1] + *param_1 * *param_1 + param_1[2] * param_1[2] == _DAT_005201d4))
                  goto LAB_0048e265;
              }
            }
          }
        }
        fStack_10 = _DAT_005201dc / fStack_10;
        *param_1 = *param_1 * fStack_10;
        param_1[1] = param_1[1] * fStack_10;
        fVar7 = -fStack_10;
        param_1[2] = param_1[2] * fStack_10;
        param_1[3] = (fStack_18 * fStack_c - fStack_14 * fVar4) * fVar7;
        param_1[4] = (fStack_18 * fStack_8 - fStack_14 * fVar5) * fVar7;
        param_1[5] = fVar7 * (fStack_18 * fStack_1c - fStack_14 * fVar6);
        if ((fStack_18 == 0.0f) && (fStack_14 == 0.0f)) {
          param_1[4] = 1.0f;
        }
        param_1 = param_1 + 6;
        piVar11 = piVar11 + 0x10;
        iStack_4 = iStack_4 + -1;
      } while (iStack_4 != 0);
    }
    if (*(int *)(param_4 + 0x84) != 0) {
      DAT_0060268c = 0;
      return;
    }
    DAT_0060268c = DAT_006026bc;
    FUN_0048e380(param_1, param_2, param_3);
  }
  return;
}
