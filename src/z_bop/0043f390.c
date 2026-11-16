/* SPDX-License-Identifier: MIT */
/* z_bop 0043f390 */
extern void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);

void FUN_004a95c0(int);
void FUN_004a9640(char);
void FUN_004a9520(short, short, short, short);
void FUN_004a9500(short, short);
void FUN_004a9630(int);
void FUN_0043c290(unsigned short, unsigned char);
void __cdecl FUN_004a9a50(unsigned char *);
void FUN_004a9590(unsigned int);
void FUN_004a9960(unsigned char *);
void FUN_00485000(int, unsigned int);
unsigned long long __cdecl FUN_004278c0(unsigned char *);
int _sprintf(char *, const char *, ...);
unsigned int FUN_0046b610(char *);
void FUN_004a97f0(char *);
void FUN_004034e0(unsigned int, unsigned int);
void FUN_0046a8d0(int, int, int);
void FUN_004277f0(unsigned short, int, int);

extern char DAT_009ec464;
extern short DAT_00a09354, DAT_00a0910a, DAT_00a09108, DAT_00a0935c, DAT_00a09356;
extern short DAT_00a09110, DAT_00a0935e, DAT_00a0933c, DAT_00a0933e;
extern unsigned char DAT_009a20ae;
extern unsigned short DAT_00a08c74;
extern short DAT_00a0a29e, DAT_00a0a2a0;
extern unsigned short DAT_00a0a2a2, DAT_00a0a2b0, DAT_00a0a2b2, DAT_00a0a2b4, DAT_00a0a2b6;
extern void (*DAT_009d1300)(void);
extern void (*DAT_009a6fe4)(int);
extern int DAT_009ecc34;
extern int DAT_009a1fe0;
extern int DAT_009a1fe8;
extern char DAT_009a2062;
extern int DAT_00a080f8;
extern char DAT_009d6959;
extern unsigned char DAT_009e968a;
extern unsigned char DAT_009e967c;
extern unsigned char DAT_009d8c88[];
extern unsigned char DAT_009d8c94[];
extern char DAT_009fd458[];
extern unsigned char DAT_009a1ed0;
extern unsigned int DAT_009a8c00;
extern short DAT_00a09122, DAT_00a09120, DAT_00a09116, DAT_00a09114, DAT_00a0911c;
extern short DAT_00a09182, DAT_00a09180, DAT_00a09188, DAT_00a08102;
extern short DAT_00a0918c, DAT_00a0918e, DAT_00a09194, DAT_00a09198, DAT_00a0919a;
extern short DAT_00a091a0, DAT_00a091a4, DAT_00a091a6, DAT_00a091ac;
extern unsigned char *DAT_00a0a0e0, *DAT_00a0a0f4, *DAT_00a607ac, *DAT_00a0a0fc, *DAT_00a0a100;
extern char *DAT_00a0a114, *DAT_00a0a118, *DAT_00a0a11c;
extern char *DAT_00a0a104, *DAT_00a0a108, *DAT_00a0a10c, *DAT_00a0a110;
extern int DAT_005233ec;
extern unsigned char DAT_005272f8[];
extern unsigned char *DAT_009a1840[];
extern char DAT_009d1320;
extern char DAT_0051a2bc;
extern char DAT_00521690;
extern char s_00_00_005216b0[];
extern char s__ld_0_ld_005216a4[];
extern char s__ld__ld_0052169c[];
extern char s_disabledpln_0051a368[];
extern char s_flyhomepln_00521648[];
extern char s_followhomepln_00521638[];
extern char s_flyhomeevadepln_00521628[];
extern char s_followhomeevadepln_0051bfc0[];
extern char s_enterhangarpln_0051bfd4[];
extern char s_exithangarpln_0051bff8[];
extern char s_intohyperspacepln_00521614[];
extern char s_outofhyperspacepln_0051bfe4[];
extern char s_starshipintohyperpln_005215fc[];
extern char s_starshipfollowhomepln_005215e4[];

#define RD(dst, addr) __builtin_memcpy(&(dst), (const void *)(unsigned int)(addr), sizeof(dst))
#define SLOT(base) ((unsigned int)&(base) + (unsigned int)DAT_009ecc34 * 0x5bd)

