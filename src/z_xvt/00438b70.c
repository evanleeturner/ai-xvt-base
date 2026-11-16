/* SPDX-License-Identifier: MIT */
/* z_xvt 00438b70 */
void __cdecl FUN_0048ee20(int);
void __stdcall FUN_004a7040(void);
void __stdcall FUN_004a6a80(void);
void __stdcall FUN_004a6f90(void);

extern unsigned int DAT_00525520;
extern unsigned int DAT_0052551c;
extern unsigned char DAT_006d4a4e;
extern unsigned int DAT_00525528;
extern unsigned int DAT_00525524;
extern unsigned int DAT_005257e0;
extern unsigned int DAT_0052ae54;
extern void (__cdecl *DAT_00732a30)(unsigned int, unsigned int, unsigned int, unsigned int, unsigned int);

void __stdcall FUN_00438b70(void)
{
  unsigned int uVar1;
  unsigned int uVar2;
  unsigned int uVar3;
  unsigned int uVar4;
  
  FUN_0048ee20(0);
  uVar4 = (DAT_00525520 - DAT_0052551c) >> 1;
  uVar2 = (unsigned int)DAT_006d4a4e;
  uVar1 = DAT_0052551c + DAT_00525520;
  uVar3 = DAT_00525528 + DAT_00525524;
  if (DAT_005257e0 != 0) {
    FUN_004a7040();
    DAT_0052ae54 = 0;
    FUN_004a6a80();
    (*DAT_00732a30)(DAT_005257e0,
                    ((uVar1 >> 1) - uVar4 / 2) - 1 & 0xffff,
                    ((uVar3 >> 1) - (uVar2 * 5) / 2) - 1 & 0xffff,
                    uVar4 + 2 & 0xffff,
                    uVar2 * 5 + 2);
    FUN_004a6f90();
    DAT_0052ae54 = 1;
    FUN_004a7040();
  }
  return;
}
