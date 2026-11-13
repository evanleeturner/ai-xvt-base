/* SPDX-License-Identifier: MIT */
/* z_bop 0042e460 */
extern int DAT_00523410;
extern int DAT_009ecc34;
extern char DAT_009d80c9;
extern char DAT_00bb7372;
extern char DAT_00bb737b;

unsigned int __cdecl FUN_004a9300(int param_1, int param_2);
int __cdecl FUN_004a9380(int param_1);

void __cdecl FUN_0042e460(int param_1)
{
  unsigned int uVar1;
  
  if ((((DAT_00523410 == 0) && (param_1 == DAT_009ecc34)) && (DAT_009d80c9 != '\0')) &&
     ((DAT_00bb7372 != '\0' && (DAT_00bb737b != '\0')))) {
    uVar1 = FUN_004a9300(0x51, 0x100);
    if (uVar1 != 0) {
      FUN_004a9380(0x51);
    }
    uVar1 = FUN_004a9300(0x54, 0x100);
    if (uVar1 != 0) {
      FUN_004a9380(0x54);
    }
  }
  return;
}
