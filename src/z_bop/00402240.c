/* SPDX-License-Identifier: MIT */
/* z_bop 00402240 */
void *__builtin_memcpy(void *, const void *, unsigned int);

extern unsigned char DAT_009e9670[];
extern unsigned char DAT_009e9684[];
extern unsigned char DAT_009e9685[];
extern unsigned char DAT_009e96dd[];
extern int DAT_009a1fe8;
extern int DAT_009a1fe0;
extern unsigned short DAT_009ec5fe;
extern char DAT_009a2062;
extern char DAT_009d695b;
extern char DAT_009d8c91[];
extern unsigned char DAT_0051c56e[];
extern unsigned char DAT_00521f1c[];
extern short DAT_00a07cec;
extern int DAT_00a07ce8;
extern int DAT_009d1308;
extern unsigned char *DAT_009ecc34;

void FUN_00433d30(unsigned short, unsigned short, int);
unsigned int FUN_004269e0(unsigned int);
void FUN_0042edc0(int, int);
void FUN_0042f030(int);
void FUN_0042e460(int);
void FUN_0045acd0(int);
void FUN_00458780(int);
void FUN_00455c80(unsigned short);
int FUN_0045a1e0(int, unsigned int, unsigned int, int);
void FUN_00485080(int);
void FUN_004851d0(unsigned char *);
void FUN_00452880(int);
void FUN_00450650(int, int);
void FUN_0042e4d0(unsigned int, unsigned int, int);
void FUN_004ad9e0(int, int);
void FUN_004ada90(int, int);

