/* SPDX-License-Identifier: MIT */
/* z_xvt 0044ff80 */
char __cdecl FUN_0049d8b0(unsigned short, unsigned short, unsigned short, unsigned short, int, int);
unsigned int __cdecl FUN_00461f20(int, int, int, int);
void __cdecl FUN_0048eda0(unsigned int);
void __cdecl FUN_0048ed00(int, int, int, int);
unsigned int __stdcall FUN_0043fdf0(void);
void __cdecl FUN_0048ee00(int);
void __cdecl FUN_0048ee10(int);
void __cdecl FUN_0048ee20(char);
void __cdecl FUN_0048efd0(char *);
void __cdecl FUN_0048f040(int);
void __cdecl FUN_0048f000(char *);
void __cdecl FUN_0048ece0(int, int);
void __cdecl FUN_0048ed70(unsigned int);
void __cdecl FUN_0048f270(unsigned char *);
int _sprintf(char *, const char *, ...);
void __cdecl FUN_0048f140(unsigned char *);
unsigned char * __cdecl FUN_0048f4f0(unsigned int);
unsigned long long __cdecl FUN_004b4060(unsigned char *);
unsigned short __cdecl FUN_004513b0(unsigned int, unsigned int, unsigned int,
                                    unsigned short, unsigned short, unsigned int,
                                    unsigned short, unsigned short, unsigned char *,
                                    unsigned int, int);
void __stdcall FUN_0048f510(void);

extern int DAT_0071ecf4;
extern char DAT_0071b743[];
extern unsigned char DAT_0071b73c[];
extern unsigned short DAT_00655a94, DAT_00655a9c, DAT_00655aa2, DAT_00655a90;
extern unsigned short DAT_006551cc, DAT_00655188, DAT_00655186, DAT_006551c8;
extern int DAT_006da3b0[];
extern unsigned char DAT_00709cec[];
extern unsigned char DAT_00525d18[];
extern unsigned char DAT_00525d30[];
extern unsigned char DAT_006daa00[];
extern unsigned char DAT_006daa01[];
extern unsigned char DAT_006daa02[];
extern unsigned char DAT_006daa03[];
extern unsigned char DAT_006daa29[];
extern unsigned char DAT_0070aee9[];
extern unsigned char DAT_0070aee5[];
extern unsigned char DAT_0070aee6[];
extern unsigned char DAT_0070aee7[];
extern signed char DAT_0070aee8[];
extern unsigned char DAT_0070aef3[];
extern unsigned char DAT_006d4b36[];
extern int DAT_006d49a0;
extern int DAT_006da9ec, DAT_006da538, DAT_006db594, DAT_00525518;
extern unsigned short DAT_00655184;
extern short DAT_00656d68[];
extern short DAT_006559b2;
extern unsigned short DAT_00525508;
extern void (*DAT_00703c80)(void);
extern short DAT_0052f0e0, DAT_0052f0e4;
extern int DAT_007092b0;
extern int DAT_005f0918, DAT_005f08e4, DAT_005f08dc, DAT_005f08e0, DAT_005f0908;
extern int DAT_005f08e8[];
extern char DAT_005f0910, DAT_005f08d8;
extern char DAT_00709d0a[];
extern char DAT_00709d0b[];
extern short DAT_0070ac4e[]; /* accessed at byte offset DAT_0071ecf4 * 0x10 */
extern char *DAT_006cf480;
extern char *DAT_006cf470[];
extern unsigned char DAT_006d4870[];
extern unsigned char DAT_00525d48[];
extern unsigned int DAT_00656c34[];
extern char s___s__ld__00525dd8[];
extern char s___ld____00525dcc[];
extern unsigned short DAT_0071b020[];
extern unsigned short DAT_0070a1e0[];
extern unsigned short DAT_00709e18[];
extern unsigned short DAT_00709d28[];
extern short DAT_00732e58;
extern int DAT_0070acc0;
extern short DAT_006d4a4e;
extern void (*DAT_006d9984)(int);
extern void (*DAT_0071eae0)(int, int, int, int, int);
extern int DAT_00524900[];
extern int DAT_005248f8[];
extern int DAT_007092d4;
extern unsigned char DAT_0070ad5d[];

