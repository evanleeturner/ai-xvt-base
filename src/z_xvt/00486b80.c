/* SPDX-License-Identifier: MIT */
/* z_xvt 00486b80 */
extern int DAT_0052a348;
extern int _DAT_005254e0;
extern int DAT_005254e4;
extern float DAT_005254e8;
extern int DAT_006db7b4;
extern float _DAT_005201dc;
extern float _DAT_005201f4;
extern int DAT_0052a46c;
extern int DAT_00602678;
extern undefined4 DAT_00602690;
extern int DAT_006026a0;
extern int *DAT_006026a4;
extern int _DAT_006026a8;
extern int DAT_006026bc;
extern float *_DAT_006026c0;

int __cdecl __strcmpi(char *, char *);
undefined4 *FUN_0048e520(undefined4 *, char *);
void FUN_00485db0(undefined4 *);
void FUN_004150a0(float *, float *);
void FUN_00414fe0(float *, float *);
void FUN_004151d0(float *, float *);
float10 FUN_00415040(float *, float *);
float10 FUN_00415060(float *, int);
float10 FUN_00415080(float *, int);
void FUN_00415300(float *, float *);

void __cdecl FUN_00486b80(int param_1, undefined4 *param_2, undefined4 *param_3)
{
  float *pfVar1;
  int *piVar2;
  char *pcVar3;
  float fVar4;
  int iVar5;
  undefined4 *puVar6;
  undefined4 *puVar7;
  int iVar8;
  int iVar9;
  int iVar10;
  float *pfVar11;
  int *piVar12;
  float *pfVar13;
  float10 fVar14;
  float fStack_138;
  int iStack_134;
  int iStack_130;
  int iStack_12c;
  int iStack_128;
  int iStack_124;
  int iStack_13c;
  int iStack_140;
  float afStack_120[4];
  float afStack_110[16];
  undefined4 auStack_d0[52];

  if (param_2 == (undefined4 *)0x0) {
    return;
  }
  do {
    iStack_134 = 0;
    iStack_130 = 0;
    while (param_2[1] == 7) {
      if (DAT_0052a348 == 0) {
        iStack_128 = 0;
        pcVar3 = (char *)param_2[5];
        __builtin_memcpy(&iStack_13c, (char *)param_1 + 6, 4);
        if (0 < iStack_13c) {
          iStack_12c = 0;
          do {
            __builtin_memcpy(&iStack_140, (char *)param_1 + 10, 4);
            puVar7 = *(undefined4 **)(iStack_140 + iStack_12c);
            if (puVar7 == (undefined4 *)0x0) {
LAB_00486d69:
              puVar7 = (undefined4 *)0x0;
            }
            else if (((char *)*puVar7 == (char *)0x0) ||
                    (iVar5 = __strcmpi((char *)*puVar7, pcVar3), iVar5 != 0)) {
              iVar5 = 0;
              if (0 < (int)puVar7[2]) {
                iVar10 = 0;
                do {
                  param_2 = FUN_0048e520(*(undefined4 **)(puVar7[3] + iVar10), pcVar3);
                  if (param_2 != (undefined4 *)0x0) goto LAB_00486da0;
                  iVar10 = iVar10 + 4;
                  iVar5 = iVar5 + 1;
                } while (iVar5 < (int)puVar7[2]);
              }
              goto LAB_00486d69;
            }
            param_2 = puVar7;
            if (puVar7 != (undefined4 *)0x0) goto LAB_00486da0;
            iStack_12c = iStack_12c + 4;
            iStack_128 = iStack_128 + 1;
            __builtin_memcpy(&iStack_13c, (char *)param_1 + 6, 4);
          } while (iStack_128 < iStack_13c);
        }
        param_2 = (undefined4 *)0x0;
      }
      else {
        pcVar3 = (char *)param_2[5];
        if (*pcVar3 == '\0') {
          param_2 = (undefined4 *)*param_2;
        }
        else {
          iStack_124 = 0;
          __builtin_memcpy(&iStack_13c, (char *)param_1 + 6, 4);
          if (0 < iStack_13c) {
            iStack_128 = 0;
            do {
              __builtin_memcpy(&iStack_140, (char *)param_1 + 10, 4);
              puVar7 = *(undefined4 **)(iStack_140 + iStack_128);
              if (puVar7 == (undefined4 *)0x0) {
LAB_00486c7a:
                puVar6 = (undefined4 *)0x0;
              }
              else if (((char *)*puVar7 == (char *)0x0) ||
                      (iVar5 = __strcmpi((char *)*puVar7, pcVar3), puVar6 = puVar7, iVar5 != 0)) {
                iStack_12c = 0;
                if (0 < (int)puVar7[2]) {
                  iVar5 = 0;
                  do {
                    puVar6 = FUN_0048e520(*(undefined4 **)(puVar7[3] + iVar5), pcVar3);
                    if (puVar6 != (undefined4 *)0x0) goto LAB_00486cd1;
                    iVar5 = iVar5 + 4;
                    iStack_12c = iStack_12c + 1;
                  } while (iStack_12c < (int)puVar7[2]);
                }
                goto LAB_00486c7a;
              }
              if (puVar6 != (undefined4 *)0x0) {
LAB_00486cd1:
                *param_2 = (undefined4)puVar6;
                goto LAB_00486cb3;
              }
              iStack_128 = iStack_128 + 4;
              iStack_124 = iStack_124 + 1;
              __builtin_memcpy(&iStack_13c, (char *)param_1 + 6, 4);
            } while (iStack_124 < iStack_13c);
          }
          *param_2 = 0;
LAB_00486cb3:
          *(undefined1 *)param_2[5] = 0;
          param_2 = (undefined4 *)*param_2;
        }
      }
LAB_00486da0:
      if (param_2 == (undefined4 *)0x0) {
        return;
      }
    }
    pfVar13 = (float *)param_2[5];
    if (pfVar13 == (float *)0x0) {
      switch (param_2[1]) {
      case 10:
        iVar5 = param_2[4];
        if ((iVar5 == 8) || (iVar5 == 7)) {
          param_3[0x22] = DAT_00602690;
        }
        else if ((iVar5 == 6) || (iVar5 == 5)) {
          param_3[0x27] = DAT_00602690;
        }
        else {
          param_3[0x1d] = DAT_00602690;
        }
        break;
      case 0x14:
        param_3[0x29] = *param_2;
        DAT_006026a4 = (int *)param_2[5];
        param_3[0x2a] = (undefined4)DAT_006026a4;
        param_3[0x2b] = param_3[0x2a];
        iVar5 = (int)(param_3[0x2a] + 0x18u);
        param_3[0x2b] = (undefined4)iVar5;
        piVar12 = (int *)(param_3 + 0x2c);
        if (DAT_006026a4[1] == 0) {
          *piVar12 = *DAT_006026a4;
        }
        else {
          *piVar12 = iVar5;
          iVar10 = param_3[0x2a];
          iVar9 = (int)((unsigned int)*(int *)(iVar10 + 0x14) * (unsigned int)*(int *)(iVar10 + 0x10));
          if (*(int *)(iVar10 + 8) == iVar9) {
            iVar9 = *(int *)(iVar10 + 0xc);
          }
          *piVar12 = (int)((unsigned int)iVar9 + (unsigned int)iVar5);
        }
        iVar5 = *piVar12;
        *piVar12 = (int)((unsigned int)iVar5 + 0x1000u);
        param_3[0x2d] = (undefined4)((unsigned int)iVar5 + 0x1000u);
        *piVar12 = (int)((unsigned int)*piVar12 - 0x1000u);
        break;
      case 0x18:
        iStack_130 = (int)((unsigned int)_DAT_005254e0 + 1u);
        iVar5 = param_2[2];
        if (iVar5 < iStack_130) goto LAB_004874c7;
      }
    }
    else {
      switch (param_2[1]) {
      case 1:
      case 0xf:
      case 0x10:
      case 0x11:
        pfVar11 = pfVar13 + 1;
        param_3[0x23] = param_2[4];
        *(float *)(param_3 + 0x24) = *pfVar13;
        param_3[0x28] = (undefined4)pfVar11;
        pfVar11 = pfVar11 + (int)((unsigned int)param_2[4] * 0x10u);
        param_3[0x25] = (undefined4)pfVar11;
        iVar5 = param_2[4];
        param_3[0x26] = (undefined4)(pfVar11 + (int)((unsigned int)iVar5 * 3u));
        piVar12 = DAT_006026a4;
        iVar10 = param_2[4];
        if (param_3[0x2a] == 0) {
          param_3[0x2a] = (undefined4)DAT_006026a4;
          param_3[0x2b] = (undefined4)piVar12;
          piVar12 = piVar12 + 6;
          param_3[0x2b] = (undefined4)piVar12;
          piVar2 = (int *)(param_3 + 0x2c);
          if (DAT_006026a4[1] == 0) {
            *piVar2 = *DAT_006026a4;
          }
          else {
            *piVar2 = (int)piVar12;
            iVar9 = param_3[0x2a];
            iVar8 = (int)((unsigned int)*(int *)(iVar9 + 0x14) * (unsigned int)*(int *)(iVar9 + 0x10));
            if (*(int *)(iVar9 + 8) == iVar8) {
              iVar8 = *(int *)(iVar9 + 0xc);
            }
            *piVar2 = (int)((unsigned int)iVar8 + (unsigned int)(int)piVar12);
          }
          iVar9 = *piVar2;
          *piVar2 = (int)((unsigned int)iVar9 + 0x1000u);
          param_3[0x2d] = (undefined4)((unsigned int)iVar9 + 0x1000u);
          *piVar2 = (int)((unsigned int)*piVar2 - 0x1000u);
        }
        if (param_3[0x21] == 0) {
          param_3[0x21] = (undefined4)(pfVar11 + (int)((unsigned int)iVar5 * 3u) +
                                       (int)((unsigned int)iVar10 * 6u));
          FUN_00485db0(param_3);
          param_3[0x21] = 0;
        }
        else {
          FUN_00485db0(param_3);
        }
        break;
      case 2:
        pfVar11 = pfVar13 + 3;
        pfVar1 = (float *)(param_3 + 2);
        FUN_004150a0((float *)(param_3 + 5), pfVar11);
        FUN_00414fe0(pfVar1, pfVar11);
        *pfVar1 = *pfVar13 + *pfVar1;
        pfVar1 = (float *)(param_3 + 0x11);
        *(float *)(param_3 + 3) = pfVar13[1] + *(float *)(param_3 + 3);
        *(float *)(param_3 + 4) = pfVar13[2] + *(float *)(param_3 + 4);
        FUN_004151d0(pfVar1, pfVar11);
        fVar14 = FUN_00415040(pfVar13, pfVar1);
        *(float *)(param_3 + 0xe) = (float)((float10)*(float *)(param_3 + 0xe) - fVar14);
        fVar14 = FUN_00415060(pfVar13, (int)pfVar1);
        *(float *)(param_3 + 0xf) = (float)((float10)*(float *)(param_3 + 0xf) - fVar14);
        fVar14 = FUN_00415080(pfVar13, (int)pfVar1);
        *(float *)(param_3 + 0x10) = (float)((float10)*(float *)(param_3 + 0x10) - fVar14);
        break;
      case 3:
        param_3[0x1e] = param_2[4];
        param_3[0x1f] = (undefined4)pfVar13;
        break;
      case 4:
        *(float *)(param_3 + 2) = *pfVar13 + *(float *)(param_3 + 2);
        pfVar11 = (float *)(param_3 + 0x11);
        *(float *)(param_3 + 3) = pfVar13[1] + *(float *)(param_3 + 3);
        *(float *)(param_3 + 4) = pfVar13[2] + *(float *)(param_3 + 4);
        fVar14 = FUN_00415040(pfVar13, pfVar11);
        *(float *)(param_3 + 0xe) = (float)((float10)*(float *)(param_3 + 0xe) - fVar14);
        fVar14 = FUN_00415060(pfVar13, (int)pfVar11);
        *(float *)(param_3 + 0xf) = (float)((float10)*(float *)(param_3 + 0xf) - fVar14);
        fVar14 = FUN_00415080(pfVar13, (int)pfVar11);
        *(float *)(param_3 + 0x10) = (float)((float10)*(float *)(param_3 + 0x10) - fVar14);
        break;
      case 5:
        FUN_004150a0((float *)(param_3 + 5), pfVar13);
        FUN_00414fe0((float *)(param_3 + 2), pfVar13);
        FUN_004151d0((float *)(param_3 + 0x11), pfVar13);
        break;
      case 6:
        pfVar11 = pfVar13 + 1;
        pfVar1 = pfVar13 + 2;
        *(float *)(param_3 + 5) = *pfVar13 * *(float *)(param_3 + 5);
        *(float *)(param_3 + 6) = *pfVar11 * *(float *)(param_3 + 6);
        *(float *)(param_3 + 7) = *pfVar1 * *(float *)(param_3 + 7);
        *(float *)(param_3 + 8) = *pfVar13 * *(float *)(param_3 + 8);
        *(float *)(param_3 + 9) = *pfVar11 * *(float *)(param_3 + 9);
        *(float *)(param_3 + 10) = *pfVar1 * *(float *)(param_3 + 10);
        *(float *)(param_3 + 0xb) = *pfVar13 * *(float *)(param_3 + 0xb);
        *(float *)(param_3 + 0xc) = *pfVar11 * *(float *)(param_3 + 0xc);
        *(float *)(param_3 + 0xd) = *pfVar1 * *(float *)(param_3 + 0xd);
        *(float *)(param_3 + 2) = *pfVar13 * *(float *)(param_3 + 2);
        *(float *)(param_3 + 3) = *pfVar11 * *(float *)(param_3 + 3);
        *(float *)(param_3 + 4) = *pfVar1 * *(float *)(param_3 + 4);
        fVar4 = _DAT_005201dc / *pfVar13;
        *(float *)(param_3 + 0x11) = *(float *)(param_3 + 0x11) * fVar4;
        *(float *)(param_3 + 0x12) = *(float *)(param_3 + 0x12) * fVar4;
        *(float *)(param_3 + 0x13) = fVar4 * *(float *)(param_3 + 0x13);
        fVar4 = _DAT_005201dc / *pfVar11;
        *(float *)(param_3 + 0x14) = *(float *)(param_3 + 0x14) * fVar4;
        *(float *)(param_3 + 0x15) = *(float *)(param_3 + 0x15) * fVar4;
        *(float *)(param_3 + 0x16) = fVar4 * *(float *)(param_3 + 0x16);
        fVar4 = _DAT_005201dc / *pfVar1;
        *(float *)(param_3 + 0x17) = *(float *)(param_3 + 0x17) * fVar4;
        *(float *)(param_3 + 0x18) = *(float *)(param_3 + 0x18) * fVar4;
        *(float *)(param_3 + 0x19) = fVar4 * *(float *)(param_3 + 0x19);
        break;
      case 10:
        iVar5 = param_2[4];
        if ((iVar5 == 8) || (iVar5 == 7)) {
          param_3[0x22] = DAT_00602690;
        }
        else if ((iVar5 == 6) || (iVar5 == 5)) {
          param_3[0x27] = DAT_00602690;
        }
        else {
          param_3[0x1d] = DAT_00602690;
        }
        break;
      case 0xb:
        _DAT_006026c0 = pfVar13;
        param_3[0x21] = (undefined4)pfVar13;
        break;
      case 0xd:
        param_3[0x20] = (undefined4)pfVar13;
        break;
      case 0x13:
        *(float *)(param_3 + 0x1a) = *pfVar13;
        *(float *)(param_3 + 0x1b) = pfVar13[1];
        *(float *)(param_3 + 0x1c) = pfVar13[2];
        break;
      case 0x14:
        param_3[0x29] = *param_2;
        DAT_006026a4 = (int *)param_2[5];
        param_3[0x2a] = (undefined4)DAT_006026a4;
        param_3[0x2b] = param_3[0x2a];
        iVar5 = (int)(param_3[0x2a] + 0x18u);
        param_3[0x2b] = (undefined4)iVar5;
        piVar12 = (int *)(param_3 + 0x2c);
        if (DAT_006026a4[1] == 0) {
          *piVar12 = *DAT_006026a4;
        }
        else {
          *piVar12 = iVar5;
          iVar10 = param_3[0x2a];
          iVar9 = (int)((unsigned int)*(int *)(iVar10 + 0x14) * (unsigned int)*(int *)(iVar10 + 0x10));
          if (*(int *)(iVar10 + 8) == iVar9) {
            *piVar12 = (int)((unsigned int)*(int *)(iVar10 + 0xc) + (unsigned int)iVar5);
          }
          else {
            *piVar12 = (int)((unsigned int)iVar5 + (unsigned int)iVar9);
          }
        }
        iVar5 = *piVar12;
        *piVar12 = (int)((unsigned int)iVar5 + 0x1000u);
        param_3[0x2d] = (undefined4)((unsigned int)iVar5 + 0x1000u);
        *piVar12 = (int)((unsigned int)*piVar12 - 0x1000u);
        break;
      case 0x15:
        if ((DAT_006db7b4 < 1) || (DAT_005254e4 != 0)) {
          iStack_134 = DAT_005254e4;
          if (DAT_005254e4 == 0) {
            iStack_134 = 1;
          }
          else if ((int)param_2[2] < DAT_005254e4) {
            iStack_134 = -1;
          }
        }
        else {
          fStack_138 = 1.0f;
          if (0 < (int)DAT_005254e8) {
            fStack_138 = _DAT_005201dc / ((float)DAT_006db7b4 * DAT_005254e8);
          }
          iStack_134 = 1;
          iVar5 = param_2[2];
          if (0 < iVar5) {
            do {
              if (*pfVar13 <= fStack_138) break;
              pfVar13 = pfVar13 + 1;
              iStack_134 = iStack_134 + 1;
            } while (iStack_134 <= iVar5);
          }
          if (iVar5 < iStack_134) {
            iStack_134 = -1;
          }
        }
        break;
      case 0x17:
        if ((param_3[1] & 0x7fffffff) != 0) {
          *(float *)(param_3 + 0xe) = *(float *)(param_3 + 0xe) - *pfVar13;
          pfVar11 = (float *)(param_3 + 5);
          *(float *)(param_3 + 0xf) = *(float *)(param_3 + 0xf) - pfVar13[1];
          *(float *)(param_3 + 0x10) = *(float *)(param_3 + 0x10) - pfVar13[2];
          fVar14 = FUN_00415040(pfVar13, pfVar11);
          *(float *)(param_3 + 2) = (float)(fVar14 + (float10)*(float *)(param_3 + 2));
          fVar14 = FUN_00415060(pfVar13, (int)pfVar11);
          *(float *)(param_3 + 3) = (float)(fVar14 + (float10)*(float *)(param_3 + 3));
          fVar14 = FUN_00415080(pfVar13, (int)pfVar11);
          *(float *)(param_3 + 4) = (float)(fVar14 + (float10)*(float *)(param_3 + 4));
          afStack_120[0] = pfVar13[3] * _DAT_005201f4;
          afStack_120[1] = pfVar13[4] * _DAT_005201f4;
          afStack_120[2] = pfVar13[5] * _DAT_005201f4;
          *(undefined4 *)(afStack_120 + 3) = param_3[1];
          FUN_00415300(afStack_110, afStack_120);
          FUN_004150a0((float *)(param_3 + 0x11), afStack_110);
          FUN_00414fe0((float *)(param_3 + 0xe), afStack_110);
          FUN_004151d0(pfVar11, afStack_110);
          *(float *)(param_3 + 0xe) = *pfVar13 + *(float *)(param_3 + 0xe);
          *(float *)(param_3 + 0xf) = pfVar13[1] + *(float *)(param_3 + 0xf);
          *(float *)(param_3 + 0x10) = pfVar13[2] + *(float *)(param_3 + 0x10);
          fVar14 = FUN_00415040(pfVar13, pfVar11);
          *(float *)(param_3 + 2) = (float)((float10)*(float *)(param_3 + 2) - fVar14);
          fVar14 = FUN_00415060(pfVar13, (int)pfVar11);
          *(float *)(param_3 + 3) = (float)((float10)*(float *)(param_3 + 3) - fVar14);
          fVar14 = FUN_00415080(pfVar13, (int)pfVar11);
          *(float *)(param_3 + 4) = (float)((float10)*(float *)(param_3 + 4) - fVar14);
        }
        break;
      case 0x18:
        iVar5 = param_2[2];
        iStack_130 = (int)((unsigned int)_DAT_005254e0 + 1u);
        if (iVar5 < iStack_130) {
LAB_004874c7:
          iStack_130 = iVar5;
        }
      }
    }
    if (param_2[2] == 0) {
      return;
    }
    if (iStack_130 == 0) {
      if (iStack_134 == 0) {
        puVar7 = auStack_d0;
        for (iVar5 = 0x34; iVar5 != 0; iVar5 = iVar5 + -1) {
          *puVar7 = *param_3;
          param_3 = param_3 + 1;
          puVar7 = puVar7 + 1;
        }
        iVar10 = 0;
        DAT_00602678 = 0;
        DAT_006026a0 = 0;
        iVar5 = 0;
        _DAT_006026c0 = (float *)0x0;
        _DAT_006026a8 = 0;
        DAT_00602690 = 0;
        DAT_006026bc = 0;
        if ((int)param_2[2] < 1) {
          DAT_00602678 = 0;
          DAT_00602690 = 0;
          DAT_006026a0 = 0;
          _DAT_006026a8 = 0;
          DAT_006026bc = 0;
          _DAT_006026c0 = (float *)0x0;
          return;
        }
        do {
          DAT_0052a46c = DAT_0052a46c + 1;
          iVar10 = iVar10 + 4;
          iVar5 = iVar5 + 1;
          FUN_00486b80(param_1, *(undefined4 **)(param_2[3] + -4 + iVar10), auStack_d0);
        } while (iVar5 < (int)param_2[2]);
        return;
      }
      if (iStack_134 == -1) {
        return;
      }
      iVar5 = param_2[3];
      iStack_130 = iStack_134;
    }
    else {
      iVar5 = param_2[3];
    }
    DAT_0052a46c = DAT_0052a46c + 1;
    param_2 = *(undefined4 **)(iVar5 + -4 + (int)((unsigned int)iStack_130 * 4u));
    if (param_2 == (undefined4 *)0x0) {
      return;
    }
  } while (1);
}
