/* SPDX-License-Identifier: MIT */
/* z_bop 00427070 */
extern int DAT_00a004a8;
extern int DAT_00555c90;
extern int DAT_00a004a4;
extern int DAT_00555c94;
extern int DAT_00a004b0;
extern int DAT_00555ca0;
extern int DAT_00a004cc;
extern int DAT_00555c98;
extern int DAT_00a004b8;
extern int DAT_00555c9c;
extern int DAT_009fe7f4;
extern int DAT_00555cac;
extern int DAT_00a00488;
extern int DAT_00555ca4;
extern int DAT_00a00480;
extern int DAT_00555ca8;
extern unsigned short DAT_009d682c;
extern unsigned short DAT_00555cbc;
extern int DAT_00a00490;
extern int DAT_00555cb0;
extern unsigned short DAT_009cd272;
extern unsigned short DAT_00a080f4;
extern unsigned short DAT_009a1fe4;
extern unsigned short DAT_00555cb8;
extern unsigned short DAT_009d1260;
extern unsigned short DAT_009a20a2;
extern unsigned int _DAT_009ed234;
extern unsigned short DAT_00555cb4;
extern unsigned short DAT_0052747c;
extern unsigned int DAT_009a7b60;
extern unsigned short DAT_00555c8c;
extern unsigned int DAT_009a8e3c;
extern unsigned short DAT_00555c88;

void __stdcall FUN_00427070(void)
{
  DAT_00a004a8 = DAT_00555c90;
  DAT_00a004a4 = DAT_00555c94;
  DAT_00a004b0 = DAT_00555ca0;
  DAT_00a004cc = DAT_00555c98;
  DAT_00a004b8 = DAT_00555c9c;
  DAT_009fe7f4 = DAT_00555cac;
  DAT_00a00488 = DAT_00555ca4;
  DAT_00a00480 = DAT_00555ca8;
  DAT_009d682c = DAT_00555cbc;
  DAT_00a00490 = DAT_00555cb0;
  DAT_009cd272 = DAT_00555cbc - 1;
  DAT_00a080f4 = DAT_00555cbc >> 1;
  DAT_009a1fe4 = DAT_00555cb8;
  DAT_009d1260 = DAT_00555cb8 - 1;
  DAT_009a20a2 = DAT_00555cb8 >> 1;
  _DAT_009ed234 = (unsigned int)DAT_00555cb4;
  DAT_0052747c = 0xc000;
  DAT_009a7b60 = (unsigned int)DAT_00555c8c;
  DAT_009a8e3c = (unsigned int)DAT_00555c88;
  return;
}
