/* SPDX-License-Identifier: MIT */
/* z_xvt 004340e0 */
extern int DAT_006d4988;
extern int DAT_0072f44c;
extern int DAT_0072f450;
extern int DAT_0072f448;
extern int DAT_006db7b4;
extern int DAT_0071bb1b;
extern int DAT_0071bb1f;
extern int DAT_0071bb23;
extern int DAT_006db7ac;
extern int DAT_006db7b0;

extern int FUN_0040ca60(int, int, int);
extern int FUN_0040ca00(int, int, int);
extern int FUN_0040ca30(int, int, int);

bool __cdecl FUN_004340e0(int param_1, unsigned int param_2, int param_3)
{
    int iVar1;
    int iVar2;
    int local_1b;
    int local_1f;
    int local_23;

    iVar2 = param_1 * 0x23 + DAT_006d4988;
    __builtin_memcpy(&local_1b, (const char *)&DAT_0071bb1b + param_3 * 0x5bd, 4);
    __builtin_memcpy(&local_1f, (const char *)&DAT_0071bb1f + param_3 * 0x5bd, 4);
    __builtin_memcpy(&local_23, (const char *)&DAT_0071bb23 + param_3 * 0x5bd, 4);
    DAT_0072f44c = *(int *)(iVar2 + 4) - local_1b;
    DAT_0072f450 = *(int *)(iVar2 + 8) - local_1f;
    DAT_0072f448 = *(int *)(iVar2 + 0xc) - local_23;
    DAT_006db7b4 = FUN_0040ca60(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
    iVar2 = DAT_006db7b4 + param_2;
    if (iVar2 < 0) {
        return 0;
    }
    if (param_2 < (unsigned int)(iVar2 >> 4)) {
        param_2 = iVar2 >> 4;
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
