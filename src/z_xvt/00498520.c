/* SPDX-License-Identifier: MIT */
/* z_xvt 00498520 */
void FUN_0042a140(unsigned int, unsigned int);
void FUN_0040a1a0(unsigned int, unsigned int, int);
void FUN_004a1d70(int, int);
void FUN_004a2e30(void);
int FUN_0042b1e0(int, int);

extern short DAT_0071e6a0;
extern short DAT_0071e6b6;
extern unsigned short DAT_0071e6be;
extern int DAT_00525550;
extern short DAT_0070ac40;
extern short DAT_0070ac50;
extern char DAT_0071b740;
extern char DAT_0071e528;
extern unsigned char DAT_0071bcdd;
extern unsigned char DAT_0071eac5;
extern int _DAT_00732a3c;
extern int DAT_006d4980;
extern int DAT_006d4988;
extern int DAT_00732e54;
extern int DAT_00703acc;
extern int DAT_0073088c;
extern short _DAT_006da9e4;
extern unsigned short _DAT_0070a13c;
extern int DAT_0071ecf4;
extern unsigned char DAT_0071b74a;
extern unsigned char DAT_0071b730;
extern unsigned char DAT_0071b759;
extern int DAT_006db580;
extern unsigned char DAT_0052686c;
extern unsigned short DAT_007091ac;
extern unsigned short DAT_006fedd0;
extern unsigned short _DAT_00732e5a;
extern short _DAT_0071f2de;
extern unsigned char DAT_0071f2dd;
extern unsigned char DAT_0071f2dc;
extern unsigned char DAT_0071f2db;
extern unsigned char DAT_0070ac34;
extern char DAT_0070ac35;
extern unsigned char DAT_007092dc;
extern unsigned char DAT_007092dd;
extern char DAT_007092de;
extern unsigned char DAT_007092c0;
extern int DAT_007092e4;
extern char DAT_006d4a02;
extern char DAT_0070ad54;
extern unsigned char DAT_0070ad48;
extern char DAT_00709d0a;
extern char DAT_00709d0b;
extern char DAT_00709d0d;
extern unsigned char DAT_0071e518;
extern unsigned short DAT_00523b70;
extern unsigned short DAT_00523b84;
extern unsigned char DAT_00523b88;
extern unsigned short _DAT_00732952;
extern unsigned short _DAT_00732950;
extern int DAT_00732a38;

