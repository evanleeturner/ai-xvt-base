/* SPDX-License-Identifier: MIT */
/* z_xvt 00444160 */
void __cdecl FUN_00421c70(uint param_1, uint param_2);
void __cdecl FUN_00418cd0(int param_1, int param_2, int param_3);
void __cdecl FUN_0042a140(uint param_1, uint param_2);

extern int DAT_0071b730;
extern int DAT_0071bb1b;
extern int DAT_0071bb1f;
extern int DAT_0071bb23;
extern int DAT_00732a24;
extern int DAT_00732a28;
extern int DAT_00732a2c;

void __cdecl FUN_00444160(int param_1, uint param_2)
{
  uint uVar1;
  int iVar2;
  
  iVar2 = param_1 * 0x5bd;
  uVar1 = *(uint *)((int)&DAT_0071b730 + iVar2);
  if (uVar1 == 0xffffffff) {
    FUN_00421c70(param_2, 0);
    FUN_00418cd0(DAT_00732a2c - *(int *)((int)&DAT_0071bb1b + iVar2),
                 DAT_00732a28 - *(int *)((int)&DAT_0071bb1f + iVar2),
                 DAT_00732a24 - *(int *)((int)&DAT_0071bb23 + iVar2));
    return;
  }
  FUN_0042a140(uVar1, param_2);
  return;
}
