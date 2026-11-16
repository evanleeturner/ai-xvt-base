/* SPDX-License-Identifier: MIT */
/* z_bop 004ee1a0 */
/* Globals at odd addresses (0xb6a5a6, 0xb6a5aa, 0xb6a606, 0xb6baca, 0xb6d962)
   are not 4-aligned, so they are declared as raw bytes and read via memcpy. */
extern int DAT_00b69e30;
extern unsigned char DAT_00b6a5a6[4];
extern unsigned char DAT_00b6a5aa[4];
extern unsigned char DAT_00b6a606[4];
extern unsigned char DAT_00b6baca[4];
extern unsigned char DAT_00b6d962[4];

extern int DAT_00aa5ac0[];
extern int DAT_00aa5cb0[];

extern int DAT_00aa6104;
extern int DAT_00aa60f8;
extern int DAT_00aa6100;
extern int DAT_00aa60d8;
extern int DAT_00aa60e4;
extern int DAT_00aa610c;
extern unsigned int DAT_00aa60e8;
extern unsigned int DAT_00aa60ec;
extern int DAT_00aa6108;
extern int DAT_00aa60f0;
extern int DAT_00aa60d4;
extern int DAT_00aa60dc;
extern int DAT_00aa60e0;

extern unsigned char DAT_00a91d21[];
extern unsigned char DAT_00a91d13[];
extern unsigned char DAT_00a92202[];
extern unsigned char DAT_00a92203[];
extern unsigned char DAT_00a921f0[];
extern unsigned char DAT_00a91d15[];
extern unsigned char DAT_00a921f8[];
extern unsigned char DAT_00a91d16[];
extern unsigned char DAT_00a921fe[];
extern unsigned char DAT_00a921e3[];
extern unsigned char DAT_00a9220c[];
extern unsigned char DAT_00a92216[];
extern unsigned char DAT_00a91d12[];
extern unsigned char DAT_0052c550[];
extern unsigned int DAT_0052c564[];
extern unsigned int DAT_0052c578[];

int FUN_004d11e0(void);
unsigned int __cdecl FUN_004ee8e0(int param_1);

