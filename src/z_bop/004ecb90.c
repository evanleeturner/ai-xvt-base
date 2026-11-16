/* SPDX-License-Identifier: MIT */
/* z_bop 004ecb90 */
extern int FUN_004d1400(void);
extern char *FUN_004ddc10(int param_1);
extern void FUN_004db4e0(int param_1, char *param_2, int param_3, int param_4, unsigned int param_5);
extern int FUN_004d11e0(void);
extern void FUN_004d6460(void *param_1, int param_2, int param_3, int param_4, int param_5);
extern void FUN_004d4cf0(void *param_1);
extern void FUN_004d4d20(void *param_1);
extern int _sprintf(char *buf, const char *fmt, ...);
extern unsigned int FUN_004ee8e0(int param_1);
extern int FUN_004ee510(int param_1);
extern char FUN_004ee650(int param_1);
extern char FUN_004ee7c0(int param_1);
extern void FUN_004db7d0(int param_1, char *param_2, int *param_3, int param_4, int param_5, unsigned int param_6);

extern unsigned char DAT_00aa615e[];   /* packed, 0x2a stride records */
extern int DAT_00a91ca0[8];
extern unsigned char DAT_00b6a5aa[];   /* packed int */
extern unsigned char DAT_00b6a5a6[];   /* packed int */
extern unsigned int DAT_00b6a2c0;
extern unsigned short DAT_00b69ccc;
extern char DAT_00b69d20[];
extern int DAT_00b69e30;
extern unsigned char DAT_00b6a270[];
extern int DAT_00aa5e10[];
extern int DAT_00aa5cb0[];
extern int DAT_00aa5e68;
extern int DAT_00aa5ac0[];
extern char s__c_s__c_s_0052a6b0[];
extern char s__s__s_0052c780[];
extern char DAT_0052b46c[];

