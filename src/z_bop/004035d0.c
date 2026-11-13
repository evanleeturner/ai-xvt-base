/* SPDX-License-Identifier: MIT */
/* z_bop 004035d0 */
extern void __cdecl FUN_00427bd0(short param_1, short param_2, int param_3);
extern void __cdecl FUN_00427d30(short param_1, short param_2, int param_3);
extern int DAT_009d8a58;
extern int DAT_009d8a54;
extern int DAT_009d8b60;

void __cdecl FUN_004035d0(int param_1, short param_2, short param_3, short param_4)
{
    int ptr_base;
    __builtin_memcpy(&ptr_base, (void*)((char*)param_1 + 0x1f), 4);

    if (*(char*)((char*)ptr_base + 0x92) != '\0') {
        short v10, v11;
        __builtin_memcpy(&v10, (void*)((char*)param_1 + 0x12), 2);
        __builtin_memcpy(&v11, (void*)((char*)param_1 + 0x10), 2);
        FUN_00427bd0(v10, v11, param_1);
        short v12;
        __builtin_memcpy(&v12, (void*)((char*)param_1 + 0x14), 2);
        FUN_00427d30(v12, 0, param_1);
    }

    int v2 = (int)param_2;
    int v3 = (int)param_3;
    int v4 = (int)param_4;

    short s99, s9f, s93;
    __builtin_memcpy(&s99, (void*)((char*)ptr_base + 0x99), 2);
    __builtin_memcpy(&s9f, (void*)((char*)ptr_base + 0x9f), 2);
    __builtin_memcpy(&s93, (void*)((char*)ptr_base + 0x93), 2);

    long long lVar1 = (long long)v2 * (long long)(int)s99;
    long long lVar2 = (long long)v3 * (long long)(int)s9f;
    long long lVar3 = (long long)v4 * (long long)(int)s93;

    DAT_009d8a58 = ((unsigned int)lVar2 >> 0xf | (int)((unsigned long long)lVar2 >> 0x20) << 0x11) +
                   ((unsigned int)lVar1 >> 0xf | (int)((unsigned long long)lVar1 >> 0x20) << 0x11) +
                   ((unsigned int)lVar3 >> 0xf | (int)((unsigned long long)lVar3 >> 0x20) << 0x11);

    short s9b, sa1, s95;
    __builtin_memcpy(&s9b, (void*)((char*)ptr_base + 0x9b), 2);
    __builtin_memcpy(&sa1, (void*)((char*)ptr_base + 0xa1), 2);
    __builtin_memcpy(&s95, (void*)((char*)ptr_base + 0x95), 2);

    lVar1 = (long long)v2 * (long long)(int)s9b;
    lVar2 = (long long)v3 * (long long)(int)sa1;
    lVar3 = (long long)v4 * (long long)(int)s95;

    DAT_009d8a54 = ((unsigned int)lVar2 >> 0xf | (int)((unsigned long long)lVar2 >> 0x20) << 0x11) +
                   ((unsigned int)lVar1 >> 0xf | (int)((unsigned long long)lVar1 >> 0x20) << 0x11) +
                   ((unsigned int)lVar3 >> 0xf | (int)((unsigned long long)lVar3 >> 0x20) << 0x11);

    short s9d, sa3, s97;
    __builtin_memcpy(&s9d, (void*)((char*)ptr_base + 0x9d), 2);
    __builtin_memcpy(&sa3, (void*)((char*)ptr_base + 0xa3), 2);
    __builtin_memcpy(&s97, (void*)((char*)ptr_base + 0x97), 2);

    lVar1 = (long long)v2 * (long long)(int)s9d;
    lVar2 = (long long)v3 * (long long)(int)sa3;
    lVar3 = (long long)v4 * (long long)(int)s97;

    DAT_009d8b60 = ((unsigned int)lVar2 >> 0xf | (int)((unsigned long long)lVar2 >> 0x20) << 0x11) +
                   ((unsigned int)lVar1 >> 0xf | (int)((unsigned long long)lVar1 >> 0x20) << 0x11) +
                   ((unsigned int)lVar3 >> 0xf | (int)((unsigned long long)lVar3 >> 0x20) << 0x11);
}
