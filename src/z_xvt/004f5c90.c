/* SPDX-License-Identifier: MIT */
/* z_xvt 004f5c90 */
extern int DAT_0081b850;
extern unsigned char DAT_00757d0a[];
extern unsigned char DAT_00757cee[];
extern unsigned char DAT_00757e2d[];
extern unsigned char DAT_00757e3e[];
extern unsigned char DAT_0081e0a2[];
extern int DAT_0081bc64;
extern unsigned char DAT_0081e0a6[];
extern unsigned char DAT_0081e0c6[];
extern unsigned char DAT_0081e0e6[];
extern unsigned char DAT_0081e1a6[];
extern unsigned char DAT_0081e266[];
extern unsigned char DAT_0081e286[];
extern unsigned char DAT_0081e2a6[];
extern unsigned char DAT_0081e366[];
extern unsigned char DAT_0081e4e6[];
extern unsigned char DAT_0081fa7e[];
extern unsigned char DAT_0081bfba[];
extern unsigned char DAT_0081c012[];
extern unsigned char DAT_0081d326[];
extern unsigned char DAT_0081d32e[];
extern unsigned char DAT_0081d3f6[];
extern unsigned char DAT_0081d3fa[];
extern unsigned char DAT_0081f802[];
extern unsigned char DAT_0081f7da[];
extern unsigned char DAT_0081f7fe[];
extern int DAT_00757870[];
extern unsigned char DAT_00757680[];
extern unsigned char DAT_00743dc3[];
extern unsigned char DAT_007438d7[];
extern unsigned char DAT_007438d2[];
extern unsigned char DAT_007438e4[];
extern unsigned char DAT_0081e00e[];
extern int DAT_00757a28;
extern int DAT_007579d0[];
extern short DAT_0075766c;
extern char DAT_0085e7c4;
extern unsigned char DAT_0081fb96[];
extern unsigned char DAT_0081fbda[];
extern unsigned char DAT_0082e3fe[];
extern unsigned char DAT_0081fb9e[];
extern unsigned char DAT_0081fba2[];
extern unsigned char DAT_0081fba6[];
extern unsigned char DAT_0081fbaa[];
extern unsigned char DAT_0081fbae[];
extern unsigned char DAT_0081fbb2[];
extern unsigned char DAT_0081fbb6[];
extern unsigned char DAT_0081fbde[];
extern unsigned char DAT_0081bfb6[];
extern unsigned char DAT_0081bfbe[];
extern unsigned char DAT_0081bfc2[];
extern unsigned char DAT_0081bfc6[];
extern unsigned char DAT_0081bfca[];
extern unsigned char DAT_0081bfce[];
extern unsigned char DAT_0081c016[];
extern unsigned char DAT_0083cbe6[];
extern unsigned char DAT_0083cbea[];
extern unsigned char DAT_0083cbee[];
extern unsigned char DAT_0083cc22[];
extern unsigned char DAT_0083cc26[];

unsigned int *FUN_004cdce0(int param_1);
int FUN_004cdbc0(void);
void FUN_004ce720(void);