void __cdecl FUN_004ecb90(int param_1)
{
  char cVar1;
  int iVar2;
  char *pcVar3;
  unsigned int uVar4;
  unsigned int uVar5;
  unsigned char *pbVar6;
  unsigned char *pbVar11;
  int *piVar7;
  int iVar8;
  unsigned short uVar9;
  unsigned int uVar10;
  int bVar12;
  int iVar13;
  int iVar14;
  int iVar15;
  unsigned int uVar16;
  int *local_44;
  int local_40;
  int *local_3c;
  int local_38;
  unsigned int local_30[4];
  unsigned int local_20[4];
  int local_10[4];
  int rec0;
  int rec1;
  int sel;
  int mode;
  int *piVar6;
  int *piVar11;

  iVar2 = FUN_004d1400();
  iVar2 = (int)(0u - (unsigned int)((int)((unsigned int)iVar2 * 0xfu) >> 1));
  iVar8 = (int)((unsigned int)iVar2 + 0x155u);
  iVar13 = 0x58;
  iVar14 = iVar8;
  uVar5 = DAT_00b6a2c0;
  pcVar3 = FUN_004ddc10(0xba);
  FUN_004db4e0(0xc, pcVar3, iVar13, iVar14, uVar5);
  iVar13 = 0xe4;
  iVar14 = iVar8;
  uVar5 = DAT_00b6a2c0;
  pcVar3 = FUN_004ddc10(0x240);
  FUN_004db4e0(0xc, pcVar3, iVar13, iVar14, uVar5);
  iVar13 = 0x152;
  iVar14 = iVar8;
  uVar5 = DAT_00b6a2c0;
  pcVar3 = FUN_004ddc10(0x241);
  FUN_004db4e0(0xc, pcVar3, iVar13, iVar14, uVar5);
  iVar13 = 0x17a;
  iVar14 = iVar8;
  uVar5 = DAT_00b6a2c0;
  pcVar3 = FUN_004ddc10(0x242);
  FUN_004db4e0(0xc, pcVar3, iVar13, iVar14, uVar5);
  iVar14 = 0x193;
  uVar5 = DAT_00b6a2c0;
  pcVar3 = FUN_004ddc10(0x243);
  iVar2 = (int)((unsigned int)iVar2 + 0x164u);
  FUN_004db4e0(0xc, pcVar3, iVar14, iVar8, uVar5);
  FUN_004d1400();
  __builtin_memcpy(&mode, DAT_00b6a5aa, 4);
  if ((mode == 0) || (mode == 3) || (mode == 4)) {
    iVar14 = 0;
    pbVar6 = DAT_00aa615e;
    piVar11 = DAT_00a91ca0;
    do {
      __builtin_memcpy(&rec0, pbVar6, 4);
      iVar8 = rec0;
      if ((iVar8 != 0) && (iVar13 = FUN_004d11e0(), iVar13 == iVar8)) {
        uVar9 = DAT_00b69ccc;
        if (*piVar11 == 0) {
          uVar9 = (unsigned short)DAT_00b6a2c0;
        }
        FUN_004d6460(local_30, 0x58, iVar2, 0xe2, (int)((unsigned int)iVar2 + 0xfu));
        FUN_004d4cf0(local_20);
        FUN_004d4d20(local_30);
        iVar8 = (int)(pbVar6 - 0xe);
        if (*piVar11 == 0) {
          uVar16 = 1;
          __builtin_memcpy(&rec1, pbVar6 + 4, 4);
          pcVar3 = FUN_004ddc10((int)((unsigned int)rec1 + 0x9au));
          _sprintf((char *)DAT_00b69d20, s__c_s__c_s_0052a6b0, 6, pcVar3, uVar16, iVar8);
          iVar8 = FUN_004d11e0();
          __builtin_memcpy(&rec0, pbVar6, 4);
          if ((iVar8 == rec0) || (DAT_00b69e30 == 2)) {
            uVar5 = *(unsigned int *)(DAT_00b6a270 + ((unsigned int)(param_1 % 0x18) & 0xfffffffeU) * 2);
          }
          else {
            uVar5 = (unsigned int)uVar9;
          }
        }
        else {
          __builtin_memcpy(&rec1, pbVar6 + 4, 4);
          pcVar3 = FUN_004ddc10((int)((unsigned int)rec1 + 0x9au));
          _sprintf((char *)DAT_00b69d20, s__s__s_0052c780, pcVar3);
          uVar5 = (unsigned int)uVar9;
        }
        FUN_004db4e0(0xc, (char *)DAT_00b69d20, 0x58, iVar2, uVar5);
        uVar10 = (unsigned int)uVar9;
        FUN_004d4d20(local_20);
        iVar13 = 0xe4;
        iVar8 = iVar2;
        uVar5 = uVar10;
        uVar4 = FUN_004ee8e0(iVar14);
        pcVar3 = FUN_004ddc10((int)(uVar4 + 0x15u));
        FUN_004db4e0(0xc, pcVar3, iVar13, iVar8, uVar5);
        iVar15 = 0x152;
        iVar8 = iVar2;
        uVar5 = uVar10;
        iVar13 = FUN_004ee510(iVar14);
        pcVar3 = FUN_004ddc10((int)((unsigned int)iVar13 + 0x244u));
        FUN_004db4e0(0xc, pcVar3, iVar15, iVar8, uVar5);
        iVar13 = 0x17a;
        iVar8 = iVar2;
        uVar5 = uVar10;
        cVar1 = FUN_004ee650(iVar14);
        pcVar3 = FUN_004ddc10((int)((unsigned int)(unsigned char)cVar1 + 0x24eu));
        FUN_004db4e0(0xc, pcVar3, iVar13, iVar8, uVar5);
        iVar13 = 0x193;
        iVar8 = iVar2;
        cVar1 = FUN_004ee7c0(iVar14);
        iVar2 = (int)((unsigned int)iVar2 + 0xfu);
        pcVar3 = FUN_004ddc10((int)((unsigned int)(unsigned char)cVar1 + 0x253u));
        FUN_004db4e0(0xc, pcVar3, iVar13, iVar8, uVar10);
      }
      pbVar6 = pbVar6 + 0x2a;
      iVar14 = iVar14 + 1;
      piVar11 = piVar11 + 1;
    } while (piVar11 < DAT_00a91ca0 + 8);
    iVar14 = 0;
    pbVar11 = DAT_00aa615e;
    local_44 = DAT_00a91ca0;
    do {
      __builtin_memcpy(&rec0, pbVar11, 4);
      iVar8 = rec0;
      if ((iVar8 != 0) && (iVar13 = FUN_004d11e0(), iVar13 != iVar8)) {
        iVar13 = 0;
        __builtin_memcpy(&sel, DAT_00b6a5a6, 4);
        iVar8 = DAT_00aa5e10[sel];
        if (0 < iVar8) {
          do {
            __builtin_memcpy(&sel, DAT_00b6a5a6, 4);
            __builtin_memcpy(&rec0, pbVar11, 4);
            if (DAT_00aa5cb0[sel * 8 + iVar13] == rec0) break;
            iVar13 = iVar13 + 1;
          } while (iVar13 < iVar8);
        }
        if (iVar13 != iVar8) {
          uVar9 = DAT_00b69ccc;
          if (*local_44 == 0) {
            uVar9 = (unsigned short)DAT_00b6a2c0;
          }
          FUN_004d6460(local_30, 0x58, iVar2, 0xe2, (int)((unsigned int)iVar2 + 0xfu));
          FUN_004d4cf0(local_20);
          FUN_004d4d20(local_30);
          iVar8 = (int)(pbVar11 - 0xe);
          if (*local_44 == 0) {
            uVar16 = 1;
            __builtin_memcpy(&rec1, pbVar11 + 4, 4);
            pcVar3 = FUN_004ddc10((int)((unsigned int)rec1 + 0x9au));
            _sprintf((char *)DAT_00b69d20, s__c_s__c_s_0052a6b0, 6, pcVar3, uVar16, iVar8);
            iVar8 = FUN_004d11e0();
            __builtin_memcpy(&rec0, pbVar11, 4);
            if ((iVar8 == rec0) || (DAT_00b69e30 == 2)) {
              uVar5 = *(unsigned int *)(DAT_00b6a270 + ((unsigned int)(param_1 % 0x18) & 0xfffffffeU) * 2);
            }
            else {
              uVar5 = (unsigned int)uVar9;
            }
          }
          else {
            __builtin_memcpy(&rec1, pbVar11 + 4, 4);
            pcVar3 = FUN_004ddc10((int)((unsigned int)rec1 + 0x9au));
            _sprintf((char *)DAT_00b69d20, s__s__s_0052c780, pcVar3);
            uVar5 = (unsigned int)uVar9;
          }
          FUN_004db4e0(0xc, (char *)DAT_00b69d20, 0x58, iVar2, uVar5);
          uVar10 = (unsigned int)uVar9;
          FUN_004d4d20(local_20);
          iVar13 = 0xe4;
          iVar8 = iVar2;
          uVar5 = uVar10;
          uVar4 = FUN_004ee8e0(iVar14);
          pcVar3 = FUN_004ddc10((int)(uVar4 + 0x15u));
          FUN_004db4e0(0xc, pcVar3, iVar13, iVar8, uVar5);
          iVar15 = 0x152;
          iVar8 = iVar2;
          uVar5 = uVar10;
          iVar13 = FUN_004ee510(iVar14);
          pcVar3 = FUN_004ddc10((int)((unsigned int)iVar13 + 0x244u));
          FUN_004db4e0(0xc, pcVar3, iVar15, iVar8, uVar5);
          iVar13 = 0x17a;
          iVar8 = iVar2;
          uVar5 = uVar10;
          cVar1 = FUN_004ee650(iVar14);
          pcVar3 = FUN_004ddc10((int)((unsigned int)(unsigned char)cVar1 + 0x24eu));
          FUN_004db4e0(0xc, pcVar3, iVar13, iVar8, uVar5);
          iVar13 = 0x193;
          iVar8 = iVar2;
          cVar1 = FUN_004ee7c0(iVar14);
          iVar2 = (int)((unsigned int)iVar2 + 0xfu);
          pcVar3 = FUN_004ddc10((int)((unsigned int)(unsigned char)cVar1 + 0x253u));
          FUN_004db4e0(0xc, pcVar3, iVar13, iVar8, uVar10);
        }
      }
      pbVar11 = pbVar11 + 0x2a;
      iVar14 = iVar14 + 1;
      local_44 = local_44 + 1;
    } while (local_44 < DAT_00a91ca0 + 8);
    piVar11 = DAT_00a91ca0;
    pbVar6 = DAT_00aa615e;
    do {
      __builtin_memcpy(&rec0, pbVar6, 4);
      iVar14 = rec0;
      if ((iVar14 != 0) && (iVar8 = FUN_004d11e0(), iVar8 != iVar14)) {
        iVar8 = 0;
        __builtin_memcpy(&sel, DAT_00b6a5a6, 4);
        iVar14 = DAT_00aa5e10[sel];
        bVar12 = iVar14 == 0;
        if (0 < iVar14) {
          do {
            __builtin_memcpy(&sel, DAT_00b6a5a6, 4);
            __builtin_memcpy(&rec0, pbVar6, 4);
            if (DAT_00aa5cb0[sel * 8 + iVar8] == rec0) break;
            iVar8 = iVar8 + 1;
          } while (iVar8 < iVar14);
          bVar12 = iVar8 == iVar14;
        }
        if (bVar12) {
          uVar9 = DAT_00b69ccc;
          if (*piVar11 == 0) {
            uVar9 = (unsigned short)DAT_00b6a2c0;
          }
          FUN_004d6460(local_30, 0x58, iVar2, 0xe2, (int)((unsigned int)iVar2 + 0xfu));
          FUN_004d4cf0(local_20);
          FUN_004d4d20(local_30);
          iVar14 = (int)(pbVar6 - 0xe);
          if (*piVar11 == 0) {
            uVar16 = 1;
            __builtin_memcpy(&rec1, pbVar6 + 4, 4);
            pcVar3 = FUN_004ddc10((int)((unsigned int)rec1 + 0x9au));
            _sprintf((char *)DAT_00b69d20, s__c_s__c_s_0052a6b0, 6, pcVar3, uVar16, iVar14);
            iVar14 = FUN_004d11e0();
            __builtin_memcpy(&rec0, pbVar6, 4);
            if ((iVar14 == rec0) || (DAT_00b69e30 == 2)) {
              uVar5 = *(unsigned int *)(DAT_00b6a270 + ((unsigned int)(param_1 % 0x18) & 0xfffffffeU) * 2);
            }
            else {
              uVar5 = (unsigned int)uVar9;
            }
          }
          else {
            __builtin_memcpy(&rec1, pbVar6 + 4, 4);
            pcVar3 = FUN_004ddc10((int)((unsigned int)rec1 + 0x9au));
            _sprintf((char *)DAT_00b69d20, s__s__s_0052c780, pcVar3);
            uVar5 = (unsigned int)uVar9;
          }
          FUN_004db4e0(0xc, (char *)DAT_00b69d20, 0x58, iVar2, uVar5);
          FUN_004d4d20(local_20);
          FUN_004d6460(local_10, 0xe4, iVar2, 0x193, (int)((unsigned int)iVar2 + 0xeu));
          piVar7 = local_10;
          uVar5 = (unsigned int)uVar9;
          iVar8 = 0;
          iVar14 = 1;
          pcVar3 = FUN_004ddc10(599);
          iVar2 = (int)((unsigned int)iVar2 + 0xfu);
          FUN_004db7d0(0xc, pcVar3, piVar7, iVar14, iVar8, uVar5);
        }
      }
      piVar11 = piVar11 + 1;
      pbVar6 = pbVar6 + 0x2a;
      if ((int *)0xa91cbf < piVar11) {
        return;
      }
    } while (1);
  }
  iVar14 = 0;
  pbVar6 = DAT_00aa615e;
  piVar11 = DAT_00a91ca0;
  do {
    __builtin_memcpy(&rec0, pbVar6, 4);
    iVar8 = rec0;
    if ((iVar8 != 0) && (iVar13 = FUN_004d11e0(), iVar13 == iVar8)) {
      uVar9 = DAT_00b69ccc;
      if (*piVar11 == 0) {
        uVar9 = (unsigned short)DAT_00b6a2c0;
      }
      FUN_004d6460(local_30, 0x58, iVar2, 0xe2, (int)((unsigned int)iVar2 + 0xfu));
      FUN_004d4cf0(local_20);
      FUN_004d4d20(local_30);
      iVar8 = (int)(pbVar6 - 0xe);
      if (*piVar11 == 0) {
        uVar16 = 1;
        __builtin_memcpy(&rec1, pbVar6 + 4, 4);
        pcVar3 = FUN_004ddc10((int)((unsigned int)rec1 + 0x9au));
        _sprintf((char *)DAT_00b69d20, s__c_s__c_s_0052a6b0, 6, pcVar3, uVar16, iVar8);
        iVar8 = FUN_004d11e0();
        __builtin_memcpy(&rec0, pbVar6, 4);
        if ((iVar8 == rec0) || (DAT_00b69e30 == 2)) {
          uVar5 = *(unsigned int *)(DAT_00b6a270 + ((unsigned int)(param_1 % 0x18) & 0xfffffffeU) * 2);
        }
        else {
          uVar5 = (unsigned int)uVar9;
        }
      }
      else {
        __builtin_memcpy(&rec1, pbVar6 + 4, 4);
        pcVar3 = FUN_004ddc10((int)((unsigned int)rec1 + 0x9au));
        _sprintf((char *)DAT_00b69d20, s__s__s_0052c780, pcVar3);
        uVar5 = (unsigned int)uVar9;
      }
      FUN_004db4e0(0xc, (char *)DAT_00b69d20, 0x58, iVar2, uVar5);
      uVar10 = (unsigned int)uVar9;
      FUN_004d4d20(local_20);
      iVar13 = 0xe4;
      iVar8 = iVar2;
      uVar5 = uVar10;
      uVar4 = FUN_004ee8e0(iVar14);
      pcVar3 = FUN_004ddc10((int)(uVar4 + 0x15u));
      FUN_004db4e0(0xc, pcVar3, iVar13, iVar8, uVar5);
      iVar15 = 0x152;
      iVar8 = iVar2;
      uVar5 = uVar10;
      iVar13 = FUN_004ee510(iVar14);
      pcVar3 = FUN_004ddc10((int)((unsigned int)iVar13 + 0x244u));
      FUN_004db4e0(0xc, pcVar3, iVar15, iVar8, uVar5);
      iVar13 = 0x17a;
      iVar8 = iVar2;
      uVar5 = uVar10;
      cVar1 = FUN_004ee650(iVar14);
      pcVar3 = FUN_004ddc10((int)((unsigned int)(unsigned char)cVar1 + 0x24eu));
      FUN_004db4e0(0xc, pcVar3, iVar13, iVar8, uVar5);
      iVar13 = 0x193;
      iVar8 = iVar2;
      cVar1 = FUN_004ee7c0(iVar14);
      iVar2 = (int)((unsigned int)iVar2 + 0xfu);
      pcVar3 = FUN_004ddc10((int)((unsigned int)(unsigned char)cVar1 + 0x253u));
      FUN_004db4e0(0xc, pcVar3, iVar13, iVar8, uVar10);
    }
    pbVar6 = pbVar6 + 0x2a;
    iVar14 = iVar14 + 1;
    piVar11 = piVar11 + 1;
  } while (piVar11 < DAT_00a91ca0 + 8);
  local_40 = 0;
  local_3c = DAT_00a91ca0;
  pbVar11 = DAT_00aa615e;
  do {
    __builtin_memcpy(&rec0, pbVar11, 4);
    iVar8 = rec0;
    iVar14 = iVar2;
    if ((iVar8 != 0) && (iVar13 = FUN_004d11e0(), iVar13 != iVar8)) {
      uVar9 = DAT_00b69ccc;
      if (*local_3c == 0) {
        uVar9 = (unsigned short)DAT_00b6a2c0;
      }
      iVar14 = 0;
      bVar12 = 0;
      local_38 = 0;
      if (0 < DAT_00aa5e68) {
        piVar6 = DAT_00aa5cb0;
        local_44 = DAT_00aa5e10;
        do {
          iVar13 = 0;
          iVar8 = *local_44;
          piVar7 = piVar6;
          if (0 < iVar8) {
            do {
              __builtin_memcpy(&rec0, pbVar11, 4);
              if (*piVar7 == rec0) {
                if (DAT_00aa5ac0[iVar14 + iVar13] == -1) {
                  bVar12 = 1;
                }
                break;
              }
              iVar13 = iVar13 + 1;
              piVar7 = piVar7 + 1;
            } while (iVar13 < iVar8);
            if (iVar13 < iVar8) break;
          }
          iVar14 = iVar14 + 8;
          local_38 = local_38 + 1;
          piVar6 = piVar6 + 8;
          local_44 = local_44 + 1;
        } while (local_38 < DAT_00aa5e68);
      }
      iVar14 = (int)((unsigned int)iVar2 + 0xfu);
      FUN_004d6460(local_30, 0x58, iVar2, 0xe2, iVar14);
      FUN_004d4cf0(local_20);
      FUN_004d4d20(local_30);
      iVar8 = (int)(pbVar11 - 0xe);
      if (*local_3c == 0) {
        uVar16 = 1;
        __builtin_memcpy(&rec1, pbVar11 + 4, 4);
        pcVar3 = FUN_004ddc10((int)((unsigned int)rec1 + 0x9au));
        _sprintf((char *)DAT_00b69d20, s__c_s__c_s_0052a6b0, 6, pcVar3, uVar16, iVar8);
        iVar8 = FUN_004d11e0();
        __builtin_memcpy(&rec0, pbVar11, 4);
        if ((iVar8 == rec0) || (DAT_00b69e30 == 2)) {
          uVar5 = *(unsigned int *)(DAT_00b6a270 + ((unsigned int)(param_1 % 0x18) & 0xfffffffeU) * 2);
        }
        else {
          uVar5 = (unsigned int)uVar9;
        }
      }
      else {
        __builtin_memcpy(&rec1, pbVar11 + 4, 4);
        pcVar3 = FUN_004ddc10((int)((unsigned int)rec1 + 0x9au));
        _sprintf((char *)DAT_00b69d20, s__s__s_0052c780, pcVar3);
        uVar5 = (unsigned int)uVar9;
      }
      FUN_004db4e0(0xc, (char *)DAT_00b69d20, 0x58, iVar2, uVar5);
      FUN_004d4d20(local_20);
      if (bVar12) {
        uVar5 = (unsigned int)uVar9;
        FUN_004db4e0(0xc, DAT_0052b46c, 0xe4, iVar2, uVar5);
        FUN_004db4e0(0xc, DAT_0052b46c, 0x152, iVar2, uVar5);
        FUN_004db4e0(0xc, DAT_0052b46c, 0x17a, iVar2, uVar5);
        iVar8 = 0x193;
        pcVar3 = DAT_0052b46c;
      }
      else {
        uVar5 = (unsigned int)uVar9;
        iVar13 = 0xe4;
        iVar8 = iVar2;
        uVar4 = uVar5;
        uVar10 = FUN_004ee8e0(local_40);
        pcVar3 = FUN_004ddc10((int)(uVar10 + 0x15u));
        FUN_004db4e0(0xc, pcVar3, iVar13, iVar8, uVar4);
        iVar15 = 0x152;
        iVar8 = iVar2;
        uVar4 = uVar5;
        iVar13 = FUN_004ee510(local_40);
        pcVar3 = FUN_004ddc10((int)((unsigned int)iVar13 + 0x244u));
        FUN_004db4e0(0xc, pcVar3, iVar15, iVar8, uVar4);
        iVar13 = 0x17a;
        iVar8 = iVar2;
        uVar4 = uVar5;
        cVar1 = FUN_004ee650(local_40);
        pcVar3 = FUN_004ddc10((int)((unsigned int)(unsigned char)cVar1 + 0x24eu));
        FUN_004db4e0(0xc, pcVar3, iVar13, iVar8, uVar4);
        iVar8 = 0x193;
        cVar1 = FUN_004ee7c0(local_40);
        pcVar3 = FUN_004ddc10((int)((unsigned int)(unsigned char)cVar1 + 0x253u));
      }
      FUN_004db4e0(0xc, pcVar3, iVar8, iVar2, uVar5);
    }
    pbVar11 = pbVar11 + 0x2a;
    local_40 = local_40 + 1;
    local_3c = local_3c + 1;
    iVar2 = iVar14;
    if ((int *)0xa91cbf < local_3c) {
      return;
    }
  } while (1);
}
