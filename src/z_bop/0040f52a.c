/* SPDX-License-Identifier: MIT */
/* z_bop 0040f52a */
extern unsigned short DAT_009fe7e4;
extern unsigned short DAT_00a08108;
extern unsigned short DAT_009a6fe0;
extern unsigned short DAT_00a08102;
extern unsigned char DAT_009a20ae;
extern char DAT_009d80c8;
extern char DAT_009a1ff4;
extern unsigned int DAT_009d7674;
extern unsigned short DAT_009d8c02;
extern unsigned short DAT_009d8c00;
extern unsigned short DAT_009cd270;
extern unsigned short DAT_00a0813c;
extern unsigned short DAT_00a07ce4;
extern unsigned int DAT_005233ec;
extern unsigned int DAT_005233f4;
extern unsigned int DAT_005233cc;
extern unsigned int DAT_009d8c08;
extern char DAT_009ec464;
extern unsigned char DAT_009a807a;
extern unsigned char DAT_00a08100;
extern unsigned char DAT_009e8f52;

void __stdcall FUN_00410110(void);
unsigned int __cdecl FUN_0040e190(int param_1);
void __stdcall FUN_00410ff0(void);
int __stdcall FUN_0040e1a0(void);

void __stdcall FUN_0040f52a(void)
{
  unsigned char bVar1;
  unsigned char bVar2;
  unsigned char bVar3;
  unsigned char in_AL = 0;   /* register input (AL): unresolved, supplied by caller in original */
  unsigned int uVar4;
  int iVar5;
  unsigned char *puVar6;
  unsigned char uVar7;
  unsigned char *pbVar8;
  int iVar9;
  unsigned char *puVar10;
  int iVar11;
  unsigned int uVar12;
  int iVar13;
  unsigned int uVar14;
  unsigned int uVar15;
  int in_ZF = 0;             /* register input (ZF): unresolved, supplied by caller in original */
  unsigned char *puStack00000004;
  unsigned int uStack0000000c;
  unsigned int uStack00000010;
  unsigned int uTmp;

  if (in_ZF) {
    if (DAT_009fe7e4 != 0) {
      FUN_00410110();
    }
    DAT_00a08108 = DAT_009a6fe0;
    DAT_00a08102 = (unsigned short)(DAT_00a08102 + DAT_009a20ae + 1);
    return;
  }
  if (in_AL < 0x20) {
    return;
  }
  if ((((DAT_009d80c8 != '\0') && (DAT_009a1ff4 == '\0')) && (0x60 < in_AL)) && (in_AL < 0x7b)) {
    in_AL = (unsigned char)(in_AL - 0x20);
  }
  pbVar8 = (unsigned char *)((unsigned int)(unsigned char)(in_AL - 0x20) * (unsigned int)DAT_009d8c02 + DAT_009d7674);
  bVar1 = *pbVar8;
  bVar2 = pbVar8[1];
  puStack00000004 = pbVar8 + 2;
  uStack0000000c = (unsigned int)(unsigned short)bVar1;
  if (((int)DAT_009d8c00 <= (int)((unsigned int)(int)DAT_00a08108 + uStack0000000c)) && (DAT_009cd270 != 0)) {
    if (DAT_009fe7e4 != 0) {
      FUN_00410110();
    }
    DAT_00a08108 = DAT_009a6fe0;
    DAT_00a08102 = (unsigned short)(DAT_00a08102 + bVar2 + 2);
  }
  uVar15 = 0;
  bVar3 = 0;
  iVar5 = (int)DAT_00a08102;
  if ((int)DAT_00a08102 < (int)DAT_00a0813c) {
    iVar5 = (int)DAT_00a0813c;
  }
  uVar4 = FUN_0040e190(iVar5);
  if (DAT_005233ec == 0x13) {
LAB_0040f6cc:
    if (DAT_005233f4 == 0xa0000) goto LAB_0040f6dd;
  }
  else if (DAT_005233f4 == 0xa0000) {
    uVar4 = uVar4 % DAT_005233cc;
    FUN_00410ff0();
    iVar11 = (int)((unsigned int)DAT_00a08102 + 1u + (unsigned int)bVar2);
    if ((int)DAT_00a07ce4 < iVar11) {
      iVar11 = (int)DAT_00a07ce4;
    }
    if (0xffff < (int)(uVar4 + ((unsigned int)iVar11 - (unsigned int)iVar5) * DAT_009d8c08)) {
      bVar3 = 1;
    }
    goto LAB_0040f6cc;
  }
  bVar3 = 1;
LAB_0040f6dd:
  if (bVar3) {
    iVar5 = (int)DAT_00a08102;
    uStack00000010 = (unsigned int)bVar2;
    if (iVar5 < (int)(uStack00000010 + (unsigned int)iVar5)) {
      do {
        __builtin_memcpy(&uTmp, puStack00000004, 4);
        uVar4 = uTmp;
        uVar14 = uStack0000000c;
        if (DAT_009ec464 != '\0') {
          uVar14 = uStack0000000c + 1;
        }
        iVar11 = (int)DAT_00a08108;
        iVar9 = (int)DAT_009a6fe0;
        if (iVar11 < iVar9) {
          if ((int)uVar14 <= iVar9 - iVar11) break;
          uVar14 = (uVar14 - (unsigned int)iVar9) + (unsigned int)iVar11;
          uVar4 = uVar4 << (((int)(char)DAT_009a6fe0 - (int)(char)DAT_00a08108) & 0x1f);
          iVar11 = iVar9;
        }
        if ((int)DAT_00a07ce4 <= iVar5) break;
        if ((int)DAT_00a0813c <= iVar5) {
          if (((int)DAT_009d8c00 < (int)((unsigned int)iVar11 + uVar14)) &&
              (uVar14 = (unsigned int)((int)DAT_009d8c00 - iVar11), (int)uVar14 < 1)) break;
          iVar9 = (int)FUN_0040e190(iVar5);
          uVar12 = (unsigned int)iVar11 + (unsigned int)iVar9;
          if ((DAT_005233ec != 0x13) && (DAT_005233f4 == 0xa0000)) {
            uVar12 = uVar12 % DAT_005233cc;
            FUN_00410ff0();
          }
          puVar10 = (unsigned char *)(uVar12 + DAT_005233f4);
          while (uVar14 != 0) {
            uVar14 = uVar14 - 1;
            uVar7 = DAT_009a807a;
            if ((((uVar4 & 0x80000000) == 0) && (uVar7 = DAT_00a08100, DAT_009ec464 != '\0')) &&
                ((uVar15 & 0x80000000) != 0)) {
              uVar7 = DAT_009e8f52;
            }
            *puVar10 = uVar7;
            puVar10 = puVar10 + 1;
            uVar4 = uVar4 * 2;
            uVar15 = uVar15 * 2;
          }
        }
        puVar6 = puStack00000004 + 8;
        iVar5 = (int)((unsigned int)iVar5 + 1u);
        __builtin_memcpy(&uTmp, puStack00000004, 4);
        uVar15 = uTmp >> 1;
        puStack00000004 = puVar6;
      } while (iVar5 < (int)((unsigned int)(int)DAT_00a08102 + uStack00000010));
    }
  }
  else {
    iVar5 = (int)(uVar4 + DAT_005233f4);
    iVar11 = (int)DAT_00a08102;
    uStack00000010 = (unsigned int)bVar2;
    if (iVar11 < (int)(uStack00000010 + (unsigned int)iVar11)) {
      do {
        __builtin_memcpy(&uTmp, puStack00000004, 4);
        uVar4 = uTmp;
        uVar14 = uStack0000000c;
        if (DAT_009ec464 != '\0') {
          uVar14 = uStack0000000c + 1;
        }
        iVar9 = (int)DAT_00a08108;
        iVar13 = (int)DAT_009a6fe0;
        if (iVar9 < iVar13) {
          if ((int)uVar14 <= iVar13 - iVar9) break;
          uVar14 = (uVar14 - (unsigned int)iVar13) + (unsigned int)iVar9;
          uVar4 = uVar4 << (((int)(char)DAT_009a6fe0 - (int)(char)DAT_00a08108) & 0x1f);
          iVar9 = iVar13;
        }
        if ((int)DAT_00a07ce4 <= iVar11) break;
        if ((int)DAT_00a0813c <= iVar11) {
          if (((int)DAT_009d8c00 < (int)((unsigned int)iVar9 + uVar14)) &&
              (uVar14 = (unsigned int)((int)DAT_009d8c00 - iVar9), (int)uVar14 < 1)) break;
          puVar10 = (unsigned char *)((unsigned int)iVar9 + (unsigned int)iVar5);
          while (uVar14 != 0) {
            uVar14 = uVar14 - 1;
            uVar7 = DAT_009a807a;
            if ((((uVar4 & 0x80000000) == 0) && (uVar7 = DAT_00a08100, DAT_009ec464 != '\0')) &&
                ((uVar15 & 0x80000000) != 0)) {
              uVar7 = DAT_009e8f52;
            }
            *puVar10 = uVar7;
            puVar10 = puVar10 + 1;
            uVar4 = uVar4 * 2;
            uVar15 = uVar15 * 2;
          }
          iVar9 = FUN_0040e1a0();
          iVar5 = (int)((unsigned int)iVar5 + (unsigned int)iVar9);
        }
        iVar11 = (int)((unsigned int)iVar11 + 1u);
        puVar6 = puStack00000004 + 8;
        __builtin_memcpy(&uTmp, puStack00000004, 4);
        uVar15 = uTmp >> 1;
        puStack00000004 = puVar6;
      } while (iVar11 < (int)((unsigned int)(int)DAT_00a08102 + uStack00000010));
    }
  }
  DAT_00a08108 = (unsigned short)(DAT_00a08108 + (unsigned short)bVar1);
  if ((DAT_009d8c00 <= DAT_00a08108) && (DAT_009cd270 != 0)) {
    if (DAT_009fe7e4 != 0) {
      FUN_00410110();
    }
    DAT_00a08108 = DAT_009a6fe0;
    DAT_00a08102 = (unsigned short)(DAT_00a08102 + bVar2 + 2);
  }
  return;
}
