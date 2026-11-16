/* SPDX-License-Identifier: MIT */
/* z_bop 0044f880 */
extern int DAT_005235f0;
extern int DAT_005233b8;
extern unsigned char DAT_00521f1c[];
extern int DAT_00a07ce8;
extern int DAT_009a1fe8;
extern int DAT_009fd3bc[];
extern int DAT_009fd43c[];
extern int DAT_009a8d8c;
extern int DAT_009a8d80;
extern int DAT_009a8d60;
extern int DAT_009d1264;
extern int DAT_009d1154;
extern int DAT_009d12e8;
extern int DAT_009a8e28;
extern int DAT_009a8e1c;
extern int DAT_009a8e20;
extern int DAT_00523400;

unsigned int __cdecl FUN_0041f410(int, int, int);
void __cdecl FUN_00427a60(short, short, short, int, int);

#define FX_MUL(a, b) \
    ((((unsigned int)((long long)(a) * (long long)(b))) >> 0xf) | \
     (((unsigned int)(int)((unsigned long long)((long long)(a) * (long long)(b)) >> 0x20)) << 0x11))

#define RD_S16(dst, base, off) \
    do { short _t; __builtin_memcpy(&_t, (const void *)((base) + (off)), 2); (dst) = (int)_t; } while (0)

