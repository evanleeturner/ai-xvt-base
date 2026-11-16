/* SPDX-License-Identifier: MIT */
/* z_xvt 00478830 */
/* prototypes */
unsigned int FUN_004a7aa0(unsigned int);
int __cdecl _sprintf(char *, const char *, ...);

/* globals (types inferred from access width/stride) */
extern short DAT_006d49a0;
extern char DAT_006d4a02;
extern unsigned char DAT_006d4a60, DAT_006d4a61;
extern unsigned char DAT_0070ad42, DAT_0070ad43, DAT_0070ad48, DAT_0070ad49, DAT_0070ad51;
extern int DAT_007092e8, DAT_00709310;
extern unsigned short DAT_00709338;
extern unsigned int DAT_00709f08;
extern char DAT_007092d4, DAT_007092da, DAT_007092db;
extern char DAT_00709d0a, DAT_00709d0b, DAT_00709d0d, DAT_00709d0e, DAT_00709d22, DAT_00709d28;
extern unsigned char DAT_005247e8, DAT_0052b07e, DAT_0052803c;
extern int DAT_00527fc4, DAT_00527fa8, DAT_0052807c, DAT_0052805c;
extern char s_Promo_points___d_Worse_Promo_poi_0052813c[];
extern char s_Player_s_team_score___d_Place____005280fc[];
extern int DAT_0071ecf4;
extern unsigned char DAT_0071bcb8, DAT_0071eaa0;
extern unsigned char DAT_0071b73c, DAT_0071b79d, DAT_0071b7a1, DAT_0071b7a5, DAT_0071b7b5, DAT_0071b7b7,
  DAT_0071b7b9, DAT_0071b7bb, DAT_0071b7bd, DAT_0071b7bf, DAT_0071b7c3, DAT_0071b7c5, DAT_0071b8e5,
  DAT_0071b97b, DAT_0071b99f, DAT_0071b9bf, DAT_0071b9c1, DAT_0071b9c3, DAT_0071b9c5, DAT_0071b9c7;
/* objects of unknown extent, written through memcpy (incomplete arrays: no fixed size) */
extern unsigned char DAT_00732800[];
extern unsigned char DAT_0081bcfe[], DAT_0081bfbe[], DAT_0081bfc2[], DAT_0081bfc6[], DAT_0081bfca[],
  DAT_0081bfce[];
extern unsigned char DAT_0081c012[], DAT_0081c01a[], DAT_0081c01e[], DAT_0081c022[], DAT_0081c026[];
extern unsigned char DAT_0081c066[], DAT_0081c486[], DAT_0081c8a6[];
extern unsigned char DAT_0081d326[], DAT_0081d32a[], DAT_0081d32e[], DAT_0081d332[], DAT_0081d3f6[],
  DAT_0081d3fa[], DAT_0081d402[], DAT_0081d40a[], DAT_0081d40e[];
extern unsigned char DAT_0081e00e[], DAT_0081e012[], DAT_0081e016[], DAT_0081e01a[], DAT_0081e01e[],
  DAT_0081e0a2[], DAT_0081e0a6[], DAT_0081e0c6[], DAT_0081e0e6[], DAT_0081e266[], DAT_0081e286[],
  DAT_0081e4e6[], DAT_0081e50a[], DAT_0081e516[], DAT_0081e522[], DAT_0081e942[], DAT_0081ed62[];
extern unsigned char DAT_0081f182[], DAT_0081f1e6[], DAT_0081f506[], DAT_0081f51e[], DAT_0081f5de[],
  DAT_0081f5ea[], DAT_0081f5f6[], DAT_0081f602[], DAT_0081f60e[], DAT_0081f61a[], DAT_0081f626[],
  DAT_0081f632[], DAT_0081f63e[], DAT_0081f7de[], DAT_0081f7e6[], DAT_0081f7ea[], DAT_0081f7ee[],
  DAT_0081f7f6[], DAT_0081f7fa[], DAT_0081fa7e[], DAT_0081fa9a[], DAT_0081fa9e[];
extern unsigned char DAT_0081fb96[], DAT_0081fb9a[], DAT_0081fbde[], DAT_0081fbf6[], DAT_0081fc0e[],
  DAT_0081fc26[], DAT_0081fc3e[], DAT_0081fc42[], DAT_0081fc46[], DAT_0081fc4a[], DAT_0081fc4e[],
  DAT_0081fc62[], DAT_0081fc66[], DAT_0081fc72[], DAT_0081fc7e[], DAT_0081fc8a[], DAT_0081fc96[],
  DAT_0081fca2[];
extern unsigned char DAT_008200c2[], DAT_008204e2[], DAT_00820902[], DAT_00820a2e[], DAT_00820b5a[],
  DAT_00820c86[], DAT_00820cce[], DAT_00820d16[], DAT_00820d5e[], DAT_00820d6a[], DAT_00820d76[],
  DAT_00820d82[], DAT_00820d8e[], DAT_00820d9a[], DAT_00820da6[], DAT_00820db2[], DAT_00820dbe[],
  DAT_00820dca[], DAT_00820ef6[];
extern unsigned char DAT_00820f42[], DAT_00820f46[], DAT_00820f4a[], DAT_00820f4e[], DAT_00820f52[],
  DAT_00820f5a[];
extern unsigned char DAT_00821d52[], DAT_00821d56[], DAT_00821d5a[], DAT_00821d5e[], DAT_00821d62[],
  DAT_00821d66[], DAT_00821d6a[], DAT_00821d6e[];
extern unsigned char DAT_0082407a[], DAT_0082407e[], DAT_00824082[], DAT_00824086[], DAT_0082408a[],
  DAT_00824092[];
extern unsigned char DAT_008263a2[], DAT_008263b2[], DAT_008263b6[], DAT_008263ba[], DAT_008263be[],
  DAT_008263c6[];
extern unsigned char DAT_00827662[], DAT_00827666[], DAT_0082766a[], DAT_0082766e[], DAT_00827672[],
  DAT_00827676[], DAT_0082767a[], DAT_0082767e[], DAT_00827682[], DAT_00827686[], DAT_0082768a[];
extern unsigned char DAT_0082a542[], DAT_0082a552[], DAT_0082a556[], DAT_0082a55a[], DAT_0082a55e[],
  DAT_0082a566[];
extern unsigned char DAT_0082d422[], DAT_0082d426[], DAT_0082d42a[], DAT_0082d42e[], DAT_0082d432[],
  DAT_0082d436[], DAT_0082d43a[], DAT_0082d43e[], DAT_0082d442[];
extern unsigned char DAT_0082d80a[], DAT_0082d80e[], DAT_0082d812[], DAT_0082d816[], DAT_0082d81a[],
  DAT_0082d81e[], DAT_0082d822[], DAT_0082d82a[], DAT_0082d82e[];
extern unsigned char DAT_0082dc56[], DAT_0082dc5a[], DAT_0082dc5e[], DAT_0082dc62[], DAT_0082dc66[],
  DAT_0082dc6e[], DAT_0082dc72[];
extern unsigned char DAT_0082dfda[], DAT_0082dfde[], DAT_0082dfe2[], DAT_0082dfe6[], DAT_0082dfea[],
  DAT_0082dff6[], DAT_0082dffa[];

/* unaligned-safe access: every load/store goes through memcpy */
#define LI(a) ({ int xi_; __builtin_memcpy(&xi_, (const void *)(a), 4); xi_; })
#define LU(a) ({ unsigned int xu_; __builtin_memcpy(&xu_, (const void *)(a), 4); xu_; })
#define LH(a) ({ unsigned short xh_; __builtin_memcpy(&xh_, (const void *)(a), 2); xh_; })
#define LS(a) ({ short xs_; __builtin_memcpy(&xs_, (const void *)(a), 2); xs_; })
#define ST4(a, v) do { unsigned int xst_ = (unsigned int)(v); \
    __builtin_memcpy((void *)(a), &xst_, 4); } while (0)
#define AD(a, v) do { unsigned int xad_ = (unsigned int)(v); unsigned int xat_; \
    __builtin_memcpy(&xat_, (const void *)(a), 4); xat_ += xad_; \
    __builtin_memcpy((void *)(a), &xat_, 4); } while (0)
#define RI(sym) LI(&(sym))
#define WI(sym, v) ST4(&(sym), (v))
#define SL ((unsigned int)RI(DAT_0081fb96))
#define GA(sym, off) ((char *)&(sym) + (off))
#define SA(sym, off) GA(sym, SL * 0xe824u + (unsigned int)(off))
#define TA(sym, idx) GA(sym, (SL * 0x3a09u + (unsigned int)(idx)) * 4u)
#define A4(sym, idx) GA(sym, (unsigned int)(idx) * 4u)
#define PL(sym) GA(sym, (unsigned int)RI(DAT_0071ecf4) * 0x5bdu)
#define AX(base, idx, mul) ((char *)(unsigned int)((unsigned int)(idx) * (mul) + (base)))

