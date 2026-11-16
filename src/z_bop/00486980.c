/* SPDX-License-Identifier: MIT */
/* z_bop 00486980 */
extern unsigned short DAT_009d682c;
extern int DAT_00523404;
extern unsigned char DAT_00999492[4]; /* packed, address 0x999492 (ptr) */
extern unsigned char DAT_0099942a[4]; /* packed, address 0x99942a (ptr) */
extern unsigned char DAT_0099942e[4]; /* packed, address 0x99942e (ptr) */

/* operand is passed in st0 in the original; no stack arguments */
long long __cdecl __ftol(void);

void __cdecl FUN_00486980(float param_1, float param_2, unsigned int param_3, int param_4)
{
  int *piVar1;
  int *piVar2;
  int *piVar3;
  float *pfVar4;
  float *pfVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  unsigned int uVar9;
  int iVar10;
  int iVar11;
  int iVar12;
  int iVar13;
  int iVar14;
  unsigned int *puVar15;
  int iVar16;
  unsigned int *puVar17;
  int iVar18;
  unsigned int *puVar19;
  unsigned int *puVar20;
  unsigned int *puVar21;
  long long lVar22;
  unsigned int *local_24;
  unsigned int local_20;
  int local_18;
  int iTbl;
  unsigned int bits;
  unsigned int *pEnd;
  float *fp4;

  fp4 = (float *)param_4;
  ((unsigned int *)((int *)param_4)[26])[param_3 - ((unsigned int *)param_4)[16]] = 0;
  __builtin_memcpy(&bits, &param_1, 4);
  if (bits < 0x80000001u) {
    lVar22 = __ftol();
    local_18 = (int)lVar22;
    if ((float)local_18 != param_1) {
      local_18 = (int)((unsigned int)local_18 + 1u);
    }
  } else {
    local_18 = 0;
  }
  __builtin_memcpy(&bits, &param_2, 4);
  if (bits < 0x80000001u) {
    lVar22 = __ftol();
    local_20 = (unsigned int)lVar22;
    if ((float)(int)local_20 != param_2) {
      local_20 = local_20 + 1;
    }
  } else {
    local_20 = 0;
  }
  uVar9 = (unsigned int)DAT_009d682c;
  if ((int)uVar9 <= (int)local_20) {
    local_20 = uVar9;
  }
  if ((int)local_20 <= local_18) {
    return;
  }
  if ((int)uVar9 <= local_18) {
    return;
  }
  if ((DAT_00523404 != 0) && ((param_3 & 1) != 0)) {
    return;
  }
  local_24 = (unsigned int *)0;
  __builtin_memcpy(&iTbl, DAT_00999492, 4);
  puVar21 = ((unsigned int **)iTbl)[param_3];
  puVar20 = local_24;
  iVar10 = local_18;
  while (puVar21 != (unsigned int *)0) {
    iVar16 = (int)puVar21[2];
    piVar1 = (int *)(puVar21 + 2);
    local_24 = puVar21;
    local_18 = iVar10;
    if (iVar10 < iVar16) {
      iVar13 = (int)puVar21[1];
      piVar2 = (int *)(puVar21 + 1);
      if (iVar10 < iVar13) break;
      iVar11 = (int)puVar21[5];
      piVar3 = (int *)(puVar21 + 5);
      if (fp4[18] <= ((float *)iVar11)[19]) {
joined_r0x00486f6b:
        local_18 = iVar16;
        if ((int)local_20 <= iVar16) {
          return;
        }
        goto LAB_00487018;
      }
      if (((float *)iVar11)[18] <= fp4[19]) {
        if ((iVar16 <= (int)local_20) && (*piVar1 = iVar10, *piVar2 == iVar10)) {
          ((unsigned int *)((int *)(*piVar3))[26])[param_3 - ((unsigned int *)(*piVar3))[16]] = 0;
          if (puVar20 == (unsigned int *)0) {
            __builtin_memcpy(&iTbl, DAT_00999492, 4);
            ((unsigned int **)iTbl)[param_3] = (unsigned int *)*puVar21;
            local_24 = puVar20;
          } else {
            *puVar20 = *puVar21;
            local_24 = puVar20;
          }
        }
        goto LAB_00487018;
      }
      fVar6 = fp4[10] * (float)iVar10 + fp4[11] * (float)(int)param_3 + fp4[12];
      pfVar4 = (float *)(iVar11 + 0x28);
      fVar7 = ((float *)iVar11)[10] * (float)iVar10 +
              (float)(int)param_3 * ((float *)iVar11)[11] + ((float *)iVar11)[12];
      if (fVar6 <= fVar7) {
        if (*pfVar4 < fp4[10]) {
          if (iVar16 < (int)local_20) {
            local_18 = iVar16;
            if ((float)(int)((unsigned int)iVar16 - (unsigned int)iVar10) * fp4[10] + fVar6 <=
                *pfVar4 * (float)(int)((unsigned int)iVar16 - (unsigned int)iVar10) + fVar7)
              goto LAB_00487018;
          } else if ((float)(int)(local_20 - (unsigned int)iVar10) * fp4[10] + fVar6 <=
                     *pfVar4 * (float)(int)(local_20 - (unsigned int)iVar10) + fVar7) {
            return;
          }
          lVar22 = __ftol();
          iVar16 = (int)((unsigned int)(int)lVar22 + 1u);
          if (iVar16 < 0) {
            iVar16 = 0;
          }
          iVar16 = (int)((unsigned int)iVar10 + (unsigned int)iVar16);
        }
        goto joined_r0x00486f6b;
      }
      if (*pfVar4 <= fp4[10]) {
        if ((iVar16 <= (int)local_20) && (*piVar1 = iVar10, *piVar2 == iVar10)) {
          ((unsigned int *)((int *)(*piVar3))[26])[param_3 - ((unsigned int *)(*piVar3))[16]] = 0;
          if (puVar20 == (unsigned int *)0) {
            __builtin_memcpy(&iTbl, DAT_00999492, 4);
            ((unsigned int **)iTbl)[param_3] = (unsigned int *)*puVar21;
            local_24 = puVar20;
          } else {
            *puVar20 = *puVar21;
            local_24 = puVar20;
          }
        }
        goto LAB_00487018;
      }
      if ((int)local_20 < iVar16) {
        if ((float)(int)(local_20 - (unsigned int)iVar10) * fp4[10] + fVar6 <
            *pfVar4 * (float)(int)(local_20 - (unsigned int)iVar10) + fVar7) {
          lVar22 = __ftol();
          iVar16 = (int)((unsigned int)(int)lVar22 + 1u);
          if (iVar16 < 0) {
            iVar16 = 0;
          }
          uVar9 = (unsigned int)iVar16 + (unsigned int)iVar10;
          if ((int)local_20 < (int)uVar9) {
            uVar9 = local_20;
          }
joined_r0x0048700e:
          local_20 = uVar9;
          if ((int)local_20 <= iVar10) {
            return;
          }
        }
        goto LAB_00487018;
      }
      iVar11 = (int)((unsigned int)iVar16 - (unsigned int)iVar10);
      if ((float)iVar11 * *pfVar4 + fVar7 <= fp4[10] * (float)iVar11 + fVar6) {
        *piVar1 = iVar10;
        if (*piVar2 == iVar10) {
          ((unsigned int *)((int *)(*piVar3))[26])[param_3 - ((unsigned int *)(*piVar3))[16]] = 0;
          if (puVar20 == (unsigned int *)0) {
            __builtin_memcpy(&iTbl, DAT_00999492, 4);
            ((unsigned int **)iTbl)[param_3] = (unsigned int *)*puVar21;
            local_24 = puVar20;
          } else {
            *puVar20 = *puVar21;
            local_24 = puVar20;
          }
        }
        goto LAB_00487018;
      }
      lVar22 = __ftol();
      iVar12 = (int)lVar22;
      if (iVar12 < 0) {
        iVar12 = 0;
      }
      if (iVar11 < iVar12) {
        iVar12 = iVar11;
      }
      iVar11 = (int)((unsigned int)iVar11 - (unsigned int)iVar12);
      iVar14 = (int)((unsigned int)iVar10 - (unsigned int)iVar13);
      iVar18 = (int)(local_20 - (unsigned int)iVar16);
      if (((iVar12 < iVar14) || (iVar11 < iVar14)) || (iVar18 < iVar14)) {
        if (((iVar14 < iVar12) || (iVar11 < iVar12)) || (iVar18 < iVar12)) {
          if (((iVar11 <= iVar14) && (iVar11 <= iVar12)) && (iVar11 <= iVar18))
            goto joined_r0x00486f6b;
          uVar9 = (unsigned int)iVar16 - (unsigned int)iVar12;
          goto joined_r0x0048700e;
        }
        *piVar1 = iVar10;
        if (*piVar2 == iVar10) {
          ((unsigned int *)((int *)(*piVar3))[26])[param_3 - ((unsigned int *)(*piVar3))[16]] = 0;
          if (puVar20 == (unsigned int *)0) {
            __builtin_memcpy(&iTbl, DAT_00999492, 4);
            ((unsigned int **)iTbl)[param_3] = (unsigned int *)*puVar21;
            local_24 = puVar20;
          } else {
            *puVar20 = *puVar21;
            local_24 = puVar20;
          }
        }
        goto LAB_00487018;
      }
      iVar12 = (int)(((unsigned int)iVar16 - (unsigned int)iVar13) - (unsigned int)iVar12);
      *piVar2 = (int)((unsigned int)iVar12 + (unsigned int)iVar13);
      ((float *)puVar21)[3] = (float)iVar12 * ((float *)puVar21)[4] + ((float *)puVar21)[3];
      puVar15 = (unsigned int *)*puVar21;
      if ((puVar15 == (unsigned int *)0) || (*piVar2 < (int)puVar15[1])) break;
      if (puVar20 == (unsigned int *)0) {
        __builtin_memcpy(&iTbl, DAT_00999492, 4);
        ((unsigned int **)iTbl)[param_3] = puVar15;
      } else {
        *puVar20 = (unsigned int)puVar15;
      }
      iVar16 = *piVar2;
      if (iVar16 < (int)puVar15[2]) {
        iVar13 = (int)(puVar15[2] - (unsigned int)iVar16);
        *piVar2 = (int)((unsigned int)iVar16 + (unsigned int)iVar13);
        ((float *)puVar21)[3] = (float)iVar13 * ((float *)puVar21)[4] + ((float *)puVar21)[3];
      }
      if (*piVar2 < *piVar1) {
        puVar17 = (unsigned int *)*puVar15;
        puVar19 = puVar17;
        if (puVar17 != (unsigned int *)0) {
          while (iVar16 = *piVar2, puVar19 = puVar17, (int)puVar17[1] < iVar16) {
            if (iVar16 < (int)puVar17[2]) {
              iVar13 = (int)(puVar17[2] - (unsigned int)iVar16);
              *piVar2 = (int)((unsigned int)iVar16 + (unsigned int)iVar13);
              ((float *)puVar21)[3] = (float)iVar13 * ((float *)puVar21)[4] + ((float *)puVar21)[3];
            }
            if ((*piVar1 <= *piVar2) ||
                (puVar19 = (unsigned int *)*puVar17, puVar15 = puVar17, puVar17 = puVar19,
                 puVar19 == (unsigned int *)0)) break;
          }
        }
        if (*piVar1 <= *piVar2) goto LAB_00486eb2;
        *puVar15 = (unsigned int)puVar21;
        *puVar21 = (unsigned int)puVar19;
      } else {
LAB_00486eb2:
        ((unsigned int *)((int *)(*piVar3))[26])[param_3 - ((unsigned int *)(*piVar3))[16]] = 0;
      }
      if (puVar20 == (unsigned int *)0) {
        __builtin_memcpy(&iTbl, DAT_00999492, 4);
        puVar21 = ((unsigned int **)iTbl)[param_3];
      } else {
        puVar21 = (unsigned int *)*puVar20;
      }
    } else {
LAB_00487018:
      puVar21 = (unsigned int *)*puVar21;
      puVar20 = local_24;
      iVar10 = local_18;
    }
  }
  __builtin_memcpy(&puVar15, DAT_0099942a, 4);
  __builtin_memcpy(&pEnd, DAT_0099942e, 4);
  puVar17 = puVar15 + 6;
  if (puVar15 + 6 == pEnd) {
    puVar17 = puVar15;
  }
  __builtin_memcpy(DAT_0099942a, &puVar17, 4);
  puVar15[1] = (unsigned int)iVar10;
  puVar15[2] = local_20;
  puVar15[5] = (unsigned int)param_4;
  if (((int *)param_4)[14] == 0) {
    puVar15[3] = 0;
  } else {
    ((float *)puVar15)[3] = (float)iVar10;
    ((float *)puVar15)[3] = ((float *)puVar15)[3] - ((float *)((int *)param_4)[14])[2];
    ((float *)puVar15)[3] = ((float *)puVar15)[3] * fp4[13];
    ((float *)puVar15)[3] = ((float *)((int *)param_4)[14])[3] + ((float *)puVar15)[3];
  }
  puVar15[4] = ((unsigned int *)param_4)[13];
  ((unsigned int *)((int *)param_4)[26])[param_3 - ((unsigned int *)param_4)[16]] =
      (unsigned int)puVar15;
  if (puVar20 == (unsigned int *)0) {
    __builtin_memcpy(&iTbl, DAT_00999492, 4);
    ((unsigned int **)iTbl)[param_3] = puVar15;
  } else {
    *puVar20 = (unsigned int)puVar15;
  }
  local_24 = puVar15;
  *puVar15 = (unsigned int)puVar21;
  if (puVar21 != (unsigned int *)0) {
    for (;;) {
      iVar10 = (int)puVar21[1];
      piVar1 = (int *)(puVar21 + 1);
      iVar16 = (int)puVar15[2];
      if (iVar16 <= iVar10) break;
      iVar13 = (int)puVar21[5];
      piVar2 = (int *)(puVar21 + 5);
      if (((float *)iVar13)[19] < fp4[18]) {
        if (fp4[19] < ((float *)iVar13)[18]) {
          fVar6 = fp4[10] * (float)iVar10 + fp4[11] * (float)(int)param_3 + fp4[12];
          pfVar4 = (float *)(iVar13 + 0x28);
          fVar7 = ((float *)iVar13)[10] * (float)iVar10 +
                  (float)(int)param_3 * ((float *)iVar13)[11] + ((float *)iVar13)[12];
          if (fVar7 < fVar6) {
            piVar3 = (int *)(puVar21 + 2);
            if (fp4[10] < *pfVar4) {
              if (iVar16 < *piVar3) {
                iVar16 = (int)((unsigned int)iVar16 - (unsigned int)iVar10);
                fVar8 = (float)iVar16;
                if (fVar8 * fp4[10] + fVar6 < *pfVar4 * fVar8 + fVar7) {
                  lVar22 = __ftol();
                  iVar13 = (int)lVar22;
                  if (iVar13 < 0) {
                    iVar13 = 0;
                  }
                  if (iVar16 < iVar13) {
                    iVar13 = iVar16;
                  }
                  pfVar4 = (float *)(puVar21 + 3);
                  pfVar5 = (float *)(puVar21 + 4);
                  *piVar1 = (int)((unsigned int)iVar10 + (unsigned int)iVar13);
                  *pfVar4 = (float)iVar13 * *pfVar5 + *pfVar4;
                  puVar20 = (unsigned int *)*puVar21;
                  if ((puVar20 != (unsigned int *)0) && ((int)puVar20[1] < *piVar1)) {
                    *local_24 = (unsigned int)puVar20;
                    iVar10 = *piVar1;
                    if (iVar10 < (int)puVar20[2]) {
                      iVar16 = (int)(puVar20[2] - (unsigned int)iVar10);
                      *piVar1 = (int)((unsigned int)iVar10 + (unsigned int)iVar16);
                      *pfVar4 = (float)iVar16 * *pfVar5 + *pfVar4;
                    }
                    if (*piVar3 <= *piVar1) goto LAB_00487946;
                    puVar17 = (unsigned int *)*puVar20;
                    if (puVar17 != (unsigned int *)0) {
                      while (puVar19 = puVar17, iVar10 = *piVar1, puVar17 = puVar19,
                             (int)puVar19[1] < iVar10) {
                        if (iVar10 < (int)puVar19[2]) {
                          iVar16 = (int)(puVar19[2] - (unsigned int)iVar10);
                          *piVar1 = (int)((unsigned int)iVar10 + (unsigned int)iVar16);
                          *pfVar4 = (float)iVar16 * *pfVar5 + *pfVar4;
                        }
                        if ((*piVar3 <= *piVar1) ||
                            (puVar17 = (unsigned int *)*puVar19, puVar20 = puVar19,
                             puVar17 == (unsigned int *)0)) break;
                      }
                    }
                    if (*piVar3 <= *piVar1) goto LAB_00487925;
LAB_00487919:
                    *puVar20 = (unsigned int)puVar21;
                    *puVar21 = (unsigned int)puVar17;
                    puVar20 = (unsigned int *)*local_24;
                    puVar21 = local_24;
                  }
                } else {
                  pfVar4 = (float *)(puVar21 + 3);
                  pfVar5 = (float *)(puVar21 + 4);
                  *piVar1 = (int)((unsigned int)iVar10 + (unsigned int)iVar16);
                  *pfVar4 = *pfVar5 * fVar8 + *pfVar4;
                  puVar20 = (unsigned int *)*puVar21;
                  if ((puVar20 != (unsigned int *)0) && ((int)puVar20[1] < *piVar1)) {
                    *local_24 = (unsigned int)puVar20;
                    iVar10 = *piVar1;
                    if (iVar10 < (int)puVar20[2]) {
                      iVar16 = (int)(puVar20[2] - (unsigned int)iVar10);
                      *piVar1 = (int)((unsigned int)iVar10 + (unsigned int)iVar16);
                      *pfVar4 = (float)iVar16 * *pfVar5 + *pfVar4;
                    }
                    if (*piVar1 < *piVar3) {
                      puVar17 = (unsigned int *)*puVar20;
                      if (puVar17 != (unsigned int *)0) {
                        while (puVar19 = puVar17, iVar10 = *piVar1, puVar17 = puVar19,
                               (int)puVar19[1] < iVar10) {
                          if (iVar10 < (int)puVar19[2]) {
                            iVar16 = (int)(puVar19[2] - (unsigned int)iVar10);
                            *piVar1 = (int)((unsigned int)iVar10 + (unsigned int)iVar16);
                            *pfVar4 = (float)iVar16 * *pfVar5 + *pfVar4;
                          }
                          if ((*piVar3 <= *piVar1) ||
                              (puVar17 = (unsigned int *)*puVar19, puVar20 = puVar19,
                               puVar17 == (unsigned int *)0)) break;
                        }
                      }
                      if (*piVar1 < *piVar3) goto LAB_00487919;
LAB_00487925:
                      ((unsigned int *)((int *)(*piVar2))[26])[param_3 - ((unsigned int *)(*piVar2))[16]] = 0;
                      puVar20 = (unsigned int *)*local_24;
                      puVar21 = local_24;
                    } else {
LAB_00487946:
                      ((unsigned int *)((int *)(*piVar2))[26])[param_3 - ((unsigned int *)(*piVar2))[16]] = 0;
                      puVar20 = (unsigned int *)*local_24;
                      puVar21 = local_24;
                    }
                  }
                }
              } else {
                iVar16 = (int)((unsigned int)*piVar3 - (unsigned int)iVar10);
                fVar8 = (float)iVar16;
                if (fp4[10] * fVar8 + fVar6 < fVar8 * *pfVar4 + fVar7) {
                  lVar22 = __ftol();
                  iVar13 = (int)lVar22;
                  if (iVar13 < 0) {
                    iVar13 = 0;
                  }
                  iVar16 = (int)((unsigned int)iVar16 - (unsigned int)iVar13);
                  if (iVar16 < 0) {
                    iVar16 = 0;
                  }
                  pfVar4 = (float *)(puVar21 + 3);
                  pfVar5 = (float *)(puVar21 + 4);
                  *piVar1 = (int)((unsigned int)iVar10 + (unsigned int)iVar16);
                  *pfVar4 = (float)iVar16 * *pfVar5 + *pfVar4;
                  if (*piVar1 < *piVar3) {
                    puVar20 = (unsigned int *)*puVar21;
                    if ((puVar20 != (unsigned int *)0) && ((int)puVar20[1] < *piVar1)) {
                      *local_24 = (unsigned int)puVar20;
                      iVar10 = *piVar1;
                      if (iVar10 < (int)puVar20[2]) {
                        iVar16 = (int)(puVar20[2] - (unsigned int)iVar10);
                        *piVar1 = (int)((unsigned int)iVar10 + (unsigned int)iVar16);
                        *pfVar4 = (float)iVar16 * *pfVar5 + *pfVar4;
                      }
                      if (*piVar1 < *piVar3) {
                        puVar17 = (unsigned int *)*puVar20;
                        if (puVar17 != (unsigned int *)0) {
                          while (puVar19 = puVar17, iVar10 = *piVar1, puVar17 = puVar19,
                                 (int)puVar19[1] < iVar10) {
                            if (iVar10 < (int)puVar19[2]) {
                              iVar16 = (int)(puVar19[2] - (unsigned int)iVar10);
                              *piVar1 = (int)((unsigned int)iVar10 + (unsigned int)iVar16);
                              *pfVar4 = (float)iVar16 * *pfVar5 + *pfVar4;
                            }
                            if ((*piVar3 <= *piVar1) ||
                                (puVar17 = (unsigned int *)*puVar19, puVar20 = puVar19,
                                 puVar17 == (unsigned int *)0)) break;
                          }
                        }
                        if (*piVar3 <= *piVar1) goto LAB_00487925;
                        goto LAB_00487919;
                      }
                      goto LAB_00487946;
                    }
                  } else {
                    ((unsigned int *)((int *)(*piVar2))[26])[param_3 - ((unsigned int *)(*piVar2))[16]] = 0;
                    *local_24 = *puVar21;
                    puVar20 = (unsigned int *)*puVar21;
                    puVar21 = local_24;
                  }
                } else {
                  ((unsigned int *)((int *)iVar13)[26])[param_3 - ((unsigned int *)iVar13)[16]] = 0;
                  *local_24 = *puVar21;
                  puVar20 = (unsigned int *)*puVar21;
                  puVar21 = local_24;
                }
              }
            } else if (iVar16 < *piVar3) {
              pfVar4 = (float *)(puVar21 + 3);
              pfVar5 = (float *)(puVar21 + 4);
              *piVar1 = (int)((unsigned int)iVar10 + (unsigned int)(int)((unsigned int)iVar16 - (unsigned int)iVar10));
              *pfVar4 = (float)(int)((unsigned int)iVar16 - (unsigned int)iVar10) * *pfVar5 + *pfVar4;
              puVar20 = (unsigned int *)*puVar21;
              if ((puVar20 != (unsigned int *)0) && ((int)puVar20[1] < *piVar1)) {
                *local_24 = (unsigned int)puVar20;
                iVar10 = *piVar1;
                if (iVar10 < (int)puVar20[2]) {
                  iVar16 = (int)(puVar20[2] - (unsigned int)iVar10);
                  *piVar1 = (int)((unsigned int)iVar10 + (unsigned int)iVar16);
                  *pfVar4 = (float)iVar16 * *pfVar5 + *pfVar4;
                }
                if (*piVar1 < *piVar3) {
                  puVar17 = (unsigned int *)*puVar20;
                  if (puVar17 != (unsigned int *)0) {
                    while (puVar19 = puVar17, iVar10 = *piVar1, puVar17 = puVar19,
                           (int)puVar19[1] < iVar10) {
                      if (iVar10 < (int)puVar19[2]) {
                        iVar16 = (int)(puVar19[2] - (unsigned int)iVar10);
                        *piVar1 = (int)((unsigned int)iVar10 + (unsigned int)iVar16);
                        *pfVar4 = (float)iVar16 * *pfVar5 + *pfVar4;
                      }
                      if ((*piVar3 <= *piVar1) ||
                          (puVar17 = (unsigned int *)*puVar19, puVar20 = puVar19,
                           puVar17 == (unsigned int *)0)) break;
                    }
                  }
                  if (*piVar3 <= *piVar1) goto LAB_00487925;
                  goto LAB_00487919;
                }
                goto LAB_00487946;
              }
            } else {
              ((unsigned int *)((int *)iVar13)[26])[param_3 - ((unsigned int *)iVar13)[16]] = 0;
              *local_24 = *puVar21;
              puVar20 = (unsigned int *)*puVar21;
              puVar21 = local_24;
            }
          } else if (*pfVar4 < fp4[10]) {
            iVar13 = (int)puVar21[2];
            piVar3 = (int *)(puVar21 + 2);
            if (iVar13 < iVar16) {
              if ((float)(int)((unsigned int)iVar13 - (unsigned int)iVar10) * *pfVar4 + fVar7 <
                  fp4[10] * (float)(int)((unsigned int)iVar13 - (unsigned int)iVar10) + fVar6) {
                lVar22 = __ftol();
                iVar10 = (int)lVar22;
                if (iVar10 < 0) {
                  iVar10 = 0;
                }
                *piVar3 = (int)((unsigned int)iVar13 - (unsigned int)iVar10);
                if (*piVar1 < (int)((unsigned int)iVar13 - (unsigned int)iVar10)) {
                  puVar20 = (unsigned int *)*puVar21;
                } else {
                  ((unsigned int *)((int *)(*piVar2))[26])[param_3 - ((unsigned int *)(*piVar2))[16]] = 0;
                  *local_24 = *puVar21;
                  puVar20 = (unsigned int *)*puVar21;
                  puVar21 = local_24;
                }
              } else {
                puVar20 = (unsigned int *)*puVar21;
              }
            } else {
              iVar11 = (int)((unsigned int)iVar16 - (unsigned int)iVar10);
              fVar8 = (float)iVar11;
              if (fVar8 * fp4[10] + fVar6 <= *pfVar4 * fVar8 + fVar7) {
                puVar15[2] = puVar21[1];
                return;
              }
              lVar22 = __ftol();
              iVar12 = (int)lVar22;
              if (iVar12 < 0) {
                iVar12 = 0;
              }
              if (iVar11 < iVar12) {
                iVar12 = iVar11;
              }
              if ((iVar12 < (int)((unsigned int)iVar11 - (unsigned int)iVar12)) &&
                  (iVar12 < (int)((unsigned int)iVar13 - (unsigned int)iVar16))) {
                puVar15[2] = puVar21[1];
                return;
              }
              if ((int)((unsigned int)iVar11 - (unsigned int)iVar12) <
                  (int)((unsigned int)iVar13 - (unsigned int)iVar16)) {
                pfVar4 = (float *)(puVar21 + 3);
                *piVar1 = (int)((unsigned int)iVar11 + (unsigned int)iVar10);
                pfVar5 = (float *)(puVar21 + 4);
                *pfVar4 = *pfVar5 * fVar8 + *pfVar4;
                puVar20 = (unsigned int *)*puVar21;
                if ((puVar20 != (unsigned int *)0) && ((int)puVar20[1] < *piVar1)) {
                  *local_24 = (unsigned int)puVar20;
                  iVar10 = *piVar1;
                  if (iVar10 < (int)puVar20[2]) {
                    iVar16 = (int)(puVar20[2] - (unsigned int)iVar10);
                    *piVar1 = (int)((unsigned int)iVar10 + (unsigned int)iVar16);
                    *pfVar4 = (float)iVar16 * *pfVar5 + *pfVar4;
                  }
                  if (*piVar1 < *piVar3) {
                    puVar17 = (unsigned int *)*puVar20;
                    if (puVar17 != (unsigned int *)0) {
                      while (puVar19 = puVar17, iVar10 = *piVar1, puVar17 = puVar19,
                             (int)puVar19[1] < iVar10) {
                        if (iVar10 < (int)puVar19[2]) {
                          iVar16 = (int)(puVar19[2] - (unsigned int)iVar10);
                          *piVar1 = (int)((unsigned int)iVar10 + (unsigned int)iVar16);
                          *pfVar4 = (float)iVar16 * *pfVar5 + *pfVar4;
                        }
                        if ((*piVar3 <= *piVar1) ||
                            (puVar17 = (unsigned int *)*puVar19, puVar20 = puVar19,
                             puVar17 == (unsigned int *)0)) break;
                      }
                    }
                    if (*piVar3 <= *piVar1) goto LAB_00487925;
                    goto LAB_00487919;
                  }
                  goto LAB_00487946;
                }
              } else {
                *piVar3 = (int)((unsigned int)iVar16 - (unsigned int)iVar12);
                puVar20 = (unsigned int *)*puVar21;
              }
            }
          } else {
            if (iVar16 <= (int)puVar21[2]) {
              puVar15[2] = puVar21[1];
              return;
            }
            puVar20 = (unsigned int *)*puVar21;
          }
        } else {
          piVar3 = (int *)(puVar21 + 2);
          if (iVar16 < (int)puVar21[2]) {
            *piVar1 = (int)((unsigned int)iVar10 + (unsigned int)(int)((unsigned int)iVar16 - (unsigned int)iVar10));
            ((float *)puVar21)[3] = (float)(int)((unsigned int)iVar16 - (unsigned int)iVar10) *
                                        ((float *)puVar21)[4] + ((float *)puVar21)[3];
            puVar20 = (unsigned int *)*puVar21;
            if ((puVar20 != (unsigned int *)0) && ((int)puVar20[1] < *piVar1)) {
              *local_24 = (unsigned int)puVar20;
              iVar10 = *piVar1;
              if (iVar10 < (int)puVar20[2]) {
                iVar16 = (int)(puVar20[2] - (unsigned int)iVar10);
                *piVar1 = (int)((unsigned int)iVar10 + (unsigned int)iVar16);
                ((float *)puVar21)[3] = (float)iVar16 * ((float *)puVar21)[4] + ((float *)puVar21)[3];
              }
              if (*piVar1 < *piVar3) {
                puVar17 = (unsigned int *)*puVar20;
                if (puVar17 != (unsigned int *)0) {
                  while (puVar19 = puVar17, iVar10 = *piVar1, puVar17 = puVar19,
                         (int)puVar19[1] < iVar10) {
                    if (iVar10 < (int)puVar19[2]) {
                      iVar16 = (int)(puVar19[2] - (unsigned int)iVar10);
                      *piVar1 = (int)((unsigned int)iVar10 + (unsigned int)iVar16);
                      ((float *)puVar21)[3] = (float)iVar16 * ((float *)puVar21)[4] + ((float *)puVar21)[3];
                    }
                    if ((*piVar3 <= *piVar1) ||
                        (puVar17 = (unsigned int *)*puVar19, puVar20 = puVar19,
                         puVar17 == (unsigned int *)0)) break;
                  }
                }
                if (*piVar3 <= *piVar1) goto LAB_00487925;
                goto LAB_00487919;
              }
              goto LAB_00487946;
            }
          } else {
            ((unsigned int *)((int *)iVar13)[26])[param_3 - ((unsigned int *)iVar13)[16]] = 0;
            *local_24 = *puVar21;
            puVar20 = (unsigned int *)*puVar21;
            puVar21 = local_24;
          }
        }
      } else {
        if (iVar16 <= (int)puVar21[2]) {
          puVar15[2] = puVar21[1];
          return;
        }
        puVar20 = (unsigned int *)*puVar21;
      }
      local_24 = puVar21;
      puVar21 = puVar20;
      if (puVar20 == (unsigned int *)0) {
        return;
      }
    }
  }
  return;
}
