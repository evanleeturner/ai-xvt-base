/* SPDX-License-Identifier: MIT */
/* z_bop 004bb680 */
/* Prototypes for every callee (calling conventions not evidenced; plain default used). */
void FUN_004d6460(int *param_1, int param_2, int param_3, int param_4, int param_5);
char *FUN_004ddc10(int param_1);
void FUN_004db640(int param_1, char *param_2, int *param_3, unsigned int param_4);
int FUN_004dbfa0(char *param_1, int param_2);
void FUN_004b4cc0(char *param_1, int *param_2);
void FUN_004de090(int *param_1, int *param_2);
void FUN_004db7d0(int param_1, char *param_2, int *param_3, int param_4, int param_5, unsigned int param_6);
void FUN_004d64a0(int *param_1, int param_2, int param_3);
void FUN_004d6480(int *param_1, int *param_2);
void FUN_004d4e20(void);
void FUN_004b4d40(char *param_1, int param_2, int param_3);
void FUN_004d4ec0(int param_1);
int FUN_004d6b90(int *param_1, int param_2, int param_3);
unsigned char FUN_004dc220(void);
unsigned char FUN_004dc240(void);
void FUN_004de5a0(char *param_1, int param_2, int param_3, int param_4);
void FUN_004b5570(char *param_1, int param_2, int param_3);
void FUN_004d64c0(int *param_1, int param_2, int param_3);
void FUN_004d6550(int *param_1, int param_2, int param_3, int param_4);
int FUN_004da380(int *param_1, char *param_2, int param_3, int param_4, int param_5, char *param_6);
void FUN_004bc1c0(char *param_1, int *param_2, int param_3);
void FUN_004bc2f0(char *param_1, int *param_2, int param_3);
void FUN_004b8ea0(char *param_1, int *param_2, int param_3);
void FUN_004b9090(char *param_1, int *param_2, int param_3);

/* Globals */
extern char s_3conbtn_0052a700[];
extern char s_configsound_0052a708[];
extern char s_offslot_0052a71c[];
extern int DAT_00b69ce0;
extern int DAT_00b69e30;
extern int DAT_00664e98;
extern int DAT_00b69cc4;
extern int DAT_0052c000;
extern unsigned int DAT_00b6a2c0;
extern char DAT_00bb72d0;
extern char DAT_00bb73a7;
extern char DAT_00bb736f;
extern char DAT_00bb7370;
extern char DAT_00bb7311[];
extern char DAT_00bb72d1[];
extern char DAT_00bb735f[];

