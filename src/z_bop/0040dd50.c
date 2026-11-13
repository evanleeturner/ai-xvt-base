/* SPDX-License-Identifier: MIT */
/* z_bop 0040dd50 */
extern int DAT_005233c4;
void FUN_004079e0(void);

void __cdecl FUN_0040dd50(ushort *param_1)
{
  ushort uVar1;
  uint uVar2;
  uint uVar3;
  int iVar4;
  ushort *puVar5;
  uint uVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  ushort *puVar10;
  int iStack_18;
  int iStack_14;
  int iStack_10;
  
  if (DAT_005233c4 != 2) {
    param_1[0x900] = 0;
    return;
  }
  iStack_18 = 0;
  iStack_14 = 0;
  iStack_10 = -1;
  puVar5 = param_1;
  do {
    uVar1 = *puVar5;
    uVar2 = uVar1 & 0x1f;
    uVar6 = (uint)(uVar1 >> 0xb);
    uVar3 = uVar1 >> 6 & 0x1f;
    if (uVar2 * uVar2 + uVar3 * uVar3 + uVar6 * uVar6 < 0x20) {
      *puVar5 = 0;
      iStack_18 = iStack_18 + 1;
      if (iStack_10 == -1) {
        iStack_10 = iStack_14;
      }
    }
    else {
      iVar7 = 1;
      puVar10 = puVar5;
      do {
        puVar10 = puVar10 + 0x100;
        uVar1 = *puVar10;
        iVar9 = (uVar1 & 0x1f) - uVar2;
        iVar4 = (uVar1 >> 6 & 0x1f) - uVar3;
        iVar8 = (uVar1 >> 0xb) - uVar6;
        if (0x10 < iVar9 * iVar9 + iVar4 * iVar4 + iVar8 * iVar8) {
          *puVar5 = 0;
          iStack_18 = iStack_18 + 1;
          if (iStack_10 == -1) {
            iStack_10 = iStack_14;
          }
          break;
        }
        iVar7 = iVar7 + 1;
      } while (iVar7 < 7);
      if (iVar7 == 7) {
        *puVar5 = puVar5[0xa00];
      }
    }
    puVar5 = puVar5 + 1;
    iStack_14 = iStack_14 + 1;
    if (0xff < iStack_14) {
      if (iStack_18 < 0x100) {
        FUN_004079e0();
      }
      else {
        FUN_004079e0();
        iStack_18 = 0;
      }
      param_1[0x100] = (ushort)iStack_10;
      param_1[0x900] = (ushort)iStack_18;
      return;
    }
  } while(1);
}
