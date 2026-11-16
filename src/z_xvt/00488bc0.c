/* SPDX-License-Identifier: MIT */
/* z_xvt 00488bc0 */
extern int *DAT_006026b8;
extern int DAT_006026bc;
extern int DAT_0052a54c;
extern int *DAT_006026b0;
extern int *DAT_00602694;
extern int *DAT_00602680;
extern int DAT_00602678;
extern int DAT_00602690;
extern int DAT_006026c0;
extern int *DAT_00602684;
extern int DAT_006026a0;
extern int *DAT_006026ac;
extern int DAT_006026a8;

int __cdecl __strcmpi(char *a, char *b);
void __cdecl FUN_00489fb0(int a, int *b, undefined4 *c, int d, int e);
int *__cdecl FUN_0048e520(undefined4 *a, char *b);
void __cdecl FUN_00489840(int a, int *b, int c, undefined4 *d);
void __cdecl FUN_004899d0(int a, int *b, int c, undefined4 *d);
void __cdecl FUN_00489b40(int a, int *b, int c, int d);
int __cdecl FUN_00488a90(char *a, int b, int c);
int __cdecl FUN_00488bc0(int *param_1, int *param_2, int param_3, int param_4, undefined4 *param_5);

int __cdecl FUN_00488bc0(int *param_1, int *param_2, int param_3, int param_4, undefined4 *param_5)
{
  char cVar1;
  undefined4 uVar2;
  int *piVar3;
  int *piVar4;
  int iVar5;
  float *pfVar6;
  unsigned int uVar7;
  unsigned int uVar8;
  int iVar9;
  int *piVar10;
  int *piVar11;
  char *pcVar12;
  int *piVar13;
  char *pcVar14;
  int *piVar15;
  int iVar16;
  undefined4 *puVar17;
  int bVar18;
  unsigned int uStack_f4;
  float fStack_ec;
  float fStack_e8;
  float fStack_e4;
  float fStack_e0;
  float fStack_dc;
  float fStack_d8;
  undefined4 auStack_d0[52];
  /* integer-typed stand-ins for Ghidra's float-typed reuse of fStack_ec/fStack_e0 in case 7 */
  int iStack_ec;
  int iStack_e0;

  piVar3 = param_1;
  if (param_2 == (int *)0x0) {
    return 0;
  }
  piVar10 = (int *)0x0;
  uStack_f4 = 0;
  bVar18 = 1;
  switch (param_2[1]) {
  case 1:
  case 0xf:
  case 0x10:
  case 0x11:
    if (*(int *)param_2[5] < 0) {
      bVar18 = 0;
    }
    else {
      bVar18 = (DAT_006026b8 == (int *)0x0);
      param_1[1] = param_2[1];
      piVar4 = param_1 + 6;
      piVar10 = piVar3;
      if (bVar18) {
        if (*param_2 == 0) {
          *param_1 = 0;
          param_1 = piVar4;
        }
        else {
          *param_1 = (int)piVar4;
          uVar7 = 0xffffffff;
          pcVar14 = (char *)*param_2;
          do {
            pcVar12 = pcVar14;
            if (uVar7 == 0) break;
            uVar7 = uVar7 - 1;
            pcVar12 = pcVar14 + 1;
            cVar1 = *pcVar14;
            pcVar14 = pcVar12;
          } while (cVar1 != '\0');
          uVar7 = ~uVar7;
          piVar11 = (int *)(pcVar12 + -uVar7);
          piVar13 = piVar4;
          for (uVar8 = uVar7 >> 2; uVar8 != 0; uVar8 = uVar8 - 1) {
            *piVar13 = *piVar11;
            piVar11 = piVar11 + 1;
            piVar13 = piVar13 + 1;
          }
          for (uVar7 = uVar7 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
            *(char *)piVar13 = (char)*piVar11;
            piVar11 = (int *)((int)piVar11 + 1);
            piVar13 = (int *)((int)piVar13 + 1);
          }
          uVar7 = 0xffffffff;
          pcVar14 = (char *)*param_2;
          do {
            if (uVar7 == 0) break;
            uVar7 = uVar7 - 1;
            cVar1 = *pcVar14;
            pcVar14 = pcVar14 + 1;
          } while (cVar1 != '\0');
          param_1 = (int *)((unsigned int)(int)piVar4 + ~uVar7);
        }
        piVar3[4] = param_2[4];
        piVar3[5] = (int)param_1;
        iVar9 = param_2[4];
        piVar4 = (int *)param_2[5];
        piVar11 = param_1;
        for (uVar7 = (iVar9 * 0x30U + 4U) >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
          *piVar11 = *piVar4;
          piVar4 = piVar4 + 1;
          piVar11 = piVar11 + 1;
        }
        for (iVar16 = 0; iVar16 != 0; iVar16 = iVar16 + -1) {
          *(char *)piVar11 = (char)*piVar4;
          piVar4 = (int *)((int)piVar4 + 1);
          piVar11 = (int *)((int)piVar11 + 1);
        }
        piVar13 = param_1 + (int)(iVar9 * 0xcU) + 1;
        uVar7 = param_2[4];
        piVar4 = (int *)param_2[5];
        piVar11 = piVar13;
        for (iVar9 = (int)((uVar7 & 0xfffffff) << 2); iVar9 != 0; iVar9 = iVar9 + -1) {
          *piVar11 = *piVar4;
          piVar4 = piVar4 + 1;
          piVar11 = piVar11 + 1;
        }
        for (iVar9 = 0; iVar9 != 0; iVar9 = iVar9 + -1) {
          *(char *)piVar11 = (char)*piVar4;
          piVar4 = (int *)((int)piVar4 + 1);
          piVar11 = (int *)((int)piVar11 + 1);
        }
        iVar9 = param_2[4];
        uStack_f4 = iVar9 * 0x24U;
        piVar4 = (int *)(iVar9 * 0x30U + 4U + (unsigned int)param_2[5]);
        piVar11 = piVar4;
        piVar15 = piVar13 + uVar7 * 4;
        for (uVar8 = uStack_f4 >> 2; uVar8 != 0; uVar8 = uVar8 - 1) {
          *piVar15 = *piVar11;
          piVar11 = piVar11 + 1;
          piVar15 = piVar15 + 1;
        }
        for (iVar16 = 0; iVar5 = DAT_006026bc, iVar16 != 0; iVar16 = iVar16 + -1) {
          *(char *)piVar15 = (char)*piVar11;
          piVar11 = (int *)((int)piVar11 + 1);
          piVar15 = (int *)((int)piVar15 + 1);
        }
        param_1 = piVar13 + uVar7 * 4 + (int)(iVar9 * 9U);
        if (param_5[0x21] == 0) {
          uStack_f4 = (unsigned int)DAT_006026bc * 0xcU;
          piVar4 = piVar4 + (int)(iVar9 * 9U);
          piVar11 = param_1;
          for (uVar7 = ((unsigned int)DAT_006026bc * 3U) & 0x3fffffff; uVar7 != 0; uVar7 = uVar7 - 1) {
            *piVar11 = *piVar4;
            piVar4 = piVar4 + 1;
            piVar11 = piVar11 + 1;
          }
          for (iVar9 = 0; iVar9 != 0; iVar9 = iVar9 + -1) {
            *(char *)piVar11 = (char)*piVar4;
            piVar4 = (int *)((int)piVar4 + 1);
            piVar11 = (int *)((int)piVar11 + 1);
          }
          param_1 = param_1 + (int)((unsigned int)iVar5 * 3U);
        }
        bVar18 = 0;
      }
      else {
        if (*param_2 == 0) {
          *param_1 = 0;
          param_1 = piVar4;
        }
        else {
          *param_1 = (int)piVar4;
          uVar7 = 0xffffffff;
          pcVar14 = (char *)*param_2;
          do {
            pcVar12 = pcVar14;
            if (uVar7 == 0) break;
            uVar7 = uVar7 - 1;
            pcVar12 = pcVar14 + 1;
            cVar1 = *pcVar14;
            pcVar14 = pcVar12;
          } while (cVar1 != '\0');
          uVar7 = ~uVar7;
          piVar11 = (int *)(pcVar12 + -uVar7);
          piVar13 = piVar4;
          for (uVar8 = uVar7 >> 2; uVar8 != 0; uVar8 = uVar8 - 1) {
            *piVar13 = *piVar11;
            piVar11 = piVar11 + 1;
            piVar13 = piVar13 + 1;
          }
          for (uVar7 = uVar7 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
            *(char *)piVar13 = (char)*piVar11;
            piVar11 = (int *)((int)piVar11 + 1);
            piVar13 = (int *)((int)piVar13 + 1);
          }
          uVar7 = 0xffffffff;
          pcVar14 = (char *)*param_2;
          do {
            if (uVar7 == 0) break;
            uVar7 = uVar7 - 1;
            cVar1 = *pcVar14;
            pcVar14 = pcVar14 + 1;
          } while (cVar1 != '\0');
          param_1 = (int *)((unsigned int)(int)piVar4 + ~uVar7);
        }
        iVar9 = 0;
        piVar3[4] = 0;
        piVar3[5] = (int)param_1;
        *param_1 = 0;
        DAT_0052a54c = 0;
        DAT_006026b0 = DAT_00602694;
        if (0 < DAT_00602680[2]) {
          iVar16 = 0;
          do {
            FUN_00489fb0((int)piVar3, param_2, *(undefined4 **)(DAT_00602680[3] + iVar16), param_3,
                         (int)param_5);
            if (DAT_0052a54c != 0) break;
            iVar16 = iVar16 + 4;
            iVar9 = iVar9 + 1;
          } while (iVar9 < DAT_00602680[2]);
        }
        bVar18 = 0;
        param_1 = param_1 + (int)((unsigned int)piVar3[4] * 0x19U) + 1;
      }
    }
    break;
  case 2:
    uStack_f4 = 0x30;
    break;
  case 3:
    DAT_00602678 = param_2[5];
    DAT_006026bc = param_2[4];
    if (DAT_006026b8 == (int *)0x0) {
      uStack_f4 = (unsigned int)param_2[4] * 0xcU;
    }
    else {
      bVar18 = 0;
    }
    break;
  case 4:
    uStack_f4 = 0xc;
    break;
  case 5:
    uStack_f4 = 0x24;
    break;
  case 6:
    uStack_f4 = 0xc;
    break;
  case 7:
    uStack_f4 = 0xffffffff;
    pcVar14 = (char *)param_2[5];
    do {
      if (uStack_f4 == 0) break;
      uStack_f4 = uStack_f4 - 1;
      cVar1 = *pcVar14;
      pcVar14 = pcVar14 + 1;
    } while (cVar1 != '\0');
    uStack_f4 = ~uStack_f4;
    piVar10 = param_2;
    do {
      if (piVar10[1] != 7) break;
      pcVar14 = (char *)piVar10[5];
      iStack_e0 = 0;
      /* packed read of the int at param_3 + 6 into iVar5 */
      __builtin_memcpy(&iVar5, (char *)param_3 + 6, 4);
      if (0 < iVar5) {
        iStack_ec = 0;
        do {
          /* packed read of the int at param_3 + 10 into piVar15 */
          __builtin_memcpy(&piVar15, (char *)param_3 + 10, 4);
          piVar4 = *(int **)((int)piVar15 + iStack_ec);
          if (piVar4 == (int *)0x0) {
LAB_00488fe5:
            piVar4 = (int *)0x0;
          }
          else if (((char *)*piVar4 == (char *)0x0) ||
                   (iVar9 = __strcmpi((char *)*piVar4, pcVar14), iVar9 != 0)) {
            iVar9 = 0;
            iVar16 = 0;
            if (0 < piVar4[2]) {
              do {
                piVar10 = FUN_0048e520(*(undefined4 **)(piVar4[3] + iVar9), pcVar14);
                if (piVar10 != (int *)0x0) goto LAB_0048900f;
                iVar9 = iVar9 + 4;
                iVar16 = iVar16 + 1;
              } while (iVar16 < piVar4[2]);
            }
            goto LAB_00488fe5;
          }
          piVar10 = piVar4;
          if (piVar4 != (int *)0x0) goto LAB_0048900f;
          iStack_ec = iStack_ec + 4;
          iStack_e0 = iStack_e0 + 1;
          __builtin_memcpy(&iVar5, (char *)param_3 + 6, 4);
        } while (iStack_e0 < iVar5);
      }
      piVar10 = (int *)0x0;
LAB_0048900f:
      ;
    } while (piVar10 != (int *)0x0);
    if ((piVar10 != (int *)0x0) && (piVar10[1] == 0x14)) {
      DAT_00602694 = piVar10;
    }
    break;
  case 9:
    uStack_f4 = (unsigned int)param_2[4] * 0x38U;
    DAT_00602690 = param_2[5];
    break;
  case 0xb:
    DAT_006026c0 = param_2[5];
    param_5[0x21] = DAT_006026c0;
    if (DAT_00602684 == (int *)0x0) {
      uStack_f4 = (unsigned int)param_2[4] * 0xcU;
    }
    else {
      bVar18 = 0;
    }
    break;
  case 0xd:
    DAT_006026a0 = param_2[5];
    if (DAT_006026ac == (int *)0x0) {
      uStack_f4 = (unsigned int)param_2[4] << 3;
    }
    else {
      bVar18 = 0;
    }
    break;
  case 0x13:
    uStack_f4 = 0xc;
    break;
  case 0x14:
    DAT_00602694 = param_2;
    piVar4 = (int *)param_2[5];
    iVar9 = (int)((unsigned int)piVar4[5] * (unsigned int)piVar4[4]);
    iVar16 = iVar9;
    if ((int)((unsigned int)iVar9 - (unsigned int)piVar4[2]) == 0) {
      iVar16 = piVar4[3];
    }
    uStack_f4 = (unsigned int)iVar16 + 0x18U;
    if (piVar4[1] == 0) {
      if ((int)((unsigned int)iVar9 - (unsigned int)piVar4[2]) == 0) {
        iVar9 = piVar4[3];
      }
      if (*piVar4 == (int)((unsigned int)(int)piVar4 + (unsigned int)iVar9 + 0x18U)) {
        uStack_f4 = (unsigned int)iVar16 + 0x3018U;
      }
    }
    else {
      uStack_f4 = uStack_f4 + (unsigned int)piVar4[1] * 0x300U;
    }
    break;
  case 0x15:
    DAT_00602680 = param_2;
    param_1[1] = 0;
    piVar10 = param_1 + 6;
    if ((char *)*param_2 == (char *)0x0) {
      *param_1 = 0;
      param_1 = piVar10;
    }
    else {
      uVar7 = 0xffffffff;
      pcVar14 = (char *)*param_2;
      do {
        pcVar12 = pcVar14;
        if (uVar7 == 0) break;
        uVar7 = uVar7 - 1;
        pcVar12 = pcVar14 + 1;
        cVar1 = *pcVar14;
        pcVar14 = pcVar12;
      } while (cVar1 != '\0');
      uVar7 = ~uVar7;
      piVar4 = (int *)(pcVar12 + -uVar7);
      piVar11 = piVar10;
      for (uVar8 = uVar7 >> 2; uVar8 != 0; uVar8 = uVar8 - 1) {
        *piVar11 = *piVar4;
        piVar4 = piVar4 + 1;
        piVar11 = piVar11 + 1;
      }
      for (uVar7 = uVar7 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
        *(char *)piVar11 = (char)*piVar4;
        piVar4 = (int *)((int)piVar4 + 1);
        piVar11 = (int *)((int)piVar11 + 1);
      }
      *param_1 = (int)piVar10;
      uVar7 = 0xffffffff;
      pcVar14 = (char *)*param_2;
      do {
        if (uVar7 == 0) break;
        uVar7 = uVar7 - 1;
        cVar1 = *pcVar14;
        pcVar14 = pcVar14 + 1;
      } while (cVar1 != '\0');
      param_1 = (int *)((unsigned int)(int)piVar10 + ~uVar7);
    }
    piVar3[5] = 0;
    piVar3[4] = 0;
    piVar10 = param_1 + 4;
    piVar3[2] = 4;
    piVar3[3] = (int)param_1;
    *param_1 = (int)piVar10;
    param_1[5] = 3;
    *piVar10 = 0;
    param_1[9] = (int)(param_1 + 10);
    param_1[8] = 0;
    param_1[6] = 0;
    param_1[7] = 0;
    FUN_00489840((int)piVar10, param_2, param_3, param_5);
    pfVar6 = (float *)param_1[9];
    fStack_ec = *pfVar6;
    fStack_e8 = pfVar6[1];
    fStack_e4 = pfVar6[2];
    iVar9 = param_1[8];
    fStack_e0 = fStack_ec;
    fStack_dc = fStack_e8;
    fStack_d8 = fStack_e4;
    if (0 < iVar9) {
      do {
        if (*pfVar6 < fStack_ec) {
          fStack_ec = *pfVar6;
        }
        if (pfVar6[1] < fStack_e8) {
          fStack_e8 = pfVar6[1];
        }
        if (pfVar6[2] < fStack_e4) {
          fStack_e4 = pfVar6[2];
        }
        if (fStack_e0 < *pfVar6) {
          fStack_e0 = *pfVar6;
        }
        if (fStack_dc < pfVar6[1]) {
          fStack_dc = pfVar6[1];
        }
        if (fStack_d8 < pfVar6[2]) {
          fStack_d8 = pfVar6[2];
        }
        pfVar6 = pfVar6 + 3;
        iVar9 = iVar9 + -1;
      } while (iVar9 != 0);
    }
    DAT_006026b8 = piVar10;
    *pfVar6 = fStack_ec;
    pfVar6[1] = fStack_e8;
    pfVar6[2] = fStack_e4;
    pfVar6[3] = fStack_e0;
    pfVar6[4] = fStack_dc;
    pfVar6[5] = fStack_d8;
    iVar9 = param_1[8];
    param_1[8] = (int)((unsigned int)iVar9 + 2U);
    piVar10 = param_1 + 10 + (int)(((unsigned int)iVar9 + 2U) * 3U);
    *(int **)(piVar3[3] + 4) = piVar10;
    piVar10[1] = 0xd;
    *piVar10 = 0;
    piVar10[5] = (int)(piVar10 + 6);
    piVar10[4] = 0;
    piVar10[2] = 0;
    piVar10[3] = 0;
    FUN_004899d0((int)piVar10, param_2, param_3, param_5);
    piVar4 = piVar10 + 6 + (int)((unsigned int)piVar10[4] * 2U);
    DAT_006026ac = piVar10;
    *(int **)(piVar3[3] + 8) = piVar4;
    piVar4[1] = 0xb;
    *piVar4 = 0;
    piVar4[5] = (int)(piVar4 + 6);
    piVar4[4] = 0;
    piVar4[2] = 0;
    piVar4[3] = 0;
    iVar9 = DAT_006026bc;
    uVar2 = param_5[0x21];
    param_5[0x21] = 0;
    FUN_00489b40((int)piVar4, param_2, param_3, (int)param_5);
    param_5[0x21] = uVar2;
    piVar10 = piVar4 + 6 + (int)((unsigned int)piVar4[4] * 3U);
    DAT_00602684 = piVar4;
    DAT_006026bc = iVar9;
    *(int **)(piVar3[3] + 0xc) = piVar10;
    piVar13 = piVar10 + 6;
    piVar10[1] = 0x15;
    *piVar10 = 0;
    piVar10[5] = (int)piVar13;
    piVar10[4] = param_2[4];
    piVar4 = (int *)param_2[5];
    piVar11 = piVar13;
    for (uVar7 = param_2[4] & 0x3fffffff; uVar7 != 0; uVar7 = uVar7 - 1) {
      *piVar11 = *piVar4;
      piVar4 = piVar4 + 1;
      piVar11 = piVar11 + 1;
    }
    bVar18 = 0;
    param_1 = piVar13 + param_2[4];
    break;
  case 0x16:
    uStack_f4 = 0x10;
    break;
  case 0x17:
    uStack_f4 = 0x30;
    break;
  case 0x19:
    uStack_f4 = 0x48;
  }
  piVar4 = param_1;
  if (bVar18) {
    piVar10 = param_1 + 6;
    param_1[1] = param_2[1];
    if (*param_2 == 0) {
      *param_1 = 0;
      param_1 = piVar10;
    }
    else {
      *param_1 = (int)piVar10;
      uVar7 = 0xffffffff;
      pcVar14 = (char *)*param_2;
      do {
        pcVar12 = pcVar14;
        if (uVar7 == 0) break;
        uVar7 = uVar7 - 1;
        pcVar12 = pcVar14 + 1;
        cVar1 = *pcVar14;
        pcVar14 = pcVar12;
      } while (cVar1 != '\0');
      uVar7 = ~uVar7;
      piVar11 = (int *)(pcVar12 + -uVar7);
      piVar13 = piVar10;
      for (uVar8 = uVar7 >> 2; uVar8 != 0; uVar8 = uVar8 - 1) {
        *piVar13 = *piVar11;
        piVar11 = piVar11 + 1;
        piVar13 = piVar13 + 1;
      }
      for (uVar7 = uVar7 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
        *(char *)piVar13 = (char)*piVar11;
        piVar11 = (int *)((int)piVar11 + 1);
        piVar13 = (int *)((int)piVar13 + 1);
      }
      uVar7 = 0xffffffff;
      pcVar14 = (char *)*param_2;
      do {
        if (uVar7 == 0) break;
        uVar7 = uVar7 - 1;
        cVar1 = *pcVar14;
        pcVar14 = pcVar14 + 1;
      } while (cVar1 != '\0');
      param_1 = (int *)((unsigned int)(int)piVar10 + ~uVar7);
    }
    piVar4[4] = param_2[4];
    piVar4[5] = (int)param_1;
    piVar10 = (int *)param_2[5];
    piVar11 = param_1;
    for (uVar7 = uStack_f4 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
      *piVar11 = *piVar10;
      piVar10 = piVar10 + 1;
      piVar11 = piVar11 + 1;
    }
    for (uVar7 = uStack_f4 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
      *(char *)piVar11 = (char)*piVar10;
      piVar10 = (int *)((int)piVar10 + 1);
      piVar11 = (int *)((int)piVar11 + 1);
    }
    param_1 = (int *)((unsigned int)(int)param_1 + uStack_f4);
    piVar10 = piVar4;
    if ((param_2[1] == 0x14) && (puVar17 = (undefined4 *)param_2[5], puVar17[1] == 0)) {
      iVar9 = (int)(puVar17[5] * puVar17[4]);
      if ((int)puVar17[2] == iVar9) {
        iVar9 = puVar17[3];
      }
      pcVar14 = (char *)*puVar17;
      if ((char *)((unsigned int)(int)puVar17 + (unsigned int)iVar9 + 0x18U) == pcVar14) {
        piVar4 = (int *)piVar4[5];
        iVar9 = (int)((unsigned int)piVar4[5] * (unsigned int)piVar4[4]);
        if (piVar4[2] == iVar9) {
          iVar9 = piVar4[3];
        }
        *piVar4 = (int)((unsigned int)(int)piVar4 + (unsigned int)iVar9 + 0x18U);
      }
      else {
        iVar16 = 0;
        iVar9 = 0;
        /* packed read of the int at param_4 + 6 into iVar5 */
        __builtin_memcpy(&iVar5, (char *)param_4 + 6, 4);
        if (0 < iVar5) {
          do {
            /* packed read of the int at param_4 + 10 into piVar15 */
            __builtin_memcpy(&piVar15, (char *)param_4 + 10, 4);
            piVar11 = *(int **)((int)piVar15 + iVar16);
            if (piVar4 == piVar11) break;
            iVar5 = FUN_00488a90(pcVar14, (int)piVar11, (int)piVar4);
            if (iVar5 != 0) {
              *(int *)piVar4[5] = iVar5;
              goto LAB_00489696;
            }
            iVar16 = iVar16 + 4;
            iVar9 = iVar9 + 1;
            __builtin_memcpy(&iVar5, (char *)param_4 + 6, 4);
          } while (iVar9 < iVar5);
        }
        piVar11 = (int *)*puVar17;
        *(int **)piVar4[5] = param_1;
        piVar4 = param_1;
        for (iVar9 = 0xc00; iVar9 != 0; iVar9 = iVar9 + -1) {
          *piVar4 = *piVar11;
          piVar11 = piVar11 + 1;
          piVar4 = piVar4 + 1;
        }
        param_1 = param_1 + 0xc00;
      }
    }
  }
LAB_00489696:
  if (piVar10 == (int *)0x0) {
    if (param_2[2] == 0) {
      return 0;
    }
    *param_1 = 0;
    param_1[1] = 0;
    param_1[4] = 0;
    param_1[5] = 0;
    piVar10 = param_1;
    param_1 = param_1 + 6;
  }
  iVar9 = 0;
  piVar10[2] = 0;
  piVar10[3] = 0;
  if (param_2[2] != 0) {
    puVar17 = auStack_d0;
    for (iVar16 = 0x34; iVar16 != 0; iVar16 = iVar16 + -1) {
      *puVar17 = *param_5;
      param_5 = param_5 + 1;
      puVar17 = puVar17 + 1;
    }
    DAT_00602678 = 0;
    DAT_006026a0 = 0;
    DAT_006026c0 = 0;
    DAT_006026a8 = 0;
    DAT_00602690 = 0;
    piVar10[3] = (int)param_1;
    iVar16 = 0;
    piVar10[2] = param_2[2];
    param_1 = param_1 + param_2[2];
    if (0 < param_2[2]) {
      do {
        *(int **)(piVar10[3] + iVar9) = param_1;
        iVar5 = FUN_00488bc0(param_1, *(int **)(param_2[3] + iVar9), param_3, param_4, auStack_d0);
        if (iVar5 == 0) {
          *(undefined4 *)(piVar10[3] + iVar9) = 0;
        }
        iVar9 = iVar9 + 4;
        param_1 = (int *)((unsigned int)(int)param_1 + (unsigned int)iVar5);
        iVar16 = iVar16 + 1;
      } while (iVar16 < param_2[2]);
    }
  }
  return (int)((unsigned int)(int)param_1 - (unsigned int)(int)piVar3);
}
