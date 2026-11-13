/* SPDX-License-Identifier: MIT */
/* z_xvt 004a1a70 */
extern int DAT_0052571c;
extern int DAT_0052552c;
extern int DAT_006da840;
extern int DAT_006d9d30;
extern int DAT_006db714;
extern int DAT_00703ad0;
extern int DAT_006da4e4;
extern int DAT_00703c78;
extern int DAT_006db710;
extern int DAT_0071e530;
extern int DAT_0071ecf8;
extern int DAT_00703c74;
extern int DAT_007092b4;
extern int DAT_006da9ec;
extern int DAT_006da538;
extern int DAT_006db594;
extern int DAT_00525518;
extern int DAT_00525508;
extern void (*DAT_00703c80)(void);
extern int DAT_00645b8c;
extern int DAT_0070a178;
extern int DAT_0070a014;
extern int _DAT_0070a006;
extern int DAT_006d9990;
extern int DAT_00645ae0;
extern int DAT_00645b38;

void __cdecl FUN_00461f20(int, int, int, int);
void __cdecl FUN_0048eda0(unsigned int);
void __cdecl FUN_0048ed00(short, short, short, short);
void __cdecl FUN_004a6a80(void);
void __cdecl FUN_0048ee00(int);
void __cdecl FUN_0048ee10(int);
void __cdecl FUN_0048ee20(int);
void __cdecl FUN_004a2e70(void);
void __cdecl FUN_0048ed70(unsigned int);
void __cdecl FUN_004a6f90(void);

void __cdecl FUN_004a1a70(int param_1)
{
  if (DAT_0052571c == -1) {
    if (DAT_0052552c == 0x13) {
      DAT_0052571c = 0x2a;
      DAT_006da840 = 0x115;
      DAT_006d9d30 = 0x2a;
      DAT_006db714 = 0x115;
      DAT_00703ad0 = 3;
      DAT_006da4e4 = 0x1a;
      DAT_00703c78 = 0x20;
      DAT_006db710 = 0x25;
      DAT_0071e530 = 0x4b;
      DAT_0071ecf8 = 0x2c;
      DAT_00703c74 = 0x109;
      DAT_007092b4 = 0x31;
    }
    else if (DAT_0052552c == 0x101) {
      DAT_00703ad0 = 6;
      DAT_006da840 = 0x1f9;
      DAT_006da4e4 = 0x35;
      DAT_00703c78 = 0x40;
      DAT_006db714 = 0x22b;
      DAT_006db710 = 0x4b;
      DAT_0071e530 = 0x96;
      DAT_0071ecf8 = 0x59;
      DAT_00703c74 = 0x212;
      DAT_007092b4 = 100;
      DAT_0052571c = 0x55;
      DAT_006d9d30 = 0x55;
    }
    else if (DAT_0052552c == 0x112) {
      DAT_00703ad0 = 4;
      DAT_006da840 = 0x17a;
      DAT_006da4e4 = 0x27;
      DAT_00703c78 = 0x30;
      DAT_006db714 = 0x19f;
      DAT_006db710 = 0x38;
      DAT_0071e530 = 0x70;
      DAT_0071ecf8 = 0x42;
      DAT_00703c74 = 0x18d;
      DAT_007092b4 = 0x4a;
      DAT_0052571c = 0x3f;
      DAT_006d9d30 = 0x3f;
    }
    FUN_00461f20(DAT_006da9ec, DAT_006da538, DAT_006db594, DAT_00525518 * DAT_006da538);
    FUN_0048eda0((unsigned int)DAT_00525508);
    FUN_0048ed00((short)DAT_0052571c, (short)DAT_00703ad0, (short)DAT_006da840, (short)DAT_006da4e4);
    DAT_00703c80();
    FUN_0048ed00((short)DAT_006d9d30, (short)DAT_00703c78, (short)DAT_006db714, (short)DAT_006db710);
    DAT_00703c80();
    FUN_0048ed00((short)DAT_0071e530, (short)DAT_0071ecf8, (short)DAT_00703c74, (short)DAT_007092b4);
    DAT_00703c80();
  }
  else if (param_1 != 0) {
    DAT_00645b8c = DAT_00645b8c + 1;
  }
  FUN_004a6a80();
  FUN_00461f20(0, 0x140, 200, 0);
  FUN_0048ee00(0);
  FUN_0048ee10(0);
  FUN_0048ee20(1);
  FUN_004a2e70();
  FUN_0048ed70(0x43);
  DAT_0070a178 = 0;
  DAT_0070a014 = 0x141;
  _DAT_0070a006 = DAT_006d9990;
  DAT_006d9990 = 0xffff;
  DAT_00645ae0 = 0xffff;
  DAT_00645b38 = 0xffff;
  FUN_004a6f90();
  return;
}
