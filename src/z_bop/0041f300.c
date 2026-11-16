/* SPDX-License-Identifier: MIT */
/* z_bop 0041f300 */
extern uint DAT_009a1fe8;
extern int DAT_009ecc34;

uint __cdecl FUN_00425d10(void);
void __cdecl FUN_0042e4d0(uint, uint, int);

void __cdecl FUN_0041f300(uint param_1, unsigned char param_2)
{
    uint uVar1;
    int iVar2;
    uint ptr_val;
    unsigned short zero_s = 0;

    iVar2 = param_1 * 0x23;
    *(unsigned char *)((unsigned char *)DAT_009a1fe8 + 3 + iVar2) = param_2;
    *(unsigned char *)((unsigned char *)DAT_009a1fe8 + 2 + iVar2) = 0xd;

    __builtin_memcpy(&ptr_val, (void *)((unsigned char *)DAT_009a1fe8 + 0x1f + iVar2), 4);
    *(unsigned char *)(ptr_val) = 5;

    *(unsigned char *)((unsigned char *)DAT_009a1fe8 + 0x19 + iVar2) = 2;

    __builtin_memcpy(&ptr_val, (void *)((unsigned char *)DAT_009a1fe8 + 0x1f + iVar2), 4);
    *(unsigned char *)(ptr_val + 1) = 0;

    __builtin_memcpy(&ptr_val, (void *)((unsigned char *)DAT_009a1fe8 + 0x1f + iVar2), 4);
    __builtin_memcpy((void *)(ptr_val + 0x79), &zero_s, 2);

    __builtin_memcpy(&ptr_val, (void *)((unsigned char *)DAT_009a1fe8 + 0x1f + iVar2), 4);
    __builtin_memcpy((void *)(ptr_val + 0x83), &zero_s, 2);

    __builtin_memcpy(&ptr_val, (void *)((unsigned char *)DAT_009a1fe8 + 0x1f + iVar2), 4);
    __builtin_memcpy((void *)(ptr_val + 0x81), &zero_s, 2);

    __builtin_memcpy((void *)((unsigned char *)DAT_009a1fe8 + 0x14 + iVar2), &zero_s, 2);

    __builtin_memcpy(&ptr_val, (void *)((unsigned char *)DAT_009a1fe8 + 0x1f + iVar2), 4);
    __builtin_memcpy((void *)(ptr_val + 0x77), &zero_s, 2);

    __builtin_memcpy(&ptr_val, (void *)((unsigned char *)DAT_009a1fe8 + 0x1f + iVar2), 4);
    *(unsigned char *)(ptr_val + 0x92) = 1;

    iVar2 = DAT_009ecc34;
    uVar1 = FUN_00425d10();
    FUN_0042e4d0((uVar1 & 3) + 0x16, param_1, iVar2);
    return;
}
