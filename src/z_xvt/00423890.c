/* SPDX-License-Identifier: MIT */
/* z_xvt 00423890 */
/* Prototypes for every callee */
void FUN_0040f0f0(unsigned int a);
void FUN_0040f050(unsigned short a);
void FUN_00423d00(int a);
unsigned int FUN_00478320(unsigned int a, unsigned short b);
void FUN_004296b0(unsigned short a);
void FUN_00429310(unsigned int a);
unsigned int FUN_0042af80(short a);
int FUN_0042b150(int a);
int FUN_00481380(char *a);

/* Globals (types inferred from access patterns; unresolved where noted) */
extern int _DAT_00732a3c;
extern int DAT_006d4980;
extern int DAT_006d4988;          /* base of 0x23-byte entry table (address held as int) */
extern int DAT_00732a38;
extern int DAT_00703c88;
extern int DAT_00732e54;
extern short DAT_006d9d62;
extern unsigned char DAT_0070b252[];
extern unsigned char DAT_0071b730[];
extern unsigned char DAT_0071b741[];
extern unsigned char DAT_0071b744[];
extern unsigned char DAT_0071b756[];
extern unsigned char DAT_0071b77b[];
extern unsigned char DAT_0071b77d[];
extern unsigned char DAT_0071b77f[];
extern unsigned char DAT_0071b781[];
extern unsigned char DAT_0071b783[];
extern unsigned char DAT_0071b75d[];
extern unsigned char DAT_0071b74a[];
extern unsigned char DAT_006da3e0[];
extern unsigned char DAT_00524798[];
extern unsigned char DAT_0070ad92[];
extern unsigned char DAT_0070ad93[];
extern unsigned char DAT_0052a650[];
extern unsigned char DAT_0052b08d[];
extern unsigned char DAT_00703ca0[];
extern char DAT_00524948[];
extern char s_stationaryldrpln_00524934[];
extern char s_stationaryflwpln_00524920[];
extern char s_disabledpln_00524914[];
extern char s_escortldr1pln_00524904[];