void __stdcall FUN_004bb680(void)
{
  unsigned char uVar1;
  char *pcVar2;
  int iVar3;
  int iVar4;
  int *piVar5;
  int iVar6;
  unsigned int uVar7;
  int iVar8;
  unsigned int uVar9;
  char local_29;
  int local_28[4];   /* local_28, local_24, local_20, local_1c */
  int local_18[4];   /* local_18, local_14, local_10, local_c */
  int local_8;
  int local_4;

  FUN_004d6460(local_18, 0x54, 0x5a, 0x25c, 0x6a);
  piVar5 = local_18;
  uVar7 = 0xffff;
  pcVar2 = FUN_004ddc10(0x130);
  FUN_004db640(0xf, pcVar2, piVar5, uVar7);
  iVar8 = 0xc;
  pcVar2 = FUN_004ddc10(0x134);
  iVar3 = FUN_004dbfa0(pcVar2, iVar8);
  iVar8 = 0xc;
  pcVar2 = FUN_004ddc10(0x135);
  iVar8 = FUN_004dbfa0(pcVar2, iVar8);
  if (iVar8 < iVar3) {
    iVar8 = iVar3;
  }
  FUN_004b4cc0(s_offslot_0052a71c, local_28);
  iVar3 = (int)((unsigned int)(local_28[2] - local_28[0]) + 1u);
  FUN_004de090(&local_4, &local_8);
  FUN_004d6460(local_18, 0x58, 0x6f, 0x14c, 0x7d);
  if ((DAT_00b69ce0 == 0) && (DAT_00b69e30 != 2)) {
    uVar7 = 0x2b4;
  }
  else {
    uVar7 = 0x1dd;
  }
  piVar5 = local_18;
  uVar9 = 0xffff;
  iVar6 = 1;
  iVar4 = 0;
  pcVar2 = FUN_004ddc10(uVar7);
  FUN_004db7d0(0xc, pcVar2, piVar5, iVar4, iVar6, uVar9);
  FUN_004d64a0(local_18, 0, 0x14);
  local_18[2] = (int)((unsigned int)local_18[0] + 0x7f);
  FUN_004d6480(local_28, local_18);
  local_28[2] = (int)((unsigned int)local_28[0] + (unsigned int)iVar3);
  if ((DAT_00664e98 != 0) && ((DAT_00b69ce0 != 0 || (DAT_00b69e30 == 2)))) {
    FUN_004d4e20();
    FUN_004b4d40(s_offslot_0052a71c, local_28[0], local_28[1]);
    FUN_004d4ec0(0);
    FUN_004b4d40(s_offslot_0052a71c, local_28[0], local_28[1]);
  }
  iVar4 = FUN_004d6b90(local_28, local_4, local_8);
  if ((iVar4 != 0) && ((DAT_00b69ce0 != 0 || (DAT_00b69e30 == 2)))) {
    uVar1 = FUN_004dc220();
    if (uVar1 == 0) {
      uVar1 = FUN_004dc240();
      if (uVar1 == 0) goto LAB_004bb8a9;
    }
    if (DAT_00bb72d0 != '\0') {
      if (DAT_00bb73a7 != '\0') {
        FUN_004de5a0(s_configsound_0052a708, 1, 0, 0xff);
      }
      if (DAT_00bb72d0 != '\0') {
        DAT_00bb736f = 0;
      }
    }
    DAT_00bb72d0 = '\0';
  }
LAB_004bb8a9:
  if (DAT_00bb72d0 == '\0') {
    FUN_004b5570(s_3conbtn_0052a700, local_28[0], local_28[1]);
  }
  local_28[0] = (int)((unsigned int)local_28[2] + 5u);
  piVar5 = local_28;
  iVar6 = 1;
  iVar4 = 0;
  local_28[2] = (int)((unsigned int)local_28[2] + 0x69u);
  uVar7 = DAT_00b6a2c0;
  pcVar2 = FUN_004ddc10(0x131);
  FUN_004db7d0(0xc, pcVar2, piVar5, iVar4, iVar6, uVar7);
  FUN_004d64a0(local_18, 0, 0x19);
  local_18[2] = (int)((unsigned int)local_18[0] + 0x7f);
  FUN_004d6480(local_28, local_18);
  local_28[2] = (int)((unsigned int)local_28[0] + (unsigned int)iVar3);
  if ((DAT_00664e98 != 0) && ((DAT_00b69ce0 != 0 || (DAT_00b69e30 == 2)))) {
    FUN_004d4e20();
    FUN_004b4d40(s_offslot_0052a71c, local_28[0], local_28[1]);
    FUN_004d4ec0(0);
    FUN_004b4d40(s_offslot_0052a71c, local_28[0], local_28[1]);
  }
  iVar4 = FUN_004d6b90(local_28, local_4, local_8);
  if ((iVar4 != 0) && ((DAT_00b69ce0 != 0 || (DAT_00b69e30 == 2)))) {
    uVar1 = FUN_004dc220();
    if (uVar1 == 0) {
      uVar1 = FUN_004dc240();
      if (uVar1 == 0) goto LAB_004bba21;
    }
    if (DAT_00bb72d0 != '\x01') {
      if (DAT_00bb73a7 != '\0') {
        FUN_004de5a0(s_configsound_0052a708, 1, 0, 0xff);
      }
      if (DAT_00bb72d0 != '\x01') {
        DAT_00bb736f = 1;
      }
    }
    DAT_00bb72d0 = '\x01';
  }
LAB_004bba21:
  if (DAT_00bb72d0 == '\x01') {
    FUN_004b5570(s_3conbtn_0052a700, local_28[0], local_28[1]);
  }
  local_28[0] = (int)((unsigned int)local_28[2] + 5u);
  piVar5 = local_28;
  iVar6 = 1;
  iVar4 = 0;
  local_28[2] = (int)((unsigned int)local_28[2] + 0x69u);
  uVar7 = DAT_00b6a2c0;
  pcVar2 = FUN_004ddc10(0x132);
  FUN_004db7d0(0xc, pcVar2, piVar5, iVar4, iVar6, uVar7);
  FUN_004d64a0(local_28, 100, 0);
  piVar5 = local_28;
  iVar6 = 1;
  iVar4 = 0;
  uVar7 = DAT_00b6a2c0;
  pcVar2 = FUN_004ddc10(0x134);
  FUN_004db7d0(0xc, pcVar2, piVar5, iVar4, iVar6, uVar7);
  local_28[0] = (int)((unsigned int)local_28[0] + (unsigned int)iVar8 + 0xfu);
  local_28[2] = (int)((unsigned int)local_28[0] + 200u);
  FUN_004d64c0(local_28, 0, -2);
  FUN_004d6550(local_28, 0, 0, DAT_00b69cc4);
  iVar4 = FUN_004da380(local_28, DAT_00bb7311, 0x40, 0, 0xc, (char *)0);
  if (iVar4 != 0) {
    DAT_0052c000 = 1;
  }
  FUN_004d64a0(local_18, 0, 0x19);
  local_18[2] = (int)((unsigned int)local_18[0] + 0x7f);
  FUN_004d6480(local_28, local_18);
  local_28[2] = (int)((unsigned int)local_28[0] + (unsigned int)iVar3);
  if ((DAT_00664e98 != 0) && ((DAT_00b69ce0 != 0 || (DAT_00b69e30 == 2)))) {
    FUN_004d4e20();
    FUN_004b4d40(s_offslot_0052a71c, local_28[0], local_28[1]);
    FUN_004d4ec0(0);
    FUN_004b4d40(s_offslot_0052a71c, local_28[0], local_28[1]);
  }
  iVar4 = FUN_004d6b90(local_28, local_4, local_8);
  if ((iVar4 != 0) && ((DAT_00b69ce0 != 0 || (DAT_00b69e30 == 2)))) {
    uVar1 = FUN_004dc220();
    if (uVar1 == 0) {
      uVar1 = FUN_004dc240();
      if (uVar1 == 0) goto LAB_004bbc37;
    }
    if (DAT_00bb72d0 != '\x02') {
      if (DAT_00bb73a7 != '\0') {
        FUN_004de5a0(s_configsound_0052a708, 1, 0, 0xff);
      }
      if (DAT_00bb72d0 != '\x02') {
        DAT_00bb736f = 0;
      }
    }
    DAT_00bb72d0 = '\x02';
  }
LAB_004bbc37:
  if (DAT_00bb72d0 == '\x02') {
    FUN_004b5570(s_3conbtn_0052a700, local_28[0], local_28[1]);
  }
  local_28[0] = (int)((unsigned int)local_28[2] + 5u);
  piVar5 = local_28;
  iVar6 = 1;
  iVar4 = 0;
  local_28[2] = (int)((unsigned int)local_28[2] + 0x69u);
  uVar7 = DAT_00b6a2c0;
  pcVar2 = FUN_004ddc10(0x133);
  FUN_004db7d0(0xc, pcVar2, piVar5, iVar4, iVar6, uVar7);
  FUN_004d64a0(local_28, 100, 0);
  piVar5 = local_28;
  iVar6 = 1;
  iVar4 = 0;
  uVar7 = DAT_00b6a2c0;
  pcVar2 = FUN_004ddc10(0x135);
  FUN_004db7d0(0xc, pcVar2, piVar5, iVar4, iVar6, uVar7);
  local_28[0] = (int)((unsigned int)local_28[0] + (unsigned int)iVar8 + 0xfu);
  local_28[2] = (int)((unsigned int)local_28[0] + 200u);
  FUN_004d64c0(local_28, 0, -2);
  FUN_004d6550(local_28, 0, 0, DAT_00b69cc4);
  iVar8 = FUN_004da380(local_28, DAT_00bb72d1, 0x40, 1, 0xc, (char *)0);
  if (iVar8 != 0) {
    DAT_0052c000 = 2;
  }
  FUN_004d64a0(local_18, 0, 0x19);
  local_18[2] = (int)((unsigned int)local_18[0] + 0x7f);
  FUN_004d6480(local_28, local_18);
  local_28[2] = (int)((unsigned int)iVar3 + (unsigned int)local_28[0]);
  if ((DAT_00664e98 != 0) && ((DAT_00b69ce0 != 0 || (DAT_00b69e30 == 2)))) {
    FUN_004d4e20();
    FUN_004b4d40(s_offslot_0052a71c, local_28[0], local_28[1]);
    FUN_004d4ec0(0);
    FUN_004b4d40(s_offslot_0052a71c, local_28[0], local_28[1]);
  }
  iVar8 = FUN_004d6b90(local_28, local_4, local_8);
  if ((iVar8 != 0) && ((DAT_00b69ce0 != 0 || (DAT_00b69e30 == 2)))) {
    uVar1 = FUN_004dc220();
    if (uVar1 == 0) {
      uVar1 = FUN_004dc240();
      if (uVar1 == 0) goto LAB_004bbe4b;
    }
    if (DAT_00bb72d0 != '\x03') {
      if (DAT_00bb73a7 != '\0') {
        FUN_004de5a0(s_configsound_0052a708, 1, 0, 0xff);
      }
      if (DAT_00bb72d0 != '\x03') {
        DAT_00bb736f = 0;
      }
    }
    DAT_00bb72d0 = '\x03';
  }
LAB_004bbe4b:
  if (DAT_00bb72d0 == '\x03') {
    FUN_004b5570(s_3conbtn_0052a700, local_28[0], local_28[1]);
  }
  local_28[0] = (int)((unsigned int)local_28[2] + 5u);
  piVar5 = local_28;
  iVar3 = 1;
  iVar8 = 0;
  local_28[2] = (int)((unsigned int)local_28[2] + 0x69u);
  uVar7 = DAT_00b6a2c0;
  pcVar2 = FUN_004ddc10(0x1c1);
  FUN_004db7d0(0xc, pcVar2, piVar5, iVar8, iVar3, uVar7);
  FUN_004d64a0(local_18, 0, 0x23);
  piVar5 = local_18;
  uVar7 = 0xffff;
  iVar3 = 1;
  iVar8 = 0;
  pcVar2 = FUN_004ddc10(0x1de);
  FUN_004db7d0(0xc, pcVar2, piVar5, iVar8, iVar3, uVar7);
  FUN_004d64a0(local_18, 0, 0x14);
  FUN_004d6480(local_28, local_18);
  piVar5 = local_28;
  iVar3 = 1;
  iVar8 = 0;
  uVar7 = DAT_00b6a2c0;
  pcVar2 = FUN_004ddc10(0x1e2);
  FUN_004db7d0(0xc, pcVar2, piVar5, iVar8, iVar3, uVar7);
  iVar8 = 0xc;
  pcVar2 = FUN_004ddc10(0x1e2);
  iVar8 = FUN_004dbfa0(pcVar2, iVar8);
  local_28[0] = (int)((unsigned int)local_28[0] + (unsigned int)iVar8 + 0xfu);
  local_28[2] = (int)((unsigned int)local_28[0] + 200u);
  FUN_004d64c0(local_28, 0, -2);
  FUN_004d6550(local_28, 0, 0, DAT_00b69cc4);
  iVar8 = FUN_004da380(local_28, DAT_00bb735f, 0x10, 2, 0xc, (char *)0);
  if (iVar8 != 0) {
    DAT_0052c000 = 0;
  }
  FUN_004d64a0(local_18, 0, 0x23);
  piVar5 = local_18;
  uVar7 = 0xffff;
  iVar3 = 1;
  iVar8 = 0;
  if (DAT_00b69e30 == 3) {
    pcVar2 = FUN_004ddc10(0x2eb);
    FUN_004db7d0(0xc, pcVar2, piVar5, iVar8, iVar3, uVar7);
    FUN_004d64a0(local_18, 0, 0x14);
    piVar5 = local_18;
    iVar3 = 1;
    iVar8 = 0;
    uVar7 = DAT_00b6a2c0;
    pcVar2 = FUN_004ddc10(0x1df);
    FUN_004db7d0(0xc, pcVar2, piVar5, iVar8, iVar3, uVar7);
    FUN_004d64a0(local_18, 0, 0x11);
    FUN_004bc1c0(&DAT_00bb736f, local_18, 0x1e0);
    FUN_004d64a0(local_18, 0, 0x14);
    piVar5 = local_18;
    iVar3 = 1;
    iVar8 = 0;
    uVar7 = DAT_00b6a2c0;
    pcVar2 = FUN_004ddc10(0x2e7);
    FUN_004db7d0(0xc, pcVar2, piVar5, iVar8, iVar3, uVar7);
    FUN_004d64a0(local_18, 0, 0x11);
    local_29 = (char)((DAT_00bb7370 >> 1) - 2);
    FUN_004bc2f0(&local_29, local_18, 0x2e8);
    return;
  }
  pcVar2 = FUN_004ddc10(0x2e6);
  FUN_004db7d0(0xc, pcVar2, piVar5, iVar8, iVar3, uVar7);
  FUN_004d64a0(local_18, 0, 0x14);
  piVar5 = local_18;
  iVar3 = 1;
  iVar8 = 0;
  uVar7 = DAT_00b6a2c0;
  pcVar2 = FUN_004ddc10(0x1df);
  FUN_004db7d0(0xc, pcVar2, piVar5, iVar8, iVar3, uVar7);
  FUN_004d64a0(local_18, 0, 0x11);
  FUN_004b8ea0(&DAT_00bb736f, local_18, 0x1e0);
  FUN_004d64a0(local_18, 0, 0x14);
  piVar5 = local_18;
  iVar3 = 1;
  iVar8 = 0;
  uVar7 = DAT_00b6a2c0;
  pcVar2 = FUN_004ddc10(0x2e7);
  FUN_004db7d0(0xc, pcVar2, piVar5, iVar8, iVar3, uVar7);
  FUN_004d64a0(local_18, 0, 0x11);
  local_29 = (char)((DAT_00bb7370 >> 1) - 2);
  FUN_004b9090(&local_29, local_18, 0x2e8);
  DAT_00bb7370 = (char)((unsigned int)((int)local_29 * 2) + 4u);
  return;
}
