/* SPDX-License-Identifier: MIT */
/* z_bop 0040f05b */
extern ushort DAT_009fe7e4;
extern ushort DAT_00a08108;
extern ushort DAT_009a6fe0;
extern ushort DAT_00a08102;
extern byte   DAT_009a20ae;
extern char   DAT_009d80c8;
extern char   DAT_009a1ff4;
extern ushort DAT_009d8c02;
extern uint   DAT_009d7674;
extern ushort DAT_009d8c00;
extern ushort DAT_009cd270;
extern ushort DAT_00a0813c;
extern ushort DAT_00a07ce4;
extern int    DAT_005233ec;
extern uint   DAT_005233f4;
extern uint   DAT_005233cc;
extern int    DAT_009d8c08;
extern char   DAT_009ec464;
extern byte   DAT_009a807a;
extern byte   DAT_00a08100;
extern byte   DAT_009e8f52;

void FUN_00410110(void);
int  FUN_0040e190(int param_1);
/* UNRESOLVED: two cdecl args pushed by the original (8 bytes); values not visible, callee is an empty RET */
void __cdecl FUN_00410ff0(int param_1, int param_2);
int  FUN_0040e1a0(void);

void __fastcall FUN_0040f05b(byte param_1)
{
  byte bVar1;
  byte bVar2;
  int bVar3;
  byte uVar4;
  int iVar5;
  byte *pbVar6;
  byte *puVar7;
  byte bVar8;
  uint uVar9;
  byte bVar10;
  int iVar11;
  uint uVar12;
  int iVar13;
  /* UNRESOLVED: ZF is an implicit register input from the caller's flags */
  volatile int in_ZF = 0;
  char cStack00000004;
  uint _cStack00000004;
  byte *pbStack00000008;
  uint uStack00000010;
  uint uStack00000014;

  if (in_ZF) {
    if (DAT_009fe7e4 != 0) {
      FUN_00410110();
    }
    DAT_00a08108 = DAT_009a6fe0;
    DAT_00a08102 = (ushort)((uint)DAT_00a08102 + (uint)(ushort)DAT_009a20ae);
    return;
  }
  if (param_1 < 0x20) {
    return;
  }
  if ((((DAT_009d80c8 != '\0') && (DAT_009a1ff4 == '\0')) && (0x60 < param_1)) && (param_1 < 0x7b)) {
    param_1 = (byte)(param_1 - 0x20);
  }
  pbStack00000008 = (byte *)((uint)(byte)(param_1 - 0x20) * (uint)DAT_009d8c02 + DAT_009d7674);
  bVar1 = *pbStack00000008;
  bVar2 = pbStack00000008[1];
  pbStack00000008 = pbStack00000008 + 2;
  uStack00000010 = (uint)(ushort)bVar1;
  if (((int)DAT_009d8c00 <= (int)((uint)DAT_00a08108 + uStack00000010)) && (DAT_009cd270 != 0)) {
    if (DAT_009fe7e4 != 0) {
      FUN_00410110();
    }
    DAT_00a08108 = DAT_009a6fe0;
    DAT_00a08102 = (ushort)((uint)DAT_00a08102 + (uint)bVar2 + 2u);
  }
  bVar10 = 0;
  bVar3 = 0;
  iVar13 = (int)DAT_00a08102;
  if ((int)DAT_00a08102 < (int)DAT_00a0813c) {
    iVar13 = (int)DAT_00a0813c;
  }
  _cStack00000004 = (uint)FUN_0040e190(iVar13);
  if (DAT_005233ec == 0x13) {
LAB_0040f1fe:
    if (DAT_005233f4 == 0xa0000) goto LAB_0040f20f;
  }
  else if (DAT_005233f4 == 0xa0000) {
    _cStack00000004 = _cStack00000004 % DAT_005233cc;
    FUN_00410ff0(0, 0);
    iVar5 = (int)((uint)DAT_00a08102 + 1u + (uint)bVar2);
    if ((int)DAT_00a07ce4 < iVar5) {
      iVar5 = (int)DAT_00a07ce4;
    }
    if (0xffff < (int)(_cStack00000004 + (uint)(int)((uint)iVar5 - (uint)iVar13) * (uint)DAT_009d8c08)) {
      bVar3 = 1;
    }
    goto LAB_0040f1fe;
  }
  bVar3 = 1;
LAB_0040f20f:
  if (bVar3) {
    iVar13 = (int)DAT_00a08102;
    uStack00000014 = (uint)bVar2;
    if (iVar13 < (int)(uStack00000014 + (uint)iVar13)) {
      do {
        bVar8 = *pbStack00000008;
        uVar12 = uStack00000010;
        if (DAT_009ec464 != '\0') {
          uVar12 = uStack00000010 + 1u;
        }
        _cStack00000004 = (uint)DAT_00a08108;
        uVar9 = (uint)DAT_009a6fe0;
        if ((int)_cStack00000004 < (int)uVar9) {
          if ((int)uVar12 <= (int)(uVar9 - _cStack00000004)) break;
          uVar12 = (uVar12 - uVar9) + _cStack00000004;
          cStack00000004 = (char)DAT_00a08108;
          bVar8 = (byte)((uint)bVar8 << (((char)DAT_009a6fe0 - cStack00000004) & 0x1f));
          _cStack00000004 = uVar9;
        }
        if ((int)DAT_00a07ce4 <= iVar13) break;
        if ((int)DAT_00a0813c <= iVar13) {
          if (((int)DAT_009d8c00 < (int)(_cStack00000004 + uVar12)) &&
              (uVar12 = (uint)DAT_009d8c00 - _cStack00000004, (int)uVar12 < 1)) break;
          iVar5 = FUN_0040e190(iVar13);
          uVar9 = _cStack00000004 + (uint)iVar5;
          if ((DAT_005233ec != 0x13) && (DAT_005233f4 == 0xa0000)) {
            uVar9 = uVar9 % DAT_005233cc;
            FUN_00410ff0(0, 0);
          }
          puVar7 = (byte *)(uVar9 + DAT_005233f4);
          while (uVar12 != 0) {
            uVar12 = uVar12 - 1u;
            uVar4 = DAT_009a807a;
            if ((((bVar8 & 0x80) == 0) && (uVar4 = DAT_00a08100, DAT_009ec464 != '\0')) &&
                ((bVar10 & 0x80) != 0)) {
              uVar4 = DAT_009e8f52;
            }
            *puVar7 = uVar4;
            puVar7 = puVar7 + 1;
            bVar10 = (byte)(bVar10 * 2);
            bVar8 = (byte)(bVar8 * 2);
          }
        }
        iVar13 = (int)((uint)iVar13 + 1u);
        pbVar6 = pbStack00000008 + 2;
        bVar10 = (byte)(*pbStack00000008 >> 1);
        pbStack00000008 = pbVar6;
      } while (iVar13 < (int)((uint)DAT_00a08102 + uStack00000014));
    }
  }
  else {
    _cStack00000004 = _cStack00000004 + DAT_005233f4;
    iVar13 = (int)DAT_00a08102;
    uStack00000014 = (uint)bVar2;
    if (iVar13 < (int)(uStack00000014 + (uint)iVar13)) {
      do {
        bVar8 = *pbStack00000008;
        uVar12 = uStack00000010;
        if (DAT_009ec464 != '\0') {
          uVar12 = uStack00000010 + 1u;
        }
        iVar5 = (int)DAT_00a08108;
        iVar11 = (int)DAT_009a6fe0;
        if (iVar5 < iVar11) {
          if ((int)uVar12 <= (int)((uint)iVar11 - (uint)iVar5)) break;
          uVar12 = (uVar12 - (uint)iVar11) + (uint)iVar5;
          bVar8 = (byte)((uint)bVar8 << (((char)DAT_009a6fe0 - (char)DAT_00a08108) & 0x1f));
          iVar5 = iVar11;
        }
        if ((int)DAT_00a07ce4 <= iVar13) break;
        if ((int)DAT_00a0813c <= iVar13) {
          if (((int)DAT_009d8c00 < (int)((uint)iVar5 + uVar12)) &&
              (uVar12 = (uint)DAT_009d8c00 - (uint)iVar5, (int)uVar12 < 1)) break;
          puVar7 = (byte *)(_cStack00000004 + (uint)iVar5);
          while (uVar12 != 0) {
            uVar12 = uVar12 - 1u;
            uVar4 = DAT_009a807a;
            if ((((bVar8 & 0x80) == 0) && (uVar4 = DAT_00a08100, DAT_009ec464 != '\0')) &&
                ((bVar10 & 0x80) != 0)) {
              uVar4 = DAT_009e8f52;
            }
            *puVar7 = uVar4;
            puVar7 = puVar7 + 1;
            bVar8 = (byte)(bVar8 * 2);
            bVar10 = (byte)(bVar10 * 2);
          }
          iVar5 = FUN_0040e1a0();
          _cStack00000004 = _cStack00000004 + (uint)iVar5;
        }
        iVar13 = (int)((uint)iVar13 + 1u);
        pbVar6 = pbStack00000008 + 2;
        bVar10 = (byte)(*pbStack00000008 >> 1);
        pbStack00000008 = pbVar6;
      } while (iVar13 < (int)((uint)DAT_00a08102 + uStack00000014));
    }
  }
  DAT_00a08108 = (ushort)((uint)DAT_00a08108 + (uint)(ushort)bVar1);
  if ((DAT_009d8c00 <= DAT_00a08108) && (DAT_009cd270 != 0)) {
    if (DAT_009fe7e4 != 0) {
      FUN_00410110();
    }
    DAT_00a08108 = DAT_009a6fe0;
    DAT_00a08102 = (ushort)((uint)DAT_00a08102 + (uint)bVar2 + 2u);
  }
  return;
}
