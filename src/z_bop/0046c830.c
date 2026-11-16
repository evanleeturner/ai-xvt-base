/* SPDX-License-Identifier: MIT */
/* z_bop 0046c830 */
uint FUN_0046fad0(void);
void FUN_0046c200(void);
void FUN_00470060(void);
void __cdecl FUN_0046c220(char *param_1);
void FUN_0046d350(int param_1, uint *param_2, int param_3);
uint FUN_0046fee0(int param_1);
void FUN_0046dd80(int param_1, char param_2, byte param_3, uint *param_4, uint param_5);
int FUN_0046fc00(int param_1, int param_2, uint param_3, uint param_4);

extern int *DAT_009994c4;
extern uint DAT_00999500;
extern uint DAT_0052702c;
extern int DAT_00527024;
extern uint DAT_00527030;
extern uint DAT_00527020;
extern uint DAT_0052701c;
extern int DAT_00999548;
extern uint DAT_009997d0;
extern uint DAT_009999dc;
extern uint DAT_0099f6e0;
extern char DAT_0052705c[];

extern uint DAT_005599a8[];
extern uint DAT_005599ac[];
extern uint DAT_005599b0[];
extern byte DAT_005599b4[];
extern byte DAT_005599b5[];
extern byte DAT_005599b6[];
extern byte DAT_005599b7[];
extern uint DAT_005599b8[];
extern uint DAT_005599bc[];

extern uint DAT_00999e14[];
extern uint DAT_00999bf0[];
extern uint DAT_00999bf8[];
extern uint DAT_00999c00[];
extern uint DAT_00999c04[];

extern uint DAT_005ee1bc[];
extern uint DAT_005ee1cc[];
extern byte DAT_005ee1c6[];
extern byte DAT_005ee1c8[];

extern uint DAT_005dd9bc[];
extern uint DAT_005dd9b8[];
extern byte DAT_005dd9c5[];
extern byte DAT_005dd9c6[];
extern byte DAT_005dd9c8[];

