/* SPDX-License-Identifier: MIT */
/* z_bop 004a2020 */
/* Prototypes */
void FUN_004035d0(int, int, short, short);
void FUN_00458e20(unsigned int, int);
void FUN_004a44d0(int, int);
unsigned int FUN_004a4d70(unsigned int, unsigned int);
void FUN_00450650(int, int);
unsigned int FUN_0042e4d0(int, int, int);
void FUN_004525a0(unsigned int, int, unsigned char *);
void FUN_00451bf0(int, unsigned char *);
unsigned int FUN_00430200(int, int, unsigned int, unsigned int);
unsigned int FUN_00425b70(unsigned int, unsigned short);
void FUN_004aba50(void);
/* original pushes 8 argument bytes (stack-check evidence); the unaff_* values
   in Ghidra's call are register leftovers, not stack arguments */
void FUN_00440da0(unsigned int, int);
void FUN_004abe50(void);
void FUN_00451c20(unsigned int, unsigned int *, int);
void FUN_004a3750(unsigned int, int, int);
int FUN_0046b610(char *);
void FUN_00402cb0(unsigned short);
void FUN_00402970(unsigned int);
void FUN_00434500(unsigned int, unsigned int);
unsigned int FUN_00425cb0(void);

/* Globals */
extern unsigned char DAT_009a73b6[4];   /* packed (unaligned) pointer global, read via memcpy */
extern unsigned char *DAT_009a1fe8;
extern unsigned char *DAT_00a08104;
extern unsigned char *DAT_009ecc34;
extern unsigned short DAT_009a73b0;
extern unsigned short DAT_009a73c0;
extern unsigned short DAT_009a73c2;
extern unsigned short DAT_009a8c00;
extern unsigned short DAT_00a08292;
extern unsigned short DAT_00a08c74;
extern int DAT_00a07cdc;
extern int DAT_00a07cd8;
extern int DAT_00a07cd4;
extern int DAT_009d8a58;
extern int DAT_009d8a54;
extern unsigned int DAT_009d8b60;
extern unsigned char DAT_009d8cd4[];
extern unsigned char DAT_009d8cd5[];
extern unsigned char DAT_009d8cd2[];
extern unsigned char DAT_009d8c80[];
extern unsigned char DAT_009d8c84[];
extern unsigned char DAT_009d8c85[];
extern unsigned char DAT_009d8c87[];
extern unsigned char DAT_0051c637[];
extern unsigned char DAT_0051c639[];
extern unsigned char DAT_0051c63b[];
extern unsigned char DAT_0051c63d[];
extern unsigned char DAT_0051c63f[];
extern unsigned char DAT_0051c632[];
extern unsigned char DAT_0051c5aa[];
extern unsigned char DAT_0051c5ac[];
extern unsigned char DAT_0051c5b0[];
extern unsigned char DAT_00524208[];
extern unsigned char DAT_00524120[];
extern unsigned char DAT_00524148[];
extern unsigned char DAT_009a7a40[];
extern unsigned char DAT_009a6ff0[];
extern unsigned char DAT_009a7338[];
extern unsigned char DAT_009a1ed0[];
extern unsigned char DAT_009ec4d0[];
extern unsigned char DAT_009a20d6[];
extern unsigned char DAT_009a20da[];
extern unsigned char DAT_009a20de[];
extern unsigned char DAT_009a20e2[];
extern unsigned char DAT_009a20fe[];
extern unsigned char DAT_009a210a[];
extern unsigned char DAT_009a210c[];
extern unsigned char DAT_009a210e[];
extern unsigned char DAT_009a211c[];
extern unsigned char DAT_009e9670[];
extern unsigned char DAT_009e968a[];
extern unsigned char DAT_009e9a77[];
extern char DAT_009d1320[];
extern char s_boardtogivepln_0051a254[];
extern char s_boardtoexchangepln_0051a230[];
extern char s_boardtocontactpln_0051a208[];
extern char s_boardtopickuppln_0051a21c[];
extern char s_boardtorepairpln_0051a1f4[];
extern char s_boardtocapturepln_0051a278[];
extern char s_boardtotakepln_0051a244[];
extern char s_boardtodestroypln_0051a264[];
extern char s_selfdestroypln_00521210[];
extern char s_stationaryldrpln_0051a1bc[];
extern char s_flyhomeevadepln_00521628[];

/* Unaligned access helpers (macros, not functions) */
#define RD16(p) ({ unsigned short t_; __builtin_memcpy(&t_, (p), 2); t_; })
#define RD32(p) ({ unsigned int t_; __builtin_memcpy(&t_, (p), 4); t_; })
#define WR16(p, v) do { unsigned short t_ = (unsigned short)(v); __builtin_memcpy((p), &t_, 2); } while (0)
#define WR32(p, v) do { unsigned int t_ = (unsigned int)(v); __builtin_memcpy((p), &t_, 4); } while (0)
/* current value of the packed pointer global DAT_009a73b6, re-read on every use */
#define SB ({ unsigned char *sbt_; __builtin_memcpy(&sbt_, DAT_009a73b6, 4); sbt_; })
/* start of the 0x37-byte block at DAT_009a73b0; the address goes through a
   volatile local so the compiler does not see the 2-byte declared object size */
#define BLK (pBlk)
#define CONCAT22(a, b) ((((unsigned int)(unsigned short)(a)) << 16) | (unsigned int)(unsigned short)(b))
#define P19 ((unsigned char *)puVar19)
#define SELF_ENT (DAT_009a1fe8 + (unsigned int)DAT_009a73b0 * 0x23)
#define local_38 (u_38.p)

