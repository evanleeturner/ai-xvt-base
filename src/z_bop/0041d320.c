/* SPDX-License-Identifier: MIT */
/* z_bop 0041d320 */
/* Packed / possibly-unaligned access helpers (records are 0x23 bytes, so
   most multi-byte fields are unaligned). */
#define LD(T, a) ({ T _t; __builtin_memcpy(&_t, (const void *)(unsigned int)(a), sizeof(T)); _t; })
#define ST(T, a, v) do { T _t = (v); __builtin_memcpy((void *)(unsigned int)(a), &_t, sizeof(T)); } while (0)
#define U8(a) (*(unsigned char *)(unsigned int)(a))
#define S8(a) (*(char *)(unsigned int)(a))
#define ENT(i) ((unsigned int)DAT_009e967c + (unsigned int)(i) * 0x5bd)

extern unsigned int DAT_009a1fe8;
extern int DAT_009a1fe0;
extern char DAT_009d8c88[];
extern char DAT_009d8c80[];
extern unsigned char DAT_009a20d2[];
extern char DAT_009a2106[];
extern char DAT_00bb7375;
extern unsigned char DAT_009e967c[];
extern char DAT_009e9681[];
extern int DAT_009ecc34;
extern char DAT_009fd458[];
extern char DAT_0051a44f[];
extern unsigned char DAT_009ed228[4];
extern unsigned char DAT_009ed22c;
extern unsigned char DAT_009ed22d;
extern int DAT_009a1fd8;
extern int DAT_00a081e8;
extern int DAT_009a1fd4;
extern int DAT_009ec604;
extern int DAT_009a1fd0;
extern int DAT_00a08298;
extern char DAT_009d695f;
extern unsigned short DAT_0051bf8e;
extern unsigned short DAT_00a07be0;
extern unsigned short DAT_00a07be2;
extern int DAT_009d77fc;
extern int DAT_009d6828;
extern int DAT_009cd260;

void FUN_00430200(int, int, unsigned int, int);
void FUN_00435640(int, unsigned int, int, int, unsigned int, unsigned int);
int FUN_00435a10(unsigned char, unsigned char, unsigned char);
void FUN_00427bd0(short, short, int);
void FUN_00427d30(short, int, int);
void FUN_0041bc50(unsigned short, unsigned short);
void FUN_00450650(int, int);
unsigned int FUN_0041dc30(unsigned int, unsigned short, unsigned int, unsigned int,
                          unsigned int, unsigned short, unsigned int, unsigned short,
                          unsigned int);
void FUN_0042e4d0(unsigned int, unsigned int, int);
unsigned int FUN_00425d10(void);

