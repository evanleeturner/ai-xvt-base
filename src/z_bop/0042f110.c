/* SPDX-License-Identifier: MIT */
/* z_bop 0042f110 */
extern int DAT_00523410;
extern char DAT_009d80c9;
extern char DAT_00bb7372;
extern char DAT_00bb737b;
extern int DAT_009e9670;
extern int DAT_009ecc34;
extern int DAT_009a1fe8;
extern char DAT_009e9681;
extern char DAT_00a0afb8;

uint __cdecl FUN_004a9300(int param_1, int param_2);
bool __cdecl FUN_004a9380(int param_1);
undefined4 __cdecl FUN_0042d120(char * param_1, undefined4 param_2, undefined4 param_3, int param_4, undefined4 param_5, undefined4 param_6);

void __stdcall FUN_0042f110(void)
{
  int iVar1;
  uint uVar2;
  int iVar3;
  
  if ((((DAT_00523410 == 0) && (DAT_009d80c9 != '\0')) && (DAT_00bb7372 != '\0')) &&
     (DAT_00bb737b != 0)) {
    iVar1 = *(int *)((int)&DAT_009e9670 + DAT_009ecc34 * 0x5bd);
    if (iVar1 == -1) {
      uVar2 = FUN_004a9300(0x13, 0x100);
      if (uVar2 != 0) {
        FUN_004a9380(0x13);
        return;
      }
    }
    else {
      iVar1 = *(int *)(*(int *)(iVar1 * 0x23 + 0x1f + DAT_009a1fe8) + 0xa9);
      if (*(char *)(iVar1 + 0x246) == '\x01') {
        if ((&DAT_009e9681)[DAT_009ecc34 * 0x5bd] == '\x01') {
          uVar2 = FUN_004a9300(0x13, 0x100);
          if (uVar2 != 0) {
            FUN_004a9380(0x13);
            return;
          }
        }
        else {
          iVar3 = 0x7f;
          if (DAT_00bb737b < 10) {
            iVar3 = (uint)DAT_00bb737b * 0xd;
          }
          if (*(short *)(iVar1 + 0x248) == 0) {
            uVar2 = FUN_004a9300(0x13, 0x100);
            if (uVar2 != 0) {
              FUN_004a9380(0x13);
            }
          }
          else {
            uVar2 = FUN_004a9300(0x13, 0x100);
            if (uVar2 == 0) {
              FUN_0042d120(&DAT_00a0afb8, 1, 1, 0x7d, iVar3 >> 2, 0x40);
              return;
            }
          }
        }
      }
    }
  }
  return;
}
