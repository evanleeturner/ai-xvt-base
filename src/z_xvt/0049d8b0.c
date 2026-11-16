/* SPDX-License-Identifier: MIT */
/* z_xvt 0049d8b0 */
extern unsigned short DAT_00645adc;
extern unsigned short DAT_00645ad8;
extern unsigned char DAT_0052acd8;
extern unsigned char DAT_00709d0a;
extern unsigned char DAT_00709d0c;
extern unsigned char DAT_00709ce8;
extern unsigned char DAT_00709ceb;
extern unsigned char DAT_00709f3a;
extern unsigned char DAT_00709f30;
extern unsigned char DAT_0071b73a;
extern unsigned char DAT_0071b73c;
extern unsigned char DAT_0071b740;
extern short DAT_006d49a0;
extern int DAT_006d4980;
extern int DAT_006d4988;
extern int _DAT_00732a3c;
extern char DAT_0070ad40;
extern unsigned char DAT_0070ad42;
extern unsigned char DAT_0070ad47;
extern unsigned char DAT_0070ad48;
extern unsigned char DAT_0070ad54;
extern unsigned char DAT_0070b252;
extern unsigned char DAT_0052b083;
extern unsigned char DAT_0052b0b4;
extern unsigned char DAT_0052b0b6;

extern short DAT_006d4a6a;
extern short DAT_006d4a8c;
extern short DAT_006d4a8e;
extern short DAT_006d4a90;
extern short DAT_006d4a92;
extern unsigned char DAT_006d4a6c;
extern unsigned char DAT_006d4a6e;
extern unsigned char DAT_006d4a70;
extern unsigned char DAT_006d4a72;
extern unsigned char DAT_006d4a74;
extern unsigned char DAT_006d4a76;
extern unsigned char DAT_006d4a78;
extern unsigned char DAT_006d4a7a;
extern unsigned char DAT_006d4a7c;
extern unsigned char DAT_006d4a7e;
extern unsigned char DAT_006d4a80;
extern unsigned char DAT_006d4a82;
extern unsigned char DAT_006d4a84;
extern unsigned char DAT_006d4a86;
extern unsigned char DAT_006d4a88;
extern unsigned char DAT_006d4a8a;
extern unsigned char DAT_006d4a94;
extern unsigned char DAT_006d4a96;
extern unsigned char DAT_006d4a98;
extern unsigned char DAT_006d4a9a;
extern unsigned char DAT_006d4a9c;
extern unsigned char DAT_006d4a9e;
extern unsigned char DAT_006d4aa0;
extern unsigned char DAT_006d4aa2;
extern unsigned char DAT_006d4aa3;
extern unsigned char DAT_006d4aa4;
extern unsigned char DAT_006d4aa5;
extern unsigned char DAT_006d4aa6;
extern unsigned char DAT_006d4aa7;
extern unsigned char DAT_006d4aa8;
extern unsigned char DAT_006d4aa9;
extern unsigned char DAT_006d4aaa;
extern unsigned char DAT_006d4aab;
extern unsigned char DAT_006d4aac;
extern unsigned char DAT_006d4aad;
extern unsigned char DAT_006d4aae;
extern unsigned char DAT_006d4aaf;
extern unsigned char DAT_006d4ab0;
extern unsigned char DAT_006d4ab1;
extern unsigned char DAT_006d4ab2;
extern unsigned char DAT_006d4ab3;
extern unsigned char DAT_006d4ab4;
extern unsigned char DAT_006d4ab5;
extern unsigned char DAT_006d4ab6;
extern unsigned char DAT_006d4ab7;
extern unsigned char DAT_006d4ab8;
extern unsigned char DAT_006d4ab9;
extern unsigned char DAT_006d4aba;
extern unsigned char DAT_006d4abb;
extern unsigned char DAT_006d4abc;
extern unsigned char DAT_006d4abd;
extern unsigned char DAT_006d4abe;
extern unsigned char DAT_006d4ac8;
extern unsigned char DAT_006d4ad2;
extern unsigned char DAT_006d4adc;
extern unsigned char DAT_006d4ae6;
extern unsigned char DAT_006d4af0;
extern unsigned char DAT_006d4afa;
extern unsigned char DAT_006d4b04;

unsigned int __cdecl FUN_004783e0(unsigned short param_1, unsigned short param_2);
short __cdecl FUN_0049ec30(unsigned short param_1, short param_2, unsigned short param_3);

