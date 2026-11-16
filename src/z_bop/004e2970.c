/* SPDX-License-Identifier: MIT */
/* z_bop 004e2970 */
extern unsigned char DAT_00b6a5a6[];
extern unsigned char DAT_00b6a5aa[];
extern unsigned char DAT_00b6a5ae[];
extern unsigned char DAT_00b6a606[];
extern unsigned char DAT_00b6a60a[];
extern unsigned char DAT_00b6d962[];
extern unsigned char DAT_00b6d96a[];
extern unsigned char DAT_00b6d96e[];
extern unsigned char DAT_00b6d972[];
extern unsigned char DAT_00b6d9aa[];
extern unsigned char DAT_00b6d9ae[];
extern unsigned char DAT_00b7e34a[];
extern unsigned char DAT_00b7e34e[];
extern unsigned char DAT_00b7e352[];

extern int DAT_00b69e30;
extern int DAT_00665d10;
extern int DAT_00aa6114;
extern int DAT_00aa6118;
extern unsigned int DAT_00aa6134;
extern unsigned int DAT_00aa6138;
extern short DAT_00aa5aac;
extern int DAT_0052c208;

extern char DAT_00bb7394;
extern char DAT_00bb7397;
extern char DAT_00bb7398;
extern char DAT_00bb739d;
extern char DAT_00bb739e;
extern char DAT_00bb73a6;
extern char DAT_00bb73a7;

extern char DAT_00bb0106[];
extern char DAT_00bb010a[];
extern char DAT_00bb010e[];
extern char DAT_00bb10a6[];
extern char DAT_00bb10aa[];
extern char DAT_00bb10ae[];
extern char DAT_00bb2046[];
extern char DAT_00bb204a[];
extern char DAT_00bb242e[];
extern char DAT_00bb2432[];
extern char DAT_00bb260e[];
extern char DAT_00bb2612[];
extern char DAT_00a91d24;
extern char DAT_00a91d17[];

extern char s_jewelsound_0052ad74[];
extern char s_reg6d_0052b590[];
extern char s_reg6u_0052b598[];
extern char s_reg7d_0052b5a0[];
extern char s_reg7u_0052b5a8[];
extern char s_background_0052bf74[];
extern char s_game1d_0052c250[];
extern char s_game1u_0052c258[];
extern char s_game2d_0052c260[];
extern char s_game2u_0052c268[];
extern char s_game3d_0052c270[];
extern char s_game3u_0052c278[];
extern char s_game4d_0052c280[];
extern char s_game4u_0052c288[];
extern char s_game5d_0052c290[];
extern char s_game5u_0052c298[];
extern char s_game8d_0052c2a0[];
extern char s_game8u_0052c2a8[];
extern char s_game6d_0052c2b0[];
extern char s_game6u_0052c2b8[];
extern char s_game7d_0052c2c0[];
extern char s_game7u_0052c2c8[];

void FUN_004de090(int *, int *);
void FUN_004daba0(int *);
int FUN_004cfee0(void);
void FUN_004d6460(int *, int, int, int, int);
char *FUN_004ddc10(int);
int FUN_004da780(int *, char *, char *, char *, int, int, int, char *);
void FUN_004daa20(int *, char *, char *, int);
void FUN_004d64a0(int *, int, int);
void FUN_004e5000(unsigned int);
void FUN_004f70f0(void);
void FUN_004f36d0(void);
void FUN_004e5590(int);
int FUN_004d6b90(int *, int, int);
unsigned char FUN_004dc240(void);
unsigned char FUN_004dc220(void);
/* Ghidra shows 4 params; stack_check on the original shows 24 bytes pushed
   and cleaned by the caller. Args 5/6 (0,0) are placeholders. */
int __cdecl FUN_004de5a0(char *, int, int, int, int, int);
void FUN_004e6310(int);
void FUN_004b4bf0(char *);
void FUN_004e9330(void);

