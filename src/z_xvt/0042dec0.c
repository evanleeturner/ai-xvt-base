/* SPDX-License-Identifier: MIT */
/* z_xvt 0042dec0 */
extern int DAT_005373d4;
extern int DAT_00525560;
extern int DAT_005373d8;
extern int DAT_005373d0;
extern int DAT_005373dc;
extern int DAT_006db5a8;
extern int DAT_005250d8;
extern int DAT_00536f70;
extern int DAT_00732a18;
extern int DAT_0071f2ec;

void __cdecl FUN_00431110(int param_1, unsigned int *param_2, unsigned int param_3);
int __cdecl FUN_0048f530(void);
int __stdcall FUN_004a53f0(void);
unsigned int __stdcall FUN_004a5470(void);
void __cdecl FUN_0042c710(void);
void __cdecl FUN_004310c0(unsigned int *param_1, unsigned int param_2);
int __cdecl FUN_00432c10(int param_1);

void __cdecl FUN_0042dec0(int param_1, unsigned int param_2)
{
  int iVar1;
  int iVar2;
  unsigned int uVar3;
  int iVar4;
  int iVar5;
  
  DAT_005373d4 = DAT_00525560;
  DAT_005373d8 = param_2;
  iVar5 = 10;
  iVar4 = 0;
  DAT_005373d0 = 0x3e;
  DAT_005373dc = DAT_006db5a8;
  FUN_00431110(param_1, (unsigned int *)&DAT_005373d0, 0x10);
  FUN_0048f530();
  do {
    iVar1 = DAT_006db5a8;
    DAT_005250d8 = 1;
    do {
      if (0xeb < (unsigned int)(DAT_006db5a8 - iVar1)) break;
      iVar2 = FUN_004a53f0();
      if ((iVar2 != 0) && (uVar3 = FUN_004a5470(), uVar3 == 0x1b)) {
        iVar5 = 1;
        DAT_006db5a8 = DAT_006db5a8 + 0xec;
        break;
      }
      FUN_0042c710();
      iVar2 = FUN_0048f530();
      DAT_006db5a8 = DAT_006db5a8 + iVar2;
      if (DAT_00536f70 != 0) {
        DAT_00536f70 = 0;
        DAT_006db5a8 = iVar1;
      }
    } while (DAT_005250d8 != 0);
    iVar4 = (iVar4 - iVar1) + DAT_006db5a8;
    if (0xeb < iVar4) {
      iVar4 = 0;
      DAT_005373d0 = 0x1d;
      FUN_004310c0((unsigned int *)&DAT_005373d0, 4);
    }
    if ((DAT_005250d8 == 0) || (iVar5 = iVar5 - 1, iVar5 == 0)) {
      if (DAT_005250d8 == 1) {
        DAT_005373d4 = FUN_00432c10(param_1);
        DAT_005373d0 = 0x15;
        FUN_004310c0((unsigned int *)&DAT_005373d0, 8);
        FUN_0048f530();
        DAT_005250d8 = 0;
        DAT_006db5a8 = DAT_00732a18 + DAT_0071f2ec;
      }
      else {
        FUN_0048f530();
        DAT_006db5a8 = DAT_00732a18 + DAT_0071f2ec;
      }
      DAT_005373d0 = 0x17;
      DAT_005373d4 = 0;
      FUN_004310c0((unsigned int *)&DAT_005373d0, 8);
      return;
    }
  } while (1);
}
