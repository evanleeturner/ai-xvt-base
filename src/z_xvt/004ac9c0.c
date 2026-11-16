/* SPDX-License-Identifier: MIT */
/* z_xvt 004ac9c0 */
extern unsigned char DAT_00656c80;
extern int DAT_0071ecf4;
extern int DAT_006d4988;
extern unsigned char DAT_0071b730[];
extern unsigned char DAT_0071b74a[];
extern unsigned char DAT_0071b757[];
extern unsigned char DAT_0071b758[];
extern unsigned char DAT_0071bb37[];
extern unsigned char DAT_0052b0ac[];
extern unsigned short DAT_00655184;
extern unsigned char DAT_00655ab0[];
extern unsigned char DAT_00655ab2[];
extern unsigned char DAT_00655ab4[];
extern unsigned char DAT_00655ab6[];
extern unsigned char DAT_00655ad4[];
extern unsigned char DAT_00655ad6[];
extern unsigned char DAT_00655ad8[];
extern unsigned char DAT_00655ada[];
extern unsigned char DAT_00655b04[];
extern unsigned char DAT_00655b06[];
extern unsigned char DAT_00656c90[];
extern unsigned char DAT_00656c96[];
extern int DAT_0052552c;
extern int DAT_00525518;
extern int DAT_006da538;
extern int DAT_006da9ec;
extern int DAT_006db594;
extern unsigned int DAT_0071f2f0[];
extern void (__cdecl *DAT_0070a184)(unsigned int, int, int, int, int);

unsigned int __cdecl FUN_004783e0(unsigned short a, int b);
void __cdecl FUN_00461f20(int a, int b, int c, int d);
void __cdecl FUN_0048ee20(char a);
void __cdecl FUN_0048ed00(int a, int b, short c, short d);
void __cdecl FUN_0048ece0(short a, unsigned short b);
void __cdecl FUN_0048eda0(int a);
void __cdecl FUN_0048ed70(int a);
void __cdecl FUN_0040ccc0(unsigned short a, unsigned int b, int c);
unsigned int __cdecl FUN_00410060(unsigned short a, unsigned short b, unsigned int c);

