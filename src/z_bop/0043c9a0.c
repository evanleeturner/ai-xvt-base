/* SPDX-License-Identifier: MIT */
/* z_bop 0043c9a0 */
extern unsigned char DAT_00a0a1d0;
extern unsigned char DAT_009e9670[];
extern unsigned char DAT_009e968a[];
extern unsigned char DAT_009e9697[];
extern unsigned char DAT_009e9698[];
extern unsigned char DAT_009e9a77[];
extern int DAT_009ecc34;
extern unsigned char *DAT_009a1fe8;
extern unsigned char DAT_0051c5a2[];
extern unsigned short DAT_00a08374;
extern unsigned char DAT_00a08cc4[];
extern unsigned char DAT_00a08cc6[];
extern unsigned char DAT_00a08cc8[];
extern unsigned char DAT_00a08cca[];
extern unsigned char DAT_00a08d24[];
extern unsigned char DAT_00a08d26[];
extern unsigned short DAT_00a0a1e6[];
extern unsigned int DAT_009ed240[];
extern int DAT_005233ec;
extern int DAT_005233d8;
extern int DAT_009a806c;
extern int DAT_009a7bac;
extern int DAT_009a8c14;
extern void (*DAT_009d80c4)(unsigned int, int, unsigned int, unsigned int, int);

unsigned int FUN_00425c20(unsigned int a, unsigned int b);
void FUN_0040e0c0(int a, int b, int c, int d);
void FUN_004a9640(char a);
void FUN_004a9520(int a, int b, short c, short d);
void FUN_004a9500(short a, unsigned short b);
void FUN_004a95c0(int a);
void FUN_004a9590(int a);
void FUN_004277f0(unsigned short a, unsigned int b, int c);
void FUN_00440760(unsigned int a, unsigned int b);
unsigned int FUN_0041ca60(unsigned short a, unsigned short b, unsigned int c);

