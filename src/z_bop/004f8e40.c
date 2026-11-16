/* SPDX-License-Identifier: MIT */
/* z_bop 004f8e40 */
extern int DAT_00aa5cb0[];
extern int DAT_00b6a5a6;
extern int DAT_00aa5e10[];
extern int DAT_00b69e30;
extern int DAT_00669790;
extern short DAT_006691e6;
extern short DAT_006696e0;
extern int DAT_00669218;
extern int DAT_0066921c;
extern int DAT_0052c208;
extern int DAT_00b6a250;
extern int DAT_00b6a2a4;
extern short DAT_00aa5aac;
extern char DAT_00a91d24;
extern char DAT_00bb73a7;
extern unsigned int DAT_00669220;

extern char s_jewelsound_0052ad74[];
extern char s_clearu_0052c8f4[];
extern char s_cleard_0052c8fc[];
extern char s_assignu_0052c8e4[];
extern char s_assignd_0052c8ec[];
extern char s_play2d_0052d090[];
extern char s_play2u_0052d088[];
extern char s_play1d_0052d080[];
extern char s_play1u_0052d078[];
extern char s_background_0052bf74[];
extern char s_frame_0052a6d4[];
extern char s_clientactive_0052b69c[];
extern char s_allactive_0052b6ac[];
extern char s_chatbox_0052bfac[];
extern char s_mapassignoverlay_0052cf8c[];
extern char s_fgslotoverlay_0052cf7c[];
extern char s_mapoverlay_0052cf70[];
extern char s_map4u_0052d068[];
extern char s_map4d_0052d070[];
extern char s_map2d_0052d060[];
extern char s_map2u_0052d058[];
extern char s_map1d_0052d050[];
extern char s_map1u_0052d048[];
extern char s_clear2u_0052d038[];
extern char s_clear2d_0052d040[];
extern char s_assign2u_0052d020[];
extern char s_assign2d_0052d02c[];

int FUN_004d11e0(void);
void FUN_004d6460(int *param_1, int param_2, int param_3, int param_4, int param_5);
void FUN_004de090(int *param_1, int *param_2);
int FUN_004d6b90(int *param_1, int param_2, int param_3);
unsigned char FUN_004dc1e0(void);
unsigned char FUN_004dc200(void);
unsigned char FUN_004dc220(void);
unsigned char FUN_004dc240(void);
void FUN_004d64a0(int *param_1, int param_2, int param_3);
void FUN_004daba0(int *param_1);
char *FUN_004ddc10(int param_1);
void FUN_004daa20(int *param_1, char *param_2, char *param_3, int param_4);
int FUN_004da780(int *param_1, char *param_2, char *param_3, char *param_4, int param_5, int param_6, int param_7, char *param_8);
void FUN_004d4e20(void);
void FUN_004b5de0(char *param_1, int param_2, int param_3);
void FUN_004b5570(char *param_1, int param_2, int param_3);
void FUN_004b4d40(char *param_1, int param_2, int param_3);
void FUN_004d4ec0(int param_1);
void FUN_004d6480(int *param_1, unsigned int *param_2);
void FUN_004d6550(int *param_1, int param_2, int param_3, int param_4);
void FUN_004de0b0(int param_1, int param_2);
void FUN_004fa690(void);
void FUN_004fa570(void);
void FUN_004f7380(void);
void FUN_004de5a0(char *param_1, int param_2, int param_3, int param_4);
void FUN_004f7480(void);

