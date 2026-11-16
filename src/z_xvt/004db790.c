/* SPDX-License-Identifier: MIT */
/* z_xvt 004db790 */
extern unsigned char DAT_0077dc66[];
extern unsigned char DAT_0077dc6e[];
extern unsigned char DAT_0077dc72[];
extern unsigned char DAT_0077dc76[];
extern unsigned char DAT_0077dc7a[];
extern unsigned char DAT_0077dc7e[];
extern unsigned char DAT_0077e082[];
extern unsigned char DAT_0077e092[];
extern unsigned char DAT_0077e096[];
extern unsigned char DAT_0077e09a[];
extern unsigned char DAT_0077e09e[];
extern unsigned char DAT_0077e0a2[];
extern unsigned char DAT_0077e0aa[];
extern unsigned char DAT_007806f2[];
extern unsigned char DAT_007596b7[];
extern unsigned char DAT_007596bb[];
extern unsigned char DAT_007596bf[];
extern unsigned char DAT_007596c3[];
extern unsigned char DAT_00759657[];
extern unsigned char DAT_0075964b[];
extern unsigned char DAT_00759645;
extern unsigned char DAT_0075965c;
extern unsigned int DAT_00758830;

void * __cdecl _malloc(unsigned int size);
void __cdecl FUN_004dcfd0(int *param_1, int param_2, int param_3, int param_4, int param_5);
void __cdecl FUN_004dd000(unsigned int *param_1, int *param_2);
unsigned int __cdecl FUN_004d4d00(void);
void __cdecl FUN_004d52b0(void);
void __cdecl FUN_004d5350(int param_1);
void __cdecl FUN_004d4d90(void);
unsigned int __cdecl FUN_004d3070(int *param_1);
void __cdecl FUN_004dbd20(void);
void __cdecl FUN_004d51b0(int *param_2);