void __cdecl FUN_0044f880(int param_1)
{
    unsigned char bVar1;
    int iVar2;
    int iVar3;
    int iVar4;
    int iVar5;
    int iVar6;
    int iVar10;
    int iVar11;
    int iVar12;
    int iVar13;
    int iVar14;
    unsigned int uVar15;
    int iVar16;
    int iVar17;
    int *piVar18;
    int *piVar19;
    int local_a8;
    int local_a4;
    int s0, s1, s2;
    int t;

    DAT_005235f0 = 0;
    if (DAT_005233b8 != 0) {
        __builtin_memcpy(&iVar2, &DAT_00521f1c[(unsigned int)*(unsigned char *)(param_1 + 3) * 0x1a], 4);
        iVar3 = *(int *)(param_1 + 4);
        iVar4 = *(int *)(param_1 + 8);
        iVar5 = *(int *)(param_1 + 0xc);
        local_a8 = 0;
        local_a4 = 0;
        if (0 < DAT_00a07ce8) {
            iVar17 = DAT_009a1fe8;
            piVar18 = DAT_009fd3bc;
            do {
                piVar19 = piVar18;
                if ((*(char *)(iVar17 + 3) != '\0') && (*(char *)(iVar17 + 2) == '\r')) {
                    __builtin_memcpy(&t, (const void *)(iVar17 + 4), 4);
                    iVar16 = (int)((unsigned int)t - (unsigned int)iVar3);
                    __builtin_memcpy(&t, (const void *)(iVar17 + 8), 4);
                    iVar13 = (int)((unsigned int)t - (unsigned int)iVar4);
                    __builtin_memcpy(&t, (const void *)(iVar17 + 0xc), 4);
                    iVar14 = (int)((unsigned int)t - (unsigned int)iVar5);
                    uVar15 = FUN_0041f410(iVar16, iVar13, iVar14);
                    if (uVar15 < (unsigned int)iVar2 + 0x4000U) {
                        __builtin_memcpy(&iVar6, (const void *)(param_1 + 0x1f), 4);
                        if (iVar6 == 0) {
                            FUN_00427a60(*(short *)(param_1 + 0x14), *(short *)(param_1 + 0x12),
                                         *(short *)(param_1 + 0x10), 0, 0);
                            iVar12 = DAT_009d1264;
                            iVar11 = DAT_009d1154;
                            piVar18[-3] = (int)(FX_MUL(iVar14, DAT_009a8d8c) +
                                                FX_MUL(iVar16, DAT_009a8d80) +
                                                FX_MUL(iVar13, DAT_009a8d60));
                            iVar10 = DAT_009a8e28;
                            iVar6 = DAT_009a8e1c;
                            piVar18[-2] = (int)(0U - (FX_MUL(iVar14, iVar11) +
                                                      FX_MUL(iVar16, DAT_009d12e8) +
                                                      FX_MUL(iVar13, iVar12)));
                            iVar13 = (int)(FX_MUL(iVar14, iVar10) +
                                           FX_MUL(iVar16, DAT_009a8e20) +
                                           FX_MUL(iVar13, iVar6));
                        } else {
                            RD_S16(s0, iVar6, 0x99);
                            RD_S16(s1, iVar6, 0x9b);
                            RD_S16(s2, iVar6, 0x9d);
                            piVar18[-3] = (int)(FX_MUL(iVar14, s2) +
                                                FX_MUL(iVar16, s0) +
                                                FX_MUL(iVar13, s1));
                            __builtin_memcpy(&iVar6, (const void *)(param_1 + 0x1f), 4);
                            RD_S16(s0, iVar6, 0x93);
                            RD_S16(s1, iVar6, 0x95);
                            RD_S16(s2, iVar6, 0x97);
                            piVar18[-2] = (int)(0U - (FX_MUL(iVar14, s2) +
                                                      FX_MUL(iVar16, s0) +
                                                      FX_MUL(iVar13, s1)));
                            __builtin_memcpy(&iVar6, (const void *)(param_1 + 0x1f), 4);
                            RD_S16(s0, iVar6, 0x9f);
                            RD_S16(s1, iVar6, 0xa1);
                            RD_S16(s2, iVar6, 0xa3);
                            iVar13 = (int)(FX_MUL(iVar14, s2) +
                                           FX_MUL(iVar16, s0) +
                                           FX_MUL(iVar13, s1));
                        }
                        piVar18[-1] = iVar13;
                        *piVar18 = 0x10;
                        switch (*(unsigned char *)(iVar17 + 3)) {
                        case 0x7f:
                        case 0x80:
                        case 0x81:
                        case 0x82:
                            switch (*(unsigned char *)(iVar17 + 0x19)) {
                            case 2:
                                *piVar18 = 0xc0;
                                break;
                            case 3:
                                *piVar18 = 0x140;
                                break;
                            case 4:
                                *piVar18 = 0x1e0;
                                break;
                            case 5:
                            case 6:
                            case 7:
                            case 8:
                                *piVar18 = 0x140;
                                break;
                            case 9:
                                *piVar18 = 0xc0;
                                break;
                            case 10:
                                *piVar18 = 0x60;
                                break;
                            case 0xb:
                                *piVar18 = 0x30;
                            }
                            __builtin_memcpy(&t, (const void *)(iVar17 + 0x1f), 4);
                            if (t != 0) {
                                bVar1 = *(unsigned char *)(t + 1);
                                if (3 < bVar1) {
                                    *piVar18 = (int)((unsigned int)(((int)(bVar1 + 4)) >> 2) * (unsigned int)*piVar18);
                                }
                            }
                            break;
                        case 0x83:
                        case 0x84:
                            switch (*(unsigned char *)(iVar17 + 0x19)) {
                            case 2:
                                *piVar18 = 0x30;
                                break;
                            case 3:
                                *piVar18 = 0x60;
                                break;
                            case 4:
                                *piVar18 = 0x40;
                                break;
                            case 5:
                                *piVar18 = 0x20;
                            }
                            break;
                        default:
                            *piVar18 = (int)((unsigned int)DAT_00523400 + 0xffffff00U);
                        }
                        piVar19 = piVar18 + 4;
                        local_a8 = local_a8 + 1;
                        *piVar18 = (int)((unsigned int)*piVar18 << 3);
                        if (piVar19 == DAT_009fd43c) break;
                    }
                }
                iVar17 = iVar17 + 0x23;
                local_a4 = local_a4 + 1;
                piVar18 = piVar19;
            } while (local_a4 < DAT_00a07ce8);
        }
        DAT_005235f0 = local_a8;
    }
    return;
}
