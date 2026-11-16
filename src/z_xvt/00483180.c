/* SPDX-License-Identifier: MIT */
/* z_xvt 00483180 */
extern int DAT_007329c4;
extern int DAT_007329c8;
extern int DAT_00732a14;
extern int DAT_00732580;
extern int DAT_00732560;
extern int DAT_00732564;
extern int DAT_00732550;
extern int DAT_00732540;
extern int DAT_00732544;
extern int DAT_0073255c;
extern int DAT_00732584;
extern int DAT_0073253c;
extern int DAT_00732558;
extern int DAT_00732578;
extern int DAT_00732538;
extern int DAT_00732568;
extern int DAT_00732530;
extern int DAT_00732548;

void __cdecl FUN_00483410(short, short, int);
void __cdecl FUN_00483540(short, short, int);
void __cdecl FUN_00483cf0(int, int, int, short);

void __cdecl FUN_00483180(short param_1, short param_2, short param_3, short param_4, short param_5, short param_6, int param_7)
{
  int iVar1;
  int iVar2;
  int iVar3;
  
  FUN_00483410(param_2, param_3, param_7);
  FUN_00483540(param_1, param_4, param_7);
  DAT_007329c4 = -DAT_007329c4;
  DAT_007329c8 = -DAT_007329c8;
  DAT_00732a14 = -DAT_00732a14;
  iVar2 = -DAT_00732580;
  iVar3 = -DAT_00732560;
  iVar1 = -DAT_00732564;
  DAT_00732560 = iVar3;
  DAT_00732564 = iVar1;
  DAT_00732580 = iVar2;
  FUN_00483cf0(DAT_00732550, DAT_00732540, DAT_00732544, param_5);
  FUN_00483cf0(iVar2, iVar3, iVar1, param_6);
  DAT_0073255c = DAT_00732550;
  DAT_00732584 = DAT_00732540;
  DAT_0073253c = DAT_00732544;
  DAT_00732558 = DAT_00732580;
  DAT_00732578 = DAT_00732560;
  DAT_00732538 = DAT_00732564;
  DAT_00732568 = DAT_007329c4;
  DAT_00732530 = DAT_007329c8;
  DAT_00732548 = DAT_00732a14;
  return;
}
