/* SPDX-License-Identifier: MIT */
/* z_bop 00482eb0 */
extern ushort DAT_00a00860;
extern byte DAT_009e9689[];
extern uint DAT_009ecc34;
extern uint DAT_00a081ec;
extern uint DAT_005233ec;
extern uint DAT_009a7bac;
extern uint DAT_009a1fe8;
extern byte DAT_009e9683[];

void __cdecl FUN_004830d0(uint param_1, uint param_2, int *param_3, int *param_4, uint *param_5);
int __cdecl FUN_00483030(uint param_1);
int __cdecl FUN_004aedf0(uint param_1, uint param_2);
void __cdecl FUN_00437860(int param_1, int param_2, uint param_3, uint param_4, byte param_5);
void __cdecl FUN_00497e00(int param_1, int param_2, uint param_3, uint param_4, uint param_5, uint param_6);

void __cdecl FUN_00482eb0(ushort param_1, ushort param_2, byte param_3)
{
  int iVar1;
  int iVar2;
  uint uVar3;
  int iVar4;
  uint local_c;
  int local_8;
  int local_4;

  if (((param_1 != 0xffff) && (DAT_00a00860 == 0)) &&
     (DAT_009e9689[DAT_009ecc34 * 0x5bd] != '\0')) {
    uVar3 = (uint)param_1;
    FUN_004830d0(uVar3, (uint)param_2, &local_4, &local_8, &local_c);
    if (0 < (int)local_c) {
      if (param_2 == 0xffff) {
        iVar1 = FUN_00483030(uVar3);
      }
      else {
        iVar1 = FUN_004aedf0((uint)*(byte *)(uVar3 * 0x23 + 3 + DAT_009a1fe8), (uint)param_2);
      }
      iVar1 = (int)(((long long)iVar1 * (long long)DAT_00a081ec & 0xffffffffU) / (unsigned long long)local_c);
      if (DAT_005233ec == 0x13) {
        iVar4 = 4;
      }
      else {
        iVar4 = 8;
      }
      iVar2 = (DAT_009a7bac >> 2) + (DAT_009a7bac >> 1);
      if (iVar1 < iVar4) {
        iVar1 = iVar4;
      }
      if (iVar2 < iVar1) {
        iVar1 = iVar2;
      }
      uVar3 = iVar1 + 4;
      if (DAT_009e9683[DAT_009ecc34 * 0x5bd] != '\0') {
        FUN_00437860(local_4 - (int)uVar3 / 2, local_8 - (int)uVar3 / 2, uVar3, uVar3, param_3);
        return;
      }
      FUN_00497e00(local_4 - (int)uVar3 / 2, local_8 - (int)uVar3 / 2, uVar3, uVar3, (uint)param_3,
                   local_c);
    }
  }
  return;
}
