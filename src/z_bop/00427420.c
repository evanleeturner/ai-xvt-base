/* SPDX-License-Identifier: MIT */
/* z_bop 00427420 */
extern int DAT_00a00488;
extern int DAT_00a004a8;
extern int DAT_00a004cc;

int __cdecl FUN_00427420(int param_1, int param_2, int param_3)
{
  long long prod1 = (long long)DAT_00a004a8 * (long long)param_1;
  unsigned int term1 = ((unsigned int)prod1 >> 15) | ((unsigned int)(prod1 >> 32) << 17);

  long long prod2 = (long long)DAT_00a004cc * (long long)param_2;
  unsigned int term2 = ((unsigned int)prod2 >> 15) | ((unsigned int)(prod2 >> 32) << 17);

  unsigned int sum = term1 + term2;

  long long prod3 = (long long)DAT_00a00488 * (long long)param_3;
  unsigned int term3 = ((unsigned int)prod3 >> 15) | ((unsigned int)(prod3 >> 32) << 17);

  return (int)(term3 + sum);
}
