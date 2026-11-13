/* SPDX-License-Identifier: MIT */
/* z_xvt 0042c5c0 */
extern int _DAT_005373d0;
extern int _DAT_005373d4;
extern int DAT_005373d0;
extern int DAT_00732920;

void __cdecl FUN_004310c0(unsigned int *, int);

void __cdecl FUN_0042c5c0(int param_1)
{
    _DAT_005373d0 = 3;
    _DAT_005373d4 = param_1;
    FUN_004310c0((unsigned int *)&DAT_005373d0, 8);
    (&DAT_00732920)[param_1] = 0;
    return;
}
