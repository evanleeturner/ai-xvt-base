/* SPDX-License-Identifier: MIT */
/* z_xvt 00499090 */
extern unsigned int *DAT_00645a80;
extern unsigned char *DAT_00645ad0;
extern int DAT_00703c88;
extern int DAT_00732a38;
extern int DAT_006db7a4;
extern int DAT_00703ac0;
extern int DAT_006d4988;
extern int DAT_007091a0;
extern int DAT_00732940;
extern int DAT_006db798;
extern int DAT_006db71c;

extern unsigned int DAT_0071f2d8;
extern unsigned int _DAT_0071f2dc;
extern unsigned int DAT_0070ac30;
extern unsigned int _DAT_0070ac34;
extern short DAT_006d49a0[81];
extern unsigned char DAT_006d4a60[];
extern unsigned char DAT_0070acf0[];
extern unsigned char DAT_007092c0[];
extern unsigned char DAT_0071e6a0[];
extern short DAT_00732fe4;
extern unsigned int DAT_00732e48;
extern unsigned char DAT_0070ace0;
extern unsigned int DAT_0071eae8;
extern unsigned int DAT_006da530;
extern unsigned int DAT_0072f4e0;
extern unsigned int DAT_00732e9c;
extern unsigned int DAT_006da4f0;
extern unsigned int DAT_0070a000;
extern unsigned int _DAT_00732a3c;
extern unsigned int DAT_006d4980;
extern unsigned int DAT_00703acc;
extern unsigned int DAT_0073088c;
extern unsigned int DAT_0071b704;
extern unsigned int DAT_00732904;
extern unsigned int DAT_006da4f4;
extern unsigned int DAT_00703ac8;
extern unsigned int DAT_00703c90;
extern unsigned int DAT_0071e53c;
extern unsigned char DAT_00703ca0[];
extern unsigned int DAT_006da9e8;
extern unsigned int DAT_006d9d44;
extern unsigned char DAT_006da3e0[];
extern short DAT_00703abc;
extern short DAT_006db6c0;
extern unsigned int DAT_00525564;
extern unsigned char DAT_0071b730[];