uint __stdcall FUN_0046c830(void)
{
  byte bVar1;
  uint uVar2;
  int iVar3;
  uint uVar4;
  int iVar5;
  uint uVar6;
  int iVar7;
  uint uVar8;
  uint uVar9;
  ushort *puVar10;
  uint *puVar11;
  byte *pbVar12;
  uint *puVar13;
  uint uVar14;
  byte *pbVar15;
  uint *puVar16;
  bool bVar17;
  ushort wTmp;
  int iStack_428;
  uint uStack_424;
  uint uStack_420;
  uint uStack_41c;
  uint uStack_418;
  int iStack_414;
  uint uStack_410;
  uint auStack_40c[2];
  int iStack_404;
  ushort uStack_400[0x200];

  if (DAT_009994c4 == (int *)0) {
    return 0;
  }
  DAT_00999500 = 0;
LAB_0046c852:
  do {
    do {
      while (1) {
        do {
          while (1) {
            while (1) {
              if (0x3fe < DAT_0052702c) {
                FUN_0046c200();
                FUN_00470060();
                return 0;
              }
              uStack_418 = 0x400;
              iVar3 = ((int (__stdcall *)(int *, int *, int *, int, ushort *, uint *))
                       *(uint *)(*DAT_009994c4 + 100))
                      (DAT_009994c4, &iStack_428, &iStack_404, 1, uStack_400, &uStack_418);
              if (iVar3 != 0) {
                return 0;
              }
              if (iStack_428 != 0) break;
              FUN_0046c220(DAT_0052705c);
              iVar3 = DAT_00527024;
              if (0x200 < uStack_418) {
                uStack_418 = 0x200;
              }
              iVar5 = (int)((uint)DAT_00527024 * 0x210U);
              iVar7 = (int)((uint)DAT_00527024 * 0x84U);
              DAT_005599a8[(uint)DAT_00527024 * 0x84U] = (uint)iStack_428;
              DAT_005599ac[(uint)iVar3 * 0x84U] = uStack_418;
              DAT_005599b7[iVar5] = 0;
              puVar10 = uStack_400;
              puVar13 = &DAT_005599b8[iVar7];
              for (uVar6 = uStack_418 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
                __builtin_memcpy(puVar13, puVar10, 4);
                puVar10 = puVar10 + 2;
                puVar13 = puVar13 + 1;
              }
              for (uVar6 = uStack_418 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
                *(byte *)puVar13 = *(byte *)puVar10;
                puVar10 = (ushort *)((byte *)puVar10 + 1);
                puVar13 = (uint *)((byte *)puVar13 + 1);
              }
              DAT_00527024 = (int)((uint)DAT_00527024 + 1U);
              DAT_0052702c = DAT_0052702c + 1;
              if (0x3ff < DAT_00527024) {
                DAT_00527024 = 0;
              }
            }
            if (DAT_00527030 == 0) break;
            DAT_00527030 = DAT_00527030 - 1;
          }
        } while (iStack_404 != DAT_00999548);
        puVar11 = (uint *)((byte *)uStack_400 + 2);
        uVar9 = uStack_400[0] & 0x7f;
        uVar6 = (uint)((uStack_400[0] & 0x80) != 0);
        uStack_410 = (uint)((uStack_400[0] & 0x8000) == 0);
        uStack_420 = (uint)((uStack_400[0] & 0x7f00) >> 8);
        if ((uVar9 < 0x3c) || (0x3f < uVar9)) {
          iStack_414 = 1;
        }
        else {
          iStack_414 = 0;
        }
        if ((iStack_414 != 0) && (uStack_424 = FUN_0046fad0(), uStack_424 == 0)) {
          puVar11 = (uint *)((byte *)uStack_400 + 4);
          __builtin_memcpy(&wTmp, (byte *)uStack_400 + 2, 2);
          uStack_424 = (uint)wTmp;
        }
        if (uVar9 != 0x35) break;
        auStack_40c[0] = 0x34;
        FUN_0046d350(iStack_428, auStack_40c, 4);
      }
    } while (uVar9 == 0x36);
    if (uVar9 == 0x33) {
      __builtin_memcpy(&uVar6, puVar11, 4);
      __builtin_memcpy(&uVar9, (byte *)puVar11 + 4, 4);
      uVar4 = FUN_0046fee0(iStack_428);
      if ((uVar4 < DAT_0099f6e0) && (uVar4 < 0x28)) {
        DAT_00999e14[uVar4 * 0x8e] = DAT_00999e14[uVar4 * 0x8e] + 1;
      }
      uVar8 = 0;
      uVar4 = DAT_00527020;
      do {
        if ((DAT_005ee1bc[uVar4 * 0x84] != 0) &&
            ((DAT_005ee1cc[uVar4 * 0x84] & 0x7fffffff) == uVar6)) {
          FUN_0046dd80(iStack_428, 0, DAT_005ee1c6[uVar4 * 0x210],
                       (uint *)&DAT_005ee1c8[uVar4 * 0x210], DAT_005ee1bc[uVar4 * 0x84]);
          break;
        }
        uVar8 = uVar8 + 1;
        uVar4 = uVar4 + 1;
        if (0xff < uVar4) {
          uVar4 = 0;
        }
      } while (uVar8 < 0x100);
      if (0xff < uVar8) {
        auStack_40c[0] = 0x39;
        FUN_0046dd80(iStack_428, 0, (byte)uVar9, auStack_40c, 4);
      }
      goto LAB_0046c852;
    }
    if (uVar9 == 0x38) {
      __builtin_memcpy(&uVar6, puVar11, 4);
      __builtin_memcpy(&uVar9, (byte *)puVar11 + 4, 4);
      uVar4 = FUN_0046fee0(iStack_428);
      if ((uVar4 < DAT_0099f6e0) && (uVar4 < 0x28)) {
        DAT_00999e14[uVar4 * 0x8e] = DAT_00999e14[uVar4 * 0x8e] + 1;
      }
      uVar8 = 0;
      uVar4 = DAT_0052701c;
      do {
        iVar3 = (int)(uVar4 * 0x210);
        if (DAT_005dd9bc[uVar4 * 0x84] != 0) {
          if ((uVar9 == 0) || (uVar9 == 2)) {
            if ((uint)DAT_005dd9c6[iVar3] == uVar6) {
              bVar1 = DAT_005dd9c5[iVar3];
              goto joined_r0x0046cb88;
            }
          }
          else if (((int)DAT_005dd9b8[uVar4 * 0x84] == iStack_428) &&
                   ((uint)DAT_005dd9c6[iVar3] == uVar6)) {
            bVar1 = DAT_005dd9c5[iVar3];
joined_r0x0046cb88:
            if ((uint)bVar1 == uVar9) {
              FUN_0046dd80(iStack_428, (char)uVar9, (byte)uVar6,
                           (uint *)&DAT_005dd9c8[uVar4 * 0x210], DAT_005dd9bc[uVar4 * 0x84]);
              break;
            }
          }
        }
        uVar8 = uVar8 + 1;
        uVar4 = uVar4 + 1;
        if (0x7f < uVar4) {
          uVar4 = 0;
        }
      } while (uVar8 < 0x80);
      if (0x7f < uVar8) {
        auStack_40c[0] = 0x39;
        FUN_0046dd80(iStack_428, (char)uVar9, (byte)uVar6, auStack_40c, 4);
      }
    }
    else if (uVar9 == 0x37) {
      __builtin_memcpy(&uVar6, puVar11, 4);
      __builtin_memcpy(&uVar9, (byte *)puVar11 + 4, 4);
      __builtin_memcpy(&uVar4, (byte *)puVar11 + 8, 4);
      if (uVar6 == DAT_009997d0) {
        uVar6 = 0x80;
      }
      if (uVar9 == DAT_009999dc) {
        uVar9 = 0x80;
      }
      uVar8 = FUN_0046fee0(iStack_428);
      if ((DAT_0099f6e0 <= uVar8) || (DAT_00999c04[uVar8 * 0x8e] == uVar4)) {
        uVar4 = 0x80;
      }
      uStack_41c = 0x80;
      uVar8 = DAT_0052701c;
      do {
        if (((uVar6 == 0x80) && (uVar9 == 0x80)) && (uVar4 == 0x80)) break;
        iVar3 = (int)(uVar8 * 0x210);
        uVar2 = DAT_005dd9bc[uVar8 * 0x84];
        uVar14 = uVar4;
        if (uVar2 != 0) {
          if (DAT_005dd9c5[iVar3] == 0) {
            if ((uint)DAT_005dd9c6[iVar3] == uVar6) {
              FUN_0046dd80(iStack_428, 0, (byte)uVar6, (uint *)&DAT_005dd9c8[iVar3], uVar2);
              uVar6 = 0x80;
            }
          }
          else if (DAT_005dd9c5[iVar3] == 2) {
            if ((uint)DAT_005dd9c6[iVar3] == uVar9) {
              FUN_0046dd80(iStack_428, 2, (byte)uVar9, (uint *)&DAT_005dd9c8[iVar3], uVar2);
              uVar9 = 0x80;
            }
          }
          else if (((int)DAT_005dd9b8[uVar8 * 0x84] == iStack_428) &&
                   ((uint)DAT_005dd9c6[iVar3] == uVar4)) {
            uVar14 = 0x80;
            FUN_0046dd80(iStack_428, 1, (byte)uVar4, (uint *)&DAT_005dd9c8[iVar3], uVar2);
          }
        }
        uVar8 = uVar8 + 1;
        if (0x7f < uVar8) {
          uVar8 = 0;
        }
        uStack_41c = uStack_41c - 1;
        uVar4 = uVar14;
      } while (uStack_41c != 0);
    }
    else {
      FUN_0046fee0(iStack_428);
      if ((uStack_410 == 0) || (uVar6 == 0)) {
        uVar4 = FUN_0046fee0(iStack_428);
        if (iStack_414 != 0) {
          iVar3 = 0x7f;
          if (uStack_420 != 0) {
            iVar3 = (int)(uStack_420 - 1);
          }
          iVar3 = FUN_0046fc00(iStack_428, iVar3, uStack_410, uVar6);
          if (iVar3 == 0) {
            if (((uVar4 < DAT_0099f6e0) && (uVar4 < 0x28)) && (uVar6 == 0)) {
              DAT_00999e14[uVar4 * 0x8e] = DAT_00999e14[uVar4 * 0x8e] + 1;
            }
            uVar4 = (uint)*((byte *)puVar11 + uStack_424);
            pbVar12 = (byte *)puVar11 + uStack_424 + 1;
            if (uVar4 != 0x39) {
              uVar8 = uStack_418 - ((uint)(pbVar12 - (byte *)uStack_400) & 0xffffU);
              if (0x1fc < uVar8) {
                uVar8 = 0x1fc;
              }
              DAT_005599b8[(uint)DAT_00527024 * 0x84U] = uVar4;
              pbVar15 = (byte *)&DAT_005599bc[(uint)DAT_00527024 * 0x84U];
              for (uVar4 = uVar8 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
                __builtin_memcpy(pbVar15, pbVar12, 4);
                pbVar12 = pbVar12 + 4;
                pbVar15 = pbVar15 + 4;
              }
              for (uVar4 = uVar8 & 3; iVar3 = DAT_00527024, uVar4 != 0; uVar4 = uVar4 - 1) {
                *pbVar15 = *pbVar12;
                pbVar12 = pbVar12 + 1;
                pbVar15 = pbVar15 + 1;
              }
              iVar7 = (int)((uint)DAT_00527024 * 0x210U);
              DAT_005599a8[(uint)DAT_00527024 * 0x84U] = (uint)iStack_428;
              DAT_005599ac[(uint)iVar3 * 0x84U] = uVar8 + 4;
              DAT_005599b0[(uint)iVar3 * 0x84U] = 0;
              DAT_005599b4[iVar7] = 0;
              DAT_005599b7[iVar7] = 0;
              if (uStack_410 == 0) {
                DAT_005599b5[iVar7] = 2;
                if (uVar6 == 0) {
                  DAT_005599b5[iVar7] = 1;
                }
              }
              else {
                DAT_005599b5[iVar7] = 0;
              }
              if (uStack_420 == 0) {
                DAT_005599b6[iVar7] = 0x7f;
              }
              else {
                DAT_005599b6[iVar7] = (byte)(uStack_420 - 1);
              }
              DAT_00527024 = (int)((uint)DAT_00527024 + 1U);
              DAT_0052702c = DAT_0052702c + 1;
              if (0x3ff < DAT_00527024) {
                DAT_00527024 = 0;
              }
            }
          }
        }
        uStack_41c = (uint)FUN_0046fc00(iStack_428, (int)uStack_420, uStack_410, uVar6);
        if ((uVar6 == 0) || (uStack_41c == 0)) {
          puVar13 = (uint *)((byte *)uStack_400 + 2);
          if (iStack_414 == 0) {
            uStack_424 = uStack_418 - 2;
          }
          else {
            uStack_424 = FUN_0046fad0();
            if (uStack_424 == 0) {
              puVar13 = (uint *)((byte *)uStack_400 + 4);
              __builtin_memcpy(&wTmp, (byte *)uStack_400 + 2, 2);
              uStack_424 = (uint)wTmp;
            }
          }
          if (0x1fc < uStack_424) {
            uStack_424 = 0x1fc;
          }
          DAT_005599b8[(uint)DAT_00527024 * 0x84U] = uVar9;
          puVar16 = &DAT_005599bc[(uint)DAT_00527024 * 0x84U];
          for (uVar9 = uStack_424 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
            __builtin_memcpy(puVar16, puVar13, 4);
            puVar13 = puVar13 + 1;
            puVar16 = puVar16 + 1;
          }
          for (uVar9 = uStack_424 & 3; iVar3 = DAT_00527024, uVar9 != 0; uVar9 = uVar9 - 1) {
            *(byte *)puVar16 = *(byte *)puVar13;
            puVar13 = (uint *)((byte *)puVar13 + 1);
            puVar16 = (uint *)((byte *)puVar16 + 1);
          }
          iVar7 = (int)((uint)DAT_00527024 * 0x210U);
          DAT_005599a8[(uint)DAT_00527024 * 0x84U] = (uint)iStack_428;
          DAT_005599ac[(uint)iVar3 * 0x84U] = uStack_424 + 4;
          DAT_005599b0[(uint)iVar3 * 0x84U] = 0;
          DAT_005599b4[iVar7] = 0;
          DAT_005599b7[iVar7] = 1;
          if (uStack_41c == 0) {
            DAT_005599b7[iVar7] = 0;
          }
          if (uStack_410 == 0) {
            DAT_005599b5[iVar7] = 2;
            if (uVar6 == 0) {
              DAT_005599b5[iVar7] = 1;
            }
          }
          else {
            DAT_005599b5[iVar7] = 0;
          }
          DAT_0052702c = DAT_0052702c + 1;
          DAT_005599b6[iVar7] = (byte)uStack_420;
          DAT_00527024 = (int)((uint)DAT_00527024 + 1U);
          if (0x3ff < DAT_00527024) {
            DAT_00527024 = 0;
          }
        }
      }
      else {
        bVar17 = ((byte *)uStack_400)[2] != 0;
        if (bVar17) {
          uStack_41c = (uint)(((byte *)uStack_400)[2] == 2);
        }
        else {
          uStack_41c = 0;
        }
        puVar13 = (uint *)((byte *)uStack_400 + 3);
        if (iStack_414 == 0) {
          uStack_424 = uStack_418 - 2;
        }
        else {
          uStack_424 = FUN_0046fad0();
          if (uStack_424 == 0) {
            puVar13 = (uint *)((byte *)uStack_400 + 5);
            __builtin_memcpy(&wTmp, (byte *)uStack_400 + 3, 2);
            uStack_424 = (uint)wTmp;
          }
        }
        if (0x1fc < uStack_424) {
          uStack_424 = 0x1fc;
        }
        DAT_005599b8[(uint)DAT_00527024 * 0x84U] = uVar9;
        puVar16 = &DAT_005599bc[(uint)DAT_00527024 * 0x84U];
        for (uVar6 = uStack_424 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
          __builtin_memcpy(puVar16, puVar13, 4);
          puVar13 = puVar13 + 1;
          puVar16 = puVar16 + 1;
        }
        for (uVar6 = uStack_424 & 3; iVar3 = DAT_00527024, uVar6 != 0; uVar6 = uVar6 - 1) {
          *(byte *)puVar16 = *(byte *)puVar13;
          puVar13 = (uint *)((byte *)puVar13 + 1);
          puVar16 = (uint *)((byte *)puVar16 + 1);
        }
        iVar7 = (int)((uint)DAT_00527024 * 0x210U);
        DAT_005599a8[(uint)DAT_00527024 * 0x84U] = (uint)iStack_428;
        DAT_005599ac[(uint)iVar3 * 0x84U] = uStack_424 + 4;
        DAT_005599b0[(uint)iVar3 * 0x84U] = 0;
        DAT_005599b4[iVar7] = 0;
        DAT_005599b7[iVar7] = 1;
        uVar6 = FUN_0046fee0(iStack_428);
        if (bVar17) {
          if (uStack_41c == 0) {
            bVar17 = uVar6 < DAT_0099f6e0;
            DAT_005599b5[(uint)DAT_00527024 * 0x210U] = 1;
            if ((bVar17) && (uVar6 < 0x28)) {
              puVar11 = &DAT_00999bf0[uVar6 * 0x8e];
              uVar6 = DAT_00999bf0[uVar6 * 0x8e] + 1;
              if (0x7f < uVar6) {
                uVar6 = 0;
              }
              goto joined_r0x0046cf8b;
            }
          }
          else {
            bVar17 = uVar6 < DAT_0099f6e0;
            DAT_005599b5[(uint)DAT_00527024 * 0x210U] = 2;
            if ((bVar17) && (uVar6 < 0x28)) {
              puVar11 = &DAT_00999c00[uVar6 * 0x8e];
              uVar6 = DAT_00999c00[uVar6 * 0x8e] + 1;
              if (0x7f < uVar6) {
                uVar6 = 0;
              }
joined_r0x0046cf8b:
              if (uVar6 == uStack_420) {
                *puVar11 = uVar6;
              }
            }
          }
        }
        else {
          bVar17 = uVar6 < DAT_0099f6e0;
          DAT_005599b5[(uint)DAT_00527024 * 0x210U] = 0;
          if ((bVar17) && (uVar6 < 0x28)) {
            puVar11 = &DAT_00999bf8[uVar6 * 0x8e];
            uVar6 = DAT_00999bf8[uVar6 * 0x8e] + 1;
            if (0x7f < uVar6) {
              uVar6 = 0;
            }
            goto joined_r0x0046cf8b;
          }
        }
        DAT_0052702c = DAT_0052702c + 1;
        DAT_005599b6[(uint)DAT_00527024 * 0x210U] = (byte)uStack_420;
        DAT_00527024 = (int)((uint)DAT_00527024 + 1U);
        if (0x3ff < DAT_00527024) {
          DAT_00527024 = 0;
        }
      }
    }
  } while (1);
}
