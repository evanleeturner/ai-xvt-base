/* SPDX-License-Identifier: MIT */
/* z_bop 00427940 */
extern int DAT_00a07c68;
extern int DAT_00a07c6c;
extern int DAT_00a07cc4;
extern int DAT_00a004d0;
extern int DAT_00a004ac;
extern int DAT_00a004b4;
extern int DAT_00a0049c;
extern int DAT_00a0048c;
extern int DAT_00a00494;
extern int DAT_00a004a8;
extern int DAT_00a004cc;
extern int DAT_00a00488;
extern int DAT_00a004a4;
extern int DAT_00a004b8;
extern int DAT_00a00480;
extern int DAT_00a004b0;
extern int DAT_009fe7f4;
extern int DAT_00a00490;

void __cdecl FUN_00427bd0(short, short, int);
void __cdecl FUN_00427d30(short, short, int);
void __cdecl FUN_004290b0(int, int, int, short);

void __cdecl FUN_00427940(short param_1, short param_2, short param_3, short param_4, short param_5, short param_6, int param_7)
{
  int iVar1;
  int iVar2;
  int iVar3;
  
  FUN_00427bd0(param_2, param_3, param_7);
  FUN_00427d30(param_1, param_4, param_7);
  DAT_00a07c68 = -DAT_00a07c68;
  DAT_00a07c6c = -DAT_00a07c6c;
  DAT_00a07cc4 = -DAT_00a07cc4;
  iVar1 = -DAT_00a004d0;
  iVar2 = -DAT_00a004ac;
  iVar3 = -DAT_00a004b4;
  DAT_00a004ac = iVar2;
  DAT_00a004b4 = iVar3;
  DAT_00a004d0 = iVar1;
  FUN_004290b0(DAT_00a0049c, DAT_00a0048c, DAT_00a00494, param_5);
  FUN_004290b0(iVar1, iVar2, iVar3, param_6);
  DAT_00a004a8 = DAT_00a0049c;
  DAT_00a004cc = DAT_00a0048c;
  DAT_00a00488 = DAT_00a00494;
  DAT_00a004a4 = DAT_00a004d0;
  DAT_00a004b8 = DAT_00a004ac;
  DAT_00a00480 = DAT_00a004b4;
  DAT_00a004b0 = DAT_00a07c68;
  DAT_009fe7f4 = DAT_00a07c6c;
  DAT_00a00490 = DAT_00a07cc4;
  return;
}
