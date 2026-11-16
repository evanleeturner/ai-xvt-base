/* SPDX-License-Identifier: MIT */
/* z_xvt 004bb3c0 */
extern int DAT_0064ef64;
extern int DAT_0081b850;
extern int DAT_0064ef60;
extern int DAT_0064ef58;
extern int DAT_0064ef5c;
extern int DAT_0085e6c0;

extern unsigned char DAT_0085e6d0, DAT_0085e6d1, DAT_0085e6d2, DAT_0085e6d3;
extern unsigned char DAT_0085e6d4, DAT_0085e6d5, DAT_0085e6d6, DAT_0085e6d7;
extern unsigned char DAT_0085e6d8, DAT_0085e6d9, DAT_0085e6da, DAT_0085e6db;
extern unsigned char DAT_0085e6dc, DAT_0085e6dd, DAT_0085e6de, DAT_0085e6df;
extern unsigned char DAT_0085e6e0, DAT_0085e6e1, DAT_0085e6e2, DAT_0085e6e3;
extern unsigned char DAT_0085e6e4, DAT_0085e6e5, DAT_0085e6e6, DAT_0085e6e7;
extern unsigned char DAT_0085e6e8, DAT_0085e6e9, DAT_0085e6ea, DAT_0085e6eb;
extern unsigned char DAT_0085e6ec, DAT_0085e6ed, DAT_0085e6ee, DAT_0085e6ef;
extern unsigned char DAT_0085e6f0;
extern unsigned char DAT_0085e78f, DAT_0085e790, DAT_0085e791, DAT_0085e792;
extern unsigned char DAT_0085e793, DAT_0085e794, DAT_0085e795, DAT_0085e796;
extern unsigned char DAT_0085e797, DAT_0085e798, DAT_0085e799, DAT_0085e79a;
extern unsigned char DAT_0085e79b, DAT_0085e79c, DAT_0085e79d, DAT_0085e79e;
extern char DAT_0085e79f;
extern unsigned char DAT_0085e7c5;
extern char DAT_0085e7a0;
extern int _DAT_0085e7a0;
extern int DAT_0085e7a4;
extern int DAT_0085e7a8;
extern int DAT_0085e7ac;
extern int DAT_0085e7b0;
extern char DAT_00859c40;

extern char s_jewelsound_00531990[];
extern char s_config6u_00531978[];
extern char s_config6d_00531984[];
extern char s_config5d_0053196c[];
extern char s_config5u_00531960[];
extern char s_config4d_00531954[];
extern char s_config4u_00531948[];
extern char s_config3d_0053193c[];
extern char s_config3u_00531930[];
extern char s_config2d_00531924[];
extern char s_config2u_00531918[];
extern char s_config1d_0053190c[];
extern char s_config1u_00531900[];
extern char s_backconfig_00531378[];
extern char s_frame_00531358[];
extern char s_alloff_00531350[];
extern char s_configoverlay_00531340[];

void FUN_004dcfd0(int *rect, int a, int b, int c, int d);
void FUN_004ddc70(int *a, int *b);
int FUN_004dd7d0(int *rect, int a, int b);
/* Full EAX is consumed by the caller (Ghidra's CONCAT31 with extraout_var), so these return int */
int FUN_004db530(void);
int FUN_004db550(void);
int FUN_004db570(void);
int FUN_004db590(void);
void FUN_004d9ea0(int *a);
char *FUN_004dcfa0(int id);
int FUN_004d9a80(int *rect, char *s1, char *s2, char *s3, int a, unsigned int b, int c, char *s4);
int FUN_004dbf00(void);
void FUN_004d0070(int a);
void FUN_004cfbc0(int a, int b, int c);
int FUN_004df3a0(int a);
void FUN_004d9d20(int *rect, char *s1, char *s2, int a);
void FUN_004d52b0(void);
void FUN_004d1750(char *s, int a, int b);
void FUN_004d0e60(char *s, int a, int b);
void FUN_004d0580(char *s, int a, int b);
unsigned int FUN_004df760(int a, int b, int c);
void FUN_004dd130(int *rect, int a, int b, unsigned int c);
void FUN_004d5350(int a);
void FUN_004dd030(int *rect, int a, int b);
void FUN_004dbea0(void);

