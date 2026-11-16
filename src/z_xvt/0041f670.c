/* SPDX-License-Identifier: MIT */
/* z_xvt 0041f670 */
/* Plain C (C99 plus GNU statement expressions and __builtin_memcpy for the
 * unaligned accesses). Each LD/ST accessor owns its own block-scoped
 * temporary, so several accessors in one expression cannot interfere.
 * Unresolved: types of the DAT_ globals are inferred from access widths only.
 * _DAT_007394d0 is read as a 32-bit value by Ghidra and overlaps the byte DAT_007394d0;
 * both are kept. Callee return types are not known; prototypes use the
 * Ghidra-inferred argument lists. The 3-short arithmetic wraps at 16 bits. */

typedef unsigned char  u8;
typedef unsigned short u16;
typedef unsigned int   u32;

#define LD8(p,o)    (*((unsigned char *)(p) + (o)))
#define ST8(p,o,v)  (*((unsigned char *)(p) + (o)) = (unsigned char)(v))
#define LD16(p,o)   ({ unsigned short t_; __builtin_memcpy(&t_, (unsigned char *)(p) + (o), 2); t_; })
#define ST16(p,o,v) ({ unsigned short s_ = (unsigned short)(v); __builtin_memcpy((unsigned char *)(p) + (o), &s_, 2); })
#define LD32(p,o)   ({ unsigned int t_; __builtin_memcpy(&t_, (unsigned char *)(p) + (o), 4); t_; })
#define ST32(p,o,v) ({ unsigned int s_ = (unsigned int)(v); __builtin_memcpy((unsigned char *)(p) + (o), &s_, 4); })
#define SUBP(p)     ((unsigned char *)LD32((p), 0x1f))

extern u16 DAT_006ccad0, DAT_006ccad2;
extern u16 DAT_006cca90[], DAT_006cca92[];
extern unsigned char *DAT_006d4988;
extern unsigned char *DAT_006db71c;
extern unsigned char *DAT_00732940;
extern unsigned char *DAT_00732e54;
extern u16 DAT_006db6c0;
extern u8 DAT_007394c4, DAT_007394c3, DAT_007394d2, DAT_007394d0, DAT_007394c5, DAT_007394c7;
extern u32 _DAT_007394d0;
extern char DAT_007394c2, DAT_007394cc, DAT_007394ca, DAT_007394cb, DAT_0073abc1;
extern u8 DAT_007394e8[], DAT_007394e4[];
extern u16 DAT_007394c8, DAT_007394ce;
extern u16 DAT_0073b374;
extern u8 DAT_0073b370, DAT_0073b371, DAT_0073b372, DAT_0073abc0;
extern char DAT_00536e8c, DAT_007092c1;
extern int DAT_0073abc4, DAT_0073abc8, DAT_0073abcc;
extern int DAT_0070ab14, DAT_0070ab18, DAT_0070ac20;
extern u32 DAT_0073abd0[];
extern u32 DAT_00709fc4[];
extern u16 DAT_006d4a68[];
extern u8 DAT_006da3e0[], DAT_006da412, DAT_006da414;
extern u8 DAT_005247e8[], DAT_0052686c[], DAT_0052686a[], DAT_00524778[], DAT_00524788[];
extern u8 DAT_00524870[], DAT_00524880[], DAT_00524898[], DAT_005248b8[], DAT_00524798[], DAT_005247c0[];
extern u8 DAT_0052a650[], DAT_0052a7b0[], DAT_0052a948[], DAT_0052aae0[], DAT_0052ac78[];
extern u8 DAT_0052b080[], DAT_0052b081[], DAT_0052b082[], DAT_0052b083[], DAT_0052b087[], DAT_0052b089[];
extern u8 DAT_0052b08d[], DAT_0052b093[], DAT_0052b097[], DAT_0052b099[], DAT_0052b0a8[], DAT_0052b0aa[];
extern u8 DAT_0052b0ac[], DAT_0052b0ae[], DAT_0052b0b0[], DAT_0052b0b4[], DAT_0052b0b6[], DAT_0052b0ba[];
extern u8 DAT_0052b13c[], DAT_0052b157[], DAT_0052b159[], DAT_0052b15b[], DAT_0052b15d[];
extern u8 DAT_0070ad42[], DAT_0070ad45[], DAT_0070ad46[], DAT_0070ad4a[], DAT_0070ad56[], DAT_0070ad40[];
extern u8 DAT_0070ad2c[], DAT_0070ad18[], DAT_0070ad92[], DAT_0070ad93[];
extern u8 DAT_0070b252[], DAT_0070b216[], DAT_0070b210[], DAT_0070b213[];
extern u8 DAT_00703ca0[];
extern char DAT_00524948[], s_stationaryldrpln_00524934[], s_stationaryflwpln_00524920[];
extern char s_disabledpln_00524914[], s_escortldr1pln_00524904[];
extern u8 DAT_0071b730[], DAT_0071b734[], DAT_0071b73e[], DAT_0071b742[], DAT_0071b757[], DAT_0071b758[];
extern u8 DAT_0071b761[], DAT_0071b763[], DAT_0071b765[], DAT_0071b766[], DAT_0071b767[], DAT_0071b768[];
extern u8 DAT_0071b769[], DAT_0071b76a[], DAT_0071b79d[], DAT_0071b7a1[], DAT_0071b7a5[], DAT_0071b7a9[];
extern u8 DAT_0071b7ad[], DAT_0071b7b1[], DAT_0071b7b5[], DAT_0071b7b7[], DAT_0071b7b9[], DAT_0071b7bb[];
extern u8 DAT_0071b7bd[], DAT_0071b7bf[], DAT_0071b7c1[], DAT_0071b7c3[], DAT_0071b7c5[], DAT_0071b825[];
extern u8 DAT_0071b885[], DAT_0071b8e5[], DAT_0071b917[], DAT_0071b949[], DAT_0071b97b[], DAT_0071b987[];
extern u8 DAT_0071b993[], DAT_0071b99f[], DAT_0071b9af[], DAT_0071b9bf[], DAT_0071b9c1[], DAT_0071b9c3[];
extern u8 DAT_0071b9c5[], DAT_0071b9c7[], DAT_0071b9c9[], DAT_0071b9d9[], DAT_0071b9e9[], DAT_0071ba49[];
extern u8 DAT_0071baa9[], DAT_0071badb[];

void FUN_0040f050(u16);
u32  FUN_004a7aa0(u32);
void FUN_0042a230(int, u32, int, short, short, short);
u32  FUN_00478320(u32, u16);
int  FUN_004490f0(u32);
int  FUN_00449790(u32, u32);
u32  FUN_0044a600(u32, u32);
int  FUN_00481380(char *);
u16  FUN_00478470(void);
void FUN_004296b0(u16);
void FUN_00429310(u32);
void FUN_004213b0(int);