char __cdecl
FUN_0049d8b0(unsigned short param_1, short param_2, unsigned short param_3,
             unsigned short param_4, short param_5, unsigned short param_6)
{
  int iVar1;
  int iVar2;
  short *psVar3;
  _Bool bVar4;
  short sVar5;
  unsigned short uVar6;
  unsigned int uVar7;
  unsigned short uVar8;
  unsigned short uVar9;
  unsigned short uVar10;
  unsigned int uVar11;
  unsigned int uVar12;
  char cVar13;
  unsigned short local_5a;
  unsigned short uStack_58;
  unsigned short uStack_56;
  unsigned short local_54;
  unsigned int local_50;
  char *local_4c;
  int local_40[3];
  int local_34[3];
  unsigned int local_28[5];
  unsigned int local_14[5];
  int local_60;
  short local_62;
  short local_64;
  short local_66;
  unsigned short local_68;
  unsigned short local_6a;
  unsigned short local_6c;

  DAT_00645adc = 0;
  DAT_00645ad8 = 0;
  uVar11 = (unsigned int)param_1;
  if ((&DAT_0052acd8)[uVar11] == 0) {
    cVar13 = 4;
    switch (uVar11) {
    case 0:
      return 1;
    case 10:
      return 0;
    case 0xd:
      if (param_2 == 0xc) {
        if ((&DAT_00709d0a)[(unsigned int)param_3 * 3] == 1) {
          return 1;
        }
        if ((&DAT_00709d0a)[(unsigned int)param_3 * 3] == 2) {
          return 2;
        }
      } else {
        if (DAT_00709ce8 == 1) {
          return 1;
        }
        if (DAT_00709ce8 == 2) {
          return 2;
        }
      }
      break;
    case 0xe:
      if (param_2 == 0xc) {
        if ((&DAT_00709d0a)[(unsigned int)param_3 * 3] == 2) {
          return 1;
        }
        if ((&DAT_00709d0a)[(unsigned int)param_3 * 3] == 1) {
          return 2;
        }
      } else {
        if (DAT_00709ce8 == 2) {
          return 1;
        }
        if (DAT_00709ce8 == 1) {
          return 2;
        }
      }
      break;
    case 0x11:
      if (param_2 == 0xc) {
        if ((&DAT_00709d0c)[(unsigned int)param_3 * 3] == 2) {
          return 1;
        }
        if ((&DAT_00709d0c)[(unsigned int)param_3 * 3] == 1) {
          return 2;
        }
      } else {
        if (DAT_00709ceb == 1) {
          return 1;
        }
        if (DAT_00709ceb == 2) {
          return 2;
        }
      }
      break;
    case 0x12:
      if (DAT_00709ceb == 2) {
        return 1;
      }
      if (DAT_00709ceb == 1) {
        return 2;
      }
      break;
    case 0x14:
      return (char)(((&DAT_00709f3a)[param_3] == 0) + 1);
    case 0x26:
      return 2;
    case 0x29:
    case 0x2a:
      cVar13 = 2;
      uVar9 = 0;
      do {
        uVar8 = (unsigned short)(uVar9 + 1);
        *(unsigned short *)((int)local_14 + (unsigned int)uVar9 * 2) = 0;
        *(unsigned short *)((int)local_28 + (unsigned int)uVar9 * 2) = 0;
        uVar9 = uVar8;
      } while (uVar8 < 10);
      uVar9 = 0;
      do {
        uVar8 = (unsigned short)(uVar9 + 1);
        *(unsigned short *)((int)local_40 + (unsigned int)uVar9 * 2) = 0;
        *(unsigned short *)((int)local_34 + (unsigned int)uVar9 * 2) = 0;
        uVar9 = uVar8;
      } while (uVar8 < 6);
      uVar11 = 0;
      do {
        iVar1 = (int)(uVar11 * 0x5bd);
        if ((&DAT_0071b740)[iVar1] != 0) {
          __builtin_memcpy(&local_6a, (unsigned char *)((int)&DAT_0071b73c + iVar1), 2);
          psVar3 = (short *)((int)local_14 + (unsigned int)local_6a * 2);
          *psVar3 = (short)(*psVar3 + 1);
          __builtin_memcpy(&local_6a, (unsigned char *)((int)&DAT_0071b73a + iVar1), 2);
          psVar3 = (short *)((int)local_40 + (unsigned int)local_6a * 2);
          *psVar3 = (short)(*psVar3 + 1);
        }
        uVar9 = (unsigned short)((short)uVar11 + 1);
        uVar11 = (unsigned int)uVar9;
      } while (uVar9 < 8);
      uVar11 = 0;
      if (0 < DAT_006d49a0) {
        do {
          if ((&DAT_0070ad54)[uVar11 * 0x566] != 0) {
            psVar3 = (short *)((int)local_28 + (unsigned int)(unsigned char)(&DAT_0070ad48)[uVar11 * 0x566] * 2);
            *psVar3 = (short)(*psVar3 + 1);
            psVar3 = (short *)((int)local_34 + (unsigned int)(unsigned char)(&DAT_0070ad47)[uVar11 * 0x566] * 2);
            *psVar3 = (short)(*psVar3 + 1);
          }
          uVar9 = (unsigned short)((short)uVar11 + 1);
          uVar11 = (unsigned int)uVar9;
        } while ((int)(unsigned int)uVar9 < (int)DAT_006d49a0);
      }
      if (param_2 == 0xd) {
        if (param_1 == 0x29) {
          uVar9 = 0;
          if (0 < DAT_006d49a0) {
            do {
              if ((((&DAT_0070ad54)[(unsigned int)uVar9 * 0x566] != 0) &&
                   ((unsigned int)(unsigned char)(&DAT_0070ad54)[(unsigned int)uVar9 * 0x566] - (unsigned int)param_3 == 1))) {
                __builtin_memcpy(&local_60, (unsigned char *)((int)&DAT_0070b252 + (unsigned int)uVar9 * 0x566), 4);
                if (local_60 != -1) {
                  cVar13 = 1;
                }
              }
              uVar9 = (unsigned short)(uVar9 + 1);
            } while ((int)(unsigned int)uVar9 < (int)DAT_006d49a0);
            return cVar13;
          }
        } else {
          uVar9 = 0;
          if (0 < DAT_006d49a0) {
            do {
              if ((((&DAT_0070ad54)[(unsigned int)uVar9 * 0x566] != 0) &&
                   ((unsigned int)(unsigned char)(&DAT_0070ad54)[(unsigned int)uVar9 * 0x566] - (unsigned int)param_3 == 1))) {
                __builtin_memcpy(&local_60, (unsigned char *)((int)&DAT_0070b252 + (unsigned int)uVar9 * 0x566), 4);
                if (local_60 == -1) {
                  cVar13 = 1;
                }
              }
              uVar9 = (unsigned short)(uVar9 + 1);
            } while ((int)(unsigned int)uVar9 < (int)DAT_006d49a0);
            return cVar13;
          }
        }
      } else {
        uVar11 = 0;
        uVar12 = 0;
        if (param_2 == 0xc) {
          iVar1 = (int)((unsigned int)param_3 * 2);
          uVar11 = (unsigned int)*(unsigned short *)((int)local_28 + iVar1);
          if (param_1 == 0x29) {
            uVar12 = (unsigned int)*(unsigned short *)((int)local_14 + iVar1);
          } else {
            uVar12 = (unsigned int)(unsigned short)(*(unsigned short *)((int)local_28 + iVar1) -
                                   *(short *)((int)local_14 + iVar1));
          }
        } else if (param_2 == 5) {
          iVar1 = (int)((unsigned int)param_3 * 2);
          uVar11 = (unsigned int)*(unsigned short *)((int)local_34 + iVar1);
          if (param_1 == 0x29) {
            uVar12 = (unsigned int)*(unsigned short *)((int)local_40 + iVar1);
          } else {
            uVar12 = (unsigned int)(unsigned short)(*(unsigned short *)((int)local_34 + iVar1) -
                                   *(short *)((int)local_40 + iVar1));
          }
        }
        uVar8 = (unsigned short)uVar12;
        uVar9 = (unsigned short)uVar11;
        switch (param_4) {
        case 0:
          if (uVar8 == uVar9) {
            return 1;
          }
          break;
        case 1:
          uVar7 = FUN_004783e0(uVar8, uVar9);
          if (0xbfff < (unsigned short)uVar7) {
            return 1;
          }
          break;
        case 2:
          uVar7 = FUN_004783e0(uVar8, uVar9);
          if (0x7fff < (unsigned short)uVar7) {
            return 1;
          }
          break;
        case 3:
          uVar7 = FUN_004783e0(uVar8, uVar9);
          if (0x3fff < (unsigned short)uVar7) {
            return 1;
          }
          break;
        case 4:
          if (uVar8 != 0) {
            return 1;
          }
          break;
        case 5:
          if (uVar11 - uVar12 == 1) {
            return 1;
          }
          break;
        case 0x10:
          uVar7 = FUN_004783e0(uVar8, uVar9);
          if (0xaaa9 < (unsigned short)uVar7) {
            return 1;
          }
          break;
        case 0x11:
          uVar7 = FUN_004783e0(uVar8, uVar9);
          if (0x5554 < (unsigned short)uVar7) {
            cVar13 = 1;
          }
        }
      }
    }
    return cVar13;
  }
  if (param_2 == 0) {
    return 2;
  }
  uStack_56 = 0;
  local_54 = 0;
  uVar10 = 0;
  uVar8 = 0;
  uVar9 = 0;
  local_50 = 0;
  uStack_58 = 0;
  if (0 < DAT_006d49a0) {
    do {
      uVar12 = (unsigned int)uStack_58;
      iVar1 = (int)(uVar12 * 0x566);
      local_14[0] = uVar12;
      if (((&DAT_0070ad42)[iVar1] == 0) ||
          (sVar5 = FUN_0049ec30(uStack_58, param_2, param_3), sVar5 == 0))
        goto switchD_0049d9e5_caseD_a;
      sVar5 = (short)((unsigned short)local_50 + (&DAT_006d4a6a)[uVar12 * 0x93]);
      local_50 = ((unsigned int)(unsigned short)(local_50 >> 16) << 16) | (unsigned short)sVar5;
      iVar2 = (int)(uVar12 * 0x126);
      uStack_56 = (unsigned short)(uStack_56 + (unsigned char)(&DAT_006d4aa2)[uVar12 * 0x126]);
      local_54 = (unsigned short)(local_54 + *(short *)(&DAT_006d4a6c + iVar2));
      uVar6 = (unsigned short)(unsigned char)(&DAT_006d4aa3)[iVar2];
      if (0x2d < uVar11 - 1) goto switchD_0049d9e5_caseD_a;
      switch (uVar11) {
      case 1:
        uVar10 = (unsigned short)(uVar10 + *(short *)(&DAT_006d4a6c + iVar2));
        uVar8 = (unsigned short)(uVar8 + (unsigned char)(&DAT_006d4aa3)[iVar2]);
        goto switchD_0049d9e5_caseD_a;
      case 2:
        uVar10 = (unsigned short)(uVar10 + *(short *)(&DAT_006d4a6e + iVar2) + *(short *)(&DAT_006d4a70 + iVar2));
        uVar8 = (unsigned short)(uVar8 + (unsigned char)(&DAT_006d4aa4)[iVar2] + (unsigned short)(unsigned char)(&DAT_006d4aa5)[iVar2]);
        if (param_5 == 0) {
          uVar9 = (unsigned short)(uVar9 + (&DAT_006d4a8c)[uVar12 * 0x93]);
          uVar6 = (unsigned short)(unsigned char)(&DAT_006d4ab3)[iVar2];
          break;
        }
        uVar10 = (unsigned short)(uVar10 + (&DAT_006d4a8c)[uVar12 * 0x93] + (&DAT_006d4a8e)[uVar12 * 0x93] +
                 (&DAT_006d4a90)[uVar12 * 0x93] + (&DAT_006d4a92)[uVar12 * 0x93]);
        uVar8 = (unsigned short)(uVar8 + (unsigned char)(&DAT_006d4ab3)[iVar2] + (unsigned short)(unsigned char)(&DAT_006d4ab4)[iVar2] +
                (unsigned short)(unsigned char)(&DAT_006d4ab5)[iVar2] + (unsigned short)(unsigned char)(&DAT_006d4ab6)[iVar2]);
        goto switchD_0049d9e5_caseD_a;
      case 3:
        uVar10 = (unsigned short)(uVar10 + *(short *)(&DAT_006d4a72 + iVar2));
        uVar9 = (unsigned short)(uVar9 + *(short *)(&DAT_006d4a74 + iVar2));
        uVar8 = (unsigned short)(uVar8 + (unsigned char)(&DAT_006d4aa6)[iVar2]);
        uVar6 = (unsigned short)(unsigned char)(&DAT_006d4aa7)[iVar2];
        break;
      case 4:
        uVar10 = (unsigned short)(uVar10 + *(short *)(&DAT_006d4a76 + iVar2));
        uVar9 = (unsigned short)(uVar9 + *(short *)(&DAT_006d4a78 + iVar2));
        uVar8 = (unsigned short)(uVar8 + (unsigned char)(&DAT_006d4aa8)[iVar2]);
        uVar6 = (unsigned short)(unsigned char)(&DAT_006d4aa9)[iVar2];
        break;
      case 5:
        if (param_6 < 10) {
          iVar1 = (int)((unsigned int)param_6 + iVar2);
          uVar10 = (unsigned short)(uVar10 + (unsigned char)(&DAT_006d4abe)[iVar2 + (unsigned int)param_6]);
          uVar8 = (unsigned short)(uVar8 + (unsigned char)(&DAT_006d4ac8)[iVar1]);
          uVar9 = (unsigned short)(uVar9 + (unsigned char)(&DAT_006d4ad2)[iVar1]);
          uVar6 = (unsigned short)(unsigned char)(&DAT_006d4adc)[iVar1];
        } else {
          uVar10 = (unsigned short)(uVar10 + *(short *)(&DAT_006d4a7a + iVar2));
          uVar9 = (unsigned short)(uVar9 + *(short *)(&DAT_006d4a7c + iVar2));
          uVar8 = (unsigned short)(uVar8 + (unsigned char)(&DAT_006d4aaa)[iVar2]);
          uVar6 = (unsigned short)(unsigned char)(&DAT_006d4aab)[iVar2];
        }
        break;
      case 6:
        uVar10 = (unsigned short)(uVar10 + *(short *)(&DAT_006d4a7e + iVar2));
        uVar9 = (unsigned short)(uVar9 + *(short *)(&DAT_006d4a80 + iVar2));
        uVar8 = (unsigned short)(uVar8 + (unsigned char)(&DAT_006d4aac)[iVar2]);
        uVar6 = (unsigned short)(unsigned char)(&DAT_006d4aad)[iVar2];
        break;
      case 7:
        uVar10 = (unsigned short)(uVar10 + *(short *)(&DAT_006d4a82 + iVar2));
        uVar9 = (unsigned short)(uVar9 + *(short *)(&DAT_006d4a84 + iVar2));
        uVar8 = (unsigned short)(uVar8 + (unsigned char)(&DAT_006d4aae)[iVar2]);
        uVar6 = (unsigned short)(unsigned char)(&DAT_006d4aaf)[iVar2];
        break;
      case 8:
        uVar10 = (unsigned short)(uVar10 + *(short *)(&DAT_006d4a86 + iVar2));
        uVar9 = (unsigned short)(uVar9 + *(short *)(&DAT_006d4a88 + iVar2));
        uVar8 = (unsigned short)(uVar8 + (unsigned char)(&DAT_006d4ab0)[iVar2]);
        uVar6 = (unsigned short)(unsigned char)(&DAT_006d4ab1)[iVar2];
        break;
      case 9:
        uVar10 = (unsigned short)((uVar10 - *(short *)(&DAT_006d4a70 + iVar2)) +
                 ((&DAT_006d4a6a)[uVar12 * 0x93] - *(short *)(&DAT_006d4a6e + iVar2)));
        uVar6 = (unsigned short)(unsigned char)(&DAT_006d4aa5)[iVar2];
        uVar8 = (unsigned short)((uVar8 - (unsigned char)(&DAT_006d4aa5)[iVar2]) +
                ((unsigned short)(unsigned char)(&DAT_006d4aa2)[uVar12 * 0x126] -
                 (unsigned short)(unsigned char)(&DAT_006d4aa4)[iVar2]));
        uVar9 = (unsigned short)(uVar9 + *(short *)(&DAT_006d4a6e + iVar2) + *(short *)(&DAT_006d4a70 + iVar2));
        local_50 = ((unsigned int)(unsigned short)((unsigned short)(local_50 >> 16) + (unsigned char)(&DAT_006d4aa4)[iVar2]) << 16) | (unsigned short)sVar5;
        break;
      default:
        goto switchD_0049d9e5_caseD_a;
      case 0xc:
        uVar10 = (unsigned short)(uVar10 + *(short *)(&DAT_006d4a8a + iVar2));
        uVar8 = (unsigned short)(uVar8 + (unsigned char)(&DAT_006d4ab2)[iVar2]);
        uVar9 = (unsigned short)(uVar9 + *(short *)(&DAT_006d4a96 + iVar2) + *(short *)(&DAT_006d4a94 + iVar2) +
                *(short *)(&DAT_006d4a6e + iVar2));
        local_50 = ((unsigned int)(unsigned short)((unsigned short)(local_50 >> 16) + (unsigned char)(&DAT_006d4ab8)[iVar2] +
                    (unsigned short)(unsigned char)(&DAT_006d4ab7)[iVar2]) << 16) | (unsigned short)sVar5;
        uVar6 = (unsigned short)(unsigned char)(&DAT_006d4aa4)[iVar2];
        break;
      case 0x15:
      case 0x16:
      case 0x17:
      case 0x18:
      case 0x22:
      case 0x23:
      case 0x24:
      case 0x25:
        if (_DAT_00732a3c < DAT_006d4980) {
          local_4c = (char *)((int)((unsigned int)_DAT_00732a3c * 0x23u + 3u + (unsigned int)DAT_006d4988));
          local_40[0] = (int)((unsigned int)DAT_006d4980 - (unsigned int)_DAT_00732a3c);
          do {
            if ((*local_4c != 0) && ((unsigned char)local_4c[0x13] == local_14[0])) {
              __builtin_memcpy(&local_60, local_4c + 0x1c, 4);
              __builtin_memcpy(&iVar2, (char *)local_60 + 0xa7, 4);
              bVar4 = 0;
              local_34[0] = (int)((unsigned int)*(unsigned char *)(iVar2 + 4) * 0xef);
              __builtin_memcpy(&local_62, &DAT_0052b083 + local_34[0], 2);
              sVar5 = (short)(local_62 * 2);
              __builtin_memcpy(&local_64, (char *)(iVar2 + 0x20b), 2);
              __builtin_memcpy(&local_66, (char *)(iVar2 + 0x20d), 2);
              if (param_1 == 0x15) {
                if ((int)local_64 + (int)local_66 < 1)
                  goto LAB_0049de8f;
              } else if (param_1 == 0x23) {
                if ((int)local_64 + (int)local_66 <=
                    (int)(short)((int)((unsigned int)(int)sVar5 +
                                       ((unsigned int)((int)sVar5 >> 0x1f) & 3U)) >> 2))
                  goto LAB_0049de8f;
              } else if (param_1 == 0x22) {
                if ((int)local_64 + (int)local_66 <= (int)(sVar5 / 2)) goto LAB_0049de8f;
              } else if (param_1 == 0x16) {
                __builtin_memcpy(&local_68, (char *)(iVar2 + 199), 2);
                __builtin_memcpy(&local_6c, (char *)(iVar2 + 0xc3), 2);
                if ((unsigned short)(local_68 >> 1) < local_6c) goto LAB_0049de8f;
              } else if (param_1 == 0x24) {
                __builtin_memcpy(&local_68, (char *)(iVar2 + 199), 2);
                __builtin_memcpy(&local_6c, (char *)(iVar2 + 0xc3), 2);
                if ((unsigned short)(local_68 >> 2) < local_6c) goto LAB_0049de8f;
              } else if (param_1 == 0x25) {
                __builtin_memcpy(&local_68, (char *)(iVar2 + 199), 2);
                __builtin_memcpy(&local_6c, (char *)(iVar2 + 0xc3), 2);
                if ((unsigned short)((local_68 >> 2) * 3) < local_6c)
                  goto LAB_0049de8f;
              } else if (param_1 == 0x17) {
                sVar5 = 0;
                local_5a = 0;
                if (*(unsigned char *)(iVar2 + 0x228) != 0) {
                  local_28[0] = (unsigned int)*(unsigned char *)(iVar2 + 0x228);
                  do {
                    uVar6 = (unsigned short)(local_5a + 1);
                    sVar5 = (short)(sVar5 + (unsigned short)*(unsigned char *)(iVar2 + 0x346 +
                                                     (unsigned int)(unsigned char)(&DAT_0052b0b4)
                                                                 [local_34[0] + (unsigned int)local_5a] * 4)
                            + (unsigned short)*(unsigned char *)(iVar2 + 0x346 +
                                               (unsigned int)(unsigned char)(&DAT_0052b0b6)
                                                           [local_34[0] + (unsigned int)local_5a] * 4));
                    local_5a = uVar6;
                  } while (uVar6 < local_28[0]);
                }
                if (sVar5 == 0) {
LAB_0049de8f:
                  bVar4 = 1;
                }
              } else if ((*(unsigned char *)(iVar2 + 0x1df) & 0x10) == 0) goto LAB_0049de8f;
              if (bVar4) {
                uVar10 = (unsigned short)(uVar10 + 1);
                if ((&DAT_0070ad40)[iVar1] == *(char *)(iVar2 + 0xb0)) {
                  uVar8 = (unsigned short)(uVar8 + 1);
                }
              } else {
                uVar9 = (unsigned short)(uVar9 + 1);
                if ((&DAT_0070ad40)[iVar1] == *(char *)(iVar2 + 0xb0)) {
                  local_50 = ((unsigned int)(unsigned short)((unsigned short)(local_50 >> 16) + 1) << 16) | (unsigned short)local_50;
                }
              }
            }
            local_4c = local_4c + 0x23;
            local_40[0] = local_40[0] + -1;
          } while (local_40[0] != 0);
          local_40[0] = 0;
        }
        goto switchD_0049d9e5_caseD_a;
      case 0x19:
        uVar9 = (unsigned short)(uVar9 + *(short *)(&DAT_006d4a6c + iVar2));
        break;
      case 0x1a:
        uVar10 = (unsigned short)(uVar10 + *(short *)(&DAT_006d4a74 + iVar2));
        uVar9 = (unsigned short)(uVar9 + *(short *)(&DAT_006d4a72 + iVar2));
        uVar8 = (unsigned short)(uVar8 + (unsigned char)(&DAT_006d4aa7)[iVar2]);
        uVar6 = (unsigned short)(unsigned char)(&DAT_006d4aa6)[iVar2];
        break;
      case 0x1b:
        uVar10 = (unsigned short)(uVar10 + *(short *)(&DAT_006d4a88 + iVar2));
        uVar9 = (unsigned short)(uVar9 + *(short *)(&DAT_006d4a86 + iVar2));
        uVar8 = (unsigned short)(uVar8 + (unsigned char)(&DAT_006d4ab1)[iVar2]);
        uVar6 = (unsigned short)(unsigned char)(&DAT_006d4ab0)[iVar2];
        break;
      case 0x1c:
        uVar10 = (unsigned short)(uVar10 + *(short *)(&DAT_006d4a78 + iVar2));
        uVar9 = (unsigned short)(uVar9 + *(short *)(&DAT_006d4a76 + iVar2));
        uVar8 = (unsigned short)(uVar8 + (unsigned char)(&DAT_006d4aa9)[iVar2]);
        uVar6 = (unsigned short)(unsigned char)(&DAT_006d4aa8)[iVar2];
        break;
      case 0x1d:
        if (param_6 < 10) {
          iVar1 = (int)((unsigned int)param_6 + iVar2);
          uVar10 = (unsigned short)(uVar10 + (unsigned char)(&DAT_006d4ad2)[iVar2 + (unsigned int)param_6]);
          uVar8 = (unsigned short)(uVar8 + (unsigned char)(&DAT_006d4adc)[iVar1]);
          uVar9 = (unsigned short)(uVar9 + (unsigned char)(&DAT_006d4abe)[iVar1]);
          uVar6 = (unsigned short)(unsigned char)(&DAT_006d4ac8)[iVar1];
        } else {
          uVar10 = (unsigned short)(uVar10 + *(short *)(&DAT_006d4a7c + iVar2));
          uVar9 = (unsigned short)(uVar9 + *(short *)(&DAT_006d4a7a + iVar2));
          uVar8 = (unsigned short)(uVar8 + (unsigned char)(&DAT_006d4aab)[iVar2]);
          uVar6 = (unsigned short)(unsigned char)(&DAT_006d4aaa)[iVar2];
        }
        break;
      case 0x1e:
        uVar10 = (unsigned short)(uVar10 + *(short *)(&DAT_006d4a9c + iVar2));
        uVar8 = (unsigned short)(uVar8 + (unsigned char)(&DAT_006d4abb)[iVar2]);
        goto switchD_0049d9e5_caseD_a;
      case 0x1f:
        uVar10 = (unsigned short)(uVar10 + *(short *)(&DAT_006d4a80 + iVar2));
        uVar9 = (unsigned short)(uVar9 + *(short *)(&DAT_006d4a7e + iVar2));
        uVar8 = (unsigned short)(uVar8 + (unsigned char)(&DAT_006d4aad)[iVar2]);
        uVar6 = (unsigned short)(unsigned char)(&DAT_006d4aac)[iVar2];
        break;
      case 0x20:
        uVar10 = (unsigned short)(uVar10 + *(short *)(&DAT_006d4a9e + iVar2));
        uVar8 = (unsigned short)(uVar8 + (unsigned char)(&DAT_006d4abc)[iVar2]);
        goto switchD_0049d9e5_caseD_a;
      case 0x21:
        uVar10 = (unsigned short)(uVar10 + *(short *)(&DAT_006d4a84 + iVar2));
        uVar9 = (unsigned short)(uVar9 + *(short *)(&DAT_006d4a82 + iVar2));
        uVar8 = (unsigned short)(uVar8 + (unsigned char)(&DAT_006d4aaf)[iVar2]);
        uVar6 = (unsigned short)(unsigned char)(&DAT_006d4aae)[iVar2];
        break;
      case 0x2b:
        uVar10 = (unsigned short)(uVar10 + (&DAT_006d4a8c)[uVar12 * 0x93] + (&DAT_006d4a8e)[uVar12 * 0x93] +
                 (&DAT_006d4a90)[uVar12 * 0x93] + (&DAT_006d4a92)[uVar12 * 0x93] +
                 *(short *)(&DAT_006d4a6e + iVar2) + *(short *)(&DAT_006d4a70 + iVar2));
        uVar8 = (unsigned short)(uVar8 + (unsigned char)(&DAT_006d4ab3)[iVar2] + (unsigned short)(unsigned char)(&DAT_006d4ab4)[iVar2] +
                (unsigned short)(unsigned char)(&DAT_006d4ab5)[iVar2] + (unsigned short)(unsigned char)(&DAT_006d4ab6)[iVar2] +
                (unsigned short)(unsigned char)(&DAT_006d4aa4)[iVar2] + (unsigned short)(unsigned char)(&DAT_006d4aa5)[iVar2]);
        goto switchD_0049d9e5_caseD_a;
      case 0x2c:
        if (param_6 < 10) {
          iVar1 = (int)((unsigned int)param_6 + iVar2);
          uVar10 = (unsigned short)(uVar10 + (unsigned char)(&DAT_006d4ae6)[iVar2 + (unsigned int)param_6]);
          uVar8 = (unsigned short)(uVar8 + (unsigned char)(&DAT_006d4af0)[iVar1]);
          uVar9 = (unsigned short)(uVar9 + (unsigned char)(&DAT_006d4afa)[iVar1]);
          uVar6 = (unsigned short)(unsigned char)(&DAT_006d4b04)[iVar1];
          break;
        }
        uVar12 = 0;
        do {
          uVar10 = (unsigned short)(uVar10 + (unsigned char)(&DAT_006d4ae6)[iVar2 + uVar12]);
          uVar8 = (unsigned short)(uVar8 + (unsigned char)(&DAT_006d4af0)[iVar2 + uVar12]);
          if (*(char *)((int)&DAT_00709f30 + uVar12) != 0) {
            uVar9 = (unsigned short)(uVar9 + (unsigned char)(&DAT_006d4afa)[iVar2 + uVar12]);
            local_50 = ((unsigned int)(unsigned short)((unsigned short)(local_50 >> 16) + (unsigned char)(&DAT_006d4b04)[iVar2 + uVar12]) << 16) |
                       (unsigned short)local_50;
          }
          uVar6 = (unsigned short)((short)uVar12 + 1);
          uVar12 = (unsigned int)uVar6;
        } while (uVar6 < 10);
        goto switchD_0049d9e5_caseD_a;
      case 0x2d:
        uVar10 = (unsigned short)(uVar10 + *(short *)(&DAT_006d4a96 + iVar2) + *(short *)(&DAT_006d4a94 + iVar2) +
                 *(short *)(&DAT_006d4a6e + iVar2) + *(short *)(&DAT_006d4a70 + iVar2));
        uVar8 = (unsigned short)(uVar8 + (unsigned char)(&DAT_006d4ab8)[iVar2] + (unsigned short)(unsigned char)(&DAT_006d4ab7)[iVar2] +
                (unsigned short)(unsigned char)(&DAT_006d4aa4)[iVar2] + (unsigned short)(unsigned char)(&DAT_006d4aa5)[iVar2]);
        uVar9 = (unsigned short)(uVar9 + *(short *)(&DAT_006d4a8a + iVar2) + *(short *)(&DAT_006d4aa0 + iVar2));
        local_50 = ((unsigned int)(unsigned short)((unsigned short)(local_50 >> 16) + (unsigned char)(&DAT_006d4ab2)[iVar2]) << 16) | (unsigned short)sVar5;
        uVar6 = (unsigned short)(unsigned char)(&DAT_006d4abd)[iVar2];
        break;
      case 0x2e:
        uVar10 = (unsigned short)(uVar10 + *(short *)(&DAT_006d4a98 + iVar2));
        uVar9 = (unsigned short)(uVar9 + *(short *)(&DAT_006d4a9a + iVar2));
        uVar8 = (unsigned short)(uVar8 + (unsigned char)(&DAT_006d4ab9)[iVar2]);
        uVar6 = (unsigned short)(unsigned char)(&DAT_006d4aba)[iVar2];
      }
      local_50 = ((unsigned int)(unsigned short)((unsigned short)(local_50 >> 16) + uVar6) << 16) | (unsigned short)local_50;
switchD_0049d9e5_caseD_a:
      uStack_58 = (unsigned short)(uStack_58 + 1);
    } while ((int)(unsigned int)uStack_58 < (int)DAT_006d49a0);
  }
  cVar13 = 4;
  if ((unsigned short)local_50 == 0) {
    goto switchD_0049e285_caseD_8;
  }
  switch (param_4) {
  case 0:
    if ((unsigned short)local_50 <= uVar10) {
      cVar13 = 1;
      goto switchD_0049e285_caseD_8;
    }
    goto joined_r0x0049e415;
  case 1:
    uVar7 = FUN_004783e0(uVar10, (unsigned short)local_50);
    if (0xbfff < (unsigned short)uVar7) {
      cVar13 = 1;
      goto switchD_0049e285_caseD_8;
    }
    uVar7 = FUN_004783e0(uVar9, (unsigned short)local_50);
    if ((unsigned short)uVar7 < 0x4001) {
      cVar13 = 4;
      goto switchD_0049e285_caseD_8;
    }
    break;
  case 2:
    uVar7 = FUN_004783e0(uVar10, (unsigned short)local_50);
    if (0x7fff < (unsigned short)uVar7) {
      cVar13 = 1;
      goto switchD_0049e285_caseD_8;
    }
    uVar7 = FUN_004783e0(uVar9, (unsigned short)local_50);
    if ((unsigned short)uVar7 < 0x8001) {
      cVar13 = 4;
      goto switchD_0049e285_caseD_8;
    }
    break;
  case 3:
    uVar7 = FUN_004783e0(uVar10, (unsigned short)local_50);
    if (0x3fff < (unsigned short)uVar7) {
      cVar13 = 1;
      goto switchD_0049e285_caseD_8;
    }
    uVar7 = FUN_004783e0(uVar9, (unsigned short)local_50);
    if ((unsigned short)uVar7 < 0xc001) {
      cVar13 = 4;
      goto switchD_0049e285_caseD_8;
    }
    break;
  case 4:
  case 0xe:
    if (uVar10 != 0) {
      cVar13 = 1;
      goto switchD_0049e285_caseD_8;
    }
    if (uVar9 != (unsigned short)local_50) {
      cVar13 = 4;
      goto switchD_0049e285_caseD_8;
    }
    break;
  case 5:
    if ((int)((local_50 & 0xffff) - 1) <= (int)(unsigned int)uVar10) {
      cVar13 = 1;
      goto switchD_0049e285_caseD_8;
    }
    if (uVar9 < 2) {
      cVar13 = 4;
      goto switchD_0049e285_caseD_8;
    }
    break;
  case 6:
    if (uStack_56 == 0) {
      cVar13 = 4;
      goto switchD_0049e285_caseD_8;
    }
    if (uVar8 == uStack_56) {
      cVar13 = 1;
      goto switchD_0049e285_caseD_8;
    }
    uVar9 = (unsigned short)(local_50 >> 16);
joined_r0x0049e415:
    if (uVar9 == 0) {
      cVar13 = 4;
      goto switchD_0049e285_caseD_8;
    }
    break;
  case 7:
    if ((local_50 & 0xffff) - (unsigned int)uStack_56 == (unsigned int)uVar10) {
      cVar13 = 1;
      goto switchD_0049e285_caseD_8;
    }
    if ((uVar9 == 0) || (uVar8 != 0)) {
      bVar4 = uVar9 < 2;
      uVar9 = uVar8;
      if (bVar4) {
        cVar13 = 4;
        goto switchD_0049e285_caseD_8;
      }
      goto joined_r0x0049e415;
    }
    break;
  default:
    goto switchD_0049e285_caseD_8;
  case 10:
    if (uVar10 < local_54) {
      cVar13 = 4;
      goto switchD_0049e285_caseD_8;
    }
    cVar13 = 1;
    goto switchD_0049e285_caseD_8;
  case 0xb:
    uVar7 = FUN_004783e0(uVar10, local_54);
    if ((unsigned short)uVar7 < 0xc000) {
      cVar13 = 4;
      goto switchD_0049e285_caseD_8;
    }
    cVar13 = 1;
    goto switchD_0049e285_caseD_8;
  case 0xc:
    uVar7 = FUN_004783e0(uVar10, local_54);
    if ((unsigned short)uVar7 < 0x8000) {
      cVar13 = 4;
      goto switchD_0049e285_caseD_8;
    }
    cVar13 = 1;
    goto switchD_0049e285_caseD_8;
  case 0xd:
    uVar7 = FUN_004783e0(uVar10, local_54);
    if ((unsigned short)uVar7 < 0x4000) {
      cVar13 = 4;
      goto switchD_0049e285_caseD_8;
    }
    cVar13 = 1;
    goto switchD_0049e285_caseD_8;
  case 0xf:
    if ((int)(unsigned int)uVar10 < (int)(local_54 - 1)) {
      cVar13 = 4;
      goto switchD_0049e285_caseD_8;
    }
    cVar13 = 1;
    goto switchD_0049e285_caseD_8;
  case 0x10:
    uVar7 = FUN_004783e0(uVar10, (unsigned short)local_50);
    if (0xaaa9 < (unsigned short)uVar7) {
      cVar13 = 1;
      goto switchD_0049e285_caseD_8;
    }
    uVar7 = FUN_004783e0(uVar9, (unsigned short)local_50);
    if ((unsigned short)uVar7 < 0x5556) {
      cVar13 = 4;
      goto switchD_0049e285_caseD_8;
    }
    break;
  case 0x11:
    uVar7 = FUN_004783e0(uVar10, (unsigned short)local_50);
    if (0x5554 < (unsigned short)uVar7) {
      cVar13 = 1;
      goto switchD_0049e285_caseD_8;
    }
    uVar7 = FUN_004783e0(uVar9, (unsigned short)local_50);
    if ((unsigned short)uVar7 < 0xaaab) {
      cVar13 = 4;
      goto switchD_0049e285_caseD_8;
    }
  }
  cVar13 = 2;
switchD_0049e285_caseD_8:
  DAT_00645adc = uVar10;
  DAT_00645ad8 = (unsigned short)local_50;
  return cVar13;
}