unsigned int __stdcall FUN_004bb3c0(void)
{
  char cVar1;
  int uVar2;
  int iVar3;
  char *pcVar4;
  unsigned int uVar5;
  unsigned int uVar6;
  int iVar7;
  char *pcVar8;
  int local_34[4];
  int local_24[8];
  int local_4;

  local_24[0] = 1;
  local_24[1] = 1;
  local_24[2] = 1;
  local_24[3] = 1;
  local_24[4] = 1;
  if (DAT_0064ef64 == 0) {
    if ((DAT_0081b850 == 4) || (DAT_0081b850 == 3)) {
      local_24[5] = 0;
    }
    else {
      local_24[5] = 1;
    }
  }
  else {
    local_24[5] = 1;
  }
  local_24[6] = 0;
  local_24[DAT_0064ef64] = 2;
  FUN_004dcfd0(local_34, 0x16, 0x132, 0x2a, 0x14a);
  FUN_004ddc70(&local_4, local_24 + 7);
  if (((local_24[5] != 0) && (iVar3 = FUN_004dd7d0(local_34, local_4, local_24[7]), iVar3 != 0)) &&
      ((FUN_004db530() != 0 ||
        ((FUN_004db550() != 0 ||
          FUN_004db570() != 0) ||
         (FUN_004db590() != 0))))) {
    local_24[5] = 2;
  }
  FUN_004d9ea0(local_24);
  if (local_24[5] != 0) {
    FUN_004dcfd0(local_34, 0x16, 0x132, 0x2a, 0x14a);
    pcVar8 = s_jewelsound_00531990;
    iVar7 = 0x10;
    uVar6 = 0;
    iVar3 = 0xc;
    pcVar4 = FUN_004dcfa0(0x197);
    iVar3 = FUN_004d9a80(local_34, s_config6u_00531978, s_config6d_00531984, pcVar4, iVar3, uVar6, iVar7,
                         pcVar8);
    if (iVar3 != 0) {
      FUN_004dcfa0(0x13);
      FUN_004dcfa0(0x1fd);
      FUN_004dcfa0(0x294);
      FUN_004dcfa0(0x293);
      FUN_004dcfa0(0x292);
      iVar3 = FUN_004dbf00();
      cVar1 = DAT_0085e79f;
      if (iVar3 != 0) {
        switch (DAT_0064ef64) {
        case 0:
          DAT_0085e790 = 8;
          DAT_0085e6f0 = 0;
          DAT_0085e78f = 0;
          break;
        case 1:
          DAT_0085e6d0 = 1;
          DAT_0085e6d2 = 2;
          DAT_0085e6d4 = 1;
          DAT_0085e6d6 = 1;
          DAT_0085e6d8 = 1;
          DAT_0085e6da = 1;
          DAT_0085e6dc = 1;
          DAT_0085e6de = 2;
          DAT_0085e6e0 = 10;
          DAT_0085e6e2 = 10;
          DAT_0085e6e4 = 2;
          DAT_0085e6e6 = 2;
          DAT_0085e6ea = 2;
          DAT_0085e6ec = 1;
          DAT_0085e6ee = 1;
          DAT_0085e6e8 = 0;
          break;
        case 2:
          DAT_0085e6d1 = 1;
          DAT_0085e6d3 = 2;
          DAT_0085e6d5 = 1;
          DAT_0085e6d7 = 1;
          DAT_0085e6d9 = 1;
          DAT_0085e6db = 1;
          DAT_0085e6dd = 1;
          DAT_0085e6df = 2;
          DAT_0085e6e1 = 10;
          DAT_0085e6e3 = 10;
          DAT_0085e6e5 = 2;
          DAT_0085e6e7 = 2;
          DAT_0085e6eb = 2;
          DAT_0085e6ed = 1;
          DAT_0085e6ef = 1;
          DAT_0085e6e9 = 0;
          break;
        case 3:
          DAT_0085e791 = 1;
          DAT_0085e792 = 1;
          DAT_0085e793 = 1;
          DAT_0085e7c5 = 1;
          DAT_0085e794 = 2;
          DAT_0085e795 = 2;
          DAT_0085e796 = 1;
          DAT_0085e797 = 1;
          DAT_0085e798 = 1;
          DAT_0085e79f = '\x01';
          DAT_0085e799 = 9;
          DAT_0085e79a = 9;
          DAT_0085e79b = 9;
          DAT_0085e79c = 9;
          DAT_0085e79d = 9;
          DAT_0085e79e = 5;
          if (cVar1 != '\x01') {
            FUN_004d0070(0x8e38);
            FUN_004cfbc0(7, 0, 0);
          }
          break;
        case 4:
          iVar3 = 0;
          _DAT_0085e7a0 = 0;
          DAT_0085e7a4 = 0;
          DAT_0085e7a8 = 0;
          DAT_0085e7ac = 0;
          DAT_0085e7b0 = 0;
          uVar2 = FUN_004df3a0(0);
          if (uVar2 != 0 && -1 < uVar2) {
            do {
              if (0xf < iVar3) break;
              switch (iVar3) {
              case 0:
                (&DAT_0085e7a0)[iVar3] = (char)0x98;
                break;
              case 1:
                (&DAT_0085e7a0)[iVar3] = (char)0x99;
                break;
              case 2:
                (&DAT_0085e7a0)[iVar3] = 0x72;
                break;
              case 3:
                (&DAT_0085e7a0)[iVar3] = 0x2e;
                break;
              case 4:
                (&DAT_0085e7a0)[iVar3] = 0x65;
                break;
              case 5:
                (&DAT_0085e7a0)[iVar3] = 0x69;
                break;
              case 6:
                (&DAT_0085e7a0)[iVar3] = 0x5b;
                break;
              case 7:
                (&DAT_0085e7a0)[iVar3] = 8;
                break;
              case 8:
                (&DAT_0085e7a0)[iVar3] = 0xd;
                break;
              case 9:
                (&DAT_0085e7a0)[iVar3] = 0x5d;
              }
              iVar3 = iVar3 + 1;
            } while (iVar3 < uVar2);
          }
          iVar3 = 0;
          if (0 < DAT_0085e6c0) {
            pcVar4 = &DAT_00859c40;
            do {
              if (*pcVar4 == (&DAT_0085e7a0)[DAT_0064ef58]) {
                DAT_0064ef5c = iVar3;
              }
              pcVar4 = pcVar4 + 0x95;
              iVar3 = iVar3 + 1;
            } while (iVar3 < DAT_0085e6c0);
          }
        }
      }
    }
  }
  FUN_004dcfd0(local_34, 0x16, 0xe2, 0x2a, 0xfa);
  if (DAT_0064ef64 == 4) {
    iVar3 = 0xc;
    pcVar4 = FUN_004dcfa0(0x192);
    FUN_004d9d20(local_34, s_config5d_0053196c, pcVar4, iVar3);
  }
  else {
    pcVar8 = s_jewelsound_00531990;
    iVar7 = 0xf;
    uVar6 = 0;
    iVar3 = 0xc;
    pcVar4 = FUN_004dcfa0(0x192);
    iVar3 = FUN_004d9a80(local_34, s_config5u_00531960, s_config5u_00531960, pcVar4, iVar3, uVar6, iVar7,
                         pcVar8);
    if (iVar3 != 0) {
      DAT_0064ef60 = 1;
      DAT_0064ef64 = 4;
      FUN_004d52b0();
      FUN_004d1750(s_backconfig_00531378, 0, 0);
      FUN_004d0e60(s_frame_00531358, 0, 0);
      FUN_004d0e60(s_alloff_00531350, 0, 0);
      FUN_004dcfd0(local_34, 0x54, 0x6b, 0x25c, 0x1b1);
      FUN_004d0580(s_configoverlay_00531340, 0, 0);
      FUN_004dcfd0(local_34, 0x14a, 0x6f, 0x24e, 0x19f);
      uVar5 = FUN_004df760(0, 0, 0x40);
      FUN_004dd130(local_34, 0, 0, uVar5);
      FUN_004d5350(1);
    }
  }
  FUN_004dd030(local_34, 0, -0x1c);
  if (DAT_0064ef64 == 3) {
    iVar3 = 0xc;
    pcVar4 = FUN_004dcfa0(0x129);
    FUN_004d9d20(local_34, s_config4d_00531954, pcVar4, iVar3);
  }
  else {
    pcVar8 = s_jewelsound_00531990;
    iVar7 = 0xe;
    uVar6 = 0;
    iVar3 = 0xc;
    pcVar4 = FUN_004dcfa0(0x129);
    iVar3 = FUN_004d9a80(local_34, s_config4u_00531948, s_config4u_00531948, pcVar4, iVar3, uVar6, iVar7,
                         pcVar8);
    if (iVar3 != 0) {
      DAT_0064ef60 = 1;
      DAT_0064ef64 = 3;
      FUN_004d52b0();
      FUN_004d1750(s_backconfig_00531378, 0, 0);
      FUN_004d0e60(s_frame_00531358, 0, 0);
      FUN_004d0e60(s_alloff_00531350, 0, 0);
      FUN_004dcfd0(local_34, 0x54, 0x6b, 0x25c, 0x1b1);
      FUN_004d0580(s_configoverlay_00531340, 0, 0);
      FUN_004d5350(1);
    }
  }
  FUN_004dd030(local_34, 0, -0x1c);
  if (DAT_0064ef64 == 2) {
    iVar3 = 0xc;
    pcVar4 = FUN_004dcfa0(0x273);
    FUN_004d9d20(local_34, s_config3d_0053193c, pcVar4, iVar3);
  }
  else {
    pcVar8 = s_jewelsound_00531990;
    iVar7 = 0xd;
    uVar6 = 0;
    iVar3 = 0xc;
    pcVar4 = FUN_004dcfa0(0x273);
    iVar3 = FUN_004d9a80(local_34, s_config3u_00531930, s_config3u_00531930, pcVar4, iVar3, uVar6, iVar7,
                         pcVar8);
    if (iVar3 != 0) {
      DAT_0064ef60 = 1;
      DAT_0064ef64 = 2;
      FUN_004d52b0();
      FUN_004d1750(s_backconfig_00531378, 0, 0);
      FUN_004d0e60(s_frame_00531358, 0, 0);
      FUN_004d0e60(s_alloff_00531350, 0, 0);
      FUN_004dcfd0(local_34, 0x54, 0x6b, 0x25c, 0x1b1);
      FUN_004d0580(s_configoverlay_00531340, 0, 0);
      FUN_004d5350(1);
    }
  }
  FUN_004dd030(local_34, 0, -0x1c);
  if (DAT_0064ef64 == 1) {
    iVar3 = 0xc;
    pcVar4 = FUN_004dcfa0(0xce);
    FUN_004d9d20(local_34, s_config2d_00531924, pcVar4, iVar3);
  }
  else {
    pcVar8 = s_jewelsound_00531990;
    iVar7 = 0xc;
    uVar6 = 0;
    iVar3 = 0xc;
    pcVar4 = FUN_004dcfa0(0xce);
    iVar3 = FUN_004d9a80(local_34, s_config2u_00531918, s_config2u_00531918, pcVar4, iVar3, uVar6, iVar7,
                         pcVar8);
    if (iVar3 != 0) {
      DAT_0064ef60 = 1;
      DAT_0064ef64 = 1;
      FUN_004d52b0();
      FUN_004d1750(s_backconfig_00531378, 0, 0);
      FUN_004d0e60(s_frame_00531358, 0, 0);
      FUN_004d0e60(s_alloff_00531350, 0, 0);
      FUN_004dcfd0(local_34, 0x54, 0x6b, 0x25c, 0x1b1);
      FUN_004d0580(s_configoverlay_00531340, 0, 0);
      FUN_004d5350(1);
    }
  }
  FUN_004dd030(local_34, 0, -0x1c);
  if (DAT_0064ef64 != 0) {
    pcVar8 = s_jewelsound_00531990;
    iVar7 = 0xb;
    uVar6 = 0;
    iVar3 = 0xc;
    pcVar4 = FUN_004dcfa0(0x123);
    iVar3 = FUN_004d9a80(local_34, s_config1u_00531900, s_config1u_00531900, pcVar4, iVar3, uVar6, iVar7,
                         pcVar8);
    if (iVar3 != 0) {
      FUN_004dbea0();
      DAT_0064ef60 = 1;
      DAT_0064ef64 = 0;
      FUN_004d52b0();
      FUN_004d1750(s_backconfig_00531378, 0, 0);
      FUN_004d0e60(s_frame_00531358, 0, 0);
      FUN_004d0e60(s_alloff_00531350, 0, 0);
      FUN_004dcfd0(local_34, 0x54, 0x6b, 0x25c, 0x1b1);
      FUN_004d0580(s_configoverlay_00531340, 0, 0);
      FUN_004d5350(1);
    }
    return 1;
  }
  iVar3 = 0xc;
  pcVar4 = FUN_004dcfa0(0x123);
  FUN_004d9d20(local_34, s_config1d_0053190c, pcVar4, iVar3);
  return 1;
}