unsigned int __cdecl FUN_004db790(unsigned int param_1, int *param_2)
{
    unsigned char cVar1;
    int iVar2;
    int iVar3;
    int iVar4;
    unsigned char *puVar5;
    unsigned int uVar6;
    unsigned char *puVar7;
    unsigned char *puVar8;
    unsigned int uVar9;
    unsigned char *puVar10;
    unsigned char *puVar11;
    unsigned char *puVar12;
    unsigned char *local_20;
    int local_1c;
    unsigned char *local_18;
    int local_10[4];
    unsigned int tmp;
    int bpp;
    int pitch;
    int cnt;

    __builtin_memcpy(&iVar2, DAT_0077dc66, 4);
    if (8 < iVar2) {
        return 0;
    }
    iVar3 = (int)((unsigned int)iVar2 * 0x440u);

    __builtin_memcpy(&tmp, DAT_007806f2, 4);
    __builtin_memcpy(DAT_0077e0aa + iVar3, &tmp, 4);
    __builtin_memcpy(&tmp, DAT_007596b7, 4);
    __builtin_memcpy(DAT_0077e092 + iVar3, &tmp, 4);
    __builtin_memcpy(&tmp, DAT_007596bb, 4);
    __builtin_memcpy(DAT_0077e09a + iVar3, &tmp, 4);
    __builtin_memcpy(&tmp, DAT_007596bf, 4);
    __builtin_memcpy(DAT_0077e096 + iVar3, &tmp, 4);
    __builtin_memcpy(&tmp, DAT_007596c3, 4);
    __builtin_memcpy(DAT_0077e09e + iVar3, &tmp, 4);

    if (param_2 == (int *)0) {
        tmp = 0;
        __builtin_memcpy(DAT_0077dc6e + iVar3, &tmp, 4);
        __builtin_memcpy(DAT_0077dc72 + iVar3, &tmp, 4);
        __builtin_memcpy(DAT_0077dc7e + iVar3, &tmp, 4);
        __builtin_memcpy(DAT_0077dc7a + iVar3, &tmp, 4);
        FUN_004dcfd0(local_10, 0, 0, 0x27f, 0x1df);
        param_2 = local_10;
        goto LAB_004dbafc;
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
    FUN_004dd000((unsigned int *)(DAT_0077e082 + iVar3), param_2);
    uVar9 = (unsigned int)((unsigned int)param_2[2] - (unsigned int)*param_2) + 1u;
    iVar4 = (int)((unsigned int)((unsigned int)param_2[3] - (unsigned int)param_2[1]) + 1u);
    local_1c = iVar4;
    __builtin_memcpy(&bpp, DAT_00759657, 4);
    if (bpp == 8) {
        local_20 = (unsigned char *)_malloc((unsigned int)iVar4 * uVar9);
        cVar1 = DAT_00759645;
        if (local_20 == (unsigned char *)0) {
            return 0;
        }
        if (DAT_0075965c == '\0') {
            DAT_00758830 = FUN_004d4d00();
        } else {
            FUN_004d52b0();
        }
        __builtin_memcpy(&pitch, DAT_0075964b, 4);
        puVar5 = (unsigned char *)((unsigned int)param_2[1] * (unsigned int)pitch + (unsigned int)*param_2 + DAT_00758830);
        puVar8 = local_20;
        if (0 < iVar4) {
            do {
                puVar10 = puVar5;
                puVar12 = puVar8;
                for (uVar6 = uVar9 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
                    __builtin_memcpy(puVar12, puVar10, 4);
                    puVar10 = puVar10 + 4;
                    puVar12 = puVar12 + 4;
                }
                for (uVar6 = uVar9 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
                    *puVar12 = *puVar10;
                    puVar10 = puVar10 + 1;
                    puVar12 = puVar12 + 1;
                }
                __builtin_memcpy(&pitch, DAT_0075964b, 4);
                puVar5 = puVar5 + pitch;
                local_1c = local_1c + -1;
                puVar8 = puVar8 + uVar9;
            } while (local_1c != 0);
        }
        if (DAT_0075965c != '\0') {
            FUN_004d5350(1);
        }
        if (cVar1 == '\0') {
            FUN_004d4d90();
        }
    } else if (bpp == 0x10) {
        local_20 = (unsigned char *)_malloc((unsigned int)iVar4 * uVar9 * 2u);
        cVar1 = DAT_00759645;
        if (local_20 == (unsigned char *)0) {
            return 0;
        }
        if (DAT_0075965c == '\0') {
            DAT_00758830 = FUN_004d4d00();
        } else {
            FUN_004d52b0();
        }
        __builtin_memcpy(&pitch, DAT_0075964b, 4);
        puVar7 = (unsigned char *)((unsigned int)param_2[1] * (unsigned int)pitch + (unsigned int)*param_2 * 2u + DAT_00758830);
        local_18 = local_20;
        if (0 < iVar4) {
            do {
                uVar6 = uVar9;
                puVar8 = local_18;
                puVar11 = puVar7;
                if (0 < (int)uVar9) {
                    do {
                        __builtin_memcpy(puVar8, puVar11, 2);
                        uVar6 = uVar6 - 1;
                        puVar8 = puVar8 + 2;
                        puVar11 = puVar11 + 2;
                    } while (uVar6 != 0);
                }
                __builtin_memcpy(&pitch, DAT_0075964b, 4);
                puVar7 = puVar7 + ((unsigned int)pitch & 0xfffffffeu);
                local_1c = local_1c + -1;
                local_18 = local_18 + uVar9 * 2u;
            } while (local_1c != 0);
        }
        if (DAT_0075965c != '\0') {
            FUN_004d5350(1);
        }
        if (cVar1 == '\0') {
            FUN_004d4d90();
        }
    }
    __builtin_memcpy(DAT_0077dc6e + iVar3, &uVar9, 4);
    __builtin_memcpy(DAT_0077dc72 + iVar3, &iVar4, 4);
    __builtin_memcpy(DAT_0077dc7e + iVar3, &local_20, 4);
    __builtin_memcpy(&bpp, DAT_00759657, 4);
    if (bpp == 8) {
        tmp = (unsigned int)iVar4 * uVar9;
        __builtin_memcpy(DAT_0077dc7a + iVar3, &tmp, 4);
        tmp = 0;
        __builtin_memcpy(DAT_0077dc76 + iVar3, &tmp, 4);
        FUN_004d3070((int *)(DAT_0077dc6e + iVar3));
    } else if (bpp == 0x10) {
        tmp = (unsigned int)iVar4 * uVar9 * 2u;
        __builtin_memcpy(DAT_0077dc7a + iVar3, &tmp, 4);
        tmp = 0;
        __builtin_memcpy(DAT_0077dc76 + iVar3, &tmp, 4);
    }
    if (DAT_0075965c != '\0') {
        FUN_004dbd20();
    }
LAB_004dbafc:
    FUN_004d51b0(param_2);
    __builtin_memcpy(&cnt, DAT_0077dc66, 4);
    cnt = (int)((unsigned int)cnt + 1u);
    __builtin_memcpy(DAT_0077dc66, &cnt, 4);
    __builtin_memcpy(DAT_0077e0a2 + (int)((unsigned int)cnt * 0x440u), &param_1, 4);
    tmp = 0xffffffffu;
    __builtin_memcpy(DAT_007806f2, &tmp, 4);
    return 1;
}
