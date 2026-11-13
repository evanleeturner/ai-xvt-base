/* SPDX-License-Identifier: MIT */
/* z_bop 0042cc40 */
extern int DAT_005233ec;
extern int DAT_009ec464;
extern int DAT_009a7bac;
extern int DAT_009a8c14;
extern unsigned short DAT_009d8b74;
extern void (*DAT_009a6fe4)(int);
extern unsigned short DAT_009d6952;
extern int DAT_009ecc34;

void __cdecl FUN_004a95c0(unsigned int param_1);
void __cdecl FUN_004a9590(unsigned int param_1);
void __cdecl FUN_004a9630(unsigned short param_1);
void __cdecl FUN_004a9640(unsigned char param_1);
void __cdecl FUN_004a9520(int param_1, short param_2, short param_3, short param_4);
void __cdecl FUN_004a9500(unsigned short param_1, short param_2);
void __cdecl FUN_004277f0(unsigned short param_1, unsigned int param_2, unsigned int param_3);
void __stdcall FUN_004aba50(void);
void __cdecl FUN_00438df0(int param_1);
void __stdcall FUN_004abe50(void);

void __stdcall FUN_0042cc40(void)
{
  unsigned short uVar1;
  unsigned short uVar2;
  short sVar3;
  
  if (DAT_005233ec == 0x13) {
    sVar3 = 0xbe;
    uVar1 = 200;
    uVar2 = 0xff;
  }
  else if (DAT_005233ec == 0x101) {
    sVar3 = 0x1c8;
    uVar1 = 0x168;
    uVar2 = 0x1d1;
  }
  else {
    sVar3 = 0xbe;
    uVar1 = 200;
    uVar2 = 0xff;
  }
  DAT_009ec464 = 1;
  FUN_004a95c0(0x2c);
  FUN_004a9590(0x43);
  FUN_004a9630(0);
  FUN_004a9640('\x01');
  FUN_004a9520(0, sVar3, (short)DAT_009a7bac, (short)DAT_009a8c14);
  FUN_004a9500(uVar1, sVar3);
  FUN_004277f0((unsigned short)(unsigned char)DAT_009d8b74, 2, 2);
  (*DAT_009a6fe4)(0x3a);
  FUN_004277f0((unsigned short)(DAT_009d8b74 >> 8), 2, 2);
  FUN_004a9500(uVar2, sVar3);
  FUN_004277f0(DAT_009d6952, 5, 5);
  FUN_004aba50();
  FUN_00438df0(DAT_009ecc34);
  FUN_004abe50();
  return;
}
