/* SPDX-License-Identifier: MIT */
/* z_xvt 004070e0 */
/* Globals: widths inferred from the decompile and P-code; the evidence gave no type info. */
extern int DAT_0052575c;
extern int DAT_00730884;
extern int DAT_00525518;
extern int DAT_006db598;
extern int DAT_006db6d0;
extern short DAT_0071b718;
extern short DAT_0071b716;
extern short DAT_0071b6b2;
extern short DAT_0071e518;
extern short DAT_0071e522;
extern short DAT_00703ad4;
extern short DAT_0071f2e0;
extern short DAT_00732902;
extern short DAT_00732900;
extern short DAT_0071b708;
extern short DAT_0071b714;
extern short DAT_00703c60;
extern short DAT_006ffbe6;
extern unsigned char DAT_0071e6d6;
extern unsigned char DAT_0071e6d7;
extern unsigned short DAT_0071e8d8[];
extern short DAT_0071e6d8[];
extern unsigned char DAT_00525750[];
extern unsigned char DAT_00525740[];
extern int DAT_006d8180[];
extern int DAT_0074110c;
extern short DAT_006d9d96;
extern short DAT_006da53c;
extern short DAT_006da53e;
extern short DAT_00732a20;
extern short DAT_00703c94;
extern short DAT_0070a170;
extern short DAT_0070acd4;
extern short DAT_0070acea;
extern short DAT_0070acec;

int __cdecl FUN_00407ed0(void);
int __cdecl FUN_00408260(void);
int __cdecl FUN_004085e0(void);
int __cdecl FUN_00408950(void);
int __cdecl FUN_00408d10(void);
int __cdecl FUN_00409050(void);
int __cdecl FUN_00409420(void);
int __cdecl FUN_004097b0(void);
int __cdecl FUN_00408140(void);
int __cdecl FUN_004084e0(void);
int __cdecl FUN_00408bd0(void);
int __cdecl FUN_004092e0(void);
int __cdecl FUN_00409680(void);
void __cdecl FUN_004079f0(void);

