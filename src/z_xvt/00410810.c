/* SPDX-License-Identifier: MIT */
/* z_xvt 00410810 */
#define RD32(a) ({ unsigned int _r32; __builtin_memcpy(&_r32, (const void *)(a), 4); _r32; })
#define RD16(a) ({ unsigned short _r16; __builtin_memcpy(&_r16, (const void *)(a), 2); _r16; })
#define WR32(a, v) do { unsigned int _w32 = (unsigned int)(v); __builtin_memcpy((void *)(a), &_w32, 4); } while (0)
#define WR16(a, v) do { unsigned short _w16 = (unsigned short)(v); __builtin_memcpy((void *)(a), &_w16, 2); } while (0)
#define PTR(a) ((unsigned char *)RD32(a))

extern unsigned char *DAT_006d4988;
extern int DAT_006d4980;
extern unsigned char DAT_0070ad48[];
extern char DAT_0070ad40[];
extern unsigned char DAT_006d4a72[];
extern unsigned char DAT_006d4aa6[];
extern char DAT_0085e795;
extern unsigned char DAT_0071b73c[];
extern char DAT_0071b741[];
extern int DAT_0071ecf4;
extern char DAT_00525bf7[];
extern unsigned char DAT_0071f2d8[4];
extern unsigned char DAT_0071f2dc;
extern unsigned char DAT_0071f2dd;
extern int DAT_006d4974, DAT_006d4978, DAT_006d4970;
extern int DAT_0071e6c4, DAT_00732f38, DAT_00732fe8;
extern char DAT_007092df;
extern int DAT_0070a17c, DAT_007091a8, DAT_006ffbe0;
extern unsigned short DAT_00523b9e;
extern short _DAT_00732950;
extern short _DAT_00732952;

void __cdecl FUN_0040c070(int, int, int, int);
void __cdecl FUN_004a1590(int, unsigned int, int, int, unsigned int, unsigned int);
unsigned char __cdecl FUN_004440b0(unsigned int, unsigned int);
int __cdecl FUN_004a18c0(unsigned int, unsigned int, unsigned int);
void __cdecl FUN_00483410(int, int, int);
void __cdecl FUN_00483540(int, int, int);
int __cdecl FUN_00413d00(int, int, int, int, int, int);
void __cdecl FUN_0040f260(unsigned short, unsigned short);
void __cdecl FUN_004a1d70(int, int);
unsigned int __cdecl FUN_004110e0(unsigned int, unsigned int, unsigned int, unsigned int,
                                  unsigned int, unsigned short, unsigned int, unsigned short,
                                  unsigned int);