void __stdcall FUN_004ac9c0(void)
{
  int iVar1;
  unsigned char bVar2;
  char cVar3;
  short sVar4;
  short sVar5;
  unsigned char *iVar6;
  int bVar7;
  unsigned int uVar8;
  short sVar9;
  unsigned short uVar10;
  unsigned int uVar11;
  int iVar12;
  unsigned int uVar13;
  unsigned int uVar14;
  unsigned short uVar15;
  unsigned short uVar16;
  unsigned int local_26;
  short local_22;
  unsigned short uStack_20;
  unsigned short uStack_1e;
  unsigned short local_1c;
  unsigned short uStack_1a;
  unsigned int slot;
  unsigned int tmp;
  unsigned short w;

  DAT_00656c80 = 0;
  __builtin_memcpy(&slot, DAT_0071b730 + (unsigned int)DAT_0071ecf4 * 0x5bd, 4);
  tmp = slot * 0x23u + 0x1fu + (unsigned int)DAT_006d4988;
  __builtin_memcpy(&tmp, (unsigned char *)tmp, 4);
  tmp = tmp + 0xa7u;
  __builtin_memcpy(&tmp, (unsigned char *)tmp, 4);
  iVar6 = (unsigned char *)tmp;
  uStack_20 = 0;
  bVar2 = iVar6[4];
  local_26 = (unsigned int)iVar6[0x213] << 0x10;
  if (iVar6[0x213] != 0) {
    do {
      uVar8 = (unsigned int)uStack_20;
      uVar13 = (unsigned int)(DAT_0052b0ac[(unsigned int)bVar2 * 0xef] < uVar8);
      iVar12 = (int)((unsigned int)DAT_00655184 + uVar8);
      sVar4 = *(short *)(DAT_00655ad4 + (unsigned int)iVar12 * 0xc);
      iVar1 = (int)((unsigned int)iVar12 * 0xc);
      uVar16 = *(unsigned short *)(DAT_00655ad6 + iVar1);
      if (((int)sVar4 + (unsigned int)uVar16 != 0) || (DAT_00655184 != 0)) {
        uVar10 = *(unsigned short *)(DAT_00655ad8 + iVar1);
        local_1c = (unsigned short)(short)(char)iVar6[0x345 + uVar8 * 4];
        __builtin_memcpy(&w, iVar6 + 0x1db, 2);
        if ((DAT_0071bb37[(unsigned int)DAT_0071ecf4 * 0x5bd] == '\0') &&
            ((w & 2) != 0) && ((w & 4) != 0)) {
          if (((short)local_1c < 1) || ((iVar6[0x1df] & 0x10) == 0)) {
            uStack_1e = 0;
            local_26 = (local_26 >> 16) << 16;
            local_22 = 0;
          }
          else {
            uVar15 = (unsigned short)(local_1c + 1);
            if ((short)uVar15 < 0x41) {
              local_26 = (local_26 >> 16) << 16;
              local_22 = 1;
              local_1c = uVar15;
            }
            else {
              local_26 = (local_26 & 0xffff0000u) | 1;
              local_22 = 2;
              local_1c = (unsigned short)(local_1c - 0x3f);
            }
            uStack_1e = (unsigned short)(local_1c / 6);
            local_1c = uVar15;
            if (10 < uStack_1e) {
              uStack_1e = 10;
            }
          }
          if (*(unsigned short *)(DAT_00656c96 + (unsigned int)iVar12 * 2) != uStack_1e) {
            *(unsigned short *)(DAT_00656c96 + (unsigned int)iVar12 * 2) = uStack_1e;
            sVar9 = 3;
            if (DAT_0052552c != 0x13) {
              sVar9 = (short)((((unsigned short)(-(unsigned short)(DAT_0052552c == 0x112))) & 0xfffe) + 6);
            }
            sVar5 = *(short *)(DAT_00655ada + iVar1);
            if (sVar5 != 0) {
              sVar9 = (short)(-sVar9);
            }
            if (DAT_00655184 == 0) {
              uVar11 = 0;
              do {
                uVar14 = ((unsigned int)uStack_20 << 16) | (unsigned int)(unsigned short)local_22;
                if (uStack_1e <= (unsigned short)uVar11) {
                  uVar14 = local_26;
                }
                uVar15 = (unsigned short)((unsigned short)uVar11 + 1);
                (*DAT_0070a184)(DAT_0071f2f0[(uVar14 & 0xffff) + (unsigned int)uVar10],
                                (int)(uVar11 * (unsigned int)(int)sVar9 + (unsigned int)(int)sVar4),
                                (int)(unsigned int)uVar16, 0xfd, sVar5 != 0);
                uVar11 = (unsigned int)uVar15;
              } while (uVar15 < 10);
            }
            else if (*(short *)(DAT_00655ad8 + iVar1) != 0) {
              uVar11 = FUN_004783e0(local_1c, 0x7f);
              FUN_00461f20(DAT_006da9ec, DAT_006da538, DAT_006db594,
                           (int)((unsigned int)DAT_00525518 * (unsigned int)DAT_006da538));
              FUN_0048ee20('\x02');
              FUN_0048ed00(0, 0, (short)DAT_006da538, (short)DAT_006db594);
              FUN_0048ece0(sVar4, uVar16);
              FUN_0048eda0(0);
              FUN_0048ed70(0x4a);
              FUN_0040ccc0((unsigned short)((uVar11 & 0xffff) / 0x28f),
                           (unsigned int)*(unsigned short *)(DAT_00655ad8 +
                               ((unsigned int)DAT_00655184 + uVar8) * 0xc), 1);
              FUN_00461f20(0, 0x140, 200, 0);
            }
          }
        }
        if (((short)local_1c < 1) || ((iVar6[0x1df] & 0x10) == 0)) {
          uStack_1a = 0;
          goto LAB_004ace1f;
        }
        else {
          if ((DAT_0071b758[(unsigned int)DAT_0071ecf4 * 0x5bd] != '\0') ||
              ((unsigned char)DAT_0071b757[(unsigned int)DAT_0071ecf4 * 0x5bd] != uVar13)) {
            uStack_1a = 1;
            goto LAB_004ace1f;
          }
          switch (iVar6[0x216 + uVar13]) {
          case 0:
          case 4:
            uStack_1a = 0;
            break;
          case 1:
            uStack_1a = (unsigned short)((((unsigned short)(-(unsigned short)(iVar6[0x21a + uVar13] == uVar8))) & 2) + 1);
            break;
          case 2:
            uVar11 = (unsigned int)iVar6[0x21a + uVar13];
            if ((uVar11 == uVar8) || ((3 < (local_26 >> 16)) && (uVar11 - uVar8 == 0xfffffffeu)))
              goto switchD_004acd94_caseD_3;
            uStack_1a = 1;
            break;
          case 3:
switchD_004acd94_caseD_3:
            uStack_1a = 3;
          }
          uVar16 = uStack_1a;
          if ((uStack_1a == 3) && (*(short *)(iVar6 + 0x21c + uVar13 * 2) != 0)) {
            uStack_1a = 5;
            uVar16 = 2;
          }
          goto LAB_004ace20;
        }
LAB_004ace1f:
        uVar16 = 0;
LAB_004ace20:
        if (local_22 == 2) {
          uStack_1a = (unsigned short)(uStack_1a + 1);
        }
        __builtin_memcpy(&iVar12, DAT_0071b730 + (unsigned int)DAT_0071ecf4 * 0x5bd, 4);
        if ((iVar12 == -1) ||
            ((((cVar3 = *(char *)((unsigned int)iVar12 * 0x23u + 3u + (unsigned int)DAT_006d4988), cVar3 != '\x01') &&
               (cVar3 != '\x02')) && (cVar3 != '\x03')) && ((cVar3 != '\x0e' && (cVar3 != '\x04'))))) {
          bVar7 = 0;
        }
        else {
          bVar7 = 1;
        }
        __builtin_memcpy(&w, iVar6 + 0x1db, 2);
        if (((bVar7) && (DAT_0071bb37[(unsigned int)DAT_0071ecf4 * 0x5bd] == '\0')) &&
            (((w & 2) != 0) &&
             (((w & 4) != 0) &&
              (iVar12 = (int)(((unsigned int)DAT_00655184 + uVar8) * 0xc),
               (unsigned int)*(unsigned short *)(DAT_00655b04 + iVar12) +
                   (unsigned int)*(unsigned short *)(DAT_00655b06 + iVar12) != 0) &&
              (iVar12 = (int)(uVar8 + 7 + (unsigned int)DAT_00655184),
               *(unsigned short *)(DAT_00656c90 + (unsigned int)iVar12 * 2) != uStack_1a)))) {
          *(unsigned short *)(DAT_00656c90 + (unsigned int)iVar12 * 2) = uStack_1a;
          iVar12 = (int)((unsigned int)iVar12 * 0xc);
          (*DAT_0070a184)(DAT_0071f2f0[(unsigned int)*(unsigned short *)(DAT_00655ab4 + iVar12) + (unsigned int)uStack_1a],
                          *(short *)(DAT_00655ab0 + iVar12),
                          *(short *)(DAT_00655ab2 + iVar12),
                          *(short *)(DAT_00655ab6 + iVar12), 0);
        }
        iVar12 = (int)((unsigned int)DAT_00655184 + 0x25 + uVar8);
        if (*(unsigned short *)(DAT_00656c90 + (unsigned int)iVar12 * 2) != uVar16) {
          *(unsigned short *)(DAT_00656c90 + (unsigned int)iVar12 * 2) = uVar16;
          iVar12 = (int)((unsigned int)iVar12 * 0xc);
          (*DAT_0070a184)(DAT_0071f2f0[(unsigned int)*(unsigned short *)(DAT_00655ab4 + iVar12) + (unsigned int)uVar16],
                          *(short *)(DAT_00655ab0 + iVar12),
                          *(short *)(DAT_00655ab2 + iVar12),
                          *(short *)(DAT_00655ab6 + iVar12), 0);
        }
        __builtin_memcpy(&w, iVar6 + 0x1df, 2);
        uVar10 = (unsigned short)(w & 4);
        if ((uVar10 == 0) || (uVar16 != 3)) {
          if (uVar10 == 0) {
            uVar16 = 0;
          }
          else if (uVar16 == 2) goto LAB_004ad01c;
        }
        else {
          __builtin_memcpy(&uVar16, DAT_0071b74a + (unsigned int)DAT_0071ecf4 * 0x5bd, 2);
          __builtin_memcpy(&tmp, DAT_0071b730 + (unsigned int)DAT_0071ecf4 * 0x5bd, 4);
          if ((uVar16 == 0xffff) ||
              (uVar13 = FUN_00410060((unsigned short)tmp, uVar16,
                                     ((unsigned int)uStack_1e << 16) | (unsigned int)uStack_20),
               (short)uVar13 == 0)) {
LAB_004ad01c:
            uVar16 = 1;
          }
          else {
            DAT_00656c80 = 1;
            uVar16 = 2;
          }
        }
        if (uVar16 == 3) {
          uVar16 = 0;
        }
        __builtin_memcpy(&iVar12, DAT_0071b730 + (unsigned int)DAT_0071ecf4 * 0x5bd, 4);
        if ((iVar12 == -1) ||
            ((((cVar3 = *(char *)((unsigned int)iVar12 * 0x23u + 3u + (unsigned int)DAT_006d4988), cVar3 != '\x01') &&
               (cVar3 != '\x02')) && (cVar3 != '\x03')) && ((cVar3 != '\x0e' && (cVar3 != '\x04'))))) {
          bVar7 = 0;
        }
        else {
          bVar7 = 1;
        }
        if ((bVar7) &&
            (iVar12 = (int)((unsigned int)DAT_00655184 + 0x29 + uVar8),
             *(unsigned short *)(DAT_00656c90 + (unsigned int)iVar12 * 2) != uVar16)) {
          *(unsigned short *)(DAT_00656c90 + (unsigned int)iVar12 * 2) = uVar16;
          iVar12 = (int)((unsigned int)iVar12 * 0xc);
          (*DAT_0070a184)(DAT_0071f2f0[(unsigned int)*(unsigned short *)(DAT_00655ab4 + iVar12) + (unsigned int)uVar16],
                          *(short *)(DAT_00655ab0 + iVar12),
                          *(short *)(DAT_00655ab2 + iVar12),
                          *(short *)(DAT_00655ab6 + iVar12), 0);
        }
      }
      uStack_20 = (unsigned short)(uStack_20 + 1);
    } while (uStack_20 < (unsigned short)(local_26 >> 16));
  }
  return;
}
