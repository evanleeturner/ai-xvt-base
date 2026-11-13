/* SPDX-License-Identifier: MIT */
/* z_bop 004226a0 */
extern unsigned char DAT_009fe7d4[];

void __stdcall FUN_00422780(void);
void __stdcall FUN_00422b10(void);
void __stdcall FUN_00422e90(void);
void __stdcall FUN_00423200(void);
void __stdcall FUN_004235d0(void);
void __stdcall FUN_00423900(void);
void __stdcall FUN_00423cd0(void);
void __stdcall FUN_00424060(void);

void __stdcall FUN_004226a0(void)
{
  switch (*(unsigned short *)(DAT_009fe7d4 + 0x16)) {
    case 0:
      FUN_00422780();
      return;
    case 1:
      FUN_00422b10();
      return;
    case 2:
      FUN_00422e90();
      return;
    case 3:
      FUN_00423200();
      return;
    case 4:
      FUN_004235d0();
      return;
    case 5:
      FUN_00423900();
      return;
    case 6:
      FUN_00423cd0();
      return;
    case 7:
      FUN_00424060();
      return;
    default:
      return;
  }
}
