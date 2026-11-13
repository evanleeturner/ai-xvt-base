/* SPDX-License-Identifier: MIT */
/* z_xvt 004785e0 */
extern int DAT_00527f7c;
extern int DAT_0052552c;
extern char DAT_00527f30[];
extern char DAT_00527ee0[];
extern char DAT_00527f31[];
extern char DAT_00527ee1[];
extern unsigned int DAT_0070acc4;
extern unsigned short DAT_00655a8c;
extern unsigned short DAT_00655a88;

extern unsigned int FUN_00418680(unsigned int, unsigned short);
extern unsigned int FUN_004187e0(unsigned int, unsigned short);
extern void FUN_00419000(unsigned int, unsigned int, unsigned short *, unsigned short *);

void __cdecl FUN_004785e0(int param_1, int param_2, int param_3)
{
  int iVar1;
  unsigned int uVar2;
  int iVar3;
  unsigned int uVar4;
  unsigned short uVar5;
  unsigned short local_4;
  unsigned short uStack_2;
  
  if (DAT_00527f7c != DAT_0052552c) {
    if (DAT_0052552c == 0x13) {
      iVar1 = 0x25;
      iVar3 = 0;
      do {
        DAT_00527f30[iVar3] = DAT_00527ee0[iVar3];
        iVar1 = iVar1 - 1;
        DAT_00527f31[iVar3] = DAT_00527ee1[iVar3];
        iVar3 = iVar3 + 2;
      } while (iVar1 != 0);
    }
    else if (DAT_0052552c == 0x112) {
      iVar1 = 0;
      iVar3 = 0;
      do {
        uVar5 = (unsigned short)iVar1;
        uVar4 = FUN_00418680(0x1e, uVar5);
        DAT_00527f30[iVar3] = (char)uVar4;
        iVar1 = iVar1 + 0x1bb;
        uVar4 = FUN_004187e0(0x1e, uVar5);
        DAT_00527f31[iVar3] = (char)uVar4;
        iVar3 = iVar3 + 2;
      } while (iVar1 < 0x4000);
    }
    else {
      iVar1 = 0;
      iVar3 = 0;
      do {
        uVar5 = (unsigned short)iVar1;
        uVar4 = FUN_00418680(0x2c, uVar5);
        DAT_00527f30[iVar3] = (char)uVar4;
        iVar1 = iVar1 + 0x1bb;
        uVar4 = FUN_004187e0(0x2c, uVar5);
        DAT_00527f31[iVar3] = (char)uVar4;
        iVar3 = iVar3 + 2;
      } while (iVar1 < 0x4000);
    }
    DAT_00527f7c = DAT_0052552c;
  }
  iVar3 = param_1;
  if (param_1 < 0) {
    iVar3 = -param_1;
  }
  uVar4 = iVar3 << ((DAT_0070acc4 - 5U) & 0x1f);
  if (param_3 != 0) {
    uVar4 = (int)uVar4 / param_3;
  }
  if (0x7fff < (int)uVar4) {
    uVar4 = 0x7fff;
  }
  iVar3 = param_2;
  if (param_2 < 0) {
    iVar3 = -param_2;
  }
  uVar2 = iVar3 << ((DAT_0070acc4 - 5U) & 0x1f);
  if (param_3 != 0) {
    uVar2 = (int)uVar2 / param_3;
  }
  if (0x7fff < (int)uVar2) {
    uVar2 = 0x7fff;
  }
  DAT_00655a8c = (unsigned short)uVar4;
  DAT_00655a88 = (unsigned short)uVar2;
  FUN_00419000(uVar4, uVar2, &uStack_2, &local_4);
  uVar5 = (unsigned short)(0x4000 - uStack_2) / 0x1bb << 1;
  if ((int)(unsigned int)(unsigned short)(unsigned char)DAT_00527f30[uVar5] < (int)(short)DAT_00655a8c) {
    DAT_00655a8c = (unsigned short)(unsigned char)DAT_00527f30[uVar5];
  }
  if (param_1 < 0) {
    DAT_00655a8c = -DAT_00655a8c;
  }
  if ((int)(unsigned int)(unsigned short)(unsigned char)DAT_00527f31[uVar5] < (int)(short)DAT_00655a88) {
    DAT_00655a88 = (unsigned short)(unsigned char)DAT_00527f31[uVar5];
  }
  if (param_2 < 0) {
    DAT_00655a88 = -DAT_00655a88;
  }
  return;
}
