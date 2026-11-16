/* SPDX-License-Identifier: MIT */
/* z_xvt 00431110 */
extern int DAT_00733380;
extern unsigned int DAT_00733384[0x80];
extern unsigned int DAT_00733584;
extern int DAT_00733588;
extern int DAT_0073358c;
extern unsigned int DAT_00733590[0x80];
extern unsigned int DAT_00733790;
extern int DAT_00733794;
extern unsigned int DAT_007337a0[];
extern unsigned int DAT_007337a8[];
extern unsigned int DAT_007337b0[];
extern int DAT_007337b4[];
extern unsigned char DAT_007337b8[];
extern unsigned char DAT_007337b9[];
extern unsigned int DAT_007339b8[];
extern unsigned int DAT_00739290;
extern char DAT_0085e78f;
extern int DAT_007330a0;
extern int DAT_007330f8;
extern int *DAT_00733074;

extern int DAT_005251a4;
extern int DAT_005251a8;
extern int DAT_005251ac;
extern int DAT_005251b4;

extern unsigned int DAT_005ce218[];
extern unsigned int DAT_005ce21c[];
extern unsigned char DAT_005ce220[];
extern unsigned char DAT_005ce224[];
extern unsigned char DAT_005ce225[];
extern unsigned char DAT_005ce226[];
extern unsigned char DAT_005ce228[];

extern unsigned int DAT_005bda18[];
extern unsigned int DAT_005bda1c[];
extern unsigned char DAT_005bda20[];
extern unsigned char DAT_005bda24[];
extern unsigned char DAT_005bda25[];
extern unsigned char DAT_005bda26[];
extern unsigned char DAT_005bda28[];

extern unsigned int DAT_00539a08[];
extern unsigned int DAT_00539a0c[];
extern unsigned int DAT_00539a10[];
extern unsigned char DAT_00539a14[];
extern unsigned char DAT_00539a15[];
extern unsigned char DAT_00539a16[];
extern unsigned char DAT_00539a17[];
extern unsigned int DAT_00539a18[];

unsigned int __cdecl FUN_00433540(int param_1);
void __cdecl FUN_004336c0(void);

