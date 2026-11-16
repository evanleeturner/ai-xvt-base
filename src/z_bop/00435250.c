/* SPDX-License-Identifier: MIT */
/* z_bop 00435250 */
extern int DAT_009a1fe8;
extern unsigned char DAT_009d8c88[];
extern uint DAT_009d6990[];
extern unsigned short DAT_009d69f4[];
extern unsigned char DAT_009d9192[];
extern char DAT_009e9680[];
extern unsigned char DAT_009e96dd[];
extern unsigned char DAT_009e9901[];
extern unsigned char DAT_009e9903[];
extern unsigned char DAT_009e9905[];
extern unsigned char DAT_009e9907[];
extern unsigned char DAT_009e9678[];
extern unsigned char DAT_009ec460[];

int __cdecl FUN_00435ad0(uint param_1);
void __cdecl FUN_00434500(uint param_1, uint param_2);
uint __cdecl FUN_00425c60(uint param_1, uint param_2);

uint __cdecl FUN_00435250(uint param_1, int param_2)
{
  int iVar1;
  uint uVar2;
  uint uVar3;
  int iVar4;
  int iVar5;
  uint uVar6;
  unsigned char *puVar7;
  unsigned char *piVar8;
  uint uVar9;
  uint uVar10;
  unsigned char *puVar11;
  unsigned char *puVar12;
  unsigned char *iVar13;
  uint uVar14;
  unsigned char *puVar15;
  uint local_4;
  uint tmpA;
  uint tmpB;
  unsigned short tmpS;

  local_4 = 0xffffffff;
  iVar5 = FUN_00435ad0(param_1);
  uVar6 = (uint)(iVar5 / 2);
  iVar13 = (unsigned char *)(param_1 * 0x23u + (uint)DAT_009a1fe8);
  DAT_009d6990[DAT_009d8c88[(uint)iVar13[0x16] * 0x566u]] =
       DAT_009d6990[DAT_009d8c88[(uint)iVar13[0x16] * 0x566u]] - uVar6;
  DAT_009d69f4[DAT_009d8c88[(uint)iVar13[0x16] * 0x566u]] =
       (unsigned short)(DAT_009d69f4[DAT_009d8c88[(uint)iVar13[0x16] * 0x566u]] + 1);
  __builtin_memcpy(&iVar5, iVar13 + 0x1b, 4);
  if (iVar5 == -1) {
    __builtin_memcpy(&iVar5, DAT_009d9192 + (uint)iVar13[0x16] * 0x566u, 4);
    if (iVar5 == -1) {
      return uVar6;
    }
    if (DAT_009e9680[(uint)iVar5 * 0x5bdu] == '\0') {
      return uVar6;
    }
  }
  __builtin_memcpy(&tmpA, iVar13 + 0x1f, 4);
  __builtin_memcpy(&tmpB, (unsigned char *)tmpA + 0xa9, 4);
  iVar13 = (unsigned char *)tmpB;
  iVar4 = (int)((uint)iVar5 * 0x5bdu);
  __builtin_memcpy(&tmpA, DAT_009e96dd + iVar4, 4);
  tmpA = tmpA - uVar6;
  __builtin_memcpy(DAT_009e96dd + iVar4, &tmpA, 4);
  if (param_2 != 0) {
    __builtin_memcpy(&uVar2, iVar13 + 0xc3, 4);
    __builtin_memcpy(&uVar3, iVar13 + 0xcb, 4);
    if (uVar2 < uVar3) {
      uVar6 = uVar3 - uVar2;
    }
    else {
      uVar6 = 0;
    }
    __builtin_memcpy(&uVar2, iVar13 + 0x211, 4);
    if (0 < (int)uVar2) {
      uVar6 = uVar6 + (uVar2 & 0xffff);
    }
    __builtin_memcpy(&uVar2, iVar13 + 0x215, 4);
    if (0 < (int)uVar2) {
      uVar6 = uVar6 + (uVar2 & 0xffff);
    }
    uVar10 = 0;
    piVar8 = iVar13 + 0x1a7;
    iVar5 = 8;
    do {
      __builtin_memcpy(&iVar1, piVar8, 4);
      piVar8 = piVar8 + 4;
      uVar10 = uVar10 + (uint)iVar1;
      iVar5 = iVar5 + -1;
    } while (iVar5 != 0);
    piVar8 = iVar13 + 0x1c7;
    iVar5 = 6;
    do {
      __builtin_memcpy(&iVar1, piVar8, 4);
      piVar8 = piVar8 + 4;
      uVar10 = uVar10 + (uint)iVar1;
      iVar5 = iVar5 + -1;
    } while (iVar5 != 0);
    if (uVar6 >> 1 < uVar10) {
      FUN_00434500(param_1, param_1);
    }
  }
  __builtin_memcpy(&uVar2, iVar13 + 0xd7, 4);
  __builtin_memcpy(&uVar3, iVar13 + 0xd3, 4);
  uVar6 = FUN_00425c60(uVar2, uVar3);
  if (0x7fff < (unsigned short)uVar6) {
    uVar6 = 0;
    __builtin_memcpy(&tmpS, DAT_009e9901 + iVar4, 2);
    tmpS = (unsigned short)(tmpS + 1);
    __builtin_memcpy(DAT_009e9901 + iVar4, &tmpS, 2);
    puVar15 = iVar13 + 0x1a7;
    iVar5 = 8;
    puVar7 = puVar15;
    do {
      __builtin_memcpy(&uVar10, puVar7, 4);
      puVar7 = puVar7 + 4;
      uVar6 = uVar6 + uVar10;
      iVar5 = iVar5 + -1;
    } while (iVar5 != 0);
    uVar10 = 0;
    puVar7 = iVar13 + 0x1c7;
    iVar5 = 6;
    puVar12 = puVar7;
    do {
      __builtin_memcpy(&uVar9, puVar12, 4);
      puVar12 = puVar12 + 4;
      uVar10 = uVar10 + uVar9;
      iVar5 = iVar5 + -1;
    } while (iVar5 != 0);
    __builtin_memcpy(&uVar14, iVar13 + 0xdb, 4);
    __builtin_memcpy(&uVar2, iVar13 + 0xdf, 4);
    __builtin_memcpy(&uVar3, iVar13 + 0xe3, 4);
    uVar9 = uVar2 | uVar14 | uVar3 | uVar10 | uVar6;
    if (uVar9 != 0) {
      if ((((uVar14 < uVar3) || (uVar14 < uVar2)) || (uVar14 < uVar6)) || (uVar14 < uVar10)) {
        if (((uVar2 < uVar3) || (uVar2 < uVar14)) || ((uVar2 < uVar6 || (uVar2 < uVar10)))) {
          if (((uVar3 < uVar2) || (uVar3 < uVar14)) || ((uVar3 < uVar6 || (uVar3 < uVar10)))) {
            if ((((uVar6 < uVar2) || (uVar6 < uVar14)) || (uVar6 < uVar3)) || (uVar6 < uVar10)) {
              uVar6 = 0;
              iVar5 = 6;
              do {
                __builtin_memcpy(&tmpA, puVar7, 4);
                if (uVar6 < tmpA) {
                  uVar6 = tmpA;
                }
                puVar7 = puVar7 + 4;
                iVar5 = iVar5 + -1;
              } while (iVar5 != 0);
            }
            else {
              uVar6 = 0;
              uVar10 = 0xffffffff;
              uVar14 = 0;
              puVar11 = DAT_009e9678;
              do {
                __builtin_memcpy(&tmpA, puVar15, 4);
                if (uVar6 < tmpA) {
                  __builtin_memcpy(&tmpS, puVar11, 2);
                  uVar10 = (uint)tmpS;
                  uVar9 = uVar14;
                  uVar6 = tmpA;
                }
                puVar11 = puVar11 + 0x5bd;
                puVar15 = puVar15 + 4;
                uVar14 = uVar14 + 1;
              } while (puVar11 < DAT_009ec460);
              if (uVar10 != 0xffffffff) {
                local_4 = uVar9;
              }
            }
          }
          else {
            __builtin_memcpy(&tmpS, DAT_009e9907 + iVar4, 2);
            tmpS = (unsigned short)(tmpS + 1);
            __builtin_memcpy(DAT_009e9907 + iVar4, &tmpS, 2);
          }
        }
        else {
          __builtin_memcpy(&tmpS, DAT_009e9905 + iVar4, 2);
          tmpS = (unsigned short)(tmpS + 1);
          __builtin_memcpy(DAT_009e9905 + iVar4, &tmpS, 2);
        }
      }
      else {
        __builtin_memcpy(&tmpS, DAT_009e9903 + iVar4, 2);
        tmpS = (unsigned short)(tmpS + 1);
        __builtin_memcpy(DAT_009e9903 + iVar4, &tmpS, 2);
      }
    }
  }
  return local_4;
}
