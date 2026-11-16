/* SPDX-License-Identifier: MIT */
/* z_bop 004065d0 */
/* Packed-access helpers (no function bodies; macros only). Offsets inside the
   0x23-byte records and the structs they point to are unaligned, so every
   access goes through __builtin_memcpy. */
#define LD(T, p) ({ T t_; __builtin_memcpy(&t_, (p), sizeof(T)); t_; })
#define ST(T, p, v) do { T t_ = (T)(v); __builtin_memcpy((p), &t_, sizeof(T)); } while (0)
#define LDP(p) LD(unsigned char *, (p))

extern unsigned char *DAT_009a1fe8;
extern unsigned char *DAT_009a8e18;
extern unsigned int DAT_009e9640;
extern unsigned short DAT_00a082d8;
extern int DAT_009d8a58;
extern int DAT_009d8a54;
extern int DAT_009d8b60;
extern unsigned char DAT_0051a44f[];
extern unsigned char DAT_0051a194[];
extern unsigned char DAT_0051c5b2[];
extern unsigned char DAT_0051c5b4[];
extern unsigned char DAT_0051c5b6[];
extern unsigned char DAT_009e9bfc[];
extern char s_starshipprotectpln_0051a2fc[];
extern char s_formevadeldr1pln_0051a394[];

unsigned short FUN_00459750(int param_1);
void FUN_0041bb00(unsigned short param_1);
unsigned int FUN_004269e0(unsigned int param_1);
short FUN_00405860(int param_1);
void FUN_004035d0(int param_1, short param_2, short param_3, short param_4);

/* ND(): data block of the new record (index iVar3); SD(): data block of the source record */
#define ND() LDP(DAT_009a1fe8 + 0x1f + iVar3)
#define SD() LDP(iVar16 + 0x1f)