void __stdcall FUN_00499090(void)
{
  int *piVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  unsigned int uVar5;
  unsigned int uVar6;
  unsigned int *puVar7;
  unsigned int *puVar8;
  unsigned int *puVar9;
  int iVar10;
  int iVar11;
  int iVar12;

  iVar10 = 0;
  puVar8 = DAT_00645a80;
  if (0 < (int)((unsigned int)DAT_00703c88 + (unsigned int)DAT_00732a38)) {
    iVar3 = 0;
    puVar7 = DAT_00645a80;
    do {
      if ((iVar10 < DAT_006db7a4) || (puVar8 = puVar7, DAT_00703ac0 <= iVar10)) {
        puVar8 = (unsigned int *)((int)puVar7 + 1);
        *(unsigned char *)puVar7 = *(unsigned char *)(DAT_006d4988 + 3 + iVar3);
        iVar4 = DAT_006d4988 + iVar3;
        if (*(char *)(iVar4 + 3) != '\0') {
          __builtin_memcpy(&iVar2, (void *)(iVar4 + 0x1f), 4);
          if (iVar2 != 0) {
            iVar2 = (int)((unsigned int)iVar2 - (unsigned int)DAT_007091a0);
            __builtin_memcpy((void *)(iVar4 + 0x1f), &iVar2, 4);
            piVar1 = (int *)(DAT_006d4988 + 0x1f + iVar3);
            __builtin_memcpy(&iVar12, piVar1, 4);
            iVar12 = (int)((unsigned int)iVar12 + 1u);
            __builtin_memcpy(piVar1, &iVar12, 4);
          }
          puVar9 = (unsigned int *)(DAT_006d4988 + iVar3);
          for (iVar4 = 8; iVar4 != 0; iVar4 = iVar4 + -1) {
            __builtin_memcpy(puVar8, puVar9, 4);
            puVar9 = puVar9 + 1;
            puVar8 = puVar8 + 1;
          }
          __builtin_memcpy(puVar8, puVar9, 2);
          *(unsigned char *)((int)puVar8 + 2) = *(unsigned char *)((int)puVar9 + 2);
          puVar8 = puVar7 + 9;
          __builtin_memcpy(&iVar4, (void *)(DAT_006d4988 + 0x1f + iVar3), 4);
          if (iVar4 != 0) {
            iVar2 = (int)((unsigned int)iVar4 - 1u);
            __builtin_memcpy((void *)(DAT_006d4988 + 0x1f + iVar3), &iVar2, 4);
            piVar1 = (int *)(DAT_006d4988 + 0x1f + iVar3);
            __builtin_memcpy(&iVar12, piVar1, 4);
            iVar12 = (int)((unsigned int)iVar12 + (unsigned int)DAT_007091a0);
            __builtin_memcpy(piVar1, &iVar12, 4);
          }
          __builtin_memcpy(&iVar4, (void *)(DAT_006d4988 + 0x1f + iVar3), 4);
          if (iVar4 != 0) {
            __builtin_memcpy(&iVar2, (void *)(iVar4 + 0xa7), 4);
            if (iVar2 != 0) {
              iVar2 = (int)((unsigned int)iVar2 - (unsigned int)DAT_00732940);
              __builtin_memcpy((void *)(iVar4 + 0xa7), &iVar2, 4);
              __builtin_memcpy(&iVar11, (void *)(DAT_006d4988 + 0x1f + iVar3), 4);
              piVar1 = (int *)(iVar11 + 0xa7);
              __builtin_memcpy(&iVar12, piVar1, 4);
              iVar12 = (int)((unsigned int)iVar12 + 1u);
              __builtin_memcpy(piVar1, &iVar12, 4);
            }
            __builtin_memcpy(&iVar4, (void *)(DAT_006d4988 + 0x1f + iVar3), 4);
            __builtin_memcpy(&iVar2, (void *)(iVar4 + 0xa3), 4);
            if (iVar2 != 0) {
              iVar2 = (int)((unsigned int)iVar2 - (unsigned int)DAT_006db798);
              __builtin_memcpy((void *)(iVar4 + 0xa3), &iVar2, 4);
              __builtin_memcpy(&iVar11, (void *)(DAT_006d4988 + 0x1f + iVar3), 4);
              piVar1 = (int *)(iVar11 + 0xa3);
              __builtin_memcpy(&iVar12, piVar1, 4);
              iVar12 = (int)((unsigned int)iVar12 + 1u);
              __builtin_memcpy(piVar1, &iVar12, 4);
            }
            __builtin_memcpy(&iVar4, (void *)(DAT_006d4988 + 0x1f + iVar3), 4);
            __builtin_memcpy(&iVar2, (void *)(iVar4 + 0xab), 4);
            if (iVar2 != 0) {
              iVar2 = (int)((unsigned int)iVar2 - (unsigned int)DAT_006db71c);
              __builtin_memcpy((void *)(iVar4 + 0xab), &iVar2, 4);
              __builtin_memcpy(&iVar11, (void *)(DAT_006d4988 + 0x1f + iVar3), 4);
              piVar1 = (int *)(iVar11 + 0xab);
              __builtin_memcpy(&iVar12, piVar1, 4);
              iVar12 = (int)((unsigned int)iVar12 + 1u);
              __builtin_memcpy(piVar1, &iVar12, 4);
            }
            __builtin_memcpy(&puVar9, (void *)(DAT_006d4988 + 0x1f + iVar3), 4);
            for (iVar4 = 0x2b; iVar4 != 0; iVar4 = iVar4 + -1) {
              __builtin_memcpy(puVar8, puVar9, 4);
              puVar9 = puVar9 + 1;
              puVar8 = puVar8 + 1;
            }
            __builtin_memcpy(puVar8, puVar9, 2);
            *(unsigned char *)((int)puVar8 + 2) = *(unsigned char *)((int)puVar9 + 2);
            __builtin_memcpy(&iVar4, (void *)(DAT_006d4988 + 0x1f + iVar3), 4);
            __builtin_memcpy(&iVar2, (void *)(iVar4 + 0xa7), 4);
            if (iVar2 != 0) {
              iVar12 = (int)((unsigned int)iVar2 - 1u);
              __builtin_memcpy((void *)(iVar4 + 0xa7), &iVar12, 4);
              __builtin_memcpy(&iVar11, (void *)(DAT_006d4988 + 0x1f + iVar3), 4);
              piVar1 = (int *)(iVar11 + 0xa7);
              __builtin_memcpy(&iVar12, piVar1, 4);
              iVar12 = (int)((unsigned int)iVar12 + (unsigned int)DAT_00732940);
              __builtin_memcpy(piVar1, &iVar12, 4);
            }
            __builtin_memcpy(&iVar4, (void *)(DAT_006d4988 + 0x1f + iVar3), 4);
            __builtin_memcpy(&iVar2, (void *)(iVar4 + 0xa3), 4);
            if (iVar2 != 0) {
              iVar12 = (int)((unsigned int)iVar2 - 1u);
              __builtin_memcpy((void *)(iVar4 + 0xa3), &iVar12, 4);
              __builtin_memcpy(&iVar11, (void *)(DAT_006d4988 + 0x1f + iVar3), 4);
              piVar1 = (int *)(iVar11 + 0xa3);
              __builtin_memcpy(&iVar12, piVar1, 4);
              iVar12 = (int)((unsigned int)iVar12 + (unsigned int)DAT_006db798);
              __builtin_memcpy(piVar1, &iVar12, 4);
            }
            __builtin_memcpy(&iVar4, (void *)(DAT_006d4988 + 0x1f + iVar3), 4);
            __builtin_memcpy(&iVar2, (void *)(iVar4 + 0xab), 4);
            if (iVar2 != 0) {
              iVar12 = (int)((unsigned int)iVar2 - 1u);
              __builtin_memcpy((void *)(iVar4 + 0xab), &iVar12, 4);
              __builtin_memcpy(&iVar11, (void *)(DAT_006d4988 + 0x1f + iVar3), 4);
              piVar1 = (int *)(iVar11 + 0xab);
              __builtin_memcpy(&iVar12, piVar1, 4);
              iVar12 = (int)((unsigned int)iVar12 + (unsigned int)DAT_006db71c);
              __builtin_memcpy(piVar1, &iVar12, 4);
            }
            puVar9 = (unsigned int *)((int)puVar7 + 0xd3);
            __builtin_memcpy(&iVar11, (void *)(DAT_006d4988 + 0x1f + iVar3), 4);
            __builtin_memcpy(&iVar12, (void *)(iVar11 + 0xa7), 4);
            if (iVar12 != 0) {
              iVar4 = 0;
              do {
                __builtin_memcpy(&iVar11, (void *)(DAT_006d4988 + 0x1f + iVar3), 4);
                __builtin_memcpy(&iVar12, (void *)(iVar11 + 0xa7), 4);
                __builtin_memcpy(&iVar2, (void *)(iVar12 + 0x3f2 + iVar4), 4);
                if (iVar2 != 0) {
                  piVar1 = (int *)(iVar12 + 0x3f2 + iVar4);
                  __builtin_memcpy(&iVar2, piVar1, 4);
                  iVar2 = (int)((unsigned int)iVar2 - (unsigned int)DAT_006d4988);
                  __builtin_memcpy(piVar1, &iVar2, 4);
                  piVar1 = (int *)(iVar12 + 0x3f2 + iVar4);
                  __builtin_memcpy(&iVar2, piVar1, 4);
                  iVar2 = (int)((unsigned int)iVar2 + 1u);
                  __builtin_memcpy(piVar1, &iVar2, 4);
                }
                iVar4 = iVar4 + 4;
              } while (iVar4 < 0x40);
              __builtin_memcpy(&iVar11, (void *)(DAT_006d4988 + 0x1f + iVar3), 4);
              __builtin_memcpy(&iVar4, (void *)(iVar11 + 0xa7), 4);
              __builtin_memcpy(&iVar2, (void *)(iVar4 + 0x432), 4);
              if (iVar2 != 0) {
                iVar12 = (int)((unsigned int)iVar2 - (unsigned int)DAT_006d4988);
                __builtin_memcpy((void *)(iVar4 + 0x432), &iVar12, 4);
                __builtin_memcpy(&iVar11, (void *)(DAT_006d4988 + 0x1f + iVar3), 4);
                __builtin_memcpy(&iVar12, (void *)(iVar11 + 0xa7), 4);
                piVar1 = (int *)(iVar12 + 0x432);
                __builtin_memcpy(&iVar12, piVar1, 4);
                iVar12 = (int)((unsigned int)iVar12 + 1u);
                __builtin_memcpy(piVar1, &iVar12, 4);
              }
              puVar9 = (unsigned int *)((int)puVar7 + 0x509);
              __builtin_memcpy(&iVar11, (void *)(DAT_006d4988 + 0x1f + iVar3), 4);
              __builtin_memcpy(&puVar8, (void *)(iVar11 + 0xa7), 4);
              puVar7 = (unsigned int *)((int)puVar7 + 0xd3);
              for (iVar4 = 0x10d; iVar4 != 0; iVar4 = iVar4 + -1) {
                __builtin_memcpy(puVar7, puVar8, 4);
                puVar8 = puVar8 + 1;
                puVar7 = puVar7 + 1;
              }
              __builtin_memcpy(puVar7, puVar8, 2);
              iVar4 = 0;
              do {
                __builtin_memcpy(&iVar11, (void *)(DAT_006d4988 + 0x1f + iVar3), 4);
                __builtin_memcpy(&iVar12, (void *)(iVar11 + 0xa7), 4);
                __builtin_memcpy(&iVar2, (void *)(iVar12 + 0x3f2 + iVar4), 4);
                if (iVar2 != 0) {
                  piVar1 = (int *)(iVar12 + 0x3f2 + iVar4);
                  __builtin_memcpy(&iVar2, piVar1, 4);
                  iVar2 = (int)((unsigned int)iVar2 + 0xffffffffu);
                  __builtin_memcpy(piVar1, &iVar2, 4);
                  piVar1 = (int *)(iVar12 + 0x3f2 + iVar4);
                  __builtin_memcpy(&iVar2, piVar1, 4);
                  iVar2 = (int)((unsigned int)iVar2 + (unsigned int)DAT_006d4988);
                  __builtin_memcpy(piVar1, &iVar2, 4);
                }
                iVar4 = iVar4 + 4;
              } while (iVar4 < 0x40);
              __builtin_memcpy(&iVar11, (void *)(DAT_006d4988 + 0x1f + iVar3), 4);
              __builtin_memcpy(&iVar4, (void *)(iVar11 + 0xa7), 4);
              __builtin_memcpy(&iVar2, (void *)(iVar4 + 0x432), 4);
              if (iVar2 != 0) {
                iVar12 = (int)((unsigned int)iVar2 - 1u);
                __builtin_memcpy((void *)(iVar4 + 0x432), &iVar12, 4);
                __builtin_memcpy(&iVar11, (void *)(DAT_006d4988 + 0x1f + iVar3), 4);
                __builtin_memcpy(&iVar12, (void *)(iVar11 + 0xa7), 4);
                piVar1 = (int *)(iVar12 + 0x432);
                __builtin_memcpy(&iVar12, piVar1, 4);
                iVar12 = (int)((unsigned int)iVar12 + (unsigned int)DAT_006d4988);
                __builtin_memcpy(piVar1, &iVar12, 4);
              }
            }
            __builtin_memcpy(&iVar11, (void *)(DAT_006d4988 + 0x1f + iVar3), 4);
            __builtin_memcpy(&puVar7, (void *)(iVar11 + 0xa3), 4);
            puVar8 = puVar9;
            if (puVar7 != (unsigned int *)0x0) {
              __builtin_memcpy(puVar9, puVar7, 4);
              puVar8 = puVar9 + 2;
              __builtin_memcpy(puVar9 + 1, puVar7 + 1, 4);
            }
            __builtin_memcpy(&iVar11, (void *)(DAT_006d4988 + 0x1f + iVar3), 4);
            __builtin_memcpy(&puVar7, (void *)(iVar11 + 0xab), 4);
            if (puVar7 != (unsigned int *)0x0) {
              puVar9 = puVar8;
              for (iVar4 = 0x13; iVar4 != 0; iVar4 = iVar4 + -1) {
                __builtin_memcpy(puVar9, puVar7, 4);
                puVar7 = puVar7 + 1;
                puVar9 = puVar9 + 1;
              }
              puVar8 = puVar8 + 0x13;
            }
          }
        }
      }
      iVar3 = (int)((unsigned int)iVar3 + 0x23u);
      iVar10 = iVar10 + 1;
      puVar7 = puVar8;
    } while (iVar10 < (int)((unsigned int)DAT_00703c88 + (unsigned int)DAT_00732a38));
  }
  __builtin_memcpy(puVar8, &DAT_0071f2d8, 4);
  __builtin_memcpy(puVar8 + 1, &_DAT_0071f2dc, 4);
  __builtin_memcpy(puVar8 + 2, &DAT_0070ac30, 4);
  __builtin_memcpy(puVar8 + 3, &_DAT_0070ac34, 4);
  puVar7 = (unsigned int *)&DAT_006d49a0[0];
  puVar9 = puVar8 + 4;
  for (iVar10 = 0x28; iVar10 != 0; iVar10 = iVar10 + -1) {
    __builtin_memcpy(puVar9, puVar7, 4);
    puVar7 = puVar7 + 1;
    puVar9 = puVar9 + 1;
  }
  __builtin_memcpy(puVar9, puVar7, 2);
  iVar10 = (int)DAT_006d49a0[0];
  puVar7 = (unsigned int *)DAT_006d4a60;
  puVar9 = (unsigned int *)((int)puVar8 + 0xb2);
  for (uVar5 = (unsigned int)((unsigned int)iVar10 * 0x126u) >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
    __builtin_memcpy(puVar9, puVar7, 4);
    puVar7 = puVar7 + 1;
    puVar9 = puVar9 + 1;
  }
  for (uVar5 = (unsigned int)iVar10 * 0x126u & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
    *(unsigned char *)puVar9 = *(unsigned char *)puVar7;
    puVar7 = (unsigned int *)((int)puVar7 + 1);
    puVar9 = (unsigned int *)((int)puVar9 + 1);
  }
  puVar8 = (unsigned int *)((unsigned int)puVar8 + 0xb2u + (unsigned int)(int)DAT_006d49a0[0] * 0x126u);
  uVar5 = (unsigned int)(int)DAT_006d49a0[0] * 0x566u;
  puVar7 = (unsigned int *)DAT_0070acf0;
  puVar9 = puVar8;
  for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
    __builtin_memcpy(puVar9, puVar7, 4);
    puVar7 = puVar7 + 1;
    puVar9 = puVar9 + 1;
  }
  for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
    *(unsigned char *)puVar9 = *(unsigned char *)puVar7;
    puVar7 = (unsigned int *)((int)puVar7 + 1);
    puVar9 = (unsigned int *)((int)puVar9 + 1);
  }
  puVar8 = (unsigned int *)((unsigned int)puVar8 + (unsigned int)(int)DAT_006d49a0[0] * 0x566u);
  puVar7 = (unsigned int *)DAT_007092c0;
  puVar9 = puVar8;
  for (iVar10 = 0x34c; iVar10 != 0; iVar10 = iVar10 + -1) {
    __builtin_memcpy(puVar9, puVar7, 4);
    puVar7 = puVar7 + 1;
    puVar9 = puVar9 + 1;
  }
  puVar7 = (unsigned int *)DAT_0071e6a0;
  puVar9 = puVar8 + 0x34c;
  for (iVar10 = 5; iVar10 != 0; iVar10 = iVar10 + -1) {
    __builtin_memcpy(puVar9, puVar7, 4);
    puVar7 = puVar7 + 1;
    puVar9 = puVar9 + 1;
  }
  __builtin_memcpy(puVar9, puVar7, 2);
  __builtin_memcpy((unsigned char *)puVar8 + 0xd46, &DAT_00732fe4, 2);
  __builtin_memcpy(puVar8 + 0x352, &DAT_00732e48, 4);
  *(unsigned char *)(puVar8 + 0x353) = DAT_0070ace0;
  __builtin_memcpy((unsigned char *)puVar8 + 0xd4d, &DAT_0071eae8, 4);
  __builtin_memcpy((unsigned char *)puVar8 + 0xd51, &DAT_006da530, 4);
  __builtin_memcpy((unsigned char *)puVar8 + 0xd55, &DAT_0072f4e0, 4);
  __builtin_memcpy((unsigned char *)puVar8 + 0xd59, &DAT_00732e9c, 4);
  __builtin_memcpy((unsigned char *)puVar8 + 0xd5d, &DAT_006da4f0, 4);
  __builtin_memcpy((unsigned char *)puVar8 + 0xd61, &DAT_0070a000, 4);
  __builtin_memcpy((unsigned char *)puVar8 + 0xd65, &_DAT_00732a3c, 4);
  __builtin_memcpy((unsigned char *)puVar8 + 0xd69, &DAT_006d4980, 4);
  __builtin_memcpy((unsigned char *)puVar8 + 0xd6d, &DAT_00703acc, 4);
  __builtin_memcpy((unsigned char *)puVar8 + 0xd71, &DAT_0073088c, 4);
  __builtin_memcpy((unsigned char *)puVar8 + 0xd75, &DAT_0071b704, 4);
  __builtin_memcpy((unsigned char *)puVar8 + 0xd79, &DAT_00732904, 4);
  __builtin_memcpy((unsigned char *)puVar8 + 0xd7d, &DAT_006da4f4, 4);
  __builtin_memcpy((unsigned char *)puVar8 + 0xd81, &DAT_00703ac8, 4);
  __builtin_memcpy((unsigned char *)puVar8 + 0xd85, &DAT_00703c90, 4);
  __builtin_memcpy((unsigned char *)puVar8 + 0xd89, &DAT_0071e53c, 4);
  __builtin_memcpy((unsigned char *)puVar8 + 0xd8d, &DAT_006db7a4, 4);
  __builtin_memcpy((unsigned char *)puVar8 + 0xd91, &DAT_00703ac0, 4);
  __builtin_memcpy((unsigned char *)puVar8 + 0xd95, &DAT_00732a38, 4);
  __builtin_memcpy((unsigned char *)puVar8 + 0xd99, &DAT_00703c88, 4);
  puVar7 = (unsigned int *)DAT_00703ca0;
  puVar9 = (unsigned int *)((int)puVar8 + 0xd9d);
  for (iVar10 = 0x1540; iVar10 != 0; iVar10 = iVar10 + -1) {
    __builtin_memcpy(puVar9, puVar7, 4);
    puVar7 = puVar7 + 1;
    puVar9 = puVar9 + 1;
  }
  __builtin_memcpy((unsigned char *)puVar8 + 0x629d, &DAT_006da9e8, 4);
  __builtin_memcpy((unsigned char *)puVar8 + 0x62a1, &DAT_006d9d44, 4);
  puVar7 = (unsigned int *)DAT_006da3e0;
  puVar9 = (unsigned int *)((int)puVar8 + 0x62a5);
  for (iVar10 = 0x40; iVar10 != 0; iVar10 = iVar10 + -1) {
    __builtin_memcpy(puVar9, puVar7, 4);
    puVar7 = puVar7 + 1;
    puVar9 = puVar9 + 1;
  }
  __builtin_memcpy((unsigned char *)puVar8 + 0x63a5, &DAT_00703abc, 2);
  __builtin_memcpy((unsigned char *)puVar8 + 0x63a7, &DAT_006db6c0, 2);
  __builtin_memcpy((unsigned char *)puVar8 + 0x63a9, &DAT_00525564, 4);
  puVar7 = (unsigned int *)DAT_0071b730;
  puVar9 = (unsigned int *)((int)puVar8 + 0x63ad);
  for (iVar10 = 0xb7a; iVar10 != 0; iVar10 = iVar10 + -1) {
    __builtin_memcpy(puVar9, puVar7, 4);
    puVar7 = puVar7 + 1;
    puVar9 = puVar9 + 1;
  }
  DAT_00645ad0 = (unsigned char *)((int)puVar8 + (0x9195 - (int)DAT_00645a80));
  return;
}