void __stdcall FUN_004ee1a0(void)
{
  unsigned char bVar1;
  unsigned char bVar2;
  int iVar3;
  int iVar4;
  unsigned int uVar5;
  int iVar6;
  unsigned int uVar7;
  int iVar8;
  int iVar9;

  if (DAT_00b69e30 == 2) {
    __builtin_memcpy(&iVar8, DAT_00b6a5a6, 4);
    DAT_00aa6104 = DAT_00aa5ac0[(int)((unsigned int)iVar8 * 8u)];
  }
  else {
    iVar6 = 0;
    do {
      __builtin_memcpy(&iVar8, DAT_00b6a5a6, 4);
      iVar4 = DAT_00aa5cb0[(int)((unsigned int)iVar6 + (unsigned int)iVar8 * 8u)];
      iVar3 = FUN_004d11e0();
      if (iVar4 == iVar3) {
        __builtin_memcpy(&iVar8, DAT_00b6a5a6, 4);
        DAT_00aa6104 = DAT_00aa5ac0[(int)((unsigned int)iVar6 + (unsigned int)iVar8 * 8u)];
      }
      iVar6 = (int)((unsigned int)iVar6 + 1u);
    } while (iVar6 < 8);
  }
  DAT_00aa60f8 = 0;
  DAT_00aa6100 = 0;
  DAT_00aa60d8 = 0;
  DAT_00aa60e4 = 0;
  DAT_00aa610c = 0;
  iVar6 = (int)((unsigned int)DAT_00aa6104 * 0x562u);
  DAT_00aa60e8 = (unsigned int)DAT_00a91d21[iVar6];
  DAT_00aa60ec = (unsigned int)DAT_00a91d13[iVar6];
  switch (DAT_00a92202[iVar6]) {
  case 0:
    DAT_00aa6108 = 0;
    DAT_00aa60f0 = 1;
    break;
  case 1:
    DAT_00aa6108 = 0xb;
    DAT_00aa60f0 = 0;
    break;
  case 2:
    DAT_00aa6108 = 6;
    DAT_00aa60f0 = 0;
    break;
  case 3:
    DAT_00aa6108 = 6;
    DAT_00aa60f0 = 0;
    break;
  case 4:
    DAT_00aa60f0 = 1;
    iVar4 = 0;
    DAT_00aa6108 = 0;
    do {
      if (DAT_00a92203[iVar4 + iVar6] != 0) {
        DAT_00aa60f0 = (int)((unsigned int)DAT_00aa60f0 + 1u);
      }
      iVar4 = (int)((unsigned int)iVar4 + 1u);
    } while (iVar4 < 10);
  }
  DAT_00aa60d4 = 0;
  iVar4 = 0;
  do {
    if (DAT_00a921f0[iVar4 + iVar6] != 0) {
      DAT_00aa60d4 = (int)((unsigned int)DAT_00aa60d4 + 1u);
    }
    iVar4 = (int)((unsigned int)iVar4 + 1u);
  } while (iVar4 < 8);
  if ((DAT_00a91d15[iVar6] != 0) || (DAT_00aa60d4 != 0)) {
    DAT_00aa60d4 = (int)((unsigned int)DAT_00aa60d4 + 1u);
  }
  DAT_00aa60dc = 0;
  iVar4 = 0;
  do {
    if (DAT_00a921f8[iVar4 + iVar6] != 0) {
      DAT_00aa60dc = (int)((unsigned int)DAT_00aa60dc + 1u);
    }
    iVar4 = (int)((unsigned int)iVar4 + 1u);
  } while (iVar4 < 6);
  if ((DAT_00a91d16[iVar6] != 0) || (DAT_00aa60dc != 0)) {
    DAT_00aa60dc = (int)((unsigned int)DAT_00aa60dc + 1u);
  }
  DAT_00aa60e0 = 0;
  iVar4 = 0;
  do {
    if (DAT_00a921fe[iVar4 + iVar6] != 0) {
      DAT_00aa60e0 = (int)((unsigned int)DAT_00aa60e0 + 1u);
    }
    iVar4 = (int)((unsigned int)iVar4 + 1u);
  } while (iVar4 < 4);
  if ((DAT_00a921e3[iVar6] != 0) || (DAT_00aa60e0 != 0)) {
    DAT_00aa60e0 = (int)((unsigned int)DAT_00aa60e0 + 1u);
  }
  __builtin_memcpy(&iVar8, DAT_00b6a606, 4);
  if (iVar8 != 1) {
    DAT_00aa60d8 = 0;
    DAT_00aa60e4 = 0;
    DAT_00aa60f8 = 0;
    DAT_00aa6100 = 0;
    DAT_00aa610c = 0;
    return;
  }
  __builtin_memcpy(&iVar8, DAT_00b6a5aa, 4);
  if ((iVar8 != 1) && (iVar8 != 2)) {
    DAT_00aa60d8 = 0;
    DAT_00aa60e4 = 0;
    DAT_00aa60f8 = 0;
    DAT_00aa6100 = 0;
    DAT_00aa610c = 0;
    return;
  }
  __builtin_memcpy(&iVar8, DAT_00b6baca, 4);
  if (iVar8 == 0) {
    DAT_00aa60d8 = 0;
    DAT_00aa60e4 = 0;
    DAT_00aa60f8 = 0;
    DAT_00aa6100 = 0;
    DAT_00aa610c = 0;
    return;
  }
  uVar7 = 0;
  uVar5 = FUN_004ee8e0(-1);
  __builtin_memcpy(&iVar9, DAT_00b6d962, 4);
  if (((int)uVar5 < 1) || ((4 < (int)uVar5 && (uVar5 != 0xe)))) {
    if (iVar9 != 0) goto LAB_004ee3cc;
  }
  else if (iVar9 != 1) goto LAB_004ee3cc;
  uVar7 = (unsigned int)DAT_0052c550[uVar5];
LAB_004ee3cc:
  if (uVar7 != 0) {
    while (uVar5 = FUN_004ee8e0(-1), uVar7 != uVar5) {
      if (DAT_00aa6108 == 0) {
        DAT_00aa60f8 = (int)((unsigned int)DAT_00aa60f8 + 1u);
        if (DAT_00aa60f0 <= DAT_00aa60f8) {
          DAT_00aa60f8 = 0;
        }
        if (DAT_00aa60f8 == 0) {
          bVar1 = DAT_00a91d13[(int)((unsigned int)DAT_00aa6104 * 0x562u)];
          bVar2 = DAT_00a91d21[(int)((unsigned int)DAT_00aa6104 * 0x562u)];
        }
        else {
          iVar6 = (int)((unsigned int)DAT_00aa6104 * 0x562u + (unsigned int)DAT_00aa60f8);
          bVar1 = DAT_00a9220c[iVar6];
          bVar2 = DAT_00a92216[iVar6];
        }
        DAT_00aa60ec = (unsigned int)bVar1;
        DAT_00aa60e8 = (unsigned int)bVar2;
      }
      else {
        DAT_00aa6100 = (int)((unsigned int)DAT_00aa6100 + 1u);
        if (DAT_00aa6108 <= DAT_00aa6100) {
          DAT_00aa6100 = 0;
        }
        if (DAT_00aa6100 != 0) {
          iVar6 = (int)((unsigned int)DAT_00aa6104 * 0x562u);
          bVar1 = DAT_00a92202[iVar6];
          if (bVar1 != 0) {
            if (bVar1 < 3) {
              if ((unsigned int)DAT_00a91d12[iVar6] == DAT_0052c564[DAT_00aa6100]) {
                DAT_00aa6100 = (int)((unsigned int)DAT_00aa6100 + 1u);
              }
            }
            else if ((bVar1 == 3) &&
                     ((unsigned int)DAT_00a91d12[iVar6] == DAT_0052c578[DAT_00aa6100])) {
              DAT_00aa6100 = (int)((unsigned int)DAT_00aa6100 + 1u);
            }
          }
        }
        if (DAT_00aa6108 <= DAT_00aa6100) {
          DAT_00aa6100 = 0;
        }
      }
    }
  }
  return;
}
