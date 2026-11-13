/* SPDX-License-Identifier: MIT */
/* z_bop 00427bd0 */
long long __cdecl FUN_0046a6d0(short param_1);
long long __cdecl FUN_0046a3c0(short param_1);

extern int DAT_00a0048c;
extern int DAT_00a00494;
extern int DAT_00a07c68;
extern int DAT_00a07c6c;
extern int DAT_00a07cc4;
extern int DAT_00a004d0;
extern int DAT_00a004b4;
extern int DAT_00a004ac;
extern int DAT_009a8da4;
extern int DAT_009a8d88;
extern int DAT_009a8d84;
extern int DAT_00a0049c;

void __cdecl FUN_00427bd0(short param_1, short param_2, int param_3)
{
    int iVar1;
    int iVar2;
    long long lVar3;
    long long lVar4;
    long long lVar5;
    int ptr_val;

    lVar3 = FUN_0046a6d0(-param_2);
    lVar4 = FUN_0046a6d0(-0x4000 - param_1);
    lVar5 = FUN_0046a3c0(-param_2);
    iVar2 = (int)(short)lVar3;
    lVar3 = FUN_0046a3c0(-0x4000 - param_1);
    DAT_00a0048c = (int)(short)lVar5;
    DAT_00a00494 = 0;
    iVar1 = (int)(short)lVar4;
    DAT_00a07c68 = (unsigned int)((long long)-DAT_00a0048c * (long long)iVar1) >> 0xf |
                  (int)((unsigned long long)((long long)-DAT_00a0048c * (long long)iVar1) >> 0x20) << 0x11;
    DAT_00a07c6c = (unsigned int)((long long)iVar2 * (long long)iVar1) >> 0xf |
                  (int)((unsigned long long)((long long)iVar2 * (long long)iVar1) >> 0x20) << 0x11;
    DAT_00a07cc4 = (int)(short)lVar3;
    DAT_00a004d0 = -((unsigned int)((long long)DAT_00a0048c * (long long)DAT_00a07cc4) >> 0xf |
                    (int)((unsigned long long)((long long)DAT_00a0048c * (long long)DAT_00a07cc4) >> 0x20) << 0x11);
    DAT_00a004b4 = -iVar1;
    DAT_00a004ac = -((unsigned int)((long long)-iVar2 * (long long)DAT_00a07cc4) >> 0xf |
                    (int)((unsigned long long)((long long)-iVar2 * (long long)DAT_00a07cc4) >> 0x20) << 0x11);
    DAT_009a8da4 = -DAT_00a07c68;
    DAT_009a8d88 = -DAT_00a07cc4;
    DAT_009a8d84 = -DAT_00a07c6c;
    DAT_00a0049c = iVar2;

    __builtin_memcpy(&ptr_val, (const void *)(param_3 + 0x1f), sizeof(int));
    if (param_3 != 0) {
        *(short *)(ptr_val + 0x8c) = (short)DAT_009a8da4;
        *(short *)(ptr_val + 0x8e) = (short)DAT_009a8d84;
        *(short *)(ptr_val + 0x90) = (short)DAT_009a8d88;
        *(unsigned char *)(ptr_val + 0x8b) = 0;
    }
    return;
}
