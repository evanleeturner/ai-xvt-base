/* SPDX-License-Identifier: MIT */
/* z_bop 004ae3c0 */
extern unsigned char DAT_00521f19[];
extern unsigned short DAT_009a7ed0[];
extern void * FUN_004ac720(int);
extern void FUN_00471ff0(void *);
extern void FUN_004adcc0(int);
extern long long __ftol(float);
extern void FUN_004ac740(int);

int __cdecl FUN_004ae3c0(int param_1, int param_2)
{
    int *piVar1;
    long long lVar2;
    int field_a;
    int field_base;
    int field_val;
    int field_6;

    if ((DAT_00521f19[param_1 * 0x1a] & 1) != 0) {
        return 0;
    }

    piVar1 = (int *)FUN_004ac720(((unsigned int)(param_1 * 5) & 0xFFFF0000) | (DAT_009a7ed0[param_1] & 0xFFFF));

    if (*piVar1 != (int)piVar1) {
        FUN_00471ff0(piVar1);
    }

    __builtin_memcpy(&field_a, (char *)piVar1 + 0xa, 4);
    __builtin_memcpy(&field_base, (void *)field_a, 4);
    __builtin_memcpy(&field_val, (char *)field_base + 4, 4);
    if (field_val != 0x14) {
        param_2 = param_2 + 1;
    }

    __builtin_memcpy(&field_6, (char *)piVar1 + 6, 4);
    if (field_6 <= param_2) {
        param_2 = field_6 - 1;
    }

    __builtin_memcpy(&field_a, (char *)piVar1 + 0xa, 4);
    __builtin_memcpy(&field_base, (void *)field_a, 4);
    __builtin_memcpy(&field_val, (char *)field_base + param_2 * 4, 4);
    FUN_004adcc0(field_val);

    lVar2 = __ftol((float)param_2);
    FUN_004ac740((int)lVar2);
    return (int)lVar2;
}