u16 __stdcall FUN_0041f670(void)
{
  unsigned char *puVar2;
  unsigned char *puVar32;
  u32 *puVar9;
  unsigned char *puVar1;
  unsigned char *piVar3;
  int iVar5, iVar7, iVar15, iVar18, iVar35;
  u32 uVar6, uVar13, uVar14, uVar24, uVar25, uVar27, uVar28, puVar22, puVar36;
  u16 uVar12, uVar17, uVar20, uVar26, uStack_20, local_24;
  short sVar11, sVar21, sVar23, sVar31;
  u8 bVar10, bVar16, uVar19;
  char cVar4;
  char cStack_1e;
  u8 uStack_1d;
  int bVar8, bVar34;
  int local_14;
  char *pcVar29, *pcVar30, *pcVar33;

  uVar20 = DAT_006ccad0;
  local_14 = 0;
  uVar26 = DAT_006ccad0;
  while ((uVar26 < DAT_006ccad2 && (DAT_006d4988[(u32)uVar26 * 0x23 + 3] != 0))) {
    uVar26 = uVar26 + 1;
  }
  if (DAT_006ccad2 <= uVar26) {
    return 0xffff;
  }
  uVar27 = (u32)uVar26;
  FUN_0040f050(uVar26);
  iVar15 = uVar27 * 0x23;
  puVar32 = DAT_006d4988 + iVar15;
  *SUBP(puVar32) = 7;
  ST8(SUBP(puVar32), 0x86, DAT_007394c4);
  ST8(SUBP(puVar32), 0x87, DAT_007394c3);
  ST16(SUBP(puVar32), 0x81, 0);
  ST16(SUBP(puVar32), 0x7f, 0);
  *(u32 *)(DAT_007394e8 + uVar27 * 0xc) = uVar27 - uVar20;
  puVar2 = DAT_006db71c + (uVar27 - uVar20) * 0x4c;
  ST32(SUBP(puVar32), 0xab, (u32)puVar2);
  ST32(SUBP(puVar32), 0xa3, 0);
  ST32(SUBP(puVar32), 0xa7, 0);
  DAT_006d4988[2 + iVar15] = 0x10;
  DAT_006d4988[3 + iVar15] = 200;
  ST16(DAT_006d4988, iVar15, DAT_006db6c0);
  DAT_006db6c0 = DAT_006db6c0 + 1;
  bVar16 = DAT_005247e8[DAT_0070ad42[(u32)DAT_0073b374 * 0x566]];
  uVar24 = (((((u32)DAT_0073b374 * 0x159u) >> 8) << 8) | bVar16) & 0xffff00ffu;
  bVar10 = (u8)uVar24;
  DAT_007394d2 = bVar10;
  uVar20 = DAT_006cca90[(u32)DAT_0073b370 * 2];
  uVar26 = uVar20;
  while ((uVar26 < DAT_006cca92[(u32)DAT_0073b370 * 2]) &&
         (DAT_006d4988[(u32)uVar26 * 0x23 + 3] != 0)) {
    uVar26 = uVar26 + 1;
  }
  uStack_1d = (u8)(uVar26 >> 8);
  cStack_1e = (char)uVar26;
  if (DAT_006cca92[(u32)DAT_0073b370 * 2] <= uVar26) {
    return 0xffff;
  }
  uVar6 = (u32)uVar26;
  FUN_0040f050(uVar26);
  iVar15 = (int)LD32(DAT_0070b252, (u32)DAT_0073b374 * 0x566);
  if (((iVar15 == -1) || (DAT_0070ad56[(u32)DAT_0073b374 * 0x566] != _DAT_007394d0)) ||
      (DAT_00536e8c == '\0')) {
    bVar8 = 0;
    ST32(DAT_006d4988, 0x1b + uVar6 * 0x23, 0xffffffffu);
  } else {
    ST32(DAT_0071b730, (u32)iVar15 * 0x5bd, uVar6);
    bVar8 = 1;
    ST32(DAT_006d4988, 0x1b + uVar6 * 0x23, iVar15);
    local_14 = iVar15;
  }
  iVar7 = uVar6 * 0x23;
  iVar15 = (int)LD32(DAT_0070b252, (u32)DAT_0073b374 * 0x566);
  iVar18 = uVar6 - uVar20;
  *(int *)(DAT_007394e4 + uVar6 * 0xc) = iVar18;
  iVar18 = (int)((u32)iVar18 * 0x436);
  ST32(SUBP(DAT_006d4988 + iVar7), 0xa7, (u32)DAT_00732940 + (u32)iVar18);
  DAT_00732e54 = DAT_00732940 + iVar18;
  DAT_0073abd0[uVar6] = uVar27;
  ST32(DAT_00732e54, 0x432, (u32)puVar32);
  ST16(DAT_00732e54, 0x384, LD16(puVar32, 0));
  DAT_006d4988[3 + iVar7] = bVar10;
  ST16(DAT_006d4988, iVar7, DAT_006db6c0);
  DAT_006db6c0 = DAT_006db6c0 + 1;
  uVar27 = (u32)bVar16;
  uVar20 = LD16(DAT_0052686c, uVar27 * 0x16);
  if ((short)uVar24 == 0x3a) {
    uVar20 = (u16)(uVar20 << 3);
  }
  if (uVar20 < 0x2000) {
    sVar21 = (short)(uVar20 << 2);
  } else {
    sVar21 = 0x7fff;
  }
  ST16(SUBP(DAT_006d4988 + iVar7), 0x7d, sVar21);
  uVar24 = FUN_004a7aa0(uVar24);
  ST8(DAT_00732e54, 4, uVar24);
  uVar24 = LD32(DAT_00732e54, 4);
  uVar20 = (u16)(u8)uVar24;
  ST8(SUBP(DAT_006d4988 + iVar7), 0x86, DAT_007394c4);
  piVar3 = DAT_006d4988 + 0x1f + iVar7;
  DAT_0073b371 = LD8(LD32(piVar3, 0), 0x86);
  ST8(LD32(piVar3, 0), 0x87, DAT_007394c3);
  DAT_006d4988[2 + iVar7] = DAT_0073b370;
  *SUBP(DAT_006d4988 + iVar7) = DAT_0052686a[uVar27 * 0x16];
  ST16(SUBP(DAT_006d4988 + iVar7), 0x81, 0);
  ST16(SUBP(DAT_006d4988 + iVar7), 0x7f, 0);
  ST32(SUBP(DAT_006d4988 + iVar7), 2, 0);
  ST8(SUBP(DAT_006d4988 + iVar7), 0x88, DAT_0070ad4a[(u32)DAT_0073b374 * 0x566]);
  ST16(SUBP(DAT_006d4988 + iVar7), 0x83, uVar26);
  ST8(SUBP(DAT_006d4988 + iVar7), 0x85, bVar10);
  DAT_006d4988[0x16 + iVar7] = (u8)DAT_0073b374;
  ST8(DAT_00732e54, 5, DAT_007394c2);
  if (DAT_007394c2 == -1) {
    DAT_007394c2 = cStack_1e;
  }
  ST8(DAT_00732e54, 0xae, DAT_0073b372);
  bVar16 = LD8(DAT_00732e54, 0xae);
  ST8(DAT_00732e54, 0xb0, DAT_007394d0);
  uVar13 = LD32(DAT_00732e54, 0xb0);
  uVar12 = (u16)DAT_007394c5;
  if (DAT_007394cc != '\0') {
    uVar12 = 0;
  }
  ST8(DAT_00732e54, 0xaf, uVar12);
  ST32(DAT_00732e54, 0xb9, 0);
  ST32(DAT_00732e54, 0xb5, LD32(DAT_00732e54, 0xb9));
  ST32(DAT_00732e54, 0xb1, LD32(DAT_00732e54, 0xb5));
  iVar18 = (u32)DAT_0073b374 * 0x566;
  if ((DAT_0070b216[iVar18] == 0) || (DAT_0070b210[iVar18] != 0)) {
    ST32(DAT_00732e54, 0, (u32)DAT_006d4a68[(u32)DAT_0073b374 * 0x93]);
    uVar28 = LD32(DAT_00732e54, 0) + 1;
  } else {
    uVar28 = DAT_00709fc4[DAT_0070b216[iVar18]] + 1;
    DAT_00709fc4[DAT_0070b216[iVar18]] = uVar28;
  }
  ST32(DAT_00732e54, 0, uVar28);
  iVar5 = (u32)uVar20 * 0xef;
  sVar21 = (short)LD16(DAT_0052b15d, iVar5);
  sVar11 = (short)(uVar12 + 1);
  iVar18 = ((u32)(u8)uVar13 + (u32)bVar16 * 6) * 2;
  sVar31 = (short)((u32)(int)(short)LD16(DAT_0052b159, iVar5) * (u32)(int)sVar11 *
                   (u32)(int)(short)LD16(DAT_0052a7b0, iVar18));
  uStack_20 = (u16)((u32)(int)sVar11 * (u32)(int)(short)LD16(DAT_0052a948, iVar18) * (u32)(int)sVar21);
  sVar23 = (short)((u32)(int)sVar11 * (u32)(int)(short)LD16(DAT_0052aae0, iVar18) *
                   (u32)(int)(short)LD16(DAT_0052b15b, iVar5));
  if (sVar11 == 1) {
    sVar31 = (short)((u32)(int)sVar31 +
                     (u32)(int)((short)LD16(DAT_0052b159, iVar5) / 2) * (u32)(int)(short)LD16(DAT_0052a7b0, iVar18));
    sVar23 = (short)((u32)(int)sVar23 +
                     (u32)(int)((short)LD16(DAT_0052b15b, iVar5) / 2) * (u32)(int)(short)LD16(DAT_0052aae0, iVar18));
    uStack_20 = (u16)((u32)uStack_20 +
                      (u32)(int)(short)(((int)((u32)(int)sVar21 + (((int)sVar21 >> 31) & 3U))) >> 2) *
                      (u32)(int)(short)LD16(DAT_0052a948, iVar18));
  }
  sVar21 = (short)LD16(DAT_0052ac78, (u32)bVar16 * 2);
  if (sVar21 != 1) {
    sVar31 = sVar31 / sVar21;
    uStack_20 = (u16)((short)uStack_20 / sVar21);
    sVar23 = sVar23 / sVar21;
  }
  if (_DAT_007394d0 == 0) {
    ST32(DAT_006d4988, 4 + iVar7, DAT_0073abc4);
    ST32(DAT_006d4988, 8 + iVar7, DAT_0073abc8);
    ST32(DAT_006d4988, 0xc + iVar7, DAT_0073abcc);
    ST16(DAT_006d4988, 0x10 + iVar7, DAT_007394c8);
    ST16(DAT_006d4988, 0x12 + iVar7, DAT_007394ce);
    ST16(DAT_006d4988, 0x14 + iVar7, 0);
    ST8(SUBP(DAT_006d4988 + iVar7), 0x90, 1);
    iVar18 = (int)LD32(DAT_006d4988, 0x1f + iVar7);
    puVar36 = ((u32)uStack_1d << 24) | ((u32)(u8)cStack_1e << 16) | (u32)uStack_20;
    ST8(iVar18, 0x89, LD8(iVar18, 0x90));
    puVar22 = puVar36;
    iVar35 = (int)(DAT_006d4988 + iVar7);
  } else {
    puVar36 = ((u32)uStack_1d << 24) | ((u32)(u8)cStack_1e << 16) | (u32)uStack_20;
    iVar18 = (int)((u32)LD8(DAT_00732e54, 5) * 0x23 + DAT_006d4988);
    puVar22 = (u32)DAT_00732e54;
    iVar35 = iVar18;
  }
  FUN_0042a230(iVar18, puVar22, iVar35, sVar31, sVar23, (short)puVar36);
  if ((short)LD16(DAT_0052b157, iVar5) != 0) {
    bVar16 = (u8)LD16(DAT_0052b157, iVar5);
    DAT_0070ab18 = (int)((u32)DAT_0070ab18 << (bVar16 & 0x1f));
    DAT_0070ab14 = (int)((u32)DAT_0070ab14 << (bVar16 & 0x1f));
    DAT_0070ac20 = (int)((u32)DAT_0070ac20 << (bVar16 & 0x1f));
  }
  ST32(DAT_006d4988, 4 + iVar7, (u32)DAT_0073abc4 + (u32)DAT_0070ab18);
  ST32(SUBP(DAT_006d4988 + iVar7), 6, LD32(DAT_006d4988, iVar7 + 4));
  ST32(DAT_006d4988, 8 + iVar7, (u32)DAT_0070ab14 + (u32)DAT_0073abc8);
  ST32(SUBP(DAT_006d4988 + iVar7), 10, LD32(DAT_006d4988, iVar7 + 8));
  ST32(DAT_006d4988, 0xc + iVar7, (u32)DAT_0070ac20 + (u32)DAT_0073abcc);
  ST32(SUBP(DAT_006d4988 + iVar7), 0xe, LD32(DAT_006d4988, iVar7 + 0xc));
  uVar13 = (u32)DAT_0073b374;
  if ((char)DAT_0070ad40[uVar13 * 0x566] == (char)LD32(DAT_00732e54, 0xb0)) {
    puVar9 = (u32 *)DAT_0070ad2c;
  } else {
    puVar9 = (u32 *)DAT_0070ad18;
  }
  uVar12 = 0;
  do {
    uVar17 = uVar12 + 1;
    ST8(DAT_00732e54, uVar12 + 0x1fb, *((u8 *)puVar9 + (u32)uVar12 + uVar13 * 0x566));
    uVar12 = uVar17;
  } while (uVar17 < 0x10);
  ST8(DAT_00732e54, 0x1fa, 0);
  ST16(DAT_00732e54, 0x1dd, 0x3ff);
  ST16(DAT_00732e54, 0x1d9, 0x1fff);
  ST16(DAT_006d4988, 0x10 + iVar7, DAT_007394c8);
  ST16(DAT_00732e54, 0xf, LD16(DAT_006d4988, 0x10 + iVar7));
  ST16(DAT_006d4988, 0x12 + iVar7, DAT_007394ce);
  ST16(DAT_00732e54, 0xd, LD16(DAT_006d4988, 0x12 + iVar7));
  ST16(DAT_006d4988, 0x14 + iVar7, 0);
  ST16(SUBP(DAT_006d4988 + iVar7), 0x77, 0);
  ST8(SUBP(DAT_006d4988 + iVar7), 0x90, 1);
  iVar18 = (int)LD32(DAT_006d4988, 0x1f + iVar7);
  ST8(iVar18, 0x89, LD8(iVar18, 0x90));
  ST8(DAT_00732e54, 0xa4, 0);
  ST8(DAT_00732e54, 0x9c, 0);
  ST8(DAT_00732e54, 0xab, 0);
  ST8(DAT_00732e54, 0x96, 0);
  ST8(DAT_00732e54, 0x97, LD8(DAT_00732e54, 0x96));
  ST8(DAT_00732e54, 0x9d, LD8(DAT_00732e54, 0x97));
  ST16(DAT_00732e54, 0x94, 0xffff);
  ST16(DAT_00732e54, 0xa2, 0xffff);
  ST16(DAT_00732e54, 0x9a, 0xffff);
  ST16(DAT_00732e54, 0xa9, 0xffff);
  ST16(DAT_00732e54, 0xa0, LD16(DAT_0052b097, iVar5));
  ST16(DAT_00732e54, 0x98, LD16(DAT_0052b099, iVar5));
  ST16(DAT_00732e54, 0xa7, LD16(DAT_0052b093, iVar5));
  ST16(DAT_00732e54, 0x92, LD16(DAT_0052b08d, iVar5));
  ST32(puVar2, 0, (u32)LD16(DAT_00524778, (u32)DAT_007394c7 * 2));
  if (DAT_007394c7 < 2) {
    uVar13 = 0;
  } else {
    uVar13 = (u32)(u16)(DAT_007394c7 - 2);
  }
  cVar4 = '\0';
  ST16(DAT_00732e54, 9, LD16(DAT_00524778, uVar13 * 2));
  uStack_20 = 0;
  ST8(DAT_00732e54, 0x211, 0);
  do {
    uVar28 = uStack_20;
    ST8(DAT_00732e54, uVar28 + 0x214, DAT_0052b0a8[iVar5 + uVar28]);
    if (bVar8) {
      ST8(DAT_00732e54, uVar28 + 0x216, 1);
    } else {
      ST8(DAT_00732e54, uVar28 + 0x216, 0);
    }
    ST8(DAT_00732e54, uVar28 + 0x218, 0);
    ST8(DAT_00732e54, uVar28 + 0x21a, 0);
    ST16(DAT_00732e54, uVar28 * 2 + 0x21c, 0);
    ST32(DAT_00732e54, 0x220 + uVar28 * 4, 0);
    if (LD8(DAT_00732e54, uVar28 + 0x214) != 0) {
      bVar16 = DAT_0052b0ac[iVar5 + uVar28];
      cVar4 = cVar4 + DAT_0052b0ae[iVar5 + uVar28];
      bVar10 = DAT_0052b0aa[iVar5 + uVar28];
      if (DAT_0052b0b0[iVar5 + uVar28] != 2) {
        ST8(DAT_00732e54, 0x211, LD8(DAT_00732e54, 0x211) + 1);
        ST8(DAT_00732e54, uVar28 + 0x21a, bVar10);
      }
      uVar12 = (u16)bVar10;
      uVar14 = (u32)bVar10;
      while (uVar12 <= bVar16) {
        if (DAT_0052b0b0[iVar5 + uVar28] == 2) {
          ST8(DAT_00732e54, 0x344 + uVar14 * 4, 2);
        } else {
          ST8(DAT_00732e54, 0x344 + uVar14 * 4, LD8(DAT_00732e54, uVar28 + 0x214));
        }
        uVar12 = (u16)((short)uVar14 + 1);
        iVar18 = uVar14 * 4;
        ST8(DAT_00732e54, iVar18 + 0x345, 0x7f);
        ST8(DAT_00732e54, iVar18 + 0x346, 0);
        ST16(DAT_00732e54, iVar18 + 0x386, 0xffff);
        uVar14 = (u32)uVar12;
      }
    }
    uStack_20 = uStack_20 + 1;
  } while (uStack_20 < 2);
  ST8(DAT_00732e54, 0x212, 2);
  ST8(DAT_00732e54, 0x213, cVar4);
  if (LD8(DAT_00732e54, 0x211) == 0) {
    ST8(DAT_00732e54, 0x1dd, LD8(DAT_00732e54, 0x1dd) ^ 0x10);
  }
  uStack_20 = 0;
  ST8(DAT_00732e54, 0x228, 0);
  do {
    if ((uStack_20 == 0) || (uVar28 = FUN_004a7aa0(0xc), (u16)uVar28 == uVar20)) {
      uVar28 = uStack_20;
      ST8(DAT_00732e54, uVar28 + 0x229, DAT_00524870[DAT_0070ad45[(u32)DAT_0073b374 * 0x566]]);
    } else {
      uVar28 = uStack_20;
      ST8(DAT_00732e54, uVar28 + 0x229, 0);
    }
    if (((uStack_20 == 1) && (uVar14 = FUN_004a7aa0(0xc), (u16)uVar14 == uVar20)) &&
        (DAT_007092c1 == '\0')) {
      ST8(DAT_00732e54, uVar28 + 0x229, 0x95);
    }
    ST8(DAT_00732e54, uVar28 + 0x22b, 1);
    ST16(DAT_00732e54, uVar28 * 2 + 0x22d, 0);
    if (LD8(DAT_00732e54, uVar28 + 0x229) != 0) {
      ST8(DAT_00732e54, 0x228, (char)LD32(DAT_00732e54, 0x228) + 1);
      bVar16 = DAT_0052b0b6[iVar5 + uVar28];
      local_24 = (u16)DAT_0052b0b4[iVar5 + uVar28];
      if (local_24 <= bVar16) {
        do {
          uVar14 = local_24;
          iVar18 = uVar14 * 4;
          ST8(DAT_00732e54, 0x344 + iVar18, LD8(DAT_00732e54, uVar28 + 0x229));
          ST16(DAT_00732e54, iVar18 + 0x386, 0xffff);
          ST8(DAT_00732e54, iVar18 + 0x345, 0x7f);
          uVar19 = DAT_0052b0ba[iVar5 + uVar28];
          uVar14 = (u32)DAT_0073b374;
          uVar25 = (u32)DAT_0070ad45[uVar14 * 0x566];
          if ((uStack_20 == 1) && (uVar14 = FUN_004a7aa0(0xc), (u16)uVar14 == uVar20)) {
            uVar25 = 5;
          }
          uVar14 = FUN_00478320((uVar14 & 0xffff0000u) | uVar19, LD16(DAT_00524880, uVar25 * 2));
          bVar10 = (u8)uVar14;
          if (bVar10 == 0) {
            bVar10 = 1;
          }
          if ((DAT_007394cb == '\x01') || (DAT_0073abc1 == '\x01')) {
            bVar10 = (u8)(bVar10 * 2);
          } else if ((DAT_007394cb == '\x02') || (DAT_0073abc1 == '\x02')) {
            bVar10 = bVar10 >> 1;
          }
          if (bVar10 == 0) {
            bVar10 = 1;
          }
          if (((iVar15 != -1) && (9 < bVar10)) && (DAT_006d4988[3 + iVar7] != '\f')) {
            bVar10 = 9;
          }
          local_24 = local_24 + 1;
          ST8(DAT_00732e54, iVar18 + 0x346, bVar10);
        } while (local_24 <= bVar16);
      }
    }
    uStack_20 = uStack_20 + 1;
  } while (uStack_20 < 2);
  ST16(DAT_00732e54, 0x231, 0);
  if ((char)LD32(DAT_00732e54, 0x228) == '\0') {
    ST8(DAT_00732e54, 0x1dd, LD8(DAT_00732e54, 0x1dd) ^ 8);
  }
  ST16(DAT_00732e54, 0x244, 0);
  ST16(DAT_00732e54, 0x242, (short)LD32(DAT_00732e54, 0x244));
  ST16(DAT_00732e54, 0x248, 0);
  ST16(DAT_00732e54, 0x246, (short)LD32(DAT_00732e54, 0x248));
  ST8(DAT_00732e54, 0x24b, 0);
  ST8(DAT_00732e54, 0x24a, LD8(DAT_00732e54, 0x24b));
  ST16(DAT_00732e54, 0x291, 0);
  uVar20 = DAT_0073b374;
  if (bVar8) {
    iVar15 = (int)((u32)local_14 * 0x5bd);
    ST32(DAT_0071b734, iVar15, (u32)LD16(DAT_006d4988, iVar7));
    ST16(DAT_0071b73e, iVar15, uVar20);
    ST16(DAT_0071b7b7, iVar15, 0);
    ST16(DAT_0071b7b5, iVar15, 0);
    ST16(DAT_0071b7bb, iVar15, 0);
    ST16(DAT_0071b7b9, iVar15, 0);
    ST16(DAT_0071b7bf, iVar15, 0);
    ST16(DAT_0071b7bd, iVar15, 0);
    ST32(DAT_0071b79d, iVar15, 0);
    ST32(DAT_0071b7a1, iVar15, 0);
    ST32(DAT_0071b7a5, iVar15, 0);
    ST32(DAT_0071b7a9, iVar15, 0);
    ST32(DAT_0071b7ad, iVar15, 0);
    ST32(DAT_0071b7b1, iVar15, 0);
    ST16(DAT_0071b9bf, iVar15, 0);
    ST16(DAT_0071b7c1, iVar15, 0);
    ST16(DAT_0071b7c3, iVar15, 0);
    uVar20 = 0;
    do {
      uVar12 = uVar20 + 1;
      ST16(DAT_0071b7c5, (u32)uVar20 * 2 + iVar15, 0);
      iVar18 = iVar15 + (u32)uVar20 * 2;
      ST16(DAT_0071b825, iVar18, 0);
      ST16(DAT_0071b885, iVar18, 0);
      ST16(DAT_0071b9e9, iVar18, 0);
      ST16(DAT_0071ba49, iVar18, 0);
      uVar20 = uVar12;
    } while (uVar12 < 0x30);
    uVar20 = 0;
    do {
      uVar12 = uVar20 + 1;
      ST16(DAT_0071b8e5, (u32)uVar20 * 2 + iVar15, 0);
      iVar18 = iVar15 + (u32)uVar20 * 2;
      ST16(DAT_0071b917, iVar18, 0);
      ST16(DAT_0071b949, iVar18, 0);
      ST16(DAT_0071baa9, iVar18, 0);
      uVar20 = uVar12;
    } while (uVar12 < 0x19);
    uVar20 = 0;
    do {
      uVar12 = uVar20 + 1;
      ST16(DAT_0071b97b, (u32)uVar20 * 2 + iVar15, 0);
      iVar18 = iVar15 + (u32)uVar20 * 2;
      ST16(DAT_0071b987, iVar18, 0);
      ST16(DAT_0071b993, iVar18, 0);
      ST16(DAT_0071badb, iVar18, 0);
      uVar20 = uVar12;
    } while (uVar12 < 6);
    uVar20 = 0;
    do {
      uVar12 = uVar20 + 1;
      ST16(DAT_0071b99f, (u32)uVar20 * 2 + iVar15, 0);
      iVar18 = iVar15 + (u32)uVar20 * 2;
      ST16(DAT_0071b9af, iVar18, 0);
      ST16(DAT_0071b9c9, iVar18, 0);
      ST16(DAT_0071b9d9, iVar18, 0);
      uVar20 = uVar12;
    } while (uVar12 < 8);
    ST16(DAT_0071b9c1, iVar15, 0);
    ST16(DAT_0071b9c3, iVar15, 0);
    ST16(DAT_0071b9c5, iVar15, 0);
    ST16(DAT_0071b9c7, iVar15, 0);
  }
  ST16(DAT_00732e54, 199, LD16(DAT_0052b087, iVar5));
  ST16(DAT_00732e54, 0xc5, LD16(DAT_0052b089, iVar5));
  ST16(DAT_00732e54, 0xc3, 0);
  ST16(DAT_00732e54, 0xcb, 0);
  ST16(DAT_00732e54, 0xc9, 0);
  ST32(DAT_00732e54, 0xcd, 0);
  ST32(DAT_00732e54, 0xd1, 0);
  ST32(DAT_00732e54, 0xd5, 0);
  ST32(DAT_00732e54, 0xd9, 0);
  ST32(DAT_00732e54, 0xdd, 0);
  iVar15 = 0;
  do {
    iVar18 = iVar15 + 4;
    ST32(DAT_00732e54, iVar15 + 0x1a1, 0);
    iVar15 = iVar18;
  } while (iVar18 < 0x20);
  iVar15 = 0;
  do {
    iVar18 = iVar15 + 1;
    ST32(DAT_00732e54, iVar18 * 4 + 0xdd, 0);
    ST8(DAT_00732e54, iVar15 + 0x1e6, 0);
    iVar15 = iVar18;
  } while (iVar18 < 10);
  iVar15 = 0;
  do {
    iVar18 = iVar15 + 4;
    ST32(DAT_00732e54, iVar15 + 0x1c1, 0);
    iVar15 = iVar18;
  } while (iVar18 < 0x18);
  ST16(DAT_00732e54, 0x1e1, 0);
  ST8(DAT_00732e54, 8, 0);
  ST8(DAT_00732e54, 0x1e3, 0);
  ST8(DAT_00732e54, 0x1e4, 0);
  ST8(DAT_00732e54, 0x1e5, 0);
  ST8(DAT_00732e54, 0x29, 0);
  uVar20 = 0;
  do {
    uVar12 = uVar20 + 1;
    ST32(DAT_00732e54, (u32)uVar20 * 4 + 0x15, 0);
    uVar20 = uVar12;
  } while (uVar12 < 5);
  uVar24 = 0;
  ST8(DAT_00732e54, 0x237, 0);
  ST16(DAT_00732e54, 0x238, 0);
  ST16(DAT_00732e54, 0x23a, 0xffff);
  do {
    if (LD8(SUBP(DAT_006d4988 + iVar7), 0x87) == uVar24) {
      ST8(DAT_00732e54, uVar24 + 0x1f0, 1);
    } else {
      ST8(DAT_00732e54, uVar24 + 0x1f0, 0);
    }
    uVar24 = uVar24 + 1;
  } while ((int)uVar24 < 10);
  if (((DAT_0052b081[iVar5] == 0) && (DAT_007394cb != '\t')) &&
      ((DAT_0073abc1 != '\t' && ((DAT_007394cb != '\x10' && (DAT_0073abc1 != '\x10')))))) {
    ST8(DAT_00732e54, 0x1dd, LD8(DAT_00732e54, 0x1dd) ^ 0x80);
  }
  if ((DAT_007394cb == '\x06') || (DAT_0073abc1 == '\x06')) {
    ST8(DAT_00732e54, 0x1dd, LD8(DAT_00732e54, 0x1dd) ^ 0x80);
  }
  ST16(DAT_00732e54, 0x20b, LD16(DAT_0052b083, iVar5));
  if (bVar8) {
    ST16(DAT_00732e54, 0x20d, LD16(DAT_0052b083, iVar5));
    ST8(DAT_00732e54, 0x210, 1);
  } else {
    ST16(DAT_00732e54, 0x20b, (short)LD16(DAT_00732e54, 0x20b) + (short)LD16(DAT_0052b083, iVar5));
    ST16(DAT_00732e54, 0x20d, 0);
    ST8(DAT_00732e54, 0x210, 0);
  }
  if ((DAT_007394cb == '\x03') || (DAT_0073abc1 == '\x03')) {
    ST16(DAT_00732e54, 0x20b, 0);
    ST16(DAT_00732e54, 0x20d, 0);
LAB_00420b41:
    ST8(DAT_00732e54, 0x1dd, LD8(DAT_00732e54, 0x1dd) ^ 1);
  } else {
    if ((DAT_007394cb == '\x04') || (DAT_0073abc1 == '\x04')) {
      ST16(DAT_00732e54, 0x20b, (short)LD16(DAT_00732e54, 0x20b) >> 1);
      ST16(DAT_00732e54, 0x20d, (short)LD16(DAT_00732e54, 0x20d) >> 1);
      goto LAB_00420b41;
    }
    if ((DAT_007394cb == '\a') || (DAT_0073abc1 == '\a')) {
      ST16(DAT_00732e54, 0x20b, 0);
      ST16(DAT_00732e54, 0x20d, 0);
    } else if ((((DAT_007394cb == '\x13') || (DAT_0073abc1 == '\x13')) || (DAT_007394cb == '\r')) ||
               (DAT_0073abc1 == '\r')) {
      ST16(DAT_00732e54, 0x20b, (short)LD16(DAT_00732e54, 0x20b) >> 1);
      ST16(DAT_00732e54, 0x20d, (short)LD16(DAT_00732e54, 0x20d) >> 1);
    } else if (((DAT_007394cb == '\x12') || (DAT_0073abc1 == '\x12')) ||
               ((DAT_007394cb == '\f' || (DAT_0073abc1 == '\f')))) {
      ST16(DAT_00732e54, 0x20b, (u32)(int)(short)LD16(DAT_00732e54, 0x20b) << 1);
      ST16(DAT_00732e54, 0x20d, (u32)(int)(short)LD16(DAT_00732e54, 0x20d) << 1);
      if ((short)LD16(DAT_00732e54, 0x20b) < 0) {
        ST16(DAT_00732e54, 0x20b, 0x7fbc);
      }
      if ((short)LD16(DAT_00732e54, 0x20d) < 0) {
        ST16(DAT_00732e54, 0x20d, 0x7fbc);
      }
    }
  }
  ST8(DAT_00732e54, 0x20f, 2);
  if (((DAT_0052b082[iVar5] == 0) && (DAT_007394cb != '\b')) &&
      ((DAT_0073abc1 != '\b' && ((DAT_007394cb != '\x10' && (DAT_0073abc1 != '\x10')))))) {
    ST8(DAT_00732e54, 0x1dd, LD8(DAT_00732e54, 0x1dd) ^ 1);
    ST16(DAT_00732e54, 0x20b, 0);
    ST16(DAT_00732e54, 0x20d, 0);
    ST8(DAT_00732e54, 0x1da, LD8(DAT_00732e54, 0x1da) ^ 8);
  }
  ST8(DAT_00732e54, 0x233, DAT_0070ad46[(u32)DAT_0073b374 * 0x566]);
  if ((((DAT_007394d2 == 1) || (DAT_007394d2 == 2)) || (DAT_007394d2 == 4)) ||
      ((DAT_007394d2 == 3 || (DAT_007394d2 == 0xe)))) {
    ST8(DAT_00732e54, 0x233, 0);
  }
  ST8(DAT_00732e54, 0x234, 2);
  ST16(DAT_00732e54, 0x235, 9999);
  if (LD8(DAT_00732e54, 0x233) == 0) {
    ST16(DAT_00732e54, 0x235, 0);
    ST8(DAT_00732e54, 0x1de, LD8(DAT_00732e54, 0x1de) ^ 1);
    ST8(DAT_00732e54, 0x1da, LD8(DAT_00732e54, 0x1da) ^ 0x10);
    ST8(DAT_00732e54, 0x1d9, LD8(DAT_00732e54, 0x1d9) ^ 0x10);
  }
  ST8(DAT_00732e54, 0x23c, DAT_0070b213[(u32)DAT_0073b374 * 0x566]);
  if ((char)LD32(DAT_00732e54, 0x23c) == '\0') {
    ST8(DAT_00732e54, 0x23d, 0);
  } else {
    ST8(DAT_00732e54, 0x23d, DAT_0052b13c[iVar5]);
    if ((char)LD32(DAT_00732e54, 0x23c) == '\x02') {
      puVar1 = DAT_00732e54 + 0x23d;
      uVar24 = FUN_00478320(((u32)DAT_00732e54 & 0xffff0000u) | *puVar1, 0xaaac);
      *puVar1 = (u8)uVar24;
    }
  }
  ST16(DAT_00732e54, 0x23e, 0);
  ST16(DAT_00732e54, 0x240, 0);
  ST16(DAT_00732e54, 0x1df, LD16(DAT_00732e54, 0x1dd));
  ST16(DAT_00732e54, 0x1db, LD16(DAT_00732e54, 0x1d9));
  uVar20 = 0;
  do {
    uVar12 = uVar20 + 1;
    DAT_006d4988[(u32)uVar20 + 0x19 + iVar7] = 0;
    uVar20 = uVar12;
  } while (uVar12 < 2);
  uVar20 = 0;
  do {
    uVar12 = uVar20 + 1;
    uVar24 = (u32)uVar20;
    ST8(DAT_00732e54, uVar24 + 0x2ca, 0);
    ST8(DAT_00732e54, uVar24 + 0x2f2, 0);
    ST8(DAT_00732e54, uVar24 + 0x31a, 0xff);
    uVar20 = uVar12;
  } while (uVar12 < 0x28);
  uVar24 = 0;
  iVar15 = FUN_004490f0(uVar27);
  if (0 < iVar15) {
    do {
      iVar18 = FUN_00449790(uVar27, uVar24);
      uVar28 = FUN_0044a600(uVar27, uVar24);
      if (uVar28 != 0) {
        ST8(DAT_00732e54, uVar24 + 0x31a, DAT_00524898[iVar18]);
      }
      if ((((DAT_007394cb == '\x05') || (DAT_0073abc1 == '\x05')) ||
           ((DAT_007394cb == '\x0e' || (DAT_0073abc1 == '\x0e')))) &&
          (((iVar18 == 4 || (iVar18 == 0x15)) || (iVar18 == 5)))) {
        ST8(DAT_00732e54, uVar24 + 0x31a, 0);
        ST8(DAT_00732e54, uVar24 + 0x2ca, 4);
      }
      uVar24 = (u32)(u16)((short)uVar24 + 1);
    } while ((int)uVar24 < iVar15);
  }
  if (((DAT_0073b370 == 5) && (0x3b < DAT_007394d2)) &&
      ((DAT_007394d2 < 0x41 && (DAT_0070ad46[(u32)DAT_0073b374 * 0x566] != 0)))) {
    uVar20 = (u16)((u32)(u16)DAT_007394d2 * 0xc - 0x2d0);
    uVar27 = (u32)uVar20;
    uVar24 = (u32)(u16)((((u32)-(int)(u16)(DAT_0070ad46[(u32)DAT_0073b374 * 0x566] == 1)) & 0xfffa) + 0xc) + uVar27;
    while (uVar27 < uVar24) {
      bVar16 = DAT_005248b8[uVar20];
      if (bVar16 != 0xff) {
        ST8(DAT_00732e54, bVar16 + 0x31a, 0);
        ST8(DAT_00732e54, bVar16 + 0x2ca, 4);
      }
      uVar20 = uVar20 + 1;
      uVar27 = (u32)uVar20;
    }
  }
  bVar16 = DAT_0070ad92[(u32)DAT_0073b374 * 0x566];
  iVar15 = FUN_00481380(DAT_00524948);
  ST8(DAT_00732e54, 0x34, iVar15);
  ST8(DAT_00732e54, 0x35, LD8(DAT_00732e54, 0x34));
  bVar10 = DAT_006da3e0[DAT_00524798[bVar16]];
  ST8(puVar2, 0xf, bVar10);
  uVar19 = DAT_006da414;
  if (((DAT_007394ca == '\0') && (uVar19 = DAT_006da412, DAT_007394cc == '\0')) &&
      (uVar19 = DAT_006da3e0[DAT_005247c0[bVar16]],
       (char)LD8(DAT_00732e54, 5) == -1)) {
    ST8(puVar2, 0xe, bVar10);
  } else {
    ST8(puVar2, 0xe, uVar19);
  }
  bVar34 = (bVar10 == 0);
  pcVar30 = (char *)(DAT_00703ca0 + (u32)bVar10 * 0x55);
  uVar24 = 8;
  pcVar29 = pcVar30;
  pcVar33 = DAT_00524948;
  do {
    if (uVar24 == 0) break;
    uVar24 = uVar24 - 1;
    bVar34 = (*pcVar29 == *pcVar33);
    pcVar29 = pcVar29 + 1;
    pcVar33 = pcVar33 + 1;
  } while (bVar34);
  if (bVar34) {
LAB_00421003:
    if (!bVar8) {
      uVar24 = uVar24 & 0xffff0000u;
      goto LAB_0042104e;
    }
  } else {
    uVar24 = 0x11;
    bVar34 = 0;
    pcVar29 = pcVar30;
    pcVar33 = s_stationaryldrpln_00524934;
    do {
      if (uVar24 == 0) break;
      uVar24 = uVar24 - 1;
      bVar34 = (*pcVar29 == *pcVar33);
      pcVar29 = pcVar29 + 1;
      pcVar33 = pcVar33 + 1;
    } while (bVar34);
    if (bVar34) goto LAB_00421003;
    uVar24 = 0x11;
    bVar34 = 0;
    pcVar29 = pcVar30;
    pcVar33 = s_stationaryflwpln_00524920;
    do {
      if (uVar24 == 0) break;
      uVar24 = uVar24 - 1;
      bVar34 = (*pcVar29 == *pcVar33);
      pcVar29 = pcVar29 + 1;
      pcVar33 = pcVar33 + 1;
    } while (bVar34);
    if (bVar34) goto LAB_00421003;
    uVar24 = 0xc;
    bVar34 = 0;
    pcVar29 = pcVar30;
    pcVar33 = s_disabledpln_00524914;
    do {
      if (uVar24 == 0) break;
      uVar24 = uVar24 - 1;
      bVar34 = (*pcVar29 == *pcVar33);
      pcVar29 = pcVar29 + 1;
      pcVar33 = pcVar33 + 1;
    } while (bVar34);
    if (bVar34) goto LAB_00421003;
  }
  bVar34 = 0;
  iVar15 = 0xe;
  pcVar29 = s_escortldr1pln_00524904;
  do {
    if (iVar15 == 0) break;
    iVar15 = iVar15 + -1;
    bVar34 = (*pcVar30 == *pcVar29);
    pcVar30 = pcVar30 + 1;
    pcVar29 = pcVar29 + 1;
  } while (bVar34);
  uVar24 = ((u32)iVar15 & 0xffff0000u) | 0x8000;
  if (!bVar34) {
    uVar24 = (((u32)DAT_0073b374 * 0x2b3) & 0xffff0000u) |
             LD16(DAT_0052a650, (u32)DAT_0070ad93[(u32)DAT_0073b374 * 0x566] * 2);
  }
LAB_0042104e:
  ST8(DAT_00732e54, 7, DAT_0073abc0);
  uVar19 = DAT_0052b080[iVar5];
  ST16(DAT_00732e54, 0xbd, (u16)uVar24);
  ST16(DAT_00732e54, 0xbf, 0xffff);
  uVar24 = FUN_00478320((uVar24 & 0xffff0000u) | LD16(DAT_0052b08d, iVar5), (u16)uVar24);
  ST16(SUBP(DAT_006d4988 + iVar7), 0x79, (short)uVar24);
  ST16(SUBP(DAT_006d4988 + iVar7), 0x7b, 0);
  if (bVar8) {
    DAT_0071b757[(u32)local_14 * 0x5bd] = 0;
    DAT_0071b758[(u32)local_14 * 0x5bd] = 0;
    local_14 = (int)((u32)local_14 * 0x5bd);
    DAT_0071b742[local_14] = uVar19;
    ST16(DAT_0071b761, local_14, 0x5555);
    DAT_0071b765[local_14] = 2;
    DAT_0071b767[local_14] = 2;
    DAT_0071b769[local_14] = 2;
    ST16(DAT_0071b763, local_14, 0xffff);
    DAT_0071b766[local_14] = 3;
    DAT_0071b768[local_14] = 2;
    DAT_0071b76a[local_14] = 2;
  }
  uVar20 = 0;
  do {
    uVar12 = uVar20 + 1;
    ST8(puVar2, uVar20 + 5, 0);
    ST8(puVar2, uVar20 + 9, 0);
    uVar20 = uVar12;
  } while (uVar12 < 4);
  ST8(puVar2, 4, 0);
  ST8(puVar2, 0xd, 0);
  ST16(puVar2, 0x1c, 0xffff);
  ST16(puVar2, 0x2f, 0xffff);
  ST16(puVar2, 0x1e, 0);
  ST8(puVar2, 0x22, 0);
  ST16(puVar2, 0x20, 0xffff);
  ST8(puVar2, 0x31, 0xff);
  ST32(puVar2, 0x23, 0);
  ST32(puVar2, 0x27, 0);
  ST32(puVar2, 0x2b, 0);
  ST16(puVar2, 0x32, 0x4000);
  ST16(puVar2, 0x34, 0);
  ST16(puVar2, 0x36, 0);
  ST8(puVar2, 0x10, 4);
  ST8(puVar2, 0x11, 0);
  ST32(puVar2, 0x12, (u32)LD16(DAT_00524788, (u32)DAT_007394c7 * 2));
  uVar20 = FUN_00478470();
  ST16(puVar2, 0x1a, uVar20 ^ 0xbeef);
  ST8(puVar2, 0x38, 0);
  ST8(puVar2, 0x39, 0);
  ST32(puVar2, 0x3a, 0);
  uVar20 = 0;
  do {
    uVar12 = uVar20 + 1;
    ST8(DAT_00732e54, uVar20 + 0x2b, 0);
    ST8(DAT_00732e54, uVar20 + 0x2f, 0);
    uVar20 = uVar12;
  } while (uVar12 < 4);
  ST8(DAT_00732e54, 0x2a, 0);
  ST8(DAT_00732e54, 0x33, 0);
  ST16(DAT_00732e54, 0x42, 0xffff);
  ST16(DAT_00732e54, 0x55, LD16(DAT_00732e54, 0x42));
  ST16(DAT_00732e54, 0x44, 0);
  ST8(DAT_00732e54, 0x48, 0);
  ST16(DAT_00732e54, 0x46, 0xffff);
  ST16(DAT_00732e54, 0x66, LD16(DAT_00732e54, 0x46));
  ST16(DAT_00732e54, 0x68, LD16(DAT_00732e54, 0x66));
  ST16(DAT_00732e54, 0x70, 0xffff);
  ST8(DAT_00732e54, 0x57, 0xff);
  ST8(DAT_00732e54, 0x72, 0);
  ST8(DAT_00732e54, 0x73, 0);
  ST8(DAT_00732e54, 0x74, 0);
  ST8(DAT_00732e54, 0x75, 0);
  ST8(DAT_00732e54, 0x76, 0);
  ST8(DAT_00732e54, 0x77, 0);
  ST8(DAT_00732e54, 0x79, 0);
  ST16(DAT_00732e54, 0xc1, 0);
  ST8(DAT_00732e54, 0x7a, 0);
  ST8(DAT_00732e54, 0x7b, 0);
  ST8(DAT_00732e54, 0x7c, 0);
  ST8(DAT_00732e54, 0x7d, 0);
  ST32(DAT_00732e54, 0x49, 0);
  uVar24 = 0;
  do {
    ST8(DAT_00732e54, uVar24 + 0x293, (char)uVar24);
    uVar20 = (u16)((short)uVar24 + 1);
    ST16(DAT_00732e54, uVar24 * 2 + 0x29e, 100);
    ST16(DAT_00732e54, uVar24 * 2 + 0x2b4, 0);
    uVar24 = (u32)uVar20;
  } while (uVar20 < 10);
  ST8(DAT_00732e54, 0x36, 4);
  ST32(DAT_00732e54, 0x38, (u32)LD16(DAT_00524788, uVar13 * 2));
  uVar20 = FUN_00478470();
  ST16(DAT_00732e54, 0x40, uVar20 ^ 0xbeef);
  FUN_004296b0(uVar26);
  FUN_00429310(uVar6);
  FUN_004213b0((int)DAT_00732e54);
  ST16(DAT_00732e54, 0x342, 0xffff);
  DAT_006d4a68[(u32)DAT_0073b374 * 0x93] = DAT_006d4a68[(u32)DAT_0073b374 * 0x93] + 1;
  return uVar26;
}
