/* SPDX-License-Identifier: MIT */
/* z_xvt 004bbd50 */
extern int DAT_0081b70c;
extern int DAT_0081b850;
extern int DAT_0064ef60;
extern int DAT_0081b6f8;
extern int DAT_00532ad8;
extern unsigned int DAT_0081bcd0;
extern char DAT_0085e6f0;
extern char DAT_0085e7c5;
extern char DAT_0085e78f;
extern char DAT_0085e790;
extern char DAT_0085e731[];
extern char DAT_0085e6f1[];
extern char DAT_0085e77f[];
extern char s_offslot_005313a0[];
extern char s_3conbtn_00531384[];
extern char s_configsound_0053138c[];

void FUN_004dcfd0(int *, int, int, int, int);
char *FUN_004dcfa0(int);
void FUN_004da920(int, char *, int *, unsigned int);
int FUN_004db2f0(char *, int);
void FUN_004d0500(char *, int *);
void FUN_004ddc70(int *, int *);
void FUN_004daac0(int, char *, int *, int, int, unsigned int);
void FUN_004dd030(int *, int, int);
void FUN_004dd000(int *, int *);
void FUN_004d52b0(void);
void FUN_004d0580(char *, int, int);
void FUN_004d5350(int);
int FUN_004dd7d0(int *, int, int);
unsigned char FUN_004db570(void);
unsigned char FUN_004db590(void);
void FUN_004de190(char *, int, int, int);
void FUN_004d0e60(char *, int, int);
void FUN_004dd070(int *, int, int);
void FUN_004dd130(int *, int, int, int);
int FUN_004d9660(int *, char *, int, int, int, char *);
void FUN_004bc8b0(char *, int *, int);
void FUN_004bc9e0(char *, int *, int);
void FUN_004b9970(char *, int *, int);
void FUN_004b9b40(char *, int *, int);