void __cdecl
FUN_0041d320(unsigned int param_1, unsigned short param_2, unsigned int param_3,
             unsigned int param_4, unsigned int param_5, unsigned int param_6,
             unsigned short param_7)
{
  unsigned char *puVar1;
  unsigned char *pbVar2;
  unsigned char bVar3;
  unsigned short uVar4;
  int iVar5;
  int iVar6;
  int iVar7;
  unsigned char bVar8;
  char cVar9;
  short sVar10;
  unsigned int uVar11;
  unsigned int uVar12;
  int iVar13;
  int iVar14;
  unsigned short uVar15;
  unsigned short uVar16;
  unsigned short unaff_BX = 0;
  unsigned int uVar17;
  unsigned int unaff_EBP = 0;
  unsigned int unaff_ESI = 0;
  unsigned short unaff_DI = 0;
  int bVar18;
  unsigned short in_stack_0000000a = 0;
  int iVar19;
  int iVar20;
  unsigned int in_stack_ffffffc8 = 0;
  unsigned char uVar21;
  short sStack_1c;

  uVar11 = param_1 & 0xffff;
  iVar20 = uVar11 * 0x23;
  uVar4 = LD(unsigned short, LD(unsigned int, DAT_009a1fe8 + 0x1f + iVar20) + 0x85);
  uVar17 = (unsigned int)uVar4;
  iVar13 = uVar17 * 0x23;
  if (uVar4 == param_2) {
    return;
  }
  iVar6 = (unsigned int)param_2 * 0x23;
  iVar5 = LD(int, LD(unsigned int, DAT_009a1fe8 + 0x1f + iVar6) + 0xa9);
  pbVar2 = (unsigned char *)(unsigned int)
           ((unsigned char)DAT_009d8c88[(unsigned int)U8(DAT_009a1fe8 + 0x16 + iVar13) * 0x566] +
            0x1ec + iVar5);
  if (*pbVar2 == 0) {
    *pbVar2 = 1;
    iVar14 = DAT_009a1fe8 + iVar6;
    uVar12 = (unsigned int)U8(iVar14 + 0x16);
    iVar7 = uVar12 * 0x126;
    *(short *)(DAT_009a20d2 + iVar7) = (short)(*(short *)(DAT_009a20d2 + iVar7) + 1);
    iVar19 = uVar12 * 0x566;
    if (DAT_009d8c80[iVar19] == S8(iVar5 + 0xb0)) {
      DAT_009a2106[iVar7] = 1;
    }
    if (((DAT_00bb7375 == '\x02') &&
         ((unsigned short)(unsigned char)DAT_009d8c88[iVar19] ==
          LD(unsigned short, ENT(DAT_009ecc34))) &&
         (S8(iVar14 + 2) != '\0'))) {
      if ((int)uVar17 < DAT_009a1fe0) {
        cVar9 = S8(DAT_009a1fe8 + 3 + iVar20);
        if ((cVar9 == -0x70) || (cVar9 == -0x6b)) {
          iVar19 = 0x2b;
        }
        else if ((cVar9 == -0x71) || (cVar9 == -0x6c)) {
          iVar19 = 0x2c;
        }
        else if (cVar9 == -0x69) {
          iVar19 = 0x2d;
        }
        else if (cVar9 == -0x6a) {
          iVar19 = 0x2e;
        }
        else {
          cVar9 = S8(DAT_009a1fe8 + 2 + iVar13);
          if (cVar9 == '\0') {
            iVar19 = 0x19;
          }
          else {
            if (cVar9 != '\x04') goto LAB_0041d52a;
            iVar19 = 0x1a;
          }
        }
      }
      else {
LAB_0041d52a:
        iVar19 = 0x18;
      }
      FUN_00430200(1, iVar19, (unsigned int)param_2, 0xffff);
    }
  }
  iVar19 = LD(int, DAT_009a1fe8 + 0x1b + iVar13);
  if (iVar19 != -1) {
    if ((*pbVar2 & 0x80) == 0) {
      bVar8 = U8(DAT_009a1fe8 + 0x16 + iVar6);
      uVar12 = (unsigned int)(DAT_009d8c80[(unsigned int)bVar8 * 0x566] == S8(iVar5 + 0xb0));
      FUN_00435640(3, (DAT_009a1fe8 & 0xffff0000) | bVar8, iVar19, 1, uVar12,
                   (unsigned int)LD(unsigned short, ENT(iVar19)));
      FUN_00435640(3, (DAT_009a1fe8 & 0xffff0000) | U8(DAT_009a1fe8 + 0x16 + iVar6), -1, 1, uVar12,
                   (unsigned int)LD(unsigned short,
                                    ENT(LD(int, DAT_009a1fe8 + 0x1b + iVar13))));
    }
    *pbVar2 = *pbVar2 | 0x80;
  }
  if (LD(short, iVar5 + 0x6a) == -1) {
    if (DAT_009a1fe0 <= (int)uVar17) goto LAB_0041d750;
    iVar13 = DAT_009a1fe8 + iVar13;
    cVar9 = S8(iVar13 + 2);
    if (((cVar9 == '\x04') || (cVar9 == '\x05')) || (cVar9 == '\x03')) goto LAB_0041d750;
    uVar21 = (unsigned char)(in_stack_ffffffc8 >> 0x18);
    uVar15 = (unsigned short)in_stack_ffffffc8;
    in_stack_ffffffc8 = ((unsigned int)uVar21 << 24) | (1u << 16) | uVar15;
    if (LD(int, iVar13 + 0x1b) != -1) {
      bVar8 = U8(LD(unsigned int, iVar13 + 0x1f) + 0x89);
      bVar3 = U8(LD(unsigned int, DAT_009a1fe8 + iVar6 + 0x1f) + 0x89);
      if (bVar8 == bVar3) {
        bVar18 = 0;
      }
      else {
        bVar18 = DAT_009fd458[(unsigned int)bVar3 + (unsigned int)bVar8 * 0x1e5] == '\0';
      }
      if ((!bVar18) && (S8(DAT_009a1fe8 + iVar6 + 2) != '\0')) {
        bVar8 = (unsigned char)((*pbVar2 & 0x70) >> 4);
        if (bVar8 < 5) {
          bVar8 = (unsigned char)(bVar8 + (((DAT_0051a44f[U8(DAT_009a1fe8 + 3 + iVar20)] == '\0') - 1U) & 4));
          if (7 < bVar8) {
            bVar8 = 7;
          }
          *pbVar2 = (unsigned char)((bVar8 << 4) | (*pbVar2 & 0x8f));
        }
        if (bVar8 < 7) {
          in_stack_ffffffc8 = ((unsigned int)uVar21 << 24) | (unsigned int)uVar15;
        }
      }
    }
    if ((char)(in_stack_ffffffc8 >> 0x10) != '\x01') goto LAB_0041d750;
  }
  else if (LD(int, DAT_009a1fe8 + 0x1b + iVar6) == -1) goto LAB_0041d750;
  ST(unsigned short, iVar5 + 0x6a, uVar4);
  iVar13 = FUN_00435a10(DAT_009ed228[3], DAT_009ed22c, DAT_009ed22d);
  ST(short, iVar5 + 0x6c, (short)iVar13);
LAB_0041d750:
  ST(unsigned short, iVar5 + 0x6e, uVar4);
  U8(iVar5 + 0x79) = (unsigned char)(U8(iVar5 + 0x79) + 1);
  iVar13 = iVar6 + DAT_009a1fe8;
  if (S8(LD(unsigned int, iVar13 + 0x1f) + 0x92) != '\0') {
    FUN_00427bd0(LD(short, iVar13 + 0x12), LD(short, iVar13 + 0x10), iVar13);
    FUN_00427d30(LD(short, DAT_009a1fe8 + iVar6 + 0x14), 0, DAT_009a1fe8 + iVar6);
  }
  iVar19 = DAT_009a1fe8 + iVar6;
  uVar17 = (unsigned int)((int)LD(short, LD(unsigned int, iVar19 + 0x1f) + 0x95) *
                          (int)(short)((short)DAT_009a1fd8 - (short)DAT_00a081e8));
  iVar13 = (int)((unsigned int)((int)LD(short, LD(unsigned int, iVar19 + 0x1f) + 0x93) *
                                (int)(short)((short)DAT_009a1fd4 - (short)DAT_009ec604)) +
                 uVar17 +
                 (unsigned int)((int)LD(short, LD(unsigned int, iVar19 + 0x1f) + 0x97) *
                                (int)(short)((short)DAT_009a1fd0 - (short)DAT_00a08298)));
  if (0x3fffffff < iVar13) {
    iVar13 = 0x3fffffff;
  }
  if (iVar13 < -0x3fffffff) {
    iVar13 = -0x3fff0000;
  }
  sStack_1c = (short)(iVar13 >> 0xf);
  uVar17 = uVar17 & 0xffff0000;
  if (LD(int, iVar19 + 0x1b) != -1) {
    uVar17 = (unsigned int)(-1 < sStack_1c);
  }
  if (((DAT_009d695f != '\0') && (S8(iVar5 + 7) == '\0')) &&
      (DAT_0051a44f[U8(DAT_009a1fe8 + 3 + iVar20)] != '\0')) {
    FUN_0041bc50(param_2, (unsigned short)param_1);
  }
  bVar8 = U8(DAT_009a1fe8 + 3 + iVar20);
  if (bVar8 == 0x98) {
    uVar4 = LD(unsigned short, iVar5 + 0x1e7);
    if (LD(int, DAT_009a1fe8 + iVar6 + 0x1b) == -1) {
      cVar9 = S8(DAT_009a1fe8 + iVar6 + 2);
      if (((cVar9 == '\0') || (cVar9 == '\x01')) || (cVar9 == '\x02')) {
        sVar10 = (short)(uVar4 + 0xdd4);
      }
      else {
        sVar10 = (short)(uVar4 + 0x1ba8);
      }
      ST(short, iVar5 + 0x1e7, sVar10);
    }
    else {
      uVar15 = 0;
      if (U8(iVar5 + 0x21d) != '\0') {
        do {
          uVar16 = (unsigned short)(uVar15 + 1);
          U8(iVar5 + 0x371 + (unsigned int)uVar15 * 4) = 0;
          uVar15 = uVar16;
        } while (uVar16 < U8(iVar5 + 0x21d));
      }
      if ((LD(unsigned short, iVar5 + 0x1e5) & 0x10) == 0) {
        FUN_00450650(0x11b, LD(int, DAT_009a1fe8 + 0x1b + iVar6));
      }
      else {
        ST(unsigned short, iVar5 + 0x1e5, (unsigned short)(LD(unsigned short, iVar5 + 0x1e5) & 0x3ef));
        ST(unsigned short, iVar5 + 0x2b2, 0);
        ST(unsigned short, iVar5 + 0x2c8, DAT_0051bf8e);
        if ((LD(int, DAT_009a1fe8 + 0x1b + iVar6) == DAT_009ecc34) &&
            (DAT_009e9681[(unsigned int)DAT_009ecc34 * 0x5bd] == '\0')) {
          DAT_00a07be0 = 0x5c;
          DAT_00a07be2 = 0x57;
          FUN_00450650(0x56, DAT_009ecc34);
        }
      }
    }
    if (LD(unsigned short, iVar5 + 0x1e7) < uVar4) {
      ST(unsigned short, iVar5 + 0x1e7, 0xffff);
    }
    cVar9 = '\x01';
  }
  else {
    bVar18 = 0;
    if (((S8(iVar5 + 0x246) == '\x01') && (LD(short, iVar5 + 0x248) != 0)) &&
        ((DAT_0051a44f[bVar8] != '\0' && (-1 < sStack_1c)))) {
      bVar18 = 1;
      FUN_00450650(0x171, LD(int, DAT_009a1fe8 + 0x1b + iVar6));
    }
    if (bVar18) {
      cVar9 = (char)(in_stack_ffffffc8 >> 0x18);
    }
    else {
      uVar17 = FUN_0041dc30(((unsigned int)in_stack_0000000a << 16) | (unsigned int)param_2,
                            (unsigned short)param_3, param_1, uVar17,
                            unaff_EBP, unaff_DI, unaff_ESI, unaff_BX, in_stack_ffffffc8);
      cVar9 = (char)uVar17;
    }
  }
  ST(int, DAT_009a1fe8 + 4 + iVar20, (int)((unsigned int)DAT_009d77fc + (unsigned int)DAT_009ec604));
  ST(int, DAT_009a1fe8 + 8 + iVar20, (int)((unsigned int)DAT_009d6828 + (unsigned int)DAT_00a081e8));
  ST(int, DAT_009a1fe8 + 0xc + iVar20, (int)((unsigned int)DAT_009cd260 + (unsigned int)DAT_00a08298));
  bVar8 = U8(DAT_009a1fe8 + 3 + iVar20);
  puVar1 = (unsigned char *)(DAT_009a1fe8 + 3 + iVar20);
  if (DAT_0051a44f[bVar8] == '\0') {
    if ((bVar8 == 0x8d) || (bVar8 == 0x8e)) {
      *puVar1 = 0x84;
    }
    else {
      *puVar1 = 0x83;
    }
  }
  else {
    *puVar1 = 0x81;
  }
  U8(DAT_009a1fe8 + 2 + iVar20) = 0xd;
  U8(LD(unsigned int, DAT_009a1fe8 + 0x1f + iVar20)) = 5;
  U8(DAT_009a1fe8 + 0x19 + iVar20) = 2;
  ST(short, LD(unsigned int, DAT_009a1fe8 + 0x1f + iVar20) + 0x83, 0);
  ST(short, LD(unsigned int, DAT_009a1fe8 + 0x1f + iVar20) + 0x81, 0);
  U8(LD(unsigned int, DAT_009a1fe8 + 0x1f + iVar20) + 1) = 0;
  ST(short, LD(unsigned int, DAT_009a1fe8 + 0x1f + iVar20) + 0x79,
     LD(short, LD(unsigned int, DAT_009a1fe8 + 0x1f + iVar6) + 0x79));
  ST(short, DAT_009a1fe8 + 0x12 + iVar20, LD(short, DAT_009a1fe8 + 0x12 + iVar6));
  ST(short, DAT_009a1fe8 + 0x10 + iVar20, LD(short, DAT_009a1fe8 + 0x10 + iVar6));
  ST(short, DAT_009a1fe8 + 0x14 + iVar20, 0);
  U8(LD(unsigned int, DAT_009a1fe8 + 0x1f + iVar20) + 0x92) = 1;
  iVar13 = LD(int, DAT_009a1fe8 + 0x1f + iVar20);
  U8(iVar13 + 0x8b) = U8(iVar13 + 0x92);
  if (cVar9 != '\0') {
    if (LD(int, DAT_009a1fe8 + 0x1b + iVar6) == DAT_009ecc34) {
      FUN_0042e4d0(0x1d, uVar11, DAT_009ecc34);
      return;
    }
    cVar9 = S8(DAT_009a1fe8 + 3 + iVar20);
    if ((cVar9 != -0x7d) && (cVar9 != -0x7c)) {
      iVar20 = DAT_009ecc34;
      uVar17 = FUN_00425d10();
      FUN_0042e4d0((uVar17 & 3) + 0x16, uVar11, iVar20);
      return;
    }
    FUN_0042e4d0(0x1c, uVar11, DAT_009ecc34);
  }
  return;
}