unsigned char __cdecl FUN_00431110(int param_1, unsigned int *param_2, unsigned int param_3)
{
    unsigned int uVar1;
    short sVar2;
    unsigned char bVar3;
    unsigned int *puVar4;
    int iVar5;
    unsigned int uVar6;
    unsigned int uVar7;
    unsigned int uVar8;
    int iVar9;
    unsigned short uVar10;
    unsigned int *puVar11;
    unsigned int *puVar12;
    unsigned int *puVar13;
    unsigned int *puVar14;
    unsigned char bVar15;
    int local_41c;
    int local_404;
    unsigned short local_400;
    short local_3fe;
    unsigned int local_3fc[255];

    local_404 = 0;
    if ((int)param_3 < 4) {
        return 0;
    }
    uVar1 = *param_2;
    bVar3 = (unsigned char)uVar1 & 0x7f;
    uVar10 = (unsigned short)bVar3;
    if ((uVar1 < 0x3c) || (0x3f < uVar1)) {
        bVar15 = 1;
    } else {
        bVar15 = 0;
    }
    sVar2 = (short)param_3;
    if ((uVar1 == 1) && (DAT_0085e78f == '\x01')) {
        local_400 = (unsigned short)((((unsigned short)DAT_0073358c & 0x7f) << 8) | 0x8081);
        DAT_0073358c = (int)((unsigned int)DAT_0073358c + 1);
        if (0x7f < DAT_0073358c) {
            DAT_0073358c = 0;
        }
        puVar4 = (unsigned int *)&local_3fe;
        local_41c = 2;
        if (bVar15) {
            local_41c = 4;
            local_3fe = (short)(sVar2 - 4);
            puVar4 = local_3fc;
        }
        uVar7 = param_3 - 4;
        puVar12 = param_2 + 1;
        puVar14 = puVar4;
        for (uVar6 = uVar7 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
            __builtin_memcpy(puVar14, puVar12, 4);
            puVar12 = puVar12 + 1;
            puVar14 = puVar14 + 1;
        }
        for (uVar6 = uVar7 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
            *(char *)puVar14 = (char)*puVar12;
            puVar12 = (unsigned int *)((int)puVar12 + 1);
            puVar14 = (unsigned int *)((int)puVar14 + 1);
        }
        puVar13 = (unsigned int *)((param_3 - 4) + (int)puVar4);
        local_41c = (int)((param_3 - 4) + (unsigned int)local_41c);
        if (bVar15) {
            if (DAT_00733794 == 0) {
                puVar11 = DAT_00733590;
                for (uVar6 = DAT_00733790 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
                    __builtin_memcpy(puVar13, puVar11, 4);
                    puVar11 = puVar11 + 1;
                    puVar13 = puVar13 + 1;
                }
                for (uVar6 = DAT_00733790 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
                    *(unsigned char *)puVar13 = *(unsigned char *)puVar11;
                    puVar11 = (unsigned int *)((int)puVar11 + 1);
                    puVar13 = (unsigned int *)((int)puVar13 + 1);
                }
                local_41c = (int)((unsigned int)local_41c + DAT_00733790);
            } else {
                *(unsigned char *)puVar13 = 0x39;
                local_41c = (int)((unsigned int)local_41c + 1);
                DAT_00733794 = 0;
            }
        }
        puVar4 = param_2 + 1;
        puVar12 = (unsigned int *)((int)DAT_00733590 + 1);
        *(unsigned char *)DAT_00733590 = bVar3;
        for (uVar6 = uVar7 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
            __builtin_memcpy(puVar12, puVar4, 4);
            puVar4 = puVar4 + 1;
            puVar12 = puVar12 + 1;
        }
        for (uVar7 = uVar7 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
            *(char *)puVar12 = (char)*puVar4;
            puVar4 = (unsigned int *)((int)puVar4 + 1);
            puVar12 = (unsigned int *)((int)puVar12 + 1);
        }
        DAT_00733790 = param_3 - 3;
    }
    else if (param_1 == 0) {
        local_400 = (unsigned short)(uVar10 | (((unsigned short)DAT_00733380 & 0x7f) << 8));
        DAT_00733380 = (int)((unsigned int)DAT_00733380 + 1);
        if (0x7f < DAT_00733380) {
            DAT_00733380 = 0;
        }
        puVar4 = (unsigned int *)&local_3fe;
        local_41c = 2;
        if (bVar15) {
            local_41c = 4;
            local_3fe = (short)(sVar2 - 4);
            puVar4 = local_3fc;
        }
        uVar7 = param_3 - 4;
        puVar12 = param_2 + 1;
        puVar14 = puVar4;
        for (uVar6 = uVar7 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
            __builtin_memcpy(puVar14, puVar12, 4);
            puVar12 = puVar12 + 1;
            puVar14 = puVar14 + 1;
        }
        for (uVar6 = uVar7 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
            *(char *)puVar14 = (char)*puVar12;
            puVar12 = (unsigned int *)((int)puVar12 + 1);
            puVar14 = (unsigned int *)((int)puVar14 + 1);
        }
        puVar13 = (unsigned int *)((param_3 - 4) + (int)puVar4);
        local_41c = (int)((param_3 - 4) + (unsigned int)local_41c);
        if (bVar15) {
            if (DAT_00733588 == 0) {
                puVar11 = DAT_00733384;
                for (uVar6 = DAT_00733584 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
                    __builtin_memcpy(puVar13, puVar11, 4);
                    puVar11 = puVar11 + 1;
                    puVar13 = puVar13 + 1;
                }
                for (uVar6 = DAT_00733584 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
                    *(unsigned char *)puVar13 = *(unsigned char *)puVar11;
                    puVar11 = (unsigned int *)((int)puVar11 + 1);
                    puVar13 = (unsigned int *)((int)puVar13 + 1);
                }
                local_41c = (int)((unsigned int)local_41c + DAT_00733584);
            } else {
                *(unsigned char *)puVar13 = 0x39;
                local_41c = (int)((unsigned int)local_41c + 1);
                DAT_00733588 = 0;
            }
        }
        puVar4 = param_2 + 1;
        puVar12 = (unsigned int *)((int)DAT_00733384 + 1);
        *(unsigned char *)DAT_00733384 = bVar3;
        for (uVar6 = uVar7 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
            __builtin_memcpy(puVar12, puVar4, 4);
            puVar4 = puVar4 + 1;
            puVar12 = puVar12 + 1;
        }
        for (uVar7 = uVar7 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
            *(char *)puVar12 = (char)*puVar4;
            puVar4 = (unsigned int *)((int)puVar4 + 1);
            puVar12 = (unsigned int *)((int)puVar12 + 1);
        }
        DAT_00733584 = param_3 - 3;
    }
    else if (DAT_007330a0 == param_1) {
        local_400 = (unsigned short)(uVar10 | (((unsigned short)DAT_0073358c & 0x7f) << 8) | 0x8080);
        DAT_0073358c = (int)((unsigned int)DAT_0073358c + 1);
        if (0x7f < DAT_0073358c) {
            DAT_0073358c = 0;
        }
        puVar4 = (unsigned int *)&local_3fe;
        local_41c = 2;
        if (bVar15) {
            local_41c = 4;
            local_3fe = (short)(sVar2 - 4);
            puVar4 = local_3fc;
        }
        uVar7 = param_3 - 4;
        puVar12 = param_2 + 1;
        puVar14 = puVar4;
        for (uVar6 = uVar7 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
            __builtin_memcpy(puVar14, puVar12, 4);
            puVar12 = puVar12 + 1;
            puVar14 = puVar14 + 1;
        }
        for (uVar6 = uVar7 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
            *(char *)puVar14 = (char)*puVar12;
            puVar12 = (unsigned int *)((int)puVar12 + 1);
            puVar14 = (unsigned int *)((int)puVar14 + 1);
        }
        puVar13 = (unsigned int *)((param_3 - 4) + (int)puVar4);
        local_41c = (int)((param_3 - 4) + (unsigned int)local_41c);
        if (bVar15) {
            if (DAT_00733794 == 0) {
                puVar11 = DAT_00733590;
                for (uVar6 = DAT_00733790 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
                    __builtin_memcpy(puVar13, puVar11, 4);
                    puVar11 = puVar11 + 1;
                    puVar13 = puVar13 + 1;
                }
                for (uVar6 = DAT_00733790 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
                    *(unsigned char *)puVar13 = *(unsigned char *)puVar11;
                    puVar11 = (unsigned int *)((int)puVar11 + 1);
                    puVar13 = (unsigned int *)((int)puVar13 + 1);
                }
                local_41c = (int)((unsigned int)local_41c + DAT_00733790);
            } else {
                *(unsigned char *)puVar13 = 0x39;
                local_41c = (int)((unsigned int)local_41c + 1);
                DAT_00733794 = 0;
            }
        }
        puVar4 = param_2 + 1;
        puVar12 = (unsigned int *)((int)DAT_00733590 + 1);
        *(unsigned char *)DAT_00733590 = bVar3;
        for (uVar6 = uVar7 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
            __builtin_memcpy(puVar12, puVar4, 4);
            puVar4 = puVar4 + 1;
            puVar12 = puVar12 + 1;
        }
        for (uVar7 = uVar7 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
            *(char *)puVar12 = (char)*puVar4;
            puVar4 = (unsigned int *)((int)puVar4 + 1);
            puVar12 = (unsigned int *)((int)puVar12 + 1);
        }
        DAT_00733790 = param_3 - 3;
    }
    else {
        uVar7 = FUN_00433540(param_1);
        if ((uVar7 < DAT_00739290) && (uVar7 < 0x28)) {
            iVar9 = DAT_007337b4[uVar7 * 0x8e];
            iVar5 = (int)((unsigned int)iVar9 + 1);
            DAT_007337b4[uVar7 * 0x8e] = iVar5;
            uVar10 = (unsigned short)(uVar10 | (((unsigned short)iVar9 & 0x7f) << 8));
            if (0x7f < iVar5) {
                DAT_007337b4[uVar7 * 0x8e] = 0;
            }
        }
        local_400 = (unsigned short)(uVar10 | 0x8000);
        puVar4 = (unsigned int *)&local_3fe;
        local_41c = 2;
        if (bVar15) {
            local_3fe = (short)(sVar2 - 4);
            puVar4 = local_3fc;
            local_41c = 4;
        }
        puVar12 = param_2 + 1;
        puVar14 = puVar4;
        for (uVar6 = (param_3 - 4) >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
            __builtin_memcpy(puVar14, puVar12, 4);
            puVar12 = puVar12 + 1;
            puVar14 = puVar14 + 1;
        }
        for (uVar6 = (param_3 - 4) & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
            *(char *)puVar14 = (char)*puVar12;
            puVar12 = (unsigned int *)((int)puVar12 + 1);
            puVar14 = (unsigned int *)((int)puVar14 + 1);
        }
        local_41c = (int)((param_3 - 4) + (unsigned int)local_41c);
        if (bVar15) {
            uVar6 = DAT_007339b8[uVar7 * 0x8e];
            puVar13 = (unsigned int *)(DAT_007337b8 + uVar7 * 0x238);
            puVar11 = (unsigned int *)((param_3 - 4) + (int)puVar4);
            for (uVar8 = uVar6 >> 2; uVar8 != 0; uVar8 = uVar8 - 1) {
                __builtin_memcpy(puVar11, puVar13, 4);
                puVar13 = puVar13 + 1;
                puVar11 = puVar11 + 1;
            }
            for (uVar6 = uVar6 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
                *(unsigned char *)puVar11 = *(unsigned char *)puVar13;
                puVar13 = (unsigned int *)((int)puVar13 + 1);
                puVar11 = (unsigned int *)((int)puVar11 + 1);
            }
            local_41c = (int)((unsigned int)local_41c + DAT_007339b8[uVar7 * 0x8e]);
        }
        DAT_007337b8[uVar7 * 0x238] = bVar3;
        puVar4 = param_2 + 1;
        puVar12 = (unsigned int *)(DAT_007337b9 + uVar7 * 0x238);
        for (uVar6 = (param_3 - 4) >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
            __builtin_memcpy(puVar12, puVar4, 4);
            puVar4 = puVar4 + 1;
            puVar12 = puVar12 + 1;
        }
        for (uVar6 = (param_3 - 4) & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
            *(char *)puVar12 = (char)*puVar4;
            puVar4 = (unsigned int *)((int)puVar4 + 1);
            puVar12 = (unsigned int *)((int)puVar12 + 1);
        }
        DAT_007339b8[uVar7 * 0x8e] = param_3 - 3;
    }
    uVar10 = local_400;
    if (((uVar1 != 1) || (DAT_0085e78f != '\x01')) && (DAT_007330f8 != param_1)) {
        bVar3 = (unsigned char)(local_400 >> 8);
        if (uVar1 == 2) {
            puVar4 = param_2;
            puVar12 = (unsigned int *)(DAT_005ce228 + (int)((unsigned int)DAT_005251a8 * 0x210));
            for (uVar7 = param_3 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
                *puVar12 = *puVar4;
                puVar4 = puVar4 + 1;
                puVar12 = puVar12 + 1;
            }
            for (uVar7 = param_3 & 3; iVar9 = DAT_005251a8, uVar7 != 0; uVar7 = uVar7 - 1) {
                *(char *)puVar12 = (char)*puVar4;
                puVar4 = (unsigned int *)((int)puVar4 + 1);
                puVar12 = (unsigned int *)((int)puVar12 + 1);
            }
            iVar5 = (int)((unsigned int)DAT_005251a8 * 0x210);
            DAT_005ce218[(int)((unsigned int)DAT_005251a8 * 0x84)] = (unsigned int)param_1;
            DAT_005ce21c[(int)((unsigned int)iVar9 * 0x84)] = param_3;
            *(unsigned int *)(DAT_005ce220 + iVar5) = 0;
            DAT_005ce224[iVar5] = 0;
            DAT_005ce225[iVar5] = 0;
            DAT_005ce226[iVar5] = bVar3 & 0x7f;
            DAT_005251a8 = (int)((unsigned int)DAT_005251a8 + 1);
            if (0xff < DAT_005251a8) {
                DAT_005251a8 = 0;
            }
        }
        puVar4 = param_2;
        puVar12 = (unsigned int *)(DAT_005bda28 + (int)((unsigned int)DAT_005251a4 * 0x210));
        for (uVar7 = param_3 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
            *puVar12 = *puVar4;
            puVar4 = puVar4 + 1;
            puVar12 = puVar12 + 1;
        }
        for (uVar7 = param_3 & 3; iVar9 = DAT_005251a4, uVar7 != 0; uVar7 = uVar7 - 1) {
            *(char *)puVar12 = (char)*puVar4;
            puVar4 = (unsigned int *)((int)puVar4 + 1);
            puVar12 = (unsigned int *)((int)puVar12 + 1);
        }
        iVar5 = (int)((unsigned int)DAT_005251a4 * 0x210);
        DAT_005bda18[(int)((unsigned int)DAT_005251a4 * 0x84)] = (unsigned int)param_1;
        DAT_005bda1c[(int)((unsigned int)iVar9 * 0x84)] = param_3;
        *(unsigned int *)(DAT_005bda20 + iVar5) = 0;
        DAT_005bda24[iVar5] = 0;
        iVar9 = DAT_007330a0;
        if (param_1 == 0) {
            DAT_005bda25[iVar5] = 0;
        } else {
            DAT_005bda25[iVar5] = 2;
            if (iVar9 != param_1) {
                DAT_005bda25[iVar5] = 1;
            }
        }
        iVar9 = (int)((unsigned int)DAT_005251a4 + 1);
        DAT_005251a4 = iVar9;
        DAT_005bda26[iVar5] = bVar3 & 0x7f;
        if (0x7f < iVar9) {
            DAT_005251a4 = 0;
        }
    }
    if (((DAT_007330f8 == param_1) || (param_1 == 0)) ||
        ((DAT_00733074 == (int *)0) || (DAT_007330a0 == param_1))) {
        if (0x3ff < DAT_005251b4) {
            FUN_004336c0();
            if (0x3ff < DAT_005251b4) goto LAB_00431a3e;
        }
        puVar4 = DAT_00539a18 + (int)((unsigned int)DAT_005251ac * 0x84);
        for (uVar7 = param_3 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
            *puVar4 = *param_2;
            param_2 = param_2 + 1;
            puVar4 = puVar4 + 1;
        }
        for (uVar7 = param_3 & 3; iVar9 = DAT_005251ac, uVar7 != 0; uVar7 = uVar7 - 1) {
            *(char *)puVar4 = (char)*param_2;
            param_2 = (unsigned int *)((int)param_2 + 1);
            puVar4 = (unsigned int *)((int)puVar4 + 1);
        }
        iVar5 = (int)((unsigned int)DAT_005251ac * 0x210);
        DAT_00539a08[(int)((unsigned int)DAT_005251ac * 0x84)] = (unsigned int)DAT_007330f8;
        DAT_00539a0c[(int)((unsigned int)iVar9 * 0x84)] = param_3;
        DAT_00539a10[(int)((unsigned int)iVar9 * 0x84)] = 0;
        DAT_00539a14[iVar5] = 0;
        DAT_00539a17[iVar5] = 0;
        uVar6 = FUN_00433540(DAT_007330f8);
        uVar7 = DAT_00739290;
        if ((uVar1 == 1) && (DAT_0085e78f == '\x01')) {
            bVar15 = uVar6 < DAT_00739290;
            DAT_00539a15[(int)((unsigned int)DAT_005251ac * 0x210)] = 2;
            if (bVar15 < 0x28) {
                DAT_007337b0[uVar6 * 0x8e] = (unsigned int)((uVar10 & 0x7f00) >> 8);
            }
        }
        else if (param_1 == 0) {
            DAT_00539a15[(int)((unsigned int)DAT_005251ac * 0x210)] = 0;
            if ((uVar6 < uVar7) && (uVar6 < 0x28)) {
                DAT_007337a8[uVar6 * 0x8e] = (unsigned int)((uVar10 & 0x7f00) >> 8);
            }
        }
        else if (DAT_007330a0 == param_1) {
            bVar15 = uVar6 < DAT_00739290;
            DAT_00539a15[(int)((unsigned int)DAT_005251ac * 0x210)] = 2;
            if (bVar15 < 0x28) {
                DAT_007337b0[uVar6 * 0x8e] = (unsigned int)((uVar10 & 0x7f00) >> 8);
            }
        }
        else {
            bVar15 = uVar6 < DAT_00739290;
            DAT_00539a15[(int)((unsigned int)DAT_005251ac * 0x210)] = 1;
            if (bVar15 < 0x28) {
                DAT_007337a0[uVar6 * 0x8e] = (unsigned int)((uVar10 & 0x7f00) >> 8);
            }
        }
        DAT_005251b4 = (int)((unsigned int)DAT_005251b4 + 1);
        DAT_00539a16[(int)((unsigned int)DAT_005251ac * 0x210)] = (unsigned char)(local_400 >> 8) & 0x7f;
        DAT_005251ac = (int)((unsigned int)DAT_005251ac + 1);
        if (0x3ff < DAT_005251ac) {
            DAT_005251ac = 0;
        }
    }
LAB_00431a3e:
    if (DAT_00733074 == (int *)0) {
        return 1;
    }
    if (DAT_007330f8 != param_1) {
        local_404 = ((int (__stdcall *)(int *, int, int, int, unsigned short *, int))
                     *(void **)(*DAT_00733074 + 0x68))
                    (DAT_00733074, DAT_007330f8, param_1, 0, &local_400, local_41c);
    }
    return local_404 == 0;
}
