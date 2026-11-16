/* SPDX-License-Identifier: MIT */
/* z_bop 00474960 */
extern int *DAT_0060f200;
extern int DAT_0060f204;
extern int DAT_005272b0;
extern int DAT_0060f1c0;
extern int *DAT_0060f1c8;
extern int *DAT_0060f1cc;
extern int DAT_0060f1d8;
extern int *DAT_0060f1dc;
extern int DAT_0060f1e8;
extern int *DAT_0060f1f4;
extern int _DAT_0060f208;

void __cdecl FUN_00476280(int param_1, int *param_2, int param_3, int param_4);
int *__cdecl FUN_00479de0(int param_1, char *param_2);
void __cdecl FUN_00475740(int param_1, int *param_2, int param_3, unsigned int *param_4);
void __cdecl FUN_00475850(int param_1, int *param_2, int param_3, unsigned int *param_4);
void __cdecl FUN_00475940(int param_1, int *param_2, int param_3, int param_4);
int __cdecl FUN_00474910(char *param_1, int param_2, int param_3);
int __cdecl FUN_00474960(int *param_1, int *param_2, int param_3, int param_4, unsigned int *param_5);

int __cdecl FUN_00474960(int *param_1, int *param_2, int param_3, int param_4, unsigned int *param_5)
{
  char cVar1;
  unsigned int uVar2;
  unsigned char bVar3;
  int iVar4;
  int iVar5;
  unsigned int uVar6;
  unsigned int uVar7;
  float *pfVar8;
  char *pcVar9;
  int iVar10;
  int *piVar11;
  char *pcVar12;
  int *piVar13;
  int *piVar14;
  char *pcVar15;
  int *piVar16;
  char *pcVar17;
  unsigned int *puVar18;
  unsigned int uStack_f0;
  int iStack_ec;
  float fStack_ec;
  float fStack_e8;
  float fStack_e4;
  float fStack_e0;
  float fStack_dc;
  float fStack_d8;
  unsigned int auStack_d0[52];

  if (param_2 == (int *)0x0) {
    return 0;
  }
  bVar3 = 1;
  piVar11 = (int *)0x0;
  iVar4 = param_2[1];
  uStack_f0 = 0;
  piVar14 = param_1;
  switch (iVar4) {
  case 1:
  case 0xf:
  case 0x10:
  case 0x11:
    if (*(int *)param_2[5] < 0) {
      bVar3 = 0;
    }
    else if (DAT_0060f200 == (int *)0x0) {
      piVar11 = param_1 + 6;
      param_1[1] = iVar4;
      if (*param_2 == 0) {
        *param_1 = 0;
      }
      else {
        uVar6 = 0xffffffff;
        *param_1 = (int)piVar11;
        pcVar12 = (char *)*param_2;
        do {
          pcVar15 = pcVar12;
          if (uVar6 == 0) break;
          uVar6 = uVar6 - 1;
          pcVar15 = pcVar12 + 1;
          cVar1 = *pcVar12;
          pcVar12 = pcVar15;
        } while (cVar1 != '\0');
        uVar6 = ~uVar6;
        piVar14 = (int *)(pcVar15 - uVar6);
        piVar13 = piVar11;
        for (uVar7 = uVar6 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
          *piVar13 = *piVar14;
          piVar14 = piVar14 + 1;
          piVar13 = piVar13 + 1;
        }
        for (uVar6 = uVar6 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
          *(char *)piVar13 = (char)*piVar14;
          piVar14 = (int *)((int)piVar14 + 1);
          piVar13 = (int *)((int)piVar13 + 1);
        }
        uVar6 = 0xffffffff;
        pcVar12 = (char *)*param_2;
        do {
          if (uVar6 == 0) break;
          uVar6 = uVar6 - 1;
          cVar1 = *pcVar12;
          pcVar12 = pcVar12 + 1;
        } while (cVar1 != '\0');
        piVar11 = (int *)((int)piVar11 + ~uVar6);
      }
      param_1[4] = param_2[4];
      param_1[5] = (int)piVar11;
      if (DAT_005272b0 == 0) {
        iVar4 = (int)((unsigned int)param_2[4] << 6);
      }
      else {
        iVar4 = (int)((unsigned int)param_2[4] * 0x30);
      }
      piVar14 = (int *)param_2[5];
      piVar13 = piVar11;
      for (uVar6 = (((unsigned int)iVar4 + 4U) >> 2); uVar6 != 0; uVar6 = uVar6 - 1) {
        *piVar13 = *piVar14;
        piVar14 = piVar14 + 1;
        piVar13 = piVar13 + 1;
      }
      pcVar12 = (char *)((int)piVar11 + (unsigned int)iVar4 + 4U);
      for (iVar4 = 0; iVar4 != 0; iVar4 = iVar4 + -1) {
        *(char *)piVar13 = (char)*piVar14;
        piVar14 = (int *)((int)piVar14 + 1);
        piVar13 = (int *)((int)piVar13 + 1);
      }
      uVar6 = (unsigned int)param_2[4];
      pcVar15 = (char *)param_2[5];
      pcVar17 = pcVar12;
      for (iVar4 = (int)((uVar6 & 0xfffffff) << 2); iVar4 != 0; iVar4 = iVar4 + -1) {
        *(unsigned int *)pcVar17 = *(unsigned int *)pcVar15;
        pcVar15 = pcVar15 + 4;
        pcVar17 = pcVar17 + 4;
      }
      pcVar12 = pcVar12 + uVar6 * 0x10;
      for (iVar4 = 0; iVar4 != 0; iVar4 = iVar4 + -1) {
        *pcVar17 = *pcVar15;
        pcVar15 = pcVar15 + 1;
        pcVar17 = pcVar17 + 1;
      }
      iVar4 = param_2[4];
      uStack_f0 = (unsigned int)iVar4 * 0x24;
      if (DAT_005272b0 == 0) {
        iVar10 = (int)((unsigned int)iVar4 << 6);
      }
      else {
        iVar10 = (int)((unsigned int)iVar4 * 0x30);
      }
      pcVar9 = (char *)((unsigned int)param_2[5] + (unsigned int)iVar10 + 4U);
      pcVar15 = pcVar9;
      pcVar17 = pcVar12;
      for (uVar6 = uStack_f0 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
        *(unsigned int *)pcVar17 = *(unsigned int *)pcVar15;
        pcVar15 = pcVar15 + 4;
        pcVar17 = pcVar17 + 4;
      }
      for (iVar10 = 0; iVar5 = DAT_0060f204, iVar10 != 0; iVar10 = iVar10 + -1) {
        *pcVar17 = *pcVar15;
        pcVar15 = pcVar15 + 1;
        pcVar17 = pcVar17 + 1;
      }
      piVar14 = (int *)(pcVar12 + (unsigned int)iVar4 * 0x24);
      if (param_5[0x21] == 0) {
        uStack_f0 = (unsigned int)DAT_0060f204 * 0xc;
        piVar11 = (int *)(pcVar9 + (unsigned int)iVar4 * 0x24);
        piVar13 = piVar14;
        for (uVar6 = ((unsigned int)DAT_0060f204 * 3) & 0x3fffffff; uVar6 != 0; uVar6 = uVar6 - 1) {
          *piVar13 = *piVar11;
          piVar11 = piVar11 + 1;
          piVar13 = piVar13 + 1;
        }
        for (iVar4 = 0; iVar4 != 0; iVar4 = iVar4 + -1) {
          *(char *)piVar13 = (char)*piVar11;
          piVar11 = (int *)((int)piVar11 + 1);
          piVar13 = (int *)((int)piVar13 + 1);
        }
        piVar14 = piVar14 + (int)((unsigned int)iVar5 * 3);
      }
      bVar3 = 0;
      piVar11 = param_1;
    }
    else {
      piVar14 = param_1 + 6;
      param_1[1] = iVar4;
      if (*param_2 == 0) {
        *param_1 = 0;
      }
      else {
        uVar6 = 0xffffffff;
        *param_1 = (int)piVar14;
        pcVar12 = (char *)*param_2;
        do {
          pcVar15 = pcVar12;
          if (uVar6 == 0) break;
          uVar6 = uVar6 - 1;
          pcVar15 = pcVar12 + 1;
          cVar1 = *pcVar12;
          pcVar12 = pcVar15;
        } while (cVar1 != '\0');
        uVar6 = ~uVar6;
        piVar11 = (int *)(pcVar15 - uVar6);
        piVar13 = piVar14;
        for (uVar7 = uVar6 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
          *piVar13 = *piVar11;
          piVar11 = piVar11 + 1;
          piVar13 = piVar13 + 1;
        }
        for (uVar6 = uVar6 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
          *(char *)piVar13 = (char)*piVar11;
          piVar11 = (int *)((int)piVar11 + 1);
          piVar13 = (int *)((int)piVar13 + 1);
        }
        uVar6 = 0xffffffff;
        pcVar12 = (char *)*param_2;
        do {
          if (uVar6 == 0) break;
          uVar6 = uVar6 - 1;
          cVar1 = *pcVar12;
          pcVar12 = pcVar12 + 1;
        } while (cVar1 != '\0');
        piVar14 = (int *)((int)piVar14 + ~uVar6);
      }
      param_1[4] = 0;
      param_1[5] = (int)piVar14;
      *piVar14 = 0;
      FUN_00476280((int)param_1, param_2, param_3, (int)param_5);
      bVar3 = 0;
      piVar11 = param_1;
      piVar14 = piVar14 + (int)((unsigned int)param_1[4] * 0x19) + 1;
    }
    break;
  case 2:
    uStack_f0 = 0x30;
    break;
  case 3:
    DAT_0060f1c0 = param_2[5];
    DAT_0060f204 = param_2[4];
    if (DAT_0060f200 == (int *)0x0) {
      uStack_f0 = (unsigned int)param_2[4] * 0xc;
    }
    else {
      bVar3 = 0;
    }
    break;
  case 4:
    uStack_f0 = 0xc;
    break;
  case 5:
    uStack_f0 = 0x24;
    break;
  case 6:
    uStack_f0 = 0xc;
    break;
  case 7:
    uStack_f0 = 0xffffffff;
    pcVar12 = (char *)param_2[5];
    do {
      if (uStack_f0 == 0) break;
      uStack_f0 = uStack_f0 - 1;
      cVar1 = *pcVar12;
      pcVar12 = pcVar12 + 1;
    } while (cVar1 != '\0');
    uStack_f0 = ~uStack_f0;
    piVar11 = param_2;
    do {
      if (piVar11[1] != 7) break;
      piVar11 = FUN_00479de0(param_3, (char *)piVar11[5]);
    } while (piVar11 != (int *)0x0);
    if ((piVar11 != (int *)0x0) && (piVar11[1] == 0x14)) {
      DAT_0060f1dc = piVar11;
    }
    break;
  case 9:
    uStack_f0 = (unsigned int)param_2[4] * 0x38;
    DAT_0060f1d8 = param_2[5];
    break;
  case 0xb:
    _DAT_0060f208 = param_2[5];
    param_5[0x21] = (unsigned int)_DAT_0060f208;
    if (DAT_0060f1cc == (int *)0x0) {
      uStack_f0 = (unsigned int)param_2[4] * 0xc;
    }
    else {
      bVar3 = 0;
    }
    break;
  case 0xd:
    DAT_0060f1e8 = param_2[5];
    if (DAT_0060f1f4 == (int *)0x0) {
      uStack_f0 = (unsigned int)param_2[4] << 3;
    }
    else {
      bVar3 = 0;
    }
    break;
  case 0x13:
    uStack_f0 = 0xc;
    break;
  case 0x14:
    DAT_0060f1dc = param_2;
    piVar13 = (int *)param_2[5];
    iVar4 = (int)((unsigned int)piVar13[5] * (unsigned int)piVar13[4]);
    iVar10 = iVar4;
    if ((int)((unsigned int)iVar4 - (unsigned int)piVar13[2]) == 0) {
      iVar10 = piVar13[3];
    }
    uStack_f0 = (unsigned int)iVar10 + 0x18;
    if (piVar13[1] == 0) {
      if ((int)((unsigned int)iVar4 - (unsigned int)piVar13[2]) == 0) {
        iVar4 = piVar13[3];
      }
      if (*piVar13 == (int)((unsigned int)(int)piVar13 + (unsigned int)iVar4 + 0x18)) {
        uStack_f0 = (unsigned int)iVar10 + 0x3018;
      }
    }
    else {
      uStack_f0 = uStack_f0 + (unsigned int)piVar13[1] * 0x300;
    }
    break;
  case 0x15:
    DAT_0060f1c8 = param_2;
    if (DAT_0060f200 == (int *)0x0) {
      param_1[1] = 0;
      piVar11 = param_1 + 6;
      if ((char *)*param_2 == (char *)0x0) {
        *param_1 = 0;
      }
      else {
        uVar6 = 0xffffffff;
        pcVar12 = (char *)*param_2;
        do {
          pcVar15 = pcVar12;
          if (uVar6 == 0) break;
          uVar6 = uVar6 - 1;
          pcVar15 = pcVar12 + 1;
          cVar1 = *pcVar12;
          pcVar12 = pcVar15;
        } while (cVar1 != '\0');
        uVar6 = ~uVar6;
        piVar14 = (int *)(pcVar15 - uVar6);
        piVar13 = piVar11;
        for (uVar7 = uVar6 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
          *piVar13 = *piVar14;
          piVar14 = piVar14 + 1;
          piVar13 = piVar13 + 1;
        }
        for (uVar6 = uVar6 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
          *(char *)piVar13 = (char)*piVar14;
          piVar14 = (int *)((int)piVar14 + 1);
          piVar13 = (int *)((int)piVar13 + 1);
        }
        *param_1 = (int)piVar11;
        uVar6 = 0xffffffff;
        pcVar12 = (char *)*param_2;
        do {
          if (uVar6 == 0) break;
          uVar6 = uVar6 - 1;
          cVar1 = *pcVar12;
          pcVar12 = pcVar12 + 1;
        } while (cVar1 != '\0');
        piVar11 = (int *)((int)piVar11 + ~uVar6);
      }
      piVar14 = piVar11 + 4;
      param_1[5] = 0;
      param_1[4] = 0;
      param_1[2] = 4;
      param_1[3] = (int)piVar11;
      *piVar11 = (int)piVar14;
      piVar11[5] = 3;
      *piVar14 = 0;
      piVar11[9] = (int)(piVar11 + 10);
      piVar11[8] = 0;
      piVar11[6] = 0;
      piVar11[7] = 0;
      FUN_00475740((int)piVar14, param_2, param_3, param_5);
      pfVar8 = (float *)piVar11[9];
      fStack_ec = *pfVar8;
      fStack_e8 = pfVar8[1];
      fStack_e4 = pfVar8[2];
      iVar4 = piVar11[8];
      fStack_e0 = fStack_ec;
      fStack_dc = fStack_e8;
      fStack_d8 = fStack_e4;
      if (0 < iVar4) {
        do {
          if (*pfVar8 < fStack_ec) {
            fStack_ec = *pfVar8;
          }
          if (pfVar8[1] < fStack_e8) {
            fStack_e8 = pfVar8[1];
          }
          if (pfVar8[2] < fStack_e4) {
            fStack_e4 = pfVar8[2];
          }
          if (fStack_e0 < *pfVar8) {
            fStack_e0 = *pfVar8;
          }
          if (fStack_dc < pfVar8[1]) {
            fStack_dc = pfVar8[1];
          }
          if (fStack_d8 < pfVar8[2]) {
            fStack_d8 = pfVar8[2];
          }
          pfVar8 = pfVar8 + 3;
          iVar4 = iVar4 + -1;
        } while (iVar4 != 0);
      }
      DAT_0060f200 = piVar14;
      if ((((pfVar8[-6] != fStack_ec) || (pfVar8[-5] != fStack_e8)) || (pfVar8[-4] != fStack_e4)) ||
         (((pfVar8[-3] != fStack_e0 || (pfVar8[-2] != fStack_dc)) || (pfVar8[-1] != fStack_d8)))) {
        *pfVar8 = fStack_ec;
        pfVar8[1] = fStack_e8;
        pfVar8[2] = fStack_e4;
        pfVar8[3] = fStack_e0;
        pfVar8[4] = fStack_dc;
        pfVar8[5] = fStack_d8;
        piVar11[8] = (int)((unsigned int)piVar11[8] + 2);
      }
      piVar11 = piVar11 + 10 + (int)((unsigned int)piVar11[8] * 3);
      *(int **)(param_1[3] + 4) = piVar11;
      piVar11[1] = 0xd;
      *piVar11 = 0;
      piVar11[5] = (int)(piVar11 + 6);
      piVar11[4] = 0;
      piVar11[2] = 0;
      piVar11[3] = 0;
      FUN_00475850((int)piVar11, param_2, param_3, param_5);
      piVar14 = piVar11 + 6 + (int)((unsigned int)piVar11[4] * 2);
      DAT_0060f1f4 = piVar11;
      *(int **)(param_1[3] + 8) = piVar14;
      piVar14[1] = 0xb;
      puVar18 = param_5 + 0x21;
      *piVar14 = 0;
      piVar14[5] = (int)(piVar14 + 6);
      piVar14[4] = 0;
      piVar14[2] = 0;
      piVar14[3] = 0;
      iVar4 = DAT_0060f204;
      uVar2 = *puVar18;
      *puVar18 = 0;
      FUN_00475940((int)piVar14, param_2, param_3, (int)param_5);
      *puVar18 = uVar2;
      piVar11 = piVar14 + 6 + (int)((unsigned int)piVar14[4] * 3);
      DAT_0060f1cc = piVar14;
      DAT_0060f204 = iVar4;
      *(int **)(param_1[3] + 0xc) = piVar11;
      piVar11[1] = 0x15;
      piVar14 = piVar11 + 6;
      *piVar11 = 0;
      piVar11[5] = (int)piVar14;
      piVar11[4] = param_2[4];
      piVar13 = (int *)param_2[5];
      piVar16 = piVar14;
      for (uVar6 = (unsigned int)param_2[4] & 0x3fffffff; uVar6 != 0; uVar6 = uVar6 - 1) {
        *piVar16 = *piVar13;
        piVar13 = piVar13 + 1;
        piVar16 = piVar16 + 1;
      }
      iVar4 = param_2[4];
      iVar10 = param_2[2];
      piVar14 = piVar14 + iVar4;
      if (iVar4 < iVar10) {
        piVar11[4] = iVar10;
        piVar13 = piVar14;
        for (uVar6 = ((unsigned int)param_2[2] - (unsigned int)param_2[4]) & 0x3fffffff; uVar6 != 0; uVar6 = uVar6 - 1) {
          *piVar13 = 0;
          piVar13 = piVar13 + 1;
        }
        bVar3 = 0;
        piVar14 = piVar14 + (int)((unsigned int)param_2[2] - (unsigned int)param_2[4]);
      }
      else {
        if (iVar10 < iVar4) {
          piVar11[4] = iVar10;
          piVar14 = piVar14 + (int)((unsigned int)param_2[2] - (unsigned int)param_2[4]);
        }
        bVar3 = 0;
      }
    }
    else {
      piVar14 = param_1 + 6;
      piVar11 = param_1 + 4;
      param_1[1] = 0x15;
      *param_1 = 0;
      param_1[5] = (int)piVar14;
      *piVar11 = param_2[4];
      piVar13 = (int *)param_2[5];
      piVar16 = piVar14;
      for (uVar6 = (unsigned int)param_2[4] & 0x3fffffff; uVar6 != 0; uVar6 = uVar6 - 1) {
        *piVar16 = *piVar13;
        piVar13 = piVar13 + 1;
        piVar16 = piVar16 + 1;
      }
      iVar4 = param_2[4];
      iVar10 = param_2[2];
      piVar14 = piVar14 + iVar4;
      if (iVar4 < iVar10) {
        *piVar11 = iVar10;
        piVar11 = piVar14;
        for (uVar6 = ((unsigned int)param_2[2] - (unsigned int)param_2[4]) & 0x3fffffff; uVar6 != 0; uVar6 = uVar6 - 1) {
          *piVar11 = 0;
          piVar11 = piVar11 + 1;
        }
        bVar3 = 0;
        piVar11 = param_1;
        piVar14 = piVar14 + (int)((unsigned int)param_2[2] - (unsigned int)param_2[4]);
      }
      else {
        if (iVar10 < iVar4) {
          *piVar11 = iVar10;
          piVar14 = piVar14 + (int)((unsigned int)param_2[2] - (unsigned int)param_2[4]);
        }
        bVar3 = 0;
        piVar11 = param_1;
      }
    }
    break;
  case 0x16:
    uStack_f0 = 0x10;
    break;
  case 0x17:
    uStack_f0 = 0x30;
    break;
  case 0x19:
    uStack_f0 = 0x48;
  }
  piVar13 = piVar14;
  if (bVar3) {
    piVar13 = piVar14 + 6;
    piVar14[1] = param_2[1];
    if (*param_2 == 0) {
      *piVar14 = 0;
    }
    else {
      uVar6 = 0xffffffff;
      *piVar14 = (int)piVar13;
      pcVar12 = (char *)*param_2;
      do {
        pcVar15 = pcVar12;
        if (uVar6 == 0) break;
        uVar6 = uVar6 - 1;
        pcVar15 = pcVar12 + 1;
        cVar1 = *pcVar12;
        pcVar12 = pcVar15;
      } while (cVar1 != '\0');
      uVar6 = ~uVar6;
      piVar11 = (int *)(pcVar15 - uVar6);
      piVar16 = piVar13;
      for (uVar7 = uVar6 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
        *piVar16 = *piVar11;
        piVar11 = piVar11 + 1;
        piVar16 = piVar16 + 1;
      }
      for (uVar6 = uVar6 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
        *(char *)piVar16 = (char)*piVar11;
        piVar11 = (int *)((int)piVar11 + 1);
        piVar16 = (int *)((int)piVar16 + 1);
      }
      uVar6 = 0xffffffff;
      pcVar12 = (char *)*param_2;
      do {
        if (uVar6 == 0) break;
        uVar6 = uVar6 - 1;
        cVar1 = *pcVar12;
        pcVar12 = pcVar12 + 1;
      } while (cVar1 != '\0');
      piVar13 = (int *)((int)piVar13 + ~uVar6);
    }
    piVar14[4] = param_2[4];
    uVar6 = uStack_f0 >> 2;
    piVar14[5] = (int)piVar13;
    piVar11 = (int *)param_2[5];
    piVar16 = piVar13;
    for (; uVar6 != 0; uVar6 = uVar6 - 1) {
      *piVar16 = *piVar11;
      piVar11 = piVar11 + 1;
      piVar16 = piVar16 + 1;
    }
    for (uVar6 = uStack_f0 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
      *(char *)piVar16 = (char)*piVar11;
      piVar11 = (int *)((int)piVar11 + 1);
      piVar16 = (int *)((int)piVar16 + 1);
    }
    piVar13 = (int *)((int)piVar13 + uStack_f0);
    piVar11 = piVar14;
    if ((param_2[1] == 0x14) && (puVar18 = (unsigned int *)param_2[5], puVar18[1] == 0)) {
      iVar4 = (int)(puVar18[5] * puVar18[4]);
      if (puVar18[2] == (unsigned int)iVar4) {
        iVar4 = (int)puVar18[3];
      }
      if ((char *)*puVar18 == (char *)((unsigned int)(int)puVar18 + (unsigned int)iVar4 + 0x18)) {
        piVar14 = (int *)piVar14[5];
        iVar4 = (int)((unsigned int)piVar14[5] * (unsigned int)piVar14[4]);
        if (piVar14[2] == iVar4) {
          iVar4 = piVar14[3];
        }
        *piVar14 = (int)((unsigned int)(int)piVar14 + (unsigned int)iVar4 + 0x18);
      }
      else {
        iVar4 = FUN_00474910((char *)*puVar18, param_4, (int)piVar14);
        if (iVar4 == 0) {
          piVar16 = (int *)*puVar18;
          *(int **)piVar14[5] = piVar13;
          piVar14 = piVar13;
          for (iVar4 = 0xc00; iVar4 != 0; iVar4 = iVar4 + -1) {
            *piVar14 = *piVar16;
            piVar16 = piVar16 + 1;
            piVar14 = piVar14 + 1;
          }
          piVar13 = piVar13 + 0xc00;
        }
        else {
          *(int *)piVar14[5] = iVar4;
        }
      }
    }
  }
  piVar14 = piVar13;
  if (piVar11 == (int *)0x0) {
    if (param_2[2] == 0) {
      return 0;
    }
    piVar14 = piVar13 + 6;
    *piVar13 = 0;
    piVar13[1] = 0;
    piVar13[4] = 0;
    piVar13[5] = 0;
    piVar11 = piVar13;
  }
  piVar11[2] = 0;
  piVar11[3] = 0;
  iVar4 = param_2[2];
  if (iVar4 != 0) {
    if (DAT_0060f200 == (int *)0x0) {
      piVar11[2] = (int)((unsigned int)iVar4 + 3);
      piVar11[3] = (int)piVar14;
      piVar13 = piVar14 + piVar11[2];
      *piVar14 = (int)piVar13;
      piVar13[1] = 3;
      *piVar13 = 0;
      piVar13[5] = (int)(piVar13 + 6);
      piVar13[4] = 0;
      piVar13[2] = 0;
      piVar13[3] = 0;
      FUN_00475740((int)piVar13, param_2, param_3, param_5);
      pfVar8 = (float *)piVar13[5];
      fStack_ec = *pfVar8;
      fStack_e8 = pfVar8[1];
      fStack_e4 = pfVar8[2];
      iVar4 = piVar13[4];
      fStack_e0 = fStack_ec;
      fStack_dc = fStack_e8;
      fStack_d8 = fStack_e4;
      if (0 < iVar4) {
        do {
          if (*pfVar8 < fStack_ec) {
            fStack_ec = *pfVar8;
          }
          if (pfVar8[1] < fStack_e8) {
            fStack_e8 = pfVar8[1];
          }
          if (pfVar8[2] < fStack_e4) {
            fStack_e4 = pfVar8[2];
          }
          if (fStack_e0 < *pfVar8) {
            fStack_e0 = *pfVar8;
          }
          if (fStack_dc < pfVar8[1]) {
            fStack_dc = pfVar8[1];
          }
          if (fStack_d8 < pfVar8[2]) {
            fStack_d8 = pfVar8[2];
          }
          pfVar8 = pfVar8 + 3;
          iVar4 = iVar4 + -1;
        } while (iVar4 != 0);
      }
      DAT_0060f200 = piVar13;
      if (((((pfVar8[-6] != fStack_ec) || (pfVar8[-5] != fStack_e8)) || (pfVar8[-4] != fStack_e4))
          || ((pfVar8[-3] != fStack_e0 || (pfVar8[-2] != fStack_dc)))) || (pfVar8[-1] != fStack_d8))
      {
        *pfVar8 = fStack_ec;
        pfVar8[1] = fStack_e8;
        pfVar8[2] = fStack_e4;
        pfVar8[3] = fStack_e0;
        pfVar8[4] = fStack_dc;
        pfVar8[5] = fStack_d8;
        piVar13[4] = (int)((unsigned int)piVar13[4] + 2);
      }
      piVar14 = piVar13 + 6 + (int)((unsigned int)piVar13[4] * 3);
      *(int **)(piVar11[3] + 4) = piVar14;
      piVar14[1] = 0xd;
      *piVar14 = 0;
      piVar14[5] = (int)(piVar14 + 6);
      piVar14[4] = 0;
      piVar14[2] = 0;
      piVar14[3] = 0;
      FUN_00475850((int)piVar14, param_2, param_3, param_5);
      piVar13 = piVar14 + 6 + (int)((unsigned int)piVar14[4] * 2);
      DAT_0060f1f4 = piVar14;
      *(int **)(piVar11[3] + 8) = piVar13;
      piVar13[1] = 0xb;
      puVar18 = param_5 + 0x21;
      *piVar13 = 0;
      piVar13[5] = (int)(piVar13 + 6);
      piVar13[4] = 0;
      piVar13[2] = 0;
      piVar13[3] = 0;
      iVar4 = DAT_0060f204;
      uVar2 = *puVar18;
      *puVar18 = 0;
      FUN_00475940((int)piVar13, param_2, param_3, (int)param_5);
      *puVar18 = uVar2;
      iVar10 = 3;
      piVar14 = piVar13 + 6 + (int)((unsigned int)piVar13[4] * 3);
      DAT_0060f1cc = piVar13;
      DAT_0060f204 = iVar4;
    }
    else {
      iVar10 = 0;
      piVar11[2] = iVar4;
      piVar11[3] = (int)piVar14;
      piVar14 = piVar14 + piVar11[2];
    }
    puVar18 = auStack_d0;
    for (iVar4 = 0x34; iVar4 != 0; iVar4 = iVar4 + -1) {
      *puVar18 = *param_5;
      param_5 = param_5 + 1;
      puVar18 = puVar18 + 1;
    }
    iVar4 = 0;
    iStack_ec = 0;
    if (0 < param_2[2]) {
      iVar10 = (int)((unsigned int)iVar10 * 4);
      do {
        *(int **)(piVar11[3] + iVar10) = piVar14;
        iVar5 = FUN_00474960(piVar14, *(int **)(param_2[3] + iVar4), param_3, param_4, auStack_d0);
        if (iVar5 == 0) {
          *(unsigned int *)(piVar11[3] + iVar10) = 0;
        }
        piVar14 = (int *)((int)piVar14 + iVar5);
        iVar4 = iVar4 + 4;
        iVar10 = iVar10 + 4;
        iStack_ec = iStack_ec + 1;
      } while (iStack_ec < param_2[2]);
    }
  }
  return (int)((unsigned int)(int)piVar14 - (unsigned int)(int)param_1);
}