unsigned int __stdcall caseD_12(void)
{
  unsigned char bVar1;
  unsigned short uVar2;
  int iVar3;
  unsigned int uVar4;
  int bVar5;
  unsigned short uVar6;
  unsigned short uVar7;
  unsigned char *pbVar8;
  unsigned char *pbVar9;
  unsigned char *pbVar10;
  unsigned int uVar11;
  unsigned int uVar12;
  unsigned short uVar13;
  short sVar14;
  int iVar15;
  unsigned char *extraout_ECX = 0;
  unsigned short uVar16;
  unsigned char bVar17;
  int iVar18;
  unsigned int *puVar19;
  char *pcVar20;
  char *pcVar22;
  char *pcVar25;
  int bVar24;
  unsigned short local_52;
  unsigned short uStack_50;
  short sStack_4c;
  int local_48;
  unsigned short local_40;
  unsigned short uStack_3e;
  int local_3c = 0;
  union { char *p; unsigned char buf[0x37]; } u_38 = { 0 };
  unsigned char *volatile pBlk = (unsigned char *)&DAT_009a73b0;

  uVar6 = *(unsigned short *)(SB + 0x18);
  pbVar8 = SB + 0x18;
  uVar13 = (unsigned short)DAT_009d8cd4[(unsigned int)DAT_009a73c0 * 0x566 + (unsigned int)DAT_009a73c2 * 0x52];
  uVar12 = (unsigned int)uVar6;
  iVar3 = (int)(uVar12 * 0x23);
  pbVar9 = DAT_009a1fe8 + iVar3;
  iVar15 = (int)RD32(pbVar9 + 0x1f);
  pbVar10 = pbVar9 + 0x1f;
  if (iVar15 == 0) {
    bVar17 = pbVar9[0x16];
    uStack_50 = 0xff;
    uVar2 = *(unsigned short *)pbVar9;
    puVar19 = (unsigned int *)local_38;
  } else {
    puVar19 = (unsigned int *)RD32((unsigned char *)iVar15 + 0xa9);
    local_3c = (int)puVar19 + 0x2a;
    uStack_50 = (unsigned short)*(unsigned char *)(puVar19 + 1);
    bVar17 = pbVar9[0x16];
    uVar2 = *(unsigned short *)pbVar9;
  }
  local_52 = (unsigned short)bVar17;
  sStack_4c = (short)(unsigned int)(pbVar9 + 0x16);
  uStack_3e = (unsigned short)((unsigned int)pbVar8 >> 0x10);

  switch (SB[0x35]) {
  case 0:
    if (iVar15 == 0) {
      FUN_00458e20(uVar12, 0);
      WR32(SB + 0x1f, DAT_00a07cdc);
      WR32(SB + 0x23, DAT_00a07cd8);
      WR32(SB + 0x27, (unsigned int)DAT_00a07cd4 + 0x800u);
    } else {
      iVar15 = (int)((unsigned int)uStack_50 * 0x101);
      if ((pbVar9[2] == 0) || (pbVar9[2] == 1)) {
        sVar14 = (short)((short)RD16(DAT_0051c639 + iVar15) -
                         (short)RD16(DAT_0051c63d + (unsigned int)DAT_00a08104[4] * 0x101));
      } else if ((DAT_009a1fe8[(unsigned int)DAT_009a73b0 * 0x23 + 2] == 0) ||
                 (DAT_009a1fe8[(unsigned int)DAT_009a73b0 * 0x23 + 2] == 1)) {
        sVar14 = (short)(((short)RD16(DAT_0051c639 + iVar15) -
                          (short)RD16(DAT_0051c63f + (unsigned int)DAT_00a08104[4] * 0x101)) +
                         (short)RD16(DAT_0051c63b + iVar15));
      } else {
        sVar14 = (short)((short)RD16(DAT_0051c63b + iVar15) * 2 -
                         (short)RD16(DAT_0051c63f + (unsigned int)DAT_00a08104[4] * 0x101));
      }
      sVar14 = (short)(sVar14 + ((short)RD16(DAT_0051c63b + iVar15) -
                                 (short)RD16(DAT_0051c63f + (unsigned int)DAT_00a08104[4] * 0x101)) * 2);
      if (sVar14 < 0) {
        sVar14 = 0x7000;
      }
      FUN_004035d0((int)pbVar9, 0, sVar14, (short)RD16(DAT_0051c637 + iVar15));
      WR32(SB + 0x1f, RD32(DAT_009a1fe8 + iVar3 + 4) + (unsigned int)DAT_009d8a58);
      WR32(SB + 0x23, RD32(DAT_009a1fe8 + iVar3 + 8) + (unsigned int)DAT_009d8a54);
      WR32(SB + 0x27, RD32(DAT_009a1fe8 + iVar3 + 0xc) + DAT_009d8b60);
    }
    FUN_004a44d0(0, 1);
    if (0x4000 < DAT_009a8c00) {
      FUN_004a4d70((unsigned int)DAT_009a73b0, 0xffff);
    }
    if (0x2000 < DAT_009a8c00) {
      uVar12 = FUN_004a4d70((unsigned int)DAT_009a73b0, 0x8000);
      return uVar12 & 0xffff0000;
    }
    if (0x800 < DAT_009a8c00) {
      uVar12 = FUN_004a4d70((unsigned int)DAT_009a73b0, 0x4000);
      return uVar12 & 0xffff0000;
    }
    if ((RD32(DAT_009a1fe8 + iVar3 + 0x1b) == 0xffffffffu) ||
        (RD32(DAT_009a1fe8 + iVar3 + 0x1f) == 0) ||
        ((short)RD16((unsigned char *)RD32(DAT_009a1fe8 + iVar3 + 0x1f) + 0x79) == 0)) {
      uVar12 = FUN_004a4d70((unsigned int)DAT_009a73b0, 0);
      SB[0x35] = 1;
      return uVar12 & 0xffff0000;
    }
    pbVar9 = DAT_009ecc34;
    if (RD32(DAT_009a1fe8 + iVar3 + 0x1b) == (unsigned int)DAT_009ecc34) {
      uVar12 = 0;
      pbVar9 = DAT_009a6ff0;
      do {
        if (*(short *)pbVar9 == 0x104) break;
        pbVar9 = pbVar9 + 0x54;
        uVar12 = uVar12 + 1;
      } while (pbVar9 < DAT_009a7338);
      if (9 < uVar12) {
        DAT_00a08292 = (unsigned short)DAT_009d8c87[(unsigned int)DAT_009a73c0 * 0x566];
        FUN_00450650(0x104, (int)DAT_009ecc34);
        uVar12 = FUN_0042e4d0(0x24, -1, (int)DAT_009ecc34);
        return uVar12 & 0xffff0000;
      }
    }
    break;

  case 1:
    if (iVar15 == 0) {
      FUN_00458e20(uVar12, 0);
      WR32(DAT_00a08104 + 0xb1, (unsigned int)DAT_00a07cdc - RD32(SELF_ENT + 4));
      local_48 = (int)RD32(DAT_00a08104 + 0xb1);
      WR32(DAT_00a08104 + 0xb5, (unsigned int)DAT_00a07cd8 - RD32(SELF_ENT + 8));
      iVar15 = (int)RD32(DAT_00a08104 + 0xb5);
      sStack_4c = 0;
      sVar14 = 0;
      WR32(DAT_00a08104 + 0xb9, ((unsigned int)DAT_00a07cd4 - RD32(SELF_ENT + 0xc)) + 0x80u);
      local_40 = 0x4000;
      iVar18 = (int)RD32(DAT_00a08104 + 0xb9);
    } else {
      iVar15 = (int)((unsigned int)uStack_50 * 0x101);
      if ((pbVar9[2] == 0) || (pbVar9[2] == 1)) {
        sVar14 = (short)((short)RD16(DAT_0051c639 + iVar15) -
                         (short)RD16(DAT_0051c63d + (unsigned int)DAT_00a08104[4] * 0x101));
      } else if ((SELF_ENT[2] == 0) || (SELF_ENT[2] == 1)) {
        sVar14 = (short)((short)RD16(DAT_0051c639 + iVar15) -
                         (short)RD16(DAT_0051c63f + (unsigned int)DAT_00a08104[4] * 0x101));
      } else {
        sVar14 = (short)((short)RD16(DAT_0051c63b + iVar15) -
                         (short)RD16(DAT_0051c63f + (unsigned int)DAT_00a08104[4] * 0x101));
      }
      FUN_004035d0((int)pbVar9, 0, sVar14, (short)RD16(DAT_0051c637 + iVar15));
      WR32(DAT_00a08104 + 0xb1,
           (RD32(DAT_009a1fe8 + iVar3 + 4) - RD32(SELF_ENT + 4)) + (unsigned int)DAT_009d8a58);
      local_48 = (int)RD32(DAT_00a08104 + 0xb1);
      WR32(DAT_00a08104 + 0xb5,
           (RD32(DAT_009a1fe8 + iVar3 + 8) - RD32(SELF_ENT + 8)) + (unsigned int)DAT_009d8a54);
      iVar15 = (int)RD32(DAT_00a08104 + 0xb5);
      WR32(DAT_00a08104 + 0xb9,
           (RD32(DAT_009a1fe8 + iVar3 + 0xc) - RD32(SELF_ENT + 0xc)) + DAT_009d8b60);
      iVar18 = (int)RD32(DAT_00a08104 + 0xb9);
      sStack_4c = (short)RD16(DAT_009a1fe8 + iVar3 + 0x14);
      sVar14 = (short)RD16(DAT_009a1fe8 + iVar3 + 0x10);
      local_40 = RD16(DAT_009a1fe8 + iVar3 + 0x12);
    }
    if ((short)RD16(SELF_ENT + 0x14) != sStack_4c) {
      DAT_00a08104[0xa4] = 1;
      pbVar10 = DAT_00a08104;
      pbVar10[0xa5] = 0;
      pbVar10[0xa6] = 0x80;
      *(short *)(SB + 0x30) = sStack_4c;
    }
    local_38 = (char *)(unsigned int)DAT_009a73b0;
    if ((short)RD16(DAT_009a1fe8 + (unsigned int)local_38 * 0x23 + 0x10) != sVar14) {
      DAT_00a08104[0xab] = 2;
      pbVar10 = DAT_00a08104;
      pbVar10[0xac] = 0;
      pbVar10[0xad] = 0x80;
      *(short *)(SB + 0x32) = sVar14;
    }
    if ((short)RD16(SELF_ENT + 0x12) != (short)RD16(DAT_009a1fe8 + iVar3 + 0x12)) {
      *(unsigned short *)(SB + 0x2e) = local_40;
      pbVar10 = DAT_00a08104;
      pbVar10[0x9e] = 0;
      pbVar10[0x9f] = 0x80;
      DAT_00a08104[0x9d] = 0;
      if (RD16(DAT_00a08104 + 0xd) < *(unsigned short *)(SB + 0x2e)) {
        DAT_00a08104[0x9c] = 2;
      } else {
        DAT_00a08104[0x9c] = 1;
      }
    }
    if (local_48 < 0) {
      local_48 = (int)(0u - (unsigned int)local_48);
    }
    if (iVar15 < 0) {
      iVar15 = (int)(0u - (unsigned int)iVar15);
    }
    if (iVar18 < 0) {
      iVar18 = (int)(0u - (unsigned int)iVar18);
    }
    pbVar9 = (unsigned char *)(int)((unsigned int)iVar15 + (unsigned int)iVar18 + (unsigned int)local_48);
    if (0xf < (int)pbVar9) break;
    pbVar10 = DAT_00a08104;
    pbVar10[0xb1] = 0;
    pbVar10[0xb2] = 0;
    pbVar10[0xb3] = 0;
    pbVar10[0xb4] = 0;
    pbVar10 = DAT_00a08104;
    pbVar10[0xb5] = 0;
    pbVar10[0xb6] = 0;
    pbVar10[0xb7] = 0;
    pbVar10[0xb8] = 0;
    pbVar10 = DAT_00a08104;
    pbVar10[0xb9] = 0;
    pbVar10[0xba] = 0;
    pbVar10[0xbb] = 0;
    pbVar10[0xbc] = 0;
    SB[0x35] = 2;
    WR32(SB + 0x36, (unsigned int)uVar13 * 0x49c);
    pbVar10 = SB;
    pbVar10[0x3a] = 0xec;
    pbVar10[0x3b] = 0;
    if ((RD32(SELF_ENT + 0x1f) != 0) && (DAT_00a08104[0x7c] == 0)) {
      DAT_00a08104[0x7c] = 1;
      uVar11 = (unsigned int)DAT_009a73c0;
      iVar15 = (int)(uVar11 * 0x126);
      *(short *)(DAT_009a20fe + iVar15) = (short)(*(short *)(DAT_009a20fe + iVar15) + 1);
      if (DAT_009d8c80[uVar11 * 0x566] == DAT_00a08104[0xb0]) {
        DAT_009a211c[iVar15] = 1;
      }
    }
    pbVar10 = DAT_009a1fe8;
    if ((RD32(DAT_009a1fe8 + iVar3 + 0x1f) != 0) &&
        (pbVar10 = (unsigned char *)(((unsigned int)DAT_009a1fe8 & 0xffffff00u) | P19[0x7a]),
         P19[0x7a] == 0)) {
      P19[0x7a] = 1;
      pbVar10 = (unsigned char *)((unsigned int)local_52 * 0x126);
      *(short *)((unsigned int)pbVar10 + 0x9a20fc) =
          (short)(*(short *)((unsigned int)pbVar10 + 0x9a20fc) + 1);
      if (DAT_009d8c80[(unsigned int)local_52 * 0x566] == P19[0xb0]) {
        *(unsigned char *)((unsigned int)pbVar10 + 0x9a211b) = 1;
      }
    }
    FUN_004525a0(CONCAT22((short)((unsigned int)pbVar10 >> 0x10), DAT_009a73b0), 1, DAT_009a1ed0);
    FUN_00451bf0(0, DAT_009a1ed0);
    FUN_004525a0(CONCAT22(sStack_4c, uVar6), 1, DAT_009ec4d0);
    FUN_00451bf0(1, DAT_009ec4d0);
    DAT_00a08292 = (unsigned short)*(unsigned char *)(RD32(SELF_ENT + 0x1f) + 0x88);
    FUN_00450650(0xea, (int)DAT_009ecc34);
    bVar24 = (uVar13 == 2);
    if (uVar13 < 2) goto LAB_004a2a87;
    pcVar25 = (char *)(DAT_009d1320 + (unsigned int)SB[0xb] * 0x55);
    iVar15 = 0xf;
    pcVar20 = pcVar25;
    pcVar22 = s_boardtogivepln_0051a254;
    do {
      if (iVar15 == 0) break;
      iVar15 = iVar15 + -1;
      bVar24 = (*pcVar20 == *pcVar22);
      pcVar20 = pcVar20 + 1;
      pcVar22 = pcVar22 + 1;
    } while (bVar24);
    if (bVar24) goto LAB_004a2a6d;
    iVar15 = 0x13;
    bVar24 = 0;
    pcVar20 = pcVar25;
    pcVar22 = s_boardtoexchangepln_0051a230;
    do {
      if (iVar15 == 0) break;
      iVar15 = iVar15 + -1;
      bVar24 = (*pcVar20 == *pcVar22);
      pcVar20 = pcVar20 + 1;
      pcVar22 = pcVar22 + 1;
    } while (bVar24);
    if (bVar24) goto LAB_004a2a6d;
    iVar15 = 0x12;
    bVar24 = 0;
    pcVar20 = pcVar25;
    pcVar22 = s_boardtocontactpln_0051a208;
    do {
      if (iVar15 == 0) break;
      iVar15 = iVar15 + -1;
      bVar24 = (*pcVar20 == *pcVar22);
      pcVar20 = pcVar20 + 1;
      pcVar22 = pcVar22 + 1;
    } while (bVar24);
    if (bVar24) goto LAB_004a2a6d;
    iVar15 = 0x11;
    bVar24 = 0;
    pcVar20 = pcVar25;
    pcVar22 = s_boardtopickuppln_0051a21c;
    do {
      if (iVar15 == 0) break;
      iVar15 = iVar15 + -1;
      bVar24 = (*pcVar20 == *pcVar22);
      pcVar20 = pcVar20 + 1;
      pcVar22 = pcVar22 + 1;
    } while (bVar24);
    if (bVar24) goto LAB_004a2a6d;
    iVar15 = 0x11;
    bVar24 = 0;
    pcVar20 = pcVar25;
    pcVar22 = s_boardtorepairpln_0051a1f4;
    do {
      if (iVar15 == 0) break;
      iVar15 = iVar15 + -1;
      bVar24 = (*pcVar20 == *pcVar22);
      pcVar20 = pcVar20 + 1;
      pcVar22 = pcVar22 + 1;
    } while (bVar24);
    if (bVar24) goto LAB_004a2a6d;
    iVar15 = 0x12;
    bVar24 = 0;
    pcVar20 = pcVar25;
    pcVar22 = s_boardtocapturepln_0051a278;
    do {
      if (iVar15 == 0) break;
      iVar15 = iVar15 + -1;
      bVar24 = (*pcVar20 == *pcVar22);
      pcVar20 = pcVar20 + 1;
      pcVar22 = pcVar22 + 1;
    } while (bVar24);
    if (!bVar24) {
      iVar15 = 0xf;
      pcVar20 = pcVar25;
      pcVar22 = s_boardtotakepln_0051a244;
      do {
        if (iVar15 == 0) break;
        iVar15 = iVar15 + -1;
        bVar24 = (*pcVar20 == *pcVar22);
        pcVar20 = pcVar20 + 1;
        pcVar22 = pcVar22 + 1;
      } while (bVar24);
      if (!bVar24) {
        iVar15 = 0x12;
        pcVar20 = s_boardtodestroypln_0051a264;
        do {
          if (iVar15 == 0) break;
          iVar15 = iVar15 + -1;
          bVar24 = (*pcVar25 == *pcVar20);
          pcVar25 = pcVar25 + 1;
          pcVar20 = pcVar20 + 1;
        } while (bVar24);
        if (!bVar24) goto LAB_004a2a87;
      }
    }
    iVar15 = 0x27;
    goto LAB_004a2a80;
LAB_004a2a6d:
    iVar15 = 0x25;
LAB_004a2a80:
    FUN_00430200(1, iVar15, (unsigned int)DAT_009a73b0, 0xffff);
LAB_004a2a87:
    uVar12 = FUN_00430200(1, 0x1e, uVar12, 0xffff);
    return uVar12 & 0xffff0000;

  case 2:
    bVar24 = (RD32(SB + 0x36) == 0);
    if (!bVar24) {
      iVar15 = 0xf;
      pcVar25 = (char *)(DAT_009d1320 + (unsigned int)SB[0xb] * 0x55);
      pcVar20 = s_boardtogivepln_0051a254;
      do {
        if (iVar15 == 0) break;
        iVar15 = iVar15 + -1;
        bVar24 = (*pcVar25 == *pcVar20);
        pcVar25 = pcVar25 + 1;
        pcVar20 = pcVar20 + 1;
      } while (bVar24);
      if ((bVar24) && (RD32(pbVar9 + 0x1b) != 0xffffffffu) &&
          (pbVar9 = SB, *(short *)(SB + 0x3a) == 0)) {
        uVar12 = 0;
        bVar24 = 0;
        bVar5 = 0;
        if (P19[0x232] != 0) {
          do {
            bVar24 = bVar5;
            if (P19[uVar12 + 0x233] != 0) {
              iVar15 = (int)((unsigned int)uStack_50 * 0x101 + uVar12);
              bVar17 = DAT_0051c5ac[uVar12 + (unsigned int)uStack_50 * 0x101];
              for (uVar6 = (unsigned short)DAT_0051c5aa[iVar15]; uVar6 <= bVar17;
                   uVar6 = (unsigned short)(uVar6 + 1)) {
                uVar11 = (unsigned int)DAT_009d8c85[(unsigned int)DAT_009a1fe8[iVar3 + 0x16] * 0x566];
                if ((short)uVar12 == 1) {
                  uVar11 = 5;
                }
                uVar11 = FUN_00425b70(
                    ((((((unsigned int)DAT_009a1fe8[iVar3 + 0x16] * 0x2b3u) >> 8) & 0xffffffu) << 8) |
                     DAT_0051c5b0[iVar15]) & 0xffff00ffu,
                    *(unsigned short *)(DAT_00524208 + uVar11 * 2));
                if ((short)uVar11 == 0) {
                  uVar11 = 1;
                }
                if (DAT_009d8c84[(unsigned int)DAT_009a1fe8[iVar3 + 0x16] * 0x566] == 1) {
                  uVar11 = (unsigned int)(unsigned short)((unsigned short)uVar11 * 2);
                } else if (DAT_009d8c84[(unsigned int)DAT_009a1fe8[iVar3 + 0x16] * 0x566] == 2) {
                  uVar11 = (unsigned int)(unsigned short)((unsigned short)uVar11 >> 1);
                }
                if ((short)uVar11 == 0) {
                  uVar11 = 1;
                }
                if (99 < (unsigned short)uVar11) {
                  uVar11 = 99;
                }
                uVar4 = (unsigned int)uVar6;
                bVar1 = P19[uVar4 * 4 + 0x372];
                local_38 = (char *)(unsigned int)bVar1;
                if ((unsigned int)local_38 < (uVar11 & 0xffff)) {
                  bVar24 = 1;
                  P19[uVar4 * 4 + 0x372] = (unsigned char)(bVar1 + 1);
                }
                P19[uVar4 * 4 + 0x371] = 0x7f;
              }
            }
            uVar12 = (unsigned int)(unsigned short)((short)uVar12 + 1);
            bVar5 = bVar24;
          } while (uVar12 < P19[0x232]);
        }
        if (P19[0x246] != 0) {
          P19[0x247] = DAT_0051c632[(unsigned int)uStack_50 * 0x101];
        }
        uVar6 = 1;
        uVar13 = 0;
        do {
          if (((uVar6 & RD16(P19 + 0x1e3)) != 0) && ((RD16(P19 + 0x1e5) & uVar6) == 0)) {
            bVar24 = 1;
            WR16(P19 + 0x1e5, RD16(P19 + 0x1e5) | uVar6);
            break;
          }
          uVar6 = (unsigned short)(uVar6 * 2);
          uVar13 = (unsigned short)(uVar13 + 1);
        } while (uVar13 < 10);
        pbVar9 = SB;
        pbVar10 = SB;
        pbVar10[0x3a] = 0xd8;
        pbVar8 = SB;
        pbVar10[0x3b] = 1;
        if (bVar24) {
          pbVar10 = SB;
          pbVar10[0x36] = 0x88;
          pbVar10[0x37] = 5;
          pbVar10[0x38] = 0;
          pbVar10[0x39] = 0;
          return (unsigned int)pbVar8 & 0xffff0000;
        }
      }
      break;
    }
    local_38 = (char *)(DAT_009d1320 + (unsigned int)SB[0xb] * 0x55);
    iVar15 = 0xf;
    pcVar25 = local_38;
    pcVar20 = s_boardtogivepln_0051a254;
    do {
      if (iVar15 == 0) break;
      iVar15 = iVar15 + -1;
      bVar24 = (*pcVar25 == *pcVar20);
      pcVar25 = pcVar25 + 1;
      pcVar20 = pcVar20 + 1;
    } while (bVar24);
    if (bVar24) {
      if (RD32(pbVar10) != 0) {
        uVar13 = 0;
        do {
          uVar7 = (unsigned short)(uVar13 + 1);
          P19[uVar13 + 0x201] = DAT_00a08104[uVar13 + 0x201];
          uVar13 = uVar7;
        } while (uVar7 < 0x10);
        P19[0x200] = 2;
      }
      local_38 = (char *)(unsigned int)DAT_009d8cd5[(unsigned int)DAT_009a73c0 * 0x566 +
                                                    (unsigned int)*SB * 0x52];
      if ((unsigned int)local_38 <= (unsigned int)(SB[*SB + 5] + 1)) {
        DAT_00a08104[0x201] = 0;
      }
      DAT_00a08104[0x200] = 1;
      pbVar10 = DAT_009a1fe8;
      if (RD32(DAT_009a1fe8 + iVar3 + 0x1b) != 0xffffffffu) {
        pbVar10 = DAT_009ecc34;
        if (RD32(DAT_009a1fe8 + iVar3 + 0x1b) == (unsigned int)DAT_009ecc34) {
          pbVar10 = (unsigned char *)((unsigned int)DAT_009ecc34 * 0x16f);
          iVar15 = (int)RD32(DAT_009e9670 + (unsigned int)DAT_009ecc34 * 0x5bd);
          if (iVar15 == -1) {
            bVar24 = 0;
          } else {
            pbVar10 = (unsigned char *)((unsigned int)iVar15 * 7);
            bVar17 = DAT_009a1fe8[(unsigned int)iVar15 * 0x23 + 3];
            if (((bVar17 != 1) && (bVar17 != 2)) &&
                ((bVar17 != 3 && ((bVar17 != 0xe && (bVar17 != 4)))))) {
              bVar24 = 0;
            } else {
              bVar24 = 1;
            }
          }
          if (bVar24) {
            uVar13 = 1;
            sVar14 = 0xd;
            do {
              if (((RD16(P19 + 0x1df) & uVar13) != 0) && ((uVar13 & RD16(P19 + 0x1e1)) == 0)) {
                WR16(P19 + 0x1e1, RD16(P19 + 0x1e1) | uVar13);
              }
              uVar13 = (unsigned short)(uVar13 * 2);
              sVar14 = (short)(sVar14 - 1);
            } while (sVar14 != 0);
            FUN_004aba50();
            FUN_00440da0((unsigned int)DAT_009e9a77[(unsigned int)DAT_009ecc34 * 0x5bd],
                         (int)DAT_009ecc34);
            FUN_004abe50();
            pbVar10 = extraout_ECX;
          }
        }
        pbVar10 = (unsigned char *)((unsigned int)pbVar10 & 0xffff0000);
        do {
          uVar12 = (unsigned int)pbVar10 & 0xffff;
          P19[uVar12 + 0x2a1] = (unsigned char)(unsigned int)pbVar10;
          uVar13 = (unsigned short)((short)(unsigned int)pbVar10 + 1);
          pbVar10 = (unsigned char *)CONCAT22((short)((unsigned int)pbVar10 >> 0x10), uVar13);
          *(unsigned short *)(P19 + uVar12 * 2 + 0x2ac) = 100;
          *(unsigned short *)(P19 + uVar12 * 2 + 0x2c2) = 0;
        } while (uVar13 < 10);
      }
      FUN_00451c20(CONCAT22((short)((unsigned int)pbVar10 >> 0x10), DAT_009a73b0),
                   (unsigned int *)DAT_00a08104, 0x9c);
      uVar12 = (unsigned int)DAT_009a73b0;
      iVar15 = 0x26;
LAB_004a3387:
      FUN_00430200(1, iVar15, uVar12, 0xffff);
    } else {
      iVar15 = 0xf;
      pcVar25 = local_38;
      pcVar20 = s_boardtotakepln_0051a244;
      do {
        if (iVar15 == 0) break;
        iVar15 = iVar15 + -1;
        bVar24 = (*pcVar25 == *pcVar20);
        pcVar25 = pcVar25 + 1;
        pcVar20 = pcVar20 + 1;
      } while (bVar24);
      uVar16 = (unsigned short)((unsigned int)iVar15 >> 0x10);
      if (bVar24) {
        if (RD32(pbVar10) != 0) {
          uVar13 = 0;
          do {
            uVar7 = (unsigned short)(uVar13 + 1);
            bVar17 = P19[uVar13 + 0x201];
            uVar16 = (unsigned short)((unsigned int)iVar15 >> 0x10);
            DAT_00a08104[uVar13 + 0x201] = bVar17;
            iVar15 = (int)((((unsigned int)iVar15 >> 8) << 8) | bVar17);
            uVar13 = uVar7;
          } while (uVar7 < 0x10);
          P19[0x201] = 0;
          P19[0x200] = 1;
        }
        DAT_00a08104[0x200] = 2;
        FUN_00451c20(CONCAT22(uVar16, DAT_009a73b0), (unsigned int *)DAT_00a08104, 0x9c);
        uVar12 = (unsigned int)DAT_009a73b0;
        iVar15 = 0x28;
        goto LAB_004a3387;
      }
      iVar15 = 0x13;
      bVar24 = 0;
      pcVar25 = local_38;
      pcVar20 = s_boardtoexchangepln_0051a230;
      do {
        if (iVar15 == 0) break;
        iVar15 = iVar15 + -1;
        bVar24 = (*pcVar25 == *pcVar20);
        pcVar25 = pcVar25 + 1;
        pcVar20 = pcVar20 + 1;
      } while (bVar24);
      uVar16 = (unsigned short)((unsigned int)iVar15 >> 0x10);
      if (bVar24) {
        if (RD32(pbVar10) != 0) {
          uVar13 = 0;
          do {
            uVar7 = (unsigned short)(uVar13 + 1);
            uVar12 = (unsigned int)uVar13;
            bVar17 = P19[uVar12 + 0x201];
            uVar16 = (unsigned short)((unsigned int)iVar15 >> 0x10);
            bVar1 = DAT_00a08104[uVar12 + 0x201];
            DAT_00a08104[uVar12 + 0x201] = bVar17;
            P19[uVar12 + 0x201] = bVar1;
            iVar15 = (int)((((unsigned int)iVar15 >> 8) << 8) | bVar17);
            uVar13 = uVar7;
          } while (uVar7 < 0x10);
          P19[0x200] = 2;
        }
        DAT_00a08104[0x200] = 2;
        FUN_00451c20(CONCAT22(uVar16, DAT_009a73b0), (unsigned int *)DAT_00a08104, 0x9c);
      } else {
        iVar15 = 0x12;
        pcVar25 = local_38;
        pcVar20 = s_boardtocapturepln_0051a278;
        do {
          if (iVar15 == 0) break;
          iVar15 = iVar15 + -1;
          bVar24 = (*pcVar25 == *pcVar20);
          pcVar25 = pcVar25 + 1;
          pcVar20 = pcVar20 + 1;
        } while (bVar24);
        if (!bVar24) {
          iVar15 = 0x12;
          pcVar25 = local_38;
          pcVar20 = s_boardtodestroypln_0051a264;
          do {
            if (iVar15 == 0) break;
            iVar15 = iVar15 + -1;
            bVar24 = (*pcVar25 == *pcVar20);
            pcVar25 = pcVar25 + 1;
            pcVar20 = pcVar20 + 1;
          } while (bVar24);
          if (bVar24) {
            if (RD32(pbVar10) != 0) {
              FUN_00434500(CONCAT22((short)(RD32(pbVar10) >> 0x10), DAT_009a73b0),
                           CONCAT22(sStack_4c, uVar6));
              iVar15 = (int)RD32(DAT_009a1fe8 + iVar3 + 0x1f);
              if ((short)RD16((unsigned char *)iVar15 + 0x81) != 0) {
                uVar13 = (unsigned short)FUN_00425cb0();
                WR16((unsigned char *)iVar15 + 0x81, ((uVar13 & 0xf) + 0xf) * 0xec);
              }
              iVar15 = FUN_0046b610(s_selfdestroypln_00521210);
              *(char *)(local_3c + 10) = (char)iVar15;
              pbVar10 = DAT_00a08104;
              __builtin_memcpy(u_38.buf, BLK, 0x37);
              DAT_00a08104 = (unsigned char *)puVar19;
              FUN_00402cb0(uVar6);
              FUN_00402970(uVar12);
              DAT_00a08104 = pbVar10;
              __builtin_memcpy(BLK, u_38.buf, 0x37);
            }
            uVar12 = (unsigned int)DAT_009a73b0;
            iVar15 = 0x28;
          } else {
            iVar15 = 0x11;
            pcVar25 = local_38;
            pcVar20 = s_boardtopickuppln_0051a21c;
            do {
              if (iVar15 == 0) break;
              iVar15 = iVar15 + -1;
              bVar24 = (*pcVar25 == *pcVar20);
              pcVar25 = pcVar25 + 1;
              pcVar20 = pcVar20 + 1;
            } while (bVar24);
            if (bVar24) {
              if (RD32(pbVar10) == 0) {
                *(short *)(DAT_009a20d6 + (unsigned int)pbVar9[0x16] * 0x126) =
                    (short)(*(short *)(DAT_009a20d6 + (unsigned int)pbVar9[0x16] * 0x126) + 1);
                pbVar9[3] = 0;
              } else {
                FUN_004a3750(uVar12, (int)puVar19, 0x80);
                *(unsigned short *)(DAT_00a08104 + 0x66) = uVar6;
                *(unsigned short *)(puVar19 + 0x1a) = DAT_009a73b0;
                WR16(P19 + 0x1e5, 0);
              }
              uVar12 = (unsigned int)DAT_009a73b0;
              iVar15 = 0x26;
            } else {
              iVar15 = 0x12;
              pcVar25 = local_38;
              pcVar20 = s_boardtocontactpln_0051a208;
              do {
                if (iVar15 == 0) break;
                iVar15 = iVar15 + -1;
                bVar24 = (*pcVar25 == *pcVar20);
                pcVar25 = pcVar25 + 1;
                pcVar20 = pcVar20 + 1;
              } while (bVar24);
              if (bVar24) {
                if (RD32(pbVar10) != 0) {
                  P19[0x200] = 2;
                }
                FUN_00451c20(CONCAT22((short)((unsigned int)iVar15 >> 0x10), DAT_009a73b0),
                             (unsigned int *)DAT_00a08104, 0x9c);
                uVar12 = (unsigned int)DAT_009a73b0;
                iVar15 = 0x28;
              } else {
                iVar15 = 0x11;
                pcVar25 = local_38;
                pcVar20 = s_boardtorepairpln_0051a1f4;
                do {
                  if (iVar15 == 0) break;
                  iVar15 = iVar15 + -1;
                  bVar24 = (*pcVar25 == *pcVar20);
                  pcVar25 = pcVar25 + 1;
                  pcVar20 = pcVar20 + 1;
                } while (bVar24);
                if (!bVar24) goto LAB_004a3391;
                uVar16 = 0;
                if (RD32(pbVar10) != 0) {
                  uVar16 = (unsigned short)(RD32(pbVar10) >> 0x10);
                  WR16(P19 + 0x1e5, RD16(P19 + 0x1e3));
                  WR16(P19 + 0xcf, 0);
                  P19[7] = 0;
                  P19[0x200] = 3;
                }
                FUN_00451c20(CONCAT22(uVar16, DAT_009a73b0), (unsigned int *)DAT_00a08104, 0x9c);
                FUN_00430200(1, 0x28, (unsigned int)DAT_009a73b0, 0xffff);
                iVar15 = 0x1d;
              }
            }
          }
          goto LAB_004a3387;
        }
        if (RD32(pbVar10) != 0) {
          FUN_004a3750(uVar12, (int)puVar19, 0x80);
          if (P19[0x1eb] == 0) {
            if ((char)P19[5] == -1) {
              bVar17 = DAT_00524120[DAT_009d8cd2[(unsigned int)local_52 * 0x566]];
            } else {
              bVar17 = DAT_00524148[DAT_009d8cd2[(unsigned int)local_52 * 0x566]];
            }
            *(unsigned char *)(local_3c + 10) = DAT_009a7a40[(unsigned int)bVar17];
          } else {
            if (*(short *)(P19 + 0x92) == 0) {
              pcVar25 = s_stationaryldrpln_0051a1bc;
            } else {
              pcVar25 = s_flyhomeevadepln_00521628;
            }
            iVar15 = FUN_0046b610(pcVar25);
            *(char *)(local_3c + 10) = (char)iVar15;
            FUN_00451c20(CONCAT22(sStack_4c, uVar6), puVar19, 0x8b);
          }
          FUN_00430200(1, 0x28, (unsigned int)DAT_009a73b0, 0xffff);
          FUN_00430200(1, 0x1f, uVar12, 0xffff);
          pbVar10 = DAT_00a08104;
          __builtin_memcpy(u_38.buf, BLK, 0x37);
          DAT_00a08104 = (unsigned char *)puVar19;
          FUN_00402cb0(uVar6);
          FUN_00402970(uVar12);
          DAT_00a08104 = pbVar10;
          __builtin_memcpy(BLK, u_38.buf, 0x37);
        }
      }
    }
LAB_004a3391:
    uVar13 = 0;
    if (RD32(DAT_009a1fe8 + iVar3 + 0x1f) != 0) {
      if (*(char *)(RD32(SELF_ENT + 0x1f) + 0x88) ==
          *(char *)(RD32(DAT_009a1fe8 + iVar3 + 0x1f) + 0x88)) {
        uVar12 = (unsigned int)*(unsigned char *)(RD32(SELF_ENT + 0x1f) + 0x89);
        if (*(char *)(uVar12 + 0x1f6 + (int)puVar19) == 0) {
          bVar17 = 0;
          do {
            bVar1 = *(unsigned char *)((unsigned int)uVar13 + 0x1f6 + (int)puVar19);
            if (bVar17 < bVar1) {
              bVar17 = bVar1;
            }
            uVar13 = (unsigned short)(uVar13 + 1);
          } while (uVar13 < 10);
          *(unsigned char *)(uVar12 + 0x1f6 + (int)puVar19) = (unsigned char)(bVar17 + 1);
          pbVar10 = DAT_009a1fe8 + iVar3 + 0x16;
          *(short *)(DAT_009a20da + (unsigned int)DAT_009a1fe8[iVar3 + 0x16] * 0x126) =
              (short)(*(short *)(DAT_009a20da + (unsigned int)DAT_009a1fe8[iVar3 + 0x16] * 0x126) + 1);
          if (DAT_009d8c80[(unsigned int)*pbVar10 * 0x566] == P19[0xb0]) {
            DAT_009a210a[(unsigned int)*pbVar10 * 0x126] = 1;
          }
        }
      } else {
        FUN_0042e4d0(0x24, -1, (int)DAT_009ecc34);
      }
    }
    SB[*SB + 5] = (unsigned char)(SB[*SB + 5] + 1);
    *(unsigned short *)(DAT_00a08104 + (unsigned int)DAT_00a08104[0x7d] * 2 + 0x7e) = uVar2;
    DAT_00a08104[0x7d] = (unsigned char)(DAT_00a08104[0x7d] + 1);
    if (9 < DAT_00a08104[0x7d]) {
      DAT_00a08104[0x7d] = (unsigned char)(DAT_00a08104[0x7d] - 1);
    }
    if (DAT_00a08104[0x7d] == 1) {
      uVar12 = (unsigned int)DAT_009a73c0;
      iVar15 = (int)(uVar12 * 0x126);
      *(short *)(DAT_009a20e2 + iVar15) = (short)(*(short *)(DAT_009a20e2 + iVar15) + 1);
      if (DAT_009d8c80[uVar12 * 0x566] == DAT_00a08104[0xb0]) {
        DAT_009a210e[iVar15] = 1;
      }
    }
    if (RD32(DAT_009a1fe8 + iVar3 + 0x1f) != 0) {
      P19[0x7b] = (unsigned char)(P19[0x7b] + 1);
      iVar15 = (int)((unsigned int)local_52 * 0x126);
      *(short *)(DAT_009a20de + iVar15) = (short)(*(short *)(DAT_009a20de + iVar15) + 1);
      if (DAT_009d8c80[(unsigned int)local_52 * 0x566] == P19[0xb0]) {
        DAT_009a210c[iVar15] = 1;
      }
    }
    SB[0x35] = 3;
    pbVar10 = SB;
    pbVar10[0x36] = 0x38;
    pbVar8 = SB;
    pbVar10[0x37] = 9;
    pbVar10[0x38] = 0;
    pbVar10[0x39] = 0;
    if (RD16(DAT_009e968a + (unsigned int)DAT_009ecc34 * 0x5bd) == uVar6) {
      DAT_00a08c74 = 0xfffd;
    }
    pbVar9 = DAT_009a1fe8;
    if (RD32(DAT_009a1fe8 + iVar3 + 0x1b) != 0xffffffffu) {
      pbVar10 = SB;
      pbVar10[0x2b] = 0xff;
      pbVar10[0x2c] = 0xff;
      return (unsigned int)pbVar8 & 0xffff0000;
    }
    break;

  case 3:
    if (RD32(SB + 0x36) == 0) {
      pbVar8[0] = 0xff;
      pbVar8[1] = 0xff;
      pbVar10 = SB;
      pbVar10[0x1a] = 0;
      pbVar10[0x1b] = 0;
      SB[0x1e] = 0;
      return CONCAT22(uStack_3e, 1);
    }
    if (iVar15 != 0) {
      FUN_004035d0((int)SELF_ENT, 0, 0x4000, 0);
      WR32(DAT_00a08104 + 0xb1,
           (RD32(DAT_009a1fe8 + iVar3 + 4) - RD32(SELF_ENT + 4)) + (unsigned int)DAT_009d8a58);
      WR32(DAT_00a08104 + 0xb5,
           (RD32(DAT_009a1fe8 + iVar3 + 8) - RD32(SELF_ENT + 8)) + (unsigned int)DAT_009d8a54);
      uVar12 = DAT_009d8b60 & 0xffff0000;
      WR32(DAT_00a08104 + 0xb9,
           (RD32(DAT_009a1fe8 + iVar3 + 0xc) - RD32(SELF_ENT + 0xc)) + DAT_009d8b60);
      return uVar12;
    }
    pbVar10 = DAT_00a08104;
    pbVar10[0xb1] = 0;
    pbVar10[0xb2] = 0;
    pbVar10[0xb3] = 0;
    pbVar10[0xb4] = 0;
    pbVar10 = DAT_00a08104;
    pbVar10[0xb5] = 0;
    pbVar9 = DAT_00a08104;
    pbVar10[0xb6] = 0;
    pbVar10[0xb7] = 0;
    pbVar10[0xb8] = 0;
    pbVar10 = DAT_00a08104;
    pbVar10[0xb9] = 0xf4;
    pbVar10[0xba] = 1;
    pbVar10[0xbb] = 0;
    pbVar10[0xbc] = 0;
  }
  return (unsigned int)pbVar9 & 0xffff0000;
}
