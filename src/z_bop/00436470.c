/* SPDX-License-Identifier: MIT */
/* z_bop 00436470 */
extern int DAT_009a1fe8;
extern int DAT_009fd39c;
extern int DAT_009e9a5b;
extern int DAT_009fd3a0;
extern int DAT_009e9a5f;
extern int DAT_009fd398;
extern int DAT_009e9a63;
extern int DAT_009a8e34;
extern int DAT_009a8e2c;
extern int DAT_009a8e30;

int __cdecl FUN_00427500(int param_1, int param_2, int param_3);
int __cdecl FUN_00427420(int param_1, int param_2, int param_3);
int __cdecl FUN_00427490(int param_1, int param_2, int param_3);

int __cdecl FUN_00436470(int param_1, unsigned int param_2, int param_3)
{
  int iVar1;
  int iVar2;
  
  iVar2 = param_1 * 0x23 + DAT_009a1fe8;
  DAT_009fd39c = *(int *)(iVar2 + 4) - *(int *)(&DAT_009e9a5b + param_3 * 0x5bd);
  DAT_009fd3a0 = *(int *)(iVar2 + 8) - *(int *)(&DAT_009e9a5f + param_3 * 0x5bd);
  DAT_009fd398 = *(int *)(iVar2 + 0xc) - *(int *)(&DAT_009e9a63 + param_3 * 0x5bd);
  DAT_009a8e34 = FUN_00427500(DAT_009fd39c, DAT_009fd3a0, DAT_009fd398);
  iVar2 = DAT_009a8e34 + param_2;
  if (iVar2 < 0) {
    return 0;
  }
  if (param_2 < (unsigned int)(iVar2 >> 4)) {
    param_2 = iVar2 >> 4;
  }
  DAT_009a8e2c = FUN_00427420(DAT_009fd39c, DAT_009fd3a0, DAT_009fd398);
  iVar1 = DAT_009a8e2c;
  if (DAT_009a8e2c < 0) {
    iVar1 = -DAT_009a8e2c;
  }
  if (iVar2 < (int)(iVar1 - param_2)) {
    return 0;
  }
  iVar1 = FUN_00427490(DAT_009fd39c, DAT_009fd3a0, DAT_009fd398);
  DAT_009a8e30 = iVar1;
  if (iVar1 < 0) {
    iVar1 = -iVar1;
  }
  return (int)(iVar1 - param_2) <= iVar2;
}
