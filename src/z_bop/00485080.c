/* SPDX-License-Identifier: MIT */
/* z_bop 00485080 */
extern void __cdecl FUN_00427720(unsigned int param_1, int param_2);
extern void __cdecl FUN_0042f270(void);
extern void __cdecl FUN_0042f110(void);
extern void __cdecl FUN_0042f5d0(void);

extern char DAT_009e9680;
extern char DAT_009ec468;
extern char DAT_009e9683;
extern char DAT_009e9a80;
extern char DAT_009e9a84;
extern char DAT_009e9a86;
extern char DAT_009e9a67;
extern char DAT_009e9a69;
extern char DAT_009e9a63;
extern char DAT_009e968a;
extern int DAT_009a1fe8;
extern char DAT_009e9a5b;
extern char DAT_009e9a5f;
extern char DAT_00521f1c;
extern char DAT_009e9681;
extern char DAT_009e9670;
extern char DAT_009e9c1d;
extern char DAT_009e96b6;
extern char DAT_009d6940;

void __cdecl FUN_00485080(int param_1)
{
    int iVar1;
    int iVar2;
    char *pcVar3;

    pcVar3 = &DAT_009e9680;
    iVar1 = param_1 * 0x5bd;
    iVar2 = 0;
    (&DAT_009e9680)[iVar1] = 2;
    do {
        if (*pcVar3 == 1) {
            iVar2 = iVar2 + 1;
        }
        pcVar3 = pcVar3 + 0x5bd;
    } while (pcVar3 < &DAT_009ec468);

    if (iVar2 != 0) {
        (&DAT_009e9683)[iVar1] = 0xff;
        FUN_00427720(0x15, param_1);
        { unsigned short tmp = 1; __builtin_memcpy((void *)((int)&DAT_009e9a80 + iVar1), &tmp, sizeof(tmp)); }
        { unsigned short tmp = 1; __builtin_memcpy((void *)((int)&DAT_009e9a84 + iVar1), &tmp, sizeof(tmp)); }
        { unsigned int tmp = 0x40000; __builtin_memcpy((void *)((int)&DAT_009e9a86 + iVar1), &tmp, sizeof(tmp)); }
        { unsigned short tmp = 0xffff; __builtin_memcpy((void *)((int)&DAT_009e9a67 + iVar1), &tmp, sizeof(tmp)); }
        { unsigned short tmp = 0xffff; __builtin_memcpy((void *)((int)&DAT_009e9a69 + iVar1), &tmp, sizeof(tmp)); }
        { unsigned int tmp = 0x40000; __builtin_memcpy((void *)((int)&DAT_009e9a63 + iVar1), &tmp, sizeof(tmp)); }
        {
            unsigned short idx;
            __builtin_memcpy(&idx, (void *)((int)&DAT_009e968a + iVar1), sizeof(idx));
            if (idx != 0xffff) {
                unsigned int src1 = (unsigned int)idx * 0x23 + 4 + DAT_009a1fe8;
                unsigned int val1;
                __builtin_memcpy(&val1, (void *)src1, sizeof(val1));
                __builtin_memcpy((void *)((int)&DAT_009e9a5b + iVar1), &val1, sizeof(val1));

                unsigned int src2 = (unsigned int)idx * 0x23 + 8 + DAT_009a1fe8;
                unsigned int val2;
                __builtin_memcpy(&val2, (void *)src2, sizeof(val2));
                __builtin_memcpy((void *)((int)&DAT_009e9a5f + iVar1), &val2, sizeof(val2));

                unsigned int byte_src = (unsigned int)idx * 0x23 + 3 + DAT_009a1fe8;
                unsigned char b;
                __builtin_memcpy(&b, (void *)byte_src, sizeof(b));
                unsigned int int_src = (int)&DAT_00521f1c + (unsigned int)b * 0x1a;
                int val3;
                __builtin_memcpy(&val3, (void *)int_src, sizeof(val3));
                val3 = val3 << 4;
                __builtin_memcpy((void *)((int)&DAT_009e9a86 + iVar1), &val3, sizeof(val3));
            }
        }
        (&DAT_009e9681)[iVar1] = 0;
        { unsigned int tmp = 0xffffffff; __builtin_memcpy((void *)((int)&DAT_009e9670 + iVar1), &tmp, sizeof(tmp)); }
        { unsigned int tmp = 0; __builtin_memcpy((void *)((int)&DAT_009e9c1d + iVar1), &tmp, sizeof(tmp)); }
        (&DAT_009e96b6)[iVar1] = 0;
        FUN_0042f270();
        FUN_0042f110();
        FUN_0042f5d0();
        return;
    }
    (&DAT_009d6940)[0] = 1;
    return;
}
