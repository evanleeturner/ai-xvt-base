/* SPDX-License-Identifier: MIT */
/* z_xvt 00495010 */
/* FINDINGS (unresolved evidence):
 *  - The 55-byte save/restore copies target a separate stack buffer (local_38_buf, 56 bytes);
 *    its true frame offset is not in the evidence. local_38 stays a scalar pointer
 *    (FUN_0042b150 result), and (int)local_38 + 10 writes into that returned object.
 *  - extraout_ECX/EDX and local_38 are zero-initialised only to satisfy the build.
 *  - FUN_004b1c00 takes 2 stack args (8 bytes) per the stack check; Ghidra's extra
 *    unaff_* arguments were register leftovers and are dropped.
 *  - DAT_006db580 is assumed unsigned.
 *  - LD/ST are GNU statement-expression macros around __builtin_memcpy for misaligned
 *    multi-byte accesses; they add call-like tokens the original lacks.
 */

#ifndef CONCAT22
#define CONCAT22(a,b) ((((uint)(ushort)(a)) << 16) | (uint)(ushort)(b))
#endif
#ifndef CONCAT31
#define CONCAT31(a,b) ((((uint)(a)) << 8) | (uint)(byte)(b))
#endif
#ifndef CONCAT12
#define CONCAT12(a,b) ((((uint)(byte)(a)) << 16) | (uint)(ushort)(b))
#endif

#define CAT_(a,b) a##b
#define LDX(T,p,n) ({ T CAT_(_v,n); __builtin_memcpy(&CAT_(_v,n), (const void *)(p), sizeof(T)); CAT_(_v,n); })
#define LD(T,p) LDX(T,p,__COUNTER__)
#define ST(T,p,v) do { T _sv = (v); __builtin_memcpy((void *)(p), &_sv, sizeof(_sv)); } while (0)
/* DAT_006d9d56 is a pointer stored at the unaligned address 0x6d9d56 */
#define P56 LD(byte *, &DAT_006d9d56)

extern byte *DAT_006d9d56;
extern byte *DAT_006d4988;
extern byte *DAT_00732e54;
extern byte *DAT_0071ecf4;
extern ushort DAT_006d9d50;
extern ushort DAT_006d9d60;
extern ushort DAT_006d9d62;
extern uint DAT_006db580;
extern int DAT_00732a2c;
extern int DAT_00732a28;
extern int DAT_00732a24;
extern int DAT_0070ab18;
extern int DAT_0070ab14;
extern int DAT_0070ac20;
extern ushort DAT_00732fe2;
extern short DAT_00655a84;
extern byte DAT_0070ad94;
extern byte DAT_0070ad95;
extern byte DAT_0070ad92;
extern byte DAT_0070ad47;
extern byte DAT_0070ad45;
extern byte DAT_0070ad44;
extern byte DAT_0070ad40;
extern byte DAT_0052b141;
extern byte DAT_0052b143;
extern byte DAT_0052b145;
extern byte DAT_0052b147;
extern byte DAT_0052b149;
extern byte DAT_0052b0b4;
extern byte DAT_0052b0b6;
extern byte DAT_0052b0ba;
extern byte DAT_0052b13c;
extern byte DAT_00524880;
extern byte DAT_00524798;
extern byte DAT_005247c0;
extern byte DAT_006da3e0;
extern byte DAT_006d9990;
extern byte DAT_006d9cd8;
extern byte DAT_00703ca0;
extern byte DAT_006d4a9e;
extern byte DAT_006d4abc;
extern byte DAT_006d4a76;
extern byte DAT_006d4a7a;
extern byte DAT_006d4a7e;
extern byte DAT_006d4a82;
extern byte DAT_006d4aaa;
extern byte DAT_006d4aac;
extern byte DAT_006d4aae;
extern byte DAT_0071b730;
extern byte DAT_0071b74a;
extern byte DAT_0071bb37;
extern char DAT_006d4870;
extern char DAT_0071e590;
extern char s_boardtogivepln_00523c18[];
extern char s_boardtoexchangepln_00524f58[];
extern char s_boardtocontactpln_00524f30[];
extern char s_boardtopickuppln_00524f44[];
extern char s_boardtorepairpln_00524f1c[];
extern char s_boardtocapturepln_00524f90[];
extern char s_boardtotakepln_00524f6c[];
extern char s_boardtodestroypln_00524f7c[];
extern char s_selfdestroypln_00528360[];
extern char s_stationaryldrpln_00524934[];
extern char s_flyhomeevadepln_00525b50[];

uint FUN_0042b150(int);
void FUN_00421c70(uint, int);
void FUN_0042a230(int, uint, int, int, short, short);
void FUN_004979e0(int, int);
void FUN_004a1d70(int, int);
uint FUN_0040a1a0(int, int, int);
void FUN_004a3e40(uint, int, char *);
void FUN_004a3490(int, char *);
uint FUN_0040c070(int, int, uint, uint);
uint FUN_00478320(uint, ushort);
void FUN_004a6a80(void);
void FUN_004b1c00(uint, int);
void FUN_004a6f90(void);
void FUN_004a34c0(uint, undefined4 *, int);
void FUN_00496770(uint, int, int);
int FUN_00481380(char *);
void FUN_004296b0(uint);
void FUN_00429310(uint);
void FUN_004a01a0(uint, uint);
uint FUN_00478470(void);

