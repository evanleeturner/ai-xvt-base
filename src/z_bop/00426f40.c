/* SPDX-License-Identifier: MIT */
/* z_bop 00426f40 */
extern unsigned int DAT_00555cbc;
extern unsigned int DAT_009d682c;
extern unsigned int DAT_00555cb8;
extern unsigned int DAT_009a1fe4;
extern unsigned int DAT_00555cb4;
extern unsigned int DAT_009ed234;
extern unsigned int DAT_00555c8c;
extern unsigned int DAT_009a7b60;
extern unsigned int DAT_00555c90;
extern unsigned int DAT_00a004a8;
extern unsigned int DAT_00555c88;
extern unsigned int DAT_009a8e3c;
extern unsigned int DAT_00555c94;
extern unsigned int DAT_00a004a4;
extern unsigned int DAT_00555ca0;
extern unsigned int DAT_00a004b0;
extern unsigned int DAT_00555c98;
extern unsigned int DAT_00a004cc;
extern unsigned int DAT_00555c9c;
extern unsigned int DAT_00a004b8;
extern unsigned int DAT_00555cac;
extern unsigned int DAT_009fe7f4;
extern unsigned int DAT_00555ca4;
extern unsigned int DAT_00a00488;
extern unsigned int DAT_00555ca8;
extern unsigned int DAT_00a00480;
extern unsigned int DAT_00555cb0;
extern unsigned int DAT_00a00490;
extern unsigned int DAT_009cd272;
extern unsigned int DAT_00a080f4;
extern unsigned int DAT_009d1260;
extern unsigned int DAT_009a20a2;
extern unsigned int DAT_009d8c08;
extern unsigned int DAT_0052747c;
extern unsigned int DAT_005233d8;

void __cdecl FUN_00426f40(unsigned short param_1, unsigned short param_2, unsigned int param_3, unsigned int param_4)
{
  DAT_00555cbc = DAT_009d682c;
  DAT_00555cb8 = DAT_009a1fe4;
  DAT_00555cb4 = DAT_009ed234;
  DAT_00555c8c = DAT_009a7b60;
  DAT_00555c90 = DAT_00a004a8;
  DAT_00555c88 = DAT_009a8e3c;
  DAT_00555c94 = DAT_00a004a4;
  DAT_00555ca0 = DAT_00a004b0;
  DAT_00555c98 = DAT_00a004cc;
  DAT_00555c9c = DAT_00a004b8;
  DAT_00555cac = DAT_009fe7f4;
  DAT_00555ca4 = DAT_00a00488;
  DAT_00555ca8 = DAT_00a00480;
  DAT_00555cb0 = DAT_00a00490;
  DAT_009d682c = param_1;
  DAT_009cd272 = param_1 - 1;
  DAT_00a080f4 = param_1 >> 1;
  DAT_009a1fe4 = param_2;
  DAT_009d1260 = param_2 - 1;
  DAT_009a20a2 = param_2 >> 1;
  DAT_009ed234 = param_4;
  DAT_009a7b60 = param_4 / DAT_009d8c08;
  DAT_0052747c = 0xe000;
  DAT_009a8e3c = (unsigned int)(((unsigned long long)param_4 % (unsigned long long)DAT_009d8c08) / (unsigned long long)DAT_005233d8);
  return;
}