void __cdecl FUN_0040a1a0(int, unsigned int, int);
unsigned int __cdecl FUN_004784d0(void);

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wuninitialized"
void __cdecl
FUN_00410810(unsigned int param_1, unsigned int param_2, int param_3, unsigned int param_4,
             unsigned int param_5, unsigned int param_6, unsigned int param_7)
{
  unsigned char *puVar1;
  unsigned char *pbVar2;
  char cVar3;
  unsigned char *iVar4;
  unsigned int iVar5;
  int bVar6;
  unsigned char bVar7;
  unsigned short uVar8;
  short sVar9;
  unsigned int uVar10;
  unsigned int uVar11;
  unsigned int uVar12;
  unsigned short uVar13;
  unsigned char *iVar14p;
  int iVar14;
  int iVar15;
  unsigned int iVar15u;
  unsigned char *pEnt;
  unsigned char bVar16;
  unsigned short unaff_BX;
  unsigned int unaff_EBP;
  unsigned int unaff_ESI;
  unsigned short unaff_DI;
  unsigned int iVar17;
  unsigned short in_stack_ffffffe0;
  unsigned short uVar18;
  char cStack_1c;
  unsigned int local_18;
  unsigned int local_14;

  uVar10 = param_1 & 0xffff;
  iVar17 = uVar10 * 0x23;
  uVar18 = RD16(PTR(DAT_006d4988 + 0x1f + iVar17) + 0x83);
  uVar12 = (unsigned int)uVar18;
  iVar15u = uVar12 * 0x23;
  if ((unsigned short)param_2 == uVar18) {
    return;
  }
  iVar5 = (param_2 & 0xffff) * 0x23;
  iVar4 = PTR(PTR(DAT_006d4988 + 0x1f + iVar5) + 0xa7);
  pbVar2 = iVar4 + 0x1e6 +
           (unsigned char)DAT_0070ad48[(unsigned int)DAT_006d4988[0x16 + iVar15u] * 0x566];
  if (*pbVar2 == 0) {
    *pbVar2 = 1;
    iVar14p = DAT_006d4988 + iVar5;
    uVar11 = (unsigned int)iVar14p[0x16];
    local_18 = uVar11 * 0x126;
    WR16(DAT_006d4a72 + local_18, RD16(DAT_006d4a72 + local_18) + 1);
    local_14 = uVar11 * 0x566;
    if (DAT_0070ad40[local_14] == (char)iVar4[0xb0]) {
      DAT_006d4aa6[local_18] = 1;
    }
    if (((DAT_0085e795 == 2) &&
         (local_18 = (unsigned int)RD16(DAT_0071b73c + DAT_0071ecf4 * 0x5bd),
          (unsigned char)DAT_0070ad48[local_14] == local_18)) &&
        (iVar14p[2] != 0)) {
      if ((int)uVar12 < DAT_006d4980) {
        cVar3 = (char)DAT_006d4988[3 + iVar17];
        if ((cVar3 == -0x70) || (cVar3 == -0x6b)) {
          iVar14 = 0x2b;
        }
        else if ((cVar3 == -0x71) || (cVar3 == -0x6c)) {
          iVar14 = 0x2c;
        }
        else if (cVar3 == -0x69) {
          iVar14 = 0x2d;
        }
        else if (cVar3 == -0x6a) {
          iVar14 = 0x2e;
        }
        else {
          cVar3 = (char)DAT_006d4988[2 + iVar15u];
          if (cVar3 == 0) {
            iVar14 = 0x19;
          }
          else {
            if (cVar3 != 4) goto LAB_00410a3a;
            iVar14 = 0x1a;
          }
        }
      }
      else {
LAB_00410a3a:
        iVar14 = 0x18;
      }
      FUN_0040c070(1, iVar14, param_2 & 0xffff, 0xffff);
    }
  }
  iVar14 = (int)RD32(DAT_006d4988 + 0x1b + iVar15u);
  if (iVar14 != -1) {
    if ((*pbVar2 & 0x80) == 0) {
      uVar8 = (unsigned short)DAT_006d4988[0x16 + iVar5];
      local_18 = (local_18 & 0xffff0000u) | uVar8;
      local_14 = (unsigned int)(DAT_0070ad40[(unsigned int)uVar8 * 0x566] == (char)iVar4[0xb0]);
      FUN_004a1590(3, local_18, iVar14, 1, local_14,
                   (unsigned int)RD16(DAT_0071b73c + iVar14 * 0x5bd));
      FUN_004a1590(3, (unsigned int)DAT_006d4988[0x16 + iVar5], -1, 1, local_14,
                   (unsigned int)RD16(DAT_0071b73c +
                                      (int)RD32(DAT_006d4988 + 0x1b + iVar15u) * 0x5bd));
    }
    *pbVar2 = *pbVar2 | 0x80;
  }
  if ((short)RD16(iVar4 + 0x6a) == -1) {
    if (DAT_006d4980 <= (int)uVar12) goto LAB_00410c5c;
    pEnt = DAT_006d4988 + iVar15u;
    cVar3 = (char)pEnt[2];
    if (((cVar3 == 4) || (cVar3 == 5)) || (cVar3 == 3)) goto LAB_00410c5c;
    bVar6 = 1;
    if (((int)RD32(pEnt + 0x1b) != -1) &&
        (bVar7 = FUN_004440b0((unsigned int)PTR(DAT_006d4988 + 0x1f + iVar5)[0x87],
                              (unsigned int)PTR(pEnt + 0x1f)[0x87]),
         bVar7 == 0) &&
        (DAT_006d4988[2 + iVar5] != 0)) {
      bVar16 = (*pbVar2 & 0x70) >> 4;
      if (bVar16 < 5) {
        bVar16 = bVar16 + (((DAT_00525bf7[DAT_006d4988[3 + iVar17]] == 0) - 1U) & 4);
        if (7 < bVar16) {
          bVar16 = 7;
        }
        *pbVar2 = bVar16 << 4 | (*pbVar2 & 0x8f);
      }
      if (bVar16 < 7) {
        bVar6 = 0;
      }
    }
    if (!bVar6) goto LAB_00410c5c;
  }
  else if ((int)RD32(DAT_006d4988 + 0x1b + iVar5) == -1) goto LAB_00410c5c;
  WR16(iVar4 + 0x6a, uVar18);
  iVar15 = FUN_004a18c0(DAT_0071f2d8[3], DAT_0071f2dc, DAT_0071f2dd);
  WR16(iVar4 + 0x6c, (short)iVar15);
LAB_00410c5c:
  WR16(iVar4 + 0x6e, uVar18);
  iVar4[0x79] = iVar4[0x79] + 1;
  pEnt = DAT_006d4988 + iVar5;
  if (PTR(pEnt + 0x1f)[0x90] != 0) {
    FUN_00483410((short)RD16(pEnt + 0x12), (short)RD16(pEnt + 0x10), (int)pEnt);
    FUN_00483540((short)RD16(DAT_006d4988 + iVar5 + 0x14), 0, (int)(DAT_006d4988 + iVar5));
  }
  {
    unsigned char *pIn = PTR(DAT_006d4988 + 0x1f + iVar5);
    iVar15 = FUN_00413d00((int)(short)RD16(pIn + 0x91), (int)(short)RD16(pIn + 0x93),
                          (int)(short)RD16(pIn + 0x95),
                          (int)(short)((short)DAT_006d4974 - (short)DAT_0071e6c4),
                          (int)(short)((short)DAT_006d4978 - (short)DAT_00732f38),
                          (int)(short)((short)DAT_006d4970 - (short)DAT_00732fe8));
  }
  if ((int)RD32(DAT_006d4988 + 0x1b + iVar5) == -1) {
    local_14 = (local_14 >> 16) << 0x10;
  }
  else {
    local_14 = (local_14 & 0xffff0000u) | (unsigned short)(-1 < (short)iVar15);
  }
  if (((DAT_007092df != 0) && (iVar4[7] == 0)) &&
      (DAT_00525bf7[DAT_006d4988[3 + iVar17]] != 0)) {
    FUN_0040f260((unsigned short)param_2, (unsigned short)param_1);
  }
  bVar16 = DAT_006d4988[3 + iVar17];
  if (bVar16 == 0x98) {
    uVar18 = RD16(iVar4 + 0x1e1);
    if ((int)RD32(DAT_006d4988 + iVar5 + 0x1b) == -1) {
      cVar3 = (char)DAT_006d4988[iVar5 + 2];
      if (((cVar3 == 0) || (cVar3 == 1)) || (cVar3 == 2)) {
        sVar9 = (short)(uVar18 + 0xdd4);
      }
      else {
        sVar9 = (short)(uVar18 + 0x1ba8);
      }
      WR16(iVar4 + 0x1e1, sVar9);
    }
    else {
      uVar8 = 0;
      if (iVar4[0x213] != 0) {
        do {
          uVar13 = uVar8 + 1;
          iVar4[0x345 + (unsigned int)uVar8 * 4] = 0;
          uVar8 = uVar13;
        } while (uVar13 < iVar4[0x213]);
      }
      if ((RD16(iVar4 + 0x1df) & 0x10) == 0) {
        FUN_004a1d70(0x111, (int)RD32(DAT_006d4988 + 0x1b + iVar5));
      }
      else {
        WR16(iVar4 + 0x1df, RD16(iVar4 + 0x1df) & 0x3ef);
        WR16(iVar4 + 0x2a4, 0);
        WR16(iVar4 + 0x2ba, DAT_00523b9e);
        if (((int)RD32(DAT_006d4988 + 0x1b + iVar5) == DAT_0071ecf4) &&
            (DAT_0071b741[DAT_0071ecf4 * 0x5bd] == 0)) {
          _DAT_00732950 = 0x5b;
          _DAT_00732952 = 0x57;
          FUN_004a1d70(0x56, DAT_0071ecf4);
        }
      }
    }
    if (RD16(iVar4 + 0x1e1) < uVar18) {
      WR16(iVar4 + 0x1e1, 0xffff);
    }
    cStack_1c = 1;
  }
  else {
    cStack_1c = 0;
    if (((iVar4[0x23c] == 1) && (RD16(iVar4 + 0x23e) != 0)) &&
        ((DAT_00525bf7[bVar16] != 0) && (-1 < (short)iVar15))) {
      cStack_1c = 1;
      FUN_004a1d70(0x167, (int)RD32(DAT_006d4988 + 0x1b + iVar5));
    }
    if (cStack_1c == 0) {
      uVar12 = FUN_004110e0(param_2, param_3, param_1, local_14, unaff_EBP, unaff_DI, unaff_ESI,
                            unaff_BX, ((unsigned int)uVar18 << 16) | in_stack_ffffffe0);
      cStack_1c = (char)uVar12;
    }
  }
  WR32(DAT_006d4988 + 4 + iVar17, (unsigned int)DAT_0070a17c + (unsigned int)DAT_0071e6c4);
  WR32(DAT_006d4988 + 8 + iVar17, (unsigned int)DAT_00732f38 + (unsigned int)DAT_007091a8);
  WR32(DAT_006d4988 + 0xc + iVar17, (unsigned int)DAT_00732fe8 + (unsigned int)DAT_006ffbe0);
  bVar16 = DAT_006d4988[3 + iVar17];
  puVar1 = DAT_006d4988 + 3 + iVar17;
  if (DAT_00525bf7[bVar16] == 0) {
    if ((bVar16 == 0x8d) || (bVar16 == 0x8e)) {
      *puVar1 = 0x84;
    }
    else {
      *puVar1 = 0x83;
    }
  }
  else {
    *puVar1 = 0x81;
  }
  DAT_006d4988[2 + iVar17] = 0xd;
  *PTR(DAT_006d4988 + 0x1f + iVar17) = 5;
  DAT_006d4988[0x19 + iVar17] = 2;
  WR16(PTR(DAT_006d4988 + 0x1f + iVar17) + 0x81, 0);
  WR16(PTR(DAT_006d4988 + 0x1f + iVar17) + 0x7f, 0);
  PTR(DAT_006d4988 + 0x1f + iVar17)[1] = 0;
  WR16(PTR(DAT_006d4988 + 0x1f + iVar17) + 0x79, RD16(PTR(DAT_006d4988 + 0x1f + iVar5) + 0x79));
  WR16(DAT_006d4988 + 0x12 + iVar17, RD16(DAT_006d4988 + 0x12 + iVar5));
  WR16(DAT_006d4988 + 0x10 + iVar17, RD16(DAT_006d4988 + 0x10 + iVar5));
  WR16(DAT_006d4988 + 0x14 + iVar17, 0);
  PTR(DAT_006d4988 + 0x1f + iVar17)[0x90] = 1;
  {
    unsigned char *pIn2 = PTR(DAT_006d4988 + 0x1f + iVar17);
    pIn2[0x89] = pIn2[0x90];
  }
  if (cStack_1c != 0) {
    if ((int)RD32(DAT_006d4988 + 0x1b + iVar5) == DAT_0071ecf4) {
      FUN_0040a1a0(0x1d, uVar10, DAT_0071ecf4);
      return;
    }
    cVar3 = (char)DAT_006d4988[3 + iVar17];
    if ((cVar3 != -0x7d) && (cVar3 != -0x7c)) {
      iVar15 = DAT_0071ecf4;
      uVar12 = FUN_004784d0();
      FUN_0040a1a0((int)((uVar12 & 3) + 0x16), uVar10, iVar15);
      return;
    }
    FUN_0040a1a0(0x1c, uVar10, DAT_0071ecf4);
  }
  return;
}
#pragma clang diagnostic pop