unsigned int __stdcall FUN_00478830(void)
{
  unsigned char bVar1;
  unsigned char bVar2;
  char cVar3;
  char cVar4;
  unsigned short uVar5;
  unsigned short uVar6;
  int bVar7;
  char *piVar8;
  unsigned char *pbVar9;
  unsigned int uVar10;
  int iVar11;
  char *puVar12;
  int iVar13;
  int iVar14;
  unsigned int uVar15;
  int iVar16;
  unsigned int uVar17;
  char *pcVar18;
  char *piVar19;
  char *puVar20;
  int iVar21;
  char *piVar22;
  char *puVar23;
  unsigned int uVar24;
  unsigned int uVar25;
  unsigned int local_30;
  int local_2c;
  int local_28;
  unsigned int local_20;
  unsigned int local_20_bytes[3];
  int local_14;
  int local_10;
  char *local_c;
  unsigned int local_8;
  unsigned int local_4;

  piVar8 = (char *)&DAT_0071bcb8;
  local_14 = 0;
  do {
    if (LI(piVar8) != 0) {
      local_14 = local_14 + 1;
    }
    piVar8 = piVar8 + 0x5bd;
  } while (piVar8 < (char *)&DAT_0071eaa0);
  iVar13 = (int)DAT_006d49a0;
  local_20 = 0;
  local_20_bytes[0] = 0;
  local_20_bytes[1] = 0;
  local_20_bytes[2] = 0;
  if (iVar13 != 0) {
    pbVar9 = &DAT_0070ad48;
    pcVar18 = (char *)&DAT_006d4a60;
    iVar21 = iVar13;
    do {
      if ((pbVar9[0xc] != 0) && (*pcVar18 != '\0')) {
        ((unsigned char *)local_20_bytes)[(unsigned int)*pbVar9] =
             (unsigned char)(((unsigned char *)local_20_bytes)[(unsigned int)*pbVar9] + 1);
      }
      pbVar9 = pbVar9 + 0x566;
      pcVar18 = pcVar18 + 0x126;
      iVar21 = iVar21 - 1;
    } while (iVar21 != 0);
  }
  uVar10 = 0;
  local_4 = 0;
  do {
    if (((unsigned char *)local_20_bytes)[uVar10] != 0) {
      local_4 = local_4 + 1;
    }
    uVar10 = uVar10 + 1;
  } while (uVar10 < 10);
  piVar8 = (char *)(((uVar10 >> 8) << 8) | (unsigned int)(unsigned char)DAT_006d4a02);
  if (DAT_006d4a02 == '\x01') goto LAB_0047be3a;
  if ((DAT_006d4a02 == '\0') || (DAT_006d4a02 == '\x03')) {
    local_2c = 0;
  }
  else if (DAT_006d4a02 == '\x02') {
    local_2c = 1;
  }
  else {
    iVar21 = 0;
    iVar11 = 0;
    local_2c = 2;
    if (iVar13 != 0) {
      pcVar18 = (char *)&DAT_0070ad48;
      do {
        if ((pcVar18[0xc] != '\0') && (LI(pcVar18 + 0x50a) != -1)) {
          if (*pcVar18 == '\0') {
            iVar11 = iVar11 + 1;
          }
          else if (*pcVar18 == '\x01') {
            iVar21 = iVar21 + 1;
          }
        }
        pcVar18 = pcVar18 + 0x566;
        iVar13 = iVar13 + -1;
      } while (iVar13 != 0);
    }
    if (RI(DAT_0081c012) == 1) {
      if (iVar21 == 0) {
        if ((DAT_00709d0a != '\x01') && (DAT_00709d0d == '\0')) {
          DAT_00709d0d = '\x01';
        }
      }
      else if (((iVar11 == 0) && (DAT_00709d0d != '\x01')) && (DAT_00709d0a == '\0')) {
        DAT_00709d0a = '\x01';
      }
    }
    else if (local_14 == 1) {
      if (iVar21 == 0) {
        if (DAT_00709d0a != '\x01') {
          DAT_00709d0b = 1;
          DAT_00709d0d = '\x01';
        }
      }
      else if ((iVar11 == 0) && (DAT_00709d0d != '\x01')) {
        DAT_00709d0e = 1;
        DAT_00709d0a = '\x01';
      }
    }
  }
  piVar8 = (char *)&DAT_0071bcb8;
  do {
    if (LI(piVar8) != 0) {
      piVar19 = (char *)&DAT_0071b79d;
      piVar22 = (char *)&DAT_0081f7de;
      do {
        if (((LI(piVar22) == LI(piVar8)) &&
            ((&DAT_00709d0a)[(unsigned int)LH(piVar19 - 0x61) * 3] == '\x01')) &&
           ((local_2c != 1 && (DAT_007092db == '\x01')))) {
          uVar10 = (unsigned int)LH(piVar19 - 0x5f);
          if ((&DAT_0070ad51)[uVar10 * 0x566] != 'c') {
            bVar1 = (&DAT_0070ad43)[uVar10 * 0x566];
            bVar2 = (&DAT_006d4a61)[uVar10 * 0x126];
            uVar10 = FUN_004a7aa0((unsigned int)(unsigned char)(&DAT_005247e8)[(unsigned char)(&DAT_0070ad42)[uVar10 * 0x566]]);
            AD(piVar19, (unsigned int)(&DAT_0052b07e)[(uVar10 & 0xffff) * 0xef] *
                        (unsigned int)bVar2 * (unsigned int)bVar1 * 0x50u);
          }
        }
        piVar19 = piVar19 + 0x5bd;
        piVar22 = piVar22 + 0x58;
      } while (piVar22 < (char *)&DAT_0081fa9e);
    }
    piVar8 = piVar8 + 0x5bd;
  } while (piVar8 < (char *)&DAT_0071eaa0);
  puVar23 = (char *)&DAT_0081e4e6;
  for (iVar13 = 0x4b6; iVar13 != 0; iVar13 = iVar13 + -1) {
    ST4(puVar23, 0);
    puVar23 = puVar23 + 4;
  }
  iVar13 = (int)((unsigned int)RI(DAT_0071ecf4) * 0x5bdu);
  AD(&DAT_0081e01a, 1);
  iVar21 = (int)((unsigned int)(&DAT_007092e8)[LH(GA(DAT_0071b73c, iVar13))] +
                 (unsigned int)LI(GA(DAT_0071b79d, iVar13)));
  AD(SA(DAT_0081fb9a, 0), 1);
  AD(TA(DAT_0081fc66, local_2c), iVar21);
  piVar8 = AX(0x81c02au, local_2c, 4u);
  AD(piVar8, iVar21);
  if (RI(DAT_0081c012) == 0) {
    AD(TA(DAT_0081fc72, local_2c), 1);
    piVar8 = AX(0x81c036u, local_2c, 4u);
    AD(piVar8, 1);
  }
  else {
    AD(TA(DAT_0081fc7e, local_2c), 1);
    piVar8 = AX(0x81c042u, local_2c, 4u);
    AD(piVar8, 1);
  }
  if ((int)DAT_006d49a0 != 0) {
    pbVar9 = &DAT_0070ad42;
    puVar20 = GA(DAT_0071b7c5, iVar13);
    local_20 = (unsigned int)(int)DAT_006d49a0;
    do {
      uVar10 = (unsigned int)(unsigned char)(&DAT_005247e8)[*pbVar9];
      if (uVar10 < 0x58) {
        uVar15 = (unsigned int)LH(puVar20);
        AD(TA(DAT_0081fc8a, local_2c), uVar15);
        AD(AX(0x81c04eu, local_2c, 4u), uVar15);
        AD(&DAT_0081e50a, uVar15);
        iVar14 = (int)((unsigned int)local_2c * 0x58u);
        AD(TA(DAT_0081fca2, uVar10 + (unsigned int)iVar14), uVar15);
        iVar11 = (int)(((unsigned int)iVar14 + uVar10) * 4u);
        AD(GA(DAT_0081c066, iVar11), uVar15);
        AD(A4(DAT_0081e522, uVar10), uVar15);
        uVar15 = (unsigned int)LH(puVar20 + 0x60);
        AD(TA(DAT_008200c2, uVar10 + (unsigned int)iVar14), uVar15);
        AD(GA(DAT_0081c486, iVar11), uVar15);
        AD(A4(DAT_0081e942, uVar10), uVar15);
        uVar15 = (unsigned int)LH(puVar20 + 0xc0);
        AD(TA(DAT_008204e2, uVar10 + (unsigned int)iVar14), uVar15);
        AD(GA(DAT_0081c8a6, iVar11), uVar15);
        AD(A4(DAT_0081ed62, uVar10), uVar15);
      }
      puVar20 = puVar20 + 2;
      pbVar9 = pbVar9 + 0x566;
      local_20 = local_20 - 1;
    } while (local_20 != 0);
  }
  uVar10 = (unsigned int)LH(GA(DAT_0071b9bf, iVar13));
  AD(TA(DAT_0081fc96, local_2c), uVar10);
  iVar14 = 0;
  piVar8 = AX(0x81c05au, local_2c, 4u);
  AD(piVar8, uVar10);
  AD(&DAT_0081e516, uVar10);
  iVar11 = (int)((unsigned int)local_2c * 0x19u);
  piVar8 = AX(0x81ccc6u, local_2c, 100u);
  puVar20 = GA(DAT_0071b8e5, iVar13);
  piVar19 = (char *)&DAT_0081f182;
  do {
    uVar10 = (unsigned int)LH(puVar20);
    AD(TA(DAT_00820902, (unsigned int)iVar14 + (unsigned int)iVar11), uVar10);
    AD(piVar8, uVar10);
    AD(piVar19, uVar10);
    uVar10 = (unsigned int)LH(puVar20 + 0x32);
    AD(TA(DAT_00820a2e, (unsigned int)iVar14 + (unsigned int)iVar11), uVar10);
    AD(piVar8 + 0x12c, uVar10);
    AD(piVar19 + 0x12c, uVar10);
    uVar10 = (unsigned int)LH(puVar20 + 0x64);
    AD(TA(DAT_00820b5a, (unsigned int)iVar14 + (unsigned int)iVar11), uVar10);
    AD(piVar8 + 0x258, uVar10);
    AD(piVar19 + 0x258, uVar10);
    piVar22 = piVar19 + 4;
    uVar10 = (unsigned int)LH(puVar20 + 0x1c4);
    iVar16 = (int)(SL * 0x3a09u + (unsigned int)iVar14);
    iVar14 = iVar14 + 1;
    AD(GA(DAT_00820dca, ((unsigned int)iVar16 + (unsigned int)iVar11) * 4u), uVar10);
    AD(piVar8 + 0x4c8, uVar10);
    AD(piVar19 + 0x4c8, uVar10);
    piVar8 = piVar8 + 4;
    puVar20 = puVar20 + 2;
    piVar19 = piVar22;
  } while (piVar22 < (char *)&DAT_0081f1e6);
  iVar14 = 0;
  iVar11 = (int)((unsigned int)local_2c * 6u);
  puVar20 = GA(DAT_0071b97b, iVar13);
  piVar8 = AX(0x81d04au, local_2c, 0x18u);
  piVar19 = (char *)&DAT_0081f506;
  do {
    uVar10 = (unsigned int)LH(puVar20);
    AD(TA(DAT_00820c86, (unsigned int)iVar14 + (unsigned int)iVar11), uVar10);
    AD(piVar8, uVar10);
    AD(piVar19, uVar10);
    uVar10 = (unsigned int)LH(puVar20 + 12);
    AD(TA(DAT_00820cce, (unsigned int)iVar14 + (unsigned int)iVar11), uVar10);
    AD(piVar8 + 0x48, uVar10);
    AD(piVar19 + 0x48, uVar10);
    uVar10 = (unsigned int)LH(puVar20 + 0x18);
    AD(TA(DAT_00820d16, (unsigned int)iVar14 + (unsigned int)iVar11), uVar10);
    AD(piVar8 + 0x90, uVar10);
    AD(piVar19 + 0x90, uVar10);
    piVar22 = piVar19 + 4;
    uVar10 = (unsigned int)LH(puVar20 + 0x160);
    iVar16 = (int)(SL * 0x3a09u + (unsigned int)iVar14);
    iVar14 = iVar14 + 1;
    AD(GA(DAT_00820ef6, ((unsigned int)iVar16 + (unsigned int)iVar11) * 4u), uVar10);
    AD(piVar8 + 0x270, uVar10);
    AD(piVar19 + 0x270, uVar10);
    puVar20 = puVar20 + 2;
    piVar8 = piVar8 + 4;
    piVar19 = piVar22;
  } while (piVar22 < (char *)&DAT_0081f51e);
  uVar10 = (unsigned int)LH(GA(DAT_0071b7c3, iVar13));
  AD(TA(DAT_00820d5e, local_2c), uVar10);
  piVar8 = AX(0x81d122u, local_2c, 4u);
  AD(piVar8, uVar10);
  AD(&DAT_0081f5de, uVar10);
  uVar10 = (unsigned int)LH(GA(DAT_0071b7b5, iVar13));
  AD(TA(DAT_00820d76, local_2c), uVar10);
  AD(AX(0x81d13au, local_2c, 4u), uVar10);
  AD(&DAT_0081f5f6, uVar10);
  uVar10 = (unsigned int)LH(GA(DAT_0071b7b9, iVar13));
  AD(TA(DAT_00820d76, local_2c), uVar10);
  piVar8 = AX(0x81d13au, local_2c, 4u);
  AD(piVar8, uVar10);
  AD(&DAT_0081f5f6, uVar10);
  uVar10 = (unsigned int)LH(GA(DAT_0071b7b7, iVar13));
  AD(TA(DAT_00820d6a, local_2c), uVar10);
  AD(AX(0x81d12eu, local_2c, 4u), uVar10);
  AD(&DAT_0081f5ea, uVar10);
  uVar10 = (unsigned int)LH(GA(DAT_0071b7bb, iVar13));
  AD(TA(DAT_00820d6a, local_2c), uVar10);
  AD(AX(0x81d12eu, local_2c, 4u), uVar10);
  AD(&DAT_0081f5ea, uVar10);
  uVar10 = (unsigned int)LH(GA(DAT_0071b7bd, iVar13));
  AD(TA(DAT_00820d8e, local_2c), uVar10);
  AD(AX(0x81d152u, local_2c, 4u), uVar10);
  AD(&DAT_0081f60e, uVar10);
  uVar10 = (unsigned int)LH(GA(DAT_0071b7bf, iVar13));
  AD(TA(DAT_00820d82, local_2c), uVar10);
  AD(AX(0x81d146u, local_2c, 4u), uVar10);
  AD(&DAT_0081f602, uVar10);
  uVar10 = (unsigned int)LH(GA(DAT_0071b9c1, iVar13));
  AD(TA(DAT_00820d9a, local_2c), uVar10);
  AD(AX(0x81d15eu, local_2c, 4u), uVar10);
  AD(&DAT_0081f61a, uVar10);
  uVar10 = (unsigned int)LH(GA(DAT_0071b9c3, iVar13));
  AD(TA(DAT_00820da6, local_2c), uVar10);
  AD(AX(0x81d16au, local_2c, 4u), uVar10);
  AD(&DAT_0081f626, uVar10);
  uVar10 = (unsigned int)LH(GA(DAT_0071b9c5, iVar13));
  AD(TA(DAT_00820db2, local_2c), uVar10);
  AD(AX(0x81d176u, local_2c, 4u), uVar10);
  AD(&DAT_0081f632, uVar10);
  uVar10 = (unsigned int)LH(GA(DAT_0071b9c7, iVar13));
  AD(TA(DAT_00820dbe, local_2c), uVar10);
  AD(AX(0x81d182u, local_2c, 4u), uVar10);
  AD(&DAT_0081f63e, uVar10);
  AD(SA(DAT_0081fc62, 0), iVar21);
  AD(&DAT_0081bcfe, iVar21);
  WI(DAT_0081e0a2, iVar21);
  _sprintf((char *)&DAT_00732800, s_Promo_points___d_Worse_Promo_poi_0052813c,
           LI(GA(DAT_0071b7a1, iVar13)), LI(GA(DAT_0071b7a5, iVar13)));
  WI(DAT_0081c022, 0);
  if (local_2c == 0) {
    if (RI(DAT_0081e016) < 7) {
      AD(&DAT_0081c01a, LI(PL(DAT_0071b7a1)));
      if (RI(DAT_0081c01e) < LI(A4(DAT_00527fc4, RI(DAT_0081e016))) / 2) {
        iVar13 = LI(PL(DAT_0071b7a5));
        AD(&DAT_0081c01e, iVar13);
        AD(&DAT_0081c01a, iVar13);
      }
      if (LI(A4(DAT_00527fc4, RI(DAT_0081e016))) <= RI(DAT_0081c01a)) {
        AD(&DAT_0081e016, 1);
        WI(DAT_0081c01a, 0);
        WI(DAT_0081c01e, 0);
        WI(DAT_0081c022, 1);
        ST4(A4(DAT_0081e01e, RI(DAT_0081e016)), RI(DAT_0081e01a));
      }
      if (RI(DAT_0081c01a) < 0) {
        if (RI(DAT_0081e016) == 0) {
          WI(DAT_0081c01a, 0);
          WI(DAT_0081c01e, 0);
          WI(DAT_0081c026, 0);
        }
        else {
          WI(DAT_0081c026, (int)((unsigned int)RI(DAT_0081c01a) * 100u) / 2000);
          if (RI(DAT_0081c026) < -100) {
            WI(DAT_0081c026, -100);
          }
        }
      }
      else {
        WI(DAT_0081c026, (int)((unsigned int)RI(DAT_0081c01a) * 100u) /
                         LI(A4(DAT_00527fc4, RI(DAT_0081e016))));
        if (100 < RI(DAT_0081c026)) {
          WI(DAT_0081c026, 100);
        }
      }
    }
    else {
      iVar13 = LI(PL(DAT_0071b7a1));
      if (iVar13 < 0) {
        AD(&DAT_0081c01a, iVar13);
      }
    }
joined_r0x004796f4:
    if ((RI(DAT_0081e016) != 0) && (RI(DAT_0081c01a) < -2000)) {
      AD(&DAT_0081e016, -1);
      WI(DAT_0081c022, -1);
      WI(DAT_0081c026, 0);
      WI(DAT_0081c01e, 0);
      WI(DAT_0081c01a, 0);
      if (RI(DAT_0081e016) < 2) {
        WI(DAT_0081c026, 0);
        WI(DAT_0081c022, -1);
        WI(DAT_0081c01e, 0);
        WI(DAT_0081c01a, 0);
        ST4(A4(DAT_0081e01e, RI(DAT_0081e016)), RI(DAT_0081e01a));
      }
    }
  }
  else if ((local_2c == 1) || (local_2c == 2)) {
    if (RI(DAT_0081e016) < 0x18) {
      AD(&DAT_0081c01a, LI(PL(DAT_0071b7a1)));
      iVar13 = LI(A4(DAT_00527fc4, RI(DAT_0081e016)));
      if (RI(DAT_0081c01e) < iVar13 / 2) {
        iVar11 = LI(PL(DAT_0071b7a5));
        AD(&DAT_0081c01e, iVar11);
        AD(&DAT_0081c01a, iVar11);
      }
      if (iVar13 <= RI(DAT_0081c01a)) {
        WI(DAT_0081c01e, 0);
        WI(DAT_0081c01a, (int)((unsigned int)RI(DAT_0081c01a) - (unsigned int)iVar13));
        WI(DAT_0081c022, 1);
        AD(&DAT_0081e016, 1);
        ST4(A4(DAT_0081e01e, RI(DAT_0081e016)), RI(DAT_0081e01a));
      }
      if (RI(DAT_0081c01a) < 0) {
        if (RI(DAT_0081e016) == 0) {
          WI(DAT_0081c01a, 0);
          WI(DAT_0081c01e, 0);
          WI(DAT_0081c026, 0);
        }
        else {
          WI(DAT_0081c026, (int)((unsigned int)RI(DAT_0081c01a) * 100u) / 2000);
          if (RI(DAT_0081c026) < -100) {
            WI(DAT_0081c026, -100);
          }
        }
      }
      else {
        WI(DAT_0081c026, (int)((unsigned int)RI(DAT_0081c01a) * 100u) /
                         LI(A4(DAT_00527fc4, RI(DAT_0081e016))));
        if (100 < RI(DAT_0081c026)) {
          WI(DAT_0081c026, 100);
        }
      }
    }
    else {
      iVar13 = LI(PL(DAT_0071b7a1));
      if (iVar13 < 0) {
        AD(&DAT_0081c01a, iVar13);
      }
    }
    goto joined_r0x004796f4;
  }
  piVar8 = (char *)&DAT_0071bcb8;
  iVar13 = (int)((unsigned int)RI(DAT_0071ecf4) * 0x5bdu);
  puVar20 = GA(DAT_0071b99f, iVar13);
  do {
    if (LI(piVar8) != 0) {
      iVar11 = 0;
      piVar19 = (char *)&DAT_0081f7de;
      do {
        if (LI(piVar19) == LI(piVar8)) {
          ST4(A4(DAT_0081e0a6, iVar11), LH(puVar20));
          ST4(A4(DAT_0081e0c6, iVar11), LH(puVar20 + 0x10));
          ST4(A4(DAT_0081e266, iVar11), LH(puVar20 + 0x2a));
          ST4(A4(DAT_0081e286, iVar11), LH(puVar20 + 0x3a));
          break;
        }
        piVar19 = piVar19 + 0x58;
        iVar11 = iVar11 + 1;
      } while (piVar19 < (char *)&DAT_0081fa9e);
    }
    puVar20 = puVar20 + 2;
    piVar8 = piVar8 + 0x5bd;
  } while (piVar8 < (char *)&DAT_0071eaa0);
  local_c = (char *)(int)DAT_006d49a0;
  uVar10 = 0;
  if (local_c != (char *)0) {
    puVar12 = (char *)&DAT_0081e0e6;
    pbVar9 = &DAT_0070ad49;
    puVar20 = GA(DAT_0071b7c5, iVar13);
    do {
      cVar4 = DAT_006d4a02;
      ST4(puVar12, LH(puVar20));
      ST4(puVar12 + 0xc0, LH(puVar20 + 0x60));
      ST4(puVar12 + 0x1c0, LH(puVar20 + 0x224));
      ST4(puVar12 + 0x280, LH(puVar20 + 0x284));
      if ((cVar4 == '\x02') && ((RI(DAT_0081c012) != 1 || (RI(DAT_0081d326) == 0)))) {
        uVar15 = LU(A4(DAT_00527fa8, *pbVar9));
        if (*pbVar9 != 0) {
          uVar15 = uVar15 + (uVar10 & 3);
        }
        ST4(puVar12 + 0x340, uVar15);
      }
      puVar12 = puVar12 + 4;
      pbVar9 = pbVar9 + 0x566;
      puVar20 = puVar20 + 2;
      uVar10 = uVar10 + 1;
    } while (uVar10 < (unsigned int)local_c);
  }
  uVar10 = 0;
  piVar8 = (char *)&DAT_007092e8;
  puVar20 = (char *)&DAT_00709338;
  piVar19 = (char *)&DAT_0081fa7e;
  pcVar18 = (char *)&DAT_00709d0a;
  do {
    if ((*pcVar18 == '\x01') && (pcVar18[1] != '\x01')) {
      ST4(piVar19 + 4, 1);
    }
    else {
      ST4(piVar19 + 4, 0);
    }
    iVar11 = LI(piVar8 + 0xc20);
    ST4(piVar19, LI(piVar8));
    uVar5 = LH(puVar20);
    ST4(piVar19 + 12, iVar11);
    ST4(piVar19 + 16, (unsigned int)uVar5);
    ST4(piVar19 + 20, (unsigned int)LH(puVar20 + 20));
    ST4(piVar19 + 24, (unsigned int)LH(puVar20 + 60));
    if (local_2c == 1) {
      AD(piVar19, LI(piVar8 + 40));
    }
    else {
      piVar22 = (char *)&DAT_0071bcb8;
      do {
        if ((LI(piVar22) != 0) && ((unsigned int)LH(piVar22 - 0x57c) == uVar10)) {
          AD(piVar19, LI(piVar22 - 0x51b));
        }
        piVar22 = piVar22 + 0x5bd;
        local_20 = (unsigned int)piVar8;
      } while (piVar22 < (char *)&DAT_0071eaa0);
    }
    piVar8 = piVar8 + 4;
    puVar20 = puVar20 + 2;
    piVar19 = piVar19 + 28;
    pcVar18 = pcVar18 + 3;
    uVar10 = uVar10 + 1;
  } while (pcVar18 < (char *)&DAT_00709d22);
  piVar8 = (char *)&DAT_0071bcb8;
  do {
    if (LI(piVar8) != 0) {
      iVar11 = 0;
      piVar19 = (char *)&DAT_0081f7de;
LAB_0047994b:
      if (LI(piVar19) != LI(piVar8)) goto code_r0x00479951;
      if (local_c != (char *)0) {
        uVar10 = (unsigned int)local_c;
        puVar20 = piVar8 - 0x4f3;
        do {
          uVar5 = LH(puVar20 + 0x60);
          AD(GA(DAT_0081f7ea, (unsigned int)iVar11 * 0x58u), (unsigned int)LH(puVar20));
          uVar6 = LH(puVar20 + 0xc0);
          AD(GA(DAT_0081f7ee, (unsigned int)iVar11 * 0x58u), (unsigned int)uVar5);
          uVar10 = uVar10 - 1;
          AD(GA(DAT_0081f7f6, (unsigned int)iVar11 * 0x58u), (unsigned int)uVar6);
          puVar20 = puVar20 + 2;
        } while (uVar10 != 0);
      }
      ST4(GA(DAT_0081f7e6, (unsigned int)iVar11 * 0x58u),
          (unsigned int)(&DAT_007092e8)[LH(piVar8 - 0x57c)] + (unsigned int)LI(piVar8 - 0x51b));
      ST4(GA(DAT_0081f7fa, (unsigned int)iVar11 * 0x58u), (unsigned int)LH(piVar8 - 0x2f7));
    }
LAB_004799ff:
    piVar8 = piVar8 + 0x5bd;
  } while (piVar8 < (char *)&DAT_0071eaa0);
  uVar10 = 0;
  do {
    iVar11 = (int)(SL * 0x3a09u + uVar10);
    uVar10 = uVar10 + 1;
    ST4(GA(DAT_0081fc42, (unsigned int)iVar11 * 4u), 0);
  } while (uVar10 < 4);
  local_30 = 0;
  if (local_2c == 0) {
    if (((&DAT_00709d0a)[(unsigned int)LH(GA(DAT_0071b73c, iVar13)) * 3] == '\x01') &&
       ((&DAT_00709d0b)[(unsigned int)LH(GA(DAT_0071b73c, iVar13)) * 3] != '\x01')) {
      iVar13 = 3;
      piVar8 = (char *)&DAT_0052807c;
      local_30 = 1;
      do {
        if (LI(piVar8) <= iVar21) break;
        piVar8 = piVar8 + 4;
        local_30 = local_30 + 1;
        iVar13 = iVar13 + -1;
      } while (iVar13 != 0);
      if (DAT_007092d4 == '\0') {
        local_30 = local_30 + 2;
      }
      else if (DAT_007092d4 == '\x01') {
        local_30 = local_30 + 1;
      }
      if ((DAT_007092db == '\x02') && ((int)local_30 < 5)) {
        local_30 = 5;
      }
      if (5 < (int)local_30) {
        local_30 = 0;
      }
    }
    else if (iVar21 < 1) {
      local_30 = 6;
    }
    goto joined_r0x00479fe9;
  }
  if (local_2c == 1) {
    local_28 = 0;
    local_8 = 0;
    uVar10 = (unsigned int)LH(GA(DAT_0071b73c, iVar13));
    piVar8 = (char *)&DAT_00709310;
    local_10 = (int)((unsigned int)(&DAT_007092e8)[uVar10] + (unsigned int)(&DAT_00709310)[uVar10]);
    uVar15 = 0;
    do {
      if (uVar10 != uVar15) {
        bVar7 = 0;
        if (local_c != (char *)0) {
          pbVar9 = &DAT_0070ad48;
          uVar24 = (unsigned int)local_c;
          do {
            if ((*pbVar9 == uVar15) &&
               ((LI(pbVar9 + 0x50a) != -1 || ((pbVar9[0xc] != 0 && (DAT_007092da != '\0')))))) {
              bVar7 = 1;
            }
            pbVar9 = pbVar9 + 0x566;
            uVar24 = uVar24 - 1;
          } while (uVar24 != 0);
        }
        if (bVar7) {
          if (local_10 < (int)((unsigned int)LI(piVar8 - 40) + (unsigned int)LI(piVar8))) {
            local_28 = local_28 + 1;
          }
          else {
            uVar24 = (unsigned int)local_10 -
                     ((unsigned int)LI(piVar8 - 40) + (unsigned int)LI(piVar8));
            if (((int)uVar24 < (int)local_8) || (local_8 == 0)) {
              local_8 = uVar24;
            }
          }
        }
      }
      piVar8 = piVar8 + 4;
      uVar15 = uVar15 + 1;
    } while (piVar8 < (char *)&DAT_00709338);
    local_20 = (unsigned int)(local_28 + 1);
    if ((local_4 < 2) || (local_14 < 2)) {
      if (DAT_007092d4 == '\0') {
        if (local_20 == 1) {
          local_30 = 5;
        }
        else if (4 < (int)local_20) {
LAB_00479e4e:
          local_30 = 6;
        }
      }
      else if (DAT_007092d4 == '\x01') {
        if (local_20 == 1) {
          if (iVar21 < 1) {
            local_30 = 5;
          }
          else {
            local_30 = 4;
            if ((int)local_8 < 0x2711) {
              if (5000 < (int)local_8) {
                local_30 = 3;
              }
            }
            else {
              local_30 = 2;
            }
          }
        }
        else if ((local_20 == 2) && (2 < local_4)) {
          local_30 = 5;
        }
        else if ((local_20 == local_4) || (6 < (int)local_20)) goto LAB_00479e4e;
      }
      else if (DAT_007092d4 == '\x02') {
        if (local_20 == 1) {
          if (iVar21 < 1) {
            local_30 = 5;
          }
          else {
            local_30 = 3;
            if ((int)local_8 < 0x2711) {
              if (5000 < (int)local_8) {
                local_30 = 2;
              }
            }
            else {
              local_30 = 1;
            }
          }
        }
        else if (local_20 == 2) {
          if (local_4 < 5) {
            if (2 < local_4) {
              local_30 = 5;
            }
          }
          else {
            local_30 = 4;
          }
        }
        else if ((local_20 == 3) && (4 < local_4)) {
          local_30 = 5;
        }
        else if (7 < (int)local_20) goto LAB_00479e4e;
      }
LAB_00479e56:
      if (local_30 != 0) {
        ST4(SA(DAT_0081fc42, 0), local_30);
      }
    }
    else {
      if ((local_4 == local_20) && (3 < local_4)) {
        local_30 = 6;
      }
      else if ((5000 < iVar21) && ((int)local_20 < 4)) {
        local_30 = (unsigned int)(&DAT_0052803c)[local_4 * 3 + local_20];
      }
      if (local_30 != 0) {
        if (local_30 != 6) {
          if (iVar21 < 0x4e2) {
            local_30 = local_30 + 2;
            if (5 < local_30) {
              local_30 = 5;
            }
          }
          else if ((iVar21 < 0x9c4) && (local_30 = local_30 + 1, 5 < local_30)) {
            local_30 = 5;
          }
        }
        goto LAB_00479e56;
      }
    }
    _sprintf((char *)&DAT_00732800, s_Player_s_team_score___d_Place____005280fc, local_10, local_20, local_8,
             local_30);
  }
  else if (local_2c == 2) {
    cVar4 = (&DAT_00709d0a)[(unsigned int)LH(GA(DAT_0071b73c, iVar13)) * 3];
    if ((cVar4 == '\x02') ||
       (cVar3 = (&DAT_00709d0b)[(unsigned int)LH(GA(DAT_0071b73c, iVar13)) * 3],
        cVar3 == '\x01')) {
LAB_00479fdb:
      local_30 = 6;
    }
    else if (local_14 == 1) {
      if ((cVar4 == '\x01') && (cVar3 != '\x01')) {
        iVar13 = 3;
        piVar8 = (char *)&DAT_0052807c;
        local_30 = 1;
        do {
          if (LI(piVar8) <= iVar21) break;
          piVar8 = piVar8 + 4;
          local_30 = local_30 + 1;
          iVar13 = iVar13 + -1;
        } while (iVar13 != 0);
        if (DAT_007092d4 == '\0') {
          local_30 = local_30 + 2;
        }
        else if (DAT_007092d4 == '\x01') {
          local_30 = local_30 + 1;
        }
        if (DAT_007092db == '\x02') {
          local_30 = 5;
        }
        if (5 < (int)local_30) {
          local_30 = 0;
        }
      }
      else if (iVar21 < 1) goto LAB_00479fdb;
    }
    else if ((cVar4 == '\x01') && (cVar3 != '\x01')) {
      local_30 = 1;
      piVar8 = (char *)&DAT_0052805c;
      do {
        if (LI(piVar8) <= iVar21) break;
        piVar8 = piVar8 + 4;
        local_30 = local_30 + 1;
      } while (piVar8 < (char *)0x52806d);
      if (DAT_007092db == '\x02') {
        local_30 = 5;
      }
      if (5 < (int)local_30) {
        local_30 = 0;
      }
    }
    else if (iVar21 < -25000) goto LAB_00479fdb;
joined_r0x00479fe9:
    if (local_30 != 0) {
      ST4(SA(DAT_0081fc4a, 0), local_30);
    }
  }
  iVar13 = DAT_0071ecf4;
  if (local_14 == 1) {
    if (local_2c == 0) {
      iVar11 = (int)((unsigned int)RI(DAT_0081bfbe) * 0x24u);
      AD(SA(DAT_00820f42, iVar11), 1);
      uVar10 = (unsigned int)LH(GA(DAT_0071b73c, (unsigned int)iVar13 * 0x5bdu));
      cVar4 = (&DAT_00709d0a)[uVar10 * 3];
      if (cVar4 == '\x01') {
        AD(SA(DAT_00820f46, iVar11), 1);
      }
      if ((cVar4 == '\x02') || ((&DAT_00709d0b)[uVar10 * 3] == '\x01')) {
        AD(SA(DAT_00820f4a, iVar11), 1);
      }
      if (DAT_007092db != '\x02') {
        if (LI(SA(DAT_00820f4e, iVar11)) < iVar21) {
          ST4(SA(DAT_00820f4e, iVar11), iVar21);
        }
        uVar10 = (&DAT_00709f08)[uVar10];
        if (uVar10 != 0) {
          if ((uVar10 < LU(SA(DAT_00820f52, iVar11))) ||
             (LU(SA(DAT_00820f52, iVar11)) == 0)) {
            ST4(SA(DAT_00820f52, iVar11), uVar10);
          }
        }
      }
      if (local_30 == 0) {
        if (LI(SA(DAT_00820f5a, iVar11)) == 6) {
          ST4(SA(DAT_00820f5a, iVar11), 0);
          iVar13 = LI(SA(DAT_0081fc26, 0));
          piVar8 = SA(DAT_0081fc26, 0);
joined_r0x0047a9d1:
          if (iVar13 != 0) {
            ST4(piVar8, (unsigned int)iVar13 - 1u);
          }
        }
      }
      else {
        uVar10 = LU(SA(DAT_00820f5a, iVar11));
        if ((local_30 < uVar10) || (uVar10 == 0)) {
          if (uVar10 != 0) {
            iVar13 = (int)(SL * 0x3a09u + uVar10);
            if (LI(GA(DAT_0081fc0e, (unsigned int)iVar13 * 4u)) != 0) {
              AD(GA(DAT_0081fc0e, (unsigned int)iVar13 * 4u), -1);
            }
          }
          ST4(SA(DAT_00820f5a, iVar11), local_30);
          AD(TA(DAT_0081fc0e, local_30), 1);
        }
      }
    }
    else if (local_2c == 1) {
      iVar11 = (int)((unsigned int)RI(DAT_0081bfc2) * 0x24u);
      AD(SA(DAT_00821d52, iVar11), 1);
      uVar10 = (unsigned int)LH(GA(DAT_0071b73c, (unsigned int)iVar13 * 0x5bdu));
      cVar4 = (&DAT_00709d0a)[uVar10 * 3];
      if (cVar4 == '\x01') {
        AD(SA(DAT_00821d56, iVar11), 1);
      }
      if ((cVar4 == '\x02') || ((&DAT_00709d0b)[uVar10 * 3] == '\x01')) {
        AD(SA(DAT_00821d5a, iVar11), 1);
      }
      if (LI(SA(DAT_00821d5e, iVar11)) < iVar21) {
        ST4(SA(DAT_00821d5e, iVar11), iVar21);
      }
      uVar10 = (&DAT_00709f08)[uVar10];
      if (uVar10 != 0) {
        if ((uVar10 < LU(SA(DAT_00821d62, iVar11))) ||
           (LU(SA(DAT_00821d62, iVar11)) == 0)) {
          ST4(SA(DAT_00821d62, iVar11), uVar10);
        }
      }
      if (local_20 != 0) {
        if ((local_20 < LU(SA(DAT_00821d66, iVar11))) ||
           (LU(SA(DAT_00821d66, iVar11)) == 0)) {
          ST4(SA(DAT_00821d66, iVar11), local_20);
        }
      }
      if (((local_20 == 1) &&
          (LU(SA(DAT_00821d6e, iVar11)) < local_8)) &&
         (0 < (int)local_8)) {
        ST4(SA(DAT_00821d6e, iVar11), local_8);
      }
      if (local_30 == 0) {
        if (LI(SA(DAT_00821d6a, iVar11)) == 6) {
          ST4(SA(DAT_00821d6a, iVar11), 0);
          if (LI(SA(DAT_0081fbf6, 0)) != 0) {
            AD(SA(DAT_0081fbf6, 0), -1);
          }
        }
      }
      else {
        uVar10 = LU(SA(DAT_00821d6a, iVar11));
        if ((local_30 < uVar10) || (uVar10 == 0)) {
          if (uVar10 != 0) {
            iVar13 = (int)(SL * 0x3a09u + uVar10);
            if (LI(GA(DAT_0081fbde, (unsigned int)iVar13 * 4u)) != 0) {
              AD(GA(DAT_0081fbde, (unsigned int)iVar13 * 4u), -1);
            }
          }
          ST4(SA(DAT_00821d6a, iVar11), local_30);
          AD(TA(DAT_0081fbde, local_30), 1);
        }
      }
    }
    else if (local_2c == 2) {
      iVar11 = (int)((unsigned int)RI(DAT_0081bfca) * 0x24u);
      AD(SA(DAT_0082407a, iVar11), 1);
      uVar10 = (unsigned int)LH(GA(DAT_0071b73c, (unsigned int)iVar13 * 0x5bdu));
      cVar4 = (&DAT_00709d0a)[uVar10 * 3];
      if (cVar4 == '\x01') {
        AD(SA(DAT_0082407e, iVar11), 1);
      }
      if ((cVar4 == '\x02') || ((&DAT_00709d0b)[uVar10 * 3] == '\x01')) {
        AD(SA(DAT_00824082, iVar11), 1);
      }
      if (DAT_007092db != '\x02') {
        if (LI(SA(DAT_00824086, iVar11)) < iVar21) {
          ST4(SA(DAT_00824086, iVar11), iVar21);
        }
        uVar10 = (&DAT_00709f08)[uVar10];
        if (uVar10 != 0) {
          if ((uVar10 < LU(SA(DAT_0082408a, iVar11))) ||
             (LU(SA(DAT_0082408a, iVar11)) == 0)) {
            ST4(SA(DAT_0082408a, iVar11), uVar10);
          }
        }
      }
      if (local_30 == 0) {
        if (LI(SA(DAT_00824092, iVar11)) == 6) {
          ST4(SA(DAT_00824092, iVar11), 0);
          if (LI(SA(DAT_0081fc26, 0)) != 0) {
            AD(SA(DAT_0081fc26, 0), -1);
          }
        }
      }
      else {
        uVar10 = LU(SA(DAT_00824092, iVar11));
        if ((local_30 < uVar10) || (uVar10 == 0)) {
          if (uVar10 != 0) {
            iVar13 = (int)(SL * 0x3a09u + uVar10);
            if (LI(GA(DAT_0081fc0e, (unsigned int)iVar13 * 4u)) != 0) {
              AD(GA(DAT_0081fc0e, (unsigned int)iVar13 * 4u), -1);
            }
          }
          ST4(SA(DAT_00824092, iVar11), local_30);
          AD(TA(DAT_0081fc0e, local_30), 1);
        }
      }
    }
  }
  else if (local_2c == 0) {
    iVar11 = (int)((unsigned int)RI(DAT_0081bfbe) * 0x30u);
    AD(SA(DAT_008263a2, iVar11), 1);
    uVar10 = (unsigned int)LH(GA(DAT_0071b73c, (unsigned int)iVar13 * 0x5bdu));
    cVar4 = (&DAT_00709d0a)[uVar10 * 3];
    if (cVar4 == '\x01') {
      AD(SA(DAT_008263b2, iVar11), 1);
    }
    if ((cVar4 == '\x02') || ((&DAT_00709d0b)[uVar10 * 3] == '\x01')) {
      AD(SA(DAT_008263b6, iVar11), 1);
    }
    if (DAT_007092db != '\x02') {
      if (LI(SA(DAT_008263ba, iVar11)) < iVar21) {
        ST4(SA(DAT_008263ba, iVar11), iVar21);
      }
      uVar10 = (&DAT_00709f08)[uVar10];
      if (uVar10 != 0) {
        if ((uVar10 < LU(SA(DAT_008263be, iVar11))) ||
           (LU(SA(DAT_008263be, iVar11)) == 0)) {
          ST4(SA(DAT_008263be, iVar11), uVar10);
        }
      }
    }
    if (local_30 == 0) {
      if (LI(SA(DAT_008263c6, iVar11)) == 6) {
        ST4(SA(DAT_008263c6, iVar11), 0);
        iVar13 = LI(SA(DAT_0081fc26, 0));
        piVar8 = SA(DAT_0081fc26, 0);
        goto joined_r0x0047a9d1;
      }
    }
    else {
      uVar10 = LU(SA(DAT_008263c6, iVar11));
      if ((local_30 < uVar10) || (uVar10 == 0)) {
        if (uVar10 == 6) {
          iVar13 = (int)(SL * 0x3a09u + 6u);
          if (LI(GA(DAT_0081fc0e, (unsigned int)iVar13 * 4u)) != 0) {
            AD(GA(DAT_0081fc0e, (unsigned int)iVar13 * 4u), -1);
          }
        }
        ST4(SA(DAT_008263c6, iVar11), local_30);
      }
      if ((local_30 != 6) || (LI(SA(DAT_008263c6, iVar11)) == 0)) {
        AD(TA(DAT_0081fc0e, local_30), 1);
      }
    }
  }
  else if (local_2c == 1) {
    iVar11 = (int)((unsigned int)RI(DAT_0081bfc2) * 0x30u);
    AD(SA(DAT_00827662, iVar11), 1);
    uVar10 = (unsigned int)LH(GA(DAT_0071b73c, (unsigned int)iVar13 * 0x5bdu));
    cVar4 = (&DAT_00709d0a)[uVar10 * 3];
    if (cVar4 == '\x01') {
      AD(SA(DAT_00827672, iVar11), 1);
    }
    if ((cVar4 == '\x02') || ((&DAT_00709d0b)[uVar10 * 3] == '\x01')) {
      AD(SA(DAT_00827676, iVar11), 1);
    }
    if (LI(SA(DAT_0082767a, iVar11)) < iVar21) {
      ST4(SA(DAT_0082767a, iVar11), iVar21);
    }
    uVar10 = (&DAT_00709f08)[uVar10];
    if (uVar10 != 0) {
      if ((uVar10 < LU(SA(DAT_0082767e, iVar11))) ||
         (LU(SA(DAT_0082767e, iVar11)) == 0)) {
        ST4(SA(DAT_0082767e, iVar11), uVar10);
      }
    }
    if (local_20 == 1) {
      AD(SA(DAT_00827666, iVar11), 1);
    }
    if (local_20 == 2) {
      AD(SA(DAT_0082766a, iVar11), 1);
    }
    if (local_20 == 3) {
      AD(SA(DAT_0082766e, iVar11), 1);
    }
    if (local_20 != 0) {
      if ((local_20 < LU(SA(DAT_00827682, iVar11))) ||
         (LU(SA(DAT_00827682, iVar11)) == 0)) {
        ST4(SA(DAT_00827682, iVar11), local_20);
      }
    }
    if (((local_20 == 1) &&
        (LU(SA(DAT_0082768a, iVar11)) < local_8)) && (0 < (int)local_8)) {
      ST4(SA(DAT_0082768a, iVar11), local_8);
    }
    if (local_30 == 0) {
      if (LI(SA(DAT_00827686, iVar11)) == 6) {
        ST4(SA(DAT_00827686, iVar11), 0);
        if (LI(SA(DAT_0081fbf6, 0)) != 0) {
          AD(SA(DAT_0081fbf6, 0), -1);
        }
      }
    }
    else {
      uVar10 = LU(SA(DAT_00827686, iVar11));
      if ((local_30 < uVar10) || (uVar10 == 0)) {
        if (uVar10 == 6) {
          iVar13 = (int)(SL * 0x3a09u + 6u);
          if (LI(GA(DAT_0081fbde, (unsigned int)iVar13 * 4u)) != 0) {
            AD(GA(DAT_0081fbde, (unsigned int)iVar13 * 4u), -1);
          }
        }
        ST4(SA(DAT_00827686, iVar11), local_30);
      }
      if ((local_30 != 6) || (LI(SA(DAT_00827686, iVar11)) == 0)) {
        AD(TA(DAT_0081fbde, local_30), 1);
      }
    }
  }
  else if (local_2c == 2) {
    iVar11 = (int)((unsigned int)RI(DAT_0081bfca) * 0x30u);
    AD(SA(DAT_0082a542, iVar11), 1);
    uVar10 = (unsigned int)LH(GA(DAT_0071b73c, (unsigned int)iVar13 * 0x5bdu));
    cVar4 = (&DAT_00709d0a)[uVar10 * 3];
    if (cVar4 == '\x01') {
      AD(SA(DAT_0082a552, iVar11), 1);
    }
    if ((cVar4 == '\x02') || ((&DAT_00709d0b)[uVar10 * 3] == '\x01')) {
      AD(SA(DAT_0082a556, iVar11), 1);
    }
    if (DAT_007092db != '\x02') {
      if (LI(SA(DAT_0082a55a, iVar11)) < iVar21) {
        ST4(SA(DAT_0082a55a, iVar11), iVar21);
      }
      uVar10 = (&DAT_00709f08)[uVar10];
      if (uVar10 != 0) {
        if ((uVar10 < LU(SA(DAT_0082a55e, iVar11))) ||
           (LU(SA(DAT_0082a55e, iVar11)) == 0)) {
          ST4(SA(DAT_0082a55e, iVar11), uVar10);
        }
      }
    }
    if (local_30 == 0) {
      if (LI(SA(DAT_0082a566, iVar11)) == 6) {
        ST4(SA(DAT_0082a566, iVar11), 0);
        piVar8 = SA(DAT_0081fc26, 0);
        iVar13 = LI(SA(DAT_0081fc26, 0));
        goto joined_r0x0047a9d1;
      }
    }
    else {
      uVar10 = LU(SA(DAT_0082a566, iVar11));
      if ((local_30 < uVar10) || (uVar10 == 0)) {
        if (uVar10 == 6) {
          iVar13 = (int)(SL * 0x3a09u + 6u);
          if (LI(GA(DAT_0081fc0e, (unsigned int)iVar13 * 4u)) != 0) {
            AD(GA(DAT_0081fc0e, (unsigned int)iVar13 * 4u), -1);
          }
        }
        ST4(SA(DAT_0082a566, iVar11), local_30);
      }
      if ((local_30 != 6) || (LI(SA(DAT_0082a566, iVar11)) == 0)) {
        AD(TA(DAT_0081fc0e, local_30), 1);
      }
    }
  }
  uVar10 = (unsigned int)RI(DAT_0081d3fa);
  iVar13 = RI(DAT_0081bfc6);
  piVar8 = (char *)(int)RI(DAT_0081c012);
  if (RI(DAT_0081c012) != 1) goto LAB_0047be3a;
  if (local_2c == 1) {
    piVar8 = (char *)&DAT_0081d332;
    local_28 = 0;
    local_c = (char *)&DAT_007092e8;
    do {
      if (LI(piVar8 - 4) == -1) {
        iVar21 = 0;
      }
      else {
        iVar21 = 0;
        iVar11 = 0;
        local_20 = (unsigned int)LI(local_c + 40) + (unsigned int)LI(local_c);
        piVar19 = (char *)&DAT_0081d32e;
        piVar22 = (char *)&DAT_00709310;
        do {
          if (((iVar11 != local_28) && (LI(piVar19) != -1)) &&
             ((int)local_20 < (int)((unsigned int)LI(piVar22 - 40) + (unsigned int)LI(piVar22)))) {
            iVar21 = iVar21 + 1;
          }
          piVar22 = piVar22 + 4;
          piVar19 = piVar19 + 20;
          iVar11 = iVar11 + 1;
        } while (piVar19 < (char *)&DAT_0081d3f6);
        iVar21 = iVar21 + 1;
      }
      AD(piVar8, local_20);
      if (iVar21 == 1) {
        AD(piVar8 + 4, 1);
      }
      else if (iVar21 == 2) {
        AD(piVar8 + 8, 1);
      }
      else if (iVar21 == 3) {
        AD(piVar8 + 12, 1);
      }
      piVar8 = piVar8 + 20;
      local_c = local_c + 4;
      local_28 = local_28 + 1;
    } while (local_c < (char *)&DAT_00709310);
    uVar17 = 0;
    iVar11 = 0;
    uVar24 = (unsigned int)LH(PL(DAT_0071b73c));
    uVar15 = 0;
    iVar21 = LI(GA(DAT_0081d332, uVar24 * 20u));
    piVar8 = (char *)&DAT_0081d332;
    do {
      if ((uVar24 != uVar17) && (LI(piVar8 - 4) != -1)) {
        if (iVar21 < LI(piVar8)) {
          iVar11 = iVar11 + 1;
        }
        else if (((int)((unsigned int)iVar21 - (unsigned int)LI(piVar8)) < (int)uVar15) || (uVar15 == 0)) {
          uVar15 = (unsigned int)iVar21 - (unsigned int)LI(piVar8);
        }
      }
      piVar8 = piVar8 + 20;
      uVar17 = uVar17 + 1;
    } while (piVar8 < (char *)&DAT_0081d3fa);
    uVar25 = (unsigned int)(iVar11 + 1);
    uVar17 = 0;
    if ((iVar21 < 0x1389) || (3 < (int)uVar25)) {
LAB_0047b167:
      if (uVar17 == 0) goto LAB_0047b16b;
    }
    else {
      uVar17 = (unsigned int)(&DAT_0052803c)[uVar25 + uVar10 * 3];
      if (uVar17 != 0) {
        if (RI(DAT_0081d3f6) < 3) {
          if (DAT_007092d4 == '\x02') {
            uVar17 = uVar17 + 1;
          }
          else {
            uVar17 = uVar17 + 2;
          }
joined_r0x0047b13b:
          if (5 < uVar17) {
            uVar17 = 5;
          }
        }
        else if (RI(DAT_0081d3f6) < 5) {
          if (DAT_007092d4 != '\x02') {
            uVar17 = uVar17 + 1;
          }
          goto joined_r0x0047b13b;
        }
        if (uVar25 == 1) {
          if ((int)uVar15 < 0x4e21) {
            if (10000 < (int)uVar15) {
              uVar17 = uVar17 - 1;
            }
          }
          else {
            uVar17 = uVar17 - 2;
          }
          if ((int)uVar17 < 1) {
            uVar17 = 1;
          }
        }
        goto LAB_0047b167;
      }
LAB_0047b16b:
      if ((3 < uVar10) && (uVar10 == uVar25)) {
        uVar17 = 6;
      }
    }
    if (RI(DAT_0081d3f6) == 1) {
      if (RI(DAT_0081d326) == 0) {
        AD(SA(DAT_0082d422, (unsigned int)iVar13 * 0x28u), 1);
      }
      else if ((int)((unsigned int)RI(DAT_0081d32a) - (unsigned int)RI(DAT_0081d326)) == 1) {
        iVar13 = (int)((unsigned int)iVar13 * 0x28u);
        AD(SA(DAT_0082d426, iVar13), 1);
        if (uVar25 == 1) {
          AD(SA(DAT_0082d42a, iVar13), 1);
        }
        if (uVar25 == 2) {
          AD(SA(DAT_0082d42e, iVar13), 1);
        }
        if (uVar25 == 3) {
          AD(SA(DAT_0082d432, iVar13), 1);
        }
        iVar21 = LI(GA(DAT_0081d332, uVar24 * 20u));
        if (LI(SA(DAT_0082d436, iVar13)) < iVar21) {
          ST4(SA(DAT_0082d436, iVar13), iVar21);
        }
        if ((uVar25 < LU(SA(DAT_0082d43a, iVar13))) ||
           (LU(SA(DAT_0082d43a, iVar13)) == 0)) {
          ST4(SA(DAT_0082d43a, iVar13), uVar25);
        }
        if (((uVar25 == 1) && (LU(SA(DAT_0082d442, iVar13)) < uVar15))
           && (0 < (int)uVar15)) {
          ST4(SA(DAT_0082d442, iVar13), uVar15);
        }
        if (uVar17 == 0) {
          if (LI(SA(DAT_0082d43e, iVar13)) == 6) {
            ST4(SA(DAT_0082d43e, iVar13), 0);
            iVar13 = LI(SA(DAT_0081fc0e, 0));
joined_r0x0047b6d6:
            if (iVar13 != 0) {
              ST4(SA(DAT_0081fc0e, 0), (unsigned int)iVar13 - 1u);
            }
          }
        }
        else {
          ST4(SA(DAT_0081fc46, 0), uVar17);
          uVar10 = LU(SA(DAT_0082d43e, iVar13));
          if ((uVar17 < uVar10) || (uVar10 == 0)) {
            if (uVar10 != 0) {
              iVar21 = (int)(SL * 0x3a09u + uVar10);
              if (LI(GA(DAT_0081fbf6, (unsigned int)iVar21 * 4u)) != 0) {
                AD(GA(DAT_0081fbf6, (unsigned int)iVar21 * 4u), -1);
              }
            }
            ST4(SA(DAT_0082d43e, iVar13), uVar17);
            AD(TA(DAT_0081fbf6, uVar17), 1);
          }
        }
      }
    }
    else if (RI(DAT_0081d326) == 0) {
      AD(SA(DAT_0082d80a, (unsigned int)iVar13 * 0x2cu), 1);
    }
    else if ((int)((unsigned int)RI(DAT_0081d32a) - (unsigned int)RI(DAT_0081d326)) == 1) {
      iVar13 = (int)((unsigned int)iVar13 * 0x2cu);
      AD(SA(DAT_0082d80e, iVar13), 1);
      if (uVar25 == 1) {
        AD(SA(DAT_0082d812, iVar13), 1);
      }
      if (uVar25 == 2) {
        AD(SA(DAT_0082d816, iVar13), 1);
      }
      if (uVar25 == 3) {
        AD(SA(DAT_0082d81a, iVar13), 1);
      }
      iVar21 = LI(GA(DAT_0081d332, uVar24 * 20u));
      if (LI(SA(DAT_0082d81e, iVar13)) < iVar21) {
        ST4(SA(DAT_0082d81e, iVar13), iVar21);
      }
      if ((uVar25 < LU(SA(DAT_0082d822, iVar13))) ||
         (LU(SA(DAT_0082d822, iVar13)) == 0)) {
        ST4(SA(DAT_0082d822, iVar13), uVar25);
      }
      if (((uVar25 == 1) && (LU(SA(DAT_0082d82e, iVar13)) < uVar15)) &&
         (0 < (int)uVar15)) {
        ST4(SA(DAT_0082d82e, iVar13), uVar15);
      }
      if (uVar17 == 0) {
        if (LI(SA(DAT_0082d82a, iVar13)) == 6) {
          ST4(SA(DAT_0082d82a, iVar13), 0);
          iVar13 = LI(SA(DAT_0081fc0e, 0));
          goto joined_r0x0047b6d6;
        }
      }
      else {
        ST4(SA(DAT_0081fc46, 0), uVar17);
        uVar10 = LU(SA(DAT_0082d82a, iVar13));
        if ((uVar17 < uVar10) || (uVar10 == 0)) {
          if (uVar10 == 6) {
            iVar21 = (int)(SL * 0x3a09u + 6u);
            if (LI(GA(DAT_0081fbf6, (unsigned int)iVar21 * 4u)) != 0) {
              AD(GA(DAT_0081fbf6, (unsigned int)iVar21 * 4u), -1);
            }
          }
          ST4(SA(DAT_0082d82a, iVar13), uVar17);
        }
        AD(TA(DAT_0081fbf6, uVar17), 1);
      }
    }
  }
  iVar13 = RI(DAT_0081bfce);
  piVar8 = (char *)(int)RI(DAT_0081c012);
  if ((RI(DAT_0081c012) != 1) || (piVar8 = (char *)local_2c, local_2c != 2))
  goto LAB_0047be3a;
  iVar21 = 0;
  pcVar18 = (char *)&DAT_00709d0a;
  do {
    if ((*pcVar18 == '\x01') && (pcVar18[1] != '\x01')) break;
    pcVar18 = pcVar18 + 3;
    iVar21 = iVar21 + 1;
  } while (pcVar18 < (char *)&DAT_00709d28);
  if (iVar21 == 10) {
    iVar11 = 2;
  }
  else {
    iVar11 = 0;
    if (iVar21 != 0) {
      iVar11 = 2 - (int)(unsigned int)(iVar21 == 1);
    }
  }
  iVar14 = 0;
  iVar16 = 0;
  ST4(A4(DAT_0081d40e, RI(DAT_0081d402)), iVar11);
  piVar8 = (char *)&DAT_0081d40e;
  iVar21 = (int)((unsigned int)RI(DAT_0081d402) + 1u);
  do {
    if (LI(piVar8) == 0) {
      iVar14 = iVar14 + 1;
    }
    else if (LI(piVar8) == 1) {
      iVar16 = iVar16 + 1;
    }
    piVar8 = piVar8 + 4;
    iVar21 = iVar21 + -1;
  } while (iVar21 != 0);
  if (iVar14 == RI(DAT_0081d40a)) {
    iVar21 = 0;
    uVar10 = (unsigned int)iVar14 - (unsigned int)iVar16;
  }
  else if (iVar16 == RI(DAT_0081d40a)) {
    iVar21 = 1;
    uVar10 = (unsigned int)iVar16 - (unsigned int)iVar14;
  }
  else {
    iVar21 = 2;
    uVar10 = 0;
  }
  iVar11 = 0;
  if (iVar21 != 2) {
    if (((iVar21 == 0) && (LS(PL(DAT_0071b73c)) == 0)) ||
       ((iVar21 == 1 && (LS(PL(DAT_0071b73c)) == 1)))) {
      iVar11 = 1;
    }
    else {
      iVar11 = 2;
    }
  }
  piVar19 = (char *)0;
  if (iVar11 != 0) {
    if ((iVar11 == 2) && (1 < (int)uVar10)) {
      piVar19 = (char *)6;
    }
    else {
      if (iVar11 != 1) goto LAB_0047b8e8;
      iVar11 = 0;
      piVar8 = (char *)&DAT_0071bcb8;
      local_20 = 0;
      do {
        if (LI(piVar8) != 0) {
          if (LS(PL(DAT_0071b73c)) == LS(piVar8 - 0x57c)) {
            local_20 = local_20 + 1;
          }
          else {
            iVar11 = iVar11 + 1;
          }
        }
        piVar8 = piVar8 + 0x5bd;
      } while (piVar8 < (char *)&DAT_0071eaa0);
      iVar11 = (int)((unsigned int)iVar11 - local_20);
      if (0 < iVar11) {
        iVar11 = (int)((unsigned int)iVar11 * 2u);
      }
      if ((RI(DAT_0081e00e) == 1) && (DAT_007092d4 == '\x02')) {
        uVar10 = uVar10 * 2;
      }
      iVar11 = (int)(uVar10 + (unsigned int)iVar11);
      if (iVar11 < 5) {
        if (iVar11 < 4) {
          piVar19 = (char *)3;
          if (iVar11 < 3) {
            piVar19 = (char *)((unsigned int)(iVar11 < 2) + 4u);
          }
        }
        else {
          piVar19 = (char *)2;
        }
      }
      else {
        piVar19 = (char *)1;
      }
    }
    ST4(SA(DAT_0081fc4e, 0), (unsigned int)piVar19);
  }
LAB_0047b8e8:
  if (RI(DAT_0081d402) == 0) {
    WI(DAT_0081e012, RI(DAT_0081e0a2));
  }
  else {
    AD(&DAT_0081e012, RI(DAT_0081e0a2));
  }
  if (RI(DAT_0081e00e) < 2) {
    if (RI(DAT_0081d402) == 0) {
      AD(SA(DAT_0082dc56, (unsigned int)iVar13 * 0x24u), 1);
    }
    iVar11 = 0;
    if (iVar21 != 2) {
      if (((iVar21 == 0) && (LS(PL(DAT_0071b73c)) == 0)) ||
         ((iVar21 == 1 && (LS(PL(DAT_0071b73c)) == 1)))) {
        AD(SA(DAT_0082dc5a, (unsigned int)iVar13 * 0x24u), 1);
        iVar11 = 1;
      }
      else {
        AD(SA(DAT_0082dc5e, (unsigned int)iVar13 * 0x24u), 1);
        iVar11 = 2;
      }
    }
    if (RI(DAT_0081d402) == 0x100) {
      AD(SA(DAT_0082dc62, (unsigned int)iVar13 * 0x24u), 1);
      iVar11 = 0;
    }
    iVar13 = (int)((unsigned int)iVar13 * 0x24u);
    if (LI(SA(DAT_0082dc66, iVar13)) < RI(DAT_0081e012)) {
      ST4(SA(DAT_0082dc66, iVar13), RI(DAT_0081e012));
    }
    if ((iVar11 == 1) && (LU(SA(DAT_0082dc72, iVar13)) < uVar10)) {
      ST4(SA(DAT_0082dc72, iVar13), uVar10);
    }
    if (piVar19 == (char *)0) {
      piVar8 = SA(DAT_0082dc6e, iVar13);
      if (LI(SA(DAT_0082dc6e, iVar13)) == 6) {
        ST4(piVar8, 0);
        piVar8 = SA(DAT_0081fc3e, 0);
        if (LI(SA(DAT_0081fc3e, 0)) != 0) {
          ST4(piVar8, (unsigned int)LI(SA(DAT_0081fc3e, 0)) - 1u);
          return (unsigned int)piVar8 & 0xffff0000u;
        }
      }
    }
    else {
      piVar8 = (char *)LU(SA(DAT_0082dc6e, iVar13));
      if (((unsigned int)piVar19 < (unsigned int)piVar8) || (piVar8 == (char *)0)) {
        if (piVar8 != (char *)0) {
          iVar21 = (int)(SL * 0x3a09u + (unsigned int)piVar8);
          if (LI(GA(DAT_0081fc26, (unsigned int)iVar21 * 4u)) != 0) {
            AD(GA(DAT_0081fc26, (unsigned int)iVar21 * 4u), -1);
          }
        }
        ST4(SA(DAT_0082dc6e, iVar13), (unsigned int)piVar19);
        uVar10 = SL * 0x3a09u + (unsigned int)piVar19;
        AD(GA(DAT_0081fc26, uVar10 * 4u), 1);
        return uVar10 & 0xffff0000u;
      }
    }
  }
  else {
    if (RI(DAT_0081d402) == 0) {
      AD(SA(DAT_0082dfda, (unsigned int)iVar13 * 0x28u), 1);
    }
    iVar11 = 0;
    if (iVar21 != 2) {
      if (((iVar21 == 0) && (LS(PL(DAT_0071b73c)) == 0)) ||
         ((iVar21 == 1 && (LS(PL(DAT_0071b73c)) == 1)))) {
        AD(SA(DAT_0082dfde, (unsigned int)iVar13 * 0x28u), 1);
        iVar11 = 1;
      }
      else {
        AD(SA(DAT_0082dfe2, (unsigned int)iVar13 * 0x28u), 1);
        iVar11 = 2;
      }
    }
    if (RI(DAT_0081d402) == 0x100) {
      AD(SA(DAT_0082dfe6, (unsigned int)iVar13 * 0x28u), 1);
      iVar11 = 0;
    }
    iVar13 = (int)((unsigned int)iVar13 * 0x28u);
    if (LI(SA(DAT_0082dfea, iVar13)) < RI(DAT_0081e012)) {
      ST4(SA(DAT_0082dfea, iVar13), RI(DAT_0081e012));
    }
    if ((iVar11 == 1) && (LU(SA(DAT_0082dffa, iVar13)) < uVar10)) {
      ST4(SA(DAT_0082dffa, iVar13), uVar10);
    }
    if (piVar19 != (char *)0) {
      piVar8 = (char *)LU(SA(DAT_0082dff6, iVar13));
      if (((unsigned int)piVar19 < (unsigned int)piVar8) || (piVar8 == (char *)0)) {
        if (piVar8 == (char *)6) {
          iVar21 = (int)(SL * 0x3a09u + 6u);
          if (LI(GA(DAT_0081fc26, (unsigned int)iVar21 * 4u)) != 0) {
            AD(GA(DAT_0081fc26, (unsigned int)iVar21 * 4u), -1);
          }
        }
        ST4(SA(DAT_0082dff6, iVar13), (unsigned int)piVar19);
      }
      uVar10 = SL * 0x3a09u + (unsigned int)piVar19;
      AD(GA(DAT_0081fc26, uVar10 * 4u), 1);
      return uVar10 & 0xffff0000u;
    }
    piVar8 = SA(DAT_0082dff6, iVar13);
    if (LI(SA(DAT_0082dff6, iVar13)) == 6) {
      ST4(piVar8, 0);
      piVar8 = SA(DAT_0081fc3e, 0);
      if (LI(SA(DAT_0081fc3e, 0)) != 0) {
        ST4(piVar8, (unsigned int)LI(SA(DAT_0081fc3e, 0)) - 1u);
      }
    }
  }
LAB_0047be3a:
  return (unsigned int)piVar8 & 0xffff0000u;
code_r0x00479951:
  piVar19 = piVar19 + 0x58;
  iVar11 = iVar11 + 1;
  if ((char *)&DAT_0081fa9a + 3 < piVar19) goto LAB_004799ff;
  goto LAB_0047994b;
}