unsigned int __stdcall FUN_0044ff80(void)
{
  char *pcVar1;
  int iVar2;
  unsigned short *puVar3;
  unsigned char bVar4;
  unsigned char bVar5;
  int bVar6;
  char cVar7;
  unsigned short uVar8;
  unsigned short uVar9;
  unsigned int uVar10;
  int iVar11;
  int iVar12;
  unsigned int uVar13;
  unsigned int uVar14;
  int iVar16;
  void *pvVar17;
  short sVar18;
  int iVar19;
  unsigned short uVar20;
  unsigned int uVar21;
  short sVar23;
  short sVar24;
  unsigned char *pbVar25;
  unsigned int uVar26;
  int iVar27;
  unsigned long long uVar28;
  unsigned int uVar29;
  unsigned short local_a0;
  unsigned short local_a2;
  short sStack_9c;
  unsigned short local_9a;
  short sStack_98;
  short sStack_96;
  unsigned int uStack_94;
  short local_8e;
  short sStack_8c;
  short local_8a;
  short sStack_88;
  unsigned short uStack_86;
  short sStack_84;
  short local_82;
  int local_7c;
  unsigned int auStack_50[20];
  unsigned short us_tmp;

  if (DAT_0071b743[DAT_0071ecf4 * 0x5bd] == '\0') {
    sStack_8c = (short)(DAT_00655a94 + 2);
    local_8e = (short)(DAT_00655a94 - 2 + DAT_00655a9c);
    local_8a = (short)(DAT_00655aa2 + 2);
    sStack_88 = (short)(DAT_00655aa2 - 2 + DAT_00655a90);
  }
  else {
    sStack_8c = (short)(DAT_006551cc + 2);
    local_8e = (short)(DAT_006551cc - 2 + DAT_00655188);
    local_8a = (short)(DAT_00655186 + 2);
    sStack_88 = (short)(DAT_00655186 - 2 + DAT_006551c8);
  }
  __builtin_memcpy(&us_tmp, &DAT_0071b73c[DAT_0071ecf4 * 0x5bd], 2);
  uVar10 = (unsigned int)us_tmp;
  local_7c = 0;
  local_82 = 0;
  do {
    iVar27 = (int)local_82;
    uStack_86 = 0;
    DAT_006da3b0[iVar27] = 0;
    do {
      uVar29 = (unsigned int)uStack_86;
      if ((DAT_00709cec[uVar10 * 3 + uVar29] != 0) &&
          (iVar19 = (uVar29 + iVar27 * 3) * 2,
           *(unsigned short *)(DAT_00525d18 + iVar19) ==
               (unsigned short)(unsigned char)DAT_00709cec[uVar10 * 3 + uVar29])) {
        sVar23 = 0;
        do {
          sVar24 = 0;
          do {
            iVar11 = (uVar10 * 7 + uVar29) * 0x2a + sVar24 * 4;
            iVar2 = iVar11 + sVar23 * 0xb;
            uVar8 = (unsigned short)DAT_006daa00[iVar2];
            if (((uVar8 != 10) && (uVar8 != 0)) &&
                (cVar7 = FUN_0049d8b0(uVar8, (unsigned short)DAT_006daa01[sVar23 * 0xb + iVar11],
                                      (unsigned short)DAT_006daa02[iVar2],
                                      (unsigned short)DAT_006daa03[iVar2], 0, 10),
                 (*(unsigned short *)(DAT_00525d30 + iVar19) &
                  (unsigned short)(unsigned char)cVar7) != 0)) {
              iVar11 = DAT_006da3b0[iVar27];
              DAT_006da3b0[iVar27] = iVar11 + 1;
              if (uStack_86 == 2) {
                DAT_006da3b0[iVar27] = iVar11 + 2;
              }
            }
            sVar24 = sVar24 + 1;
          } while (sVar24 < 2);
          sVar23 = sVar23 + 1;
        } while (sVar23 < 2);
      }
      local_9a = 0;
      uVar8 = *(unsigned short *)(DAT_00525d18 + (uVar29 + iVar27 * 3) * 2);
      do {
        sVar23 = 0;
        if (0 < DAT_006d49a0) {
          do {
            uVar21 = (unsigned int)local_9a * 0x4e + sVar23 * 0x566;
            if (((DAT_0070aee9[uVar21 + uVar10] != 0) &&
                 ((unsigned char)DAT_0070aee5[uVar21] == uVar29)) &&
                ((unsigned char)DAT_006d4b36[(unsigned int)local_9a + uVar10 * 8 + sVar23 * 0x126] == uVar8)) {
              if (uStack_86 == 2) {
                if ((local_82 == 0) && ((signed char)DAT_0070aee8[uVar21] * 0xfa < 0)) {
                  iVar19 = DAT_006da3b0[0] + 2;
                }
                else {
                  if ((local_82 != 3) ||
                      (pcVar1 = (char *)&DAT_0070aee8[uVar21], uVar21 = *pcVar1 * 5, *pcVar1 * 0xfa < 0))
                    goto LAB_00450293;
                  iVar19 = DAT_006da3b0[3] + 2;
                }
              }
              else {
                iVar19 = DAT_006da3b0[iVar27] + 1;
              }
              DAT_006da3b0[iVar27] = iVar19;
            }
LAB_00450293:
            sVar23 = sVar23 + 1;
          } while (sVar23 < DAT_006d49a0);
        }
        local_9a = local_9a + 1;
      } while (local_9a < 8);
      if (DAT_006da3b0[iVar27] != 0) {
        DAT_006da3b0[iVar27] = DAT_006da3b0[iVar27] + 1;
      }
      uStack_86 = uStack_86 + 1;
      local_7c = local_7c + DAT_006da3b0[iVar27];
    } while (uStack_86 < 3);
    local_82 = local_82 + 1;
  } while (local_82 < 4);
  FUN_00461f20(DAT_006da9ec, DAT_006da538, DAT_006db594,
               (int)((unsigned int)DAT_00525518 * (unsigned int)DAT_006da538));
  if (DAT_00656d68[DAT_00655184] != DAT_006559b2) {
    if (DAT_0071b743[DAT_0071ecf4 * 0x5bd] == '\0') {
      uVar29 = (unsigned int)DAT_00525508;
    }
    else {
      uVar29 = 0x34;
    }
    FUN_0048eda0(uVar29);
    FUN_0048ed00(sStack_8c + -2, local_8a + -2, local_8e + 2, sStack_88 + 2);
    (*DAT_00703c80)();
    if (DAT_006559b2 == -2) {
      if (DAT_0052f0e0 == 1) {
        DAT_0052f0e0 = DAT_0052f0e4;
        uVar21 = FUN_0043fdf0();
        DAT_0052f0e4 = (short)uVar21;
      }
      uVar21 = FUN_00461f20(0, 0x140, 200, 0);
      return uVar21 & 0xffff0000;
    }
  }
  FUN_0048ed00(sStack_8c, local_8a, local_8e, sStack_88);
  FUN_0048ee00(1);
  FUN_0048ee10(1);
  FUN_0048ee20('\0');
  if (DAT_0071b743[DAT_0071ecf4 * 0x5bd] == '\0') {
    uVar29 = (unsigned int)DAT_00525508;
  }
  else {
    uVar29 = 0x34;
  }
  FUN_0048eda0(uVar29);
  sVar23 = DAT_006d4a4e + 2;
  if (DAT_00656d68[DAT_00655184] == DAT_006559b2) {
    if (DAT_0052f0e0 == 1) {
      if (DAT_007092b0 == 0xa2) {
        if (0 < DAT_005f0918) {
          DAT_005f08e4 = 1;
          DAT_005f0918 = DAT_005f0918 + -1;
        }
      }
      else if ((DAT_007092b0 == 0xa3) &&
               (DAT_005f0918 <= DAT_005f08dc + ((int)local_8a - (int)sStack_88) / (int)sVar23)) {
        DAT_005f08e4 = 1;
        DAT_005f0918 = DAT_005f0918 + 1;
      }
    }
    if (DAT_005f08e0 != local_7c) {
      DAT_005f08e4 = 1;
      DAT_005f08e0 = local_7c;
    }
    sVar24 = 0;
    do {
      iVar27 = (int)sVar24;
      if (DAT_005f08e8[iVar27] != DAT_006da3b0[sVar24]) {
        DAT_005f08e4 = 1;
        break;
      }
      sVar24 = sVar24 + 1;
    } while (sVar24 < 8);
    __builtin_memcpy(&us_tmp, &DAT_0071b73c[DAT_0071ecf4 * 0x5bd], 2);
    uVar29 = (unsigned int)us_tmp;
    if ((DAT_00709d0a[uVar29 * 3] != DAT_005f0910) ||
        (DAT_00709d0b[uVar29 * 3] != DAT_005f08d8)) {
      DAT_005f08e4 = 1;
    }
  }
  else {
    DAT_005f0918 = 0;
    DAT_005f0908 = 0;
    DAT_005f08e0 = 0;
    DAT_005f08dc = 0;
    *(short *)((char *)DAT_0070ac4e + DAT_0071ecf4 * 0x10) = 0x1d8;
    DAT_005f08e4 = 1;
    DAT_005f0910 = -1;
    DAT_005f08d8 = -1;
  }
  if (*(short *)((char *)DAT_0070ac4e + DAT_0071ecf4 * 0x10) < 1) {
    DAT_005f08e4 = 1;
    *(short *)((char *)DAT_0070ac4e + DAT_0071ecf4 * 0x10) = 0x1d8;
  }
  uStack_94 = ((unsigned int)DAT_005f0918 + 0xffffffffu +
               (unsigned int)(((int)sStack_88 - (int)local_8a) / (int)sVar23)) * 0x10000u;
  if (DAT_005f08e4 != 0) {
    FUN_0048ed00(sStack_8c + -2, local_8a + -2, local_8e + 2, sStack_88 + 2);
    (*DAT_00703c80)();
    iVar19 = 0;
    FUN_0048ed00(sStack_8c, local_8a, local_8e, sStack_88);
    sVar24 = 4;
    FUN_0048efd0(DAT_006cf480);
    __builtin_memcpy(&us_tmp, &DAT_0071b73c[DAT_0071ecf4 * 0x5bd], 2);
    uVar29 = (unsigned int)us_tmp;
    cVar7 = DAT_00709d0a[uVar29 * 3];
    iVar27 = uVar29 * 3;
    if (cVar7 == '\0') {
      cVar7 = DAT_00709d0b[iVar27];
      if (cVar7 == '\0') {
LAB_00450aa6:
        sVar24 = 7;
        auStack_50[0] = 0x4e;
      }
      else if (cVar7 == '\x01') {
        sVar24 = 6;
        auStack_50[0] = 0x4a;
      }
      else if (cVar7 == '\x02') goto LAB_00450aa6;
    }
    else if (cVar7 == '\x01') {
      cVar7 = DAT_00709d0b[iVar27];
      if (cVar7 == '\0') {
LAB_00450adc:
        sVar24 = 5;
        auStack_50[0] = 0x52;
      }
      else if (cVar7 == '\x01') {
        sVar24 = 8;
        auStack_50[0] = 0x46;
      }
      else if (cVar7 == '\x02') goto LAB_00450adc;
    }
    else if (cVar7 == '\x02') {
      sVar24 = 6;
      auStack_50[0] = 0x4a;
    }
    FUN_0048f040(0x20);
    FUN_0048f000(DAT_006cf470[sVar24]);
    FUN_0048ece0(sStack_8c, local_8a);
    FUN_0048ed70(auStack_50[0]);
    FUN_0048f270(DAT_006d4870);
    local_82 = 0;
    sStack_84 = local_8a + sVar23;
    do {
      iVar27 = (int)local_82;
      FUN_0048ed70((unsigned int)(unsigned char)DAT_00525d48[iVar27]);
      uStack_86 = 0;
      bVar6 = 1;
      do {
        if (DAT_006da3b0[iVar27] != 0) {
          uVar29 = (unsigned int)uStack_86;
          iVar11 = uVar29 + uVar10 * 3;
          uVar21 = uVar29;
          if ((DAT_00709cec[iVar11] != 0) &&
              (pvVar17 = (void *)((iVar27 * 3 + uVar29) * 2),
               *(unsigned short *)(DAT_00525d18 + (int)pvVar17) ==
                   (unsigned short)(unsigned char)DAT_00709cec[iVar11])) {
            sStack_96 = 0;
            iVar16 = uVar10 * 7 + uVar29;
            local_9a = 0;
            iVar2 = iVar16 * 0x2a;
            do {
              sStack_98 = 0;
              do {
                iVar12 = sStack_96 * 0xb + iVar2 + sStack_98 * 4;
                bVar4 = DAT_006daa01[iVar12];
                uVar9 = (unsigned short)bVar4;
                bVar5 = DAT_006daa02[iVar12];
                uVar8 = (unsigned short)DAT_006daa03[iVar12];
                uStack_94 = (uStack_94 & 0xffff0000) | uVar8;
                uVar20 = (unsigned short)DAT_006daa00[iVar12];
                if (((uVar20 != 10) && (uVar20 != 0)) &&
                    (cVar7 = FUN_0049d8b0(uVar20, uVar9, (unsigned short)bVar5, uVar8, 0, 10),
                     (*(unsigned short *)(DAT_00525d30 + (int)pvVar17) &
                      (unsigned short)(unsigned char)cVar7) != 0)) {
                  if (uStack_86 == 2) {
                    iVar12 = (signed char)DAT_006daa29[iVar2] * 0xfa;
                    if (((local_82 == 0) && (-1 < iVar12)) || ((local_82 == 3) && (iVar12 < 0)))
                      goto LAB_00450e1d;
                    if (bVar6) {
                      if ((DAT_005f0918 <= iVar19) && (iVar19 < (short)(uStack_94 >> 16))) {
                        FUN_0048ece0(sStack_8c, sStack_84);
                        FUN_0048f270((unsigned char *)DAT_006cf470[iVar27]);
                        sStack_84 = sStack_84 + sVar23;
                      }
                      bVar6 = 0;
                      iVar19 = iVar19 + 1;
                    }
                    FUN_0048ece0(sStack_8c, sStack_84);
                    _sprintf((char *)auStack_50, s___s__ld__00525dd8,
                             DAT_00656c34[(unsigned int)(iVar12 < 0)], iVar12);
                    uVar13 = 0x52;
                    if (iVar12 < 0) {
                      uVar13 = 0x4a;
                    }
                    FUN_0048ed70(uVar13);
                    FUN_0048f140((unsigned char *)auStack_50);
                    FUN_0048ed70((unsigned int)(unsigned char)DAT_00525d48[iVar27]);
                  }
                  if (bVar6) {
                    if ((DAT_005f0918 <= iVar19) && (iVar19 < (short)(uStack_94 >> 16))) {
                      FUN_0048ece0(sStack_8c, sStack_84);
                      FUN_0048f270((unsigned char *)DAT_006cf470[iVar27]);
                      sStack_84 = sStack_84 + sVar23;
                    }
                    bVar6 = 0;
                    iVar19 = iVar19 + 1;
                  }
                  local_a0 = 0;
                  if ((DAT_005f0918 <= iVar19) && (iVar19 <= (short)(uStack_94 >> 16))) {
                    iVar12 = (int)(short)local_9a;
                    pbVar25 = (unsigned char *)0;
                    if (DAT_0071b020[(iVar12 + iVar16 * 4) * 3 +
                                     (*(unsigned short *)(DAT_00525d18 + (int)pvVar17) & 3)] != 0) {
                      pbVar25 = FUN_0048f4f0((unsigned int)DAT_0071b020[(iVar12 + iVar16 * 4) * 3 +
                                     (*(unsigned short *)(DAT_00525d18 + (int)pvVar17) & 3)]);
                    }
                    FUN_0048ece0(sStack_8c, sStack_84);
                    if ((local_82 == 2) && (uStack_86 == 1)) {
                      local_a0 = 5;
                    }
                    else {
                      local_a0 = *(unsigned short *)(DAT_00525d18 + (int)pvVar17);
                    }
                    if (uStack_86 == 2) {
                      uVar28 = FUN_004b4060((unsigned char *)auStack_50);
                      DAT_00732e58 = DAT_00732e58 + (short)uVar28;
                    }
                    if (pbVar25 == (unsigned char *)0) {
                      iVar12 = iVar12 + iVar11 * 4;
                      if (((((DAT_00709e18[iVar12] < 2) || (uVar20 == 6)) ||
                            (uVar20 == 7)) || ((uVar20 == 0x1e || (uVar20 == 0x1f)))) ||
                          (uVar20 == 0x20) || (uVar20 == 0x21)) {
                        uVar13 = 0xffffffff;
                      }
                      else {
                        uVar13 = ((unsigned int)DAT_00709d28[iVar12] * 100) /
                                 (unsigned int)DAT_00709e18[iVar12];
                      }
                      uVar14 = ((unsigned int)bVar4 << 16) | local_a0;
                      local_a0 = FUN_004513b0(((unsigned int)bVar5 << 16) | uVar9, uVar14,
                                              ((unsigned int)local_9a << 16) | (unsigned short)bVar5,
                                              uVar20, uVar9, uVar14, uVar8, 0,
                                              (unsigned char *)0, uVar13, iVar27);
                      sStack_84 = sStack_84 + local_a0;
                      if (local_a0 <= (unsigned short)(DAT_006d4a4e + 2)) {
                        local_a0 = 0;
                      }
                    }
                    else {
                      FUN_0048f140(pbVar25);
                      iVar12 = iVar12 + iVar11 * 4;
                      if (((((DAT_00709e18[iVar12] < 2) || (uVar20 == 6)) ||
                            ((uVar20 == 7 ||
                              (((uVar20 == 0x1e || (uVar20 == 0x1f)) || (uVar20 == 0x20)))))) ||
                           (uVar20 == 0x21))) {
                        (*DAT_006d9984)(10);
                        if ((pbVar25 == (unsigned char *)0) ||
                            (uVar28 = FUN_004b4060(pbVar25),
                             (int)(((unsigned int)uVar28 & 0xffff) + (int)DAT_00732e58) <= DAT_0070acc0 + -0xb)) {
                          local_a0 = 0;
                        }
                        else {
                          local_a0 = DAT_006d4a4e + 1;
                        }
                        sStack_84 = sStack_84 + DAT_006d4a4e + local_a0 + 2;
                      }
                      else {
                        _sprintf((char *)auStack_50, s___ld____00525dcc,
                                 ((unsigned int)DAT_00709d28[iVar12] * 100) /
                                     (unsigned int)DAT_00709e18[iVar12]);
                        if ((local_82 == 0) || (local_82 == 2)) {
                          uVar13 = 0x4a;
                        }
                        else if ((local_82 == 3) || (local_82 == 1)) {
                          uVar13 = 0x52;
                        }
                        else {
                          uVar13 = 0x43;
                        }
                        FUN_0048ed70(uVar13);
                        FUN_0048efd0((char *)pbVar25);
                        FUN_0048f000((char *)auStack_50);
                        FUN_0048f140((unsigned char *)auStack_50);
                        FUN_0048ed70((unsigned int)(unsigned char)DAT_00525d48[iVar27]);
                        (*DAT_006d9984)(10);
                        uVar28 = FUN_004b4060(DAT_006d4870);
                        if (DAT_0070acc0 + -0xb < (int)(((unsigned int)uVar28 & 0xffff) + (int)DAT_00732e58)) {
                          local_a0 = DAT_006d4a4e + 1;
                        }
                        else {
                          local_a0 = 0;
                        }
                        sStack_84 = sStack_84 + DAT_006d4a4e + local_a0 + 2;
                      }
                    }
                    if (pbVar25 != (unsigned char *)0) {
                      FUN_0048f510();
                    }
                  }
                  if (local_a0 == 0) {
                    iVar19 = iVar19 + 1;
                  }
                  else {
                    iVar19 = iVar19 + 2;
                  }
                }
                sStack_98 = sStack_98 + 1;
                local_9a = local_9a + 1;
              } while (sStack_98 < 2);
              sStack_96 = sStack_96 + 1;
            } while (sStack_96 < 2);
          }
LAB_00450e1d:
          local_9a = 0;
          puVar3 = (unsigned short *)(DAT_00525d18 + (iVar27 * 3 + uVar29) * 2);
          uVar8 = *puVar3;
          do {
            sStack_9c = 0;
            if (0 < DAT_006d49a0) {
              uVar13 = (unsigned int)local_9a;
              do {
                uVar26 = (unsigned int)sStack_9c;
                iVar11 = uVar13 * 0x4e + uVar26 * 0x566;
                if (DAT_0070aee9[iVar11 + uVar10] != 0 &&
                    (DAT_00524900[DAT_007092d4] &
                     DAT_005248f8[(unsigned char)DAT_0070ad5d[uVar26 * 0x566]]) != 0 &&
                    (unsigned char)DAT_0070aee5[iVar11] == uVar29 &&
                    (unsigned char)DAT_006d4b36[uVar13 + uVar10 * 8 + uVar26 * 0x126] == uVar8 &&
                    (uVar9 = (unsigned short)(unsigned char)DAT_0070aee6[iVar11], uVar9 != 10) &&
                    uVar9 != 0 &&
                    (uStack_86 != 2 ||
                     ((local_82 != 0 || ((signed char)DAT_0070aee8[iVar11] * 0xfa < 0)) &&
                      (local_82 != 3 || (-1 < (signed char)DAT_0070aee8[iVar11] * 0xfa))))) {
                  if (bVar6) {
                    if ((DAT_005f0918 <= iVar19) && (iVar19 < (short)(uStack_94 >> 16))) {
                      FUN_0048ece0(sStack_8c, sStack_84);
                      FUN_0048f270((unsigned char *)DAT_006cf470[iVar27]);
                      sStack_84 = sStack_84 + sVar23;
                    }
                    bVar6 = 0;
                    iVar19 = iVar19 + 1;
                  }
                  local_a2 = 0;
                  bVar4 = DAT_0070aee7[iVar11];
                  uStack_94 = (uStack_94 & 0xffff0000) | (unsigned short)bVar4;
                  bVar5 = DAT_0070aef3[iVar11];
                  if ((DAT_005f0918 <= iVar19) && (iVar19 < (short)(uStack_94 >> 16))) {
                    pbVar25 = (unsigned char *)0;
                    if (DAT_0070a1e0[(uVar13 + uVar26 * 8) * 3 + (*puVar3 & 3)] != 0) {
                      pbVar25 = FUN_0048f4f0((unsigned int)DAT_0070a1e0[(uVar13 + uVar26 * 8) * 3 + (*puVar3 & 3)]);
                    }
                    FUN_0048ece0(sStack_8c, sStack_84);
                    if ((local_82 == 2) && (uStack_86 == 1)) {
                      uVar26 = 5;
                    }
                    else {
                      uVar26 = (unsigned int)*puVar3;
                    }
                    if (uStack_86 == 2) {
                      uVar21 = (signed char)DAT_0070aee8[iVar11] * 0xfa;
                      _sprintf((char *)auStack_50, s___s__ld__00525dd8,
                               DAT_00656c34[(unsigned int)((int)uVar21 < 0)], uVar21);
                      uVar14 = 0x52;
                      if ((int)uVar21 < 0) {
                        uVar14 = 0x4a;
                      }
                      FUN_0048ed70(uVar14);
                      FUN_0048f140((unsigned char *)auStack_50);
                      FUN_0048ed70((unsigned int)(unsigned char)DAT_00525d48[iVar27]);
                    }
                    if (pbVar25 == (unsigned char *)0) {
                      uVar9 = FUN_004513b0(((unsigned int)(unsigned short)sStack_9c << 16) | uVar9,
                                           uStack_94,
                                           ((unsigned int)local_9a << 16) | (unsigned short)sStack_9c,
                                           uVar9, 1, uVar26, (unsigned short)bVar4,
                                           (unsigned short)bVar5, (unsigned char *)0, (unsigned int)-1,
                                           iVar27);
                      sStack_84 = sStack_84 + uVar9;
                      local_a2 = uVar9;
                      if (uVar9 <= (unsigned short)(DAT_006d4a4e + 2)) {
                        local_a2 = 0;
                      }
                    }
                    else {
                      FUN_0048f140(pbVar25);
                      (*DAT_006d9984)(10);
                      if ((pbVar25 == (unsigned char *)0) ||
                          (uVar28 = FUN_004b4060(pbVar25),
                           (int)(((unsigned int)uVar28 & 0xffff) + (int)DAT_00732e58) <= DAT_0070acc0 + -0xb)) {
                        local_a2 = 0;
                      }
                      else {
                        local_a2 = (unsigned short)((DAT_006d4a4e & 0xff) + 1);
                      }
                      sStack_84 = sStack_84 + (unsigned short)DAT_006d4a4e + (short)local_a2 + 2;
                    }
                    if (pbVar25 != (unsigned char *)0) {
                      FUN_0048f510();
                    }
                  }
                  if ((short)local_a2 == 0) {
                    iVar19 = iVar19 + 1;
                  }
                  else {
                    iVar19 = iVar19 + 2;
                  }
                }
                sStack_9c = sStack_9c + 1;
              } while (sStack_9c < DAT_006d49a0);
            }
            local_9a = local_9a + 1;
          } while (local_9a < 8);
        }
        uStack_86 = uStack_86 + 1;
      } while (uStack_86 < 3);
      local_82 = local_82 + 1;
    } while (local_82 < 4);
    DAT_005f08dc = iVar19;
    sVar24 = 0;
    do {
      sVar18 = sVar24 + 1;
      DAT_005f08e8[sVar24] = DAT_006da3b0[sVar24];
      sVar24 = sVar18;
    } while (sVar18 < 8);
    __builtin_memcpy(&us_tmp, &DAT_0071b73c[DAT_0071ecf4 * 0x5bd], 2);
    uVar21 = (unsigned int)us_tmp;
    DAT_005f0910 = DAT_00709d0a[uVar21 * 3];
    DAT_005f08d8 = DAT_00709d0b[uVar21 * 3];
  }
  if (DAT_0071b743[DAT_0071ecf4 * 0x5bd] == '\0') {
    sStack_88 = (short)(((short)DAT_005f08dc + 1) * sVar23 + DAT_00655aa2);
    if ((int)((unsigned int)DAT_00655a90 + (unsigned int)DAT_00655aa2 + -2) < (int)sStack_88) {
      sStack_88 = (short)((DAT_00655aa2 - 2) + DAT_00655a90);
    }
    if (DAT_0052f0e0 == -1) {
      DAT_0052f0e0 = 1;
    }
  }
  if (DAT_0052f0e0 == 1) {
    uVar21 = 0x46;
  }
  else {
    if (DAT_0052f0e4 != 1) goto LAB_00451379;
    if (DAT_0071b743[DAT_0071ecf4 * 0x5bd] == '\0') {
      uVar21 = (unsigned int)DAT_00525508;
    }
    else {
      uVar21 = 0x34;
    }
  }
  FUN_0048eda0(uVar21);
  FUN_0048ed00(sStack_8c + -2, local_8a + -2, local_8e + 2, sStack_88 + 2);
  /* UNRESOLVED: upper 16 bits of the 3rd argument are register garbage in the decompile;
     widening (movsx vs raw register) at 0x00451360 not confirmed. */
  (*DAT_0071eae0)(sStack_8c + -2, local_8a + -2, local_8e + 2, sStack_88 + 2, 1);
LAB_00451379:
  DAT_005f08e4 = 0;
  FUN_0048ee00(0);
  uVar21 = FUN_00461f20(0, 0x140, 200, 0);
  return uVar21 & 0xffff0000;
}
