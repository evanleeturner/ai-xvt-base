/* SPDX-License-Identifier: MIT */
/* z_bop 00477c80 */
extern int *DAT_0060f1c0;
extern int DAT_0060f1d8;
extern int *DAT_0060f1e8;
extern int DAT_0060f1f0;
extern int DAT_0060f204;
extern int *DAT_0060f208;
extern int DAT_0099947e;
extern int DAT_00999488;

int __cdecl FUN_004792f0(int, int, int);
int *__cdecl FUN_004794c0(int, int *, int);
void __cdecl FUN_00421300(float *, float *);
void __cdecl FUN_00420fe0(float *, float *);
unsigned char *__cdecl FUN_00479e80(int *, char *);
int __cdecl FUN_00477c80(unsigned int param_1, int *param_2, int param_3, int *param_4,
                         unsigned int *param_5, unsigned char *param_6);

int __cdecl FUN_00477c80(unsigned int param_1, int *param_2, int param_3, int *param_4,
                         unsigned int *param_5, unsigned char *param_6)
{
  unsigned int *puVar1;
  char cVar2;
  float fVar3;
  int *pfVar4;
  int *pfVar5;
  unsigned char *pbVar6;
  int iVar7;
  unsigned int uVar8;
  unsigned int uVar9;
  int fVar10;
  int fVar11;
  int fVar12;
  int fVar13;
  int iVar14;
  int *pfVar15;
  int iVar16;
  unsigned int *unaff_EBP = (unsigned int *)0;
  int *pfVar17;
  int fVar18;
  int fVar19;
  unsigned char *unaff_EDI = (unsigned char *)0;
  int iVar20;
  char *pcVar21;
  char *pcVar22;
  int iVar23;
  int iVar3;
  int *piVar24;
  int local_e4;
  int local_dc;
  int *local_d8;
  int local_d0[52];

  pfVar4 = param_4;
  if (param_2 == (int *)0x0) {
    return 0;
  }
  pfVar5 = param_4 + 6;
  fVar13 = param_2[1];
  param_4[1] = fVar13;
  if (*param_2 == 0) {
    *param_4 = 0;
    param_4 = pfVar5;
  }
  else {
    *param_4 = (int)pfVar5;
    uVar8 = 0xffffffff;
    pcVar22 = (char *)*param_2;
    do {
      pcVar21 = pcVar22;
      if (uVar8 == 0) break;
      uVar8 = uVar8 - 1;
      pcVar21 = pcVar22 + 1;
      cVar2 = *pcVar22;
      pcVar22 = pcVar21;
    } while (cVar2 != '\0');
    uVar8 = ~uVar8;
    pfVar15 = (int *)(pcVar21 - uVar8);
    pfVar17 = pfVar5;
    for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
      *pfVar17 = *pfVar15;
      pfVar15 = pfVar15 + 1;
      pfVar17 = pfVar17 + 1;
    }
    for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
      *(char *)pfVar17 = *(char *)pfVar15;
      pfVar15 = (int *)((int)pfVar15 + 1);
      pfVar17 = (int *)((int)pfVar17 + 1);
    }
    uVar8 = 0xffffffff;
    pcVar22 = (char *)*param_2;
    do {
      if (uVar8 == 0) break;
      uVar8 = uVar8 - 1;
      cVar2 = *pcVar22;
      pcVar22 = pcVar22 + 1;
    } while (cVar2 != '\0');
    param_4 = (int *)((unsigned int)pfVar5 + ~uVar8);
  }
  pfVar4[4] = 1;
  pfVar4[5] = (int)param_4;
  pfVar4[2] = 0;
  pfVar4[3] = 0;
  iVar16 = param_2[5];
  pfVar5 = param_4;
  if ((iVar16 != 0) && (param_2[4] != 0)) {
    local_d8 = *(int **)((unsigned int)iVar16 + 8U);
    puVar1 = (unsigned int *)((unsigned int)iVar16 + 8U);
    if (local_d8 != (int *)0x0) {
      switch (fVar13) {
      case 1:
        iVar7 = 0;
        fVar13 = 0;
        pfVar4[4] = 0;
        iVar14 = *(int *)(iVar16 + 4);
        if (*(int *)(iVar16 + 0x28) == iVar14) {
          local_dc = *(int *)(iVar16 + 0x2c);
        }
        else {
          local_dc = 0;
        }
        iVar23 = 0;
        if (*(int *)(iVar16 + 0x1c) == iVar14) {
          iVar23 = *(int *)(iVar16 + 0x20);
        }
        pfVar5 = param_4 + 1;
        if (0 < iVar14) {
          do {
            iVar14 = (int)((unsigned int)iVar7 + 1U);
            do {
              iVar20 = iVar14;
              iVar14 = (int)((unsigned int)iVar20 + 2U);
              *pfVar5 = local_d8[iVar7];
              pfVar5[1] = local_d8[iVar20];
              pfVar5[2] = local_d8[(int)((unsigned int)iVar20 + 1U)];
              pfVar5[3] = local_d8[iVar14];
              if (local_dc == 0) {
                pfVar5[8] = *pfVar5;
                pfVar5[9] = pfVar5[1];
                pfVar5[10] = pfVar5[2];
                fVar10 = pfVar5[3];
              }
              else {
                pfVar5[8] = *(int *)((unsigned int)local_dc + (unsigned int)iVar7 * 4U);
                pfVar5[9] = *(int *)((unsigned int)local_dc - 8U + (unsigned int)iVar14 * 4U);
                pfVar5[10] = *(int *)((unsigned int)local_dc - 4U + (unsigned int)iVar14 * 4U);
                fVar10 = *(int *)((unsigned int)local_dc + (unsigned int)iVar14 * 4U);
              }
              pfVar5[0xb] = fVar10;
              if (iVar23 == 0) {
                pfVar5[0xc] = *pfVar5;
                pfVar5[0xd] = pfVar5[1];
                pfVar5[0xe] = pfVar5[2];
                pfVar5[0xf] = pfVar5[3];
              }
              else {
                pfVar5[0xc] = *(int *)((unsigned int)iVar23 + (unsigned int)iVar7 * 4U);
                pfVar5[0xd] = *(int *)((unsigned int)iVar23 - 8U + (unsigned int)iVar14 * 4U);
                pfVar5[0xe] = *(int *)((unsigned int)iVar23 - 4U + (unsigned int)iVar14 * 4U);
                pfVar5[0xf] = *(int *)((unsigned int)iVar23 + (unsigned int)iVar14 * 4U);
              }
              fVar10 = FUN_004792f0((int)pfVar4, *pfVar5, pfVar5[1]);
              pfVar5[4] = fVar10;
              if (fVar10 == -1) {
                pfVar5[4] = fVar13;
                fVar13 = (int)((unsigned int)fVar13 + 1U);
              }
              fVar10 = FUN_004792f0((int)pfVar4, pfVar5[1], pfVar5[2]);
              pfVar5[5] = fVar10;
              if (fVar10 == -1) {
                pfVar5[5] = fVar13;
                fVar13 = (int)((unsigned int)fVar13 + 1U);
              }
              if (pfVar5[3] == -1) {
                fVar10 = FUN_004792f0((int)pfVar4, pfVar5[2], *pfVar5);
                pfVar5[6] = fVar10;
                if (fVar10 == -1) {
                  pfVar5[6] = fVar13;
                  fVar13 = (int)((unsigned int)fVar13 + 1U);
                }
                pfVar5[7] = -1;
              }
              else {
                fVar10 = FUN_004792f0((int)pfVar4, pfVar5[2], pfVar5[3]);
                pfVar5[6] = fVar10;
                if (fVar10 == -1) {
                  pfVar5[6] = fVar13;
                  fVar13 = (int)((unsigned int)fVar13 + 1U);
                }
                fVar10 = FUN_004792f0((int)pfVar4, pfVar5[3], *pfVar5);
                pfVar5[7] = fVar10;
                if (fVar10 == -1) {
                  pfVar5[7] = fVar13;
                  fVar13 = (int)((unsigned int)fVar13 + 1U);
                }
              }
              pfVar5 = pfVar5 + 0x10;
              pfVar4[4] = (int)((unsigned int)pfVar4[4] + 1U);
              if (local_d8[iVar14] == -1) {
                iVar7 = (int)((unsigned int)iVar20 + 3U);
                goto LAB_00477f99;
              }
            } while (local_d8[(int)((unsigned int)iVar20 + 3U)] != -1);
            iVar7 = (int)((unsigned int)iVar20 + 4U);
LAB_00477f99:
            ;
          } while (iVar7 < *(int *)(iVar16 + 4));
        }
        pfVar5 = FUN_004794c0((int)pfVar4, param_4 + 1, param_3);
        *(int *)pfVar4[5] = fVar13;
        if (DAT_00999488 < fVar13) {
          DAT_00999488 = fVar13;
        }
        break;
      case 2:
        pfVar5 = param_4 + 0xc;
        pfVar15 = *(int **)(iVar16 + 0x38);
        if (pfVar15 != (int *)0x0) {
          fVar3 = -*(float *)pfVar15;
          *(float *)param_4 = fVar3;
          ((float *)param_4)[1] = -((float *)pfVar15)[1];
          fVar13 = param_4[1];
          ((float *)param_4)[2] = -((float *)pfVar15)[2];
          fVar10 = param_4[2];
          if (*(float **)(iVar16 + 0x2c) != (float *)0x0) {
            pfVar15 = param_4 + 3;
            FUN_00421300((float *)pfVar15, *(float **)(iVar16 + 0x2c));
            FUN_00420fe0((float *)param_4, (float *)pfVar15);
            *(float *)param_4 = *(float *)param_4 - fVar3;
            ((float *)param_4)[1] = ((float *)param_4)[1] - *(float *)&fVar13;
            ((float *)param_4)[2] = ((float *)param_4)[2] - *(float *)&fVar10;
            pfVar17 = *(int **)(iVar16 + 0x20);
            if (pfVar17 != (int *)0x0) {
              ((float *)param_4)[3] = *(float *)pfVar17 * ((float *)param_4)[3];
              ((float *)param_4)[4] = ((float *)pfVar17)[1] * ((float *)param_4)[4];
              ((float *)param_4)[5] = ((float *)param_4)[5] * ((float *)pfVar17)[2];
              ((float *)param_4)[6] = *(float *)pfVar17 * ((float *)param_4)[6];
              ((float *)param_4)[7] = ((float *)param_4)[7] * ((float *)pfVar17)[1];
              ((float *)param_4)[8] = ((float *)param_4)[8] * ((float *)pfVar17)[2];
              ((float *)param_4)[9] = *(float *)pfVar17 * ((float *)param_4)[9];
              ((float *)param_4)[10] = ((float *)param_4)[10] * ((float *)pfVar17)[1];
              ((float *)param_4)[0xb] = ((float *)param_4)[0xb] * ((float *)pfVar17)[2];
              *(float *)param_4 = *(float *)pfVar17 * *(float *)param_4;
              ((float *)param_4)[1] = ((float *)param_4)[1] * ((float *)pfVar17)[1];
              ((float *)param_4)[2] = ((float *)param_4)[2] * ((float *)pfVar17)[2];
              pfVar17 = *(int **)(iVar16 + 0x14);
              if (pfVar17 != (int *)0x0) {
                *(float *)param_4 = *(float *)param_4 + fVar3;
                ((float *)param_4)[1] = ((float *)param_4)[1] + *(float *)&fVar13;
                ((float *)param_4)[2] = ((float *)param_4)[2] + *(float *)&fVar10;
                FUN_00421300((float *)pfVar15, (float *)pfVar17);
                FUN_00420fe0((float *)param_4, (float *)pfVar15);
                *(float *)param_4 = *(float *)param_4 - fVar3;
                ((float *)param_4)[1] = ((float *)param_4)[1] - *(float *)&fVar13;
                ((float *)param_4)[2] = ((float *)param_4)[2] - *(float *)&fVar10;
                pfVar15 = *(int **)(iVar16 + 8);
                if (pfVar15 != (int *)0x0) {
                  *(float *)param_4 = *(float *)param_4 - *(float *)pfVar15;
                  ((float *)param_4)[1] = ((float *)param_4)[1] - ((float *)pfVar15)[1];
                  ((float *)param_4)[2] = ((float *)param_4)[2] - ((float *)pfVar15)[2];
                }
              }
            }
          }
        }
        break;
      case 3:
        pfVar4[4] = *(int *)(iVar16 + 4);
        pfVar5 = (int *)*puVar1;
        pfVar15 = param_4;
        for (uVar8 = ((unsigned int)*(int *)(iVar16 + 4) * 0xcU) >> 2; uVar8 != 0;
             uVar8 = uVar8 - 1) {
          *pfVar15 = *pfVar5;
          pfVar5 = pfVar5 + 1;
          pfVar15 = pfVar15 + 1;
        }
        DAT_0060f1c0 = param_4;
        DAT_0060f204 = *(int *)(iVar16 + 4);
        pfVar5 = param_4 + (unsigned int)DAT_0060f204 * 3U;
        __builtin_memcpy(&iVar7, &DAT_0099947e, 4);
        if (iVar7 < pfVar4[4]) {
          __builtin_memcpy(&DAT_0099947e, &pfVar4[4], 4);
        }
        break;
      case 4:
        *param_4 = *local_d8;
        param_4[1] = local_d8[1];
        param_4[2] = local_d8[2];
        pfVar5 = param_4 + 3;
        break;
      case 5:
        FUN_00421300((float *)param_4, (float *)local_d8);
        pfVar5 = param_4 + 9;
        break;
      case 6:
        *param_4 = *local_d8;
        param_4[1] = local_d8[1];
        param_4[2] = local_d8[2];
        pfVar5 = param_4 + 3;
        break;
      case 7:
        if (iVar16 != 0) {
          uVar8 = 0xffffffff;
          pfVar4[4] = 1;
          pfVar4[5] = (int)param_4;
          pcVar22 = (char *)*puVar1;
          do {
            pcVar21 = pcVar22;
            if (uVar8 == 0) break;
            uVar8 = uVar8 - 1;
            pcVar21 = pcVar22 + 1;
            cVar2 = *pcVar22;
            pcVar22 = pcVar21;
          } while (cVar2 != '\0');
          uVar8 = ~uVar8;
          pfVar5 = (int *)(pcVar21 - uVar8);
          pfVar15 = param_4;
          for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
            *pfVar15 = *pfVar5;
            pfVar5 = pfVar5 + 1;
            pfVar15 = pfVar15 + 1;
          }
          for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
            *(char *)pfVar15 = *(char *)pfVar5;
            pfVar5 = (int *)((int)pfVar5 + 1);
            pfVar15 = (int *)((int)pfVar15 + 1);
          }
          uVar8 = 0xffffffff;
          pcVar22 = (char *)*puVar1;
          do {
            if (uVar8 == 0) break;
            uVar8 = uVar8 - 1;
            cVar2 = *pcVar22;
            pcVar22 = pcVar22 + 1;
          } while (cVar2 != '\0');
          pfVar5 = (int *)((unsigned int)param_4 + ~uVar8);
        }
        break;
      case 9:
        fVar13 = *(int *)(iVar16 + 4);
        if (*(int *)(iVar16 + 4) < *(int *)(iVar16 + 0x10)) {
          fVar13 = *(int *)(iVar16 + 0x10);
        }
        if (fVar13 < *(int *)(iVar16 + 0x1c)) {
          fVar13 = *(int *)(iVar16 + 0x1c);
        }
        if (fVar13 < *(int *)(iVar16 + 0x28)) {
          fVar13 = *(int *)(iVar16 + 0x28);
        }
        if (fVar13 < *(int *)(iVar16 + 0x34)) {
          fVar13 = *(int *)(iVar16 + 0x34);
        }
        if (fVar13 < *(int *)(iVar16 + 0x40)) {
          fVar13 = *(int *)(iVar16 + 0x40);
        }
        pfVar4[4] = fVar13;
        pfVar5 = *(int **)(iVar16 + 8);
        pfVar15 = param_4;
        for (uVar8 = ((unsigned int)*(int *)(iVar16 + 4) * 0xcU) >> 2; uVar8 != 0;
             uVar8 = uVar8 - 1) {
          *pfVar15 = *pfVar5;
          pfVar5 = pfVar5 + 1;
          pfVar15 = pfVar15 + 1;
        }
        iVar14 = *(int *)(iVar16 + 4);
        pfVar5 = param_4 + (unsigned int)iVar14 * 3U;
        param_4 = pfVar5;
        if (iVar14 < fVar13) {
          pfVar15 = (int *)((unsigned int)*(int *)(iVar16 + 8) - 0xcU + (unsigned int)iVar14 * 0xcU);
          iVar7 = (int)((unsigned int)fVar13 - (unsigned int)iVar14);
          param_4 = pfVar5 + (unsigned int)iVar7 * 3U;
          if (iVar14 < fVar13) {
            do {
              *pfVar5 = *pfVar15;
              iVar7 = (int)((unsigned int)iVar7 - 1U);
              pfVar5[1] = pfVar15[1];
              pfVar5[2] = pfVar15[2];
              pfVar5 = pfVar5 + 3;
            } while (iVar7 != 0);
          }
        }
        pfVar5 = param_4;
        if (*(int **)(iVar16 + 0x14) != (int *)0x0) {
          pfVar5 = *(int **)(iVar16 + 0x14);
          pfVar15 = param_4;
          for (uVar8 = ((unsigned int)*(int *)(iVar16 + 0x10) * 0xcU) >> 2; uVar8 != 0;
               uVar8 = uVar8 - 1) {
            *pfVar15 = *pfVar5;
            pfVar5 = pfVar5 + 1;
            pfVar15 = pfVar15 + 1;
          }
          iVar14 = *(int *)(iVar16 + 0x10);
          pfVar5 = param_4 + (unsigned int)iVar14 * 3U;
          param_4 = pfVar5;
          if (iVar14 < fVar13) {
            pfVar15 = (int *)((unsigned int)*(int *)(iVar16 + 0x14) - 0xcU +
                              (unsigned int)iVar14 * 0xcU);
            iVar7 = (int)((unsigned int)fVar13 - (unsigned int)iVar14);
            param_4 = pfVar5 + (unsigned int)iVar7 * 3U;
            if (iVar14 < fVar13) {
              do {
                *pfVar5 = *pfVar15;
                iVar7 = (int)((unsigned int)iVar7 - 1U);
                pfVar5[1] = pfVar15[1];
                pfVar5[2] = pfVar15[2];
                pfVar5 = pfVar5 + 3;
              } while (iVar7 != 0);
            }
          }
          pfVar5 = param_4;
          if (*(int **)(iVar16 + 0x20) != (int *)0x0) {
            pfVar5 = *(int **)(iVar16 + 0x20);
            pfVar15 = param_4;
            for (uVar8 = ((unsigned int)*(int *)(iVar16 + 0x1c) * 0xcU) >> 2; uVar8 != 0;
                 uVar8 = uVar8 - 1) {
              *pfVar15 = *pfVar5;
              pfVar5 = pfVar5 + 1;
              pfVar15 = pfVar15 + 1;
            }
            iVar14 = *(int *)(iVar16 + 0x1c);
            pfVar5 = param_4 + (unsigned int)iVar14 * 3U;
            param_4 = pfVar5;
            if (iVar14 < fVar13) {
              pfVar15 = (int *)((unsigned int)*(int *)(iVar16 + 0x20) - 0xcU +
                                (unsigned int)iVar14 * 0xcU);
              iVar7 = (int)((unsigned int)fVar13 - (unsigned int)iVar14);
              param_4 = pfVar5 + (unsigned int)iVar7 * 3U;
              if (iVar14 < fVar13) {
                do {
                  *pfVar5 = *pfVar15;
                  iVar7 = (int)((unsigned int)iVar7 - 1U);
                  pfVar5[1] = pfVar15[1];
                  pfVar5[2] = pfVar15[2];
                  pfVar5 = pfVar5 + 3;
                } while (iVar7 != 0);
              }
            }
            pfVar5 = param_4;
            if (*(int **)(iVar16 + 0x2c) != (int *)0x0) {
              pfVar5 = *(int **)(iVar16 + 0x2c);
              pfVar15 = param_4;
              for (uVar8 = ((unsigned int)*(int *)(iVar16 + 0x28) * 0xcU) >> 2; uVar8 != 0;
                   uVar8 = uVar8 - 1) {
                *pfVar15 = *pfVar5;
                pfVar5 = pfVar5 + 1;
                pfVar15 = pfVar15 + 1;
              }
              iVar14 = *(int *)(iVar16 + 0x28);
              pfVar5 = param_4 + (unsigned int)iVar14 * 3U;
              param_4 = pfVar5;
              if (iVar14 < fVar13) {
                pfVar15 = (int *)((unsigned int)*(int *)(iVar16 + 0x2c) - 0xcU +
                                  (unsigned int)iVar14 * 0xcU);
                iVar7 = (int)((unsigned int)fVar13 - (unsigned int)iVar14);
                param_4 = pfVar5 + (unsigned int)iVar7 * 3U;
                if (iVar14 < fVar13) {
                  do {
                    *pfVar5 = *pfVar15;
                    iVar7 = (int)((unsigned int)iVar7 - 1U);
                    pfVar5[1] = pfVar15[1];
                    pfVar5[2] = pfVar15[2];
                    pfVar5 = pfVar5 + 3;
                  } while (iVar7 != 0);
                }
              }
              pfVar5 = param_4;
              if (*(int **)(iVar16 + 0x38) != (int *)0x0) {
                pfVar5 = *(int **)(iVar16 + 0x38);
                pfVar15 = param_4;
                for (uVar8 = *(unsigned int *)(iVar16 + 0x34) & 0x3fffffffU; uVar8 != 0;
                     uVar8 = uVar8 - 1) {
                  *pfVar15 = *pfVar5;
                  pfVar5 = pfVar5 + 1;
                  pfVar15 = pfVar15 + 1;
                }
                iVar14 = *(int *)(iVar16 + 0x34);
                pfVar5 = param_4 + (unsigned int)iVar14;
                param_4 = pfVar5;
                if (iVar14 < fVar13) {
                  iVar7 = *(int *)(iVar16 + 0x38);
                  iVar23 = (int)((unsigned int)fVar13 - (unsigned int)iVar14);
                  param_4 = pfVar5 + (unsigned int)iVar23;
                  if (iVar14 < fVar13) {
                    do {
                      iVar23 = (int)((unsigned int)iVar23 - 1U);
                      *pfVar5 = *(int *)((unsigned int)iVar7 - 4U + (unsigned int)iVar14 * 4U);
                      pfVar5 = pfVar5 + 1;
                    } while (iVar23 != 0);
                  }
                }
                pfVar5 = param_4;
                if (*(int **)(iVar16 + 0x44) != (int *)0x0) {
                  pfVar5 = *(int **)(iVar16 + 0x44);
                  pfVar15 = param_4;
                  for (uVar8 = *(unsigned int *)(iVar16 + 0x40) & 0x3fffffffU; uVar8 != 0;
                       uVar8 = uVar8 - 1) {
                    *pfVar15 = *pfVar5;
                    pfVar5 = pfVar5 + 1;
                    pfVar15 = pfVar15 + 1;
                  }
                  iVar14 = *(int *)(iVar16 + 0x40);
                  pfVar5 = param_4 + (unsigned int)iVar14;
                  if (iVar14 < fVar13) {
                    iVar16 = *(int *)(iVar16 + 0x44);
                    iVar7 = (int)((unsigned int)fVar13 - (unsigned int)iVar14);
                    pfVar15 = pfVar5 + (unsigned int)iVar7;
                    pfVar17 = pfVar5;
                    pfVar5 = pfVar15;
                    if (iVar14 < fVar13) {
                      do {
                        iVar7 = (int)((unsigned int)iVar7 - 1U);
                        *pfVar17 = *(int *)((unsigned int)iVar16 - 4U + (unsigned int)iVar14 * 4U);
                        pfVar17 = pfVar17 + 1;
                      } while (iVar7 != 0);
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
        pfVar4[4] = *local_d8;
        break;
      case 11:
        pfVar4[4] = *(int *)(iVar16 + 4);
        pfVar5 = (int *)*puVar1;
        pfVar15 = param_4;
        for (uVar8 = ((unsigned int)*(int *)(iVar16 + 4) * 0xcU) >> 2; uVar8 != 0;
             uVar8 = uVar8 - 1) {
          *pfVar15 = *pfVar5;
          pfVar5 = pfVar5 + 1;
          pfVar15 = pfVar15 + 1;
        }
        DAT_0060f208 = param_4;
        *(int **)((unsigned int)param_3 + 0x84U) = param_4;
        pfVar5 = param_4 + (unsigned int)*(int *)(iVar16 + 4) * 3U;
        break;
      case 13:
        pfVar4[4] = *(int *)(iVar16 + 4);
        pfVar5 = (int *)*puVar1;
        pfVar15 = param_4;
        for (iVar14 = (int)((*(unsigned int *)(iVar16 + 4) & 0x1fffffffU) << 1); iVar14 != 0;
             iVar14 = (int)((unsigned int)iVar14 - 1U)) {
          *pfVar15 = *pfVar5;
          pfVar5 = pfVar5 + 1;
          pfVar15 = pfVar15 + 1;
        }
        DAT_0060f1e8 = param_4;
        pfVar5 = param_4 + *(unsigned int *)(iVar16 + 4) * 2U;
        break;
      case 15:
        local_e4 = *local_d8;
        pfVar5 = param_4 + 1;
        iVar14 = 0;
        if (*(int **)(iVar16 + 0x14) != (int *)0x0) {
          iVar7 = **(int **)(iVar16 + 0x14);
          if (*(int **)(iVar16 + 0x20) != (int *)0x0) {
            iVar16 = **(int **)(iVar16 + 0x20);
            fVar13 = 0;
            pfVar15 = pfVar5;
            local_d8 = (int *)local_e4;
            if (iVar16 != 1 && -1 < (int)((unsigned int)iVar16 - 1U)) {
              do {
                local_d8 = (int *)((unsigned int)local_d8 + (unsigned int)iVar7);
                iVar23 = 0;
                if (iVar7 != 1 && -1 < (int)((unsigned int)iVar7 - 1U)) {
                  do {
                    *pfVar15 = (int)((unsigned int)local_e4 + (unsigned int)iVar23);
                    pfVar15[1] = (int)((unsigned int)local_e4 + (unsigned int)iVar23 + 1U);
                    pfVar15[2] = (int)((unsigned int)local_d8 + (unsigned int)iVar23 + 1U);
                    pfVar15[3] = (int)((unsigned int)local_d8 + (unsigned int)iVar23);
                    pfVar15[8] = *pfVar15;
                    pfVar15[9] = pfVar15[1];
                    pfVar15[10] = pfVar15[2];
                    pfVar15[0xb] = pfVar15[3];
                    pfVar15[0xc] = *pfVar15;
                    pfVar15[0xd] = pfVar15[1];
                    pfVar15[0xe] = pfVar15[2];
                    pfVar15[0xf] = pfVar15[3];
                    if (iVar14 == 0) {
                      pfVar15[4] = fVar13;
                      fVar10 = (int)((unsigned int)fVar13 + 1U);
                    }
                    else {
                      if (iVar14 == 1) {
                        fVar10 = (int)((unsigned int)fVar13 + (unsigned int)iVar7 * 0xfffffffdU +
                                       (unsigned int)iVar23 + 4U);
                      }
                      else {
                        fVar10 = (int)((unsigned int)fVar13 + (unsigned int)iVar7 * 0xfffffffeU);
                      }
                      pfVar15[4] = fVar10;
                      fVar10 = fVar13;
                    }
                    pfVar15[5] = fVar10;
                    pfVar15[6] = (int)((unsigned int)fVar10 + 1U);
                    pfVar17 = pfVar15 + 7;
                    fVar13 = (int)((unsigned int)fVar10 + 2U);
                    if (iVar23 == 0) {
                      *pfVar17 = fVar13;
                      fVar13 = (int)((unsigned int)fVar10 + 3U);
                    }
                    else {
                      *pfVar17 = (int)((unsigned int)fVar10 - 2U);
                      if (iVar14 == 0) {
                        *pfVar17 = (int)((unsigned int)fVar10 - 3U);
                      }
                      if (iVar23 == 1) {
                        *pfVar17 = (int)((unsigned int)*pfVar17 - 1U);
                      }
                    }
                    pfVar15 = pfVar15 + 0x10;
                    iVar23 = (int)((unsigned int)iVar23 + 1U);
                  } while (iVar23 < (int)((unsigned int)iVar7 - 1U));
                }
                local_e4 = (int)((unsigned int)local_e4 + (unsigned int)iVar7);
                iVar14 = (int)((unsigned int)iVar14 + 1U);
              } while (iVar14 < (int)((unsigned int)iVar16 - 1U));
            }
            pfVar4[4] = (int)(((unsigned int)iVar7 - 1U) * ((unsigned int)iVar16 - 1U));
            pfVar5 = FUN_004794c0((int)pfVar4, pfVar5, param_3);
            *(int *)pfVar4[5] = fVar13;
            if (DAT_00999488 < fVar13) {
              DAT_00999488 = fVar13;
            }
          }
        }
        break;
      case 16:
        fVar13 = 0;
        fVar10 = 0;
        local_e4 = 0;
        pfVar4[4] = 0;
        pfVar5 = param_4 + 1;
        if (0 < *(int *)(iVar16 + 4)) {
          do {
            iVar3 = *local_d8;
            fVar12 = fVar10;
            if (2 < iVar3) {
              iVar14 = (int)(((unsigned int)fVar10 + 1U) - (unsigned int)fVar10);
              fVar12 = (int)((unsigned int)fVar10 + 1U);
              fVar18 = (int)((unsigned int)fVar13 + 1U);
              fVar19 = fVar13;
              while (fVar13 = fVar18, iVar14 < iVar3) {
                pfVar4[4] = (int)((unsigned int)pfVar4[4] + 1U);
                *pfVar5 = fVar10;
                pfVar5[1] = fVar12;
                pfVar5[2] = (int)((unsigned int)fVar12 + 1U);
                fVar11 = (int)((unsigned int)fVar12 + 2U);
                if ((int)((unsigned int)fVar11 - (unsigned int)iVar3) == fVar10) {
                  pfVar5[3] = -1;
                  pfVar5[4] = fVar19;
                  pfVar5[5] = fVar13;
                  pfVar5[6] = (int)((unsigned int)fVar13 + 1U);
                  fVar18 = (int)((unsigned int)fVar13 + 2U);
                  pfVar5[7] = -1;
                }
                else {
                  pfVar5[3] = fVar11;
                  fVar11 = (int)((unsigned int)fVar12 + 3U);
                  if ((int)((unsigned int)fVar11 - (unsigned int)iVar3) != fVar10) {
                    fVar11 = (int)((unsigned int)fVar12 + 2U);
                  }
                  pfVar5[4] = fVar19;
                  pfVar5[5] = fVar13;
                  pfVar5[6] = (int)((unsigned int)fVar13 + 1U);
                  fVar19 = (int)((unsigned int)fVar13 + 2U);
                  pfVar5[7] = fVar19;
                  fVar18 = (int)((unsigned int)fVar13 + 3U);
                }
                pfVar5[8] = *pfVar5;
                pfVar5[9] = pfVar5[1];
                pfVar5[10] = pfVar5[2];
                pfVar5[0xb] = pfVar5[3];
                pfVar5[0xc] = *pfVar5;
                pfVar5[0xd] = pfVar5[1];
                pfVar5[0xe] = pfVar5[2];
                pfVar5[0xf] = pfVar5[3];
                pfVar5 = pfVar5 + 0x10;
                fVar12 = fVar11;
                iVar14 = (int)((unsigned int)fVar11 - (unsigned int)fVar10);
              }
            }
            local_d8 = local_d8 + 1;
            local_e4 = (int)((unsigned int)local_e4 + 1U);
            fVar10 = fVar12;
          } while (local_e4 < *(int *)(iVar16 + 4));
        }
        pfVar5 = FUN_004794c0((int)pfVar4, param_4 + 1, param_3);
        *(int *)pfVar4[5] = fVar13;
        if (DAT_00999488 < fVar13) {
          DAT_00999488 = fVar13;
        }
        break;
      case 17:
        fVar13 = *local_d8;
        pfVar5 = param_4 + 1;
        piVar24 = *(int **)(iVar16 + 0x14);
        if (piVar24 != (int *)0x0) {
          local_e4 = 0;
          pfVar4[4] = 0;
          pfVar15 = pfVar5;
          fVar10 = local_d0[0];
          if (0 < *(int *)(iVar16 + 0x10)) {
            do {
              uVar8 = 2;
              if (2 < *piVar24) {
                do {
                  fVar10 = (int)(uVar8 + (unsigned int)fVar13);
                  if ((uVar8 & 1) == 0) {
                    *pfVar15 = (int)((unsigned int)fVar10 - 2U);
                    pfVar15[1] = (int)((unsigned int)fVar10 - 1U);
                    pfVar15[2] = fVar10;
                    pfVar15[3] = -1;
                    if (uVar8 == 2) {
                      pfVar15[4] = local_d0[0];
                      local_d0[0] = (int)((unsigned int)local_d0[0] + 1U);
                    }
                    else {
                      pfVar15[4] = (int)((unsigned int)local_d0[0] - 2U);
                    }
                    pfVar15[5] = local_d0[0];
                  }
                  else {
                    *pfVar15 = fVar10;
                    pfVar15[1] = (int)((unsigned int)fVar10 - 1U);
                    pfVar15[2] = (int)((unsigned int)fVar10 - 2U);
                    pfVar15[3] = -1;
                    pfVar15[4] = local_d0[0];
                    pfVar15[5] = (int)((unsigned int)local_d0[0] - 2U);
                  }
                  pfVar15[6] = (int)((unsigned int)local_d0[0] + 1U);
                  local_d0[0] = (int)((unsigned int)local_d0[0] + 2U);
                  uVar8 = uVar8 + 1;
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
                  pfVar4[4] = (int)((unsigned int)pfVar4[4] + 1U);
                } while ((int)uVar8 < *piVar24);
              }
              iVar14 = *piVar24;
              piVar24 = piVar24 + 1;
              fVar13 = (int)((unsigned int)fVar13 + (unsigned int)iVar14);
              local_e4 = (int)((unsigned int)local_e4 + 1U);
              fVar10 = local_d0[0];
            } while (local_e4 < *(int *)(iVar16 + 0x10));
          }
          pfVar5 = FUN_004794c0((int)pfVar4, pfVar5, param_3);
          *(int *)pfVar4[5] = fVar10;
          if (DAT_00999488 < fVar10) {
            DAT_00999488 = fVar10;
          }
        }
        break;
      case 20:
        pfVar4[5] = (int)param_4;
        pbVar6 = FUN_00479e80(param_4, (char *)*puVar1);
        pfVar5 = (int *)((unsigned int)param_4 + (unsigned int)pbVar6);
        break;
      case 21:
        iVar14 = 0;
        iVar7 = 0;
        pfVar4[4] = 0;
        if (0 < *(int *)(iVar16 + 4)) {
          do {
            if ((*(int *)((unsigned int)param_2[3] + (unsigned int)iVar14) != 0) &&
                ((iVar14 == 0 || (((float *)local_d8)[-1] != *(float *)local_d8)))) {
              *param_4 = *local_d8;
              *(int *)((unsigned int)param_2[3] + (unsigned int)pfVar4[4] * 4U) =
                  *(int *)((unsigned int)param_2[3] + (unsigned int)iVar14);
              pfVar4[4] = (int)((unsigned int)pfVar4[4] + 1U);
              param_4 = param_4 + 1;
            }
            iVar7 = (int)((unsigned int)iVar7 + 1U);
            local_d8 = local_d8 + 1;
            iVar14 = (int)((unsigned int)iVar14 + 4U);
          } while (iVar7 < *(int *)(iVar16 + 4));
        }
        param_2[2] = pfVar4[4];
        pfVar5 = param_4;
        break;
      case 22:
        *param_4 = *local_d8;
        pfVar5 = *(int **)(iVar16 + 0x14);
        param_4[1] = *pfVar5;
        param_4[2] = pfVar5[1];
        param_4[3] = pfVar5[2];
        pfVar5 = param_4 + 4;
        break;
      case 23:
        *param_4 = *local_d8;
        param_4[1] = local_d8[1];
        param_4[2] = local_d8[2];
        pfVar5 = *(int **)(iVar16 + 0x14);
        param_4[3] = *pfVar5;
        param_4[4] = pfVar5[1];
        param_4[5] = pfVar5[2];
        pfVar5 = *(int **)(iVar16 + 0x20);
        param_4[6] = *pfVar5;
        param_4[7] = pfVar5[1];
        param_4[8] = pfVar5[2];
        pfVar5 = *(int **)(iVar16 + 0x2c);
        param_4[9] = *pfVar5;
        param_4[10] = pfVar5[1];
        param_4[0xb] = pfVar5[2];
        pfVar5 = param_4 + 0xc;
        break;
      case 25:
        *param_4 = *local_d8;
        param_4[1] = **(int **)(iVar16 + 0x14);
        pfVar5 = *(int **)(iVar16 + 0x20);
        param_4[2] = *pfVar5;
        param_4[3] = pfVar5[1];
        param_4[4] = pfVar5[2];
        pfVar5 = *(int **)(iVar16 + 0x2c);
        param_4[5] = *pfVar5;
        param_4[6] = pfVar5[1];
        param_4[7] = pfVar5[2];
        pfVar5 = *(int **)(iVar16 + 0x38);
        param_4[8] = *pfVar5;
        param_4[9] = pfVar5[1];
        param_4[10] = pfVar5[2];
        pfVar5 = *(int **)(iVar16 + 0x44);
        param_4[0xb] = *pfVar5;
        param_4[0xc] = pfVar5[1];
        param_4[0xd] = pfVar5[2];
        param_4[0xe] = **(int **)(iVar16 + 0x50);
        pfVar5 = *(int **)(iVar16 + 0x5c);
        param_4[0xf] = *pfVar5;
        param_4[0x10] = pfVar5[1];
        param_4[0x11] = pfVar5[2];
        pfVar5 = param_4 + 0x12;
        break;
      }
    }
  }
  param_4 = pfVar5;
  iVar16 = 0;
  if (param_2[2] != 0) {
    pfVar5 = local_d0;
    for (iVar14 = 0x34; iVar14 != 0; iVar14 = (int)((unsigned int)iVar14 - 1U)) {
      *pfVar5 = *(int *)param_3;
      param_3 = (int)((unsigned int)param_3 + 4U);
      pfVar5 = pfVar5 + 1;
    }
    DAT_0060f208 = (int *)0;
    DAT_0060f1f0 = 0;
    DAT_0060f1d8 = 0;
    iVar14 = 0;
    pfVar4[2] = param_2[2];
    pfVar4[3] = (int)param_4;
    param_4 = param_4 + (unsigned int)param_2[2];
    if (0 < param_2[2]) {
      do {
        iVar16 = (int)((unsigned int)iVar16 + 4U);
        iVar14 = (int)((unsigned int)iVar14 + 1U);
        *(int **)((unsigned int)pfVar4[3] - 4U + (unsigned int)iVar16) = param_4;
        iVar7 = FUN_00477c80(param_1,
                             *(int **)((unsigned int)param_2[3] - 4U + (unsigned int)iVar16),
                             (int)local_d0, param_4, unaff_EBP, unaff_EDI);
        param_4 = (int *)((unsigned int)param_4 + (unsigned int)iVar7);
      } while (iVar14 < param_2[2]);
    }
  }
  return (int)((unsigned int)param_4 - (unsigned int)pfVar4);
}
