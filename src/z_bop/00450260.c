/* SPDX-License-Identifier: MIT */
/* z_bop 00450260 */
void __cdecl FUN_0040e0c0(int param_1, int param_2, unsigned int param_3, int param_4);
void __cdecl FUN_004a95c0(unsigned int param_1);
void __cdecl FUN_004a9520(short param_1, short param_2, short param_3, short param_4);
void __cdecl FUN_004aba50(void);
void __cdecl FUN_004a9620(void *param_1);
void __cdecl FUN_004a9630(unsigned short param_1);
void __cdecl FUN_004a9640(unsigned char param_1);
void __cdecl FUN_004515d0(void);
void __cdecl FUN_004a9590(unsigned int param_1);
void __stdcall FUN_004abe50(void);

int DAT_005235dc;
int DAT_005233ec;
int DAT_009a7ec0;
int DAT_009a7390;
int DAT_009a8d98;
int DAT_009d1148;
int DAT_009a7b44;
int DAT_009d12f8;
int DAT_009a8d94;
int DAT_009ec470;
int DAT_009ecc38;
int DAT_009d12f4;
int DAT_009d6934;
int DAT_009a806c;
int DAT_009a7bac;
int DAT_009a8c14;
int DAT_005233d8;
int DAT_005233c8;
void (*DAT_009d1300)(void);
int DAT_00556aa4;
int DAT_009ecc3c;
int DAT_009d77f8;
int DAT_009d7694;
unsigned short _DAT_009d7686;
int DAT_009a6ff0;
int DAT_005569f8;
int DAT_00556a50;

void __cdecl FUN_00450260(int param_1)
{
  if (DAT_005235dc == -1) {
    if (DAT_005233ec == 0x13) {
      DAT_005235dc = 0x2a;
      DAT_009a7ec0 = 0x115;
      DAT_009a7390 = 0x2a;
      DAT_009a8d98 = 0x115;
      DAT_009d1148 = 3;
      DAT_009a7b44 = 0x1a;
      DAT_009d12f8 = 0x20;
      DAT_009a8d94 = 0x25;
      DAT_009ec470 = 0x4b;
      DAT_009ecc38 = 0x2c;
      DAT_009d12f4 = 0x109;
      DAT_009d6934 = 0x31;
    }
    else if (DAT_005233ec == 0x101) {
      DAT_009d1148 = 6;
      DAT_009a7ec0 = 0x1f9;
      DAT_009a7b44 = 0x35;
      DAT_009d12f8 = 0x40;
      DAT_009a8d98 = 0x22b;
      DAT_009a8d94 = 0x4b;
      DAT_009ec470 = 0x96;
      DAT_009ecc38 = 0x59;
      DAT_009d12f4 = 0x212;
      DAT_009d6934 = 100;
      DAT_005235dc = 0x55;
      DAT_009a7390 = 0x55;
    }
    else if (DAT_005233ec == 0x112) {
      DAT_009d1148 = 4;
      DAT_009a7ec0 = 0x17a;
      DAT_009a7b44 = 0x27;
      DAT_009d12f8 = 0x30;
      DAT_009a8d98 = 0x19f;
      DAT_009a8d94 = 0x38;
      DAT_009ec470 = 0x70;
      DAT_009ecc38 = 0x42;
      DAT_009d12f4 = 0x18d;
      DAT_009d6934 = 0x4a;
      DAT_005235dc = 0x3f;
      DAT_009a7390 = 0x3f;
    }
    FUN_0040e0c0(DAT_009a806c, DAT_009a7bac, DAT_009a8c14, DAT_005233d8 * DAT_009a7bac);
    FUN_004a95c0((unsigned int)DAT_005233c8);
    FUN_004a9520((short)DAT_005235dc, (short)DAT_009d1148, (short)DAT_009a7ec0, (short)DAT_009a7b44);
    (*DAT_009d1300)();
    FUN_004a9520((short)DAT_009a7390, (short)DAT_009d12f8, (short)DAT_009a8d98, (short)DAT_009a8d94);
    (*DAT_009d1300)();
    FUN_004a9520((short)DAT_009ec470, (short)DAT_009ecc38, (short)DAT_009d12f4, (short)DAT_009d6934);
    (*DAT_009d1300)();
  }
  else if (param_1 != 0) {
    DAT_00556aa4 = DAT_00556aa4 + 1;
  }
  FUN_004aba50();
  FUN_0040e0c0(0, 0x140, 200, 0);
  FUN_004a9620(0);
  FUN_004a9630(0);
  FUN_004a9640(1);
  FUN_004515d0();
  FUN_004a9590(0x43);
  DAT_009ecc3c = 0;
  DAT_009d77f8 = 0;
  DAT_009d7694 = 0x14b;
  {
    unsigned short tmp = (unsigned short)DAT_009a6ff0;
    __builtin_memcpy(&_DAT_009d7686, &tmp, 2);
  }
  {
    unsigned short tmp = 0xffff;
    __builtin_memcpy(&DAT_009a6ff0, &tmp, 2);
  }
  {
    unsigned short tmp = 0xffff;
    __builtin_memcpy(&DAT_005569f8, &tmp, 2);
  }
  {
    unsigned short tmp = 0xffff;
    __builtin_memcpy(&DAT_00556a50, &tmp, 2);
  }
  FUN_004abe50();
  return;
}
