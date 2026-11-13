/* SPDX-License-Identifier: MIT */
/* z_bop 00452880 */
extern int DAT_009e9670;
extern int DAT_009ecc34;
extern unsigned short DAT_00a08292;
extern int DAT_0051c560;
extern int DAT_009a1fe8;
extern int DAT_009d8c30;
extern unsigned short DAT_00a07be4;

void __cdecl FUN_00451bf0(int param_1, int param_2);
unsigned int __cdecl FUN_0043c740(int param_1, unsigned int * param_2);
void __cdecl FUN_00450650(int param_1, int param_2);

void __cdecl FUN_00452880(int param_1)
{
  int iVar1;
  unsigned int uVar2;
  
  iVar1 = *(int *)((int)&DAT_009e9670 + DAT_009ecc34 * 0x5bd) * 0x23;
  DAT_00a08292 = (unsigned short)*(unsigned char *)(*(int *)((int)DAT_009a1fe8 + 0x1f + iVar1) + 0x88);
  FUN_00451bf0(0, (int)&DAT_0051c560 + (unsigned int)*(unsigned char *)(*(int *)(*(int *)((int)DAT_009a1fe8 + 0x1f + iVar1) + 0xa9) + 4) * 0x101);
  FUN_00451bf0(1, (int)&DAT_009d8c30 + (unsigned int)*(unsigned char *)((int)DAT_009a1fe8 + 0x16 + iVar1) * 0x566);
  uVar2 = FUN_0043c740((unsigned int)*(unsigned char *)((int)DAT_009a1fe8 + iVar1 + 0x16),
                       *(unsigned int **)(*(int *)((int)DAT_009a1fe8 + iVar1 + 0x1f) + 0xa9));
  DAT_00a07be4 = (unsigned short)uVar2;
  FUN_00450650(param_1, DAT_009ecc34);
  return;
}
