/* SPDX-License-Identifier: MIT */
/* z_xvt 00412e60 */
extern int DAT_006d4988;
extern unsigned short DAT_0071f714;
extern int DAT_00526872;
extern int DAT_006db7b4;
extern int DAT_0070a134;
extern int DAT_0070a00c;
extern unsigned int DAT_0070a130;
extern unsigned int DAT_0070a010;
extern unsigned int DAT_0070a12c;
extern unsigned int DAT_0070a138;
extern int DAT_006db7ac;
extern int DAT_006db7b0;
extern unsigned short DAT_006d4984;

void FUN_0049c2b0(unsigned int);
void FUN_00486310(unsigned int);
unsigned int FUN_00419100(unsigned int, unsigned int);
unsigned int FUN_0040ca90(int, int);
unsigned int FUN_0040cad0(int, int);
void FUN_0047f360(unsigned short, unsigned short, unsigned int, short, short, int, short);

void __thiscall FUN_00412e60(void *this, unsigned short param_1)
{
  unsigned short uVar1;
  short sVar2;
  unsigned int uVar3;
  unsigned int uVar4;
  unsigned int uVar5;
  int iVar6;
  int iVar7;
  
  iVar7 = (unsigned int)param_1 * 0x23 + DAT_006d4988;
  DAT_0071f714 = param_1;
  if (*(int *)(&DAT_00526872 + (unsigned int)*(unsigned char *)(iVar7 + 3) * 0x16) == 0) {
    if (*(char *)(iVar7 + 0x19) == '\0') {
      FUN_0049c2b0(((unsigned int)this & 0xFFFF0000) | param_1);
      FUN_00486310((unsigned int)DAT_0071f714 * 0x23 + DAT_006d4988);
      return;
    }
  }
  else {
    uVar1 = *(unsigned short *)
             (*(int *)(&DAT_00526872 + (unsigned int)*(unsigned char *)(iVar7 + 3) * 0x16) +
             (unsigned int)*(unsigned char *)(iVar7 + 0x19) * 2);
    if (uVar1 < 0xff00) {
      if (uVar1 < 0x8000) {
        FUN_00486310(iVar7);
        return;
      }
      if (-1 < (int)DAT_006db7b4) {
        iVar7 = DAT_0070a134;
        if (DAT_0070a134 < 0) {
          iVar7 = -DAT_0070a134;
        }
        iVar6 = DAT_0070a00c;
        if (DAT_0070a00c < 0) {
          iVar6 = -DAT_0070a00c;
        }
        uVar4 = DAT_0070a130;
        uVar5 = DAT_0070a010;
        if (iVar7 < iVar6) {
          uVar4 = DAT_0070a12c;
          uVar5 = DAT_0070a138;
        }
        if ((int)uVar4 < 0) {
          uVar3 = FUN_00419100(uVar5,-uVar4);
          sVar2 = (short)uVar3;
        }
        else {
          uVar3 = FUN_00419100(uVar5,uVar4);
          sVar2 = -(short)uVar3;
        }
        uVar4 = FUN_0040ca90(DAT_006db7ac,DAT_006db7b4);
        if (((int)(uVar4 & 0xffff0000) < 1) && (-0x10001 < (int)(uVar4 & 0xffff0000))) {
          uVar5 = FUN_0040cad0(DAT_006db7b0,DAT_006db7b4);
          if (((int)(uVar5 & 0xffff0000) < 1) && (-0x10001 < (int)(uVar5 & 0xffff0000))) {
            FUN_0047f360(DAT_0071f714,uVar1,0x100,(short)uVar4,
                         (DAT_006d4984 & 0xfffe) - (short)uVar5,DAT_006db7b4,sVar2);
          }
        }
      }
    }
  }
  return;
}
