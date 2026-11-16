/* SPDX-License-Identifier: MIT */
/* z_bop 004dc450 */
/* Prototypes for every callee */
void  FUN_004d6480(unsigned int *param_1, int *param_2);
void *_malloc(unsigned int size);
void  FUN_004d4e20(void);
int   FUN_004d48e0(void);
void  FUN_004d4ec0(int param_1);
void  FUN_004d4970(void);
void  FUN_004b75b0(int *param_1);
void  FUN_004dc9b0(void);
void  FUN_004d6460(int *param_1, int param_2, int param_3, int param_4, int param_5);
void  FUN_004d4d20(int *param_1);

/* Globals. Several sit at addresses that are not a multiple of their width,
   so they are packed and accessed only through __builtin_memcpy. */
extern int           DAT_00acc131;
extern int           DAT_00acebbd;
extern int           DAT_00aa7b82;
extern int           DAT_00aa7b86;
extern int           DAT_00aa7b8a;
extern int           DAT_00aa7b8e;
extern int           DAT_00aa7b1f;
extern int           DAT_00aa7b13;
extern unsigned char DAT_00aa7b0d;
extern unsigned char DAT_00aa7b24;
extern int           DAT_00aa6cfc;

extern unsigned char DAT_00acc575;
extern unsigned char DAT_00acc55d;
extern unsigned char DAT_00acc565;
extern unsigned char DAT_00acc561;
extern unsigned char DAT_00acc569;
extern unsigned char DAT_00acc139;
extern unsigned char DAT_00acc13d;
extern unsigned char DAT_00acc149;
extern unsigned char DAT_00acc145;
extern unsigned char DAT_00acc141;
extern unsigned char DAT_00acc54d;
extern unsigned char DAT_00acc56d;

