/* SPDX-License-Identifier: MIT */
/* z_bop 004d3520 */
extern uint DAT_00aa7fd3;
DWORD __stdcall GetTickCount(void);
uint __cdecl FUN_004d34a0(uint param_1);

uint __cdecl FUN_004d3520(uint param_1, uint param_2, int param_3)
{
  uint uVar1;
  DWORD DVar2;
  DWORD DVar3;
  uint uVar4;
  bool bVar5;

  if (DAT_00aa7fd3 == 0) {
    return 0;
  }
  bVar5 = param_1 <= param_2;
  if (param_2 != param_1) {
    if (bVar5) {
      uVar4 = param_2 - param_1;
    }
    else {
      uVar4 = param_1 - param_2;
    }
    DVar2 = GetTickCount();
    do {
      while (1) {
        DVar3 = GetTickCount();
        if ((int)((uint)(param_3 << 8) / uVar4 + DVar2) < (int)DVar3) {
          if (bVar5) {
            uVar1 = param_1 + 0x100;
            param_1 = 0xFFFF;
            if (uVar1 < 0x10000) {
              param_1 = uVar1;
            }
          }
          else if (param_1 < 0x100) {
            param_1 = 0;
          }
          else {
            param_1 = param_1 - 0x100;
          }
          FUN_004d34a0(param_1);
          DVar2 = DVar3;
        }
        if (!bVar5) break;
        if (param_2 <= param_1) {
          return 1;
        }
      }
    } while (param_2 < param_1);
    return 1;
  }
  return 1;
}
