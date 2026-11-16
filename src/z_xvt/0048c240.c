/* SPDX-License-Identifier: MIT */
/* z_xvt 0048c240 */
extern int DAT_0060268c;
extern int DAT_006a8884;
extern unsigned char DAT_006a887a[4]; /* unaligned global: memcpy only */
extern float *DAT_00602678;
extern int DAT_006026bc;
extern float *DAT_006026c0;
extern float *DAT_006026a0;
extern int DAT_006026a8;
extern int DAT_00602690;

#define ADDI(a, b) ((int)((unsigned int)(a) + (unsigned int)(b)))
#define SUBI(a, b) ((int)((unsigned int)(a) - (unsigned int)(b)))
#define MULI(a, b) ((int)((unsigned int)(a) * (unsigned int)(b)))

int FUN_0048d980(int param_1, int param_2, int param_3);
void FUN_0048dba0(int *param_1, int param_2, int param_3, int param_4);
void FUN_00415300(float *param_1, float *param_2);
void FUN_00414fe0(float *param_1, float *param_2);
unsigned char *FUN_0048e580(int *param_1, char *param_2);
int FUN_0048c240(undefined4 param_1, int *param_2, int param_3, int *param_4,
                 undefined4 *param_5, unsigned char *param_6);

int FUN_0048c240(undefined4 param_1, int *param_2, int param_3, int *param_4,
                 undefined4 *param_5, unsigned char *param_6)
{
  int *pfVar3;
  int *pfVar9;
  int *pfVar15;
  int *pfVar22;
  int *puVar1;
  int *piVar19;
  int *local_d8;
  int local_dc;
  int local_e0;
  int local_d0[52];
  int fVar10, fVar12, fVar4, fVar14, fVar13, fVar23, fVar24;
  int iVar6, iVar11, iVar16, iVar17, iVar18;
  unsigned int uVar7, uVar8;
  char *pcVar20;
  char *pcVar21;
  char cVar2;
  unsigned char *pbVar5;
  int tmp;

  pfVar3 = param_4;
  if (param_2 == (int *)0) {
    return 0;
  }
  pfVar9 = param_4 + 6;
  fVar10 = param_2[1];
  param_4[1] = fVar10;
  if (*param_2 == 0) {
    *param_4 = 0;
    param_4 = pfVar9;
  } else {
    *param_4 = (int)pfVar9;
    uVar7 = 0xffffffff;
    pcVar21 = (char *)*param_2;
    do {
      pcVar20 = pcVar21;
      if (uVar7 == 0) break;
      uVar7 = uVar7 - 1;
      pcVar20 = pcVar21 + 1;
      cVar2 = *pcVar21;
      pcVar21 = pcVar20;
    } while (cVar2 != '\0');
    uVar7 = ~uVar7;
    pfVar15 = (int *)(pcVar20 - uVar7);
    pfVar22 = pfVar9;
    for (uVar8 = uVar7 >> 2; uVar8 != 0; uVar8 = uVar8 - 1) {
      *pfVar22 = *pfVar15;
      pfVar15 = pfVar15 + 1;
      pfVar22 = pfVar22 + 1;
    }
    for (uVar7 = uVar7 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
      *(char *)pfVar22 = *(char *)pfVar15;
      pfVar15 = (int *)((char *)pfVar15 + 1);
      pfVar22 = (int *)((char *)pfVar22 + 1);
    }
    uVar7 = 0xffffffff;
    pcVar21 = (char *)*param_2;
    do {
      if (uVar7 == 0) break;
      uVar7 = uVar7 - 1;
      cVar2 = *pcVar21;
      pcVar21 = pcVar21 + 1;
    } while (cVar2 != '\0');
    param_4 = (int *)((char *)pfVar9 + ~uVar7);
  }
  pfVar3[4] = 1;
  pfVar3[5] = (int)param_4;
  pfVar3[2] = 0;
  pfVar3[3] = 0;
  iVar17 = param_2[5];
  pfVar9 = param_4;
  if ((iVar17 != 0) && (param_2[4] != 0)) {
    local_d8 = *(int **)(iVar17 + 8);
    puVar1 = (int *)(iVar17 + 8);
    if (local_d8 != (int *)0) {
      switch (fVar10) {
      case 1:
        iVar6 = 0;
        fVar10 = 0;
        pfVar9 = param_4 + 1;
        pfVar3[4] = 0;
        iVar11 = *(int *)(iVar17 + 4);
        if (*(int *)(iVar17 + 0x28) == iVar11) {
          local_dc = *(int *)(iVar17 + 0x2c);
        } else {
          local_dc = 0;
        }
        iVar16 = 0;
        if (*(int *)(iVar17 + 0x1c) == iVar11) {
          iVar16 = *(int *)(iVar17 + 0x20);
        }
        pfVar15 = pfVar9;
        if (0 < iVar11) {
          do {
            iVar11 = ADDI(iVar6, 1);
            do {
              iVar18 = iVar11;
              iVar11 = ADDI(iVar18, 2);
              *pfVar15 = local_d8[iVar6];
              pfVar15[1] = local_d8[iVar18];
              pfVar15[2] = local_d8[ADDI(iVar18, 1)];
              pfVar15[3] = local_d8[iVar11];
              if (local_dc == 0) {
                pfVar15[8] = *pfVar15;
                pfVar15[9] = pfVar15[1];
                pfVar15[10] = pfVar15[2];
                pfVar15[0xb] = pfVar15[3];
              } else {
                pfVar15[8] = ((int *)local_dc)[iVar6];
                pfVar15[9] = ((int *)local_dc)[SUBI(iVar11, 2)];
                pfVar15[10] = ((int *)local_dc)[SUBI(iVar11, 1)];
                pfVar15[0xb] = ((int *)local_dc)[iVar11];
              }
              if (iVar16 == 0) {
                pfVar15[0xc] = *pfVar15;
                pfVar15[0xd] = pfVar15[1];
                pfVar15[0xe] = pfVar15[2];
                pfVar15[0xf] = pfVar15[3];
              } else {
                pfVar15[0xc] = ((int *)iVar16)[iVar6];
                pfVar15[0xd] = ((int *)iVar16)[SUBI(iVar11, 2)];
                pfVar15[0xe] = ((int *)iVar16)[SUBI(iVar11, 1)];
                pfVar15[0xf] = ((int *)iVar16)[iVar11];
              }
              fVar12 = FUN_0048d980((int)pfVar3, pfVar15[0], pfVar15[1]);
              pfVar15[4] = fVar12;
              if (fVar12 == -1) {
                pfVar15[4] = fVar10;
                fVar10 = ADDI(fVar10, 1);
              }
              fVar12 = FUN_0048d980((int)pfVar3, pfVar15[1], pfVar15[2]);
              pfVar15[5] = fVar12;
              if (fVar12 == -1) {
                pfVar15[5] = fVar10;
                fVar10 = ADDI(fVar10, 1);
              }
              if (pfVar15[3] == -1) {
                fVar12 = FUN_0048d980((int)pfVar3, pfVar15[2], pfVar15[0]);
                pfVar15[6] = fVar12;
                if (fVar12 == -1) {
                  pfVar15[6] = fVar10;
                  fVar10 = ADDI(fVar10, 1);
                }
                pfVar15[7] = -1;
              } else {
                fVar12 = FUN_0048d980((int)pfVar3, pfVar15[2], pfVar15[3]);
                pfVar15[6] = fVar12;
                if (fVar12 == -1) {
                  pfVar15[6] = fVar10;
                  fVar10 = ADDI(fVar10, 1);
                }
                fVar12 = FUN_0048d980((int)pfVar3, pfVar15[3], pfVar15[0]);
                pfVar15[7] = fVar12;
                if (fVar12 == -1) {
                  pfVar15[7] = fVar10;
                  fVar10 = ADDI(fVar10, 1);
                }
              }
              pfVar15 = pfVar15 + 0x10;
              pfVar3[4] = ADDI(pfVar3[4], 1);
              if (local_d8[iVar11] == -1) {
                iVar6 = ADDI(iVar18, 3);
                goto LAB_0048c560;
              }
            } while (local_d8[ADDI(iVar18, 3)] != -1);
            iVar6 = ADDI(iVar18, 4);
LAB_0048c560:;
          } while (iVar6 < *(int *)(iVar17 + 4));
        }
        pfVar9 = pfVar9 + MULI(pfVar3[4], 0x10);
        FUN_0048dba0(pfVar9, pfVar3[5], pfVar3[4], param_3);
        fVar12 = pfVar3[4];
        iVar17 = MULI(DAT_0060268c, 3);
        *(int *)pfVar3[5] = fVar10;
        pfVar9 = pfVar9 + MULI(fVar12, 9) + iVar17;
        if (DAT_006a8884 < fVar10) {
          DAT_006a8884 = fVar10;
        }
        break;
      case 2: {
        float *fp4 = (float *)param_4;
        float *fpS;
        float fF4, fF10, fF12;
        float *fp15;
        pfVar9 = param_4 + 0xc;
        fp15 = *(float **)(iVar17 + 0x38);
        if (fp15 != (float *)0) {
          fF4 = -*fp15;
          *fp4 = fF4;
          fp4[1] = -fp15[1];
          fF10 = fp4[1];
          fp4[2] = -fp15[2];
          fF12 = fp4[2];
          if (*(float **)(iVar17 + 0x2c) != (float *)0) {
            fp15 = fp4 + 3;
            FUN_00415300(fp15, *(float **)(iVar17 + 0x2c));
            FUN_00414fe0(fp4, fp15);
            *fp4 = *fp4 - fF4;
            fp4[1] = fp4[1] - fF10;
            fp4[2] = fp4[2] - fF12;
            fpS = *(float **)(iVar17 + 0x20);
            if (fpS != (float *)0) {
              fp4[3] = fp4[3] * *fpS;
              fp4[4] = fpS[1] * fp4[4];
              fp4[5] = fp4[5] * fpS[2];
              fp4[6] = fp4[6] * *fpS;
              fp4[7] = fp4[7] * fpS[1];
              fp4[8] = fp4[8] * fpS[2];
              fp4[9] = fp4[9] * *fpS;
              fp4[10] = fp4[10] * fpS[1];
              fp4[0xb] = fp4[0xb] * fpS[2];
              *fp4 = *fp4 * *fpS;
              fp4[1] = fp4[1] * fpS[1];
              fp4[2] = fp4[2] * fpS[2];
              fpS = *(float **)(iVar17 + 0x14);
              if (fpS != (float *)0) {
                *fp4 = *fp4 + fF4;
                fp4[1] = fp4[1] + fF10;
                fp4[2] = fp4[2] + fF12;
                FUN_00415300(fp15, fpS);
                FUN_00414fe0(fp4, fp15);
                *fp4 = *fp4 - fF4;
                fp4[1] = fp4[1] - fF10;
                fp4[2] = fp4[2] - fF12;
                fp15 = *(float **)(iVar17 + 8);
                if (fp15 != (float *)0) {
                  *fp4 = *fp4 - *fp15;
                  fp4[1] = fp4[1] - fp15[1];
                  fp4[2] = fp4[2] - fp15[2];
                }
              }
            }
          }
        }
        break;
      }
      case 3:
        pfVar3[4] = *(int *)(iVar17 + 4);
        pfVar9 = *(int **)puVar1;
        pfVar15 = param_4;
        for (uVar7 = (unsigned int)MULI(*(int *)(iVar17 + 4), 0xc) >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
          *pfVar15 = *pfVar9;
          pfVar9 = pfVar9 + 1;
          pfVar15 = pfVar15 + 1;
        }
        DAT_00602678 = (float *)param_4;
        DAT_006026bc = *(int *)(iVar17 + 4);
        pfVar9 = param_4 + MULI(DAT_006026bc, 3);
        __builtin_memcpy(&tmp, DAT_006a887a, 4);
        if (tmp < pfVar3[4]) {
          tmp = pfVar3[4];
          __builtin_memcpy(DAT_006a887a, &tmp, 4);
        }
        break;
      case 4:
        *param_4 = *local_d8;
        param_4[1] = local_d8[1];
        param_4[2] = local_d8[2];
        pfVar9 = param_4 + 3;
        break;
      case 5:
        FUN_00415300((float *)param_4, (float *)local_d8);
        pfVar9 = param_4 + 9;
        break;
      case 6:
        *param_4 = *local_d8;
        param_4[1] = local_d8[1];
        param_4[2] = local_d8[2];
        pfVar9 = param_4 + 3;
        break;
      case 7:
        if (iVar17 != 0) {
          uVar7 = 0xffffffff;
          pfVar3[4] = 1;
          pfVar3[5] = (int)param_4;
          pcVar21 = (char *)*puVar1;
          do {
            pcVar20 = pcVar21;
            if (uVar7 == 0) break;
            uVar7 = uVar7 - 1;
            pcVar20 = pcVar21 + 1;
            cVar2 = *pcVar21;
            pcVar21 = pcVar20;
          } while (cVar2 != '\0');
          uVar7 = ~uVar7;
          pfVar9 = (int *)(pcVar20 - uVar7);
          pfVar15 = param_4;
          for (uVar8 = uVar7 >> 2; uVar8 != 0; uVar8 = uVar8 - 1) {
            *pfVar15 = *pfVar9;
            pfVar9 = pfVar9 + 1;
            pfVar15 = pfVar15 + 1;
          }
          for (uVar7 = uVar7 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
            *(char *)pfVar15 = *(char *)pfVar9;
            pfVar9 = (int *)((char *)pfVar9 + 1);
            pfVar15 = (int *)((char *)pfVar15 + 1);
          }
          uVar7 = 0xffffffff;
          pcVar21 = (char *)*puVar1;
          do {
            if (uVar7 == 0) break;
            uVar7 = uVar7 - 1;
            cVar2 = *pcVar21;
            pcVar21 = pcVar21 + 1;
          } while (cVar2 != '\0');
          pfVar9 = (int *)((char *)param_4 + ~uVar7);
        }
        break;
      case 9:
        fVar10 = *(int *)(iVar17 + 4);
        if (*(int *)(iVar17 + 4) < *(int *)(iVar17 + 0x10)) {
          fVar10 = *(int *)(iVar17 + 0x10);
        }
        if (fVar10 < *(int *)(iVar17 + 0x1c)) {
          fVar10 = *(int *)(iVar17 + 0x1c);
        }
        if (fVar10 < *(int *)(iVar17 + 0x28)) {
          fVar10 = *(int *)(iVar17 + 0x28);
        }
        if (fVar10 < *(int *)(iVar17 + 0x34)) {
          fVar10 = *(int *)(iVar17 + 0x34);
        }
        if (fVar10 < *(int *)(iVar17 + 0x40)) {
          fVar10 = *(int *)(iVar17 + 0x40);
        }
        pfVar3[4] = fVar10;
        pfVar9 = *(int **)(iVar17 + 8);
        pfVar15 = param_4;
        for (uVar7 = (unsigned int)MULI(*(int *)(iVar17 + 4), 0xc) >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
          *pfVar15 = *pfVar9;
          pfVar9 = pfVar9 + 1;
          pfVar15 = pfVar15 + 1;
        }
        iVar11 = *(int *)(iVar17 + 4);
        pfVar9 = param_4 + MULI(iVar11, 3);
        param_4 = pfVar9;
        if (iVar11 < fVar10) {
          pfVar15 = (int *)ADDI(SUBI(*(int *)(iVar17 + 8), 0xc), MULI(iVar11, 0xc));
          iVar6 = SUBI(fVar10, iVar11);
          param_4 = pfVar9 + MULI(iVar6, 3);
          if (iVar11 < fVar10) {
            do {
              *pfVar9 = *pfVar15;
              iVar6 = SUBI(iVar6, 1);
              pfVar9[1] = pfVar15[1];
              pfVar9[2] = pfVar15[2];
              pfVar9 = pfVar9 + 3;
            } while (iVar6 != 0);
          }
        }
        pfVar9 = param_4;
        if (*(int **)(iVar17 + 0x14) != (int *)0) {
          pfVar9 = *(int **)(iVar17 + 0x14);
          pfVar15 = param_4;
          for (uVar7 = (unsigned int)MULI(*(int *)(iVar17 + 0x10), 0xc) >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
            *pfVar15 = *pfVar9;
            pfVar9 = pfVar9 + 1;
            pfVar15 = pfVar15 + 1;
          }
          iVar11 = *(int *)(iVar17 + 0x10);
          pfVar9 = param_4 + MULI(iVar11, 3);
          param_4 = pfVar9;
          if (iVar11 < fVar10) {
            pfVar15 = (int *)ADDI(SUBI(*(int *)(iVar17 + 0x14), 0xc), MULI(iVar11, 0xc));
            iVar6 = SUBI(fVar10, iVar11);
            param_4 = pfVar9 + MULI(iVar6, 3);
            if (iVar11 < fVar10) {
              do {
                *pfVar9 = *pfVar15;
                iVar6 = SUBI(iVar6, 1);
                pfVar9[1] = pfVar15[1];
                pfVar9[2] = pfVar15[2];
                pfVar9 = pfVar9 + 3;
              } while (iVar6 != 0);
            }
          }
          pfVar9 = param_4;
          if (*(int **)(iVar17 + 0x20) != (int *)0) {
            pfVar9 = *(int **)(iVar17 + 0x20);
            pfVar15 = param_4;
            for (uVar7 = (unsigned int)MULI(*(int *)(iVar17 + 0x1c), 0xc) >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
              *pfVar15 = *pfVar9;
              pfVar9 = pfVar9 + 1;
              pfVar15 = pfVar15 + 1;
            }
            iVar11 = *(int *)(iVar17 + 0x1c);
            pfVar9 = param_4 + MULI(iVar11, 3);
            param_4 = pfVar9;
            if (iVar11 < fVar10) {
              pfVar15 = (int *)ADDI(SUBI(*(int *)(iVar17 + 0x20), 0xc), MULI(iVar11, 0xc));
              iVar6 = SUBI(fVar10, iVar11);
              param_4 = pfVar9 + MULI(iVar6, 3);
              if (iVar11 < fVar10) {
                do {
                  *pfVar9 = *pfVar15;
                  iVar6 = SUBI(iVar6, 1);
                  pfVar9[1] = pfVar15[1];
                  pfVar9[2] = pfVar15[2];
                  pfVar9 = pfVar9 + 3;
                } while (iVar6 != 0);
              }
            }
            pfVar9 = param_4;
            if (*(int **)(iVar17 + 0x2c) != (int *)0) {
              pfVar9 = *(int **)(iVar17 + 0x2c);
              pfVar15 = param_4;
              for (uVar7 = (unsigned int)MULI(*(int *)(iVar17 + 0x28), 0xc) >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
                *pfVar15 = *pfVar9;
                pfVar9 = pfVar9 + 1;
                pfVar15 = pfVar15 + 1;
              }
              iVar11 = *(int *)(iVar17 + 0x28);
              pfVar9 = param_4 + MULI(iVar11, 3);
              param_4 = pfVar9;
              if (iVar11 < fVar10) {
                pfVar15 = (int *)ADDI(SUBI(*(int *)(iVar17 + 0x2c), 0xc), MULI(iVar11, 0xc));
                iVar6 = SUBI(fVar10, iVar11);
                param_4 = pfVar9 + MULI(iVar6, 3);
                if (iVar11 < fVar10) {
                  do {
                    *pfVar9 = *pfVar15;
                    iVar6 = SUBI(iVar6, 1);
                    pfVar9[1] = pfVar15[1];
                    pfVar9[2] = pfVar15[2];
                    pfVar9 = pfVar9 + 3;
                  } while (iVar6 != 0);
                }
              }
              pfVar9 = param_4;
              if (*(int **)(iVar17 + 0x38) != (int *)0) {
                pfVar9 = *(int **)(iVar17 + 0x38);
                pfVar15 = param_4;
                for (uVar7 = *(unsigned int *)(iVar17 + 0x34) & 0x3fffffff; uVar7 != 0; uVar7 = uVar7 - 1) {
                  *pfVar15 = *pfVar9;
                  pfVar9 = pfVar9 + 1;
                  pfVar15 = pfVar15 + 1;
                }
                iVar11 = *(int *)(iVar17 + 0x34);
                pfVar9 = param_4 + iVar11;
                param_4 = pfVar9;
                if (iVar11 < fVar10) {
                  iVar6 = *(int *)(iVar17 + 0x38);
                  iVar16 = SUBI(fVar10, iVar11);
                  param_4 = pfVar9 + iVar16;
                  if (iVar11 < fVar10) {
                    do {
                      iVar16 = SUBI(iVar16, 1);
                      *pfVar9 = *(int *)ADDI(SUBI(iVar6, 4), MULI(iVar11, 4));
                      pfVar9 = pfVar9 + 1;
                    } while (iVar16 != 0);
                  }
                }
                pfVar9 = param_4;
                if (*(int **)(iVar17 + 0x44) != (int *)0) {
                  pfVar9 = *(int **)(iVar17 + 0x44);
                  pfVar15 = param_4;
                  for (uVar7 = *(unsigned int *)(iVar17 + 0x40) & 0x3fffffff; uVar7 != 0; uVar7 = uVar7 - 1) {
                    *pfVar15 = *pfVar9;
                    pfVar9 = pfVar9 + 1;
                    pfVar15 = pfVar15 + 1;
                  }
                  iVar11 = *(int *)(iVar17 + 0x40);
                  pfVar9 = param_4 + iVar11;
                  if (iVar11 < fVar10) {
                    iVar17 = *(int *)(iVar17 + 0x44);
                    iVar6 = SUBI(fVar10, iVar11);
                    pfVar15 = pfVar9 + iVar6;
                    pfVar22 = pfVar9;
                    pfVar9 = pfVar15;
                    if (iVar11 < fVar10) {
                      do {
                        iVar6 = SUBI(iVar6, 1);
                        *pfVar22 = *(int *)ADDI(SUBI(iVar17, 4), MULI(iVar11, 4));
                        pfVar22 = pfVar22 + 1;
                      } while (iVar6 != 0);
                    }
                  }
                }
              }
            }
          }
        }
        break;
      case 10:
      case 14:
        pfVar3[4] = *local_d8;
        break;
      case 11:
        pfVar3[4] = *(int *)(iVar17 + 4);
        pfVar9 = *(int **)puVar1;
        pfVar15 = param_4;
        for (uVar7 = (unsigned int)MULI(*(int *)(iVar17 + 4), 0xc) >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
          *pfVar15 = *pfVar9;
          pfVar9 = pfVar9 + 1;
          pfVar15 = pfVar15 + 1;
        }
        DAT_006026c0 = (float *)param_4;
        *(int **)(param_3 + 0x84) = param_4;
        pfVar9 = param_4 + MULI(*(int *)(iVar17 + 4), 3);
        break;
      case 13:
        pfVar3[4] = *(int *)(iVar17 + 4);
        pfVar9 = *(int **)puVar1;
        pfVar15 = param_4;
        for (iVar11 = (int)((*(unsigned int *)(iVar17 + 4) & 0x1fffffff) << 1); iVar11 != 0; iVar11 = SUBI(iVar11, 1)) {
          *pfVar15 = *pfVar9;
          pfVar9 = pfVar9 + 1;
          pfVar15 = pfVar15 + 1;
        }
        DAT_006026a0 = (float *)param_4;
        pfVar9 = param_4 + *(unsigned int *)(iVar17 + 4) * 2u;
        break;
      case 15:
        fVar10 = *local_d8;
        pfVar9 = param_4 + 1;
        iVar11 = 0;
        if (*(int **)(iVar17 + 0x14) != (int *)0) {
          iVar6 = **(int **)(iVar17 + 0x14);
          if (*(int **)(iVar17 + 0x20) != (int *)0) {
            iVar17 = **(int **)(iVar17 + 0x20);
            fVar12 = 0;
            if (iVar17 != 1 && -1 < SUBI(iVar17, 1)) {
              local_dc = 0;
              pfVar15 = pfVar9;
              local_e0 = iVar6;
              do {
                iVar16 = 0;
                if (iVar6 != 1 && -1 < SUBI(iVar6, 1)) {
                  do {
                    pfVar22 = pfVar15 + 4;
                    fVar4 = ADDI(ADDI(fVar10, local_dc), iVar16);
                    *pfVar15 = fVar4;
                    pfVar15[1] = ADDI(fVar4, 1);
                    fVar4 = ADDI(ADDI(fVar10, local_e0), iVar16);
                    pfVar15[2] = ADDI(fVar4, 1);
                    pfVar15[3] = fVar4;
                    pfVar15[8] = *pfVar15;
                    pfVar15[9] = pfVar15[1];
                    pfVar15[10] = pfVar15[2];
                    pfVar15[0xb] = pfVar15[3];
                    pfVar15[0xc] = *pfVar15;
                    pfVar15[0xd] = pfVar15[1];
                    pfVar15[0xe] = pfVar15[2];
                    pfVar15[0xf] = pfVar15[3];
                    if (iVar11 == 0) {
                      *pfVar22 = fVar12;
                      fVar4 = ADDI(fVar12, 1);
                    } else {
                      fVar4 = fVar12;
                      if (iVar11 == 1) {
                        *pfVar22 = ADDI(ADDI(ADDI(iVar16, MULI(iVar6, -3)), fVar12), 4);
                      } else {
                        *pfVar22 = ADDI(fVar12, MULI(iVar6, -2));
                      }
                    }
                    pfVar15[5] = fVar4;
                    pfVar15[6] = ADDI(fVar4, 1);
                    pfVar22 = pfVar15 + 7;
                    fVar12 = ADDI(fVar4, 2);
                    if (iVar16 == 0) {
                      *pfVar22 = fVar12;
                      fVar12 = ADDI(fVar4, 3);
                    } else {
                      *pfVar22 = SUBI(fVar4, 2);
                      if (iVar11 == 0) {
                        *pfVar22 = SUBI(fVar4, 3);
                      }
                      if (iVar16 == 1) {
                        *pfVar22 = SUBI(*pfVar22, 1);
                      }
                    }
                    pfVar15 = pfVar15 + 0x10;
                    iVar16 = ADDI(iVar16, 1);
                  } while (iVar16 < SUBI(iVar6, 1));
                }
                local_e0 = ADDI(local_e0, iVar6);
                local_dc = ADDI(local_dc, iVar6);
                iVar11 = ADDI(iVar11, 1);
              } while (iVar11 < SUBI(iVar17, 1));
            }
            fVar10 = MULI(SUBI(iVar6, 1), SUBI(iVar17, 1));
            pfVar3[4] = fVar10;
            pfVar9 = pfVar9 + MULI(fVar10, 0x10);
            FUN_0048dba0(pfVar9, pfVar3[5], fVar10, param_3);
            fVar10 = pfVar3[4];
            iVar17 = MULI(DAT_0060268c, 3);
            *(int *)pfVar3[5] = fVar12;
            pfVar9 = pfVar9 + MULI(fVar10, 9) + iVar17;
            if (DAT_006a8884 < fVar12) {
              DAT_006a8884 = fVar12;
            }
          }
        }
        break;
      case 16:
        fVar10 = 0;
        pfVar15 = param_4 + 1;
        fVar12 = 0;
        local_e0 = 0;
        pfVar3[4] = 0;
        pfVar9 = pfVar15;
        if (0 < *(int *)(iVar17 + 4)) {
          do {
            fVar4 = *local_d8;
            fVar14 = fVar12;
            if (2 < fVar4) {
              iVar11 = SUBI(ADDI(fVar12, 1), fVar12);
              fVar14 = ADDI(fVar12, 1);
              fVar23 = ADDI(fVar10, 1);
              fVar24 = fVar10;
              for (;;) {
                fVar10 = fVar23;
                if (!(iVar11 < fVar4)) break;
                pfVar3[4] = ADDI(pfVar3[4], 1);
                *pfVar9 = fVar12;
                pfVar9[1] = fVar14;
                pfVar9[2] = ADDI(fVar14, 1);
                fVar13 = ADDI(fVar14, 2);
                if (SUBI(fVar13, fVar4) == fVar12) {
                  pfVar9[3] = -1;
                  pfVar9[4] = fVar24;
                  pfVar9[5] = fVar10;
                  pfVar9[6] = ADDI(fVar10, 1);
                  fVar23 = ADDI(fVar10, 2);
                  pfVar9[7] = -1;
                } else {
                  pfVar9[3] = fVar13;
                  fVar13 = ADDI(fVar14, 3);
                  if (SUBI(fVar13, fVar4) != fVar12) {
                    fVar13 = ADDI(fVar14, 2);
                  }
                  pfVar9[4] = fVar24;
                  pfVar9[5] = fVar10;
                  pfVar9[6] = ADDI(fVar10, 1);
                  fVar24 = ADDI(fVar10, 2);
                  pfVar9[7] = fVar24;
                  fVar23 = ADDI(fVar10, 3);
                }
                pfVar9[8] = *pfVar9;
                pfVar9[9] = pfVar9[1];
                pfVar9[10] = pfVar9[2];
                pfVar9[0xb] = pfVar9[3];
                pfVar9[0xc] = *pfVar9;
                pfVar9[0xd] = pfVar9[1];
                pfVar9[0xe] = pfVar9[2];
                pfVar9[0xf] = pfVar9[3];
                pfVar9 = pfVar9 + 0x10;
                fVar14 = fVar13;
                iVar11 = SUBI(fVar13, fVar12);
              }
            }
            local_d8 = local_d8 + 1;
            local_e0 = ADDI(local_e0, 1);
            fVar12 = fVar14;
          } while (local_e0 < *(int *)(iVar17 + 4));
        }
        pfVar15 = pfVar15 + MULI(pfVar3[4], 0x10);
        FUN_0048dba0(pfVar15, pfVar3[5], pfVar3[4], param_3);
        fVar12 = pfVar3[4];
        iVar17 = MULI(DAT_0060268c, 3);
        *(int *)pfVar3[5] = fVar10;
        pfVar9 = pfVar15 + MULI(fVar12, 9) + iVar17;
        if (DAT_006a8884 < fVar10) {
          DAT_006a8884 = fVar10;
        }
        break;
      case 17:
        fVar10 = *local_d8;
        pfVar9 = param_4 + 1;
        piVar19 = *(int **)(iVar17 + 0x14);
        if (piVar19 != (int *)0) {
          local_e0 = 0;
          pfVar3[4] = 0;
          pfVar15 = pfVar9;
          /* NOTE: local_d0[0] is read uninitialised here, as in the original */
          fVar12 = local_d0[0];
          if (0 < *(int *)(iVar17 + 0x10)) {
            do {
              uVar7 = 2;
              if (2 < *piVar19) {
                do {
                  fVar12 = ADDI(fVar10, (int)uVar7);
                  if ((uVar7 & 1) == 0) {
                    *pfVar15 = SUBI(fVar12, 2);
                    pfVar15[1] = SUBI(fVar12, 1);
                    pfVar15[2] = fVar12;
                    pfVar15[3] = -1;
                    if (uVar7 == 2) {
                      pfVar15[4] = local_d0[0];
                      local_d0[0] = ADDI(local_d0[0], 1);
                    } else {
                      pfVar15[4] = SUBI(local_d0[0], 2);
                    }
                    pfVar15[5] = local_d0[0];
                  } else {
                    *pfVar15 = fVar12;
                    pfVar15[1] = SUBI(fVar12, 1);
                    pfVar15[2] = SUBI(fVar12, 2);
                    pfVar15[3] = -1;
                    pfVar15[4] = local_d0[0];
                    pfVar15[5] = SUBI(local_d0[0], 2);
                  }
                  pfVar15[6] = ADDI(local_d0[0], 1);
                  local_d0[0] = ADDI(local_d0[0], 2);
                  uVar7 = uVar7 + 1;
                  pfVar15[7] = -1;
                  pfVar15[8] = *pfVar15;
                  pfVar15[9] = pfVar15[1];
                  pfVar15[10] = pfVar15[2];
                  pfVar15[0xb] = pfVar15[3];
                  pfVar15[0xc] = *pfVar15;
                  pfVar15[0xd] = pfVar15[1];
                  pfVar15[0xe] = pfVar15[2];
                  pfVar15[0xf] = pfVar15[3];
                  pfVar15 = pfVar15 + 0x10;
                  pfVar3[4] = ADDI(pfVar3[4], 1);
                } while ((int)uVar7 < *piVar19);
              }
              iVar11 = *piVar19;
              piVar19 = piVar19 + 1;
              fVar10 = ADDI(fVar10, iVar11);
              local_e0 = ADDI(local_e0, 1);
              fVar12 = local_d0[0];
            } while (local_e0 < *(int *)(iVar17 + 0x10));
          }
          pfVar9 = pfVar9 + MULI(pfVar3[4], 0x10);
          FUN_0048dba0(pfVar9, pfVar3[5], pfVar3[4], param_3);
          fVar10 = pfVar3[4];
          iVar17 = MULI(DAT_0060268c, 3);
          *(int *)pfVar3[5] = fVar12;
          pfVar9 = pfVar9 + MULI(fVar10, 9) + iVar17;
          if (DAT_006a8884 < fVar12) {
            DAT_006a8884 = fVar12;
          }
        }
        break;
      case 20:
        pfVar3[5] = (int)param_4;
        pbVar5 = FUN_0048e580(param_4, (char *)*puVar1);
        pfVar9 = (int *)((char *)param_4 + (int)pbVar5);
        break;
      case 21:
        iVar11 = 0;
        iVar6 = 0;
        pfVar3[4] = 0;
        if (0 < *(int *)(iVar17 + 4)) {
          do {
            if ((*(int *)ADDI(param_2[3], iVar11) != 0) &&
                ((iVar11 == 0 || (local_d8[-1] != *local_d8)))) {
              *param_4 = *local_d8;
              *(int *)ADDI(param_2[3], MULI(pfVar3[4], 4)) = *(int *)ADDI(iVar11, param_2[3]);
              pfVar3[4] = ADDI(pfVar3[4], 1);
              param_4 = param_4 + 1;
            }
            iVar6 = ADDI(iVar6, 1);
            local_d8 = local_d8 + 1;
            iVar11 = ADDI(iVar11, 4);
          } while (iVar6 < *(int *)(iVar17 + 4));
        }
        param_2[2] = pfVar3[4];
        pfVar9 = param_4;
        break;
      case 22:
        *param_4 = *local_d8;
        pfVar9 = *(int **)(iVar17 + 0x14);
        param_4[1] = *pfVar9;
        param_4[2] = pfVar9[1];
        param_4[3] = pfVar9[2];
        pfVar9 = param_4 + 4;
        break;
      case 23:
        *param_4 = *local_d8;
        param_4[1] = local_d8[1];
        param_4[2] = local_d8[2];
        pfVar9 = *(int **)(iVar17 + 0x14);
        param_4[3] = *pfVar9;
        param_4[4] = pfVar9[1];
        param_4[5] = pfVar9[2];
        pfVar9 = *(int **)(iVar17 + 0x20);
        param_4[6] = *pfVar9;
        param_4[7] = pfVar9[1];
        param_4[8] = pfVar9[2];
        pfVar9 = *(int **)(iVar17 + 0x2c);
        param_4[9] = *pfVar9;
        param_4[10] = pfVar9[1];
        param_4[0xb] = pfVar9[2];
        pfVar9 = param_4 + 0xc;
        break;
      case 25:
        *param_4 = *local_d8;
        param_4[1] = **(int **)(iVar17 + 0x14);
        pfVar9 = *(int **)(iVar17 + 0x20);
        param_4[2] = *pfVar9;
        param_4[3] = pfVar9[1];
        param_4[4] = pfVar9[2];
        pfVar9 = *(int **)(iVar17 + 0x2c);
        param_4[5] = *pfVar9;
        param_4[6] = pfVar9[1];
        param_4[7] = pfVar9[2];
        pfVar9 = *(int **)(iVar17 + 0x38);
        param_4[8] = *pfVar9;
        param_4[9] = pfVar9[1];
        param_4[10] = pfVar9[2];
        pfVar9 = *(int **)(iVar17 + 0x44);
        param_4[0xb] = *pfVar9;
        param_4[0xc] = pfVar9[1];
        param_4[0xd] = pfVar9[2];
        param_4[0xe] = **(int **)(iVar17 + 0x50);
        pfVar9 = *(int **)(iVar17 + 0x5c);
        param_4[0xf] = *pfVar9;
        param_4[0x10] = pfVar9[1];
        param_4[0x11] = pfVar9[2];
        pfVar9 = param_4 + 0x12;
      }
    }
  }
  param_4 = pfVar9;
  iVar17 = 0;
  if (param_2[2] != 0) {
    pfVar9 = local_d0;
    for (iVar11 = 0x34; iVar11 != 0; iVar11 = SUBI(iVar11, 1)) {
      *pfVar9 = *(int *)param_3;
      param_3 = ADDI(param_3, 4);
      pfVar9 = pfVar9 + 1;
    }
    DAT_006026c0 = (float *)0;
    DAT_006026a8 = 0;
    DAT_00602690 = 0;
    iVar11 = 0;
    pfVar3[2] = param_2[2];
    pfVar3[3] = (int)param_4;
    param_4 = param_4 + param_2[2];
    if (0 < param_2[2]) {
      do {
        iVar17 = ADDI(iVar17, 4);
        iVar11 = ADDI(iVar11, 1);
        *(int **)ADDI(SUBI(pfVar3[3], 4), iVar17) = param_4;
        iVar6 = FUN_0048c240(param_1, *(int **)ADDI(SUBI(param_2[3], 4), iVar17), (int)local_d0,
                             param_4, param_5, param_6);
        param_4 = (int *)((char *)param_4 + iVar6);
      } while (iVar11 < param_2[2]);
    }
  }
  return SUBI((int)param_4, (int)pfVar3);
}