unsigned int __cdecl FUN_004dc450(unsigned int param_1, int *param_2)
{
    unsigned char cVar1;
    int iVar2;
    unsigned int iVar3;
    int iVar4;
    unsigned int *puVar5;
    unsigned int uVar6;
    unsigned short *puVar7;
    unsigned int *puVar8;
    unsigned int uVar9;
    unsigned int *puVar10;
    unsigned short *puVar11;
    unsigned int *puVar12;
    unsigned short *puVar13;
    unsigned int *local_20;
    int local_1c;
    unsigned int *local_14;
    int local_10[4];
    int tmp;
    unsigned int utmp;
    int fmt;
    int stride;

    __builtin_memcpy(&iVar2, &DAT_00acc131, 4);
    if (8 < iVar2) {
        return 0;
    }
    iVar3 = (unsigned int)iVar2 * 0x440u;

    __builtin_memcpy(&tmp, &DAT_00acebbd, 4);
    __builtin_memcpy(&DAT_00acc575 + iVar3, &tmp, 4);
    __builtin_memcpy(&tmp, &DAT_00aa7b82, 4);
    __builtin_memcpy(&DAT_00acc55d + iVar3, &tmp, 4);
    __builtin_memcpy(&tmp, &DAT_00aa7b86, 4);
    __builtin_memcpy(&DAT_00acc565 + iVar3, &tmp, 4);
    __builtin_memcpy(&tmp, &DAT_00aa7b8a, 4);
    __builtin_memcpy(&DAT_00acc561 + iVar3, &tmp, 4);
    __builtin_memcpy(&tmp, &DAT_00aa7b8e, 4);
    __builtin_memcpy(&DAT_00acc569 + iVar3, &tmp, 4);

    if (param_2 == (int *)0) {
        tmp = 0;
        __builtin_memcpy(&DAT_00acc139 + iVar3, &tmp, 4);
        __builtin_memcpy(&DAT_00acc13d + iVar3, &tmp, 4);
        __builtin_memcpy(&DAT_00acc149 + iVar3, &tmp, 4);
        __builtin_memcpy(&DAT_00acc145 + iVar3, &tmp, 4);
        FUN_004d6460(local_10, 0, 0, 0x27f, 0x1df);
        param_2 = local_10;
        goto LAB_004dc798;
    }
    if (*param_2 < 0) {
        *param_2 = 0;
    }
    if (param_2[1] < 0) {
        param_2[1] = 0;
    }
    if (0x27f < param_2[2]) {
        param_2[2] = 0x27f;
    }
    if (0x1df < param_2[3]) {
        param_2[3] = 0x1df;
    }
    if ((param_2[2] < *param_2) || (param_2[3] < param_2[1])) {
        return 0;
    }
    FUN_004d6480((unsigned int *)(&DAT_00acc54d + iVar3), param_2);
    uVar9 = (unsigned int)(param_2[2] - *param_2) + 1u;
    iVar4 = (int)((unsigned int)(param_2[3] - param_2[1]) + 1u);
    local_1c = iVar4;

    __builtin_memcpy(&fmt, &DAT_00aa7b1f, 4);
    if (fmt == 8) {
        local_20 = (unsigned int *)_malloc((unsigned int)iVar4 * uVar9);
        cVar1 = DAT_00aa7b0d;
        if (local_20 == (unsigned int *)0) {
            return 0;
        }
        if (DAT_00aa7b24 == 0) {
            DAT_00aa6cfc = FUN_004d48e0();
        } else {
            FUN_004d4e20();
        }
        __builtin_memcpy(&stride, &DAT_00aa7b13, 4);
        puVar5 = (unsigned int *)((unsigned int)param_2[1] * (unsigned int)stride
                                  + (unsigned int)*param_2
                                  + (unsigned int)DAT_00aa6cfc);
        puVar8 = local_20;
        if (0 < iVar4) {
            do {
                puVar10 = puVar5;
                puVar12 = puVar8;
                for (uVar6 = uVar9 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
                    *puVar12 = *puVar10;
                    puVar10 = puVar10 + 1;
                    puVar12 = puVar12 + 1;
                }
                for (uVar6 = uVar9 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
                    *(unsigned char *)puVar12 = *(unsigned char *)puVar10;
                    puVar10 = (unsigned int *)((unsigned char *)puVar10 + 1);
                    puVar12 = (unsigned int *)((unsigned char *)puVar12 + 1);
                }
                __builtin_memcpy(&stride, &DAT_00aa7b13, 4);
                puVar5 = (unsigned int *)((unsigned char *)puVar5 + stride);
                local_1c = local_1c + -1;
                puVar8 = (unsigned int *)((unsigned char *)puVar8 + uVar9);
            } while (local_1c != 0);
        }
        if (DAT_00aa7b24 != 0) {
            FUN_004d4ec0(1);
        }
        if (cVar1 == 0) {
            FUN_004d4970();
        }
    } else if (fmt == 0x10) {
        local_20 = (unsigned int *)_malloc((unsigned int)iVar4 * uVar9 * 2u);
        cVar1 = DAT_00aa7b0d;
        if (local_20 == (unsigned int *)0) {
            return 0;
        }
        if (DAT_00aa7b24 == 0) {
            DAT_00aa6cfc = FUN_004d48e0();
        } else {
            FUN_004d4e20();
        }
        __builtin_memcpy(&stride, &DAT_00aa7b13, 4);
        puVar7 = (unsigned short *)((unsigned int)param_2[1] * (unsigned int)stride
                                    + (unsigned int)*param_2 * 2u
                                    + (unsigned int)DAT_00aa6cfc);
        local_14 = local_20;
        if (0 < iVar4) {
            do {
                uVar6 = uVar9;
                puVar13 = (unsigned short *)local_14;
                puVar11 = puVar7;
                if (0 < (int)uVar9) {
                    do {
                        *puVar13 = *puVar11;
                        uVar6 = uVar6 - 1;
                        puVar13 = puVar13 + 1;
                        puVar11 = puVar11 + 1;
                    } while (uVar6 != 0);
                }
                __builtin_memcpy(&stride, &DAT_00aa7b13, 4);
                puVar7 = (unsigned short *)((unsigned char *)puVar7
                                            + ((unsigned int)stride & 0xfffffffeu));
                local_1c = local_1c + -1;
                local_14 = (unsigned int *)((unsigned char *)local_14 + uVar9 * 2u);
            } while (local_1c != 0);
        }
        if (DAT_00aa7b24 != 0) {
            FUN_004d4ec0(1);
        }
        if (cVar1 == 0) {
            FUN_004d4970();
        }
    }

    __builtin_memcpy(&DAT_00acc139 + iVar3, &uVar9, 4);
    __builtin_memcpy(&DAT_00acc13d + iVar3, &iVar4, 4);
    __builtin_memcpy(&DAT_00acc149 + iVar3, &local_20, 4);

    __builtin_memcpy(&fmt, &DAT_00aa7b1f, 4);
    if (fmt == 8) {
        utmp = (unsigned int)iVar4 * uVar9;
        __builtin_memcpy(&DAT_00acc145 + iVar3, &utmp, 4);
        tmp = 0;
        __builtin_memcpy(&DAT_00acc141 + iVar3, &tmp, 4);
        FUN_004b75b0((int *)(&DAT_00acc139 + iVar3));
    } else {
        __builtin_memcpy(&fmt, &DAT_00aa7b1f, 4);
        if (fmt == 0x10) {
            utmp = (unsigned int)iVar4 * uVar9 * 2u;
            __builtin_memcpy(&DAT_00acc145 + iVar3, &utmp, 4);
            tmp = 0;
            __builtin_memcpy(&DAT_00acc141 + iVar3, &tmp, 4);
        }
    }
    if (DAT_00aa7b24 != 0) {
        FUN_004dc9b0();
    }

LAB_004dc798:
    FUN_004d4d20(param_2);
    __builtin_memcpy(&tmp, &DAT_00acc131, 4);
    tmp = (int)((unsigned int)tmp + 1u);
    __builtin_memcpy(&DAT_00acc131, &tmp, 4);
    __builtin_memcpy(&DAT_00acc56d + (unsigned int)tmp * 0x440u, &param_1, 4);
    tmp = -1;
    __builtin_memcpy(&DAT_00acebbd, &tmp, 4);
    return 1;
}