void __cdecl FUN_00402240(unsigned char *param_1, unsigned int param_2, int param_3)
{
  short sVar1;
  char cVar2;
  unsigned char bVar3;
  int iVar4;
  int bVar5;
  unsigned int uVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  unsigned short uVar10;
  int iVar11;
  unsigned short uVar12;
  int iVar13;
  unsigned short uVar14;
  unsigned int uVar15;
  int iVar16;
  int tmp32;
  int tmp32b;
  int tmpA;
  int tmpB;
  unsigned short tmp16;

  iVar4 = (int)((unsigned int)param_1 * 0x5bdU);
  iVar11 = *(int *)((unsigned int)DAT_009e9670 + (unsigned int)iVar4);
  iVar9 = (int)((unsigned int)iVar11 * 0x23U);
  iVar7 = (int)((unsigned int)DAT_009a1fe8 + (unsigned int)iVar9);
  __builtin_memcpy(&tmp32, (unsigned char *)((unsigned int)DAT_009a1fe8 + 0x1fU + (unsigned int)iVar9), 4);
  __builtin_memcpy(&iVar8, (unsigned char *)((unsigned int)tmp32 + 0xa9U), 4);
  if (*(char *)((unsigned int)iVar8 + 7U) != '\0') {
    DAT_009e9684[iVar4] = 0;
  }
  uVar15 = (unsigned int)DAT_009ec5fe;
  __builtin_memcpy(&tmp32, DAT_009e9685 + iVar4, 4);
  uVar6 = (unsigned int)tmp32 + uVar15;
  __builtin_memcpy(DAT_009e9685 + iVar4, &uVar6, 4);
  if (DAT_009e9684[iVar4] != 1) {
    if (DAT_009e9684[iVar4] != 2) {
      return;
    }
    if (0x49b < uVar6) {
      bVar3 = *(unsigned char *)((unsigned int)iVar7 + 0x16U);
      FUN_00433d30((unsigned short)iVar11, (unsigned short)bVar3, 0x11);
      if (((DAT_009a2062 != '\x02') && (DAT_009d695b == '\x01')) &&
          (DAT_009d8c91[(unsigned int)bVar3 * 0x566U] != 'c')) {
        uVar6 = FUN_004269e0(((((unsigned int)DAT_009a1fe8 >> 8) << 8) |
                              (unsigned int)*(unsigned char *)((unsigned int)DAT_009a1fe8 + 3U + (unsigned int)iVar9)) &
                             0xffff00ffU);
        __builtin_memcpy(&tmp32, DAT_009e96dd + iVar4, 4);
        tmp32 = (int)((unsigned int)tmp32 +
                      (unsigned int)DAT_0051c56e[(uVar6 & 0xffff) * 0x101U] * 0x28U);
        __builtin_memcpy(DAT_009e96dd + iVar4, &tmp32, 4);
      }
      if (*(int *)((unsigned int)DAT_009e9670 + (unsigned int)iVar4) != -1) {
        FUN_0042edc0(0, (int)param_1);
        FUN_0042f030(0);
        FUN_0042e460((int)param_1);
      }
      *(unsigned char *)((unsigned int)DAT_009a1fe8 + 3U + (unsigned int)iVar9) = 0;
      FUN_0045acd0((int)param_1);
      FUN_00458780(iVar8);
      DAT_009e9684[iVar4] = 0;
      FUN_00455c80((unsigned short)bVar3);
      iVar7 = FUN_0045a1e0((int)param_1, 0xffffffffU, 0, 0);
      if (iVar7 == 0) {
        if (param_1 == DAT_009ecc34) {
          FUN_00452880(0x123);
        }
        return;
      }
      FUN_00485080((int)param_1);
      FUN_004851d0(param_1);
      return;
    }
    bVar5 = 0;
    iVar7 = *(int *)((unsigned int)DAT_009e9670 + (unsigned int)iVar4);
    sVar1 = DAT_00a07cec;
    if (uVar6 <= uVar15) {
      for (; sVar1 < DAT_009a1fe0; sVar1 = (short)(sVar1 + 1)) {
        iVar11 = (int)((unsigned int)((int)sVar1) * 0x23U + (unsigned int)DAT_009a1fe8);
        if (*(char *)((unsigned int)iVar11 + 3U) != '\0') {
          iVar8 = (int)((unsigned int)iVar7 * 0x23U + (unsigned int)DAT_009a1fe8);
          tmpA = *(int *)((unsigned int)iVar11 + 4U);
          tmpB = *(int *)((unsigned int)iVar8 + 4U);
          iVar9 = (int)((unsigned int)tmpA - (unsigned int)tmpB);
          tmpA = *(int *)((unsigned int)iVar11 + 8U);
          tmpB = *(int *)((unsigned int)iVar8 + 8U);
          iVar13 = (int)((unsigned int)tmpA - (unsigned int)tmpB);
          tmpA = *(int *)((unsigned int)iVar11 + 0xcU);
          tmpB = *(int *)((unsigned int)iVar8 + 0xcU);
          iVar16 = (int)((unsigned int)tmpA - (unsigned int)tmpB);
          if (iVar9 < 0) {
            iVar9 = (int)(0U - (unsigned int)iVar9);
          }
          if (iVar16 < 0) {
            iVar16 = (int)(0U - (unsigned int)iVar16);
          }
          tmpA = *(int *)((unsigned int)DAT_00521f1c + (unsigned int)*(unsigned char *)((unsigned int)iVar11 + 3U) * 0x1aU);
          tmpB = *(int *)((unsigned int)DAT_00521f1c + (unsigned int)*(unsigned char *)((unsigned int)iVar8 + 3U) * 0x1aU);
          if ((((int)((unsigned int)iVar9 - (unsigned int)tmpA) < tmpB) &&
               ((int)((unsigned int)iVar16 - (unsigned int)tmpA) < tmpB)) &&
              ((0 < iVar13 && (iVar13 < 0x40000)))) {
            bVar5 = 1;
            break;
          }
        }
      }
      if (bVar5) goto LAB_00402636;
      bVar5 = 0;
      for (sVar1 = (short)DAT_00a07ce8;
           (int)sVar1 < (int)((unsigned int)DAT_009d1308 + (unsigned int)DAT_00a07ce8);
           sVar1 = (short)(sVar1 + 1)) {
        iVar11 = (int)((unsigned int)((int)sVar1) * 0x23U + (unsigned int)DAT_009a1fe8);
        if (*(char *)((unsigned int)iVar11 + 3U) != '\0') {
          iVar8 = (int)((unsigned int)iVar7 * 0x23U + (unsigned int)DAT_009a1fe8);
          tmpA = *(int *)((unsigned int)iVar11 + 4U);
          tmpB = *(int *)((unsigned int)iVar8 + 4U);
          iVar9 = (int)((unsigned int)tmpA - (unsigned int)tmpB);
          tmpA = *(int *)((unsigned int)iVar11 + 8U);
          tmpB = *(int *)((unsigned int)iVar8 + 8U);
          iVar13 = (int)((unsigned int)tmpA - (unsigned int)tmpB);
          tmpA = *(int *)((unsigned int)iVar11 + 0xcU);
          tmpB = *(int *)((unsigned int)iVar8 + 0xcU);
          iVar16 = (int)((unsigned int)tmpA - (unsigned int)tmpB);
          if (iVar9 < 0) {
            iVar9 = (int)(0U - (unsigned int)iVar9);
          }
          if (iVar16 < 0) {
            iVar16 = (int)(0U - (unsigned int)iVar16);
          }
          tmpA = *(int *)((unsigned int)DAT_00521f1c + (unsigned int)*(unsigned char *)((unsigned int)iVar11 + 3U) * 0x1aU);
          tmpB = *(int *)((unsigned int)DAT_00521f1c + (unsigned int)*(unsigned char *)((unsigned int)iVar8 + 3U) * 0x1aU);
          if ((((int)((unsigned int)iVar9 - (unsigned int)tmpA) < tmpB) &&
               ((int)((unsigned int)iVar16 - (unsigned int)tmpA) < tmpB)) &&
              ((0 < iVar13 && (iVar13 < 0x40000)))) {
            bVar5 = 1;
            break;
          }
        }
      }
    }
    if (!bVar5) {
      uVar10 = (unsigned short)(uVar6 / 0xec);
      if (uVar10 == 0) {
        iVar11 = 10;
      }
      else if (uVar10 == 1) {
        iVar11 = 0x19;
      }
      else if (uVar10 == 2) {
        iVar11 = 0x32;
      }
      else if (uVar10 == 3) {
        iVar11 = 100;
      }
      else {
        iVar11 = 1000;
      }
      FUN_004ad9e0(iVar7, iVar11);
      tmp32 = *(int *)((unsigned int)iVar7 * 0x23U + 8U + (unsigned int)DAT_009a1fe8);
      *(int *)((unsigned int)iVar7 * 0x23U + 8U + (unsigned int)DAT_009a1fe8) =
          (int)((unsigned int)tmp32 +
                (unsigned int)uVar10 * (unsigned int)DAT_009ec5fe * 0xe0U);
      return;
    }
LAB_00402636:
    FUN_00450650(0x70, (int)param_1);
    tmp32 = *(int *)((unsigned int)DAT_009e9670 + (unsigned int)iVar4);
    if (tmp32 == -1) {
      bVar5 = 0;
    }
    else {
      cVar2 = *(char *)((unsigned int)tmp32 * 0x23U + 3U + (unsigned int)DAT_009a1fe8);
      if (((cVar2 != '\x01') && (cVar2 != '\x02')) && (cVar2 != '\x03') &&
          (cVar2 != '\x0e') && (cVar2 != '\x04')) {
        bVar5 = 0;
      }
      else {
        bVar5 = 1;
      }
    }
    if (bVar5) {
      uVar6 = 0x57;
    }
    else {
      uVar6 = 0x24;
    }
    FUN_0042e4d0(uVar6, 0xffffffffU, (int)param_1);
    DAT_009e9684[iVar4] = 0;
    __builtin_memcpy(&tmp32, (unsigned char *)((unsigned int)iVar7 * 0x23U + 0x1fU + (unsigned int)DAT_009a1fe8), 4);
    tmp16 = 10;
    __builtin_memcpy((unsigned char *)((unsigned int)tmp32 + 0x79U), &tmp16, 2);
    return;
  }
  uVar10 = *(unsigned short *)((unsigned int)iVar7 + 0x14U);
  uVar14 = *(unsigned short *)((unsigned int)iVar7 + 0x10U);
  uVar12 = *(unsigned short *)((unsigned int)iVar7 + 0x12U);
  if (((uVar10 == 0) && (uVar12 == 0x4000)) && (uVar14 == 0)) {
    uVar10 = 0;
    uVar14 = 0;
    if (0x49b < uVar6) {
      DAT_009e9684[iVar4] = 2;
      FUN_00450650(0x6c, (int)param_1);
      tmp32 = 0;
      __builtin_memcpy(DAT_009e9685 + iVar4, &tmp32, 4);
    }
    goto LAB_004023af;
  }
  if (uVar10 < 0x8000) {
    uVar10 = (unsigned short)((unsigned int)uVar10 + (unsigned int)DAT_009ec5fe * (unsigned int)-0x20);
    if (0x7fff < uVar10) {
LAB_00402356:
      uVar10 = 0;
    }
  }
  else {
    uVar10 = (unsigned short)((unsigned int)uVar10 + (unsigned int)DAT_009ec5fe * 0x20U);
    if (uVar10 < 0x8000) goto LAB_00402356;
  }
  if (uVar14 < 0x8000) {
    uVar14 = (unsigned short)((unsigned int)uVar14 + (unsigned int)DAT_009ec5fe * (unsigned int)-0x10);
    if (0x7fff < uVar14) {
LAB_00402376:
      uVar14 = 0;
    }
  }
  else {
    uVar14 = (unsigned short)((unsigned int)uVar14 + (unsigned int)DAT_009ec5fe * 0x10U);
    if (uVar14 < 0x8000) goto LAB_00402376;
  }
  if ((uVar12 < 0xc000) && (0x4000 < uVar12)) {
    uVar12 = (unsigned short)((unsigned int)uVar12 + (unsigned int)DAT_009ec5fe * (unsigned int)-0x10);
    if (0x3fff < uVar12) goto LAB_004023af;
  }
  else if ((uVar12 == 0x4000) ||
           ((uVar12 = (unsigned short)((unsigned int)uVar12 + (unsigned int)DAT_009ec5fe * 0x10U),
             uVar12 < 0x4001) || (0xbfff < uVar12)))
    goto LAB_004023af;
  uVar12 = 0x4000;
LAB_004023af:
  *(unsigned short *)((unsigned int)DAT_009a1fe8 + 0x14U + (unsigned int)iVar9) = uVar10;
  *(unsigned short *)((unsigned int)DAT_009a1fe8 + 0x10U + (unsigned int)iVar9) = uVar14;
  *(unsigned short *)((unsigned int)DAT_009a1fe8 + 0x12U + (unsigned int)iVar9) = uVar12;
  __builtin_memcpy(&tmp32, (unsigned char *)((unsigned int)DAT_009a1fe8 + 0x1fU + (unsigned int)iVar9), 4);
  *(unsigned char *)((unsigned int)tmp32 + 0x92U) = 1;
  __builtin_memcpy(&iVar7, (unsigned char *)((unsigned int)DAT_009a1fe8 + 0x1fU + (unsigned int)iVar9), 4);
  *(unsigned char *)((unsigned int)iVar7 + 0x8bU) = *(unsigned char *)((unsigned int)iVar7 + 0x92U);
  __builtin_memcpy(&tmp32, (unsigned char *)((unsigned int)DAT_009a1fe8 + 0x1fU + (unsigned int)iVar9), 4);
  __builtin_memcpy(&tmp32b, (unsigned char *)((unsigned int)tmp32 + 0xa9U), 4);
  __builtin_memcpy((unsigned char *)((unsigned int)tmp32b + 0xdU), &uVar12, 2);
  FUN_004ada90(iVar11, 200);
  return;
}