unsigned int FUN_00423890(int param_1, int param_2, int param_3)
{
  char cVar1;
  unsigned char bVar2;
  int iVar3;
  unsigned short uVar4;
  int iVar5;
  unsigned int uVar7;
  unsigned int uVar8;
  unsigned int uVar9;
  int iVar10;
  unsigned short uVar11;
  int iVar13;
  unsigned char *pcVar15;
  unsigned char *ent;
  char *pcVar14;
  char *pcVar16;
  char *pcS;
  int bVar17;
  int t0;
  int t1;
  int t2;
  unsigned int u32;
  unsigned short u16;
  unsigned int zero32 = 0;
  unsigned short zero16 = 0;

  if (param_2 == 1) {
    uVar11 = 0;
    if (_DAT_00732a3c < DAT_006d4980) {
      pcVar15 = (unsigned char *)(int)((unsigned int)_DAT_00732a3c * 0x23u + 3u + (unsigned int)DAT_006d4988);
      iVar5 = (int)((unsigned int)DAT_006d4980 - (unsigned int)_DAT_00732a3c);
      do {
        if (*pcVar15 != 0) {
          __builtin_memcpy(&t0, DAT_0070b252 + (unsigned int)pcVar15[0x13] * 0x566u, 4);
          if (t0 == param_1) {
            __builtin_memcpy(&t1, pcVar15 + 0x1c, 4);
            __builtin_memcpy(&t2, (unsigned char *)t1 + 0xa7, 4);
            cVar1 = *(char *)(t2 + 7);
            if ((cVar1 != 3) && (cVar1 != 5) && (cVar1 != 4)) {
              uVar11 = uVar11 + 1;
            }
          }
        }
        pcVar15 = pcVar15 + 0x23;
        iVar5 = (int)((unsigned int)iVar5 - 1u);
      } while (iVar5 != 0);
    }
    if (uVar11 < 2) {
      return 0;
    }
  }
  iVar5 = (int)((unsigned int)param_1 * 0x5bdu);
  __builtin_memcpy(&uVar8, DAT_0071b730 + iVar5, 4);
  if (uVar8 == 0xffffffff) {
    return 0;
  }
  iVar3 = (int)(uVar8 * 0x23u);
  ent = (unsigned char *)(int)((unsigned int)DAT_006d4988 + (unsigned int)iVar3);
  u32 = 0xffffffff;
  __builtin_memcpy(ent + 0x1b, &u32, 4);
  __builtin_memcpy(&t0, ent + 0x1f, 4);
  *(unsigned char *)(t0 + 0x90) = 1;
  __builtin_memcpy(&t0, ent + 0x1f, 4);
  *(unsigned char *)(t0 + 0x89) = 1;
  FUN_0040f0f0(uVar8);
  uVar11 = (unsigned short)uVar8;
  FUN_0040f050(uVar11);
  FUN_00423d00(param_1);
  iVar13 = 0;
  __builtin_memcpy(&t0, ent + 0x1f, 4);
  __builtin_memcpy(&iVar10, (unsigned char *)t0 + 0xa7, 4);
  do {
    *(unsigned char *)(iVar10 + 0x216 + iVar13) = 0;
    *(unsigned char *)(iVar10 + 0x218 + iVar13) = 0;
    *(unsigned char *)(iVar10 + 0x21a + iVar13) = 0;
    __builtin_memcpy((unsigned char *)iVar10 + 0x21c + iVar13 * 2, &zero16, 2);
    __builtin_memcpy((unsigned char *)iVar10 + 0x220 + iVar13 * 4, &zero32, 4);
    iVar13 = iVar13 + 1;
  } while (iVar13 < 2);
  iVar13 = 0;
  do {
    *(unsigned char *)(iVar10 + 0x22b + iVar13) = 1;
    iVar13 = iVar13 + 1;
    __builtin_memcpy((unsigned char *)iVar10 + 0x22b + iVar13 * 2, &zero16, 2);
  } while (iVar13 < 2);
  __builtin_memcpy((unsigned char *)iVar10 + 0x231, &zero16, 2);
  {
    unsigned short a;
    unsigned short b;
    __builtin_memcpy(&a, (unsigned char *)iVar10 + 0x20b, 2);
    __builtin_memcpy(&b, (unsigned char *)iVar10 + 0x20d, 2);
    a = (unsigned short)(a + b);
    __builtin_memcpy((unsigned char *)iVar10 + 0x20b, &a, 2);
  }
  __builtin_memcpy((unsigned char *)iVar10 + 0x20d, &zero16, 2);
  *(unsigned char *)(iVar10 + 0x210) = 0;
  u32 = 0xffffffff;
  __builtin_memcpy(DAT_0071b730 + iVar5, &u32, 4);
  DAT_0071b741[iVar5] = 0;
  DAT_0071b744[iVar5] = 0;
  DAT_0071b756[iVar5] = 0;
  __builtin_memcpy(DAT_0071b77b + iVar5, &zero16, 2);
  __builtin_memcpy(DAT_0071b77d + iVar5, &zero16, 2);
  __builtin_memcpy(DAT_0071b77f + iVar5, &zero16, 2);
  __builtin_memcpy(DAT_0071b781 + iVar5, &zero16, 2);
  __builtin_memcpy(DAT_0071b783 + iVar5, &zero16, 2);
  u16 = 0xffff;
  __builtin_memcpy(DAT_0071b75d + iVar5, &u16, 2);
  __builtin_memcpy(&t0, (unsigned char *)iVar10 + 0x432, 4);
  __builtin_memcpy(&t1, (unsigned char *)t0 + 0x1f, 4);
  __builtin_memcpy(&iVar13, (unsigned char *)t1 + 0xab, 4);
  *(unsigned char *)(iVar13 + 4) = 0;
  if (param_3 == 0) {
    t0 = FUN_00481380(DAT_00524948);
    *(char *)(iVar13 + 0xe) = (char)t0;
    *(char *)(iVar13 + 0xf) = (char)t0;
    DAT_00732e54 = iVar10;
    FUN_004296b0(uVar11);
    FUN_00429310(uVar8);
    *(unsigned char *)(iVar10 + 0xa4) = 0;
    *(unsigned char *)(iVar10 + 0x9c) = 0;
    *(unsigned char *)(iVar10 + 0xab) = 0;
    *(unsigned char *)(iVar10 + 0x96) = 0;
    *(unsigned char *)(iVar10 + 0x97) = 0;
    return 1;
  }
  bVar2 = DAT_006da3e0[(unsigned int)DAT_00524798[DAT_0070ad92[(unsigned int)ent[0x16] * 0x566u]]];
  *(unsigned char *)(iVar13 + 0xf) = bVar2;
  *(unsigned char *)(iVar13 + 0xe) = bVar2;
  bVar17 = (bVar2 == 0);
  pcVar15 = DAT_00703ca0 + (unsigned int)bVar2 * 0x55u;
  pcS = (char *)pcVar15;
  iVar13 = 8;
  pcVar14 = (char *)pcVar15;
  pcVar16 = DAT_00524948;
  do {
    if (iVar13 == 0) break;
    iVar13 = iVar13 - 1;
    bVar17 = (*pcVar14 == *pcVar16);
    pcVar14 = pcVar14 + 1;
    pcVar16 = pcVar16 + 1;
  } while (bVar17);
  if (!bVar17) {
    iVar13 = 0x11;
    bVar17 = 0;
    pcVar14 = (char *)pcVar15;
    pcVar16 = s_stationaryldrpln_00524934;
    do {
      if (iVar13 == 0) break;
      iVar13 = iVar13 - 1;
      bVar17 = (*pcVar14 == *pcVar16);
      pcVar14 = pcVar14 + 1;
      pcVar16 = pcVar16 + 1;
    } while (bVar17);
    if (!bVar17) {
      iVar13 = 0x11;
      bVar17 = 0;
      pcVar14 = (char *)pcVar15;
      pcVar16 = s_stationaryflwpln_00524920;
      do {
        if (iVar13 == 0) break;
        iVar13 = iVar13 - 1;
        bVar17 = (*pcVar14 == *pcVar16);
        pcVar14 = pcVar14 + 1;
        pcVar16 = pcVar16 + 1;
      } while (bVar17);
      if (!bVar17) {
        iVar13 = 0xc;
        bVar17 = 0;
        pcVar14 = (char *)pcVar15;
        pcVar16 = s_disabledpln_00524914;
        do {
          if (iVar13 == 0) break;
          iVar13 = iVar13 - 1;
          bVar17 = (*pcVar14 == *pcVar16);
          pcVar14 = pcVar14 + 1;
          pcVar16 = pcVar16 + 1;
        } while (bVar17);
        if (!bVar17) {
          iVar13 = 0xe;
          bVar17 = 0;
          pcVar14 = s_escortldr1pln_00524904;
          do {
            if (iVar13 == 0) break;
            iVar13 = iVar13 - 1;
            bVar17 = (*pcS == *pcVar14);
            pcS = pcS + 1;
            pcVar14 = pcVar14 + 1;
          } while (bVar17);
          if (bVar17) {
            uVar4 = 0x8000;
          }
          else {
            __builtin_memcpy(&uVar4,
                             DAT_0052a650 + (unsigned int)DAT_0070ad93[(unsigned int)ent[0x16] * 0x566u] * 2u,
                             2);
          }
          goto LAB_00423ba7;
        }
      }
    }
  }
  uVar4 = 0;
LAB_00423ba7:
  __builtin_memcpy((unsigned char *)iVar10 + 0xbd, &uVar4, 2);
  __builtin_memcpy(&u16, DAT_0052b08d + (unsigned int)*(unsigned char *)(iVar10 + 4) * 0xefu, 2);
  uVar7 = FUN_00478320((unsigned int)u16, uVar4);
  __builtin_memcpy(&t0, ent + 0x1f, 4);
  u16 = (unsigned short)uVar7;
  __builtin_memcpy((unsigned char *)t0 + 0x79, &u16, 2);
  __builtin_memcpy(&t0, ent + 0x1f, 4);
  __builtin_memcpy((unsigned char *)t0 + 0x7b, &zero16, 2);
  DAT_00732e54 = iVar10;
  FUN_004296b0(uVar11);
  FUN_00429310(uVar8);
  __builtin_memcpy(&u16, DAT_0071b74a + iVar5, 2);
  if (u16 != 0xffff) {
    uVar8 = (unsigned int)u16;
    if (((int)uVar8 < DAT_006d4980) ||
        ((DAT_00732a38 <= (int)uVar8) &&
         ((int)uVar8 < (int)((unsigned int)DAT_00703c88 + (unsigned int)DAT_00732a38)))) {
      bVar17 = 0;
      uVar8 = 0;
      do {
        DAT_006d9d62 = (short)uVar8;
        __builtin_memcpy(&u16, DAT_0071b74a + iVar5, 2);
        uVar9 = FUN_0042af80((short)u16);
        if ((short)uVar9 != 0) {
          bVar17 = 1;
        }
        uVar8 = uVar8 + 1;
      } while (uVar8 < 3);
      if (bVar17) {
        iVar10 = FUN_0042b150(iVar10);
        __builtin_memcpy(&u16, DAT_0071b74a + iVar5, 2);
        __builtin_memcpy((unsigned char *)iVar10 + 0x2b, &u16, 2);
      }
    }
  }
  return 1;
}