unsigned int __cdecl FUN_004065d0(int param_1, int param_2, int param_3)
{
  unsigned char bVar1;
  char cVar2;
  int iVar3;
  long long lVar4;
  long long lVar5;
  long long lVar6;
  short sVar7;
  unsigned int uVar8;
  unsigned int uVar9;
  int iVar10;
  int iVar11;
  unsigned short uVar12;
  unsigned int *puVar13;
  int iVar14;
  unsigned short uVar15;
  unsigned char *iVar16;
  int iVar17;
  int local_10;
  unsigned char uStack_a;

  iVar3 = LD(int, DAT_009a1fe8 + 0x1b + param_1 * 0x23);
  if (iVar3 == -1) {
    uStack_a = 7;
    uVar15 = FUN_00459750(7);
LAB_0040673d:
    if (uVar15 == 0xffff) {
      return 0xffffffff;
    }
    iVar16 = param_1 * 0x23 + DAT_009a1fe8;
    bVar1 = iVar16[3];
    uVar8 = (unsigned int)uVar15;
    iVar3 = (int)(uVar8 * 0x23);
    *ND() = 1;
    DAT_009a1fe8[2 + iVar3] = uStack_a;
    DAT_009a1fe8[3 + iVar3] = (unsigned char)param_3;
    ST(short, ND() + 0x83, 1);
    ST(short, ND() + 0x85, (short)param_1);
    *(ND() + 0x87) = bVar1;
    uVar9 = FUN_004269e0((unsigned int)bVar1);
    *(ND() + 0x88) = *(SD() + 0x88);
    ST(short, DAT_009a1fe8 + 0x12 + iVar3, LD(short, iVar16 + 0x12));
    ST(short, DAT_009a1fe8 + 0x14 + iVar3, LD(short, iVar16 + 0x14));
    ST(short, DAT_009a1fe8 + 0x10 + iVar3, LD(short, iVar16 + 0x10));
    ST(short, ND() + 0x79,
       (short)(LD(short, SD() + 0x79) +
               LD(short, s_starshipprotectpln_0051a2fc + param_3 * 2 + 10)));
    ST(short, DAT_009a8e18 + 8 + (uVar8 - DAT_009e9640) * 10, LD(short, ND() + 0x79));
    ST(unsigned int, ND() + 0x7d,
       (unsigned int)LD(unsigned short, SD() + 0x79) +
       LD(unsigned int, DAT_0051a194 + param_3 * 4));
    puVar13 = (unsigned int *)(ND() + 0x7d);
    if (LD(unsigned int, puVar13) < LD(unsigned int, DAT_0051a194 + param_3 * 4)) {
      ST(unsigned int, puVar13, LD(unsigned int, DAT_0051a194 + param_3 * 4));
    }
    sVar7 = FUN_00405860(param_3);
    ST(short, ND() + 0x81, sVar7);
    iVar10 = LD(int, iVar16 + 4);
    iVar11 = LD(int, iVar16 + 8);
    local_10 = LD(int, iVar16 + 0xc);
    iVar14 = (int)((uVar9 & 0xffff) * 0x101);
    sVar7 = LD(short, DAT_0051c5b4 + param_2 * 8 + iVar14);
    iVar14 = iVar14 + param_2 * 8;
    FUN_004035d0((int)iVar16, LD(short, DAT_0051c5b2 + iVar14), sVar7,
                 LD(short, DAT_0051c5b6 + iVar14));
    if (bVar1 == 0x35) {
      DAT_009d8a58 = (int)((unsigned int)DAT_009d8a58 * 2u);
      DAT_009d8a54 = (int)((unsigned int)DAT_009d8a54 * 2u);
      DAT_009d8b60 = (int)((unsigned int)DAT_009d8b60 * 2u);
    }
    iVar10 = (int)((unsigned int)iVar10 + (unsigned int)DAT_009d8a58);
    iVar11 = (int)((unsigned int)iVar11 + (unsigned int)DAT_009d8a54);
    local_10 = (int)((unsigned int)local_10 + (unsigned int)DAT_009d8b60);
    ST(int, ND() + 6, iVar10);
    ST(int, ND() + 10, iVar11);
    ST(int, ND() + 0xe, local_10);
    if (LD(int, iVar16 + 0x1b) != -1) {
      ST(unsigned int, ND() + 2,
         LD(unsigned int, DAT_009e9bfc + (int)((unsigned int)LD(int, iVar16 + 0x1b) * 0x5bdu)));
    }
    if ((DAT_0051a44f[param_3] == '\0') ||
        (((cVar2 = (char)iVar16[2], cVar2 != '\x04') && (cVar2 != '\x03')) && (cVar2 != '\x05'))) {
      iVar14 = (int)LD(short, s_formevadeldr1pln_0051a394 + param_3 * 2 + 2);
      lVar4 = (long long)iVar14 * (long long)(int)LD(short, SD() + 0x93);
      lVar5 = (long long)iVar14 * (long long)(int)LD(short, SD() + 0x95);
      lVar6 = (long long)iVar14 * (long long)(int)LD(short, SD() + 0x97);
      ST(unsigned int, DAT_009a1fe8 + 4 + iVar3,
         (unsigned int)iVar10 +
         ((unsigned int)lVar4 >> 0xf |
          (unsigned int)((unsigned long long)lVar4 >> 0x20) << 0x11));
      ST(unsigned int, DAT_009a1fe8 + 8 + iVar3,
         (unsigned int)iVar11 +
         ((unsigned int)lVar5 >> 0xf |
          (unsigned int)((unsigned long long)lVar5 >> 0x20) << 0x11));
      ST(unsigned int, DAT_009a1fe8 + 0xc + iVar3,
         (unsigned int)local_10 +
         ((unsigned int)lVar6 >> 0xf |
          (unsigned int)((unsigned long long)lVar6 >> 0x20) << 0x11));
      ST(short, ND() + 0x8c, LD(short, SD() + 0x8c));
      ST(short, ND() + 0x8e, LD(short, SD() + 0x8e));
      ST(short, ND() + 0x90, LD(short, SD() + 0x90));
      ST(short, ND() + 0x99, LD(short, SD() + 0x99));
      ST(short, ND() + 0x9b, LD(short, SD() + 0x9b));
      ST(short, ND() + 0x9d, LD(short, SD() + 0x9d));
      ST(short, ND() + 0x9f, LD(short, SD() + 0x9f));
      ST(short, ND() + 0xa1, LD(short, SD() + 0xa1));
      ST(short, ND() + 0xa3, LD(short, SD() + 0xa3));
      ST(short, ND() + 0x93, LD(short, SD() + 0x93));
      ST(short, ND() + 0x95, LD(short, SD() + 0x95));
      ST(short, ND() + 0x97, LD(short, SD() + 0x97));
      *(ND() + 0x92) = 0;
      iVar16 = ND();
      *(iVar16 + 0x8b) = *(iVar16 + 0x92);
    }
    else {
      iVar17 = (int)LD(short, s_formevadeldr1pln_0051a394 + param_3 * 2 + 2);
      if (sVar7 < 0) {
        iVar17 = (int)(0u - (unsigned int)iVar17);
        ST(short, DAT_009a1fe8 + 0x12 + iVar3, (short)0x8000);
      }
      else {
        ST(short, DAT_009a1fe8 + 0x12 + iVar3, 0);
      }
      local_10 = (int)((unsigned int)local_10 + (unsigned int)iVar17);
      *(ND() + 0x92) = 1;
      iVar16 = ND();
      *(iVar16 + 0x8b) = *(iVar16 + 0x92);
      ST(int, DAT_009a1fe8 + 4 + iVar3, iVar10);
      ST(int, DAT_009a1fe8 + 8 + iVar3, iVar11);
      ST(int, DAT_009a1fe8 + 0xc + iVar3, local_10);
    }
    iVar17 = (int)((unsigned int)(unsigned short)(uVar15 - (short)DAT_009e9640) * 10);
    DAT_009a8e18[1 + iVar17] = 0;
    ST(unsigned short, DAT_009a8e18 + 4 + iVar17, 0xffff);
    ST(unsigned short, DAT_009a8e18 + 6 + iVar17, 0);
    ST(unsigned short, DAT_009a8e18 + 2 + iVar17, 0xffff);
    DAT_009a8e18[iVar17] = 0xff;
    ST(unsigned char *, ND() + 0xa5, DAT_009a8e18 + iVar17);
    return uVar8;
  }
  uVar15 = (unsigned short)((short)iVar3 * 0xc + DAT_00a082d8);
  uStack_a = 6;
  uVar12 = (unsigned short)(uVar15 + 0xc);
  if (DAT_0051a44f[param_3] != '\0') {
    uVar15 = (unsigned short)(uVar15 + 8);
  }
  if (uVar15 < uVar12) {
    do {
      if (DAT_009a1fe8[(unsigned int)uVar15 * 0x23 + 3] == '\0') {
        ST(short, LDP(DAT_009a1fe8 + 0x1f + (unsigned int)uVar15 * 0x23) + 0x85, 0);
        *(LDP(DAT_009a1fe8 + 0x1f + (unsigned int)uVar15 * 0x23) + 1) = 0;
        break;
      }
      uVar15 = (unsigned short)(uVar15 + 1);
    } while (uVar15 < uVar12);
    if (uVar15 < uVar12) {
      FUN_0041bb00(uVar15);
      goto LAB_0040673d;
    }
  }
  uVar15 = (unsigned short)(DAT_00a082d8 + 0x60);
  uVar12 = (unsigned short)(DAT_00a082d8 + 0x80);
  if (uVar15 < uVar12) {
    do {
      if (DAT_009a1fe8[(unsigned int)uVar15 * 0x23 + 3] == '\0') {
        ST(short, LDP(DAT_009a1fe8 + 0x1f + (unsigned int)uVar15 * 0x23) + 0x85, 0);
        *(LDP(DAT_009a1fe8 + 0x1f + (unsigned int)uVar15 * 0x23) + 1) = 0;
        break;
      }
      uVar15 = (unsigned short)(uVar15 + 1);
    } while (uVar15 < uVar12);
    if (uVar15 < uVar12) {
      FUN_0041bb00(uVar15);
      goto LAB_0040673d;
    }
  }
  return 0xffffffff;
}