void __stdcall FUN_0043c9a0(void)
{
  unsigned char bVar1;
  char cVar2;
  short sVar3;
  short sVar4;
  unsigned char *iVar5;
  int bVar6;
  short sVar7;
  unsigned short uVar8;
  unsigned int uVar9;
  int iVar10;
  unsigned int uVar11;
  unsigned int uVar12;
  unsigned short uVar13;
  unsigned short uVar14;
  int iVar15;
  unsigned int local_26;
  short local_22;
  unsigned short uStack_20;
  unsigned short uStack_1e;
  unsigned short local_1c;
  unsigned short uStack_1a;
  int tmpInt;
  unsigned int tmpPtr;
  unsigned short tmp16;
  unsigned int callRes;

  DAT_00a0a1d0 = 0;
  __builtin_memcpy(&tmpInt, &DAT_009e9670[(unsigned int)DAT_009ecc34 * 0x5bdu], 4);
  __builtin_memcpy(&tmpPtr, DAT_009a1fe8 + ((unsigned int)tmpInt * 0x23u + 0x1fu), 4);
  __builtin_memcpy(&tmpPtr, ((unsigned char *)tmpPtr) + 0xa9, 4);
  iVar5 = (unsigned char *)tmpPtr;
  uStack_20 = 0;
  bVar1 = iVar5[4];
  local_26 = (unsigned int)iVar5[0x21d] << 0x10;
  if (iVar5[0x21d] != 0) {
    do {
      uVar11 = (unsigned int)(DAT_0051c5a2[(unsigned int)bVar1 * 0x101u] < uStack_20);
      iVar15 = (int)((unsigned int)uStack_20 + (unsigned int)DAT_00a08374);
      sVar3 = *(short *)(DAT_00a08cc4 + (unsigned int)iVar15 * 0xcu);
      iVar10 = (int)((unsigned int)iVar15 * 0xcu);
      uVar14 = *(unsigned short *)(DAT_00a08cc6 + iVar10);
      if (((int)sVar3 + (unsigned int)uVar14 != 0) || (DAT_00a08374 != 0)) {
        uVar8 = *(unsigned short *)(DAT_00a08cc8 + iVar10);
        local_1c = (unsigned short)(char)iVar5[0x371u + (unsigned int)uStack_20 * 4u];
        __builtin_memcpy(&tmp16, iVar5 + 0x1e1, 2);
        if ((DAT_009e9a77[(unsigned int)DAT_009ecc34 * 0x5bdu] == 0) &&
            ((tmp16 & 2) != 0) && ((tmp16 & 4) != 0)) {
          if (((short)local_1c < 1) || ((iVar5[0x1e5] & 0x10) == 0)) {
            uStack_1e = 0;
            local_26 = (unsigned int)(unsigned short)(local_26 >> 16) << 0x10;
            local_22 = 0;
          }
          else {
            uVar13 = (unsigned short)(local_1c + 1);
            if ((short)uVar13 < 0x41) {
              local_26 = (unsigned int)(unsigned short)(local_26 >> 16) << 0x10;
              local_22 = 1;
              local_1c = uVar13;
            }
            else {
              local_26 = ((unsigned int)(unsigned short)(local_26 >> 16) << 0x10) | 1u;
              local_22 = 2;
              local_1c = (unsigned short)(local_1c - 0x3f);
            }
            uStack_1e = (unsigned short)(local_1c / 6);
            local_1c = uVar13;
            if (10 < uStack_1e) {
              uStack_1e = 10;
            }
          }
          if (DAT_00a0a1e6[iVar15] != uStack_1e) {
            DAT_00a0a1e6[iVar15] = uStack_1e;
            sVar7 = 3;
            if (DAT_005233ec != 0x13) {
              sVar7 = (short)((((unsigned short)(-(unsigned short)(DAT_005233ec == 0x112))) & 0xfffe) + 6);
            }
            sVar4 = *(short *)(DAT_00a08cca + iVar10);
            if (sVar4 != 0) {
              sVar7 = (short)(0u - (unsigned int)(int)sVar7);
            }
            if (DAT_00a08374 == 0) {
              uVar9 = 0;
              do {
                uVar12 = ((unsigned int)uStack_20 << 16) | (unsigned int)(unsigned short)local_22;
                if (uStack_1e <= (unsigned short)uVar9) {
                  uVar12 = local_26;
                }
                uVar13 = (unsigned short)((unsigned short)uVar9 + 1);
                (*DAT_009d80c4)(DAT_009ed240[(uVar12 & 0xffff) + (unsigned int)uVar8],
                                (int)(uVar9 * (unsigned int)(int)sVar7 + (unsigned int)(int)sVar3),
                                (unsigned int)uVar14, 0xfd, sVar4 != 0);
                uVar9 = (unsigned int)uVar13;
              } while (uVar13 < 10);
            }
            else if (*(short *)(DAT_00a08cc8 + iVar10) != 0) {
              uVar9 = FUN_00425c20(local_1c, 0x7f);
              FUN_0040e0c0(DAT_009a806c, DAT_009a7bac, DAT_009a8c14,
                           (int)((unsigned int)DAT_005233d8 * (unsigned int)DAT_009a7bac));
              FUN_004a9640('\x02');
              FUN_004a9520(0, 0, (short)DAT_009a7bac, (short)DAT_009a8c14);
              FUN_004a9500(sVar3, uVar14);
              FUN_004a95c0(0);
              FUN_004a9590(0x4a);
              FUN_004277f0((unsigned short)((uVar9 & 0xffff) / 0x28f),
                           (unsigned int)*(unsigned short *)
                             (DAT_00a08cc8 + ((unsigned int)DAT_00a08374 + (unsigned int)uStack_20) * 0xcu),
                           1);
              FUN_0040e0c0(0, 0x140, 200, 0);
            }
          }
        }
        if (((short)local_1c < 1) || ((iVar5[0x1e5] & 0x10) == 0)) {
          uStack_1a = 0;
LAB_0043ce01:
          uVar14 = 0;
        }
        else {
          if ((DAT_009e9698[(unsigned int)DAT_009ecc34 * 0x5bdu] != 0) ||
              ((unsigned int)DAT_009e9697[(unsigned int)DAT_009ecc34 * 0x5bdu] != uVar11)) {
            uStack_1a = 1;
            goto LAB_0043ce01;
          }
          switch (iVar5[0x220u + uVar11]) {
          case 0:
          case 4:
            uStack_1a = 0;
            break;
          case 1:
            uStack_1a = (unsigned short)(((unsigned short)(-(unsigned short)(iVar5[0x224u + uVar11] == uStack_20)) & 2) + 1);
            break;
          case 2:
            uVar9 = (unsigned int)iVar5[0x224u + uVar11];
            if ((uVar9 == uStack_20) ||
                ((3 < (unsigned short)(local_26 >> 16)) &&
                 (uVar9 - (unsigned int)uStack_20 == 0xfffffffeu)))
              goto switchD_0043cd6c_caseD_3;
            uStack_1a = 1;
            break;
          case 3:
switchD_0043cd6c_caseD_3:
            uStack_1a = 3;
          }
          uVar14 = uStack_1a;
          if ((uStack_1a == 3) && (*(short *)(iVar5 + 0x226u + uVar11 * 2u) != 0)) {
            uStack_1a = 5;
            uVar14 = 2;
          }
        }
        if (local_22 == 2) {
          uStack_1a = (unsigned short)(uStack_1a + 1);
        }
        __builtin_memcpy(&iVar10, &DAT_009e9670[(unsigned int)DAT_009ecc34 * 0x5bdu], 4);
        if ((iVar10 == -1) ||
            (((cVar2 = (char)DAT_009a1fe8[(unsigned int)iVar10 * 0x23u + 3u], cVar2 != '\x01') &&
              (cVar2 != '\x02')) && (cVar2 != '\x03') && (cVar2 != '\x0e') && (cVar2 != '\x04'))) {
          bVar6 = 0;
        }
        else {
          bVar6 = 1;
        }
        __builtin_memcpy(&tmp16, iVar5 + 0x1e1, 2);
        if (((bVar6) && (DAT_009e9a77[(unsigned int)DAT_009ecc34 * 0x5bdu] == 0)) &&
            ((tmp16 & 2) != 0) && ((tmp16 & 4) != 0) &&
            (iVar10 = (int)(((unsigned int)uStack_20 + (unsigned int)DAT_00a08374) * 0xcu),
             (unsigned int)*(unsigned short *)(DAT_00a08d24 + iVar10) +
             (unsigned int)*(unsigned short *)(DAT_00a08d26 + iVar10) != 0)) {
          FUN_00440760((unsigned int)DAT_00a08374 + (unsigned int)uStack_20 + 0xb, (unsigned int)uStack_1a);
        }
        FUN_00440760((unsigned int)DAT_00a08374 + (unsigned int)uStack_20 + 0x35, (unsigned int)uVar14);
        __builtin_memcpy(&tmp16, iVar5 + 0x1e5, 2);
        uVar8 = (unsigned short)(tmp16 & 4);
        if ((uVar8 == 0) || (uVar14 != 3)) {
          if (uVar8 == 0) {
            uVar14 = 0;
          }
          else if (uVar14 == 2) goto LAB_0043cf5d;
        }
        else {
          __builtin_memcpy(&uVar14, &DAT_009e968a[(unsigned int)DAT_009ecc34 * 0x5bdu], 2);
          if (uVar14 == 0xffff) goto LAB_0043cf5d;
          __builtin_memcpy(&tmpInt, &DAT_009e9670[(unsigned int)DAT_009ecc34 * 0x5bdu], 4);
          callRes = FUN_0041ca60((unsigned short)tmpInt, uVar14,
                                 ((unsigned int)uStack_1e << 16) | (unsigned int)uStack_20);
          if ((short)callRes == 0) {
LAB_0043cf5d:
            uVar14 = 1;
          }
          else {
            DAT_00a0a1d0 = 1;
            uVar14 = 2;
          }
        }
        if (uVar14 == 3) {
          uVar14 = 0;
        }
        __builtin_memcpy(&iVar10, &DAT_009e9670[(unsigned int)DAT_009ecc34 * 0x5bdu], 4);
        if ((iVar10 == -1) ||
            (((cVar2 = (char)DAT_009a1fe8[(unsigned int)iVar10 * 0x23u + 3u], cVar2 != '\x01') &&
              (cVar2 != '\x02')) && (cVar2 != '\x03') && (cVar2 != '\x0e') && (cVar2 != '\x04'))) {
          bVar6 = 0;
        }
        else {
          bVar6 = 1;
        }
        if (bVar6) {
          FUN_00440760((unsigned int)DAT_00a08374 + (unsigned int)uStack_20 + 0x3d, (unsigned int)uVar14);
        }
      }
      uStack_20 = (unsigned short)(uStack_20 + 1);
    } while (uStack_20 < (unsigned short)(local_26 >> 16));
  }
  return;
}
