/* SPDX-License-Identifier: MIT */
/* z_bop 00444e60 */
extern uint DAT_0055692c;

void __cdecl FUN_00444e60(byte *param_1);
void __cdecl FUN_00444eb0(byte *param_1);

void __cdecl FUN_00444e60(byte *param_1)
{
  int iVar1;
  byte *ptr;
  
  if (param_1[2] != 0) {
    iVar1 = 0;
    do {
      if ((1 << ((byte)iVar1 & 0x1f) & (uint)param_1[2]) != 0) {
        __builtin_memcpy(&ptr, param_1 + iVar1 * 4 + 0x32, 4);
        FUN_00444e60(ptr);
      }
      iVar1 = iVar1 + 1;
    } while (iVar1 < 8);
  }
  if (param_1[1] == DAT_0055692c) {
    FUN_00444eb0(param_1);
  }
  return;
}