int __stdcall FUN_004e2970(void)
{
  unsigned char uVar1;
  int iVar2;
  char *pcVar3;
  int *piVar4;
  unsigned int uVar5;
  int iVar6;
  unsigned char *puVar7;
  unsigned char *puVar8;
  int iVar9;
  int uVar10;
  int iVar11;
  char *pcVar12;
  int local_40[4];
  unsigned int local_30;
  int local_2c;
  int local_28;
  int local_24;
  int local_20[8];
  unsigned int tmpTab;
  unsigned int tmpItem;
  unsigned int tmpVal;
  unsigned int tmpSlot;

  iVar6 = 0;
  FUN_004de090(&local_28, &local_2c);
  local_20[0] = 1;
  local_20[1] = 1;
  local_20[2] = 1;
  local_20[3] = 1;
  local_20[4] = 1;
  if (DAT_00b69e30 == 0) {
    local_20[5] = 0;
    local_20[6] = 0;
  }
  else {
    local_20[5] = 1;
    local_20[6] = 1;
    __builtin_memcpy(&tmpSlot, DAT_00b6d962, 4);
    iVar2 = (int)tmpSlot;
    if (DAT_00b69e30 != 2) {
      iVar2 = DAT_00665d10;
    }
    local_20[iVar2 + 5] = 2;
  }
  local_20[7] = 1;
  __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
  if (tmpTab == 5) {
    local_20[7] = 2;
  }
  else {
    local_20[tmpTab] = 2;
  }
  FUN_004daba0(local_20);
  iVar2 = FUN_004cfee0();
  if (DAT_00b69e30 == 2) {
    iVar2 = 1;
  }
  FUN_004d6460(local_40, 0x16, 0x14e, 0x2a, 0x166);
  if (DAT_00b69e30 == 2) {
    __builtin_memcpy(&tmpSlot, DAT_00b6d962, 4);
    if (tmpSlot == 1) {
      iVar9 = 0xc;
      pcVar3 = FUN_004ddc10(0x283);
      FUN_004daa20(local_40, s_reg7d_0052b5a0, pcVar3, iVar9);
    }
    else {
      pcVar12 = s_jewelsound_0052ad74;
      iVar11 = 0x11;
      uVar10 = 0;
      iVar9 = 0xc;
      pcVar3 = FUN_004ddc10(0x283);
      iVar9 = FUN_004da780(local_40, s_reg7u_0052b5a8, s_reg7u_0052b5a8, pcVar3, iVar9, uVar10, iVar11, pcVar12);
      if (iVar9 != 0) {
        iVar6 = 1;
        __builtin_memcpy(&tmpSlot, DAT_00b6d962, 4);
        __builtin_memcpy(&tmpVal, DAT_00b6a5a6, 4);
        __builtin_memcpy(DAT_00b6d96a + tmpSlot * 0x109E0u, &tmpVal, 4);
        __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
        __builtin_memcpy(DAT_00b6d96e + tmpSlot * 0x109E0u, &tmpTab, 4);
        puVar7 = DAT_00b6a5ae;
        puVar8 = DAT_00b6d972 + tmpSlot * 0x109E0u;
        for (iVar9 = 6; iVar9 != 0; iVar9 = iVar9 + -1) {
          __builtin_memcpy(puVar8, puVar7, 4);
          puVar7 = puVar7 + 4;
          puVar8 = puVar8 + 4;
        }
        __builtin_memcpy(&tmpVal, DAT_00b6a606, 4);
        __builtin_memcpy(DAT_00b6d9aa + tmpSlot * 0x109E0u, &tmpVal, 4);
        __builtin_memcpy(&tmpVal, DAT_00b6a60a, 4);
        __builtin_memcpy(DAT_00b6d9ae + tmpSlot * 0x109E0u, &tmpVal, 4);
        __builtin_memcpy(&local_30, DAT_00b6a5aa, 4);
        __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
        __builtin_memcpy(&tmpItem, DAT_00b6a5ae + tmpTab * 4u, 4);
        local_24 = (int)tmpItem;
        tmpSlot = 1;
        __builtin_memcpy(DAT_00b6d962, &tmpSlot, 4);
        __builtin_memcpy(&tmpVal, DAT_00b7e34a, 4);
        __builtin_memcpy(DAT_00b6a5a6, &tmpVal, 4);
        __builtin_memcpy(&tmpVal, DAT_00b7e34e, 4);
        __builtin_memcpy(DAT_00b6a5aa, &tmpVal, 4);
        puVar7 = DAT_00b7e352;
        puVar8 = DAT_00b6a5ae;
        for (iVar9 = 6; iVar9 != 0; iVar9 = iVar9 + -1) {
          __builtin_memcpy(puVar8, puVar7, 4);
          puVar7 = puVar7 + 4;
          puVar8 = puVar8 + 4;
        }
        __builtin_memcpy(&tmpVal, DAT_00b6d9aa + tmpSlot * 0x109E0u, 4);
        __builtin_memcpy(DAT_00b6a606, &tmpVal, 4);
        __builtin_memcpy(&tmpVal, DAT_00b6d9ae + tmpSlot * 0x109E0u, 4);
        __builtin_memcpy(DAT_00b6a60a, &tmpVal, 4);
      }
    }
    FUN_004d64a0(local_40, 0, -0x1c);
    __builtin_memcpy(&tmpSlot, DAT_00b6d962, 4);
    if (tmpSlot == 0) {
      iVar9 = 0xc;
      pcVar3 = FUN_004ddc10(0x282);
      FUN_004daa20(local_40, s_reg6d_0052b590, pcVar3, iVar9);
    }
    else {
      pcVar12 = s_jewelsound_0052ad74;
      iVar11 = 0x10;
      uVar10 = 0;
      iVar9 = 0xc;
      pcVar3 = FUN_004ddc10(0x282);
      iVar9 = FUN_004da780(local_40, s_reg6u_0052b598, s_reg6u_0052b598, pcVar3, iVar9, uVar10, iVar11, pcVar12);
      if (iVar9 != 0) {
        iVar6 = 1;
        __builtin_memcpy(&tmpSlot, DAT_00b6d962, 4);
        __builtin_memcpy(&tmpVal, DAT_00b6a5a6, 4);
        __builtin_memcpy(DAT_00b6d96a + tmpSlot * 0x109E0u, &tmpVal, 4);
        __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
        __builtin_memcpy(DAT_00b6d96e + tmpSlot * 0x109E0u, &tmpTab, 4);
        puVar7 = DAT_00b6a5ae;
        puVar8 = DAT_00b6d972 + tmpSlot * 0x109E0u;
        for (iVar9 = 6; iVar9 != 0; iVar9 = iVar9 + -1) {
          __builtin_memcpy(puVar8, puVar7, 4);
          puVar7 = puVar7 + 4;
          puVar8 = puVar8 + 4;
        }
        __builtin_memcpy(&tmpVal, DAT_00b6a606, 4);
        __builtin_memcpy(DAT_00b6d9aa + tmpSlot * 0x109E0u, &tmpVal, 4);
        __builtin_memcpy(&tmpVal, DAT_00b6a60a, 4);
        __builtin_memcpy(DAT_00b6d9ae + tmpSlot * 0x109E0u, &tmpVal, 4);
        __builtin_memcpy(&local_30, DAT_00b6a5aa, 4);
        __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
        __builtin_memcpy(&tmpItem, DAT_00b6a5ae + tmpTab * 4u, 4);
        local_24 = (int)tmpItem;
        tmpSlot = 0;
        __builtin_memcpy(DAT_00b6d962, &tmpSlot, 4);
        __builtin_memcpy(&tmpVal, DAT_00b6d96a, 4);
        __builtin_memcpy(DAT_00b6a5a6, &tmpVal, 4);
        __builtin_memcpy(&tmpVal, DAT_00b6d96e, 4);
        __builtin_memcpy(DAT_00b6a5aa, &tmpVal, 4);
        puVar7 = DAT_00b6d972;
        puVar8 = DAT_00b6a5ae;
        for (iVar9 = 6; iVar9 != 0; iVar9 = iVar9 + -1) {
          __builtin_memcpy(puVar8, puVar7, 4);
          puVar7 = puVar7 + 4;
          puVar8 = puVar8 + 4;
        }
        __builtin_memcpy(&tmpVal, DAT_00b6d9aa + tmpSlot * 0x109E0u, 4);
        __builtin_memcpy(DAT_00b6a606, &tmpVal, 4);
        __builtin_memcpy(&tmpVal, DAT_00b6d9ae + tmpSlot * 0x109E0u, 4);
        __builtin_memcpy(DAT_00b6a60a, &tmpVal, 4);
      }
    }
    if (iVar6 != 0) {
      __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
      if ((tmpTab != 5) && (DAT_00bb7394 == '\x03')) {
        DAT_00bb7394 = '\0';
      }
      __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
      if ((tmpTab != 4) && (DAT_00bb7397 == '\x02')) {
        DAT_00bb7397 = '\0';
      }
      __builtin_memcpy(DAT_00b6a5aa, &local_30, 4);
      FUN_004e5000(local_30);
      if (DAT_00aa6114 != 0) {
        DAT_00aa6134 = 0;
        if (DAT_00aa6138 != 0) {
          piVar4 = (int *)(DAT_00aa6114 + 0x140);
          do {
            if (*piVar4 == local_24) {
              __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
              __builtin_memcpy(DAT_00b6a5ae + tmpTab * 4u, &local_24, 4);
              break;
            }
            piVar4 = piVar4 + 0x52;
            DAT_00aa6134 = DAT_00aa6134 + 1;
          } while (DAT_00aa6134 < DAT_00aa6138);
        }
        if (DAT_00aa6138 <= DAT_00aa6134) {
          DAT_00aa6134 = 0;
          if (DAT_00aa6138 != 0) {
            piVar4 = (int *)(DAT_00aa6114 + 0x140);
            do {
              __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
              __builtin_memcpy(&tmpItem, DAT_00b6a5ae + tmpTab * 4u, 4);
              if ((unsigned int)*piVar4 == tmpItem) break;
              piVar4 = piVar4 + 0x52;
              DAT_00aa6134 = DAT_00aa6134 + 1;
            } while (DAT_00aa6134 < DAT_00aa6138);
          }
          if (DAT_00aa6138 <= DAT_00aa6134) {
            tmpVal = *(unsigned int *)(DAT_00aa6114 + 0x140);
            __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
            __builtin_memcpy(DAT_00b6a5ae + tmpTab * 4u, &tmpVal, 4);
          }
        }
      }
      FUN_004f70f0();
      FUN_004f36d0();
      FUN_004e5590(DAT_00aa6118);
      DAT_0052c208 = 0;
      __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
      if (tmpTab == 4) {
        piVar4 = (int *)(DAT_00aa6114 + 0x140 + DAT_00aa6134 * 0x148);
        iVar9 = (int)((unsigned int)*piVar4 * 0xa0u);
        __builtin_memcpy(&tmpVal, DAT_00bb0106 + iVar9, 4);
        if (tmpVal != 0) {
          DAT_00bb7398 = DAT_00bb010a[iVar9];
          DAT_00bb7397 = DAT_00bb010e[(int)((unsigned int)*piVar4 * 0xa0u)];
        }
      }
      else if (tmpTab == 5) {
        iVar9 = (int)((unsigned int)*(int *)(DAT_00aa6114 + 0x140 + DAT_00aa6134 * 0x148) * 0x28u);
        __builtin_memcpy(&tmpVal, DAT_00bb2046 + iVar9, 4);
        if (tmpVal != 0) {
          DAT_00bb7397 = DAT_00bb204a[iVar9];
        }
      }
    }
  }
  else {
    if (local_20[6] != 0) {
      if (DAT_00665d10 == 1) {
        iVar9 = 0xc;
        pcVar3 = FUN_004ddc10(0xcb);
        FUN_004daa20(local_40, s_game7d_0052c2c0, pcVar3, iVar9);
      }
      else {
        pcVar12 = s_jewelsound_0052ad74;
        iVar11 = 0x11;
        uVar10 = 0;
        iVar9 = 0xc;
        pcVar3 = FUN_004ddc10(0xcb);
        iVar9 = FUN_004da780(local_40, s_game7u_0052c2c8, s_game7u_0052c2c8, pcVar3, iVar9, uVar10, iVar11, pcVar12);
        if (iVar9 != 0) {
          iVar6 = 1;
          DAT_00665d10 = 1;
        }
      }
    }
    FUN_004d64a0(local_40, 0, -0x1c);
    if (local_20[5] != 0) {
      if (DAT_00665d10 == 0) {
        iVar9 = 0xc;
        pcVar3 = FUN_004ddc10(0xca);
        FUN_004daa20(local_40, s_game6d_0052c2b0, pcVar3, iVar9);
      }
      else {
        pcVar12 = s_jewelsound_0052ad74;
        iVar11 = 0x10;
        uVar10 = 0;
        iVar9 = 0xc;
        pcVar3 = FUN_004ddc10(0xca);
        iVar9 = FUN_004da780(local_40, s_game6u_0052c2b8, s_game6u_0052c2b8, pcVar3, iVar9, uVar10, iVar11, pcVar12);
        if (iVar9 != 0) {
          DAT_00665d10 = 0;
          iVar6 = 1;
        }
      }
    }
  }
  FUN_004d6460(local_40, 0x16, 0xfe, 0x2a, 0x116);
  __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
  if (tmpTab == 5) {
    iVar9 = 0xc;
    pcVar3 = FUN_004ddc10(199);
    FUN_004daa20(local_40, s_game8d_0052c2a0, pcVar3, iVar9);
    if ((iVar2 != 0) && (iVar9 = FUN_004d6b90(local_40, local_28, local_2c), iVar9 != 0)) {
      uVar1 = FUN_004dc240();
      if (uVar1 == 0) {
        uVar1 = FUN_004dc220();
        if (uVar1 != 0) {
          if (DAT_00aa6114 != 0) {
            if (DAT_00bb73a7 != '\0') {
              FUN_004de5a0(s_jewelsound_0052ad74, 1, 0, 0xff, 0, 0);
            }
            do {
              if (DAT_00aa6138 - DAT_00aa6134 == 1) {
                DAT_00aa6134 = 0;
              }
              else {
                DAT_00aa6134 = DAT_00aa6134 + 1;
              }
            } while (*(int *)(DAT_00aa6114 + 0x144 + DAT_00aa6134 * 0x148) != 0);
            DAT_00bb73a6 = 1;
            tmpVal = *(unsigned int *)(DAT_00aa6114 + 0x140 + DAT_00aa6134 * 0x148);
            __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
            __builtin_memcpy(DAT_00b6a5ae + tmpTab * 4u, &tmpVal, 4);
            FUN_004f70f0();
            FUN_004f36d0();
            FUN_004e5590(DAT_00aa6118);
            DAT_0052c208 = 0;
            if ((DAT_00aa6114 != 0) && (DAT_00aa6134 = 0, DAT_00aa6138 != 0)) {
              piVar4 = (int *)(DAT_00aa6114 + 0x140);
              do {
                __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
                __builtin_memcpy(&tmpItem, DAT_00b6a5ae + tmpTab * 4u, 4);
                if ((unsigned int)*piVar4 == tmpItem) break;
                piVar4 = piVar4 + 0x52;
                DAT_00aa6134 = DAT_00aa6134 + 1;
              } while (DAT_00aa6134 < DAT_00aa6138);
            }
            if (DAT_00b69e30 == 2) {
              iVar9 = (int)((unsigned int)*(int *)(DAT_00aa6114 + 0x140 + DAT_00aa6134 * 0x148) * 0x28u);
              __builtin_memcpy(&tmpVal, DAT_00bb2046 + iVar9, 4);
              if (tmpVal != 0) {
                DAT_00bb7397 = DAT_00bb204a[iVar9];
              }
            }
            else {
              iVar11 = FUN_004cfee0();
              iVar9 = *(int *)(DAT_00aa6114 + 0x140 + DAT_00aa6134 * 0x148);
              if (iVar11 == 0) {
                iVar9 = (int)((unsigned int)iVar9 * 0x28u);
                __builtin_memcpy(&tmpVal, DAT_00bb260e + iVar9, 4);
                if (tmpVal != 0) {
                  DAT_00bb7397 = DAT_00bb2612[iVar9];
                }
              }
              else {
                iVar9 = (int)((unsigned int)iVar9 * 0x28u);
                __builtin_memcpy(&tmpVal, DAT_00bb242e + iVar9, 4);
                if (tmpVal != 0) {
                  DAT_00bb7397 = DAT_00bb2432[iVar9];
                }
              }
            }
          }
          if (DAT_00b69e30 != 2) {
            iVar9 = FUN_004cfee0();
            goto joined_r0x004e36cf;
          }
        }
      }
      else {
        if (DAT_00aa6114 != 0) {
          if (DAT_00bb73a7 != '\0') {
            FUN_004de5a0(s_jewelsound_0052ad74, 1, 0, 0xff, 0, 0);
          }
          do {
            if (DAT_00aa6134 == 0) {
              DAT_00aa6134 = DAT_00aa6138;
            }
            DAT_00aa6134 = DAT_00aa6134 - 1;
          } while (*(int *)(DAT_00aa6114 + 0x144 + DAT_00aa6134 * 0x148) != 0);
          DAT_00bb73a6 = 1;
          tmpVal = *(unsigned int *)(DAT_00aa6114 + 0x140 + DAT_00aa6134 * 0x148);
          __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
          __builtin_memcpy(DAT_00b6a5ae + tmpTab * 4u, &tmpVal, 4);
          FUN_004f70f0();
          FUN_004f36d0();
          FUN_004e5590(DAT_00aa6118);
          if (DAT_00aa6114 != 0) {
            DAT_0052c208 = 0;
            DAT_00aa6134 = 0;
            if (DAT_00aa6138 != 0) {
              piVar4 = (int *)(DAT_00aa6114 + 0x140);
              do {
                __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
                __builtin_memcpy(&tmpItem, DAT_00b6a5ae + tmpTab * 4u, 4);
                if ((unsigned int)*piVar4 == tmpItem) break;
                piVar4 = piVar4 + 0x52;
                DAT_00aa6134 = DAT_00aa6134 + 1;
              } while (DAT_00aa6134 < DAT_00aa6138);
            }
          }
          if (DAT_00b69e30 == 2) {
            iVar9 = (int)((unsigned int)*(int *)(DAT_00aa6114 + 0x140 + DAT_00aa6134 * 0x148) * 0x28u);
            __builtin_memcpy(&tmpVal, DAT_00bb2046 + iVar9, 4);
            if (tmpVal != 0) {
              DAT_00bb7397 = DAT_00bb204a[iVar9];
            }
          }
          else {
            iVar11 = FUN_004cfee0();
            iVar9 = *(int *)(DAT_00aa6114 + 0x140 + DAT_00aa6134 * 0x148);
            if (iVar11 == 0) {
              iVar9 = (int)((unsigned int)iVar9 * 0x28u);
              __builtin_memcpy(&tmpVal, DAT_00bb260e + iVar9, 4);
              if (tmpVal != 0) {
                DAT_00bb7397 = DAT_00bb2612[iVar9];
              }
            }
            else {
              iVar9 = (int)((unsigned int)iVar9 * 0x28u);
              __builtin_memcpy(&tmpVal, DAT_00bb242e + iVar9, 4);
              if (tmpVal != 0) {
                DAT_00bb7397 = DAT_00bb2432[iVar9];
              }
            }
          }
        }
        if (DAT_00b69e30 != 2) {
          iVar9 = FUN_004cfee0();
joined_r0x004e36cf:
          if (iVar9 != 0) {
            FUN_004e6310(0);
          }
        }
      }
    }
  }
  else if (iVar2 == 0) {
    iVar9 = 0xc;
    pcVar3 = FUN_004ddc10(199);
    FUN_004daa20(local_40, s_game8u_0052c2a8, pcVar3, iVar9);
  }
  else {
    pcVar12 = s_jewelsound_0052ad74;
    iVar11 = 0x12;
    uVar10 = 0;
    iVar9 = 0xc;
    pcVar3 = FUN_004ddc10(199);
    iVar9 = FUN_004da780(local_40, s_game8u_0052c2a8, s_game8u_0052c2a8, pcVar3, iVar9, uVar10, iVar11, pcVar12);
    if (iVar9 != 0) {
      __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
      if (((DAT_00b69e30 != 2) && (tmpTab != 0)) && (tmpTab != 5)) {
        DAT_00bb739e = (char)0xff;
      }
      tmpVal = 5;
      __builtin_memcpy(DAT_00b6a5aa, &tmpVal, 4);
      DAT_00bb7397 = '\0';
      iVar6 = 1;
      DAT_00bb73a6 = 1;
      FUN_004f70f0();
      FUN_004f36d0();
      FUN_004e5590(DAT_00aa6118);
      DAT_0052c208 = 0;
      if (DAT_00aa6114 != 0) {
        DAT_00aa6134 = 0;
        if (DAT_00aa6138 != 0) {
          piVar4 = (int *)(DAT_00aa6114 + 0x140);
          do {
            __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
            __builtin_memcpy(&tmpItem, DAT_00b6a5ae + tmpTab * 4u, 4);
            if ((unsigned int)*piVar4 == tmpItem) break;
            piVar4 = piVar4 + 0x52;
            DAT_00aa6134 = DAT_00aa6134 + 1;
          } while (DAT_00aa6134 < DAT_00aa6138);
        }
        if ((DAT_00aa6134 == DAT_00aa6138) ||
            (*(int *)(DAT_00aa6114 + 0x144 + DAT_00aa6134 * 0x148) != 0)) {
          uVar5 = 0;
          if (0 < (int)DAT_00aa6138) {
            piVar4 = (int *)(DAT_00aa6114 + 0x144);
            do {
              if (*piVar4 == 0) {
                DAT_00aa6134 = uVar5;
                tmpVal = *(unsigned int *)(DAT_00aa6114 + 0x140 + uVar5 * 0x148);
                __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
                __builtin_memcpy(DAT_00b6a5ae + tmpTab * 4u, &tmpVal, 4);
                break;
              }
              piVar4 = piVar4 + 0x52;
              uVar5 = uVar5 + 1;
            } while ((int)uVar5 < (int)DAT_00aa6138);
          }
          FUN_004f70f0();
          FUN_004f36d0();
          FUN_004e5590(DAT_00aa6118);
        }
      }
      DAT_0052c208 = 0;
      __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
      if ((tmpTab != 4) && (DAT_00bb7397 == '\x02')) {
        DAT_00bb7397 = '\0';
      }
      if (DAT_00b69e30 == 2) {
        iVar9 = (int)((unsigned int)*(int *)(DAT_00aa6114 + 0x140 + DAT_00aa6134 * 0x148) * 0x28u);
        __builtin_memcpy(&tmpVal, DAT_00bb2046 + iVar9, 4);
        if (tmpVal != 0) {
          DAT_00bb7397 = DAT_00bb204a[iVar9];
        }
      }
      else {
        iVar11 = FUN_004cfee0();
        iVar9 = *(int *)(DAT_00aa6114 + 0x140 + DAT_00aa6134 * 0x148);
        if (iVar11 == 0) {
          iVar9 = (int)((unsigned int)iVar9 * 0x28u);
          __builtin_memcpy(&tmpVal, DAT_00bb260e + iVar9, 4);
          if (tmpVal != 0) {
            DAT_00bb7397 = DAT_00bb2612[iVar9];
          }
        }
        else {
          iVar9 = (int)((unsigned int)iVar9 * 0x28u);
          __builtin_memcpy(&tmpVal, DAT_00bb242e + iVar9, 4);
          if (tmpVal != 0) {
            DAT_00bb7397 = DAT_00bb2432[iVar9];
          }
        }
      }
    }
  }
  FUN_004d6460(local_40, 0x16, 0xe2, 0x2a, 0xfa);
  __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
  if (tmpTab == 4) {
    iVar9 = 0xc;
    pcVar3 = FUN_004ddc10(0xc6);
    FUN_004daa20(local_40, s_game5d_0052c290, pcVar3, iVar9);
    if ((iVar2 != 0) && (iVar9 = FUN_004d6b90(local_40, local_28, local_2c), iVar9 != 0)) {
      uVar1 = FUN_004dc240();
      if (uVar1 == 0) {
        uVar1 = FUN_004dc220();
        if (uVar1 != 0) {
          if (DAT_00aa6114 != 0) {
            if (DAT_00bb73a7 != '\0') {
              FUN_004de5a0(s_jewelsound_0052ad74, 1, 0, 0xff, 0, 0);
            }
            do {
              if (DAT_00aa6138 - DAT_00aa6134 == 1) {
                DAT_00aa6134 = 0;
              }
              else {
                DAT_00aa6134 = DAT_00aa6134 + 1;
              }
            } while (*(int *)(DAT_00aa6114 + 0x144 + DAT_00aa6134 * 0x148) != 0);
            DAT_00bb73a6 = 1;
            tmpVal = *(unsigned int *)(DAT_00aa6114 + 0x140 + DAT_00aa6134 * 0x148);
            __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
            __builtin_memcpy(DAT_00b6a5ae + tmpTab * 4u, &tmpVal, 4);
            FUN_004f70f0();
            FUN_004f36d0();
            FUN_004e5590(DAT_00aa6118);
            DAT_0052c208 = 0;
            if ((DAT_00aa6114 != 0) && (DAT_00aa6134 = 0, DAT_00aa6138 != 0)) {
              piVar4 = (int *)(DAT_00aa6114 + 0x140);
              do {
                __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
                __builtin_memcpy(&tmpItem, DAT_00b6a5ae + tmpTab * 4u, 4);
                if ((unsigned int)*piVar4 == tmpItem) break;
                piVar4 = piVar4 + 0x52;
                DAT_00aa6134 = DAT_00aa6134 + 1;
              } while (DAT_00aa6134 < DAT_00aa6138);
            }
            piVar4 = (int *)(DAT_00aa6114 + 0x140 + DAT_00aa6134 * 0x148);
            if (DAT_00b69e30 == 2) {
              iVar9 = (int)((unsigned int)*piVar4 * 0xa0u);
              __builtin_memcpy(&tmpVal, DAT_00bb0106 + iVar9, 4);
              if (tmpVal != 0) {
                DAT_00bb7398 = DAT_00bb010a[iVar9];
                DAT_00bb7397 = DAT_00bb010e[(int)((unsigned int)*piVar4 * 0xa0u)];
              }
            }
            else {
              iVar9 = (int)((unsigned int)*piVar4 * 0xa0u);
              __builtin_memcpy(&tmpVal, DAT_00bb10a6 + iVar9, 4);
              if (tmpVal != 0) {
                DAT_00bb7398 = DAT_00bb10aa[iVar9];
                DAT_00bb7397 = DAT_00bb10ae[(int)((unsigned int)*piVar4 * 0xa0u)];
              }
            }
          }
          if (DAT_00b69e30 != 2) {
            iVar9 = FUN_004cfee0();
            goto joined_r0x004e3ccb;
          }
        }
      }
      else {
        if (DAT_00aa6114 != 0) {
          if (DAT_00bb73a7 != '\0') {
            FUN_004de5a0(s_jewelsound_0052ad74, 1, 0, 0xff, 0, 0);
          }
          do {
            if (DAT_00aa6134 == 0) {
              DAT_00aa6134 = DAT_00aa6138;
            }
            DAT_00aa6134 = DAT_00aa6134 - 1;
          } while (*(int *)(DAT_00aa6114 + 0x144 + DAT_00aa6134 * 0x148) != 0);
          DAT_00bb73a6 = 1;
          tmpVal = *(unsigned int *)(DAT_00aa6114 + 0x140 + DAT_00aa6134 * 0x148);
          __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
          __builtin_memcpy(DAT_00b6a5ae + tmpTab * 4u, &tmpVal, 4);
          FUN_004f70f0();
          FUN_004f36d0();
          FUN_004e5590(DAT_00aa6118);
          if (DAT_00aa6114 != 0) {
            DAT_0052c208 = 0;
            DAT_00aa6134 = 0;
            if (DAT_00aa6138 != 0) {
              piVar4 = (int *)(DAT_00aa6114 + 0x140);
              do {
                __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
                __builtin_memcpy(&tmpItem, DAT_00b6a5ae + tmpTab * 4u, 4);
                if ((unsigned int)*piVar4 == tmpItem) break;
                piVar4 = piVar4 + 0x52;
                DAT_00aa6134 = DAT_00aa6134 + 1;
              } while (DAT_00aa6134 < DAT_00aa6138);
            }
          }
          piVar4 = (int *)(DAT_00aa6114 + 0x140 + DAT_00aa6134 * 0x148);
          if (DAT_00b69e30 == 2) {
            iVar9 = (int)((unsigned int)*piVar4 * 0xa0u);
            __builtin_memcpy(&tmpVal, DAT_00bb0106 + iVar9, 4);
            if (tmpVal != 0) {
              DAT_00bb7398 = DAT_00bb010a[iVar9];
              DAT_00bb7397 = DAT_00bb010e[(int)((unsigned int)*piVar4 * 0xa0u)];
            }
          }
          else {
            iVar9 = (int)((unsigned int)*piVar4 * 0xa0u);
            __builtin_memcpy(&tmpVal, DAT_00bb10a6 + iVar9, 4);
            if (tmpVal != 0) {
              DAT_00bb7398 = DAT_00bb10aa[iVar9];
              DAT_00bb7397 = DAT_00bb10ae[(int)((unsigned int)*piVar4 * 0xa0u)];
            }
          }
        }
        if (DAT_00b69e30 != 2) {
          iVar9 = FUN_004cfee0();
joined_r0x004e3ccb:
          if (iVar9 != 0) {
            FUN_004e6310(0);
          }
        }
      }
    }
  }
  else if (iVar2 == 0) {
    iVar9 = 0xc;
    pcVar3 = FUN_004ddc10(0xc6);
    FUN_004daa20(local_40, s_game5u_0052c298, pcVar3, iVar9);
  }
  else {
    pcVar12 = s_jewelsound_0052ad74;
    iVar11 = 0xf;
    uVar10 = 0;
    iVar9 = 0xc;
    pcVar3 = FUN_004ddc10(0xc6);
    iVar9 = FUN_004da780(local_40, s_game5u_0052c298, s_game5u_0052c298, pcVar3, iVar9, uVar10, iVar11, pcVar12);
    if (iVar9 != 0) {
      if (DAT_00b69e30 != 2) {
        __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
        if ((tmpTab != 3) && (tmpTab != 4)) {
          DAT_00bb739e = (char)0xff;
        }
        if (DAT_00bb739d == '\x02') {
          DAT_00bb739d = '\x01';
        }
      }
      tmpVal = 4;
      __builtin_memcpy(DAT_00b6a5aa, &tmpVal, 4);
      iVar6 = 1;
      DAT_00bb73a6 = 1;
      if (DAT_00bb7394 == '\x03') {
        DAT_00bb7394 = '\0';
      }
      FUN_004f70f0();
      FUN_004f36d0();
      FUN_004e5590(DAT_00aa6118);
      DAT_0052c208 = 0;
      if ((DAT_00aa6114 != 0) && (DAT_00aa6134 = 0, DAT_00aa6138 != 0)) {
        piVar4 = (int *)(DAT_00aa6114 + 0x140);
        do {
          __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
          __builtin_memcpy(&tmpItem, DAT_00b6a5ae + tmpTab * 4u, 4);
          if ((unsigned int)*piVar4 == tmpItem) {
            if (*(int *)(DAT_00aa6114 + 0x144 + DAT_00aa6134 * 0x148) != 0) {
              uVar5 = 0;
              if ((int)DAT_00aa6138 < 1) goto LAB_004e386f;
              piVar4 = (int *)(DAT_00aa6114 + 0x144);
              goto LAB_004e3842;
            }
            break;
          }
          piVar4 = piVar4 + 0x52;
          DAT_00aa6134 = DAT_00aa6134 + 1;
        } while (DAT_00aa6134 < DAT_00aa6138);
      }
      goto LAB_004e3887;
    }
  }
  goto LAB_004e3cd7;
  while (1) {
    piVar4 = piVar4 + 0x52;
    uVar5 = uVar5 + 1;
    if ((int)DAT_00aa6138 <= (int)uVar5) break;
LAB_004e3e4b:
    if (*piVar4 == 0) {
      tmpVal = *(unsigned int *)(DAT_00aa6114 + 0x140 + uVar5 * 0x148);
      __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
      __builtin_memcpy(DAT_00b6a5ae + tmpTab * 4u, &tmpVal, 4);
      DAT_00aa6134 = uVar5;
      break;
    }
  }
LAB_004e3e78:
  FUN_004f70f0();
  FUN_004f36d0();
  FUN_004e5590(DAT_00aa6118);
  goto LAB_004e4168;
  while (1) {
    piVar4 = piVar4 + 0x52;
    uVar5 = uVar5 + 1;
    if ((int)DAT_00aa6138 <= (int)uVar5) break;
LAB_004e42d1:
    if (*piVar4 == 0) {
      tmpVal = *(unsigned int *)(DAT_00aa6114 + 0x140 + uVar5 * 0x148);
      __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
      __builtin_memcpy(DAT_00b6a5ae + tmpTab * 4u, &tmpVal, 4);
      DAT_00aa6134 = uVar5;
      break;
    }
  }
LAB_004e42fd:
  FUN_004f70f0();
  FUN_004f36d0();
  FUN_004e5590(DAT_00aa6118);
  goto LAB_004e45ed;
  while (1) {
    piVar4 = piVar4 + 0x52;
    uVar5 = uVar5 + 1;
    if ((int)DAT_00aa6138 <= (int)uVar5) break;
LAB_004e4747:
    if (*piVar4 == 0) {
      tmpVal = *(unsigned int *)(DAT_00aa6114 + 0x140 + uVar5 * 0x148);
      __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
      __builtin_memcpy(DAT_00b6a5ae + tmpTab * 4u, &tmpVal, 4);
      DAT_00aa6134 = uVar5;
      break;
    }
  }
LAB_004e4773:
  FUN_004f70f0();
  FUN_004f36d0();
  FUN_004e5590(DAT_00aa6118);
  goto LAB_004e4a63;
  while (1) {
    piVar4 = piVar4 + 0x52;
    uVar5 = uVar5 + 1;
    if ((int)DAT_00aa6138 <= (int)uVar5) break;
LAB_004e4bb0:
    if (*piVar4 == 0) {
      tmpVal = *(unsigned int *)(DAT_00aa6114 + 0x140 + uVar5 * 0x148);
      __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
      __builtin_memcpy(DAT_00b6a5ae + tmpTab * 4u, &tmpVal, 4);
      DAT_00aa6134 = uVar5;
      break;
    }
  }
LAB_004e4bdc:
  FUN_004f70f0();
  FUN_004f36d0();
  FUN_004e5590(DAT_00aa6118);
  goto LAB_004e4fd8;
  while (1) {
    piVar4 = piVar4 + 0x52;
    uVar5 = uVar5 + 1;
    if ((int)DAT_00aa6138 <= (int)uVar5) break;
LAB_004e3842:
    if (*piVar4 == 0) {
      tmpVal = *(unsigned int *)(DAT_00aa6114 + 0x140 + uVar5 * 0x148);
      __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
      __builtin_memcpy(DAT_00b6a5ae + tmpTab * 4u, &tmpVal, 4);
      DAT_00aa6134 = uVar5;
      break;
    }
  }
LAB_004e386f:
  FUN_004f70f0();
  FUN_004f36d0();
  FUN_004e5590(DAT_00aa6118);
LAB_004e3887:
  piVar4 = (int *)(DAT_00aa6114 + 0x140 + DAT_00aa6134 * 0x148);
  if (DAT_00b69e30 == 2) {
    iVar9 = (int)((unsigned int)*piVar4 * 0xa0u);
    __builtin_memcpy(&tmpVal, DAT_00bb0106 + iVar9, 4);
    if (tmpVal != 0) {
      DAT_00bb7398 = DAT_00bb010a[iVar9];
      DAT_00bb7397 = DAT_00bb010e[(int)((unsigned int)*piVar4 * 0xa0u)];
    }
  }
  else {
    iVar9 = (int)((unsigned int)*piVar4 * 0xa0u);
    __builtin_memcpy(&tmpVal, DAT_00bb10a6 + iVar9, 4);
    if (tmpVal != 0) {
      DAT_00bb7398 = DAT_00bb10aa[iVar9];
      DAT_00bb7397 = DAT_00bb10ae[(int)((unsigned int)*piVar4 * 0xa0u)];
    }
  }
LAB_004e3cd7:
  FUN_004d64a0(local_40, 0, -0x1c);
  __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
  if (tmpTab == 3) {
    iVar9 = 0xc;
    pcVar3 = FUN_004ddc10(0xbf);
    FUN_004daa20(local_40, s_game4d_0052c280, pcVar3, iVar9);
    if ((iVar2 != 0) && (iVar9 = FUN_004d6b90(local_40, local_28, local_2c), iVar9 != 0)) {
      uVar1 = FUN_004dc240();
      if (uVar1 == 0) {
        uVar1 = FUN_004dc220();
        if (uVar1 != 0) {
          if (DAT_00aa6114 != 0) {
            if (DAT_00bb73a7 != '\0') {
              FUN_004de5a0(s_jewelsound_0052ad74, 1, 0, 0xff, 0, 0);
            }
            do {
              if (DAT_00aa6138 - DAT_00aa6134 == 1) {
                DAT_00aa6134 = 0;
              }
              else {
                DAT_00aa6134 = DAT_00aa6134 + 1;
              }
            } while (*(int *)(DAT_00aa6114 + 0x144 + DAT_00aa6134 * 0x148) != 0);
            DAT_00bb73a6 = 1;
            tmpVal = *(unsigned int *)(DAT_00aa6114 + 0x140 + DAT_00aa6134 * 0x148);
            __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
            __builtin_memcpy(DAT_00b6a5ae + tmpTab * 4u, &tmpVal, 4);
            FUN_004f70f0();
            FUN_004f36d0();
            FUN_004e5590(DAT_00aa6118);
            DAT_0052c208 = 0;
            if ((DAT_00aa6114 != 0) && (DAT_00aa6134 = 0, DAT_00aa6138 != 0)) {
              piVar4 = (int *)(DAT_00aa6114 + 0x140);
              do {
                __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
                __builtin_memcpy(&tmpItem, DAT_00b6a5ae + tmpTab * 4u, 4);
                if ((unsigned int)*piVar4 == tmpItem) break;
                piVar4 = piVar4 + 0x52;
                DAT_00aa6134 = DAT_00aa6134 + 1;
              } while (DAT_00aa6134 < DAT_00aa6138);
            }
          }
          if (DAT_00b69e30 != 2) {
            iVar9 = FUN_004cfee0();
            goto joined_r0x004e415c;
          }
        }
      }
      else {
        if (DAT_00aa6114 != 0) {
          if (DAT_00bb73a7 != '\0') {
            FUN_004de5a0(s_jewelsound_0052ad74, 1, 0, 0xff, 0, 0);
          }
          do {
            if (DAT_00aa6134 == 0) {
              DAT_00aa6134 = DAT_00aa6138;
            }
            DAT_00aa6134 = DAT_00aa6134 - 1;
          } while (*(int *)(DAT_00aa6114 + 0x144 + DAT_00aa6134 * 0x148) != 0);
          DAT_00bb73a6 = 1;
          tmpVal = *(unsigned int *)(DAT_00aa6114 + 0x140 + DAT_00aa6134 * 0x148);
          __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
          __builtin_memcpy(DAT_00b6a5ae + tmpTab * 4u, &tmpVal, 4);
          FUN_004f70f0();
          FUN_004f36d0();
          FUN_004e5590(DAT_00aa6118);
          DAT_0052c208 = 0;
          if ((DAT_00aa6114 != 0) && (DAT_00aa6134 = 0, DAT_00aa6138 != 0)) {
            piVar4 = (int *)(DAT_00aa6114 + 0x140);
            do {
              __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
              __builtin_memcpy(&tmpItem, DAT_00b6a5ae + tmpTab * 4u, 4);
              if ((unsigned int)*piVar4 == tmpItem) break;
              piVar4 = piVar4 + 0x52;
              DAT_00aa6134 = DAT_00aa6134 + 1;
            } while (DAT_00aa6134 < DAT_00aa6138);
          }
        }
        if (DAT_00b69e30 != 2) {
          iVar9 = FUN_004cfee0();
joined_r0x004e415c:
          if (iVar9 != 0) {
            FUN_004e6310(0);
          }
        }
      }
    }
  }
  else if (iVar2 == 0) {
    iVar9 = 0xc;
    pcVar3 = FUN_004ddc10(0xbf);
    FUN_004daa20(local_40, s_game4u_0052c288, pcVar3, iVar9);
  }
  else {
    pcVar12 = s_jewelsound_0052ad74;
    iVar11 = 0xe;
    uVar10 = 0;
    iVar9 = 0xc;
    pcVar3 = FUN_004ddc10(0xbf);
    iVar9 = FUN_004da780(local_40, s_game4u_0052c288, s_game4u_0052c288, pcVar3, iVar9, uVar10, iVar11, pcVar12);
    if (iVar9 != 0) {
      if (DAT_00b69e30 != 2) {
        __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
        if ((tmpTab != 3) && (tmpTab != 4)) {
          DAT_00bb739e = (char)0xff;
        }
        if (DAT_00bb739d == '\x02') {
          DAT_00bb739d = '\x01';
        }
      }
      tmpVal = 3;
      __builtin_memcpy(DAT_00b6a5aa, &tmpVal, 4);
      iVar6 = 1;
      DAT_00bb73a6 = 1;
      if (DAT_00bb7394 == '\x03') {
        DAT_00bb7394 = '\0';
      }
      if (DAT_00bb7397 == '\x02') {
        DAT_00bb7397 = '\0';
      }
      FUN_004f70f0();
      FUN_004f36d0();
      FUN_004e5590(DAT_00aa6118);
      DAT_0052c208 = 0;
      DAT_00aa6134 = 0;
      if ((DAT_00aa6114 != 0) && (DAT_00aa6138 != 0)) {
        piVar4 = (int *)(DAT_00aa6114 + 0x140);
        do {
          __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
          __builtin_memcpy(&tmpItem, DAT_00b6a5ae + tmpTab * 4u, 4);
          if ((unsigned int)*piVar4 == tmpItem) {
            if (*(int *)(DAT_00aa6114 + 0x144 + DAT_00aa6134 * 0x148) != 0) {
              uVar5 = 0;
              if ((int)DAT_00aa6138 < 1) goto LAB_004e3e78;
              piVar4 = (int *)(DAT_00aa6114 + 0x144);
              goto LAB_004e3e4b;
            }
            break;
          }
          piVar4 = piVar4 + 0x52;
          DAT_00aa6134 = DAT_00aa6134 + 1;
        } while (DAT_00aa6134 < DAT_00aa6138);
      }
    }
  }
LAB_004e4168:
  FUN_004d64a0(local_40, 0, -0x1c);
  if (local_20[2] != 0) {
    __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
    if (tmpTab == 2) {
      iVar9 = 0xc;
      pcVar3 = FUN_004ddc10(0xc4);
      FUN_004daa20(local_40, s_game3d_0052c270, pcVar3, iVar9);
      if ((iVar2 != 0) && (iVar9 = FUN_004d6b90(local_40, local_28, local_2c), iVar9 != 0)) {
        uVar1 = FUN_004dc240();
        if (uVar1 == 0) {
          uVar1 = FUN_004dc220();
          if (uVar1 != 0) {
            if (DAT_00aa6114 != 0) {
              if (DAT_00bb73a7 != '\0') {
                FUN_004de5a0(s_jewelsound_0052ad74, 1, 0, 0xff, 0, 0);
              }
              do {
                if (DAT_00aa6138 - DAT_00aa6134 == 1) {
                  DAT_00aa6134 = 0;
                }
                else {
                  DAT_00aa6134 = DAT_00aa6134 + 1;
                }
              } while (*(int *)(DAT_00aa6114 + 0x144 + DAT_00aa6134 * 0x148) != 0);
              DAT_00bb73a6 = 1;
              tmpVal = *(unsigned int *)(DAT_00aa6114 + 0x140 + DAT_00aa6134 * 0x148);
              __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
              __builtin_memcpy(DAT_00b6a5ae + tmpTab * 4u, &tmpVal, 4);
              FUN_004f70f0();
              FUN_004f36d0();
              FUN_004e5590(DAT_00aa6118);
              DAT_0052c208 = 0;
              if ((DAT_00aa6114 != 0) && (DAT_00aa6134 = 0, DAT_00aa6138 != 0)) {
                piVar4 = (int *)(DAT_00aa6114 + 0x140);
                do {
                  __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
                  __builtin_memcpy(&tmpItem, DAT_00b6a5ae + tmpTab * 4u, 4);
                  if ((unsigned int)*piVar4 == tmpItem) break;
                  piVar4 = piVar4 + 0x52;
                  DAT_00aa6134 = DAT_00aa6134 + 1;
                } while (DAT_00aa6134 < DAT_00aa6138);
              }
            }
            if (DAT_00b69e30 != 2) {
              iVar9 = FUN_004cfee0();
              goto joined_r0x004e45e1;
            }
          }
        }
        else {
          if (DAT_00aa6114 != 0) {
            if (DAT_00bb73a7 != '\0') {
              FUN_004de5a0(s_jewelsound_0052ad74, 1, 0, 0xff, 0, 0);
            }
            do {
              if (DAT_00aa6134 == 0) {
                DAT_00aa6134 = DAT_00aa6138;
              }
              DAT_00aa6134 = DAT_00aa6134 - 1;
            } while (*(int *)(DAT_00aa6114 + 0x144 + DAT_00aa6134 * 0x148) != 0);
            DAT_00bb73a6 = 1;
            tmpVal = *(unsigned int *)(DAT_00aa6114 + 0x140 + DAT_00aa6134 * 0x148);
            __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
            __builtin_memcpy(DAT_00b6a5ae + tmpTab * 4u, &tmpVal, 4);
            FUN_004f70f0();
            FUN_004f36d0();
            FUN_004e5590(DAT_00aa6118);
            DAT_0052c208 = 0;
            if ((DAT_00aa6114 != 0) && (DAT_00aa6134 = 0, DAT_00aa6138 != 0)) {
              piVar4 = (int *)(DAT_00aa6114 + 0x140);
              do {
                __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
                __builtin_memcpy(&tmpItem, DAT_00b6a5ae + tmpTab * 4u, 4);
                if ((unsigned int)*piVar4 == tmpItem) break;
                piVar4 = piVar4 + 0x52;
                DAT_00aa6134 = DAT_00aa6134 + 1;
              } while (DAT_00aa6134 < DAT_00aa6138);
            }
          }
          if (DAT_00b69e30 != 2) {
            iVar9 = FUN_004cfee0();
joined_r0x004e45e1:
            if (iVar9 != 0) {
              FUN_004e6310(0);
            }
          }
        }
      }
    }
    else if (iVar2 == 0) {
      iVar9 = 0xc;
      pcVar3 = FUN_004ddc10(0xc4);
      FUN_004daa20(local_40, s_game3u_0052c278, pcVar3, iVar9);
    }
    else {
      pcVar12 = s_jewelsound_0052ad74;
      iVar11 = 0xd;
      uVar10 = 0;
      iVar9 = 0xc;
      pcVar3 = FUN_004ddc10(0xc4);
      iVar9 = FUN_004da780(local_40, s_game3u_0052c278, s_game3u_0052c278, pcVar3, iVar9, uVar10, iVar11, pcVar12);
      if (iVar9 != 0) {
        __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
        if (((DAT_00b69e30 != 2) && (tmpTab != 1)) && (tmpTab != 2)) {
          DAT_00bb739e = (char)0xff;
        }
        tmpVal = 2;
        __builtin_memcpy(DAT_00b6a5aa, &tmpVal, 4);
        iVar6 = 1;
        DAT_00bb73a6 = 1;
        if (DAT_00bb7394 == '\x03') {
          DAT_00bb7394 = '\0';
        }
        if (DAT_00bb7397 == '\x02') {
          DAT_00bb7397 = '\0';
        }
        FUN_004f70f0();
        FUN_004f36d0();
        FUN_004e5590(DAT_00aa6118);
        DAT_0052c208 = 0;
        if ((DAT_00aa6114 != 0) && (DAT_00aa6134 = 0, DAT_00aa6138 != 0)) {
          piVar4 = (int *)(DAT_00aa6114 + 0x140);
          do {
            __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
            __builtin_memcpy(&tmpItem, DAT_00b6a5ae + tmpTab * 4u, 4);
            if ((unsigned int)*piVar4 == tmpItem) {
              if (*(int *)(DAT_00aa6114 + 0x144 + DAT_00aa6134 * 0x148) != 0) {
                uVar5 = 0;
                if ((int)DAT_00aa6138 < 1) goto LAB_004e42fd;
                piVar4 = (int *)(DAT_00aa6114 + 0x144);
                goto LAB_004e42d1;
              }
              break;
            }
            piVar4 = piVar4 + 0x52;
            DAT_00aa6134 = DAT_00aa6134 + 1;
          } while (DAT_00aa6134 < DAT_00aa6138);
        }
      }
    }
  }
LAB_004e45ed:
  FUN_004d64a0(local_40, 0, -0x1c);
  __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
  if (tmpTab == 1) {
    iVar9 = 0xc;
    pcVar3 = FUN_004ddc10(0xbe);
    FUN_004daa20(local_40, s_game2d_0052c260, pcVar3, iVar9);
    if ((iVar2 != 0) && (iVar9 = FUN_004d6b90(local_40, local_28, local_2c), iVar9 != 0)) {
      uVar1 = FUN_004dc240();
      if (uVar1 == 0) {
        uVar1 = FUN_004dc220();
        if (uVar1 != 0) {
          if (DAT_00aa6114 != 0) {
            if (DAT_00bb73a7 != '\0') {
              FUN_004de5a0(s_jewelsound_0052ad74, 1, 0, 0xff, 0, 0);
            }
            do {
              if (DAT_00aa6138 - DAT_00aa6134 == 1) {
                DAT_00aa6134 = 0;
              }
              else {
                DAT_00aa6134 = DAT_00aa6134 + 1;
              }
            } while (*(int *)(DAT_00aa6114 + 0x144 + DAT_00aa6134 * 0x148) != 0);
            DAT_00bb73a6 = 1;
            tmpVal = *(unsigned int *)(DAT_00aa6114 + 0x140 + DAT_00aa6134 * 0x148);
            __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
            __builtin_memcpy(DAT_00b6a5ae + tmpTab * 4u, &tmpVal, 4);
            FUN_004f70f0();
            FUN_004f36d0();
            FUN_004e5590(DAT_00aa6118);
            DAT_0052c208 = 0;
            if ((DAT_00aa6114 != 0) && (DAT_00aa6134 = 0, DAT_00aa6138 != 0)) {
              piVar4 = (int *)(DAT_00aa6114 + 0x140);
              do {
                __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
                __builtin_memcpy(&tmpItem, DAT_00b6a5ae + tmpTab * 4u, 4);
                if ((unsigned int)*piVar4 == tmpItem) break;
                piVar4 = piVar4 + 0x52;
                DAT_00aa6134 = DAT_00aa6134 + 1;
              } while (DAT_00aa6134 < DAT_00aa6138);
            }
          }
          if (DAT_00b69e30 != 2) {
            iVar9 = FUN_004cfee0();
            goto joined_r0x004e4a57;
          }
        }
      }
      else {
        if (DAT_00aa6114 != 0) {
          if (DAT_00bb73a7 != '\0') {
            FUN_004de5a0(s_jewelsound_0052ad74, 1, 0, 0xff, 0, 0);
          }
          do {
            if (DAT_00aa6134 == 0) {
              DAT_00aa6134 = DAT_00aa6138;
            }
            DAT_00aa6134 = DAT_00aa6134 - 1;
          } while (*(int *)(DAT_00aa6114 + 0x144 + DAT_00aa6134 * 0x148) != 0);
          DAT_00bb73a6 = 1;
          tmpVal = *(unsigned int *)(DAT_00aa6114 + 0x140 + DAT_00aa6134 * 0x148);
          __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
          __builtin_memcpy(DAT_00b6a5ae + tmpTab * 4u, &tmpVal, 4);
          FUN_004f70f0();
          FUN_004f36d0();
          FUN_004e5590(DAT_00aa6118);
          DAT_0052c208 = 0;
          if ((DAT_00aa6114 != 0) && (DAT_00aa6134 = 0, DAT_00aa6138 != 0)) {
            piVar4 = (int *)(DAT_00aa6114 + 0x140);
            do {
              __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
              __builtin_memcpy(&tmpItem, DAT_00b6a5ae + tmpTab * 4u, 4);
              if ((unsigned int)*piVar4 == tmpItem) break;
              piVar4 = piVar4 + 0x52;
              DAT_00aa6134 = DAT_00aa6134 + 1;
            } while (DAT_00aa6134 < DAT_00aa6138);
          }
        }
        if (DAT_00b69e30 != 2) {
          iVar9 = FUN_004cfee0();
joined_r0x004e4a57:
          if (iVar9 != 0) {
            FUN_004e6310(0);
          }
        }
      }
    }
  }
  else if (iVar2 == 0) {
    iVar9 = 0xc;
    pcVar3 = FUN_004ddc10(0xbe);
    FUN_004daa20(local_40, s_game2u_0052c268, pcVar3, iVar9);
  }
  else {
    pcVar12 = s_jewelsound_0052ad74;
    iVar11 = 0xc;
    uVar10 = 0;
    iVar9 = 0xc;
    pcVar3 = FUN_004ddc10(0xbe);
    iVar9 = FUN_004da780(local_40, s_game2u_0052c268, s_game2u_0052c268, pcVar3, iVar9, uVar10, iVar11, pcVar12);
    if (iVar9 != 0) {
      __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
      if (((DAT_00b69e30 != 2) && (tmpTab != 1)) && (tmpTab != 2)) {
        DAT_00bb739e = (char)0xff;
      }
      iVar6 = 1;
      DAT_00bb73a6 = 1;
      tmpVal = 1;
      __builtin_memcpy(DAT_00b6a5aa, &tmpVal, 4);
      if (DAT_00bb7394 == '\x03') {
        DAT_00bb7394 = '\0';
      }
      if (DAT_00bb7397 == '\x02') {
        DAT_00bb7397 = '\0';
      }
      FUN_004f70f0();
      FUN_004f36d0();
      FUN_004e5590(DAT_00aa6118);
      DAT_0052c208 = 0;
      if ((DAT_00aa6114 != 0) && (DAT_00aa6134 = 0, DAT_00aa6138 != 0)) {
        piVar4 = (int *)(DAT_00aa6114 + 0x140);
        do {
          __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
          __builtin_memcpy(&tmpItem, DAT_00b6a5ae + tmpTab * 4u, 4);
          if ((unsigned int)*piVar4 == tmpItem) {
            if (*(int *)(DAT_00aa6114 + 0x144 + DAT_00aa6134 * 0x148) != 0) {
              uVar5 = 0;
              if ((int)DAT_00aa6138 < 1) goto LAB_004e4773;
              piVar4 = (int *)(DAT_00aa6114 + 0x144);
              goto LAB_004e4747;
            }
            break;
          }
          piVar4 = piVar4 + 0x52;
          DAT_00aa6134 = DAT_00aa6134 + 1;
        } while (DAT_00aa6134 < DAT_00aa6138);
      }
    }
  }
LAB_004e4a63:
  FUN_004d64a0(local_40, 0, -0x1c);
  __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
  if (tmpTab == 0) {
    iVar9 = 0xc;
    pcVar3 = FUN_004ddc10(0xbd);
    FUN_004daa20(local_40, s_game1d_0052c250, pcVar3, iVar9);
    if ((iVar2 != 0) && (iVar2 = FUN_004d6b90(local_40, local_28, local_2c), iVar2 != 0)) {
      uVar1 = FUN_004dc240();
      if (uVar1 == 0) {
        uVar1 = FUN_004dc220();
        if (uVar1 == 0) goto LAB_004e4fd8;
        if (DAT_00aa6114 != 0) {
          if (DAT_00bb73a7 != '\0') {
            FUN_004de5a0(s_jewelsound_0052ad74, 1, 0, 0xff, 0, 0);
          }
          do {
            if (DAT_00aa6138 - DAT_00aa6134 == 1) {
              DAT_00aa6134 = 0;
            }
            else {
              DAT_00aa6134 = DAT_00aa6134 + 1;
            }
          } while (*(int *)(DAT_00aa6114 + 0x144 + DAT_00aa6134 * 0x148) != 0);
          tmpVal = *(unsigned int *)(DAT_00aa6114 + 0x140 + DAT_00aa6134 * 0x148);
          __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
          __builtin_memcpy(DAT_00b6a5ae + tmpTab * 4u, &tmpVal, 4);
          iVar2 = 0;
          if (0 < DAT_00aa5aac) {
            pcVar3 = &DAT_00a91d24;
            do {
              if (*pcVar3 != '\0') break;
              pcVar3 = pcVar3 + 0x562;
              iVar2 = iVar2 + 1;
            } while (iVar2 < DAT_00aa5aac);
          }
          DAT_00bb73a6 = 1;
          local_30 = (unsigned int)(unsigned char)DAT_00a91d17[(unsigned int)iVar2 * 0x562u];
          FUN_004f70f0();
          iVar2 = 0;
          if (0 < DAT_00aa5aac) {
            pcVar3 = &DAT_00a91d24;
            do {
              if (*pcVar3 != '\0') break;
              pcVar3 = pcVar3 + 0x562;
              iVar2 = iVar2 + 1;
            } while (iVar2 < DAT_00aa5aac);
          }
          if ((unsigned char)DAT_00a91d17[(unsigned int)iVar2 * 0x562u] != local_30) {
            iVar6 = 1;
          }
          FUN_004f36d0();
          FUN_004e5590(DAT_00aa6118);
          DAT_0052c208 = 0;
          if ((DAT_00aa6114 != 0) && (DAT_00aa6134 = 0, DAT_00aa6138 != 0)) {
            piVar4 = (int *)(DAT_00aa6114 + 0x140);
            do {
              __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
              __builtin_memcpy(&tmpItem, DAT_00b6a5ae + tmpTab * 4u, 4);
              if ((unsigned int)*piVar4 == tmpItem) break;
              piVar4 = piVar4 + 0x52;
              DAT_00aa6134 = DAT_00aa6134 + 1;
            } while (DAT_00aa6134 < DAT_00aa6138);
          }
        }
        if (DAT_00b69e30 == 2) goto LAB_004e4fd8;
        iVar2 = FUN_004cfee0();
      }
      else {
        if (DAT_00aa6114 != 0) {
          if (DAT_00bb73a7 != '\0') {
            FUN_004de5a0(s_jewelsound_0052ad74, 1, 0, 0xff, 0, 0);
          }
          do {
            if (DAT_00aa6134 == 0) {
              DAT_00aa6134 = DAT_00aa6138;
            }
            DAT_00aa6134 = DAT_00aa6134 - 1;
          } while (*(int *)(DAT_00aa6114 + 0x144 + DAT_00aa6134 * 0x148) != 0);
          tmpVal = *(unsigned int *)(DAT_00aa6114 + 0x140 + DAT_00aa6134 * 0x148);
          __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
          __builtin_memcpy(DAT_00b6a5ae + tmpTab * 4u, &tmpVal, 4);
          iVar2 = 0;
          if (0 < DAT_00aa5aac) {
            pcVar3 = &DAT_00a91d24;
            do {
              if (*pcVar3 != '\0') break;
              pcVar3 = pcVar3 + 0x562;
              iVar2 = iVar2 + 1;
            } while (iVar2 < DAT_00aa5aac);
          }
          DAT_00bb73a6 = 1;
          local_30 = (unsigned int)(unsigned char)DAT_00a91d17[(unsigned int)iVar2 * 0x562u];
          FUN_004f70f0();
          iVar2 = 0;
          if (0 < DAT_00aa5aac) {
            pcVar3 = &DAT_00a91d24;
            do {
              if (*pcVar3 != '\0') break;
              pcVar3 = pcVar3 + 0x562;
              iVar2 = iVar2 + 1;
            } while (iVar2 < DAT_00aa5aac);
          }
          if ((unsigned char)DAT_00a91d17[(unsigned int)iVar2 * 0x562u] != local_30) {
            iVar6 = 1;
          }
          FUN_004f36d0();
          FUN_004e5590(DAT_00aa6118);
          DAT_0052c208 = 0;
          if ((DAT_00aa6114 != 0) && (DAT_00aa6134 = 0, DAT_00aa6138 != 0)) {
            piVar4 = (int *)(DAT_00aa6114 + 0x140);
            do {
              __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
              __builtin_memcpy(&tmpItem, DAT_00b6a5ae + tmpTab * 4u, 4);
              if ((unsigned int)*piVar4 == tmpItem) break;
              piVar4 = piVar4 + 0x52;
              DAT_00aa6134 = DAT_00aa6134 + 1;
            } while (DAT_00aa6134 < DAT_00aa6138);
          }
        }
        if (DAT_00b69e30 == 2) goto LAB_004e4fd8;
        iVar2 = FUN_004cfee0();
      }
      if (iVar2 != 0) {
        FUN_004e6310(0);
      }
    }
  }
  else if (iVar2 == 0) {
    iVar2 = 0xc;
    pcVar3 = FUN_004ddc10(0xbd);
    FUN_004daa20(local_40, s_game1u_0052c258, pcVar3, iVar2);
  }
  else {
    pcVar12 = s_jewelsound_0052ad74;
    iVar9 = 0xb;
    uVar10 = 0;
    iVar2 = 0xc;
    pcVar3 = FUN_004ddc10(0xbd);
    iVar2 = FUN_004da780(local_40, s_game1u_0052c258, s_game1u_0052c258, pcVar3, iVar2, uVar10, iVar9, pcVar12);
    if (iVar2 != 0) {
      if (DAT_00b69e30 != 2) {
        DAT_00bb739e = (char)0xff;
      }
      iVar6 = 1;
      DAT_00bb73a6 = 1;
      tmpVal = 0;
      __builtin_memcpy(DAT_00b6a5aa, &tmpVal, 4);
      if (DAT_00bb7394 == '\x03') {
        DAT_00bb7394 = '\0';
      }
      if (DAT_00bb7397 == '\x02') {
        DAT_00bb7397 = '\0';
      }
      FUN_004f70f0();
      FUN_004f36d0();
      FUN_004e5590(DAT_00aa6118);
      DAT_0052c208 = 0;
      if ((DAT_00aa6114 != 0) && (DAT_00aa6134 = 0, DAT_00aa6138 != 0)) {
        piVar4 = (int *)(DAT_00aa6114 + 0x140);
        do {
          __builtin_memcpy(&tmpTab, DAT_00b6a5aa, 4);
          __builtin_memcpy(&tmpItem, DAT_00b6a5ae + tmpTab * 4u, 4);
          if ((unsigned int)*piVar4 == tmpItem) {
            if (*(int *)(DAT_00aa6114 + 0x144 + DAT_00aa6134 * 0x148) != 0) {
              uVar5 = 0;
              if ((int)DAT_00aa6138 < 1) goto LAB_004e4bdc;
              piVar4 = (int *)(DAT_00aa6114 + 0x144);
              goto LAB_004e4bb0;
            }
            break;
          }
          piVar4 = piVar4 + 0x52;
          DAT_00aa6134 = DAT_00aa6134 + 1;
        } while (DAT_00aa6134 < DAT_00aa6138);
      }
    }
  }
LAB_004e4fd8:
  if (iVar6 != 0) {
    FUN_004b4bf0(s_background_0052bf74);
    FUN_004e9330();
  }
  return iVar6;
}
