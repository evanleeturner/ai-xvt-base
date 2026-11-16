/* SPDX-License-Identifier: MIT */
/* z_xvt 00462410 */
extern int DAT_00525518;
extern unsigned char DAT_00732f44;

void __cdecl FUN_00462010(int, int, int, int);
void __cdecl FUN_004a68c0(int, int, int);

void __stdcall FUN_00462410(void)
{
  unsigned char local_300[768];
  
  FUN_00462010(0x6da540, (int)local_300, 0, 0x100);
  if (DAT_00525518 == 1) {
    FUN_004a68c0((int)local_300, 0, 0x100);
  }
  DAT_00732f44 = DAT_00732f44 & 0xfe;
  return;
}