void __cdecl FUN_004070e0(unsigned char *param_1, int param_2)
{
  unsigned char *pbVar1;
  unsigned char bVar2;
  int iVar3;
  unsigned short uVar4;
  short sVar5;
  int iVar6;
  unsigned char bVar7;
  int iVar8;
  int iVar9;
  unsigned short uVar10;
  short sVar11;
  unsigned char bVar12;
  int iVar13;
  int *piVar14;
  int iVar15;
  int *piVar16;
  int iStack_54;
  int iStack_1c;
  int iStack_18;
  int iStack_14;
  short sStack_c;
  unsigned char bStack_7;
  unsigned char bStack_6;
  unsigned char bStack_5;

  DAT_0052575c = 1;
  switch (*(unsigned short *)(DAT_00730884 + 0x16)) {
  case 0:
    iStack_54 = FUN_00407ed0();
    break;
  case 1:
    iStack_54 = FUN_00408260();
    break;
  case 2:
    iStack_54 = FUN_004085e0();
    break;
  case 3:
    iStack_54 = FUN_00408950();
    break;
  case 4:
    iStack_54 = FUN_00408d10();
    break;
  case 5:
    iStack_54 = FUN_00409050();
    break;
  case 6:
    iStack_54 = FUN_00409420();
    break;
  case 7:
    iStack_54 = FUN_004097b0();
  }
  if (iStack_54 != 0) {
    DAT_00732902 = DAT_0071b718;
    DAT_00732900 = DAT_0071b716;
    DAT_0071b708 = DAT_0071e518;
    DAT_0071b714 = DAT_00703ad4;
    if (DAT_0071b6b2 < 1) {
      *(int *)(DAT_00730884 + 0x25b4) =
           (int)(((unsigned int)(int)DAT_0071f2e0 - (unsigned int)(int)DAT_0071b716) *
                 (unsigned int)DAT_006db6d0 +
                 (unsigned int)(int)DAT_0071b718 * (unsigned int)DAT_00525518 +
                 (unsigned int)DAT_006db598);
      *(int *)(DAT_00730884 + 0x3eb8) = DAT_006db6d0;
    }
    else {
      *(int *)(DAT_00730884 + 0x25b4) =
           (int)((unsigned int)(int)DAT_0071b718 * (unsigned int)DAT_00525518 +
                 (unsigned int)(int)DAT_0071b716 * (unsigned int)DAT_006db6d0 +
                 (unsigned int)DAT_006db598);
      *(int *)(DAT_00730884 + 0x3eb8) = (int)(0u - (unsigned int)DAT_006db6d0);
    }
    DAT_006db598 = *(int *)(DAT_00730884 + 0x25b4);
    bStack_6 = 0;
    DAT_00703c60 = 0;
    bStack_5 = 0;
    bStack_7 = 0;
    DAT_006ffbe6 = 1;
    bVar2 = 0;
    bVar7 = *param_1;
    while (bVar7 != 0xff) {
      uVar10 = (unsigned short)bStack_7;
      bVar7 = bStack_5;
      if (((unsigned char)(DAT_0071e6d6 + bStack_6) < bStack_6) &&
          (bVar7 = (unsigned char)(bStack_5 + 1), bVar7 == 0)) {
        uVar10 = (unsigned short)(unsigned char)(bStack_7 + 1);
      }
      if ((unsigned char)(bVar7 + DAT_0071e6d7) < bVar7) {
        uVar10 = (unsigned short)(unsigned char)((char)uVar10 + 1);
      }
      iStack_1c = 0;
      sVar11 = (short)((uVar10 * 0x100 + (unsigned short)(unsigned char)(bVar7 + DAT_0071e6d7)) -
                       ((unsigned short)bStack_7 * 0x100 + (unsigned short)bStack_5));
      iStack_18 = 0;
      sStack_c = sVar11;
      if (DAT_0052575c == 1) {
        bVar7 = *param_1;
        piVar16 = &DAT_006d8180[0];
        uVar10 = 0;
        for (;;) {
          iVar3 = iStack_18;
          if (bVar7 == 0xfe) break;
          bVar7 = *param_1;
          piVar14 = piVar16;
          if (bVar7 == 0xfb) {
            bVar2 = param_1[1];
            uVar4 = uVar10;
            param_1 = param_1 + 3;
          }
          else if (bVar7 == 0xfc) {
            pbVar1 = param_1 + 2;
            iStack_18 = (int)((unsigned int)iStack_18 +
                              (unsigned int)DAT_0071e8d8[param_1[1]]);
            uVar4 = (unsigned short)(uVar10 + DAT_0071e6d8[param_1[1]]);
            param_1 = pbVar1;
            if (uVar4 < uVar10) {
              iStack_18 = (int)((unsigned int)iStack_18 + 1u);
            }
          }
          else {
            if (bVar7 == 0xfd) {
              bVar7 = param_1[1];
              bVar12 = param_1[2];
              param_1 = param_1 + 3;
            }
            else {
              bVar12 = (unsigned char)((bVar7 >> (DAT_00525750[param_2] & 0x1f)) + bVar2);
              bVar7 = bVar7 & DAT_00525740[param_2];
              param_1 = param_1 + 1;
            }
            uVar4 = (unsigned short)(uVar10 + DAT_0071e6d8[bVar7]);
            iStack_18 = (int)((unsigned int)DAT_0071e8d8[bVar7] + (unsigned int)iStack_18);
            if (uVar4 < uVar10) {
              iStack_18 = (int)((unsigned int)iStack_18 + 1u);
            }
            *piVar16 = iVar3;
            piVar14 = piVar16 + 3;
            iStack_1c = (int)((unsigned int)iStack_1c + 1u);
            piVar16[1] = (unsigned int)bVar12;
            piVar16[2] = (int)(((unsigned int)iStack_18 - (unsigned int)iVar3) + 1u);
          }
          piVar16 = piVar14;
          uVar10 = uVar4;
          bVar7 = *param_1;
        }
        param_1 = param_1 + 1;
      }
      do {
        iVar3 = DAT_006db598;
        if ((iStack_1c != 0) && (-1 < DAT_00703ad4)) {
          sVar5 = (short)((short)iStack_18 + DAT_0071e522);
          if ((sVar5 < 0) ||
              (((DAT_00703ad4 <= sVar5 || (DAT_0071e522 < 0)) || (DAT_0071e522 < DAT_0071e518)))) {
            DAT_0074110c = iStack_1c;
            if (DAT_00525518 == 2) {
              iVar6 = (int)((unsigned int)DAT_00730884 + 0x25b8u);
              piVar16 = &DAT_006d8180[0];
              do {
                iVar15 = (int)((unsigned int)(int)DAT_0071e522 + (unsigned int)*piVar16);
                iVar9 = (int)((unsigned int)piVar16[2] + (unsigned int)iVar15);
                iVar8 = (int)DAT_0071e518;
                iVar13 = (int)DAT_00703ad4;
                if (iVar15 < iVar8) {
                  iVar15 = iVar8;
                }
                if ((iVar15 <= iVar13) && (iVar8 <= iVar9)) {
                  if (iVar13 < iVar9) {
                    iVar9 = iVar13;
                  }
                  iVar9 = (int)((unsigned int)iVar9 - (unsigned int)iVar15);
                  if (iVar9 != 0) {
                    iVar8 = piVar16[1];
                    piVar14 = (int *)((unsigned int)iVar15 * 4u + (unsigned int)iVar6);
                    do {
                      iVar15 = *piVar14;
                      piVar14 = piVar14 + 1;
                      *(char *)((unsigned int)iVar3 + (unsigned int)iVar15) = (char)iVar8;
                      *(unsigned char *)((unsigned int)iVar3 + 1u + (unsigned int)iVar15) = 0x80;
                      iVar9 = (int)((unsigned int)iVar9 - 1u);
                    } while (iVar9 != 0);
                  }
                }
                DAT_0074110c = (int)((unsigned int)DAT_0074110c - 1u);
                piVar16 = piVar16 + 3;
              } while (DAT_0074110c != 0);
            }
            else {
              iVar6 = (int)((unsigned int)DAT_00730884 + 0x25b8u);
              piVar16 = &DAT_006d8180[0];
              do {
                iVar15 = (int)((unsigned int)(int)DAT_0071e522 + (unsigned int)*piVar16);
                iVar9 = (int)((unsigned int)piVar16[2] + (unsigned int)iVar15);
                iVar8 = (int)DAT_0071e518;
                iVar13 = (int)DAT_00703ad4;
                if (iVar15 < iVar8) {
                  iVar15 = iVar8;
                }
                if ((iVar15 <= iVar13) && (iVar8 <= iVar9)) {
                  if (iVar13 < iVar9) {
                    iVar9 = iVar13;
                  }
                  iVar9 = (int)((unsigned int)iVar9 - (unsigned int)iVar15);
                  if (iVar9 != 0) {
                    iVar8 = piVar16[1];
                    piVar14 = (int *)((unsigned int)iVar15 * 4u + (unsigned int)iVar6);
                    do {
                      iVar15 = *piVar14;
                      piVar14 = piVar14 + 1;
                      *(char *)((unsigned int)iVar3 + (unsigned int)iVar15) = (char)iVar8;
                      iVar9 = (int)((unsigned int)iVar9 - 1u);
                    } while (iVar9 != 0);
                  }
                }
                DAT_0074110c = (int)((unsigned int)DAT_0074110c - 1u);
                piVar16 = piVar16 + 3;
              } while (DAT_0074110c != 0);
            }
          }
          else {
            DAT_0074110c = iStack_1c;
            if (DAT_00525518 == 2) {
              iVar6 = (int)((unsigned int)DAT_00730884 + 0x25b8u);
              piVar16 = &DAT_006d8180[0];
              do {
                iVar9 = piVar16[1];
                iVar15 = *piVar16;
                iVar8 = piVar16[2];
                piVar16 = piVar16 + 3;
                piVar14 = (int *)(((unsigned int)(int)DAT_0071e522 + (unsigned int)iVar15) * 4u +
                                  (unsigned int)iVar6);
                do {
                  iVar15 = *piVar14;
                  piVar14 = piVar14 + 1;
                  *(char *)((unsigned int)iVar3 + (unsigned int)iVar15) = (char)iVar9;
                  *(unsigned char *)((unsigned int)iVar3 + 1u + (unsigned int)iVar15) = 0x80;
                  iVar8 = (int)((unsigned int)iVar8 - 1u);
                } while (iVar8 != 0);
                DAT_0074110c = (int)((unsigned int)DAT_0074110c - 1u);
              } while (DAT_0074110c != 0);
            }
            else {
              iVar6 = (int)((unsigned int)DAT_00730884 + 0x25b8u);
              piVar16 = &DAT_006d8180[0];
              do {
                iVar9 = piVar16[1];
                iVar15 = *piVar16;
                iVar8 = piVar16[2];
                piVar16 = piVar16 + 3;
                piVar14 = (int *)(((unsigned int)(int)DAT_0071e522 + (unsigned int)iVar15) * 4u +
                                  (unsigned int)iVar6);
                do {
                  iVar15 = *piVar14;
                  piVar14 = piVar14 + 1;
                  *(char *)((unsigned int)iVar3 + (unsigned int)iVar15) = (char)iVar9;
                  iVar8 = (int)((unsigned int)iVar8 - 1u);
                } while (iVar8 != 0);
                DAT_0074110c = (int)((unsigned int)DAT_0074110c - 1u);
              } while (DAT_0074110c != 0);
            }
          }
        }
        if (sVar11 == 0) goto LAB_00407953;
        switch (*(unsigned short *)(DAT_00730884 + 0x16)) {
        case 0:
          iStack_14 = FUN_00408140();
          break;
        case 1:
          iStack_14 = FUN_004084e0();
          break;
        case 2:
          DAT_006db598 = (int)((unsigned int)DAT_006db598 +
                               (unsigned int)*(int *)(DAT_00730884 + 0x3eb8));
          DAT_0071b716 = (short)(DAT_0071b716 + -1);
          if (DAT_0071b716 == DAT_0071f2e0) {
            DAT_0071e518 = 0;
            DAT_00703ad4 = -1;
            DAT_00703c94 = -1;
          }
          else if (DAT_0071b716 < 0) {
            if (DAT_0071b716 == -1) {
              DAT_006d9d96 = -1;
            }
            DAT_006d9d96 = (short)(DAT_006d9d96 + 1);
            DAT_0071e518 = (short)(DAT_0071e518 +
                                   *(short *)(DAT_00730884 + 0x1934 + DAT_006d9d96 * 2));
          }
          DAT_006da53c = (short)(DAT_006da53c + -1);
          if (DAT_006da53c == DAT_0071f2e0) {
            DAT_00703ad4 = (short)(*(short *)(DAT_00730884 + 0xe) + -1);
          }
          else {
            if (DAT_006da53c < 0) {
              iStack_14 = 0;
              break;
            }
            if ((DAT_00732a20 <= DAT_006da53c) && (DAT_0071b716 < DAT_00732a20)) {
              DAT_00703c94 = (short)(DAT_00703c94 + 1);
              sVar5 = DAT_00703c94;
LAB_0040792b:
              sVar5 = *(short *)(DAT_00730884 + 0x1934 + sVar5 * 2);
LAB_00407939:
              DAT_00703ad4 = (short)(DAT_00703ad4 + sVar5);
            }
          }
          goto LAB_00407940;
        case 3:
          iStack_14 = FUN_00408bd0();
          break;
        case 4:
          DAT_0071b718 = (short)(DAT_0071b718 + -1);
          DAT_006db598 = (int)((unsigned int)DAT_006db598 - (unsigned int)DAT_00525518);
          if (DAT_0071b718 == DAT_0070a170) {
            DAT_0071e518 = 0;
            DAT_00703ad4 = -1;
            DAT_0070acd4 = -1;
          }
          else if (DAT_0071b718 < 0) {
            DAT_0070acea = (short)(DAT_0070acea + 1);
            DAT_0071e518 = (short)(DAT_0071e518 +
                                   *(short *)(DAT_00730884 + 0x1934 + DAT_0070acea * 2));
          }
          DAT_006da53e = (short)(DAT_006da53e + -1);
          if (-1 < DAT_006da53e) {
            if (DAT_006da53e == DAT_0070a170) {
              DAT_00703ad4 = (short)(*(short *)(DAT_00730884 + 0xe) + -1);
            }
            else if ((DAT_0070acec <= DAT_006da53e) && (DAT_0071b718 < DAT_0070acec)) {
              DAT_0070acd4 = (short)(DAT_0070acd4 + 1);
              sVar5 = *(short *)(DAT_00730884 + 0x1934 + DAT_0070acd4 * 2);
              goto LAB_00407939;
            }
            goto LAB_00407940;
          }
          iStack_14 = 0;
          break;
        case 5:
          iStack_14 = FUN_004092e0();
          break;
        case 6:
          iStack_14 = FUN_00409680();
          break;
        case 7:
          DAT_006db598 = (int)((unsigned int)DAT_006db598 + (unsigned int)DAT_00525518);
          DAT_0071b718 = (short)(DAT_0071b718 + 1);
          if (DAT_0071b718 == 0) {
            DAT_0071e518 = 0;
            DAT_00703ad4 = -1;
            DAT_0070acd4 = -1;
          }
          else if (DAT_0070acec <= DAT_0071b718) {
            if (DAT_0070acec == DAT_0071b718) {
              DAT_0070acea = -1;
            }
            DAT_0070acea = (short)(DAT_0070acea + 1);
            DAT_0071e518 = (short)(DAT_0071e518 +
                                   *(short *)(DAT_00730884 + 0x1934 + DAT_0070acea * 2));
          }
          DAT_006da53e = (short)(DAT_006da53e + 1);
          if (DAT_006da53e == 0) {
            DAT_00703ad4 = (short)(*(short *)(DAT_00730884 + 0xe) + -1);
          }
          else {
            if (DAT_0070acec <= DAT_006da53e) {
              iStack_14 = 0;
              break;
            }
            if ((DAT_006da53e < 0) && (-1 < DAT_0071b718)) {
              DAT_0070acd4 = (short)(DAT_0070acd4 + 1);
              sVar5 = DAT_0070acd4;
              goto LAB_0040792b;
            }
          }
LAB_00407940:
          iStack_14 = 1;
        }
        if (iStack_14 == 0) {
          return;
        }
        FUN_004079f0();
LAB_00407953:
        sStack_c = (short)(sStack_c + -1);
      } while ((sStack_c != 0) && (sVar11 != 0));
      if ((unsigned char)(bStack_6 + DAT_0071e6d6) < bStack_6) {
        bStack_5 = (unsigned char)(bStack_5 + 1);
      }
      if ((unsigned char)(bStack_5 + DAT_0071e6d7) < bStack_5) {
        bStack_7 = (unsigned char)(bStack_7 + 1);
      }
      bStack_6 = (unsigned char)(bStack_6 + DAT_0071e6d6);
      bStack_5 = (unsigned char)(bStack_5 + DAT_0071e6d7);
      bVar7 = *param_1;
    }
  }
  return;
}
