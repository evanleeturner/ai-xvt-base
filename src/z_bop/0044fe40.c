/* SPDX-License-Identifier: MIT */
/* z_bop 0044fe40 */
extern int DAT_009a1fe8;
extern int DAT_009fd39c;
extern int DAT_009fd3a0;
extern int DAT_009fd398;
extern int DAT_009a8e2c;
extern int DAT_009a8e30;
extern int DAT_009a8e34;
extern int DAT_009e9a5b;
extern int DAT_009e9a5f;
extern int DAT_009e9a63;
extern int DAT_009ecc34;

int __cdecl FUN_00427420(int param_1, int param_2, int param_3);
int __cdecl FUN_00427490(int param_1, int param_2, int param_3);
int __cdecl FUN_00427500(int param_1, int param_2, int param_3);

void __cdecl FUN_0044fe40(unsigned int param_1)
{
  int iVar1;
  int src1, src2, src3;

  iVar1 = (param_1 & 0xffff) * 0x23 + DAT_009a1fe8;
  
  __builtin_memcpy(&src1, (char *)&DAT_009e9a5b + DAT_009ecc34 * 0x5bd, 4);
  __builtin_memcpy(&src2, (char *)&DAT_009e9a5f + DAT_009ecc34 * 0x5bd, 4);
  __builtin_memcpy(&src3, (char *)&DAT_009e9a63 + DAT_009ecc34 * 0x5bd, 4);
  
  DAT_009fd39c = *(int *)(iVar1 + 4) - src1;
  DAT_009fd3a0 = *(int *)(iVar1 + 8) - src2;
  DAT_009fd398 = *(int *)(iVar1 + 0xc) - src3;
  
  DAT_009a8e2c = FUN_00427420(DAT_009fd39c, DAT_009fd3a0, DAT_009fd398);
  DAT_009a8e30 = FUN_00427490(DAT_009fd39c, DAT_009fd3a0, DAT_009fd398);
  DAT_009a8e34 = FUN_00427500(DAT_009fd39c, DAT_009fd3a0, DAT_009fd398);
  return;
}