void __stdcall FUN_004f5c90(void)
{
  unsigned char uVar1;
  unsigned int uVar2;
  unsigned int uVar3;
  int iVar4;
  unsigned char *puVar5;
  int *piVar6;
  int iVar7;
  char *pcVar8;
  unsigned int uVar9;
  unsigned char *puVar10;
  int iVar11;
  int iVar12;
  int *piVar13;
  int *piVar14;
  int *piStack_10;
  int iStack_c;
  int iStack_8;
  int iStack_4;

  if (DAT_0081b850 == 2) {
    puVar5 = DAT_00757d0a;
    do {
      puVar10 = puVar5 + 0x2a;
      for (iVar7 = 10; iVar7 != 0; iVar7 = iVar7 + -1) {
        puVar5[0] = 0; puVar5[1] = 0; puVar5[2] = 0; puVar5[3] = 0;
        puVar5 = puVar5 + 4;
      }
      puVar5[0] = 0; puVar5[1] = 0;
      puVar5 = puVar10;
    } while (puVar10 < DAT_00757e2d + 3);
  }
  else {
    piVar14 = (int *)DAT_00757cee;
    do {
      __builtin_memcpy(&uVar2, piVar14, 4);
      if ((uVar2 != 0) && (puVar5 = (unsigned char *)FUN_004cdce0((int)uVar2), puVar5 == (unsigned char *)0)) {
        puVar5 = (unsigned char *)piVar14 - 0xe;
        for (iVar7 = 10; iVar7 != 0; iVar7 = iVar7 + -1) {
          puVar5[0] = 0; puVar5[1] = 0; puVar5[2] = 0; puVar5[3] = 0;
          puVar5 = puVar5 + 4;
        }
        puVar5[0] = 0; puVar5[1] = 0;
      }
      piVar14 = (int *)((unsigned char *)piVar14 + 0x2a);
    } while (piVar14 < (int *)DAT_00757e3e);
  }
  DAT_0081e0a2[0] = 0; DAT_0081e0a2[1] = 0;
  DAT_0081bc64 = 0;
  puVar5 = DAT_0081e0a6;
  for (iVar7 = 8; iVar7 != 0; iVar7 = iVar7 + -1) {
    puVar5[0] = 0; puVar5[1] = 0; puVar5[2] = 0; puVar5[3] = 0;
    puVar5 = puVar5 + 4;
  }
  puVar5 = DAT_0081e0c6;
  for (iVar7 = 8; iVar7 != 0; iVar7 = iVar7 + -1) {
    puVar5[0] = 0; puVar5[1] = 0; puVar5[2] = 0; puVar5[3] = 0;
    puVar5 = puVar5 + 4;
  }
  puVar5 = DAT_0081e0e6;
  for (iVar7 = 0x30; iVar7 != 0; iVar7 = iVar7 + -1) {
    puVar5[0] = 0; puVar5[1] = 0; puVar5[2] = 0; puVar5[3] = 0;
    puVar5 = puVar5 + 4;
  }
  puVar5 = DAT_0081e1a6;
  for (iVar7 = 0x30; iVar7 != 0; iVar7 = iVar7 + -1) {
    puVar5[0] = 0; puVar5[1] = 0; puVar5[2] = 0; puVar5[3] = 0;
    puVar5 = puVar5 + 4;
  }
  puVar5 = DAT_0081e266;
  for (iVar7 = 8; iVar7 != 0; iVar7 = iVar7 + -1) {
    puVar5[0] = 0; puVar5[1] = 0; puVar5[2] = 0; puVar5[3] = 0;
    puVar5 = puVar5 + 4;
  }
  puVar5 = DAT_0081e286;
  for (iVar7 = 8; iVar7 != 0; iVar7 = iVar7 + -1) {
    puVar5[0] = 0; puVar5[1] = 0; puVar5[2] = 0; puVar5[3] = 0;
    puVar5 = puVar5 + 4;
  }
  puVar5 = DAT_0081e2a6;
  for (iVar7 = 0x30; iVar7 != 0; iVar7 = iVar7 + -1) {
    puVar5[0] = 0; puVar5[1] = 0; puVar5[2] = 0; puVar5[3] = 0;
    puVar5 = puVar5 + 4;
  }
  puVar5 = DAT_0081e366;
  for (iVar7 = 0x30; iVar7 != 0; iVar7 = iVar7 + -1) {
    puVar5[0] = 0; puVar5[1] = 0; puVar5[2] = 0; puVar5[3] = 0;
    puVar5 = puVar5 + 4;
  }
  puVar5 = DAT_0081e4e6;
  for (iVar7 = 0x4b6; iVar7 != 0; iVar7 = iVar7 + -1) {
    puVar5[0] = 0; puVar5[1] = 0; puVar5[2] = 0; puVar5[3] = 0;
    puVar5 = puVar5 + 4;
  }
  puVar5 = DAT_0081fa7e;
  for (iVar7 = 0x46; iVar7 != 0; iVar7 = iVar7 + -1) {
    puVar5[0] = 0; puVar5[1] = 0; puVar5[2] = 0; puVar5[3] = 0;
    puVar5 = puVar5 + 4;
  }
  iStack_4 = 0;
  __builtin_memcpy(&uVar2, DAT_0081bfba, 4);
  __builtin_memcpy(&uVar3, DAT_0081c012, 4);
  __builtin_memcpy(&iVar4, DAT_0081d326, 4);
  if (((uVar2 == 1) && (uVar3 == 1)) && (iVar4 == 0)) {
    uVar9 = 0xffffffff;
    puVar5 = DAT_0081d32e;
    do {
      __builtin_memcpy(puVar5, &uVar9, 4);
      puVar5 = puVar5 + 0x14;
    } while (puVar5 < DAT_0081d3f6);
  }
  iStack_8 = 0;
  piStack_10 = (int *)DAT_00757cee;
  piVar14 = (int *)DAT_0081f802;
  do {
    __builtin_memcpy(&uVar2, piStack_10, 4);
    if (uVar2 != 0) {
      iStack_4 = iStack_4 + 1;
      __builtin_memcpy(&uVar2, (unsigned char *)piStack_10 - 10, 4);
      __builtin_memcpy(&uVar3, (unsigned char *)piStack_10 - 6, 4);
      __builtin_memcpy((unsigned char *)piVar14 - 0x36, (unsigned char *)piStack_10 - 0xe, 4);
      uVar1 = *((unsigned char *)piStack_10 - 2);
      __builtin_memcpy((unsigned char *)piVar14 - 0x32, &uVar2, 4);
      __builtin_memcpy((unsigned char *)piVar14 - 0x2e, &uVar3, 4);
      *((unsigned char *)piVar14 - 0x2a) = uVar1;
      __builtin_memcpy(&iVar7, (unsigned char *)piStack_10 + 12, 4);
      __builtin_memcpy(&iVar12, (unsigned char *)piStack_10 + 8, 4);
      *((unsigned char *)piVar14 - 0x2a) = 0;
      __builtin_memcpy((unsigned char *)piVar14 - 4, &iVar12, 4);
      __builtin_memcpy((unsigned char *)piVar14, &iVar7, 4);
      __builtin_memcpy(&iVar4, (unsigned char *)piStack_10 + 16, 4);
      iVar4 = (int)((unsigned int)iVar4 - 1u);
      __builtin_memcpy((unsigned char *)piVar14 + 4, &iVar4, 4);
      __builtin_memcpy(&iVar7, piStack_10, 4);
      __builtin_memcpy(&iVar12, (unsigned char *)piStack_10 + 4, 4);
      __builtin_memcpy(&iVar4, (unsigned char *)piStack_10 + 20, 4);
      iVar4 = (int)((unsigned int)iVar4 - 1u);
      __builtin_memcpy((unsigned char *)piVar14 + 8, &iVar4, 4);
      __builtin_memcpy(&iVar4, (unsigned char *)piStack_10 + 24, 4);
      iVar4 = (int)((unsigned int)iVar4 - 1u);
      __builtin_memcpy((unsigned char *)piVar14 + 12, &iVar4, 4);
      uVar9 = 0;
      __builtin_memcpy((unsigned char *)piVar14 - 0x24, &iVar7, 4);
      __builtin_memcpy((unsigned char *)piVar14 - 0x20, &iVar12, 4);
      __builtin_memcpy((unsigned char *)piVar14 - 0x1c, &uVar9, 4);
      __builtin_memcpy((unsigned char *)piVar14 - 0x18, &uVar9, 4);
      __builtin_memcpy((unsigned char *)piVar14 - 0x14, &uVar9, 4);
      __builtin_memcpy((unsigned char *)piVar14 - 0x10, &uVar9, 4);
      __builtin_memcpy((unsigned char *)piVar14 - 0xc, &uVar9, 4);
      __builtin_memcpy((unsigned char *)piVar14 - 8, &uVar9, 4);
      __builtin_memcpy((unsigned char *)piVar14 + 0x10, &uVar9, 4);
      iStack_c = 10;
      iVar12 = 0;
      do {
        iVar11 = 8;
        do {
          if (*(int *)((unsigned char *)DAT_00757870 + iVar12) == iVar7) {
            iVar4 = *(int *)(DAT_00757680 + iVar12);
            __builtin_memcpy((unsigned char *)piVar14 - 0x28, &iVar4, 4);
            __builtin_memcpy(&uVar3, (unsigned char *)piVar14 - 4, 4);
            if (uVar3 == 0) {
              __builtin_memcpy(&uVar2, piVar14, 4);
              if (uVar2 != 0xffffffffu) {
                if (DAT_00743dc3[uVar2 + (unsigned int)iVar4 * 0x562u] == 0) {
                  uVar9 = 0xffffffff;
                  __builtin_memcpy(piVar14, &uVar9, 4);
                }
              }
            }
          }
          iVar12 = iVar12 + 4;
          iVar11 = iVar11 + -1;
        } while (iVar11 != 0);
        iStack_c = iStack_c + -1;
      } while (iStack_c != 0);
      iVar12 = FUN_004cdbc0();
      if (iVar12 == iVar7) {
        DAT_0081bc64 = iStack_8;
      }
    }
    piVar14 = piVar14 + 0x16;
    piStack_10 = (int *)((unsigned char *)piStack_10 + 0x2a);
    iStack_8 = iStack_8 + 1;
  } while (piStack_10 < (int *)DAT_00757e3e);

  __builtin_memcpy(&uVar2, DAT_0081bfba, 4);
  __builtin_memcpy(&uVar3, DAT_0081c012, 4);
  if ((uVar2 == 3) && (uVar3 == 1)) {
    __builtin_memcpy(DAT_0081e00e, &iStack_4, 4);
  }
  if (uVar2 == 1) {
    uVar9 = 0;
    __builtin_memcpy(&iVar4, DAT_0081d326, 4);
    if ((uVar3 == 1) && (iVar4 == 0)) {
      if (0 < DAT_00757a28) {
        puVar5 = DAT_0081d32e;
        piVar13 = DAT_00757870;
        piVar14 = DAT_007579d0;
        iVar7 = DAT_00757a28;
        do {
          iVar11 = 0;
          iVar12 = *piVar14;
          piVar6 = piVar13;
          if (0 < iVar12) {
            do {
              if (*piVar6 != 0) break;
              iVar11 = iVar11 + 1;
              piVar6 = piVar6 + 1;
            } while (iVar11 < iVar12);
            if (iVar11 < iVar12) {
              __builtin_memcpy(puVar5, &uVar9, 4);
            }
          }
          puVar5 = puVar5 + 0x14;
          piVar13 = piVar13 + 8;
          piVar14 = piVar14 + 1;
          iVar7 = iVar7 + -1;
        } while (iVar7 != 0);
      }
      if (((DAT_0085e7c4 == '\x01') || (DAT_0081b850 == 2)) && (0 < DAT_0075766c)) {
        pcVar8 = (char *)DAT_007438e4;
        iVar7 = (int)DAT_0075766c;
        do {
          if (*pcVar8 != '\0') {
            __builtin_memcpy(DAT_0081d32e + (unsigned int)(unsigned char)pcVar8[-0xc] * 20u, &uVar9, 4);
          }
          pcVar8 = pcVar8 + 0x562;
          iVar7 = iVar7 + -1;
        } while (iVar7 != 0);
      }
      __builtin_memcpy(DAT_0081d3f6, &iStack_4, 4);
      if ((DAT_0081b850 == 2) || (DAT_0085e7c4 != '\0')) {
        __builtin_memcpy(DAT_0081d3fa, &DAT_00757a28, 4);
      }
      else {
        __builtin_memcpy(DAT_0081d3fa, &uVar9, 4);
        if (0 < DAT_00757a28) {
          puVar5 = DAT_0081d32e;
          piVar14 = DAT_00757870;
          piVar13 = DAT_007579d0;
          iStack_8 = DAT_00757a28;
          do {
            iVar12 = 0;
            iVar7 = *piVar13;
            piVar6 = piVar14;
            if (0 < iVar7) {
              do {
                if (*piVar6 != 0) break;
                iVar12 = iVar12 + 1;
                piVar6 = piVar6 + 1;
              } while (iVar12 < iVar7);
              if (iVar12 < iVar7) {
                __builtin_memcpy(&iVar4, DAT_0081d3fa, 4);
                iVar4 = (int)((unsigned int)iVar4 + 1u);
                __builtin_memcpy(DAT_0081d3fa, &iVar4, 4);
                __builtin_memcpy(puVar5, &uVar9, 4);
              }
            }
            puVar5 = puVar5 + 0x14;
            piVar14 = piVar14 + 8;
            piVar13 = piVar13 + 1;
            iStack_8 = iStack_8 + -1;
          } while (iStack_8 != 0);
        }
      }
    }
    if (uVar2 != 1) goto LAB_004f606a;
  }
  else {
LAB_004f606a:
    if (uVar2 != 2) {
      __builtin_memcpy(&iVar12, DAT_0081f7da + (unsigned int)DAT_0081bc64 * 0x58u, 4);
      uVar9 = (unsigned int)DAT_007438d7[(unsigned int)iVar12 * 0x562u];
      __builtin_memcpy(DAT_0081fb96, &uVar9, 4);
      goto LAB_004f6132;
    }
  }
  __builtin_memcpy(&uVar9, DAT_0081f7fe + (unsigned int)DAT_0081bc64 * 0x58u, 4);
  if (uVar9 == 0) {
    __builtin_memcpy(&iVar7, DAT_0081f802 + (unsigned int)DAT_0081bc64 * 0x58u, 4);
    __builtin_memcpy(&iVar12, DAT_0081f7da + (unsigned int)DAT_0081bc64 * 0x58u, 4);
    if ((iVar7 == -1) || (9 < iVar7)) {
      uVar9 = (unsigned int)DAT_007438d2[(unsigned int)iVar12 * 0x562u];
    }
    else {
      uVar9 = (unsigned int)DAT_00743dc3[(unsigned int)iVar7 + (unsigned int)iVar12 * 0x562u];
    }
  }
  if (((int)uVar9 < 1) || ((3 < (int)uVar9 && (uVar9 != 0xe)))) {
    uVar3 = 1;
    __builtin_memcpy(DAT_0081fb96, &uVar3, 4);
    uVar3 = 0;
    __builtin_memcpy(DAT_0081fbda, &uVar3, 4);
  }
  else {
    uVar3 = 0;
    __builtin_memcpy(DAT_0081fb96, &uVar3, 4);
    __builtin_memcpy(DAT_0082e3fe, &uVar3, 4);
  }
LAB_004f6132:
  if (DAT_0081b850 == 2) {
    __builtin_memcpy(&uVar9, DAT_0081fb96, 4);
    __builtin_memcpy(DAT_0081fb9e + uVar9 * 0xe824u, DAT_0081bfb6, 4);
    __builtin_memcpy(DAT_0081fba2 + uVar9 * 0xe824u, DAT_0081bfba, 4);
    __builtin_memcpy(&uVar2, DAT_0081bfba, 4);
    if (uVar2 == 0) {
      __builtin_memcpy(DAT_0081fba6 + uVar9 * 0xe824u, DAT_0081bfbe, 4);
    }
    else if ((uVar2 == 1) || (uVar2 == 2)) {
      __builtin_memcpy(DAT_0081fbaa + uVar9 * 0xe824u, DAT_0081bfc2, 4);
      __builtin_memcpy(DAT_0081fbae + uVar9 * 0xe824u, DAT_0081bfc6, 4);
    }
    else {
      __builtin_memcpy(DAT_0081fbb2 + uVar9 * 0xe824u, DAT_0081bfca, 4);
      __builtin_memcpy(DAT_0081fbb6 + uVar9 * 0xe824u, DAT_0081bfce, 4);
    }
    __builtin_memcpy(DAT_0081fbda + uVar9 * 0xe824u, DAT_0081c012, 4);
    uVar3 = 0;
    __builtin_memcpy(DAT_0083cc22, &uVar3, 4);
    __builtin_memcpy(DAT_0081fbde + uVar9 * 0xe824u, DAT_0081c016, 4);
  }
  else {
    __builtin_memcpy(DAT_0083cbe6, DAT_0081bfb6, 4);
    __builtin_memcpy(DAT_0083cbea, DAT_0081bfba, 4);
    __builtin_memcpy(DAT_0083cbee, DAT_0081bfbe, 20);
    __builtin_memcpy(DAT_0083cc22, DAT_0081c012, 4);
    __builtin_memcpy(DAT_0083cc26, DAT_0081c016, 4);
    uVar3 = 0;
    __builtin_memcpy(DAT_0081fbda, &uVar3, 4);
    __builtin_memcpy(DAT_0082e3fe, &uVar3, 4);
  }
  FUN_004ce720();
  return;
}
