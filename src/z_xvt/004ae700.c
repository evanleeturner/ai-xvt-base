/* SPDX-License-Identifier: MIT */
/* z_xvt 004ae700 */
extern int DAT_0052552c;
extern unsigned short DAT_00655184;
extern unsigned short DAT_00656c90[];
extern unsigned char DAT_00655ab0[];
extern unsigned int DAT_0071f2f0[];
extern int DAT_0071ecf4;
extern int DAT_006d4988;
extern unsigned char DAT_0071b730;
extern void (__cdecl *DAT_0070a184)(unsigned int, unsigned short, short, int, int);

void __stdcall FUN_004ae700(void)
{
    char cVar1;
    unsigned char bVar2;
    unsigned short uVar3;
    int iVar4;
    unsigned short uVar5;
    int iVar6;
    unsigned int uVar7;
    unsigned int uVar8;
    short sVar9;
    unsigned short uVar10;
    unsigned short uVar11;
    int iVar12;
    int bVar13;
    unsigned int local_12;
    unsigned short local_e;
    unsigned short local_c;
    short local_a;
    int tmp;
    unsigned short us;

    __builtin_memcpy(&iVar6, &DAT_0071b730 + (unsigned int)DAT_0071ecf4 * 0x5bd, 4);
    iVar12 = (int)((unsigned int)iVar6 * 0x23u + (unsigned int)DAT_006d4988);
    __builtin_memcpy(&tmp, (unsigned char *)iVar12 + 0x1f, 4);
    __builtin_memcpy(&iVar4, (unsigned char *)tmp + 0xa7, 4);

    if (iVar6 == -1) {
        bVar13 = 0;
    } else {
        cVar1 = *(char *)(iVar12 + 3);
        if (cVar1 != 1 && cVar1 != 2 && cVar1 != 3 && cVar1 != 0xe && cVar1 != 4) {
            bVar13 = 0;
        } else {
            bVar13 = 1;
        }
    }

    if (!bVar13) {
        if (DAT_0052552c == 0x13) {
            local_a = 2;
        } else {
            local_a = (short)((unsigned short)((unsigned short)(-(unsigned short)(DAT_0052552c == 0x112)) & 0xfffe) + 6);
        }

        if ((*(unsigned char *)(iVar4 + 0x1dc) & 2) != 0) {
            uVar7 = (unsigned short)(DAT_00655184 + 0x1b);
            uVar5 = (unsigned short)((unsigned short)*(unsigned char *)(iVar4 + 0x212) * 3);
            if (DAT_00656c90[uVar7] != uVar5) {
                uVar10 = 0;
                DAT_00656c90[uVar7] = uVar5;
                iVar6 = uVar7 * 0xc;
                uVar3 = *(unsigned short *)(DAT_00655ab0 + iVar6);
                sVar9 = *(short *)(DAT_00655ab0 + iVar6 + 2);
                uVar11 = *(unsigned short *)(DAT_00655ab0 + iVar6 + 4);
                do {
                    bVar13 = uVar10 < uVar5;
                    uVar10 = uVar10 + 1;
                    (*DAT_0070a184)(DAT_0071f2f0[(unsigned int)bVar13 + (unsigned int)uVar11], uVar3, sVar9, 0xfd, 0);
                    sVar9 = (short)((unsigned int)sVar9 - (unsigned int)local_a);
                } while (uVar10 < 0xc);
            }
        }

        __builtin_memcpy(&iVar6, &DAT_0071b730 + (unsigned int)DAT_0071ecf4 * 0x5bd, 4);
        __builtin_memcpy(&tmp, (unsigned char *)((unsigned int)iVar6 * 0x23u + 0x1fu + (unsigned int)DAT_006d4988), 4);
        __builtin_memcpy(&iVar6, (unsigned char *)tmp + 0xa7, 4);
        if (((*(unsigned char *)(iVar6 + 0x1dc) & 8) != 0) && ((*(unsigned char *)(iVar6 + 0x1dd) & 1) != 0)) {
            uVar7 = (unsigned short)(DAT_00655184 + 0x1c);
            uVar5 = (unsigned short)((unsigned short)*(unsigned char *)(iVar6 + 0x20f) * 3);
            if (DAT_00656c90[uVar7] != uVar5) {
                uVar10 = 0;
                DAT_00656c90[uVar7] = uVar5;
                iVar6 = uVar7 * 0xc;
                uVar3 = *(unsigned short *)(DAT_00655ab0 + iVar6);
                sVar9 = *(short *)(DAT_00655ab0 + iVar6 + 2);
                uVar11 = *(unsigned short *)(DAT_00655ab0 + iVar6 + 4);
                do {
                    bVar13 = uVar10 < uVar5;
                    uVar10 = uVar10 + 1;
                    (*DAT_0070a184)(DAT_0071f2f0[(unsigned int)bVar13 + (unsigned int)uVar11], uVar3, sVar9, 0xfd, 0);
                    sVar9 = (short)((unsigned int)sVar9 - (unsigned int)local_a);
                } while (uVar10 < 0xc);
            }
        }

        __builtin_memcpy(&iVar6, &DAT_0071b730 + (unsigned int)DAT_0071ecf4 * 0x5bd, 4);
        __builtin_memcpy(&tmp, (unsigned char *)((unsigned int)iVar6 * 0x23u + 0x1fu + (unsigned int)DAT_006d4988), 4);
        __builtin_memcpy(&iVar6, (unsigned char *)tmp + 0xa7, 4);
        if (((*(unsigned char *)(iVar6 + 0x1dc) & 0x10) != 0) && ((*(unsigned char *)(iVar6 + 0x1de) & 1) != 0)) {
            uVar7 = (unsigned short)(DAT_00655184 + 0x1d);
            uVar5 = (unsigned short)((unsigned short)*(unsigned char *)(iVar6 + 0x234) * 3);
            if (DAT_00656c90[uVar7] != uVar5) {
                uVar10 = 0;
                DAT_00656c90[uVar7] = uVar5;
                iVar6 = uVar7 * 0xc;
                uVar3 = *(unsigned short *)(DAT_00655ab0 + iVar6);
                sVar9 = *(short *)(DAT_00655ab0 + iVar6 + 2);
                uVar11 = *(unsigned short *)(DAT_00655ab0 + iVar6 + 4);
                do {
                    bVar13 = uVar10 < uVar5;
                    uVar10 = uVar10 + 1;
                    (*DAT_0070a184)(DAT_0071f2f0[(unsigned int)bVar13 + (unsigned int)uVar11], uVar3, sVar9, 0xfd, 0);
                    sVar9 = (short)((unsigned int)sVar9 - (unsigned int)local_a);
                } while (uVar10 < 0xc);
            }
        }

        __builtin_memcpy(&iVar6, &DAT_0071b730 + (unsigned int)DAT_0071ecf4 * 0x5bd, 4);
        __builtin_memcpy(&tmp, (unsigned char *)((unsigned int)iVar6 * 0x23u + 0x1fu + (unsigned int)DAT_006d4988), 4);
        __builtin_memcpy(&iVar6, (unsigned char *)tmp + 0xa7, 4);
        if ((*(unsigned char *)(iVar6 + 0x1dc) & 4) == 0) {
            return;
        }
        uVar5 = (unsigned short)(8 - *(unsigned char *)(iVar6 + 0x212));
        __builtin_memcpy(&us, (unsigned char *)iVar6 + 0x1dd, 2);
        if ((us & 1) != 0) {
            uVar5 = (unsigned short)((uVar5 - *(unsigned char *)(iVar6 + 0x20f)) + 2);
        }
        __builtin_memcpy(&us, (unsigned char *)iVar6 + 0x1dd, 2);
        if ((us & 0x100) != 0) {
            uVar5 = (unsigned short)((uVar5 - *(unsigned char *)(iVar6 + 0x234)) + 2);
        }
        uVar7 = (unsigned short)(DAT_00655184 + 0x1a);
        if (DAT_00656c90[uVar7] == uVar5) {
            return;
        }
        DAT_00656c90[uVar7] = uVar5;
        iVar6 = uVar7 * 0xc;
        uVar3 = *(unsigned short *)(DAT_00655ab0 + iVar6);
        sVar9 = *(short *)(DAT_00655ab0 + iVar6 + 2);
        uVar11 = *(unsigned short *)(DAT_00655ab0 + iVar6 + 4);
        uVar10 = 0;
        do {
            bVar13 = uVar10 < uVar5;
            uVar10 = uVar10 + 1;
            (*DAT_0070a184)(DAT_0071f2f0[(unsigned int)bVar13 + (unsigned int)uVar11], uVar3, sVar9, 0xfd, 0);
            sVar9 = (short)((unsigned int)sVar9 - (unsigned int)local_a);
        } while (uVar10 < 0xc);
        return;
    }

    __builtin_memcpy(&us, (unsigned char *)iVar4 + 0x1db, 2);
    if ((us & 0xe00) == 0) {
        return;
    }
    if (DAT_0052552c == 0x13) {
        local_a = 2;
    } else if (DAT_0052552c == 0x101) {
        local_a = 5;
    } else {
        if (DAT_0052552c != 0x112) goto LAB_004aebb5;
        local_a = 3;
    }
    local_c = 4;
LAB_004aebb5:
    bVar2 = *(unsigned char *)(iVar4 + 0x20f);
    local_e = (unsigned short)*(unsigned char *)(iVar4 + 0x212);
    uVar5 = (unsigned short)((8 - local_e) - (unsigned short)bVar2);
    uVar7 = (unsigned int)bVar2;
    if (DAT_00655184 != 0) {
        local_c = (unsigned short)(local_c * 2);
        local_e = (unsigned short)((unsigned short)*(unsigned char *)(iVar4 + 0x212) * 2);
        uVar5 = (unsigned short)(uVar5 * 2);
        uVar7 = (unsigned int)(unsigned short)((unsigned short)bVar2 * 2);
    }
    local_12 = uVar7 << 0x10;

    __builtin_memcpy(&us, (unsigned char *)iVar4 + 0x1db, 2);
    if (((us & 0x400) != 0) &&
        (uVar8 = (unsigned short)(DAT_00655184 + 0x1a), DAT_00656c90[uVar8] != uVar5)) {
        DAT_00656c90[uVar8] = uVar5;
        iVar6 = uVar8 * 0xc;
        uVar3 = *(unsigned short *)(DAT_00655ab0 + iVar6);
        uVar11 = *(unsigned short *)(DAT_00655ab0 + iVar6 + 4);
        uVar10 = 0;
        local_12 = ((unsigned int)(unsigned short)uVar7 << 16) |
                   (unsigned int)*(unsigned short *)(DAT_00655ab0 + iVar6 + 2);
        if ((local_c & 0x7fff) != 0) {
            do {
                bVar13 = uVar10 < uVar5;
                uVar10 = uVar10 + 1;
                (*DAT_0070a184)(DAT_0071f2f0[(unsigned int)bVar13 + (unsigned int)uVar11], uVar3,
                                (short)(local_12 & 0xffff), 0xfd, 0);
                local_12 = (local_12 & 0xffff0000u) |
                           (unsigned int)(unsigned short)(short)((unsigned int)(short)local_12 - (unsigned int)local_a);
            } while (uVar10 < (unsigned short)(local_c * 2));
        }
    }

    if ((*(unsigned char *)(iVar4 + 0x1dc) & 8) != 0) {
        uVar7 = (unsigned short)(DAT_00655184 + 0x1c);
        if (DAT_00656c90[uVar7] != (unsigned short)(local_12 >> 16)) {
            uVar11 = 0;
            DAT_00656c90[uVar7] = (unsigned short)(local_12 >> 16);
            iVar6 = uVar7 * 0xc;
            uVar3 = *(unsigned short *)(DAT_00655ab0 + iVar6);
            sVar9 = *(short *)(DAT_00655ab0 + iVar6 + 2);
            uVar5 = *(unsigned short *)(DAT_00655ab0 + iVar6 + 4);
            if (local_c != 0) {
                do {
                    bVar13 = uVar11 < (unsigned short)(local_12 >> 16);
                    uVar11 = uVar11 + 1;
                    (*DAT_0070a184)(DAT_0071f2f0[(unsigned int)bVar13 + (unsigned int)uVar5], uVar3, sVar9, 0xfd, 0);
                    sVar9 = (short)((unsigned int)sVar9 + (unsigned int)((int)local_a * -2));
                } while (uVar11 < local_c);
            }
        }
    }

    if (((*(unsigned char *)(iVar4 + 0x1dc) & 2) != 0) &&
        (uVar7 = (unsigned short)(DAT_00655184 + 0x1b), DAT_00656c90[uVar7] != local_e)) {
        uVar11 = 0;
        DAT_00656c90[uVar7] = local_e;
        iVar6 = uVar7 * 0xc;
        uVar3 = *(unsigned short *)(DAT_00655ab0 + iVar6);
        sVar9 = *(short *)(DAT_00655ab0 + iVar6 + 2);
        uVar5 = *(unsigned short *)(DAT_00655ab0 + iVar6 + 4);
        if (local_c != 0) {
            do {
                bVar13 = uVar11 < local_e;
                uVar11 = uVar11 + 1;
                (*DAT_0070a184)(DAT_0071f2f0[(unsigned int)bVar13 + (unsigned int)uVar5], uVar3, sVar9, 0xfd, 0);
                sVar9 = (short)((unsigned int)sVar9 + (unsigned int)((int)local_a * -2));
            } while (uVar11 < local_c);
        }
    }
    return;
}
