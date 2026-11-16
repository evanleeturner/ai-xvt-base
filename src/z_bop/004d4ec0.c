/* SPDX-License-Identifier: MIT */
/* z_bop 004d4ec0 */
extern undefined4 DAT_00aa7b2e;
extern undefined4 DAT_00aa7b32;
extern undefined4 DAT_00aa6cfc;
extern undefined4 DAT_00aa7b92;
extern undefined4 DAT_00aa7b1b;
extern undefined4 DAT_00aa7b13;
extern undefined4 DAT_00aa7b17;
extern undefined4 DAT_00aa7b0d;
extern undefined4 DAT_00aa7ac5;
extern undefined4 __cdecl FUN_004d48e0(void);

undefined4 __cdecl FUN_004d4ec0(int param_1)
{
  int iVar1;
  undefined4 *puVar2;
  undefined4 *puVar3;
  
  if (DAT_00aa7b2e == 0) {
    return 0;
  }
  if (DAT_00aa7b32 == 0) {
    return 0;
  }
  if (DAT_00aa6cfc != 0 && DAT_00aa7b92 != 0 && param_1 != 0) {
    puVar2 = (undefined4 *)DAT_00aa6cfc;
    puVar3 = (undefined4 *)DAT_00aa7b92;
    for (iVar1 = (DAT_00aa7b1b * 0xf & 0x7ffffffU) << 3; iVar1 != 0; iVar1--) {
      *puVar3 = *puVar2;
      puVar2 = puVar2 + 1;
      puVar3 = puVar3 + 1;
    }
  }
  ((void (__stdcall *)(void *, int))(*(void **)(void *)DAT_00aa7b32 + 0x80))((void *)DAT_00aa7b32, 0);
  DAT_00aa7b13 = DAT_00aa7b17;
  if (DAT_00aa7b0d != 0) {
    DAT_00aa6cfc = DAT_00aa7ac5;
    return 1;
  }
  DAT_00aa6cfc = FUN_004d48e0();
  return 1;
}
