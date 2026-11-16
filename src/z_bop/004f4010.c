/* SPDX-License-Identifier: MIT */
/* z_bop 004f4010 */
/* Prototypes */
int  FUN_004cfee0(void);
void FUN_004de090(int *, int *);
void FUN_004d6460(int *, int, int, int, int);
int  FUN_004d6b90(int *, int, int);
unsigned char FUN_004dc1e0(void);
unsigned char FUN_004dc200(void);
unsigned char FUN_004dc220(void);
unsigned char FUN_004dc240(void);
void FUN_004d64a0(int *, int, int);
void FUN_004daba0(int *);
char *FUN_004ddc10(int);
int  FUN_004da780(int *, char *, char *, char *, int, int, int, char *);
void FUN_004f46c0(void);
void FUN_004f4580(void);
void FUN_004daa20(int *, char *, char *, int);
void FUN_004b4a60(char *, char *);
void FUN_004d4e20(void);
void FUN_004b5de0(char *, int, int);
void FUN_004b5570(char *, int, int);
void FUN_004b4d40(char *, int, int);
void FUN_004d4ec0(int);

/* Globals */
extern int DAT_006691d0;
extern int DAT_00b69e30;
extern unsigned char DAT_00b6a5aa[4];   /* 4-byte value at unaligned address 0x00b6a5aa */
extern unsigned char DAT_00b6a606[4];   /* 4-byte value at unaligned address 0x00b6a606 */
extern int DAT_0052c208;
extern int DAT_00b6a250;
extern int DAT_00aa5e68;
extern int DAT_00aa5e10[];

extern char s_jewelsound_0052ad74[];
extern char s_clearu_0052c8f4[];
extern char s_cleard_0052c8fc[];
extern char s_assignu_0052c8e4[];
extern char s_assignd_0052c8ec[];
extern char s_team2d_0052c8dc[];
extern char s_team2u_0052c8d4[];
extern char s_team1d_0052c8cc[];
extern char s_team1u_0052c8c4[];
extern char s_frontres_soloteam_bmp_0052c890[];
extern char s_background_0052bf74[];
extern char s_frame_0052a6d4[];
extern char s_clientactive_0052b69c[];
extern char s_allactive_0052b6ac[];
extern char s_chatbox_0052bfac[];
extern char s_teamoverlay_0052c884[];
extern char s_captoverlay_0052c878[];
extern char s_slotoverlay_0052c86c[];

