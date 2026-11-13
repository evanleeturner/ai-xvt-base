/* SPDX-License-Identifier: MIT */
/* z_bop 004021a0 */
extern unsigned int DAT_00a90a60;
extern unsigned short _DAT_00a90a5e;
extern unsigned int DAT_009a1fe8;
extern unsigned int DAT_009a1fe0;

void __cdecl FUN_00458780(int param_1);

void __cdecl FUN_004021a0(unsigned int param_1)
{
  unsigned short uVar1;
  int iVar2;
  
  if (DAT_00a90a60 != 0) {
    _DAT_00a90a5e = _DAT_00a90a5e + 1;
    uVar1 = *(unsigned short *)(DAT_00a90a60 + (unsigned int)_DAT_00a90a5e * 2);
    if (uVar1 == 0xffff) {
      *(unsigned char *)(DAT_009a1fe8 + 3 + param_1 * 0x23) = 0;
      if ((param_1 < DAT_009a1fe0) &&
         (iVar2 = *(int *)(*(int *)(DAT_009a1fe8 + 0x1f + param_1 * 0x23) + 0xa9), iVar2 != 0)) {
        FUN_00458780(iVar2);
        return;
      }
    }
    else {
      if (uVar1 == 0xfffd) {
        _DAT_00a90a5e = 0;
        return;
      }
      if ((0xfeff < uVar1) && (uVar1 != 0xfffe)) {
        _DAT_00a90a5e = uVar1 + 0x100;
      }
    }
  }
  return;
}