uint __stdcall caseD_12(void)
{
  byte bVar1;
  byte bVar2;
  undefined2 uVar3;
  byte *pbVar4;
  ushort uVar5;
  ushort uVar6;
  ushort uVar7;
  short sVar8;
  byte *pbVar9;
  uint uVar10;
  uint uVar11;
  byte bVar12;
  byte *pbVar13;
  int iVar14;
  int iVar15;
  uint uVar16;
  byte *extraout_ECX = 0;
  undefined2 uVar17;
  int extraout_EDX = 0;
  int iVar18;
  undefined4 *puVar19;
  char *pcVar20;
  undefined4 *puVar21;
  char *pcVar22;
  undefined4 *puVar23;
  byte bVar24 = 0;
  char *pcVar25;
  undefined2 local_4c;
  ushort local_48;
  short sStack_46;
  ushort local_44;
  int local_3c;
  undefined4 *local_38 = 0;
  undefined4 local_38_buf[14];

  uVar5 = LD(ushort, P56 + 0x18);
  iVar15 = (uint)DAT_006d9d62 * 0x52;
  bVar12 = (&DAT_0070ad94)[(uint)DAT_006d9d60 * 0x566 + iVar15];
  uVar6 = (ushort)bVar12;
  iVar14 = ((uint)uVar5 * 8 - (uint)uVar5) * 5;
  pbVar13 = DAT_006d4988 + iVar14;
  if (LD(int, pbVar13 + 0x1f) == 0) {
    bVar1 = pbVar13[0x16];
    uVar3 = LD(undefined2, pbVar13);
    local_48 = 0xff;
    puVar19 = local_38;
  }
  else {
    puVar19 = LD(undefined4 *, (byte *)LD(int, pbVar13 + 0x1f) + 0xa7);
    local_38 = (undefined4 *)FUN_0042b150((int)puVar19);
    local_48 = (ushort)*(byte *)(puVar19 + 1);
    bVar1 = (DAT_006d4988 + iVar14)[0x16];
    uVar3 = LD(undefined2, DAT_006d4988 + iVar14);
    iVar15 = extraout_EDX;
  }
  pbVar13 = DAT_006d4988;
  local_44 = (ushort)bVar1;
  pbVar9 = P56;
  switch (P56[0x35]) {
  case 0:
    pbVar13 = DAT_006d4988 + iVar14;
    if (LD(int, pbVar13 + 0x1f) == 0) {
      FUN_00421c70((uint)uVar5, 0);
      ST(int, P56 + 0x1f, DAT_00732a2c);
      ST(int, P56 + 0x23, DAT_00732a28);
      iVar15 = (int)((uint)DAT_00732a24 + 0x800);
    }
    else {
      iVar15 = (uint)local_48 * 0xef;
      if ((pbVar13[2] == 0) || (pbVar13[2] == 1)) {
        sVar8 = (short)((uint)LD(short, &DAT_0052b143 + iVar15) -
                        (uint)LD(short, &DAT_0052b147 + (uint)DAT_00732e54[4] * 0xef));
      }
      else if ((DAT_006d4988[(uint)DAT_006d9d50 * 0x23 + 2] == 0) ||
               (DAT_006d4988[(uint)DAT_006d9d50 * 0x23 + 2] == 1)) {
        sVar8 = (short)(((uint)LD(short, &DAT_0052b143 + iVar15) -
                         (uint)LD(short, &DAT_0052b149 + (uint)DAT_00732e54[4] * 0xef)) +
                        (uint)LD(short, &DAT_0052b145 + iVar15));
      }
      else {
        sVar8 = (short)((uint)LD(short, &DAT_0052b145 + iVar15) * 2u -
                        (uint)LD(short, &DAT_0052b149 + (uint)DAT_00732e54[4] * 0xef));
      }
      sVar8 = (short)((uint)sVar8 +
                      ((uint)LD(short, &DAT_0052b145 + iVar15) -
                       (uint)LD(short, &DAT_0052b149 + (uint)DAT_00732e54[4] * 0xef)) * 2u);
      if (sVar8 < 0) {
        sVar8 = 0x7000;
      }
      FUN_0042a230((uint)DAT_00732e54[4] * 0xef,
                   (uint)(ushort)(LD(short, &DAT_0052b145 + iVar15) -
                                  LD(short, &DAT_0052b149 + (uint)DAT_00732e54[4] * 0xef)),
                   (int)pbVar13, 0, sVar8, LD(short, &DAT_0052b141 + iVar15));
      ST(int, P56 + 0x1f, (int)((uint)LD(int, DAT_006d4988 + iVar14 + 4) + (uint)DAT_0070ab18));
      ST(int, P56 + 0x23, (int)((uint)LD(int, DAT_006d4988 + iVar14 + 8) + (uint)DAT_0070ab14));
      iVar15 = (int)((uint)LD(int, DAT_006d4988 + iVar14 + 0xc) + (uint)DAT_0070ac20);
    }
    ST(int, P56 + 0x27, iVar15);
    FUN_004979e0(0, 1);
    if (0x4000 < DAT_006db580) {
      pbVar13 = DAT_00732e54;
      pbVar13[0xbd] = 0xff;
      pbVar13[0xbe] = 0xff;
    }
    pbVar13 = DAT_00732e54;
    if (0x2000 < DAT_006db580) {
      pbVar13[0xbd] = 0;
      pbVar13[0xbe] = 0x80;
      return (uint)pbVar13 & 0xffff0000;
    }
    if (0x800 < DAT_006db580) {
      pbVar13[0xbd] = 0;
      pbVar13[0xbe] = 0x40;
      return (uint)pbVar13 & 0xffff0000;
    }
    if (((LD(byte *, DAT_006d4988 + iVar14 + 0x1b) == (byte *)0xffffffff) ||
         (LD(int, DAT_006d4988 + iVar14 + 0x1f) == 0)) ||
        (LD(short, (byte *)LD(int, DAT_006d4988 + iVar14 + 0x1f) + 0x79) == 0)) {
      pbVar13[0xbd] = 0;
      pbVar13[0xbe] = 0;
      P56[0x35] = 1;
      return (uint)pbVar13 & 0xffff0000;
    }
    pbVar9 = DAT_0071ecf4;
    if (DAT_0071ecf4 == LD(byte *, DAT_006d4988 + iVar14 + 0x1b)) {
      uVar11 = 0;
      pbVar9 = (byte *)&DAT_006d9990;
      do {
        if (*(short *)pbVar9 == 0x103) break;
        pbVar9 = pbVar9 + 0x54;
        uVar11 = uVar11 + 1;
      } while (pbVar9 < &DAT_006d9cd8);
      if (9 < uVar11) {
        DAT_00732fe2 = (ushort)(byte)(&DAT_0070ad47)[(uint)DAT_006d9d60 * 0x566];
        FUN_004a1d70(0x103, (int)DAT_0071ecf4);
        uVar11 = FUN_0040a1a0(0x24, 0xffffffff, (int)DAT_0071ecf4);
        return uVar11 & 0xffff0000;
      }
    }
    break;
  case 1:
    pbVar13 = DAT_006d4988 + iVar14;
    if (LD(int, pbVar13 + 0x1f) == 0) {
      FUN_00421c70((uint)uVar5, 0);
      ST(int, DAT_00732e54 + 0xb1,
         (int)((uint)DAT_00732a2c - (uint)LD(int, DAT_006d4988 + (uint)DAT_006d9d50 * 0x23 + 4)));
      local_3c = LD(int, DAT_00732e54 + 0xb1);
      ST(int, DAT_00732e54 + 0xb5,
         (int)((uint)DAT_00732a28 - (uint)LD(int, DAT_006d4988 + (uint)DAT_006d9d50 * 0x23 + 8)));
      iVar15 = LD(int, DAT_00732e54 + 0xb5);
      local_48 = 0;
      sStack_46 = 0;
      ST(int, DAT_00732e54 + 0xb9,
         (int)(((uint)DAT_00732a24 - (uint)LD(int, DAT_006d4988 + (uint)DAT_006d9d50 * 0x23 + 0xc)) + 0x80));
      local_4c = 0x4000;
      iVar18 = LD(int, DAT_00732e54 + 0xb9);
    }
    else {
      iVar15 = (uint)local_48 * 0xef;
      if ((pbVar13[2] == 0) || (pbVar13[2] == 1)) {
        iVar18 = (uint)DAT_00732e54[4] * 0xef;
        uVar11 = (uint)(ushort)(LD(short, &DAT_0052b143 + iVar15) -
                                LD(short, &DAT_0052b147 + iVar18));
      }
      else if ((DAT_006d4988[(uint)DAT_006d9d50 * 0x23 + 2] == 0) ||
               (DAT_006d4988[(uint)DAT_006d9d50 * 0x23 + 2] == 1)) {
        iVar18 = (uint)DAT_00732e54[4] * 0xef;
        uVar11 = (uint)(ushort)(LD(short, &DAT_0052b143 + iVar15) -
                                LD(short, &DAT_0052b149 + iVar18));
      }
      else {
        iVar18 = (uint)DAT_00732e54[4] * 0xef;
        uVar11 = (uint)(ushort)(LD(short, &DAT_0052b145 + iVar15) -
                                LD(short, &DAT_0052b149 + iVar18));
      }
      FUN_0042a230(iVar18, uVar11, (int)pbVar13, 0, (short)uVar11,
                   LD(short, &DAT_0052b141 + iVar15));
      ST(int, DAT_00732e54 + 0xb1,
         (int)(((uint)LD(int, DAT_006d4988 + iVar14 + 4) -
                (uint)LD(int, DAT_006d4988 + (uint)DAT_006d9d50 * 0x23 + 4)) + (uint)DAT_0070ab18));
      local_3c = LD(int, DAT_00732e54 + 0xb1);
      ST(int, DAT_00732e54 + 0xb5,
         (int)(((uint)LD(int, DAT_006d4988 + iVar14 + 8) -
                (uint)LD(int, DAT_006d4988 + (uint)DAT_006d9d50 * 0x23 + 8)) + (uint)DAT_0070ab14));
      iVar15 = LD(int, DAT_00732e54 + 0xb5);
      ST(int, DAT_00732e54 + 0xb9,
         (int)(((uint)LD(int, DAT_006d4988 + iVar14 + 0xc) -
                (uint)LD(int, DAT_006d4988 + (uint)DAT_006d9d50 * 0x23 + 0xc)) + (uint)DAT_0070ac20));
      iVar18 = LD(int, DAT_00732e54 + 0xb9);
      local_48 = LD(ushort, DAT_006d4988 + iVar14 + 0x14);
      sStack_46 = LD(short, DAT_006d4988 + iVar14 + 0x10);
      local_4c = LD(undefined2, DAT_006d4988 + iVar14 + 0x12);
    }
    if (LD(short, DAT_006d4988 + (uint)DAT_006d9d50 * 0x23 + 0x14) != local_48) {
      DAT_00732e54[0xa4] = 1;
      pbVar13 = DAT_00732e54;
      pbVar13[0xa5] = 0;
      pbVar13[0xa6] = 0x80;
      ST(ushort, P56 + 0x30, local_48);
    }
    local_38 = (undefined4 *)(uint)DAT_006d9d50;
    if (LD(short, DAT_006d4988 + (uint)local_38 * 0x23 + 0x10) != sStack_46) {
      DAT_00732e54[0xab] = 2;
      pbVar13 = DAT_00732e54;
      pbVar13[0xac] = 0;
      pbVar13[0xad] = 0x80;
      ST(short, P56 + 0x32, sStack_46);
    }
    if (LD(short, DAT_006d4988 + (uint)DAT_006d9d50 * 0x23 + 0x12) !=
        LD(short, DAT_006d4988 + iVar14 + 0x12)) {
      ST(undefined2, P56 + 0x2e, local_4c);
      pbVar13 = DAT_00732e54;
      pbVar13[0x9e] = 0;
      pbVar13[0x9f] = 0x80;
      DAT_00732e54[0x9d] = 0;
      if (LD(ushort, DAT_00732e54 + 0xd) < LD(ushort, P56 + 0x2e)) {
        DAT_00732e54[0x9c] = 2;
      }
      else {
        DAT_00732e54[0x9c] = 1;
      }
    }
    if (local_3c < 0) {
      local_3c = (int)(0u - (uint)local_3c);
    }
    if (iVar15 < 0) {
      iVar15 = (int)(0u - (uint)iVar15);
    }
    if (iVar18 < 0) {
      iVar18 = (int)(0u - (uint)iVar18);
    }
    pbVar9 = (byte *)((uint)iVar15 + (uint)iVar18 + (uint)local_3c);
    if (0xf < (int)pbVar9) break;
    pbVar13 = DAT_00732e54;
    pbVar13[0xb1] = 0;
    pbVar13[0xb2] = 0;
    pbVar13[0xb3] = 0;
    pbVar13[0xb4] = 0;
    pbVar13 = DAT_00732e54;
    pbVar13[0xb5] = 0;
    pbVar13[0xb6] = 0;
    pbVar13[0xb7] = 0;
    pbVar13[0xb8] = 0;
    pbVar13 = DAT_00732e54;
    pbVar13[0xb9] = 0;
    pbVar13[0xba] = 0;
    pbVar13[0xbb] = 0;
    pbVar13[0xbc] = 0;
    P56[0x35] = 2;
    ST(uint, P56 + 0x36, (uint)uVar6 * 0x49c);
    pbVar13 = P56;
    pbVar13[0x3a] = 0xec;
    pbVar13[0x3b] = 0;
    if ((LD(int, DAT_006d4988 + (uint)DAT_006d9d50 * 0x23 + 0x1f) != 0) &&
        (DAT_00732e54[0x7c] == 0)) {
      DAT_00732e54[0x7c] = 1;
      uVar11 = (uint)DAT_006d9d60;
      iVar15 = uVar11 * 0x126;
      *(short *)(&DAT_006d4a9e + iVar15) = *(short *)(&DAT_006d4a9e + iVar15) + 1;
      if ((&DAT_0070ad40)[uVar11 * 0x566] == DAT_00732e54[0xb0]) {
        (&DAT_006d4abc)[iVar15] = 1;
      }
    }
    pbVar13 = DAT_006d4988;
    if (LD(int, DAT_006d4988 + iVar14 + 0x1f) != 0) {
      pbVar13 = (byte *)(((uint)DAT_006d4988 & 0xffffff00u) | (uint)*(byte *)((int)puVar19 + 0x7a));
      if (*(char *)((int)puVar19 + 0x7a) == '\0') {
        *(byte *)((int)puVar19 + 0x7a) = 1;
        pbVar13 = (byte *)((uint)local_44 * 0x126);
        *(short *)((uint)pbVar13 + 0x6d4a9c) = *(short *)((uint)pbVar13 + 0x6d4a9c) + 1;
        if ((&DAT_0070ad40)[(uint)local_44 * 0x566] == *(char *)(puVar19 + 0x2c)) {
          *(byte *)((uint)pbVar13 + 0x6d4abb) = 1;
        }
      }
    }
    FUN_004a3e40(CONCAT22((short)((uint)pbVar13 >> 0x10), DAT_006d9d50), 1, (char *)&DAT_006d4870);
    FUN_004a3490(0, &DAT_006d4870);
    FUN_004a3e40(CONCAT22(uVar5, uVar5), 1, &DAT_0071e590);
    FUN_004a3490(1, &DAT_0071e590);
    DAT_00732fe2 = (ushort)*(byte *)((byte *)LD(int, DAT_006d4988 + (uint)DAT_006d9d50 * 0x23 + 0x1f) + 0x86);
    FUN_004a1d70(0xe9, (int)DAT_0071ecf4);
    bVar24 = uVar6 == 2;
    if (uVar6 < 2) goto LAB_00495a8f;
    pcVar25 = (char *)((int)&DAT_00703ca0 + (uint)P56[0xb] * 0x55);
    iVar14 = 0xf;
    pcVar20 = pcVar25;
    pcVar22 = s_boardtogivepln_00523c18;
    do {
      if (iVar14 == 0) break;
      iVar14 = iVar14 + -1;
      bVar24 = *pcVar20 == *pcVar22;
      pcVar20 = pcVar20 + 1;
      pcVar22 = pcVar22 + 1;
    } while (bVar24);
    if (bVar24) {
LAB_00495a75:
      iVar14 = 0x25;
    }
    else {
      iVar14 = 0x13;
      bVar24 = 0;
      pcVar20 = pcVar25;
      pcVar22 = s_boardtoexchangepln_00524f58;
      do {
        if (iVar14 == 0) break;
        iVar14 = iVar14 + -1;
        bVar24 = *pcVar20 == *pcVar22;
        pcVar20 = pcVar20 + 1;
        pcVar22 = pcVar22 + 1;
      } while (bVar24);
      if (bVar24) goto LAB_00495a75;
      iVar14 = 0x12;
      bVar24 = 0;
      pcVar20 = pcVar25;
      pcVar22 = s_boardtocontactpln_00524f30;
      do {
        if (iVar14 == 0) break;
        iVar14 = iVar14 + -1;
        bVar24 = *pcVar20 == *pcVar22;
        pcVar20 = pcVar20 + 1;
        pcVar22 = pcVar22 + 1;
      } while (bVar24);
      if (bVar24) goto LAB_00495a75;
      iVar14 = 0x11;
      bVar24 = 0;
      pcVar20 = pcVar25;
      pcVar22 = s_boardtopickuppln_00524f44;
      do {
        if (iVar14 == 0) break;
        iVar14 = iVar14 + -1;
        bVar24 = *pcVar20 == *pcVar22;
        pcVar20 = pcVar20 + 1;
        pcVar22 = pcVar22 + 1;
      } while (bVar24);
      if (bVar24) goto LAB_00495a75;
      iVar14 = 0x11;
      bVar24 = 0;
      pcVar20 = pcVar25;
      pcVar22 = s_boardtorepairpln_00524f1c;
      do {
        if (iVar14 == 0) break;
        iVar14 = iVar14 + -1;
        bVar24 = *pcVar20 == *pcVar22;
        pcVar20 = pcVar20 + 1;
        pcVar22 = pcVar22 + 1;
      } while (bVar24);
      if (bVar24) goto LAB_00495a75;
      iVar14 = 0x12;
      bVar24 = 0;
      pcVar20 = pcVar25;
      pcVar22 = s_boardtocapturepln_00524f90;
      do {
        if (iVar14 == 0) break;
        iVar14 = iVar14 + -1;
        bVar24 = *pcVar20 == *pcVar22;
        pcVar20 = pcVar20 + 1;
        pcVar22 = pcVar22 + 1;
      } while (bVar24);
      if (!bVar24) {
        iVar14 = 0xf;
        pcVar20 = pcVar25;
        pcVar22 = s_boardtotakepln_00524f6c;
        do {
          if (iVar14 == 0) break;
          iVar14 = iVar14 + -1;
          bVar24 = *pcVar20 == *pcVar22;
          pcVar20 = pcVar20 + 1;
          pcVar22 = pcVar22 + 1;
        } while (bVar24);
        if (!bVar24) {
          iVar14 = 0x12;
          pcVar20 = s_boardtodestroypln_00524f7c;
          do {
            if (iVar14 == 0) break;
            iVar14 = iVar14 + -1;
            bVar24 = *pcVar25 == *pcVar20;
            pcVar25 = pcVar25 + 1;
            pcVar20 = pcVar20 + 1;
          } while (bVar24);
          if (!bVar24) goto LAB_00495a8f;
        }
      }
      iVar14 = 0x27;
    }
    FUN_0040c070(1, iVar14, (uint)DAT_006d9d50, 0xffff);
LAB_00495a8f:
    uVar11 = FUN_0040c070(1, 0x1e, (uint)uVar5, 0xffff);
    return uVar11 & 0xffff0000;
  case 2:
    bVar24 = LD(int, P56 + 0x36) == 0;
    if (!bVar24) {
      iVar15 = 0xf;
      pcVar25 = (char *)((int)&DAT_00703ca0 + (uint)P56[0xb] * 0x55);
      pcVar20 = s_boardtogivepln_00523c18;
      do {
        if (iVar15 == 0) break;
        iVar15 = iVar15 + -1;
        bVar24 = *pcVar25 == *pcVar20;
        pcVar25 = pcVar25 + 1;
        pcVar20 = pcVar20 + 1;
      } while (bVar24);
      pbVar9 = (byte *)0x0;
      if (bVar24) {
        pbVar9 = DAT_006d4988;
        if (LD(int, DAT_006d4988 + iVar14 + 0x1b) != -1) {
          pbVar9 = P56;
          if (LD(short, P56 + 0x3a) == 0) {
            uVar11 = 0;
            bVar24 = 0;
            if (*(char *)(puVar19 + 0x8a) != '\0') {
              do {
                if (*(char *)(uVar11 + 0x229 + (int)puVar19) != '\0') {
                  iVar15 = (uint)local_48 * 0xef + uVar11;
                  bVar1 = (&DAT_0052b0b6)[uVar11 + (uint)local_48 * 0xef];
                  for (uVar5 = (ushort)(byte)(&DAT_0052b0b4)[iVar15]; uVar5 <= bVar1;
                       uVar5 = uVar5 + 1) {
                    uVar10 = (uint)(byte)(&DAT_0070ad45)[(uint)DAT_006d4988[iVar14 + 0x16] * 0x566];
                    if ((short)uVar11 == 1) {
                      uVar10 = 5;
                    }
                    uVar10 = FUN_00478320(
                        CONCAT31(((uint)DAT_006d4988[iVar14 + 0x16] * 0x2b3) >> 8,
                                 (&DAT_0052b0ba)[iVar15]) & 0xffff00ff,
                        *(ushort *)(&DAT_00524880 + uVar10 * 2));
                    if ((short)uVar10 == 0) {
                      uVar10 = 1;
                    }
                    if ((&DAT_0070ad44)[(uint)DAT_006d4988[iVar14 + 0x16] * 0x566] == '\x01') {
                      uVar10 = (uint)(ushort)((ushort)uVar10 * 2);
                    }
                    else if ((&DAT_0070ad44)[(uint)DAT_006d4988[iVar14 + 0x16] * 0x566] == '\x02') {
                      uVar10 = (uint)(ushort)((ushort)uVar10 >> 1);
                    }
                    if ((short)uVar10 == 0) {
                      uVar10 = 1;
                    }
                    if (99 < (ushort)uVar10) {
                      uVar10 = 99;
                    }
                    uVar16 = CONCAT12(bVar12, uVar5) & 0xffff;
                    bVar2 = *(byte *)((int)puVar19 + uVar16 * 4 + 0x346);
                    local_38 = (undefined4 *)(uint)bVar2;
                    if (local_38 < (undefined4 *)(uVar10 & 0xffff)) {
                      bVar24 = 1;
                      *(byte *)((int)puVar19 + uVar16 * 4 + 0x346) = bVar2 + 1;
                    }
                    *(byte *)((int)puVar19 + uVar16 * 4 + 0x345) = 0x7f;
                  }
                }
                uVar11 = (uint)(ushort)((short)uVar11 + 1);
              } while (uVar11 < *(byte *)(puVar19 + 0x8a));
            }
            if (*(char *)(puVar19 + 0x8f) != '\0') {
              *(byte *)((int)puVar19 + 0x23d) = (&DAT_0052b13c)[(uint)local_48 * 0xef];
            }
            uVar5 = 1;
            uVar6 = 0;
            do {
              if (((LD(ushort, (byte *)puVar19 + 0x1dd) & uVar5) != 0) &&
                  ((LD(ushort, (byte *)puVar19 + 0x1df) & uVar5) == 0)) {
                bVar24 = 1;
                ST(ushort, (byte *)puVar19 + 0x1df, LD(ushort, (byte *)puVar19 + 0x1df) | uVar5);
                break;
              }
              uVar5 = uVar5 * 2;
              uVar6 = uVar6 + 1;
            } while (uVar6 < 10);
            pbVar9 = P56;
            pbVar13 = P56;
            pbVar13[0x3a] = 0xd8;
            pbVar4 = P56;
            pbVar13[0x3b] = 1;
            if (bVar24) {
              pbVar13 = P56;
              pbVar13[0x36] = 0x88;
              pbVar13[0x37] = 5;
              pbVar13[0x38] = 0;
              pbVar13[0x39] = 0;
              return (uint)pbVar4 & 0xffff0000;
            }
          }
        }
      }
      break;
    }
    pcVar25 = (char *)((int)&DAT_00703ca0 + (uint)P56[0xb] * 0x55);
    iVar15 = 0xf;
    pcVar20 = pcVar25;
    pcVar22 = s_boardtogivepln_00523c18;
    do {
      if (iVar15 == 0) break;
      iVar15 = iVar15 + -1;
      bVar24 = *pcVar20 == *pcVar22;
      pcVar20 = pcVar20 + 1;
      pcVar22 = pcVar22 + 1;
    } while (bVar24);
    if (bVar24) {
      if (LD(int, DAT_006d4988 + iVar14 + 0x1f) != 0) {
        uVar6 = 0;
        do {
          uVar7 = uVar6 + 1;
          *(byte *)(uVar6 + 0x1fb + (int)puVar19) = DAT_00732e54[uVar6 + 0x1fb];
          uVar6 = uVar7;
        } while (uVar7 < 0x10);
        *(byte *)((int)puVar19 + 0x1fa) = 2;
      }
      local_38 = (undefined4 *)(uint)(byte)(&DAT_0070ad95)
                     [(uint)DAT_006d9d60 * 0x566 + (uint)*P56 * 0x52];
      if (local_38 <= (undefined4 *)(P56[*P56 + 5] + 1)) {
        DAT_00732e54[0x1fb] = 0;
      }
      DAT_00732e54[0x1fa] = 1;
      pbVar13 = DAT_006d4988;
      if (LD(byte *, DAT_006d4988 + iVar14 + 0x1b) != (byte *)0xffffffff) {
        pbVar13 = DAT_0071ecf4;
        if (DAT_0071ecf4 == LD(byte *, DAT_006d4988 + iVar14 + 0x1b)) {
          iVar15 = LD(int, (byte *)&DAT_0071b730 + (uint)DAT_0071ecf4 * 0x5bd);
          if (iVar15 == -1) {
LAB_00495e65:
            bVar24 = 0;
          }
          else {
            pbVar13 = (byte *)((uint)iVar15 * 7u);
            bVar12 = DAT_006d4988[(uint)iVar15 * 0x23 + 3];
            if (((bVar12 != 1) && (bVar12 != 2)) &&
                ((bVar12 != 3 && ((bVar12 != 0xe && (bVar12 != 4)))))) goto LAB_00495e65;
            bVar24 = 1;
          }
          if (bVar24) {
            uVar6 = 1;
            sVar8 = 0xd;
            do {
              if (((LD(ushort, (byte *)puVar19 + 0x1d9) & uVar6) != 0) &&
                  ((LD(ushort, (byte *)puVar19 + 0x1db) & uVar6) == 0)) {
                ST(ushort, (byte *)puVar19 + 0x1db,
                   uVar6 | LD(ushort, (byte *)puVar19 + 0x1db));
              }
              uVar6 = uVar6 * 2;
              sVar8 = sVar8 + -1;
            } while (sVar8 != 0);
            FUN_004a6a80();
            FUN_004b1c00((uint)(byte)(&DAT_0071bb37)[(uint)DAT_0071ecf4 * 0x5bd],
                         (int)DAT_0071ecf4);
            FUN_004a6f90();
            pbVar13 = extraout_ECX;
          }
        }
        pbVar13 = (byte *)((uint)pbVar13 & 0xffff0000);
        do {
          uVar11 = (uint)pbVar13 & 0xffff;
          *(char *)(uVar11 + 0x293 + (int)puVar19) = (char)(uint)pbVar13;
          uVar6 = (ushort)((short)(uint)pbVar13 + 1);
          pbVar13 = (byte *)CONCAT22((short)((uint)pbVar13 >> 0x10), uVar6);
          *(undefined2 *)((int)puVar19 + uVar11 * 2 + 0x29e) = 100;
          *(undefined2 *)((int)puVar19 + uVar11 * 2 + 0x2b4) = 0;
        } while (uVar6 < 10);
      }
      FUN_004a34c0(CONCAT22((short)((uint)pbVar13 >> 0x10), DAT_006d9d50),
                   (undefined4 *)DAT_00732e54, 0x9b);
      uVar11 = (uint)DAT_006d9d50;
      iVar15 = 0x26;
LAB_004963af:
      FUN_0040c070(1, iVar15, uVar11, 0xffff);
    }
    else {
      iVar15 = 0xf;
      pcVar20 = pcVar25;
      pcVar22 = s_boardtotakepln_00524f6c;
      do {
        if (iVar15 == 0) break;
        iVar15 = iVar15 + -1;
        bVar24 = *pcVar20 == *pcVar22;
        pcVar20 = pcVar20 + 1;
        pcVar22 = pcVar22 + 1;
      } while (bVar24);
      if (bVar24) {
        uVar17 = 0;
        if (LD(int, DAT_006d4988 + iVar14 + 0x1f) != 0) {
          iVar15 = LD(int, DAT_006d4988 + iVar14 + 0x1f);
          uVar6 = 0;
          do {
            uVar7 = uVar6 + 1;
            bVar12 = *(byte *)(uVar6 + 0x1fb + (int)puVar19);
            uVar17 = (undefined2)((uint)iVar15 >> 0x10);
            DAT_00732e54[uVar6 + 0x1fb] = bVar12;
            iVar15 = CONCAT31(((uint)iVar15) >> 8, bVar12);
            uVar6 = uVar7;
          } while (uVar7 < 0x10);
          *(byte *)((int)puVar19 + 0x1fb) = 0;
          *(byte *)((int)puVar19 + 0x1fa) = 1;
        }
        DAT_00732e54[0x1fa] = 2;
        FUN_004a34c0(CONCAT22(uVar17, DAT_006d9d50), (undefined4 *)DAT_00732e54, 0x9b);
        uVar11 = (uint)DAT_006d9d50;
        iVar15 = 0x28;
        goto LAB_004963af;
      }
      iVar15 = 0x13;
      bVar24 = 0;
      pcVar20 = pcVar25;
      pcVar22 = s_boardtoexchangepln_00524f58;
      do {
        if (iVar15 == 0) break;
        iVar15 = iVar15 + -1;
        bVar24 = *pcVar20 == *pcVar22;
        pcVar20 = pcVar20 + 1;
        pcVar22 = pcVar22 + 1;
      } while (bVar24);
      if (bVar24) {
        uVar17 = 0;
        if (LD(int, DAT_006d4988 + iVar14 + 0x1f) != 0) {
          iVar15 = LD(int, DAT_006d4988 + iVar14 + 0x1f);
          uVar6 = 0;
          do {
            uVar7 = uVar6 + 1;
            uVar11 = (uint)uVar6;
            bVar12 = *(byte *)(uVar11 + 0x1fb + (int)puVar19);
            uVar17 = (undefined2)((uint)iVar15 >> 0x10);
            bVar1 = DAT_00732e54[uVar11 + 0x1fb];
            DAT_00732e54[uVar11 + 0x1fb] = bVar12;
            *(byte *)(uVar11 + 0x1fb + (int)puVar19) = bVar1;
            iVar15 = CONCAT31(((uint)iVar15) >> 8, bVar12);
            uVar6 = uVar7;
          } while (uVar7 < 0x10);
          *(byte *)((int)puVar19 + 0x1fa) = 2;
        }
        DAT_00732e54[0x1fa] = 2;
        FUN_004a34c0(CONCAT22(uVar17, DAT_006d9d50), (undefined4 *)DAT_00732e54, 0x9b);
      }
      else {
        iVar15 = 0x12;
        pcVar20 = pcVar25;
        pcVar22 = s_boardtocapturepln_00524f90;
        do {
          if (iVar15 == 0) break;
          iVar15 = iVar15 + -1;
          bVar24 = *pcVar20 == *pcVar22;
          pcVar20 = pcVar20 + 1;
          pcVar22 = pcVar22 + 1;
        } while (bVar24);
        if (!bVar24) {
          iVar15 = 0x12;
          pcVar20 = pcVar25;
          pcVar22 = s_boardtodestroypln_00524f7c;
          do {
            if (iVar15 == 0) break;
            iVar15 = iVar15 + -1;
            bVar24 = *pcVar20 == *pcVar22;
            pcVar20 = pcVar20 + 1;
            pcVar22 = pcVar22 + 1;
          } while (bVar24);
          if (bVar24) {
            if (LD(int, DAT_006d4988 + iVar14 + 0x1f) != 0) {
              FUN_004a01a0(CONCAT22((short)((uint)DAT_006d4988 >> 0x10), DAT_006d9d50),
                           CONCAT22(uVar5, uVar5));
              iVar15 = LD(int, DAT_006d4988 + iVar14 + 0x1f);
              if (LD(short, (byte *)iVar15 + 0x7f) != 0) {
                uVar6 = FUN_00478470();
                ST(short, (byte *)iVar15 + 0x7f, ((uVar6 & 0xf) + 0xf) * 0xec);
              }
              iVar15 = FUN_00481380(s_selfdestroypln_00528360);
              *(char *)((int)local_38 + 10) = (char)iVar15;
              pbVar13 = DAT_00732e54;
              puVar21 = (undefined4 *)&DAT_006d9d50;
              puVar23 = local_38_buf;
              for (iVar15 = 0xd; iVar15 != 0; iVar15 = iVar15 + -1) {
                *puVar23 = *puVar21;
                puVar21 = puVar21 + 1;
                puVar23 = puVar23 + 1;
              }
              *(undefined2 *)puVar23 = *(undefined2 *)puVar21;
              *(byte *)((int)puVar23 + 2) = *(byte *)((int)puVar21 + 2);
              DAT_00732e54 = (byte *)puVar19;
              FUN_004296b0(uVar5);
              FUN_00429310((uint)uVar5);
              puVar21 = local_38_buf;
              puVar23 = (undefined4 *)&DAT_006d9d50;
              DAT_00732e54 = pbVar13;
              for (iVar15 = 0xd; iVar15 != 0; iVar15 = iVar15 + -1) {
                *puVar23 = *puVar21;
                puVar21 = puVar21 + 1;
                puVar23 = puVar23 + 1;
              }
              *(undefined2 *)puVar23 = *(undefined2 *)puVar21;
              *(byte *)((int)puVar23 + 2) = *(byte *)((int)puVar21 + 2);
            }
            uVar11 = (uint)DAT_006d9d50;
            iVar15 = 0x28;
          }
          else {
            iVar15 = 0x11;
            pcVar20 = pcVar25;
            pcVar22 = s_boardtopickuppln_00524f44;
            do {
              if (iVar15 == 0) break;
              iVar15 = iVar15 + -1;
              bVar24 = *pcVar20 == *pcVar22;
              pcVar20 = pcVar20 + 1;
              pcVar22 = pcVar22 + 1;
            } while (bVar24);
            if (bVar24) {
              if (LD(int, DAT_006d4988 + iVar14 + 0x1f) == 0) {
                *(short *)(&DAT_006d4a76 + (uint)DAT_006d4988[iVar14 + 0x16] * 0x126) =
                     *(short *)(&DAT_006d4a76 + (uint)DAT_006d4988[iVar14 + 0x16] * 0x126) + 1;
                pbVar13[iVar14 + 3] = 0;
              }
              else {
                FUN_00496770((uint)uVar5, (int)puVar19, 0x80);
                *(ushort *)(DAT_00732e54 + 0x66) = uVar5;
                *(ushort *)(puVar19 + 0x1a) = DAT_006d9d50;
              }
              uVar11 = (uint)DAT_006d9d50;
              iVar15 = 0x26;
            }
            else {
              iVar15 = 0x12;
              pcVar20 = pcVar25;
              pcVar22 = s_boardtocontactpln_00524f30;
              do {
                if (iVar15 == 0) break;
                iVar15 = iVar15 + -1;
                bVar24 = *pcVar20 == *pcVar22;
                pcVar20 = pcVar20 + 1;
                pcVar22 = pcVar22 + 1;
              } while (bVar24);
              if (bVar24) {
                iVar15 = LD(int, DAT_006d4988 + iVar14 + 0x1f);
                if (iVar15 != 0) {
                  *(byte *)((int)puVar19 + 0x1fa) = 2;
                }
                FUN_004a34c0(CONCAT22((short)((uint)iVar15 >> 0x10), DAT_006d9d50),
                             (undefined4 *)DAT_00732e54, 0x9b);
                uVar11 = (uint)DAT_006d9d50;
                iVar15 = 0x28;
              }
              else {
                iVar15 = 0x11;
                pcVar20 = s_boardtorepairpln_00524f1c;
                do {
                  if (iVar15 == 0) break;
                  iVar15 = iVar15 + -1;
                  bVar24 = *pcVar25 == *pcVar20;
                  pcVar25 = pcVar25 + 1;
                  pcVar20 = pcVar20 + 1;
                } while (bVar24);
                if (!bVar24) goto LAB_004963b9;
                if (LD(int, DAT_006d4988 + iVar14 + 0x1f) != 0) {
                  ST(undefined2, (byte *)puVar19 + 0x1df,
                     LD(undefined2, (byte *)puVar19 + 0x1dd));
                  ST(undefined2, (byte *)puVar19 + 0xc9, 0);
                  *(byte *)((int)puVar19 + 7) = 0;
                }
                FUN_004a34c0((uint)DAT_006d9d50, (undefined4 *)DAT_00732e54, 0x9b);
                FUN_0040c070(1, 0x28, (uint)DAT_006d9d50, 0xffff);
                uVar11 = (uint)uVar5;
                iVar15 = 0x1d;
              }
            }
          }
          goto LAB_004963af;
        }
        if (LD(int, DAT_006d4988 + iVar14 + 0x1f) != 0) {
          FUN_00496770((uint)uVar5, (int)puVar19, 0x80);
          if (*(char *)((int)puVar19 + 0x1e5) == '\0') {
            if (*(char *)((int)puVar19 + 5) == -1) {
              bVar12 = (&DAT_00524798)[(byte)(&DAT_0070ad92)[(uint)local_44 * 0x566]];
            }
            else {
              bVar12 = (&DAT_005247c0)[(byte)(&DAT_0070ad92)[(uint)local_44 * 0x566]];
            }
            *(byte *)((int)local_38 + 10) = *(byte *)((int)&DAT_006da3e0 + (uint)bVar12);
          }
          else {
            if (*(short *)((int)puVar19 + 0x92) == 0) {
              pcVar25 = s_stationaryldrpln_00524934;
            }
            else {
              pcVar25 = s_flyhomeevadepln_00525b50;
            }
            iVar15 = FUN_00481380(pcVar25);
            *(char *)((int)local_38 + 10) = (char)iVar15;
            FUN_004a34c0(CONCAT22(uVar5, uVar5), puVar19, 0x8a);
          }
          FUN_0040c070(1, 0x28, (uint)DAT_006d9d50, 0xffff);
          FUN_0040c070(1, 0x1f, (uint)uVar5, 0xffff);
          pbVar13 = DAT_00732e54;
          puVar21 = (undefined4 *)&DAT_006d9d50;
          puVar23 = local_38_buf;
          for (iVar15 = 0xd; iVar15 != 0; iVar15 = iVar15 + -1) {
            *puVar23 = *puVar21;
            puVar21 = puVar21 + 1;
            puVar23 = puVar23 + 1;
          }
          *(undefined2 *)puVar23 = *(undefined2 *)puVar21;
          *(byte *)((int)puVar23 + 2) = *(byte *)((int)puVar21 + 2);
          DAT_00732e54 = (byte *)puVar19;
          FUN_004296b0(uVar5);
          FUN_00429310((uint)uVar5);
          puVar21 = local_38_buf;
          puVar23 = (undefined4 *)&DAT_006d9d50;
          DAT_00732e54 = pbVar13;
          for (iVar15 = 0xd; iVar15 != 0; iVar15 = iVar15 + -1) {
            *puVar23 = *puVar21;
            puVar21 = puVar21 + 1;
            puVar23 = puVar23 + 1;
          }
          *(undefined2 *)puVar23 = *(undefined2 *)puVar21;
          *(byte *)((int)puVar23 + 2) = *(byte *)((int)puVar21 + 2);
        }
      }
    }
LAB_004963b9:
    uVar6 = 0;
    if (LD(int, DAT_006d4988 + iVar14 + 0x1f) != 0) {
      if (*(char *)((byte *)LD(int, DAT_006d4988 + iVar14 + 0x1f) + 0x86) ==
          *(char *)((byte *)LD(int, DAT_006d4988 + (uint)DAT_006d9d50 * 0x23 + 0x1f) + 0x86)) {
        uVar11 = (uint)*(byte *)((byte *)LD(int, DAT_006d4988 + (uint)DAT_006d9d50 * 0x23 + 0x1f) + 0x87);
        if (*(char *)(uVar11 + 0x1f0 + (int)puVar19) == '\0') {
          bVar12 = 0;
          do {
            bVar1 = *(byte *)(uVar6 + 0x1f0 + (int)puVar19);
            if (bVar12 < bVar1) {
              bVar12 = bVar1;
            }
            uVar6 = uVar6 + 1;
          } while (uVar6 < 10);
          *(byte *)(uVar11 + 0x1f0 + (int)puVar19) = bVar12 + 1;
          pbVar13 = DAT_006d4988 + iVar14 + 0x16;
          *(short *)(&DAT_006d4a7a + (uint)DAT_006d4988[iVar14 + 0x16] * 0x126) =
               *(short *)(&DAT_006d4a7a + (uint)DAT_006d4988[iVar14 + 0x16] * 0x126) + 1;
          if ((&DAT_0070ad40)[(uint)*pbVar13 * 0x566] == *(char *)(puVar19 + 0x2c)) {
            (&DAT_006d4aaa)[(uint)*pbVar13 * 0x126] = 1;
          }
        }
      }
      else {
        FUN_0040a1a0(0x24, 0xffffffff, (int)DAT_0071ecf4);
      }
    }
    P56[*P56 + 5] = P56[*P56 + 5] + 1;
    *(undefined2 *)(DAT_00732e54 + (uint)DAT_00732e54[0x7d] * 2 + 0x7e) = uVar3;
    DAT_00732e54[0x7d] = DAT_00732e54[0x7d] + 1;
    if (9 < DAT_00732e54[0x7d]) {
      DAT_00732e54[0x7d] = DAT_00732e54[0x7d] - 1;
    }
    if (DAT_00732e54[0x7d] == 1) {
      uVar11 = (uint)DAT_006d9d60;
      iVar15 = uVar11 * 0x126;
      *(short *)(&DAT_006d4a82 + iVar15) = *(short *)(&DAT_006d4a82 + iVar15) + 1;
      if ((&DAT_0070ad40)[uVar11 * 0x566] == DAT_00732e54[0xb0]) {
        (&DAT_006d4aae)[iVar15] = 1;
      }
    }
    if ((LD(int, DAT_006d4988 + iVar14 + 0x1f) != 0) && (*(char *)((int)puVar19 + 0x7b) == '\0')) {
      iVar15 = (uint)local_44 * 0x126;
      *(short *)(&DAT_006d4a7e + iVar15) = *(short *)(&DAT_006d4a7e + iVar15) + 1;
      if ((&DAT_0070ad40)[(uint)local_44 * 0x566] == *(char *)(puVar19 + 0x2c)) {
        (&DAT_006d4aac)[iVar15] = 1;
      }
    }
    P56[0x35] = 3;
    pbVar13 = P56;
    pbVar13[0x36] = 0x38;
    pbVar4 = P56;
    pbVar13[0x37] = 9;
    pbVar13[0x38] = 0;
    pbVar13[0x39] = 0;
    if (LD(ushort, (byte *)&DAT_0071b74a + (uint)DAT_0071ecf4 * 0x5bd) == uVar5) {
      DAT_00655a84 = (short)0xfffd;
    }
    pbVar9 = DAT_006d4988;
    if (LD(int, DAT_006d4988 + iVar14 + 0x1b) != -1) {
      pbVar13 = P56;
      pbVar13[0x2b] = 0xff;
      pbVar13[0x2c] = 0xff;
      return (uint)pbVar4 & 0xffff0000;
    }
    break;
  case 3:
    if (LD(int, P56 + 0x36) == 0) {
      pbVar13 = P56;
      pbVar13[0x18] = 0xff;
      pbVar13[0x19] = 0xff;
      pbVar13 = P56;
      pbVar13[0x1a] = 0;
      pbVar9 = P56;
      pbVar13[0x1b] = 0;
      P56[0x1e] = 0;
      return CONCAT22((short)((uint)pbVar9 >> 0x10), 1);
    }
    if (LD(int, DAT_006d4988 + iVar14 + 0x1f) != 0) {
      FUN_0042a230((int)DAT_006d4988, (uint)iVar15,
                   (int)(DAT_006d4988 + (uint)DAT_006d9d50 * 0x23), 0, 0x4000, 0);
      ST(int, DAT_00732e54 + 0xb1,
         (int)(((uint)LD(int, DAT_006d4988 + iVar14 + 4) -
                (uint)LD(int, DAT_006d4988 + (uint)DAT_006d9d50 * 0x23 + 4)) + (uint)DAT_0070ab18));
      ST(int, DAT_00732e54 + 0xb5,
         (int)(((uint)LD(int, DAT_006d4988 + iVar14 + 8) -
                (uint)LD(int, DAT_006d4988 + (uint)DAT_006d9d50 * 0x23 + 8)) + (uint)DAT_0070ab14));
      uVar11 = (uint)DAT_0070ac20 & 0xffff0000;
      ST(int, DAT_00732e54 + 0xb9,
         (int)(((uint)LD(int, DAT_006d4988 + iVar14 + 0xc) -
                (uint)LD(int, DAT_006d4988 + (uint)DAT_006d9d50 * 0x23 + 0xc)) + (uint)DAT_0070ac20));
      return uVar11;
    }
    pbVar13 = DAT_00732e54;
    pbVar13[0xb1] = 0;
    pbVar13[0xb2] = 0;
    pbVar13[0xb3] = 0;
    pbVar13[0xb4] = 0;
    pbVar13 = DAT_00732e54;
    pbVar13[0xb5] = 0;
    pbVar9 = DAT_00732e54;
    pbVar13[0xb6] = 0;
    pbVar13[0xb7] = 0;
    pbVar13[0xb8] = 0;
    pbVar13 = DAT_00732e54;
    pbVar13[0xb9] = 0xf4;
    pbVar13[0xba] = 1;
    pbVar13[0xbb] = 0;
    pbVar13[0xbc] = 0;
  }
  return (uint)pbVar9 & 0xffff0000;
}