unsigned int __stdcall FUN_004f4010(void)
{
  char bVar1;
  unsigned char uVar2;
  int iVar3;
  char *pcVar4;
  int *piVar5;
  int uVar6;
  int iVar7;
  char *pcVar8;
  int local_38[4];
  int local_28;
  int local_24;
  /* local_20[0..5] at -0x20..-0xc, local_20[6] is local_8 (-0x8), local_20[7] is local_4 (-0x4) */
  int local_20[8];
  int tmp_5aa;
  int tmp_606;

  iVar3 = FUN_004cfee0();
  if ((iVar3 != 0) || (bVar1 = 0, DAT_00b69e30 == 2)) {
    bVar1 = 1;
  }
  __builtin_memcpy(&tmp_5aa, DAT_00b6a5aa, 4);
  __builtin_memcpy(&tmp_606, DAT_00b6a606, 4);
  if ((tmp_5aa == 3) && (tmp_606 == 1)) {
    local_20[0] = 1;
    local_20[1] = 1;
    local_20[DAT_006691d0] = 2;
  }
  else {
    local_20[0] = 0;
    local_20[1] = 0;
  }
  local_20[2] = 0;
  local_20[3] = 0;
  local_20[4] = 0;
  if ((bVar1) && (DAT_006691d0 == 0)) {
    local_20[5] = 1;
  }
  else {
    local_20[5] = 0;
  }
  local_20[7] = 0;            /* local_4 */
  local_20[6] = local_20[5];  /* local_8 */
  FUN_004de090(&local_24, &local_28);
  FUN_004de090(&local_24, &local_28);
  if (bVar1) {
    FUN_004d6460(local_38, 0x16, 0x132, 0x2a, 0x14a);
    piVar5 = local_20 + 5;
    do {
      iVar3 = FUN_004d6b90(local_38, local_24, local_28);
      if (iVar3 != 0) {
        uVar2 = FUN_004dc1e0();
        if (uVar2 == 0) {
          uVar2 = FUN_004dc200();
          if (uVar2 == 0) {
            uVar2 = FUN_004dc220();
            if (uVar2 == 0) {
              uVar2 = FUN_004dc240();
              if (uVar2 == 0) goto LAB_004f412d;
            }
          }
        }
        *piVar5 = 2;
      }
LAB_004f412d:
      piVar5 = piVar5 + 1;
      FUN_004d64a0(local_38, 0, 0x1c);
    } while (piVar5 < local_20 + 7);   /* &local_4 */
  }
  FUN_004daba0(local_20);
  if ((DAT_006691d0 != 0) || (!bVar1)) goto LAB_004f425b;
  FUN_004d6460(local_38, 0x16, 0x14e, 0x2a, 0x166);
  pcVar8 = s_jewelsound_0052ad74;
  iVar7 = 0x10;
  uVar6 = 0;
  iVar3 = 0xc;
  pcVar4 = FUN_004ddc10(0xd7);
  FUN_004da780(local_38, s_clearu_0052c8f4, s_cleard_0052c8fc, pcVar4, iVar3, uVar6, iVar7, pcVar8);
  iVar3 = FUN_004d6b90(local_38, local_24, local_28);
  if (iVar3 != 0) {
    uVar2 = FUN_004dc220();
    if (uVar2 == 0) {
      uVar2 = FUN_004dc240();
      if (uVar2 == 0) goto LAB_004f41e8;
    }
    FUN_004f46c0();
  }
LAB_004f41e8:
  FUN_004d64a0(local_38, 0, -0x1c);
  pcVar8 = s_jewelsound_0052ad74;
  iVar7 = 0x11;
  uVar6 = 0;
  iVar3 = 0xc;
  pcVar4 = FUN_004ddc10(0xd6);
  FUN_004da780(local_38, s_assignu_0052c8e4, s_assignd_0052c8ec, pcVar4, iVar3, uVar6, iVar7, pcVar8);
  iVar3 = FUN_004d6b90(local_38, local_24, local_28);
  if (iVar3 != 0) {
    uVar2 = FUN_004dc220();
    if (uVar2 == 0) {
      uVar2 = FUN_004dc240();
      if (uVar2 == 0) goto LAB_004f425b;
    }
    FUN_004f4580();
  }
LAB_004f425b:
  FUN_004d6460(local_38, 0x16, 0x8e, 0x2a, 0xa6);
  if (local_20[1] != 0) {
    if (DAT_006691d0 == 1) {
      iVar3 = 0xc;
      pcVar4 = FUN_004ddc10(0x31e);
      FUN_004daa20(local_38, s_team2d_0052c8dc, pcVar4, iVar3);
    }
    else {
      pcVar8 = s_jewelsound_0052ad74;
      iVar7 = 0xc;
      uVar6 = 0;
      iVar3 = 0xc;
      pcVar4 = FUN_004ddc10(0x31e);
      iVar3 = FUN_004da780(local_38, s_team2u_0052c8d4, s_team2u_0052c8d4, pcVar4, iVar3, uVar6,
                           iVar7, pcVar8);
      if (iVar3 != 0) {
        DAT_006691d0 = 1;
        DAT_0052c208 = 0;
        FUN_004b4a60(s_frontres_soloteam_bmp_0052c890, s_background_0052bf74);
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
        FUN_004b4d40(s_teamoverlay_0052c884, 0, 0);
        FUN_004d4ec0(1);
      }
    }
  }
  FUN_004d64a0(local_38, 0, -0x1c);
  if (local_20[0] != 0) {
    if (DAT_006691d0 == 0) {
      iVar3 = 0xc;
      pcVar4 = FUN_004ddc10(0x31d);
      FUN_004daa20(local_38, s_team1d_0052c8cc, pcVar4, iVar3);
    }
    else {
      pcVar8 = s_jewelsound_0052ad74;
      iVar7 = 0xb;
      uVar6 = 0;
      iVar3 = 0xc;
      pcVar4 = FUN_004ddc10(0x31d);
      iVar3 = FUN_004da780(local_38, s_team1u_0052c8c4, s_team1u_0052c8c4, pcVar4, iVar3, uVar6,
                           iVar7, pcVar8);
      if (iVar3 != 0) {
        DAT_006691d0 = 0;
        FUN_004b4a60(s_frontres_soloteam_bmp_0052c890, s_background_0052bf74);
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
        FUN_004b4d40(s_teamoverlay_0052c884, 0, 0);
        iVar3 = 0;
        FUN_004d6460(local_38, 0x58, 0xcf, 0x100, 0xdd);
        if (0 < DAT_00aa5e68) {
          do {
            if (((DAT_00aa5e68 == 8) || (DAT_00aa5e68 == 7)) && (iVar3 == 4)) {
              FUN_004d6460(local_38, 0x104, 0xcf, 0x1ac, 0xdd);
            }
            iVar7 = 0;
            FUN_004d64a0(local_38, 0, 0xf);
            if (0 < DAT_00aa5e10[iVar3]) {
              do {
                if (iVar7 == 0) {
                  pcVar4 = s_captoverlay_0052c878;
                }
                else {
                  pcVar4 = s_slotoverlay_0052c86c;
                }
                iVar7 = iVar7 + 1;
                FUN_004b4d40(pcVar4, local_38[0], local_38[1]);
                FUN_004d64a0(local_38, 0, 0xf);
              } while (iVar7 < DAT_00aa5e10[iVar3]);
            }
            iVar3 = iVar3 + 1;
          } while (iVar3 < DAT_00aa5e68);
        }
        FUN_004d4ec0(1);
      }
    }
  }
  return 1;
}
