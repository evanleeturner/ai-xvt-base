/* SPDX-License-Identifier: MIT */
/* z_bop 004b3380 */
extern void __stdcall FUN_004079e0(void);
extern int DAT_00528c58;
extern int DAT_00a91120;

unsigned int __cdecl FUN_004b3380(int *param_1)
{
  int *piVar1;
  unsigned int uVar2;
  unsigned int uVar3;
  unsigned int uVar4;
  unsigned int local_4;
  
  if (DAT_00528c58 == 0) {
    return 0;
  }
  uVar4 = 0;
  piVar1 = &DAT_00a91120;
  uVar3 = 0;
  local_4 = 0;
  if (DAT_00528c58 != 0) {
    do {
      uVar2 = 0;
      if ((param_1[1] == 0) || (piVar1[1] == param_1[1])) {
        uVar2 = 1;
        if (((param_1[2] == 0) || (piVar1[2] == param_1[2])) &&
           ((uVar2 = 2, (piVar1[9] & param_1[9]) != 0 && (uVar2 = 3, *piVar1 == *param_1)))) {
          FUN_004079e0();
          return uVar3;
        }
      }
      if (uVar4 < uVar2) {
        uVar4 = uVar2;
        local_4 = uVar3;
      }
      piVar1 = piVar1 + 0x8b;
      uVar3 = uVar3 + 1;
    } while (uVar3 < (unsigned int)DAT_00528c58);
  }
  FUN_004079e0();
  return local_4;
}
