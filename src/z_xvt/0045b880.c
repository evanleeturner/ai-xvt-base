/* SPDX-License-Identifier: MIT */
/* z_xvt 0045b880 */
extern int DAT_006d4988;
extern int DAT_0071ecf4;
extern int DAT_0071bb1b;
extern int DAT_0071bb1f;
extern int DAT_0071bb23;
extern int DAT_0072f44c;
extern int DAT_0072f450;
extern int DAT_0072f448;
extern int DAT_006db7b4;
extern int DAT_006db7ac;
extern int DAT_006db7b0;

extern int __cdecl FUN_0040ca60(int, int, int);
extern int __cdecl FUN_0040ca00(int, int, int);
extern int __cdecl FUN_0040ca30(int, int, int);

int __cdecl FUN_0045b880(int param_1, unsigned int param_2)
{
  int iVar1;
  int iVar2;
  
  iVar2 = param_1 * 0x23 + DAT_006d4988;
  DAT_0072f44c = *(int *)(iVar2 + 4) - *(int *)(&DAT_0071bb1b + DAT_0071ecf4 * 0x5bd);
  DAT_0072f450 = *(int *)(iVar2 + 8) - *(int *)(&DAT_0071bb1f + DAT_0071ecf4 * 0x5bd);
  DAT_0072f448 = *(int *)(iVar2 + 0xc) - *(int *)(&DAT_0071bb23 + DAT_0071ecf4 * 0x5bd);
  DAT_006db7b4 = FUN_0040ca60(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
  iVar2 = DAT_006db7b4 + param_2;
  if (iVar2 < 0) {
    return 0;
  }
  if (param_2 < (unsigned int)(iVar2 >> 8)) {
    return 0;
  }
  DAT_006db7ac = FUN_0040ca00(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
  iVar1 = DAT_006db7ac;
  if (DAT_006db7ac < 0) {
    iVar1 = -DAT_006db7ac;
  }
  if (iVar2 < (int)(iVar1 - param_2)) {
    return 0;
  }
  iVar1 = FUN_0040ca30(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
  DAT_006db7b0 = iVar1;
  if (iVar1 < 0) {
    iVar1 = -iVar1;
  }
  return (int)(iVar1 - param_2) <= iVar2;
}
