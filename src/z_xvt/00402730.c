/* SPDX-License-Identifier: MIT */
/* z_xvt 00402730 */
void __cdecl FUN_0048eda0(unsigned int);
void __cdecl FUN_0048ed70(unsigned int);
void __cdecl FUN_0048ee10(int);
void __cdecl FUN_0048ee20(int);
void __cdecl FUN_0048ed00(int, short, short, short);
void __cdecl FUN_0048ece0(unsigned short, short);
void __cdecl FUN_0040ccc0(unsigned short, int, int);
void __cdecl FUN_004a6a80(void);
void __cdecl FUN_004a80d0(int);
void __cdecl FUN_004a6f90(void);

extern int DAT_0052552c;
extern char DAT_0071e524;
extern int DAT_006da538;
extern int DAT_006db594;
extern unsigned char DAT_0070ac34;
extern void (*DAT_006d9984)(unsigned int);
extern unsigned char DAT_0070ac35;
extern unsigned short DAT_007092d2;
extern int DAT_0071ecf4;

void __stdcall FUN_00402730(void)
{
  unsigned short uVar1;
  unsigned short uVar2;
  short sVar3;
  
  if (DAT_0052552c == 0x13) {
    sVar3 = 0xbe;
    uVar1 = 200;
    uVar2 = 0xff;
  }
  else if (DAT_0052552c == 0x101) {
    sVar3 = 0x1c8;
    uVar1 = 0x168;
    uVar2 = 0x1d1;
  }
  else {
    sVar3 = 0xbe;
    uVar1 = 200;
    uVar2 = 0xff;
  }
  DAT_0071e524 = 1;
  FUN_0048eda0(0x2c);
  FUN_0048ed70(0x43);
  FUN_0048ee10(0);
  FUN_0048ee20(1);
  FUN_0048ed00(0, sVar3, (short)DAT_006da538, (short)DAT_006db594);
  FUN_0048ece0(uVar1, sVar3);
  FUN_0040ccc0((unsigned short)DAT_0070ac34, 2, 2);
  (*DAT_006d9984)(0x3a);
  FUN_0040ccc0((unsigned short)DAT_0070ac35, 2, 2);
  FUN_0048ece0(uVar2, sVar3);
  FUN_0040ccc0(DAT_007092d2, 5, 5);
  FUN_004a6a80();
  FUN_004a80d0(DAT_0071ecf4);
  FUN_004a6f90();
  return;
}
