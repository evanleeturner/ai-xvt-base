/* SPDX-License-Identifier: MIT */
/* z_bop 00425e30 */
extern unsigned int __cdecl FUN_0046a5f0(unsigned short param_1, unsigned short param_2);
extern unsigned int __cdecl FUN_0046a700(unsigned short param_1, unsigned short param_2);
extern void __cdecl FUN_0046aaf0(unsigned int param_1, unsigned int param_2, unsigned short *param_3, unsigned short *param_4);

extern int DAT_005233ec;
extern int DAT_0051c55c;
extern unsigned char DAT_0051c510[];
extern unsigned char DAT_0051c4c0[];
extern unsigned char DAT_0051c4c1[];
extern unsigned char DAT_0051c511[];
extern unsigned int DAT_009d8c04;
extern unsigned short DAT_00a08c7c;
extern unsigned short DAT_00a08c78;

void __cdecl FUN_00425e30(int param_1, int param_2, int param_3)
{
  int iVar1;
  unsigned int uVar2;
  int iVar3;
  unsigned int uVar4;
  unsigned short uVar5;
  unsigned short local_4;
  unsigned short uStack_2;

  if (DAT_005233ec != DAT_0051c55c) {
    if (DAT_005233ec == 0x13) {
      iVar1 = 0x25;
      iVar3 = 0;
      do {
        DAT_0051c510[iVar3] = DAT_0051c4c0[iVar3];
        iVar1 = iVar1 - 1;
        DAT_0051c511[iVar3] = DAT_0051c4c1[iVar3];
        iVar3 = iVar3 + 2;
      } while (iVar1 != 0);
    }
    else if (DAT_005233ec == 0x112) {
      iVar1 = 0;
      iVar3 = 0;
      do {
        uVar5 = (unsigned short)iVar1;
        uVar4 = FUN_0046a5f0(0x1e, uVar5);
        DAT_0051c510[iVar3] = (unsigned char)uVar4;
        iVar1 = iVar1 + 0x1bb;
        uVar4 = FUN_0046a700(0x1e, uVar5);
        DAT_0051c511[iVar3] = (unsigned char)uVar4;
        iVar3 = iVar3 + 2;
      } while (iVar1 < 0x4000);
    }
    else {
      iVar1 = 0;
      iVar3 = 0;
      do {
        uVar5 = (unsigned short)iVar1;
        uVar4 = FUN_0046a5f0(0x2c, uVar5);
        DAT_0051c510[iVar3] = (unsigned char)uVar4;
        iVar1 = iVar1 + 0x1bb;
        uVar4 = FUN_0046a700(0x2c, uVar5);
        DAT_0051c511[iVar3] = (unsigned char)uVar4;
        iVar3 = iVar3 + 2;
      } while (iVar1 < 0x4000);
    }
    DAT_0051c55c = DAT_005233ec;
  }
  iVar3 = param_1;
  if (param_1 < 0) {
    iVar3 = -param_1;
  }
  uVar4 = (unsigned int)iVar3 << ((DAT_009d8c04 - 5U) & 0x1f);
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
  uVar2 = (unsigned int)iVar3 << ((DAT_009d8c04 - 5U) & 0x1f);
  if (param_3 != 0) {
    uVar2 = (int)uVar2 / param_3;
  }
  if (0x7fff < (int)uVar2) {
    uVar2 = 0x7fff;
  }
  DAT_00a08c7c = (unsigned short)uVar4;
  DAT_00a08c78 = (unsigned short)uVar2;
  FUN_0046aaf0(uVar4, uVar2, &uStack_2, &local_4);
  uVar5 = (unsigned short)((0x4000 - uStack_2) / 0x1bb << 1);
  if ((int)(unsigned char)DAT_0051c510[uVar5] < (int)(short)DAT_00a08c7c) {
    DAT_00a08c7c = (unsigned short)(unsigned char)DAT_0051c510[uVar5];
  }
  if (param_1 < 0) {
    DAT_00a08c7c = -DAT_00a08c7c;
  }
  if ((int)(unsigned char)DAT_0051c511[uVar5] < (int)(short)DAT_00a08c78) {
    DAT_00a08c78 = (unsigned short)(unsigned char)DAT_0051c511[uVar5];
  }
  if (param_2 < 0) {
    DAT_00a08c78 = -DAT_00a08c78;
  }
  return;
}
