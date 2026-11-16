/* SPDX-License-Identifier: MIT */
/* z_xvt 0041d620 */
extern void __stdcall FUN_004a6f90(void);
extern void __cdecl FUN_0041eea0(int);
extern void __cdecl FUN_004215c0(int);
extern void __stdcall FUN_004b1730(void);
extern void __cdecl FUN_004b1bc0(int, int);
extern void __stdcall FUN_0048f530(void);
extern void __stdcall FUN_004a6a80(void);

extern unsigned short DAT_0073b374;
extern unsigned short DAT_006db6c0;
extern unsigned char DAT_00536e8c;
extern short DAT_006d49a0;
extern unsigned char DAT_006d4a60[];
extern unsigned char DAT_006d4a61[];
extern unsigned char DAT_006d4a62[];
extern unsigned char DAT_006d4a64[];
extern unsigned short DAT_006d4a68[];
extern unsigned short DAT_006d4a6a[];
extern unsigned char DAT_006d4aa2[];
extern unsigned char DAT_006d4abe[];
extern unsigned char DAT_006d4ac8[];
extern unsigned char DAT_006d4ad2[];
extern unsigned char DAT_006d4adc[];
extern unsigned char DAT_006d4ae6[];
extern unsigned char DAT_006d4af0[];
extern unsigned char DAT_006d4afa[];
extern unsigned char DAT_006d4b04[];
extern unsigned char DAT_006d4b0e[];
extern unsigned char DAT_006d4b18[];
extern unsigned char DAT_006d4b22[];
extern unsigned char DAT_006d4b2c[];
extern unsigned char DAT_006d4b36[];
extern unsigned char DAT_006d4b3a[];
extern unsigned char DAT_006d4a02;
extern unsigned char DAT_0070ad04[];
extern unsigned char DAT_0070ad05[];
extern unsigned char DAT_0070ad06[];
extern unsigned char DAT_0070ad07[];
extern unsigned char DAT_0070ad08[];
extern unsigned char DAT_0070ad40[];
extern unsigned char DAT_0070ad41[];
extern unsigned char DAT_0070ad42[];
extern unsigned char DAT_0070ad43[];
extern unsigned char DAT_0070ad48[];
extern unsigned char DAT_0070ad51[];
extern unsigned char DAT_0070ad54[];
extern unsigned char DAT_0070ad55[];
extern unsigned char DAT_0070ad5d[];
extern unsigned char DAT_0070ad5e[];
extern unsigned char DAT_0070ad76[];
extern unsigned char DAT_0070ad77[];
extern unsigned char DAT_0070b252[];
extern unsigned char DAT_005247e8[];
extern unsigned char DAT_0052686b[];
extern unsigned char DAT_0052687a[];
extern unsigned char DAT_005248f8[];
extern unsigned char DAT_00524900[];
extern unsigned char DAT_007092d4;
extern unsigned char DAT_00709f30[];
extern unsigned char DAT_00709f3a[];
extern unsigned char DAT_007092e8[];
extern unsigned char DAT_00709310[];
extern unsigned char DAT_00709338[];
extern unsigned char DAT_00709388[];
extern unsigned char DAT_00709cec[];
extern unsigned char DAT_00709d0a[];
extern unsigned char DAT_00709d28[];
extern unsigned char DAT_00709b08[];
extern unsigned char DAT_0072f508[];
extern unsigned char DAT_0071b741[];
extern unsigned char DAT_0071b744[];
extern unsigned char DAT_0071b745[];
extern unsigned char DAT_0071b749[];
extern unsigned char DAT_0071b74a[];
extern unsigned char DAT_0071b74c[];
extern unsigned char DAT_0071b74e[];
extern unsigned char DAT_0071b752[];
extern unsigned char DAT_0071b759[];
extern unsigned char DAT_0071b75b[];
extern unsigned char DAT_0071b75d[];
extern unsigned char DAT_0071b75f[];
extern unsigned char DAT_0071b77b[];
extern unsigned char DAT_0071b77d[];
extern unsigned char DAT_0071b77f[];
extern unsigned char DAT_0071b781[];
extern unsigned char DAT_0071b783[];
extern unsigned char DAT_0071bae7[];
extern unsigned char DAT_0071bb19[];
extern unsigned char DAT_0071bb1a[];
extern unsigned short DAT_006db586;
extern unsigned short DAT_006db7a4;
extern int DAT_0070acd0;
extern unsigned short _DAT_0070a13c;
extern unsigned int DAT_0071e6a0;
extern unsigned short _DAT_00732e5a;
extern unsigned int _DAT_0071e6a4;
extern unsigned char DAT_00525580;
extern unsigned short DAT_006fedd0;
extern unsigned int DAT_0071e6a8;
extern unsigned int DAT_0071e6ac;
extern unsigned int DAT_0071e6b0;
extern unsigned short DAT_0071e6b4;
extern unsigned char DAT_0070ac40[];
extern unsigned char DAT_0070ac50[];
extern unsigned char DAT_0071bcdd[];
extern unsigned char DAT_0071eac5[];
extern unsigned char DAT_0071bb39[];
extern unsigned char DAT_0071e921[];
extern unsigned char DAT_0071b775[];
extern unsigned char DAT_0071e55d[];
extern unsigned char DAT_00703ac4;
extern unsigned short DAT_006db592;
extern unsigned short DAT_006ffbe4;
extern unsigned char DAT_007327e0;
extern unsigned short DAT_0070a1d2;
extern unsigned short DAT_0071e6be;
extern unsigned short _DAT_006db584;
extern unsigned char DAT_0070a178;
extern unsigned int DAT_006db5a8;
extern unsigned char DAT_00709f44[];
extern unsigned char DAT_00709f84[];
extern unsigned char _DAT_00709ce9[];
extern unsigned char DAT_007092c0;
extern unsigned int DAT_007092e4;
extern unsigned char DAT_00709ce8;
extern unsigned char DAT_00709ceb;