void __stdcall FUN_00498520(void)
{
    char cVar1;
    int bVar2;
    short sVar3;
    short *psVar4;
    int iVar5;
    int iVar6;
    short *psVar9;
    unsigned short uVar8;
    unsigned short uVar10;
    int iVar12;
    unsigned char *pbVar11;
    unsigned short *puVar13;
    char *pcVar14;
    int iVar15;
    unsigned char *pbVar16;
    unsigned int uVar17;
    unsigned short local_12;
    int local_c[3];
    int t_i;
    int t_i2;
    unsigned short t_us;
    unsigned short t_us2;
    short t_s;

    psVar9 = &DAT_0071e6a0;
    do {
        if ((*psVar9 != 0) && (sVar3 = (short)(*psVar9 - DAT_0071e6be), *psVar9 = sVar3, sVar3 < 0)) {
            *psVar9 = 0;
        }
        psVar9 = psVar9 + 1;
    } while (psVar9 < &DAT_0071e6b6);
    if (DAT_00525550 == 0) {
        psVar9 = &DAT_0070ac40;
        do {
            pcVar14 = &DAT_0071b740;
            psVar4 = psVar9;
            do {
                if (((*pcVar14 != '\0') && (*psVar4 != 0)) &&
                    (sVar3 = (short)(*psVar4 - DAT_0071e6be), *psVar4 = sVar3, sVar3 < 0)) {
                    *psVar4 = 0;
                }
                psVar4 = psVar4 + 8;
                pcVar14 = pcVar14 + 0x5bd;
            } while (pcVar14 < &DAT_0071e528);
            psVar9 = psVar9 + 1;
        } while (psVar9 < &DAT_0070ac50);
    }
    pbVar11 = &DAT_0071bcdd;
    do {
        __builtin_memcpy(&t_i, pbVar11, 4);
        if (t_i != 0) {
            iVar5 = (int)((unsigned int)t_i - (unsigned int)DAT_0071e6be);
            __builtin_memcpy(pbVar11, &iVar5, 4);
            if (iVar5 < 0) {
                t_i2 = 0;
                __builtin_memcpy(pbVar11, &t_i2, 4);
            }
        }
        __builtin_memcpy(&t_i, pbVar11 + 4, 4);
        if (t_i != 0) {
            iVar5 = (int)((unsigned int)t_i - (unsigned int)DAT_0071e6be);
            __builtin_memcpy(pbVar11 + 4, &iVar5, 4);
            if (iVar5 < 0) {
                t_i2 = 0;
                __builtin_memcpy(pbVar11 + 4, &t_i2, 4);
            }
        }
        pbVar11 = pbVar11 + 0x5bd;
    } while (pbVar11 < &DAT_0071eac5);
    if (_DAT_00732a3c < DAT_006d4980) {
        iVar5 = (int)((unsigned int)_DAT_00732a3c * 0x23);
        iVar15 = _DAT_00732a3c;
        do {
            if (*(char *)((unsigned int)DAT_006d4988 + iVar5 + 3) != '\0') {
                __builtin_memcpy(&iVar12, (void *)((unsigned int)DAT_006d4988 + iVar5 + 0x1f), 4);
                __builtin_memcpy(&iVar12, (void *)((unsigned int)iVar12 + 0xa7), 4);
                DAT_00732e54 = iVar12;
                if (*(int *)((unsigned int)iVar12 + 0x3c) != 0) {
                    *(unsigned int *)((unsigned int)iVar12 + 0x3c) =
                        (unsigned int)*(int *)((unsigned int)iVar12 + 0x3c) - (unsigned int)DAT_0071e6be;
                }
                if ((*(int *)((unsigned int)iVar12 + 0x60) != 0) &&
                    (iVar6 = (int)((unsigned int)*(int *)((unsigned int)iVar12 + 0x60) - (unsigned int)DAT_0071e6be),
                     *(int *)((unsigned int)iVar12 + 0x60) = iVar6, iVar6 < 0)) {
                    *(int *)((unsigned int)iVar12 + 0x60) = 0;
                }
                if ((*(short *)((unsigned int)iVar12 + 100) != 0) &&
                    (sVar3 = (short)(*(short *)((unsigned int)iVar12 + 100) - DAT_0071e6be),
                     *(short *)((unsigned int)iVar12 + 100) = sVar3, sVar3 < 0)) {
                    *(short *)((unsigned int)iVar12 + 100) = 0;
                }
                __builtin_memcpy(&uVar10, (void *)((unsigned int)DAT_00732e54 + 0x1e1), 2);
                if (uVar10 != 0) {
                    t_us = (unsigned short)(uVar10 - DAT_0071e6be);
                    __builtin_memcpy((void *)((unsigned int)DAT_00732e54 + 0x1e1), &t_us, 2);
                    __builtin_memcpy(&t_us2, (void *)((unsigned int)DAT_00732e54 + 0x1e1), 2);
                    if (uVar10 < t_us2) {
                        t_us = 0;
                        __builtin_memcpy((void *)((unsigned int)DAT_00732e54 + 0x1e1), &t_us, 2);
                    }
                }
                if (*(short *)((unsigned int)DAT_00732e54 + 0x240) != 0) {
                    *(unsigned short *)((unsigned int)DAT_00732e54 + 0x240) =
                        (unsigned short)(*(short *)((unsigned int)DAT_00732e54 + 0x240) - DAT_0071e6be);
                    if (*(short *)((unsigned int)DAT_00732e54 + 0x240) < 0) {
                        *(short *)((unsigned int)DAT_00732e54 + 0x240) = 0;
                    }
                }
                iVar12 = 0;
                if (*(char *)((unsigned int)DAT_00732e54 + 0x213) != '\0') {
                    do {
                        iVar6 = FUN_0042b1e0(DAT_00732e54, iVar12);
                        if (0 < *(short *)((unsigned int)iVar6 + 2)) {
                            *(unsigned short *)((unsigned int)iVar6 + 2) =
                                (unsigned short)(*(short *)((unsigned int)iVar6 + 2) - DAT_0071e6be);
                        }
                        iVar12 = iVar12 + 1;
                    } while (iVar12 < (int)(unsigned int)*(unsigned char *)((unsigned int)DAT_00732e54 + 0x213));
                }
            }
            iVar5 = iVar5 + 0x23;
            iVar15 = iVar15 + 1;
        } while (iVar15 < DAT_006d4980);
    }
    if (DAT_00703acc < DAT_0073088c) {
        iVar5 = (int)((unsigned int)DAT_00703acc * 0x23);
        iVar15 = DAT_00703acc;
        do {
            if (*(char *)((unsigned int)DAT_006d4988 + iVar5 + 3) != '\0') {
                __builtin_memcpy(&iVar12, (void *)((unsigned int)DAT_006d4988 + iVar5 + 0x1f), 4);
                __builtin_memcpy(&iVar12, (void *)((unsigned int)iVar12 + 0xab), 4);
                __builtin_memcpy(&t_i, (void *)((unsigned int)iVar12 + 0x16), 4);
                if (t_i != 0) {
                    t_i = (int)((unsigned int)t_i - (unsigned int)DAT_0071e6be);
                    __builtin_memcpy((void *)((unsigned int)iVar12 + 0x16), &t_i, 4);
                }
                __builtin_memcpy(&t_i, (void *)((unsigned int)iVar12 + 0x3a), 4);
                if (t_i != 0) {
                    iVar6 = (int)((unsigned int)t_i - (unsigned int)DAT_0071e6be);
                    __builtin_memcpy((void *)((unsigned int)iVar12 + 0x3a), &iVar6, 4);
                    if (iVar6 < 0) {
                        t_i = 0;
                        __builtin_memcpy((void *)((unsigned int)iVar12 + 0x3a), &t_i, 4);
                    }
                }
                if ((*(short *)((unsigned int)iVar12 + 0x3e) != 0) &&
                    (sVar3 = (short)(*(short *)((unsigned int)iVar12 + 0x3e) - DAT_0071e6be),
                     *(short *)((unsigned int)iVar12 + 0x3e) = sVar3, sVar3 < 0)) {
                    *(short *)((unsigned int)iVar12 + 0x3e) = 0;
                }
            }
            iVar5 = iVar5 + 0x23;
            iVar15 = iVar15 + 1;
        } while (iVar15 < DAT_0073088c);
    }
    _DAT_006da9e4 = (short)(_DAT_006da9e4 - DAT_0071e6be);
    if (_DAT_006da9e4 < 0) {
        _DAT_0070a13c = _DAT_0070a13c ^ 0x400;
        __builtin_memcpy(&uVar10, &DAT_0071b74a + (unsigned int)DAT_0071ecf4 * 0x5bd, 2);
        if (uVar10 != 0xffff) {
            __builtin_memcpy(&uVar17, &DAT_0071b730 + (unsigned int)DAT_0071ecf4 * 0x5bd, 4);
            if (uVar17 != 0xffffffff) {
                FUN_0042a140((unsigned int)uVar10, uVar17);
                DAT_006db580 = DAT_006db580 >> 5;
                __builtin_memcpy(&t_us, &DAT_0071b74a + (unsigned int)DAT_0071ecf4 * 0x5bd, 2);
                *(unsigned short *)&local_c[0] =
                    *(unsigned short *)(&DAT_0052686c +
                        (unsigned int)*(unsigned char *)((unsigned int)t_us * 0x23 + 3 + (unsigned int)DAT_006d4988) * 0x16);
            }
        }
        if ((_DAT_0070a13c & 0x400) == 0) {
            _DAT_006da9e4 = 0xe;
            if ((int)(unsigned int)(unsigned short)local_c[0] <= DAT_006db580) goto LAB_004988be;
        }
        else if (DAT_006db580 < (int)(unsigned int)(unsigned short)local_c[0]) {
LAB_004988be:
            _DAT_006da9e4 = 0x76;
        }
        else {
            _DAT_006da9e4 = 0xe;
        }
    }
    __builtin_memcpy(&DAT_007091ac, &DAT_0071b759 + (unsigned int)DAT_0071ecf4 * 0x5bd, 2);
    __builtin_memcpy(&t_us, &DAT_0071b74a + (unsigned int)DAT_0071ecf4 * 0x5bd, 2);
    DAT_006fedd0 = (unsigned short)(t_us | _DAT_00732e5a | _DAT_0070a13c);
    _DAT_0071f2de = (short)(_DAT_0071f2de - DAT_0071e6be);
    if (0 < _DAT_0071f2de) {
        return;
    }
    _DAT_0071f2de = (short)(_DAT_0071f2de + 0xec);
    DAT_0071f2dd = (unsigned char)(DAT_0071f2dd + 1);
    if (0x3b < DAT_0071f2dd) {
        DAT_0071f2dd = 0;
        DAT_0071f2dc = (unsigned char)(DAT_0071f2dc + 1);
        if (0x3b < DAT_0071f2dc) {
            DAT_0071f2dc = 0;
            DAT_0071f2db = (unsigned char)(DAT_0071f2db + 1);
            if (0x17 < DAT_0071f2db) {
                DAT_0071f2db = 0;
            }
        }
    }
    if ((DAT_0070ac34 == 0) && (DAT_0070ac35 == '\0')) {
LAB_00498a03:
        if (DAT_007092dc != 0) {
            if ((DAT_0070ac34 == 2) && (DAT_0070ac35 == '\0')) {
                FUN_0040a1a0(0x23, 0xffffffff, DAT_0071ecf4);
                FUN_004a1d70(200, DAT_0071ecf4);
            }
            if ((DAT_0070ac34 == 1) && (DAT_0070ac35 == '\0')) {
                FUN_0040a1a0(0x59, 0xffffffff, DAT_0071ecf4);
                FUN_004a1d70(0xc9, DAT_0071ecf4);
            }
            if (DAT_0070ac34 == 0) {
                if (DAT_0070ac35 == '\x0f') {
                    __builtin_memcpy(&iVar5, &DAT_0071b730 + (unsigned int)DAT_0071ecf4 * 0x5bd, 4);
                    if ((iVar5 == -1) ||
                        ((((cVar1 = *(char *)((unsigned int)iVar5 * 0x23 + 3 + (unsigned int)DAT_006d4988), cVar1 != '\x01') &&
                           (cVar1 != '\x02')) && (cVar1 != '\x03')) && ((cVar1 != '\x0e') && (cVar1 != '\x04')))) {
                        bVar2 = 0;
                    }
                    else {
                        bVar2 = 1;
                    }
                    if (bVar2) {
                        uVar17 = 0x54;
                    }
                    else {
                        FUN_0040a1a0(0x59, 0xffffffff, DAT_0071ecf4);
                        uVar17 = 0x59;
                    }
                    FUN_0040a1a0(uVar17, 0xffffffff, DAT_0071ecf4);
                }
                if ((DAT_0070ac34 == 0) && (DAT_0070ac35 == '\x02')) {
                    FUN_004a1d70(0xca, DAT_0071ecf4);
                }
            }
        }
    }
    else {
        DAT_0070ac35 = (char)(DAT_0070ac35 + -1);
        if (DAT_0070ac35 == -1) {
            DAT_0070ac35 = ';';
            DAT_0070ac34 = (unsigned char)(DAT_0070ac34 - 1);
            if (DAT_0070ac34 == 0xff) {
                DAT_0070ac35 = '\0';
                DAT_0070ac34 = 0;
            }
        }
        if (DAT_007092dc != 0) {
            if (((DAT_0070ac34 == 0) && (DAT_0070ac35 == '\0')) && (DAT_00525550 == 0)) {
                pcVar14 = &DAT_0071b740;
                do {
                    *pcVar14 = 2;
                    pcVar14 = pcVar14 + 0x5bd;
                } while (pcVar14 < &DAT_0071e528);
                DAT_007092c0 = 1;
            }
            goto LAB_00498a03;
        }
    }
    if (((DAT_007092e4 < 2) || (DAT_007092dd == 0)) || (DAT_007092de != '\0')) goto LAB_00498ce9;
    if (((DAT_006d4a02 == '\x02') || (DAT_006d4a02 == '\x04')) &&
        ((DAT_007092dd < DAT_0070ac34) || ((DAT_0070ac34 == 0) && (DAT_0070ac35 == '\0')))) {
        local_c[0] = 0;
        local_c[1] = 0;
        local_c[2] = 0;
        pcVar14 = &DAT_0071b740;
        do {
            if (*pcVar14 == '\x01') {
                __builtin_memcpy(&t_us, pcVar14 + -4, 2);
                *((char *)local_c + (unsigned int)t_us) = 1;
            }
            pcVar14 = pcVar14 + 0x5bd;
        } while (pcVar14 < &DAT_0071e528);
        if ((DAT_006d4a02 == '\x02') && (_DAT_00732a3c < DAT_006d4980)) {
            iVar5 = (int)((unsigned int)DAT_006d4980 - (unsigned int)_DAT_00732a3c);
            pbVar16 = (unsigned char *)((unsigned int)_DAT_00732a3c * 0x23 + 0x16 + (unsigned int)DAT_006d4988);
            do {
                if ((pbVar16[-0x13] != 0) && ((&DAT_0070ad54)[(unsigned int)*pbVar16 * 0x566] != '\0')) {
                    *((char *)local_c + (unsigned int)(unsigned char)(&DAT_0070ad48)[(unsigned int)*pbVar16 * 0x566]) = 1;
                }
                pbVar16 = pbVar16 + 0x23;
                iVar5 = iVar5 + -1;
            } while (iVar5 != 0);
        }
        iVar5 = 0;
        iVar15 = 0;
        do {
            if (*((char *)local_c + iVar15) != '\0') {
                iVar5 = iVar5 + 1;
                local_c[0] = iVar15;
            }
            iVar15 = iVar15 + 1;
        } while (iVar15 < 10);
        if (iVar5 == 1) {
            if (DAT_006d4a02 != '\x02') {
                if ((((&DAT_00709d0a)[local_c[0] * 3] != '\x01') &&
                     ((&DAT_00709d0a)[local_c[0] * 3] != '\x02')) &&
                    ((&DAT_00709d0b)[local_c[0] * 3] != '\x01')) goto LAB_00498ca8;
            }
            DAT_0070ac35 = '\0';
            DAT_007092de = '\x01';
            DAT_0070ac34 = DAT_007092dd;
            DAT_007092dc = DAT_007092dd;
        }
    }
LAB_00498ca8:
    if (((DAT_007092de == '\0') && (DAT_006d4a02 == '\x04')) &&
        ((DAT_00709d0a == '\x01') || (DAT_00709d0d == '\x01'))) {
        DAT_0070ac35 = '\0';
        DAT_007092de = '\x01';
        DAT_0070ac34 = DAT_007092dd;
        DAT_007092dc = DAT_007092dd;
    }
LAB_00498ce9:
    local_c[0] = 0;
    pbVar11 = &DAT_0071b730;
    do {
        __builtin_memcpy(&iVar5, pbVar11, 4);
        if (((char)pbVar11[16] != '\0') && (iVar5 != -1)) {
            uVar10 = 0xffff;
            local_12 = 0xffff;
            __builtin_memcpy(&iVar12, pbVar11, 4);
            __builtin_memcpy(&iVar5, (void *)((unsigned int)iVar12 * 0x23 + 0x1f + (unsigned int)DAT_006d4988), 4);
            __builtin_memcpy(&iVar5, (void *)((unsigned int)iVar5 + 0xa7), 4);
            __builtin_memcpy(&t_s, (void *)((unsigned int)iVar5 + 0x1df), 2);
            if (t_s != 0) {
                iVar15 = 0;
                do {
                    if ((*(short *)((unsigned int)iVar5 + 0x29e + (unsigned int)iVar15 * 2) == 0) &&
                        (uVar8 = (unsigned short)*(unsigned char *)((unsigned int)iVar5 + 0x293 + iVar15), uVar8 < uVar10)) {
                        local_12 = (unsigned short)iVar15;
                        uVar10 = uVar8;
                    }
                    iVar15 = iVar15 + 1;
                } while (iVar15 < 10);
                uVar17 = 0;
                puVar13 = &DAT_00523b70;
                psVar9 = (short *)((unsigned int)iVar5 + 0x2b4);
                do {
                    if ((psVar9[-0xb] == 0) && (local_12 == uVar17)) {
                        if (*psVar9 == 0) {
                            psVar9[-0xb] = 100;
                            __builtin_memcpy(&t_us, (void *)((unsigned int)iVar5 + 0x1df), 2);
                            t_us = (unsigned short)(t_us | *puVar13);
                            __builtin_memcpy((void *)((unsigned int)iVar5 + 0x1df), &t_us, 2);
                            _DAT_00732952 = 0x58;
                            _DAT_00732950 = (unsigned short)(unsigned char)(&DAT_00523b88)[uVar17];
                            FUN_004a1d70(0x56, local_c[0]);
                        }
                        else {
                            *psVar9 = (short)(*psVar9 + -1);
                        }
                    }
                    puVar13 = puVar13 + 1;
                    psVar9 = psVar9 + 1;
                    uVar17 = uVar17 + 1;
                } while (puVar13 < &DAT_00523b84);
            }
        }
        pbVar11 = pbVar11 + 0x5bd;
        local_c[0] = local_c[0] + 1;
    } while (pbVar11 < &DAT_0071e518);
    iVar5 = 0;
    if (0 < DAT_00732a38) {
        iVar15 = 0;
        do {
            if (*(char *)((unsigned int)DAT_006d4988 + iVar15 + 3) != '\0') {
                __builtin_memcpy(&iVar12, (void *)((unsigned int)DAT_006d4988 + iVar15 + 0x1f), 4);
                __builtin_memcpy(&t_s, (void *)((unsigned int)iVar12 + 0x81), 2);
                t_s = (short)(t_s + 1);
                __builtin_memcpy((void *)((unsigned int)iVar12 + 0x81), &t_s, 2);
            }
            iVar15 = iVar15 + 0x23;
            iVar5 = iVar5 + 1;
        } while (iVar5 < DAT_00732a38);
    }
    FUN_004a2e30();
    return;
}