void __stdcall FUN_0043f390(void)
{
  short sVar1;
  short sVar2;
  short sVar3;
  int iVar4;
  unsigned int uVar5;
  unsigned int uVar6;
  int iVar7;
  int iVar8;
  unsigned short uVar9;
  unsigned short uVar10;
  short sVar11;
  unsigned char *pbVar12;
  char *pcVar13;
  char *pcVar14;
  int bVar15;
  unsigned long long uVar16;
  char *pcVar17;
  unsigned char bVar18;
  unsigned int uVar19;
  unsigned short uStack_10 = 0;
  unsigned short uStack_e = 0;
  unsigned short local_8;
  unsigned short t16;
  unsigned short t16b;
  int t32;
  int t32b;
  int t32c;

  DAT_009ec464 = 0;
  FUN_004a95c0(0x2c);
  FUN_004a9640('\x02');
  sVar3 = DAT_00a09354;
  sVar11 = DAT_00a0910a;
  sVar1 = DAT_00a09108;
  uVar10 = DAT_00a08c74;
  sVar2 = (short)(DAT_00a0935c + DAT_00a09354);
  RD(uVar9, SLOT(DAT_009e968a));
  if (uVar9 != DAT_00a08c74) {
    DAT_00a0a29e = -1;
    DAT_00a0a2a0 = -1;
    DAT_00a0a2a2 = 0xffff;
    DAT_00a0a2b0 = 0xffff;
    DAT_00a0a2b2 = 0xfffe;
    DAT_00a0a2b4 = 0xfffe;
    DAT_00a0a2b6 = 0xffff;
    DAT_00a08c74 = uVar9;
    FUN_004a9520(DAT_00a09108, DAT_00a0910a, (short)(DAT_00a09110 + DAT_00a09108),
                 (short)((unsigned short)DAT_009a20ae + DAT_00a0910a + 1));
    (*DAT_009d1300)();
    FUN_004a9500(sVar1, sVar11);
    FUN_004a9630(1);
    RD(uVar9, SLOT(DAT_009e968a));
    if (uVar9 != 0xffff) {
      iVar4 = (int)((unsigned int)uVar9 * 0x23 + (unsigned int)DAT_009a1fe8);
      RD(iVar7, iVar4 + 0x1f);
      if (iVar7 == 0) {
        iVar8 = (int)(((unsigned int)uStack_e << 16) | uStack_10);
      }
      else {
        RD(iVar8, iVar7 + 0xa9);
      }
      if ((DAT_009a2062 == '\x02') && (1 < DAT_00a080f8)) {
        bVar18 = 3;
        if ((iVar7 != 0) && (iVar8 != 0) && (DAT_009d6959 == '\0')) {
          RD(t16, SLOT(DAT_009e967c));
          uVar6 = t16;
          if (*(char *)(uVar6 + 0x1f6 + iVar8) == '\0') {
            iVar7 = (int)((unsigned int)*(unsigned char *)(iVar4 + 0x16) * 0x566);
            if (DAT_009d8c88[iVar7] == uVar6) {
              bVar15 = 0;
            }
            else {
              bVar15 = DAT_009fd458[(unsigned int)DAT_009d8c88[iVar7] + uVar6 * 0x1e5] == '\0';
            }
            if ((bVar15) && (DAT_009d8c94[iVar7] != '\0')) {
              bVar18 = 1;
            }
          }
        }
      }
      else {
        bVar18 = 3;
      }
      FUN_0043c290(uVar9, bVar18);
      FUN_004a9a50(&DAT_009a1ed0);
    }
    local_8 = uVar10;
    if (DAT_009a1fe0 <= (int)(unsigned int)DAT_00a08c74) {
      FUN_004a9520(sVar3, DAT_00a09356, sVar2, (short)(DAT_00a09356 + DAT_00a0935e));
      (*DAT_009d1300)();
    }
  }
  sVar11 = DAT_00a09122;
  sVar1 = DAT_00a09120;
  RD(t16, SLOT(DAT_009e968a));
  if ((short)t16 != -1) {
    if (local_8 == 0xffff) {
      FUN_004a9500(DAT_00a0933c, DAT_00a0933e);
      FUN_004a9590(0x49);
      FUN_004a9960(DAT_00a0a0e0);
    }
    FUN_00485000(DAT_009ecc34, (unsigned int)DAT_00a08c74);
    sVar3 = (short)((unsigned short)DAT_009a20ae + sVar11 + 1);
    uVar16 = FUN_004278c0((unsigned char *)s_00_00_005216b0);
    FUN_004a9520(sVar1, sVar11, (short)((short)uVar16 + sVar1), sVar3);
    FUN_004a9590(0x4a);
    DAT_009a8c00 = DAT_009a8c00 * 0xa1;
    uVar6 = DAT_009a8c00 >> 0x10;
    if (9999 < (unsigned short)(DAT_009a8c00 >> 0x10)) {
      uVar6 = 9999;
    }
    uVar5 = (uVar6 & 0xffff) / 100;
    sVar3 = (short)uVar5;
    uVar6 = (unsigned short)(sVar3 * -100) + uVar6;
    uVar9 = (unsigned short)uVar6;
    if ((sVar3 != DAT_00a0a2a0) || (uVar9 != DAT_00a0a2a2)) {
      DAT_00a0a2a0 = sVar3;
      DAT_00a0a2a2 = uVar9;
      FUN_004a9500(sVar1, sVar11);
      (*DAT_009d1300)();
      if (uVar9 < 10) {
        pcVar17 = s__ld_0_ld_005216a4;
      }
      else {
        pcVar17 = s__ld__ld_0052169c;
      }
      _sprintf((char *)&DAT_009a1ed0, pcVar17, uVar5, uVar6 & 0xffff);
      FUN_004a9960(&DAT_009a1ed0);
    }
    sVar11 = DAT_00a09116;
    sVar1 = DAT_00a09114;
    sVar3 = 2;
    RD(t16, SLOT(DAT_009e968a));
    uVar6 = t16;
    pbVar12 = DAT_00a0a0f4;
    if ((int)uVar6 < DAT_009a1fe0) {
      RD(t32, uVar6 * 0x23 + 0x1f + (unsigned int)DAT_009a1fe8);
      pcVar17 = (char *)t32;
      if (*pcVar17 == '\0') {
        RD(iVar7, pcVar17 + 0xa9);
        RD(t16b, SLOT(DAT_009e967c));
        if (*(char *)((unsigned int)t16b + 0x1f6 + iVar7) == '\0') {
          sVar3 = 0;
          pbVar12 = DAT_00a607ac;
        }
        else {
          sVar3 = 1;
          pbVar12 = (unsigned char *)(iVar7 + 0x201);
          if (*(char *)(iVar7 + 0x201) == '\0') {
            sVar3 = 2;
            pbVar12 = DAT_00a0a0f4;
          }
        }
      }
    }
    if (sVar3 != DAT_00a0a29e) {
      DAT_00a0a29e = sVar3;
      FUN_004a9520(DAT_00a09114, DAT_00a09116, (short)(DAT_00a0911c + DAT_00a09114),
                   (short)((unsigned short)DAT_009a20ae + DAT_00a09116 + 1));
      (*DAT_009d1300)();
      FUN_004a9500(sVar1, sVar11);
      FUN_004a9590(0x46);
      FUN_004a9960(pbVar12);
    }
  }
  RD(uVar9, SLOT(DAT_009e968a));
  uVar6 = (unsigned int)uVar9;
  if (DAT_009a1fe0 <= (int)uVar6) {
    return;
  }
  if (uVar9 == 0xffff) {
    return;
  }
  iVar8 = (int)(uVar6 * 0x23);
  RD(iVar7, (unsigned int)DAT_009a1fe8 + (unsigned int)iVar8 + 0x1f);
  RD(iVar4, iVar7 + 0xa9);
  if ((DAT_009a2062 == '\x02') && (1 < DAT_00a080f8) && (iVar7 != 0) &&
      (iVar4 != 0) && (DAT_009d6959 == '\0')) {
    RD(t16, SLOT(DAT_009e967c));
    uVar5 = t16;
    if (*(char *)(uVar5 + 0x1f6 + iVar4) == '\0') {
      bVar15 = 0;
      if (uVar5 != DAT_009d8c88[(unsigned int)*(unsigned char *)((unsigned int)DAT_009a1fe8 + iVar8 + 0x16) * 0x566]) {
        bVar15 = DAT_009fd458[(unsigned int)DAT_009d8c88[(unsigned int)*(unsigned char *)((unsigned int)DAT_009a1fe8 + iVar8 + 0x16) * 0x566] +
                              uVar5 * 0x1e5] == '\0';
      }
      if (bVar15) {
        return;
      }
    }
  }
  uVar5 = (unsigned int)*(unsigned char *)(iVar4 + 0x34);
  RD(t16, iVar4 + 0x1e5);
  if (t16 == 0) {
    pcVar17 = s_disabledpln_0051a368;
  }
  else {
    RD(t16b, iVar7 + 0x79);
    if (t16b != 0) goto LAB_0043fa40;
    pcVar17 = (char *)((unsigned int)&DAT_009d1320 + uVar5 * 0x55);
    iVar7 = 0xb;
    bVar15 = 1;
    pcVar13 = pcVar17;
    pcVar14 = s_flyhomepln_00521648;
    do {
      if (iVar7 == 0) break;
      iVar7 = iVar7 + -1;
      bVar15 = *pcVar13 == *pcVar14;
      pcVar13 = pcVar13 + 1;
      pcVar14 = pcVar14 + 1;
    } while (bVar15);
    if (bVar15) goto L_match;
    iVar7 = 0xe;
    pcVar13 = pcVar17;
    pcVar14 = s_followhomepln_00521638;
    do {
      if (iVar7 == 0) break;
      iVar7 = iVar7 + -1;
      bVar15 = *pcVar13 == *pcVar14;
      pcVar13 = pcVar13 + 1;
      pcVar14 = pcVar14 + 1;
    } while (bVar15);
    if (bVar15) goto L_match;
    iVar7 = 0x10;
    pcVar13 = pcVar17;
    pcVar14 = s_flyhomeevadepln_00521628;
    do {
      if (iVar7 == 0) break;
      iVar7 = iVar7 + -1;
      bVar15 = *pcVar13 == *pcVar14;
      pcVar13 = pcVar13 + 1;
      pcVar14 = pcVar14 + 1;
    } while (bVar15);
    if (bVar15) goto L_match;
    iVar7 = 0x13;
    pcVar13 = pcVar17;
    pcVar14 = s_followhomeevadepln_0051bfc0;
    do {
      if (iVar7 == 0) break;
      iVar7 = iVar7 + -1;
      bVar15 = *pcVar13 == *pcVar14;
      pcVar13 = pcVar13 + 1;
      pcVar14 = pcVar14 + 1;
    } while (bVar15);
    if (bVar15) goto L_match;
    iVar7 = 0xf;
    pcVar13 = pcVar17;
    pcVar14 = s_enterhangarpln_0051bfd4;
    do {
      if (iVar7 == 0) break;
      iVar7 = iVar7 + -1;
      bVar15 = *pcVar13 == *pcVar14;
      pcVar13 = pcVar13 + 1;
      pcVar14 = pcVar14 + 1;
    } while (bVar15);
    if (bVar15) goto L_match;
    iVar7 = 0xe;
    pcVar13 = pcVar17;
    pcVar14 = s_exithangarpln_0051bff8;
    do {
      if (iVar7 == 0) break;
      iVar7 = iVar7 + -1;
      bVar15 = *pcVar13 == *pcVar14;
      pcVar13 = pcVar13 + 1;
      pcVar14 = pcVar14 + 1;
    } while (bVar15);
    if (bVar15) goto L_match;
    iVar7 = 0x12;
    pcVar13 = pcVar17;
    pcVar14 = s_intohyperspacepln_00521614;
    do {
      if (iVar7 == 0) break;
      iVar7 = iVar7 + -1;
      bVar15 = *pcVar13 == *pcVar14;
      pcVar13 = pcVar13 + 1;
      pcVar14 = pcVar14 + 1;
    } while (bVar15);
    if (bVar15) goto L_match;
    iVar7 = 0x13;
    pcVar13 = pcVar17;
    pcVar14 = s_outofhyperspacepln_0051bfe4;
    do {
      if (iVar7 == 0) break;
      iVar7 = iVar7 + -1;
      bVar15 = *pcVar13 == *pcVar14;
      pcVar13 = pcVar13 + 1;
      pcVar14 = pcVar14 + 1;
    } while (bVar15);
    if (bVar15) goto L_match;
    iVar7 = 0x15;
    pcVar13 = pcVar17;
    pcVar14 = s_starshipintohyperpln_005215fc;
    do {
      if (iVar7 == 0) break;
      iVar7 = iVar7 + -1;
      bVar15 = *pcVar13 == *pcVar14;
      pcVar13 = pcVar13 + 1;
      pcVar14 = pcVar14 + 1;
    } while (bVar15);
    if (bVar15) goto L_match;
    iVar7 = 0x16;
    pcVar13 = s_starshipfollowhomepln_005215e4;
    do {
      if (iVar7 == 0) break;
      iVar7 = iVar7 + -1;
      bVar15 = *pcVar17 == *pcVar13;
      pcVar17 = pcVar17 + 1;
      pcVar13 = pcVar13 + 1;
    } while (bVar15);
    if (!bVar15) goto LAB_0043fa40;
  L_match:
    pcVar17 = &DAT_0051a2bc;
  }
  uVar5 = FUN_0046b610(pcVar17);
LAB_0043fa40:
  sVar11 = DAT_00a09182;
  sVar1 = DAT_00a09180;
  if (DAT_00a0a2b0 != uVar5) {
    DAT_00a0a2b0 = (unsigned short)uVar5;
    FUN_004a9520(DAT_00a09180, DAT_00a09182, sVar2,
                 (short)((unsigned short)DAT_009a20ae + DAT_00a09182 + 1));
    (*DAT_009d1300)();
    FUN_004a9500(sVar1, sVar11);
    if (DAT_005233ec == 0x13) {
      uVar19 = 0x45;
    }
    else {
      uVar19 = 0x46;
    }
    FUN_004a9590(uVar19);
    FUN_004a9960(DAT_00a0a0fc);
    if (DAT_00a09188 == 0) {
      uVar16 = FUN_004278c0(DAT_00a0a0fc);
      DAT_00a09188 = (short)uVar16;
    }
    FUN_004a9500((short)(DAT_00a09188 + sVar1), DAT_00a08102);
    FUN_004a9590(0x4e);
    FUN_004a9960(DAT_009a1840[DAT_005272f8[uVar5]]);
    sVar11 = DAT_00a091a6;
    sVar1 = DAT_00a091a4;
    FUN_004a9520(DAT_00a091a4, DAT_00a091a6, sVar2,
                 (short)((unsigned short)DAT_009a20ae + DAT_00a091a6 + 1));
    (*DAT_009d1300)();
    FUN_004a9500(sVar1, sVar11);
    if (DAT_005233ec == 0x13) {
      uVar19 = 0x45;
    }
    else {
      uVar19 = 0x46;
    }
    FUN_004a9590(uVar19);
    pcVar17 = DAT_00a0a114;
    RD(t16, SLOT(DAT_009e968a));
    RD(t32, (unsigned int)t16 * 0x23 + 0x1f + (unsigned int)DAT_009a1fe8);
    RD(t16b, t32 + 0x79);
    if (t16b != 0) {
      pcVar17 = DAT_00a0a11c;
      RD(t16, iVar4 + 0x42);
      if (t16 < 0x8000) {
        pcVar17 = DAT_00a0a118;
      }
    }
    FUN_004a97f0(pcVar17);
    uVar16 = FUN_004278c0(&DAT_009a1ed0);
    DAT_00a091ac = (short)uVar16;
    FUN_004a9960(&DAT_009a1ed0);
  }
  RD(iVar7, (unsigned int)DAT_009a1fe8 + 0x1b + (unsigned int)iVar8);
  if (iVar7 == -1) {
    RD(uVar9, iVar4 + 0x42);
  }
  else {
    RD(uVar9, (unsigned int)&DAT_009e968a + (unsigned int)iVar7 * 0x5bd);
  }
  RD(t16, iVar4 + 0x1e5);
  bVar15 = t16 == 0;
  if (bVar15) {
    uVar9 = 0xffff;
  }
  iVar7 = 8;
  pcVar17 = (char *)((unsigned int)&DAT_009d1320 + uVar5 * 0x55);
  pcVar13 = &DAT_0051a2bc;
  do {
    if (iVar7 == 0) break;
    iVar7 = iVar7 + -1;
    bVar15 = *pcVar17 == *pcVar13;
    pcVar17 = pcVar17 + 1;
    pcVar13 = pcVar13 + 1;
  } while (bVar15);
  if (bVar15) {
    uVar9 = 0xffff;
  }
  if (uVar9 != DAT_00a0a2b2) {
    if (DAT_005233ec == 0x13) {
      uVar5 = 0x45;
    }
    else {
      uVar5 = 0x46;
    }
    DAT_00a0a2b2 = uVar9;
    FUN_004a9590(uVar5);
    FUN_004a9520(DAT_00a0918c, DAT_00a0918e, sVar2,
                 (short)((unsigned short)DAT_009a20ae + DAT_00a0918e));
    (*DAT_009d1300)();
    FUN_004a9500(DAT_00a0918c, DAT_00a0918e);
    pcVar17 = DAT_00a0a108;
    if (uVar9 < 0x8000) {
      pcVar17 = DAT_00a0a104;
    }
    FUN_004a97f0(pcVar17);
    FUN_004a9960(&DAT_009a1ed0);
    uVar16 = FUN_004278c0(&DAT_009a1ed0);
    DAT_00a09194 = (short)uVar16;
    FUN_004a9520(DAT_00a09198, DAT_00a0919a, sVar2,
                 (short)((unsigned short)DAT_009a20ae + DAT_00a0919a));
    (*DAT_009d1300)();
    FUN_004a9500(DAT_00a09198, DAT_00a0919a);
    pcVar17 = DAT_00a0a110;
    if (uVar9 < 0x8000) {
      pcVar17 = DAT_00a0a10c;
    }
    FUN_004a97f0(pcVar17);
    FUN_004a9960(&DAT_009a1ed0);
    uVar16 = FUN_004278c0(&DAT_009a1ed0);
    sVar1 = DAT_00a0918e;
    sVar11 = (short)(DAT_00a09194 + DAT_00a0918c);
    DAT_00a091a0 = (short)uVar16;
    FUN_004a9520(sVar11, DAT_00a0918e, sVar2,
                 (short)((unsigned short)DAT_009a20ae + DAT_00a0918e + 1));
    (*DAT_009d1300)();
    FUN_004a9500(sVar11, sVar1);
    pbVar12 = DAT_00a0a100;
    if ((uVar9 != 0xff) && (uVar9 != 0xffff)) {
      FUN_0043c290(uVar9, 3);
      pbVar12 = &DAT_009a1ed0;
    }
    FUN_004a9960(pbVar12);
  }
  uStack_10 = 0;
  uStack_e = 0;
  if (uVar9 != 0xffff) {
    iVar7 = (int)((unsigned int)iVar8 + (unsigned int)DAT_009a1fe8);
    RD(t32, iVar7 + 0x1b);
    if (t32 == -1) {
      int a0, a1, a2, b0, b1, b2;
      RD(a0, iVar4 + 0x49);
      RD(b0, iVar7 + 4);
      RD(a1, iVar4 + 0x4d);
      RD(b1, iVar7 + 8);
      RD(a2, iVar4 + 0x51);
      RD(b2, iVar7 + 0xc);
      FUN_0046a8d0((int)((unsigned int)a0 - (unsigned int)b0),
                   (int)((unsigned int)a1 - (unsigned int)b1),
                   (int)((unsigned int)a2 - (unsigned int)b2));
    }
    else {
      FUN_004034e0(uVar6, (unsigned int)uVar9);
    }
    sVar1 = DAT_00a0919a;
    sVar11 = (short)(DAT_00a09198 + DAT_00a091a0);
    uStack_10 = (unsigned short)DAT_009a8c00;
    uStack_e = (unsigned short)(DAT_009a8c00 >> 0x10);
    FUN_004a9520(sVar11, DAT_00a0919a, sVar2,
                 (short)((unsigned short)DAT_009a20ae + DAT_00a0919a + 1));
    FUN_004a9590(0x4a);
    DAT_009a8c00 = DAT_009a8c00 * 0xa1;
    uVar6 = DAT_009a8c00 >> 0x10;
    if (9999 < (unsigned short)(DAT_009a8c00 >> 0x10)) {
      uVar6 = 9999;
    }
    uVar9 = (unsigned short)((uVar6 & 0xffff) / 100);
    uVar10 = (unsigned short)(uVar9 * -100 + (short)uVar6);
    if (uVar10 != DAT_00a0a2b4) {
      DAT_00a0a2b4 = uVar10;
      FUN_004a9500(sVar11, sVar1);
      FUN_004277f0(uVar9, 2, 1);
      (*DAT_009a6fe4)(0x2e);
      FUN_004277f0(uVar10, 2, 2);
    }
  }
  sVar1 = DAT_00a091a6;
  sVar11 = (short)(DAT_00a091ac + DAT_00a091a4);
  FUN_004a9520(sVar11, DAT_00a091a6, sVar2,
               (short)((unsigned short)DAT_009a20ae + DAT_00a091a6 + 1));
  FUN_004a9590(0x52);
  FUN_004a9500(sVar11, sVar1);
  RD(t32, (unsigned int)DAT_009a1fe8 + 0x1f + (unsigned int)iVar8);
  RD(sVar1, t32 + 0x79);
  if (sVar1 == 0) {
    iVar7 = 10;
    bVar15 = 1;
    pcVar17 = (char *)((unsigned int)&DAT_009d1320 + (unsigned int)*(unsigned char *)(iVar4 + 0x34) * 0x55);
    pcVar13 = &DAT_00521690;
    do {
      if (iVar7 == 0) break;
      iVar7 = iVar7 + -1;
      bVar15 = *pcVar17 == *pcVar13;
      pcVar17 = pcVar17 + 1;
      pcVar13 = pcVar13 + 1;
    } while (bVar15);
    if (!bVar15) {
      iVar7 = 8;
      pcVar17 = (char *)((unsigned int)&DAT_009d1320 + (unsigned int)*(unsigned char *)(iVar4 + 0x34) * 0x55);
      pcVar13 = &DAT_0051a2bc;
      do {
        if (iVar7 == 0) break;
        iVar7 = iVar7 + -1;
        bVar15 = *pcVar17 == *pcVar13;
        pcVar17 = pcVar17 + 1;
        pcVar13 = pcVar13 + 1;
      } while (bVar15);
      if (!bVar15) {
        if ((int)(((unsigned int)uStack_e << 16) | uStack_10) == 0) {
          DAT_00a0a2b6 = 0;
          (*DAT_009d1300)();
          FUN_004277f0(0, 2, 1);
          (*DAT_009a6fe4)(0x3a);
          FUN_004277f0(0, 2, 2);
          return;
        }
        FUN_004a9960(DAT_00a607ac);
        return;
      }
    }
    RD(t32b, iVar4 + 0x60);
    uVar6 = (unsigned int)(t32b / 0xec);
    uVar9 = (unsigned short)((uVar6 & 0xffff) / 0x3c);
    uVar10 = (unsigned short)((short)uVar6 + uVar9 * -0x3c);
    if (uVar10 != DAT_00a0a2b6) {
      DAT_00a0a2b6 = uVar10;
      (*DAT_009d1300)();
      FUN_004277f0(uVar9, 2, 1);
      (*DAT_009a6fe4)(0x3a);
      FUN_004277f0(uVar10, 2, 2);
      return;
    }
  }
  else {
    t32c = (int)(unsigned short)(sVar1 * 0x12);
    if (t32c == 0) {
      RaiseException(0xC0000094UL, 0, 0, NULL);
    }
    uVar9 = (unsigned short)((((unsigned int)uStack_e << 16) | uStack_10) / (unsigned int)t32c);
    uVar10 = uVar9 % 0x3c;
    if (uVar10 != DAT_00a0a2b6) {
      DAT_00a0a2b6 = uVar10;
      (*DAT_009d1300)();
      FUN_004277f0(uVar9 / 0x3c, 2, 1);
      (*DAT_009a6fe4)(0x3a);
      FUN_004277f0(uVar10, 2, 2);
    }
  }
  return;
}