#define ST16(p, v) do { t16 = (unsigned short)(v); __builtin_memcpy((p), &t16, 2); } while (0)
#define ST32(p, v) do { t32 = (unsigned int)(v); __builtin_memcpy((p), &t32, 4); } while (0)

void __stdcall FUN_0041d620(void)
{
  unsigned char bVar1;
  char cVar2;
  char cVar3;
  char cVar4;
  unsigned int uVar5;
  char *pcVar6;
  int iVar7;
  unsigned int uVar8;
  int iVar13;
  int iVar14;
  unsigned short *puVar15;
  unsigned short *puVar16;
  unsigned char bVar17;
  int iVar18;
  unsigned int uVar19;
  int iVar20;
  unsigned char *puVar21;
  unsigned char *puVar11;
  unsigned char *puVar9;
  unsigned char *puVar10;
  char cVar12;
  unsigned short local_12;
  unsigned short *local_10;
  unsigned int *local_c;
  unsigned int local_4;
  int iTmp;
  unsigned short t16;
  unsigned int t32;

  FUN_004a6f90();
  DAT_0073b374 = 0;
  DAT_006db6c0 = 1;
  DAT_00536e8c = 1;
  if (0 < DAT_006d49a0) {
    do {
      uVar5 = (unsigned int)DAT_0073b374;
      puVar21 = DAT_006d4a60 + uVar5 * 0x126;
      puVar11 = DAT_006d4a61 + uVar5 * 0x126;
      iVar7 = (int)(uVar5 * 0x126);
      *puVar21 = 0;
      DAT_006d4a64[iVar7] = 0;
      *puVar11 = 0;
      DAT_006d4a68[uVar5 * 0x93] = 0;
      *(unsigned short *)(DAT_006d4a62 + iVar7) = 0;
      iVar20 = (int)(uVar5 * 0x566);
      local_12 = (unsigned short)DAT_0070ad43[iVar20];
      local_4 = (unsigned int)DAT_0070ad42[iVar20];
      iVar18 = (int)((unsigned int)DAT_005247e8[local_4] * 0x16);
      if (DAT_0052686b[iVar18] == 8) {
        local_12 = (unsigned short)(local_12 * local_12);
      }
      local_4 = (unsigned int)DAT_007092d4;
      bVar1 = DAT_005248f8[DAT_0070ad5d[iVar20]];
      bVar17 = DAT_00524900[local_4];
      __builtin_memcpy(&iTmp, DAT_0070b252 + iVar20, 4);
      if (((bVar1 & bVar17) == 0) && (iTmp == -1)) {
        DAT_006d4a6a[uVar5 * 0x93] = 0;
        DAT_006d4aa2[iVar7] = 0;
      }
      else {
        cVar2 = (char)DAT_0070ad41[iVar20];
        DAT_006d4a6a[uVar5 * 0x93] = (unsigned short)(((unsigned int)DAT_0070ad51[iVar20] + 1) * local_12);
        if ((cVar2 == '\0') && (DAT_0070ad43[iVar20] <= DAT_0070ad40[iVar20])) {
          DAT_006d4aa2[iVar7] = 0;
        }
        else {
          DAT_006d4aa2[iVar7] = (unsigned char)(DAT_0070ad51[iVar20] + 1);
        }
      }
      iVar14 = 1;
      do {
        iVar13 = iVar14 + 1;
        DAT_006d4a6a[iVar14 + uVar5 * 0x93] = 0;
        DAT_006d4aa2[iVar7 + iVar14] = 0;
        iVar14 = iVar13;
      } while (iVar13 < 0x1c);
      iVar14 = 0;
      do {
        DAT_006d4abe[iVar7 + iVar14] = 0;
        iVar13 = iVar14 + 1;
        DAT_006d4ac8[iVar7 + iVar14] = 0;
        DAT_006d4ad2[iVar7 + iVar14] = 0;
        DAT_006d4adc[iVar7 + iVar14] = 0;
        DAT_006d4ae6[iVar7 + iVar14] = 0;
        DAT_006d4af0[iVar7 + iVar14] = 0;
        DAT_006d4afa[iVar7 + iVar14] = 0;
        DAT_006d4b04[iVar7 + iVar14] = 0;
        DAT_006d4b0e[iVar7 + iVar14] = 0;
        DAT_006d4b18[iVar7 + iVar14] = 0;
        DAT_006d4b22[iVar7 + iVar14] = 0;
        DAT_006d4b2c[iVar7 + iVar14] = 0;
        iVar14 = iVar13;
      } while (iVar13 < 10);
      iVar14 = 10;
      do {
        iVar14 = iVar14 + -1;
        ST32(DAT_006d4b36 + iVar7, 0x4040404);
        ST32(DAT_006d4b3a + iVar7, 0x4040404);
        iVar7 = iVar7 + 8;
      } while (iVar14 != 0);
      __builtin_memcpy(&iTmp, DAT_0070b252 + iVar20, 4);
      if ((DAT_0070ad42[iVar20] != '\0') &&
          ((((bVar1 & bVar17) != 0) || (iTmp != -1)) &&
           (DAT_006d4a6a[uVar5 * 0x93] != 0))) {
        __builtin_memcpy(&iTmp, DAT_0070b252 + iVar20, 4);
        if (iTmp == -1) {
          if ((((DAT_0070ad5e[iVar20] == '\0') && (DAT_0070ad76[iVar20] == '\0')) &&
               (DAT_0070ad77[iVar20] == '\0')) &&
              ((DAT_0070ad54[iVar20] == '\0') || (DAT_0070ad55[iVar20] == '\0'))) {
            *puVar21 = 1;
            if ((DAT_0052687a[iVar18] & 0x80) != 0) {
              if (DAT_006d4a02 != 2) goto LAB_0041d93b;
              *puVar11 = DAT_0070ad51[iVar20];
              goto LAB_0041d93e;
            }
            *puVar11 = DAT_0070ad51[iVar20];
            FUN_0041eea0(0xffff);
          }
        }
        else {
          *puVar21 = 1;
          if ((DAT_0052687a[iVar18] & 0x80) == 0) {
            *puVar11 = DAT_0070ad51[iVar20];
            FUN_0041eea0(0xffff);
          }
          else {
            if (DAT_006d4a02 == 2) {
              *puVar11 = DAT_0070ad51[iVar20];
            }
            else {
LAB_0041d93b:
              *puVar11 = 0;
            }
LAB_0041d93e:
            FUN_004215c0(0xffff);
          }
        }
        if ((DAT_0052687a[(unsigned int)DAT_005247e8[DAT_0070ad42[(unsigned int)DAT_0073b374 * 0x566]] * 0x16] & 0x20) == 0) {
          DAT_00709f30[DAT_0070ad48[(unsigned int)DAT_0073b374 * 0x566]] = 1;
        }
      }
      DAT_0073b374 = (unsigned short)(DAT_0073b374 + 1);
    } while ((int)(unsigned int)DAT_0073b374 < (int)DAT_006d49a0);
  }
  iVar7 = 0;
  puVar15 = (unsigned short *)DAT_00709388;
  local_4 = 0;
  local_10 = (unsigned short *)DAT_00709338;
  local_c = (unsigned int *)DAT_007092e8;
  iVar20 = 0;
  do {
    DAT_00709f3a[local_4] = 0;
    *local_c = 0;
    local_c[10] = 0;
    local_c[0x308] = 0;
    iVar18 = 0;
    do {
      iVar14 = iVar18 + iVar20;
      DAT_00709cec[iVar14] = 4;
      DAT_00709d0a[iVar14] = 0;
      iVar13 = 4;
      puVar16 = (unsigned short *)DAT_00709d28 + iVar14 * 4;
      do {
        *puVar16 = 0;
        puVar16[0x78] = 0;
        iVar13 = iVar13 + -1;
        puVar16 = puVar16 + 1;
      } while (iVar13 != 0);
      iVar18 = iVar18 + 1;
    } while (iVar18 < 3);
    *local_10 = 0;
    local_10[10] = 0;
    local_10[0x14] = 0;
    local_10[0x1e] = 0;
    puVar16 = puVar15;
    iVar18 = 0;
    do {
      *puVar16 = 0;
      puVar15 = puVar16 + 1;
      puVar16[0x1e0] = 0;
      iVar14 = iVar18 + 1;
      DAT_00709b08[iVar7 + iVar18] = 0;
      puVar16 = puVar15;
      iVar18 = iVar14;
    } while (iVar14 < 0x30);
    iVar7 = iVar7 + 0x30;
    local_10 = local_10 + 1;
    iVar20 = iVar20 + 3;
    local_c = local_c + 1;
    local_4 = local_4 + 1;
  } while (local_c < (unsigned int *)DAT_00709310);
  DAT_0073b374 = 0;
  if (0 < DAT_006d49a0) {
    iVar7 = (int)DAT_006d49a0;
    do {
      uVar5 = (unsigned int)DAT_0073b374;
      iVar20 = (int)(uVar5 * 0x566);
      cVar2 = (char)DAT_0070ad04[iVar20];
      uVar8 = (unsigned int)DAT_0070ad48[iVar20];
      for (iVar18 = 0; (cVar2 != '\0' && (iVar18 < 0x10)); iVar18 = iVar18 + 4) {
        bVar17 = 0xff;
        bVar1 = DAT_0070ad04[iVar20 + iVar18];
        if (bVar1 == 0x41) {
          bVar17 = 10;
        }
        else if (bVar1 == 0x4f) {
          bVar17 = 0xb;
        }
        else if (bVar1 == 0x46) {
          bVar17 = 0xc;
        }
        else if (bVar1 == 0x48) {
          bVar17 = 0xd;
        }
        else if ((0x30 < bVar1) && (bVar1 < 0x3a)) {
          bVar17 = (unsigned char)(bVar1 - 0x31);
        }
        if (bVar17 != 0xff) {
          cVar2 = (char)DAT_0070ad05[iVar20 + iVar18];
          cVar3 = (char)DAT_0070ad06[iVar20 + iVar18];
          cVar4 = (char)DAT_0070ad07[iVar20 + iVar18];
          cVar12 = '\0';
          if (((cVar2 == 'C') && (cVar3 == 'O')) && (cVar4 == 'M')) {
            cVar12 = '\x01';
          }
          else if (((cVar2 == 'B') && (cVar3 == 'A')) && (cVar4 == 'S')) {
            cVar12 = '\x02';
          }
          else if (((cVar2 == 'S') && (cVar3 == 'T')) && (cVar4 == 'A')) {
            cVar12 = '\x03';
          }
          else if (((cVar2 == 'M') && (cVar3 == 'I')) && (cVar4 == 'S')) {
            cVar12 = '\x04';
          }
          else if (((cVar2 == 'C') && (cVar3 == 'O')) && (cVar4 == 'N')) {
            cVar12 = '\x05';
          }
          else if (((cVar2 == 'S') && (cVar3 == 'T')) && (cVar4 == 'R')) {
            cVar12 = '\x06';
          }
          else if (((cVar2 == 'R') && (cVar3 == 'E')) && (cVar4 == 'L')) {
            cVar12 = '\a';
          }
          else if (((cVar2 == 'P') && (cVar3 == 'R')) && (cVar4 == 'I')) {
            cVar12 = '\b';
          }
          else if (((cVar2 == 'S') && (cVar3 == 'E')) && (cVar4 == 'C')) {
            cVar12 = '\t';
          }
          else if (((cVar2 == 'T') && (cVar3 == 'E')) && (cVar4 == 'R')) {
            cVar12 = '\n';
          }
          else if (((cVar2 == 'R') && (cVar3 == 'E')) && (cVar4 == 'S')) {
            cVar12 = '\v';
          }
          else if (((cVar2 == 'M') && (cVar3 == 'A')) && (cVar4 == 'N')) {
            cVar12 = '\f';
          }
          if (cVar12 != '\0') {
            if (bVar17 < 10) {
              DAT_00709b08[uVar5 + (unsigned int)bVar17 * 0x30] = (unsigned char)cVar12;
            }
            else {
              uVar19 = 0;
              pcVar6 = (char *)DAT_00709b08 + uVar5;
              do {
                if (bVar17 == 10) {
LAB_0041dd37:
                  *pcVar6 = cVar12;
                }
                else {
                  if (bVar17 == 0xb) {
                    if (uVar8 == uVar19) goto LAB_0041dd39;
                    goto LAB_0041dd37;
                  }
                  if (bVar17 == 0xc) {
                    if (DAT_0072f508[uVar19 + uVar8 * 0x1e5] != '\0') goto LAB_0041dd37;
                  }
                  else if ((bVar17 == 0xd) && (DAT_0072f508[uVar19 + uVar8 * 0x1e5] == '\0'))
                    goto LAB_0041dd37;
                }
LAB_0041dd39:
                pcVar6 = pcVar6 + 0x30;
                uVar19 = uVar19 + 1;
              } while ((int)uVar19 < 10);
            }
          }
        }
        cVar2 = (char)DAT_0070ad08[iVar20 + iVar18];
      }
      DAT_0073b374 = (unsigned short)(DAT_0073b374 + 1);
    } while ((int)(unsigned int)DAT_0073b374 < iVar7);
  }
  iVar7 = 0;
  do {
    DAT_0071b741[iVar7] = 0;
    DAT_0071b749[iVar7] = 1;
    ST16(DAT_0071b74a + iVar7, 0xffff);
    ST16(DAT_0071b74c + iVar7, 0xffff);
    ST16(DAT_0071b759 + iVar7, 0xffff);
    ST16(DAT_0071b75b + iVar7, 0xffff);
    ST32(DAT_0071b74e + iVar7, 0xffffffffu);
    ST32(DAT_0071b752 + iVar7, 0xffffffffu);
    ST16(DAT_0071b75d + iVar7, 0xffff);
    ST16(DAT_0071b75f + iVar7, 0);
    ST16(DAT_0071b77b + iVar7, 0);
    ST16(DAT_0071b77d + iVar7, 0);
    ST16(DAT_0071b77f + iVar7, 0);
    ST16(DAT_0071b781 + iVar7, 0);
    ST16(DAT_0071b783 + iVar7, 0);
    DAT_0071b744[iVar7] = 0;
    ST32(DAT_0071b745 + iVar7, 0);
    puVar9 = DAT_0071bae7 + iVar7;
    for (iVar20 = 0xc; iVar20 != 0; iVar20 = iVar20 + -1) {
      ST32(puVar9, 0);
      puVar9 = puVar9 + 4;
    }
    ST16(puVar9, 0);
    DAT_0071bb19[iVar7] = 0;
    iVar20 = iVar7 + 0x5bd;
    DAT_0071bb1a[iVar7] = 0;
    iVar7 = iVar20;
  } while (iVar20 < 0x2de8);
  DAT_006db586 = (unsigned short)DAT_006db7a4;
  if (DAT_0070acd0 == 0) {
    FUN_004b1730();
  }
  _DAT_0070a13c = 0;
  DAT_0071e6a0 = 0;
  _DAT_00732e5a = 0;
  _DAT_0071e6a4 = 0;
  DAT_00525580 = 1;
  DAT_006fedd0 = 0xffff;
  DAT_0071e6a8 = 0;
  DAT_0071e6ac = 0;
  DAT_0071e6b0 = 0;
  DAT_0071e6b4 = 0;
  puVar15 = (unsigned short *)DAT_0070ac40;
  do {
    iVar7 = 8;
    puVar16 = puVar15;
    do {
      *puVar16 = 0;
      puVar16 = puVar16 + 8;
      iVar7 = iVar7 + -1;
    } while (iVar7 != 0);
    puVar15 = puVar15 + 1;
  } while (puVar15 < (unsigned short *)DAT_0070ac50);
  puVar9 = DAT_0071bcdd;
  do {
    ST32(puVar9, 0);
    puVar10 = puVar9 + 0x5bd;
    ST32(puVar9 + 4, 0);
    puVar9 = puVar10;
  } while (puVar10 < DAT_0071eac5);
  puVar21 = DAT_0071bb39;
  do {
    *puVar21 = 0;
    puVar11 = puVar21 + 0x5bd;
    ST16(puVar21 + -6, 0);
    ST16(puVar21 + -4, 0);
    ST16(puVar21 + 0xb, 0);
    ST32(puVar21 + 0xd, 0x400);
    ST16(puVar21 + 7, 0);
    ST16(puVar21 + 0x11, 0);
    __builtin_memcpy(&t32, puVar21 + -0x409, 4);
    ST16(puVar21 + -0x12, t32);
    puVar21 = puVar11;
  } while (puVar11 < DAT_0071e921);
  puVar21 = DAT_0071b775;
  DAT_00703ac4 = 0xff;
  iVar7 = 0;
  do {
    *puVar21 = 0;
    puVar21 = puVar21 + 0x5bd;
    FUN_004b1bc0(0, iVar7);
    iVar7 = iVar7 + 1;
  } while (puVar21 < DAT_0071e55d);
  DAT_006db592 = 0;
  DAT_006ffbe4 = 1;
  DAT_007327e0 = 1;
  DAT_0070a1d2 = 0xf;
  DAT_0071e6be = 0xf;
  _DAT_006db584 = 0xf;
  FUN_0048f530();
  DAT_0070a178 = 0;
  DAT_006db5a8 = 0;
  iVar7 = 0;
  do {
    DAT_00709f44[iVar7] = 0;
    iVar20 = iVar7 + 1;
    DAT_00709f84[iVar7] = 0;
    iVar7 = iVar20;
  } while (iVar20 < 0x40);
  ST16(_DAT_00709ce9, 0);
  DAT_007092c0 = 0;
  DAT_007092e4 = 0;
  DAT_00709ce8 = 0;
  DAT_00709ceb = 0;
  DAT_00536e8c = 0;
  FUN_004a6a80();
  return;
}