unsigned int __stdcall FUN_004f8e40(void)
{
  int iVar2;
  int iVar3;
  char *pcVar4;
  int *piVar5;
  int bVar6;
  int bVar7;
  int bVar8;
  unsigned int uVar9;
  char *pcVar10;
  int local_38[4];
  int local_28;
  int local_24[9];

  iVar2 = FUN_004d11e0();
  bVar6 = iVar2 == DAT_00aa5cb0[DAT_00b6a5a6 * 8];
  bVar7 = DAT_00b69e30 == 2;
  bVar8 = DAT_00669790 != 0;
  if (bVar8) {
    local_24[5] = (int)(bVar7 || bVar6);
    local_24[8] = local_24[5];
  }
  else {
    local_24[5] = 0;
  }
  local_24[4] = (int)!bVar8;
  local_24[2] = (int)!bVar8;
  local_24[1] = (int)!bVar8;
  local_24[3] = 0;
  local_24[6] = 0;
  local_24[7] = 0;
  if (DAT_00aa5e10[DAT_00b6a5a6] < 5) {
    if ((bVar7 || bVar6) && (1 < DAT_00aa5e10[DAT_00b6a5a6])) {
      local_24[6] = 1;
      local_24[7] = 1;
    }
  }
  else {
    local_24[6] = 1;
    local_24[7] = 1;
    local_24[DAT_00669790 + 6] = 2;
  }
  if (DAT_00669790 == 0) {
    local_24[(short)(DAT_006691e6 ^ 1) + 1] = 2;
  }
  FUN_004d6460(local_38, 0x16, 0xc6, 0x2a, 0xde);
  FUN_004de090(local_24, &local_28);
  if (((DAT_00669790 == 0) && (iVar2 = FUN_004d6b90(local_38, local_24[0], local_28), iVar2 != 0)) &&
      ((FUN_004dc1e0() != 0 ||
        ((FUN_004dc200() != 0 ||
          (FUN_004dc220() != 0)) ||
         (FUN_004dc240() != 0))))) {
    local_24[4] = 2;
  }
  if (DAT_00aa5e10[DAT_00b6a5a6] < 5) {
    if ((bVar7 || bVar6) && (local_24[6] != 0)) {
      FUN_004d6460(local_38, 0x16, 0x132, 0x2a, 0x14a);
      piVar5 = local_24 + 6;
      do {
        iVar2 = FUN_004d6b90(local_38, local_24[0], local_28);
        if ((iVar2 != 0) &&
            ((FUN_004dc1e0() != 0 ||
              FUN_004dc200() != 0) ||
             (FUN_004dc220() != 0 ||
              FUN_004dc240() != 0))) {
          *piVar5 = 2;
        }
        piVar5 = piVar5 + 1;
        FUN_004d64a0(local_38, 0, 0x1c);
      } while (piVar5 < local_24 + 8);
    }
  }
  else if ((DAT_00669790 == 1) && (bVar7 || bVar6)) {
    iVar2 = 0;
    FUN_004d6460(local_38, 0x16, 0xe2, 0x2a, 0xfa);
    do {
      iVar3 = FUN_004d6b90(local_38, local_24[0], local_28);
      if ((iVar3 != 0) &&
          (((FUN_004dc1e0() != 0 ||
             FUN_004dc200() != 0) ||
            FUN_004dc220() != 0) ||
           FUN_004dc240() != 0)) {
        if (iVar2 == 0) {
          local_24[6] = 2;
        }
        else {
          local_24[7] = 2;
        }
      }
      iVar2 = iVar2 + 1;
      FUN_004d64a0(local_38, 0, 0x1c);
    } while (iVar2 < 2);
  }
  FUN_004daba0(local_24 + 1);
  if (DAT_00aa5e10[DAT_00b6a5a6] < 5) {
    if ((bVar7 || bVar6) && (local_24[6] != 0)) {
      FUN_004d6460(local_38, 0x16, 0x14e, 0x2a, 0x166);
      pcVar10 = s_jewelsound_0052ad74;
      iVar3 = 0x11;
      uVar9 = 0;
      iVar2 = 0xc;
      pcVar4 = FUN_004ddc10(0xd7);
      iVar2 = FUN_004da780(local_38, s_clearu_0052c8f4, s_cleard_0052c8fc, pcVar4, iVar2, uVar9, iVar3,
                           pcVar10);
      if (iVar2 != 0) {
        FUN_004fa690();
      }
      FUN_004d64a0(local_38, 0, -0x1c);
      pcVar10 = s_jewelsound_0052ad74;
      iVar3 = 0x10;
      uVar9 = 0;
      iVar2 = 0xc;
      pcVar4 = FUN_004ddc10(0xd6);
      iVar2 = FUN_004da780(local_38, s_assignu_0052c8e4, s_assignd_0052c8ec, pcVar4, iVar2, uVar9, iVar3,
                           pcVar10);
      if (iVar2 != 0) {
        FUN_004fa570();
      }
    }
  }
  else {
    FUN_004d6460(local_38, 0x16, 0x14e, 0x2a, 0x166);
    if (DAT_00669790 == 1) {
      iVar2 = 0xc;
      pcVar4 = FUN_004ddc10(799);
      FUN_004daa20(local_38, s_play2d_0052d090, pcVar4, iVar2);
    }
    else {
      pcVar10 = s_jewelsound_0052ad74;
      iVar3 = 0x11;
      uVar9 = 0;
      iVar2 = 0xc;
      pcVar4 = FUN_004ddc10(799);
      iVar2 = FUN_004da780(local_38, s_play2u_0052d088, s_play2u_0052d088, pcVar4, iVar2, uVar9, iVar3,
                           pcVar10);
      if (iVar2 != 0) {
        DAT_00669790 = 1;
        DAT_0052c208 = 0;
        FUN_004d4e20();
        FUN_004b5de0(s_background_0052bf74, 0, 0);
        FUN_004b5570(s_frame_0052a6d4, 0, 0);
        if (DAT_00b6a250 == 0) {
          pcVar4 = s_clientactive_0052b69c;
        }
        else {
          pcVar4 = s_allactive_0052b6ac;
        }
        FUN_004b5570(pcVar4, 0, 0);
        if (DAT_00b69e30 != 2) {
          FUN_004b4d40(s_chatbox_0052bfac, 0, 0);
        }
        iVar2 = 0;
        FUN_004b4d40(s_mapassignoverlay_0052cf8c, 0, 0);
        local_24[0] = 0xe6;
        local_28 = 0x11c;
        if (0 < DAT_00aa5aac) {
          pcVar4 = &DAT_00a91d24;
          do {
            if ((*pcVar4 != '\0') && ((unsigned char)pcVar4[-0xc] == DAT_00b6a5a6)) {
              FUN_004b4d40(s_fgslotoverlay_0052cf7c, local_24[0], local_28);
              local_28 = (int)((unsigned int)local_28 + 0x11u);
            }
            pcVar4 = pcVar4 + 0x562;
            iVar2 = iVar2 + 1;
          } while (iVar2 < DAT_00aa5aac);
        }
        FUN_004d4ec0(1);
      }
    }
    FUN_004d64a0(local_38, 0, -0x1c);
    if (DAT_00669790 == 0) {
      iVar2 = 0xc;
      pcVar4 = FUN_004ddc10(800);
      FUN_004daa20(local_38, s_play1d_0052d080, pcVar4, iVar2);
    }
    else {
      pcVar10 = s_jewelsound_0052ad74;
      iVar3 = 0x10;
      uVar9 = 0;
      iVar2 = 0xc;
      pcVar4 = FUN_004ddc10(800);
      iVar2 = FUN_004da780(local_38, s_play1u_0052d078, s_play1u_0052d078, pcVar4, iVar2, uVar9, iVar3,
                           pcVar10);
      if (iVar2 != 0) {
        DAT_00669790 = 0;
        FUN_004d4e20();
        FUN_004b5de0(s_background_0052bf74, 0, 0);
        FUN_004b5570(s_frame_0052a6d4, 0, 0);
        if (DAT_00b6a250 == 0) {
          pcVar4 = s_clientactive_0052b69c;
        }
        else {
          pcVar4 = s_allactive_0052b6ac;
        }
        FUN_004b5570(pcVar4, 0, 0);
        if (DAT_00b69e30 != 2) {
          FUN_004b4d40(s_chatbox_0052bfac, 0, 0);
        }
        FUN_004b4d40(s_mapoverlay_0052cf70, 0, 0);
        FUN_004d6480(local_38, &DAT_00669220);
        FUN_004d64a0(local_38, 0x54, 0x60);
        local_38[1] = (int)((unsigned int)local_38[3] - 0x1bu);
        FUN_004d6550(local_38, 0, 0, DAT_00b6a2a4);
        FUN_004d4ec0(1);
        FUN_004de0b0(0x25, 0x1bd);
      }
    }
  }
  if (DAT_00669790 == 0) {
    FUN_004d6460(local_38, 0x16, 0xc6, 0x2a, 0xde);
    pcVar10 = s_jewelsound_0052ad74;
    iVar3 = 0xe;
    uVar9 = 0;
    iVar2 = 0xc;
    pcVar4 = FUN_004ddc10(0xd2);
    iVar2 = FUN_004da780(local_38, s_map4u_0052d068, s_map4d_0052d070, pcVar4, iVar2, uVar9, iVar3,
                         pcVar10);
    if (iVar2 != 0) {
      DAT_00669218 = 0;
      DAT_0066921c = 0;
      FUN_004f7380();
    }
    FUN_004d64a0(local_38, 0, -0x38);
    if (DAT_006691e6 == 0) {
      iVar2 = 0xc;
      pcVar4 = FUN_004ddc10(0xd0);
      FUN_004daa20(local_38, s_map2d_0052d060, pcVar4, iVar2);
    }
    else {
      pcVar10 = s_jewelsound_0052ad74;
      iVar3 = 0xc;
      uVar9 = 0;
      iVar2 = 0xc;
      pcVar4 = FUN_004ddc10(0xd0);
      iVar2 = FUN_004da780(local_38, s_map2u_0052d058, s_map2u_0052d058, pcVar4, iVar2, uVar9, iVar3,
                           pcVar10);
      if (iVar2 != 0) {
        DAT_006691e6 = 0;
      }
    }
    FUN_004d64a0(local_38, 0, -0x1c);
    if (DAT_006691e6 != 0) {
      iVar2 = FUN_004d6b90(local_38, local_24[0], local_28);
      if ((iVar2 != 0) &&
          (FUN_004dc220() != 0 ||
           FUN_004dc240() != 0)) {
        if (DAT_00bb73a7 != '\0') {
          FUN_004de5a0(s_jewelsound_0052ad74, 1, 0, 0xff);
        }
        FUN_004f7480();
        if (DAT_006696e0 == DAT_00669218) {
          DAT_00669218 = 0;
          DAT_0066921c = 0;
          FUN_004f7380();
        }
      }
      iVar2 = 0xc;
      pcVar4 = FUN_004ddc10(0xd1);
      FUN_004daa20(local_38, s_map1d_0052d050, pcVar4, iVar2);
      return 0;
    }
    pcVar10 = s_jewelsound_0052ad74;
    iVar3 = 0xb;
    uVar9 = 0;
    iVar2 = 0xc;
    pcVar4 = FUN_004ddc10(0x231);
    iVar2 = FUN_004da780(local_38, s_map1u_0052d048, s_map1u_0052d048, pcVar4, iVar2, uVar9, iVar3,
                         pcVar10);
    if (iVar2 != 0) {
      DAT_006691e6 = 1;
      return 0;
    }
  }
  else if ((bVar7 || bVar6) && (local_24[5] != 0)) {
    FUN_004d6460(local_38, 0x16, 0xfe, 0x2a, 0x116);
    pcVar10 = s_jewelsound_0052ad74;
    iVar3 = 0xc;
    uVar9 = 0;
    iVar2 = 0xc;
    pcVar4 = FUN_004ddc10(0xd7);
    iVar2 = FUN_004da780(local_38, s_clear2u_0052d038, s_clear2d_0052d040, pcVar4, iVar2, uVar9, iVar3,
                         pcVar10);
    if (iVar2 != 0) {
      FUN_004fa690();
    }
    FUN_004d64a0(local_38, 0, -0x1c);
    pcVar10 = s_jewelsound_0052ad74;
    iVar3 = 0xb;
    uVar9 = 0;
    iVar2 = 0xc;
    pcVar4 = FUN_004ddc10(0xd6);
    iVar2 = FUN_004da780(local_38, s_assign2u_0052d020, s_assign2d_0052d02c, pcVar4, iVar2, uVar9, iVar3,
                         pcVar10);
    if (iVar2 != 0) {
      FUN_004fa570();
    }
  }
  return 0;
}