void __cdecl
FUN_004bbd50(char *param_1, int param_2, int param_3, char *param_4, int *param_5, char *param_6,
             int *param_7, char *param_8, char *param_9, unsigned int *param_10, int param_11,
             char *param_12, int *param_13, int *param_14, int *param_15, unsigned int param_16,
             unsigned int param_17, int param_18, int param_19, unsigned int param_20,
             unsigned int param_21)
{
  char *pcVar2;
  int iVar3;
  int iVar4;
  int *piVar5;
  int iVar6;
  unsigned int uVar7;
  int iVar8;
  unsigned int uVar9;
  char cStack_29;
  int local_28[4];
  int local_18[4];
  int local_8;
  int local_4;

  FUN_004dcfd0(local_18, 0x54, 0x5a, 0x25c, 0x6a);
  piVar5 = local_18;
  uVar7 = 0xffff;
  pcVar2 = FUN_004dcfa0(0x123);
  FUN_004da920(0xf, pcVar2, piVar5, uVar7);
  iVar8 = 0xc;
  pcVar2 = FUN_004dcfa0(0x127);
  iVar3 = FUN_004db2f0(pcVar2, iVar8);
  iVar8 = 0xc;
  pcVar2 = FUN_004dcfa0(0x128);
  iVar8 = FUN_004db2f0(pcVar2, iVar8);
  if (iVar8 < iVar3) {
    iVar8 = iVar3;
  }
  FUN_004d0500(s_offslot_005313a0, local_28);
  iVar3 = (int)((unsigned int)(local_28[2] - local_28[0]) + 1u);
  FUN_004ddc70(&local_4, &local_8);
  FUN_004dcfd0(local_18, 0x58, 0x6f, 0x14c, 0x7d);
  if ((DAT_0081b70c == 0) && (DAT_0081b850 != 2)) {
    uVar7 = 0x2a6;
  }
  else {
    uVar7 = 0x1cf;
  }
  piVar5 = local_18;
  uVar9 = 0xffff;
  iVar6 = 1;
  iVar4 = 0;
  pcVar2 = FUN_004dcfa0(uVar7);
  FUN_004daac0(0xc, pcVar2, piVar5, iVar4, iVar6, uVar9);
  FUN_004dd030(local_18, 0, 0x14);
  local_18[2] = (int)((unsigned int)local_18[0] + 0x7fu);
  FUN_004dd000(local_28, local_18);
  local_28[2] = (int)((unsigned int)local_28[0] + (unsigned int)iVar3);
  if ((DAT_0064ef60 != 0) && ((DAT_0081b70c != 0 || (DAT_0081b850 == 2)))) {
    FUN_004d52b0();
    FUN_004d0580(s_offslot_005313a0, local_28[0], local_28[1]);
    FUN_004d5350(0);
    FUN_004d0580(s_offslot_005313a0, local_28[0], local_28[1]);
  }
  iVar4 = FUN_004dd7d0(local_28, local_4, local_8);
  if ((iVar4 != 0) && ((DAT_0081b70c != 0 || (DAT_0081b850 == 2)))) {
    if (FUN_004db570() == 0) {
      if (FUN_004db590() == 0) goto LAB_004bbf80;
    }
    if (DAT_0085e6f0 != '\0') {
      if (DAT_0085e7c5 != '\0') {
        FUN_004de190(s_configsound_0053138c, 1, 0, 0xff);
      }
      if (DAT_0085e6f0 != '\0') {
        DAT_0085e78f = 0;
      }
    }
    DAT_0085e6f0 = '\0';
  }
LAB_004bbf80:
  if (DAT_0085e6f0 == '\0') {
    FUN_004d0e60(s_3conbtn_00531384, local_28[0], local_28[1]);
  }
  local_28[0] = (int)((unsigned int)local_28[2] + 5u);
  piVar5 = local_28;
  iVar6 = 1;
  iVar4 = 0;
  local_28[2] = (int)((unsigned int)local_28[2] + 0x69u);
  uVar7 = DAT_0081bcd0;
  pcVar2 = FUN_004dcfa0(0x124);
  FUN_004daac0(0xc, pcVar2, piVar5, iVar4, iVar6, uVar7);
  FUN_004dd030(local_18, 0, 0x19);
  local_18[2] = (int)((unsigned int)local_18[0] + 0x7fu);
  FUN_004dd000(local_28, local_18);
  local_28[2] = (int)((unsigned int)local_28[0] + (unsigned int)iVar3);
  if ((DAT_0064ef60 != 0) && ((DAT_0081b70c != 0 || (DAT_0081b850 == 2)))) {
    FUN_004d52b0();
    FUN_004d0580(s_offslot_005313a0, local_28[0], local_28[1]);
    FUN_004d5350(0);
    FUN_004d0580(s_offslot_005313a0, local_28[0], local_28[1]);
  }
  iVar4 = FUN_004dd7d0(local_28, local_4, local_8);
  if ((iVar4 != 0) && ((DAT_0081b70c != 0 || (DAT_0081b850 == 2)))) {
    if (FUN_004db570() == 0) {
      if (FUN_004db590() == 0) goto LAB_004bc0fb;
    }
    if (DAT_0085e6f0 != '\x01') {
      if (DAT_0085e7c5 != '\0') {
        FUN_004de190(s_configsound_0053138c, 1, 0, 0xff);
      }
      if (DAT_0085e6f0 != '\x01') {
        DAT_0085e78f = 1;
      }
    }
    DAT_0085e6f0 = '\x01';
  }
LAB_004bc0fb:
  if (DAT_0085e6f0 == '\x01') {
    FUN_004d0e60(s_3conbtn_00531384, local_28[0], local_28[1]);
  }
  local_28[0] = (int)((unsigned int)local_28[2] + 5u);
  piVar5 = local_28;
  iVar6 = 1;
  iVar4 = 0;
  local_28[2] = (int)((unsigned int)local_28[2] + 0x69u);
  uVar7 = DAT_0081bcd0;
  pcVar2 = FUN_004dcfa0(0x125);
  FUN_004daac0(0xc, pcVar2, piVar5, iVar4, iVar6, uVar7);
  FUN_004dd030(local_28, 100, 0);
  piVar5 = local_28;
  iVar6 = 1;
  iVar4 = 0;
  uVar7 = DAT_0081bcd0;
  pcVar2 = FUN_004dcfa0(0x127);
  FUN_004daac0(0xc, pcVar2, piVar5, iVar4, iVar6, uVar7);
  local_28[0] = (int)((unsigned int)iVar8 + 0xfu + (unsigned int)local_28[0]);
  local_28[2] = (int)((unsigned int)local_28[0] + 200u);
  FUN_004dd070(local_28, 0, -2);
  FUN_004dd130(local_28, 0, 0, DAT_0081b6f8);
  iVar4 = FUN_004d9660(local_28, (char *)&DAT_0085e731, 0x40, 0, 0xc, (char *)0);
  if (iVar4 != 0) {
    DAT_00532ad8 = 1;
  }
  FUN_004dd030(local_18, 0, 0x19);
  local_18[2] = (int)((unsigned int)local_18[0] + 0x7fu);
  FUN_004dd000(local_28, local_18);
  local_28[2] = (int)((unsigned int)local_28[0] + (unsigned int)iVar3);
  if ((DAT_0064ef60 != 0) && ((DAT_0081b70c != 0 || (DAT_0081b850 == 2)))) {
    FUN_004d52b0();
    FUN_004d0580(s_offslot_005313a0, local_28[0], local_28[1]);
    FUN_004d5350(0);
    FUN_004d0580(s_offslot_005313a0, local_28[0], local_28[1]);
  }
  iVar4 = FUN_004dd7d0(local_28, local_4, local_8);
  if ((iVar4 != 0) && ((DAT_0081b70c != 0 || (DAT_0081b850 == 2)))) {
    if (FUN_004db570() == 0) {
      if (FUN_004db590() == 0) goto LAB_004bc315;
    }
    if (DAT_0085e6f0 != '\x02') {
      if (DAT_0085e7c5 != '\0') {
        FUN_004de190(s_configsound_0053138c, 1, 0, 0xff);
      }
      if (DAT_0085e6f0 != '\x02') {
        DAT_0085e78f = 0;
      }
    }
    DAT_0085e6f0 = '\x02';
  }
LAB_004bc315:
  if (DAT_0085e6f0 == '\x02') {
    FUN_004d0e60(s_3conbtn_00531384, local_28[0], local_28[1]);
  }
  local_28[0] = (int)((unsigned int)local_28[2] + 5u);
  piVar5 = local_28;
  iVar6 = 1;
  iVar4 = 0;
  local_28[2] = (int)((unsigned int)local_28[2] + 0x69u);
  uVar7 = DAT_0081bcd0;
  pcVar2 = FUN_004dcfa0(0x126);
  FUN_004daac0(0xc, pcVar2, piVar5, iVar4, iVar6, uVar7);
  FUN_004dd030(local_28, 100, 0);
  piVar5 = local_28;
  iVar6 = 1;
  iVar4 = 0;
  uVar7 = DAT_0081bcd0;
  pcVar2 = FUN_004dcfa0(0x128);
  FUN_004daac0(0xc, pcVar2, piVar5, iVar4, iVar6, uVar7);
  local_28[0] = (int)((unsigned int)local_28[0] + (unsigned int)iVar8 + 0xfu);
  local_28[2] = (int)((unsigned int)local_28[0] + 200u);
  FUN_004dd070(local_28, 0, -2);
  FUN_004dd130(local_28, 0, 0, DAT_0081b6f8);
  iVar8 = FUN_004d9660(local_28, (char *)&DAT_0085e6f1, 0x40, 1, 0xc, (char *)0);
  if (iVar8 != 0) {
    DAT_00532ad8 = 2;
  }
  FUN_004dd030(local_18, 0, 0x19);
  local_18[2] = (int)((unsigned int)local_18[0] + 0x7fu);
  FUN_004dd000(local_28, local_18);
  local_28[2] = (int)((unsigned int)iVar3 + (unsigned int)local_28[0]);
  if ((DAT_0064ef60 != 0) && ((DAT_0081b70c != 0 || (DAT_0081b850 == 2)))) {
    FUN_004d52b0();
    FUN_004d0580(s_offslot_005313a0, local_28[0], local_28[1]);
    FUN_004d5350(0);
    FUN_004d0580(s_offslot_005313a0, local_28[0], local_28[1]);
  }
  iVar8 = FUN_004dd7d0(local_28, local_4, local_8);
  if ((iVar8 != 0) && ((DAT_0081b70c != 0 || (DAT_0081b850 == 2)))) {
    if (FUN_004db570() == 0) {
      if (FUN_004db590() == 0) goto LAB_004bc531;
    }
    if (DAT_0085e6f0 != '\x03') {
      if (DAT_0085e7c5 != '\0') {
        FUN_004de190(s_configsound_0053138c, 1, 0, 0xff);
      }
      if (DAT_0085e6f0 != '\x03') {
        DAT_0085e78f = 0;
      }
    }
    DAT_0085e6f0 = '\x03';
  }
LAB_004bc531:
  if (DAT_0085e6f0 == '\x03') {
    FUN_004d0e60(s_3conbtn_00531384, local_28[0], local_28[1]);
  }
  local_28[0] = (int)((unsigned int)local_28[2] + 5u);
  piVar5 = local_28;
  iVar3 = 1;
  iVar8 = 0;
  local_28[2] = (int)((unsigned int)local_28[2] + 0x69u);
  uVar7 = DAT_0081bcd0;
  pcVar2 = FUN_004dcfa0(0x1b4);
  FUN_004daac0(0xc, pcVar2, piVar5, iVar8, iVar3, uVar7);
  FUN_004dd030(local_18, 0, 0x23);
  piVar5 = local_18;
  uVar7 = 0xffff;
  iVar3 = 1;
  iVar8 = 0;
  pcVar2 = FUN_004dcfa0(0x1d0);
  FUN_004daac0(0xc, pcVar2, piVar5, iVar8, iVar3, uVar7);
  FUN_004dd030(local_18, 0, 0x14);
  FUN_004dd000(local_28, local_18);
  piVar5 = local_28;
  iVar3 = 1;
  iVar8 = 0;
  uVar7 = DAT_0081bcd0;
  pcVar2 = FUN_004dcfa0(0x1d4);
  FUN_004daac0(0xc, pcVar2, piVar5, iVar8, iVar3, uVar7);
  iVar8 = 0xc;
  pcVar2 = FUN_004dcfa0(0x1d4);
  iVar8 = FUN_004db2f0(pcVar2, iVar8);
  local_28[0] = (int)((unsigned int)local_28[0] + (unsigned int)iVar8 + 0xfu);
  local_28[2] = (int)((unsigned int)local_28[0] + 200u);
  FUN_004dd070(local_28, 0, -2);
  FUN_004dd130(local_28, 0, 0, DAT_0081b6f8);
  iVar8 = FUN_004d9660(local_28, (char *)&DAT_0085e77f, 0x10, 2, 0xc, (char *)0);
  if (iVar8 != 0) {
    DAT_00532ad8 = 0;
  }
  FUN_004dd030(local_18, 0, 0x23);
  piVar5 = local_18;
  uVar7 = 0xffff;
  iVar3 = 1;
  iVar8 = 0;
  if (DAT_0081b850 == 3) {
    pcVar2 = FUN_004dcfa0(0x2dd);
    FUN_004daac0(0xc, pcVar2, piVar5, iVar8, iVar3, uVar7);
    FUN_004dd030(local_18, 0, 0x14);
    piVar5 = local_18;
    iVar3 = 1;
    iVar8 = 0;
    uVar7 = DAT_0081bcd0;
    pcVar2 = FUN_004dcfa0(0x1d1);
    FUN_004daac0(0xc, pcVar2, piVar5, iVar8, iVar3, uVar7);
    FUN_004dd030(local_18, 0, 0x11);
    FUN_004bc8b0(&DAT_0085e78f, local_18, 0x1d2);
    FUN_004dd030(local_18, 0, 0x14);
    piVar5 = local_18;
    iVar3 = 1;
    iVar8 = 0;
    uVar7 = DAT_0081bcd0;
    pcVar2 = FUN_004dcfa0(0x2d9);
    FUN_004daac0(0xc, pcVar2, piVar5, iVar8, iVar3, uVar7);
    FUN_004dd030(local_18, 0, 0x11);
    cStack_29 = (char)((DAT_0085e790 >> 1) - 2);
    FUN_004bc9e0(&cStack_29, local_18, 0x2da);
    return;
  }
  pcVar2 = FUN_004dcfa0(0x2d8);
  FUN_004daac0(0xc, pcVar2, piVar5, iVar8, iVar3, uVar7);
  FUN_004dd030(local_18, 0, 0x14);
  piVar5 = local_18;
  iVar3 = 1;
  iVar8 = 0;
  uVar7 = DAT_0081bcd0;
  pcVar2 = FUN_004dcfa0(0x1d1);
  FUN_004daac0(0xc, pcVar2, piVar5, iVar8, iVar3, uVar7);
  FUN_004dd030(local_18, 0, 0x11);
  FUN_004b9970(&DAT_0085e78f, local_18, 0x1d2);
  FUN_004dd030(local_18, 0, 0x14);
  piVar5 = local_18;
  iVar3 = 1;
  iVar8 = 0;
  uVar7 = DAT_0081bcd0;
  pcVar2 = FUN_004dcfa0(0x2d9);
  FUN_004daac0(0xc, pcVar2, piVar5, iVar8, iVar3, uVar7);
  FUN_004dd030(local_18, 0, 0x11);
  cStack_29 = (char)((DAT_0085e790 >> 1) - 2);
  FUN_004b9b40(&cStack_29, local_18, 0x2da);
  DAT_0085e790 = (char)(cStack_29 * 2 + 4);
  return;
}
