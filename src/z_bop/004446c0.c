/* SPDX-License-Identifier: MIT */
/* z_bop 004446c0 */
extern int DAT_00556928;
extern unsigned char DAT_00556934;
extern unsigned char DAT_00556935;
extern unsigned char DAT_00556936;
extern unsigned char _DAT_00556941[4];
extern unsigned char _DAT_00556945[4];
extern unsigned char _DAT_00556965[4];

extern char s_Memory_allocation_failed_00521be4[];
extern char s_Dithering_image_00521c5c[];
extern char s_Unable_to_dither_image_00521c74[];

int __cdecl FUN_00443d10(int param_1);
void __cdecl FUN_004438b0(const char *param_1, const char *param_2);
void __cdecl FUN_004444f0(int param_1);
void __cdecl FUN_00443890(const char *param_1);
void *__cdecl _malloc(unsigned int size);
void __cdecl _free(void *p);

unsigned int __cdecl FUN_004446c0(int param_1)
{
    unsigned char bVar1;
    unsigned char bVar2;
    unsigned char bVar3;
    unsigned short uVar4;
    int iVar5;
    unsigned int *_Memory;
    int *_Memory_00;
    void *_Memory_01;
    void *_Memory_02;
    int *piVar6;
    unsigned int uVar7;
    unsigned int uVar8;
    int *piVar9;
    unsigned int uVar10;
    int *piVar11;
    unsigned char *pbVar12;
    int iVar13;
    int iVar14;
    int *puVar15;
    unsigned int uVar16;
    int iVar17;
    short local_32;
    unsigned char *local_2c;
    unsigned int local_24;
    unsigned int local_10;
    unsigned char *base;
    unsigned int tmp;
    int t;
    int palette;
    unsigned int val;

    base = (unsigned char *)param_1;

    iVar5 = FUN_00443d10(param_1);
    if (iVar5 == 0) {
        return 1;
    }
    _Memory = (unsigned int *)_malloc(0x100000);
    _Memory_00 = (int *)_malloc((*(int *)(base + 0x1030) * 3U + 6U) * 8U);
    _Memory_01 = _malloc(0x7fc);
    _Memory_02 = _malloc(0x300);
    uVar10 = 0;
    if ((((_Memory == (unsigned int *)0) || (_Memory_00 == (int *)0)) ||
         (_Memory_01 == (void *)0)) || (_Memory_02 == (void *)0)) {
        FUN_004438b0(s_Unable_to_dither_image_00521c74, s_Memory_allocation_failed_00521be4);
        return 1;
    }
    {
        unsigned int *pu = _Memory;
        for (iVar5 = 0x40000; iVar5 != 0; iVar5 = iVar5 - 1) {
            *pu = 0xffffffff;
            pu = pu + 1;
        }
    }
    puVar15 = _Memory_00;
    if (((unsigned int)*(int *)(base + 0x1030) + 2U & 0x7fffffff) != 0) {
        do {
            *puVar15 = 0;
            uVar10 = uVar10 + 1;
            puVar15[1] = 0;
            puVar15[2] = 0;
            puVar15 = puVar15 + 3;
        } while (uVar10 < (unsigned int)*(int *)(base + 0x1030) * 2U + 4U);
    }
    uVar10 = *(unsigned int *)(base + 0x1054) >> 4;
    if (uVar10 < 2) {
        uVar10 = 1;
    }
    piVar6 = (int *)((unsigned char *)_Memory_01 + 0x3fc);
    uVar7 = 0x100U / uVar10;
    iVar5 = 0;
    piVar11 = piVar6;
    piVar9 = piVar6;
    uVar16 = 0;
    for (uVar8 = uVar7; uVar8 != 0; uVar8 = uVar8 - 1) {
        *piVar9 = iVar5;
        piVar9 = piVar9 + 1;
        *piVar11 = (int)(0U - (unsigned int)iVar5);
        iVar5 = iVar5 + 1;
        piVar11 = piVar11 + -1;
        uVar16 = uVar7;
    }
    if ((3 < uVar10) && (uVar10 = 0x300U / uVar10, uVar16 < uVar10)) {
        piVar9 = piVar6 + uVar16;
        piVar11 = piVar6 - uVar16;
        do {
            *piVar9 = iVar5;
            piVar9 = piVar9 + 1;
            *piVar11 = (int)(0U - (unsigned int)iVar5);
            uVar8 = ~uVar16;
            uVar16 = uVar16 + 1;
            iVar5 = (int)((unsigned int)iVar5 + (uVar8 & 1));
            piVar11 = piVar11 + -1;
        } while (uVar16 < uVar10);
    }
    if ((int)uVar16 < 0x100) {
        piVar9 = piVar6 + uVar16;
        iVar13 = (int)(0x100U - uVar16);
        piVar11 = piVar6 - uVar16;
        do {
            *piVar9 = iVar5;
            piVar9 = piVar9 + 1;
            iVar13 = iVar13 + -1;
            *piVar11 = (int)(0U - (unsigned int)iVar5);
            piVar11 = piVar11 + -1;
        } while (iVar13 != 0);
    }
    iVar5 = 0;
    do {
        *((unsigned char *)_Memory_02 + iVar5) = 0;
        *((unsigned char *)_Memory_02 + iVar5 + 0x100) = (unsigned char)iVar5;
        iVar13 = iVar5 + 1;
        *((unsigned char *)_Memory_02 + iVar5 + 0x200) = 0xff;
        iVar5 = iVar13;
    } while (iVar13 < 0x100);
    local_24 = 0;
    if (*(int *)(base + 0x1034) != 0) {
        do {
            iVar5 = *(int *)(base + 0x1030);
            __builtin_memcpy(&tmp, base + 0x1086, 4);
            local_2c = (unsigned char *)(tmp + (unsigned int)iVar5 * local_24 * 6U);
            piVar11 = _Memory_00 + ((unsigned int)iVar5 * 3U + 9U);
            iVar13 = (int)((0U - (unsigned int)((local_24 & 1) == 0) & 2U) - 1U);
            piVar9 = _Memory_00 + 3;
            if (iVar13 < 0) {
                local_2c = local_2c + ((unsigned int)iVar5 * 6U - 6U);
                piVar11 = _Memory_00 + (unsigned int)iVar5 * 3U;
                piVar9 = piVar11 + ((unsigned int)iVar5 * 3U + 6U);
            }
            local_10 = 0;
            if (iVar5 != 0) {
                do {
                    t = (int)((unsigned int)*piVar9 + 8U);
                    bVar1 = *((unsigned char *)_Memory_02 +
                              piVar6[(int)((unsigned int)t + ((unsigned int)(t >> 0x1f) & 0xfU)) >> 4] +
                              (unsigned int)*local_2c + 0x100);
                    t = (int)((unsigned int)piVar9[1] + 8U);
                    bVar2 = *((unsigned char *)_Memory_02 +
                              piVar6[(int)((unsigned int)t + ((unsigned int)(t >> 0x1f) & 0xfU)) >> 4] +
                              (unsigned int)local_2c[1] + 0x100);
                    t = (int)((unsigned int)piVar9[2] + 8U);
                    bVar3 = *((unsigned char *)_Memory_02 +
                              piVar6[(int)((unsigned int)t + ((unsigned int)(t >> 0x1f) & 0xfU)) >> 4] +
                              (unsigned int)local_2c[2] + 0x100);
                    uVar10 = ((((unsigned int)bVar3 & 0xfffffffcU) << 6 |
                               ((unsigned int)bVar2 & 0xfffffffcU)) << 4) |
                             (unsigned int)(bVar1 >> 2);
                    if ((int)_Memory[uVar10] < 0) {
                        local_32 = 7;
                        iVar5 = DAT_00556928;
                        do {
                            uVar16 = ((((unsigned int)bVar1 >> ((unsigned char)local_32 & 0x1f) & 1U) * 2U |
                                       ((unsigned int)bVar2 >> ((unsigned char)local_32 & 0x1f) & 1U)) * 2U) |
                                     ((unsigned int)bVar3 >> ((unsigned char)local_32 & 0x1f) & 1U);
                            if ((1U << (unsigned char)uVar16 &
                                 (unsigned int)*(unsigned char *)(iVar5 + 2)) == 0) {
                                break;
                            }
                            local_32 = (short)(local_32 + -1);
                            __builtin_memcpy(&iVar5, (unsigned char *)(iVar5 + 0x32 + uVar16 * 4U), 4);
                        } while (local_32 != 0);
                        val = 0;
                        __builtin_memcpy(_DAT_00556941, &val, 4);
                        val = 0x41080000;
                        __builtin_memcpy(_DAT_00556945, &val, 4);
                        DAT_00556934 = bVar1;
                        DAT_00556935 = bVar2;
                        DAT_00556936 = bVar3;
                        __builtin_memcpy(&tmp, (unsigned char *)(iVar5 + 0x2e), 4);
                        FUN_004444f0((int)tmp);
                        __builtin_memcpy(&val, _DAT_00556965, 4);
                        _Memory[uVar10] = val;
                    }
                    uVar4 = *(unsigned short *)(_Memory + uVar10);
                    palette = *(int *)(base + 0x104c);
                    if (*(int *)(base + 0x1024) == 2) {
                        *(unsigned short *)(local_2c + 4) = uVar4;
                    } else {
                        iVar5 = (int)((unsigned int)uVar4 * 9U);
                        *local_2c = *(unsigned char *)(palette + iVar5);
                        local_2c[1] = *(unsigned char *)(palette + 1 + iVar5);
                        local_2c[2] = *(unsigned char *)(palette + 2 + iVar5);
                    }
                    local_2c = local_2c + iVar13 * 6;
                    pbVar12 = (unsigned char *)((unsigned int)uVar4 * 9U + *(unsigned int *)(base + 0x104c));
                    iVar5 = (int)((unsigned int)bVar1 - (unsigned int)*pbVar12);
                    iVar14 = (int)((unsigned int)bVar2 - (unsigned int)pbVar12[1]);
                    iVar17 = (int)((unsigned int)bVar3 - (unsigned int)pbVar12[2]);
                    *piVar9 = 0;
                    piVar9[1] = 0;
                    piVar9[2] = 0;
                    piVar9 = piVar9 + iVar13 * 3;
                    *piVar9 = (int)((unsigned int)*piVar9 + (unsigned int)iVar5 * 7U);
                    piVar9[1] = (int)((unsigned int)piVar9[1] + (unsigned int)iVar14 * 7U);
                    piVar11 = piVar11 + iVar13 * -3;
                    piVar9[2] = (int)((unsigned int)piVar9[2] + (unsigned int)iVar17 * 7U);
                    *piVar11 = (int)((unsigned int)*piVar11 + (unsigned int)iVar5 * 3U);
                    piVar11[1] = (int)((unsigned int)piVar11[1] + (unsigned int)iVar14 * 3U);
                    piVar11[2] = (int)((unsigned int)piVar11[2] + (unsigned int)iVar17 * 3U);
                    piVar11 = piVar11 + iVar13 * 3;
                    *piVar11 = (int)((unsigned int)*piVar11 + (unsigned int)iVar5 * 5U);
                    piVar11[1] = (int)((unsigned int)piVar11[1] + (unsigned int)iVar14 * 5U);
                    piVar11[2] = (int)((unsigned int)piVar11[2] + (unsigned int)iVar17 * 5U);
                    piVar11 = piVar11 + iVar13 * 3;
                    *piVar11 = (int)((unsigned int)*piVar11 + (unsigned int)iVar5);
                    piVar11[1] = (int)((unsigned int)piVar11[1] + (unsigned int)iVar14);
                    local_10 = local_10 + 1;
                    piVar11[2] = (int)((unsigned int)piVar11[2] + (unsigned int)iVar17);
                } while (local_10 < *(unsigned int *)(base + 0x1030));
            }
            FUN_00443890(s_Dithering_image_00521c5c);
            local_24 = local_24 + 1;
        } while (local_24 < *(unsigned int *)(base + 0x1034));
    }
    _free(_Memory_02);
    _free(_Memory_01);
    _free(_Memory_00);
    _free(_Memory);
    return 0;
}
