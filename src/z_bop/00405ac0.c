/* SPDX-License-Identifier: MIT */
/* z_bop 00405ac0 */
extern int DAT_009a1fe8;
extern int DAT_00a08104;
extern int DAT_009a8e18;
extern int DAT_009e9640;
extern int DAT_009ecc34;
extern unsigned char DAT_0051c5a0[];
extern unsigned char DAT_0051c5a2[];
extern unsigned char DAT_0051c59e[];
extern char DAT_009d8c84[];
extern char DAT_009d9155[];
extern char DAT_009e968a[];
extern char DAT_009e96f9[];
extern char DAT_009e96f5[];

void __cdecl FUN_00450650(int a, int b);
unsigned int __cdecl FUN_004065d0(int a, unsigned int b, unsigned int c);
unsigned int __cdecl FUN_004269e0(int a);
void __cdecl FUN_0042e720(unsigned int a, int b);

void __cdecl FUN_00405ac0(int param_1, int param_2)
{
    unsigned char *pbVar1;
    unsigned char bVar3;
    unsigned char bVar4;
    unsigned short uVar5;
    int iVar6;
    unsigned short uVar7;
    unsigned int uVar8;
    int iVar9;
    unsigned int uVar10;
    unsigned int uVar11;
    char cVar12;
    int iVar13;
    short sVar15;
    unsigned int uVar16;
    int iVar17;
    unsigned int uVar18;
    unsigned int local_16 = 0;
    unsigned short local_12 = 0;
    unsigned short uStack_e = 0;
    unsigned char uStack_c = 0;
    unsigned char uStack_b = 0;
    short ss;
    unsigned int ui;

    iVar9 = (int)((unsigned int)DAT_009a1fe8 + (unsigned int)param_1 * 0x23u);
    __builtin_memcpy(&iVar6, (char *)iVar9 + 0x1b, 4);
    __builtin_memcpy(&iVar13, (char *)iVar9 + 0x1f, 4);
    __builtin_memcpy(&iVar9, (char *)iVar13 + 0xa9, 4);
    bVar3 = *(unsigned char *)(iVar9 + 4);
    uVar7 = (unsigned short)bVar3;
    DAT_00a08104 = iVar9;

    if (*(char *)(iVar9 + 0x29) == '\0') {
        uVar16 = 0;
        iVar13 = iVar9 + param_2;
        cVar12 = *(char *)(iVar13 + 0x220);
        if (cVar12 == '\x01') {
            iVar17 = (int)((unsigned int)uVar7 * 0x101u + (unsigned int)param_2);
            if ((*(unsigned char *)(iVar13 + 0x224) < DAT_0051c5a0[iVar17]) ||
                (DAT_0051c5a2[iVar17] < *(unsigned char *)(iVar13 + 0x224))) {
                *(unsigned char *)(iVar13 + 0x224) = DAT_0051c5a0[iVar17];
            }
            bVar4 = *(unsigned char *)(DAT_00a08104 + 0x224 + param_2);
            *(unsigned char *)(DAT_00a08104 + 0x224 + param_2) = (unsigned char)(bVar4 + 1);
            local_12 = (unsigned short)bVar4;
            pbVar1 = (unsigned char *)(DAT_00a08104 + 0x224 + param_2);
            if (DAT_0051c5a2[iVar17] < *pbVar1) {
                *pbVar1 = DAT_0051c5a0[iVar17];
            }
            local_16 = (local_16 & 0xffffu) | (1u << 16);
            uStack_c = 1;
            uStack_b = 0;
            uStack_e = local_12;
        }
        else if (cVar12 == '\x02') {
            unsigned char *pbVar14 = (unsigned char *)(iVar13 + 0x224);
            iVar13 = (int)((unsigned int)uVar7 * 0x101u + (unsigned int)param_2);
            pbVar1 = &DAT_0051c5a0[iVar13];
            if ((*pbVar14 < DAT_0051c5a0[iVar13]) || (DAT_0051c5a2[iVar13] < *pbVar14)) {
                *pbVar14 = *pbVar1;
            }
            bVar4 = *(unsigned char *)(DAT_00a08104 + 0x224 + param_2);
            *(unsigned char *)(DAT_00a08104 + 0x224 + param_2) = (unsigned char)(bVar4 ^ 1);
            local_12 = (unsigned short)bVar4;
            pbVar14 = (unsigned char *)(DAT_00a08104 + 0x224 + param_2);
            if (DAT_0051c5a2[iVar13] < *pbVar14) {
                *pbVar14 = *pbVar1;
            }
            uStack_e = (unsigned short)DAT_0051c5a2[iVar13];
            uStack_c = 2;
            uStack_b = 0;
            local_16 = (local_16 & 0xffffu) |
                       ((unsigned int)(unsigned short)(((int)uStack_e - (int)(unsigned int)*pbVar1 + 1) / 2) << 16);
        }
        else if (cVar12 == '\x03') {
            local_12 = (unsigned short)DAT_0051c5a0[(unsigned int)param_2 + (unsigned int)uVar7 * 0x101u];
            uStack_c = 1;
            uStack_b = 0;
            uStack_e = (unsigned short)DAT_0051c5a2[(unsigned int)uVar7 * 0x101u + (unsigned int)param_2];
            local_16 = (local_16 & 0xffffu) |
                       ((unsigned int)(unsigned short)((unsigned int)uStack_e - (unsigned int)local_12 + 1u) << 16);
        }

        local_16 = (local_16 & 0xffff0000u) | (unsigned int)local_12;
        uVar8 = local_16;
        uVar5 = local_12;
        while (uVar5 <= uStack_e) {
            uVar18 = uVar8 & 0xffffu;
            iVar13 = (int)((unsigned int)DAT_00a08104 + uVar18 * 4u);
            local_16 = (local_16 & 0xffffu) | (uVar8 & 0xffff0000u);
            if ((*(char *)(iVar13 + 0x370) != '\0') && ('\0' < *(char *)(iVar13 + 0x371))) {
                local_12 = (unsigned short)DAT_0051c59e[(unsigned int)param_2 + (unsigned int)uVar7 * 0x101u];
                if ('?' < *(char *)(iVar13 + 0x371)) {
                    local_12 = (unsigned short)(local_12 + 1);
                }
                uVar10 = FUN_004065d0(param_1, uVar18, (unsigned int)local_12);
                if (uVar10 != 0xffffffffu) {
                    iVar13 = (int)((unsigned int)*(unsigned char *)((int)((unsigned int)DAT_009a1fe8 + 0x16u + (unsigned int)param_1 * 0x23u)) * 0x566u);
                    if ((DAT_009d8c84[iVar13] != '\x15') && (DAT_009d9155[iVar13] != '\x15')) {
                        if (iVar6 == -1) {
                            cVar12 = (char)(*(char *)((unsigned int)DAT_00a08104 + 0x371u + uVar18 * 4u) + -1);
                        }
                        else {
                            uVar11 = FUN_004269e0(5);
                            if (((unsigned short)uVar11 == uVar7) ||
                                (uVar11 = FUN_004269e0(7), (unsigned short)uVar11 == uVar7)) {
                                cVar12 = (char)(*(char *)((unsigned int)DAT_00a08104 + 0x371u + uVar18 * 4u) + -3);
                            }
                            else {
                                cVar12 = (char)(*(char *)((unsigned int)DAT_00a08104 + 0x371u + uVar18 * 4u) + -4);
                            }
                        }
                        *(char *)((unsigned int)DAT_00a08104 + 0x371u + uVar18 * 4u) = cVar12;
                    }
                    if ((unsigned short)(local_16 >> 16) < 2) {
                        if ((unsigned short)uVar16 < 2) goto LAB_00405e65;
                    }
                    else if ((uVar16 & 1) == 0) {
LAB_00405e65:
                        FUN_0042e720(uVar10, iVar6);
                    }
                    if (*(char *)((unsigned int)DAT_00a08104 + 0x371u + uVar18 * 4u) < '\0') {
                        *(char *)((unsigned int)DAT_00a08104 + 0x371u + uVar18 * 4u) = 0;
                    }
                    iVar13 = (int)((uVar10 - (unsigned int)DAT_009e9640) * 10u);
                    if (iVar6 == -1) {
                        *(short *)(DAT_009a8e18 + 4 + iVar13) = *(short *)(iVar9 + 0x42);
                        uVar5 = *(unsigned short *)(iVar9 + 0x42);
                        if ((uVar5 == 0xffff) || (0x7fff < uVar5)) goto LAB_00405f40;
                        __builtin_memcpy((char *)(DAT_009a8e18 + 6 + iVar13),
                                         (char *)((unsigned int)uVar5 * 0x23u + (unsigned int)DAT_009a1fe8), 2);
                    }
                    else {
                        __builtin_memcpy((char *)(DAT_009a8e18 + 4 + iVar13),
                                         &DAT_009e968a[(int)((unsigned int)iVar6 * 0x5bdu)], 2);
                        __builtin_memcpy(&uVar5, &DAT_009e968a[(int)((unsigned int)iVar6 * 0x5bdu)], 2);
                        if (uVar5 == 0xffff) {
LAB_00405f40:
                            *(short *)(DAT_009a8e18 + 6 + iVar13) = 0;
                        }
                        else {
                            __builtin_memcpy((char *)(DAT_009a8e18 + 6 + iVar13),
                                             (char *)((unsigned int)uVar5 * 0x23u + (unsigned int)DAT_009a1fe8), 2);
                        }
                    }
                    uVar16 = (unsigned int)(unsigned short)((unsigned short)uVar16 + 1);
                    *(char *)(DAT_009a8e18 + iVar13) = (char)iVar6;
                }
            }
            local_16 = (local_16 & 0xffff0000u) | (uVar8 & 0xffffu);
            if ((unsigned short)((unsigned short)(local_16 >> 16) - 1) == 0) break;
            uVar5 = (unsigned short)((unsigned short)local_16 +
                                     (unsigned short)(((unsigned int)uStack_b << 8) | (unsigned int)uStack_c));
            local_16 = ((unsigned int)(unsigned short)((unsigned short)(local_16 >> 16) - 1) << 16) | (unsigned int)uVar5;
            uVar8 = local_16;
        }

        sVar15 = (short)uVar16;
        if ((local_12 == 0x8d) || (local_12 == 0x8e)) {
            *(short *)(DAT_00a08104 + 0x250) = (short)(*(short *)(DAT_00a08104 + 0x250) + sVar15);
            if (iVar6 != -1) {
                __builtin_memcpy(&ss, &DAT_009e96f9[(int)((unsigned int)iVar6 * 0x5bdu)], 2);
                ss = (short)(ss + sVar15);
                __builtin_memcpy(&DAT_009e96f9[(int)((unsigned int)iVar6 * 0x5bdu)], &ss, 2);
            }
        }
        else {
            *(short *)(DAT_00a08104 + 0x24c) = (short)(*(short *)(DAT_00a08104 + 0x24c) + sVar15);
            if (iVar6 != -1) {
                __builtin_memcpy(&ss, &DAT_009e96f5[(int)((unsigned int)iVar6 * 0x5bdu)], 2);
                ss = (short)(ss + sVar15);
                __builtin_memcpy(&DAT_009e96f5[(int)((unsigned int)iVar6 * 0x5bdu)], &ss, 2);
            }
        }
        *(short *)(DAT_00a08104 + 0x226 + (int)((unsigned int)param_2 * 2u)) =
            (short)(*(short *)(DAT_00a08104 + 0x226 + (int)((unsigned int)param_2 * 2u)) + sVar15 * 0x2f + 2);
        __builtin_memcpy(&ui, (char *)(DAT_00a08104 + 0x22a + (int)((unsigned int)param_2 * 4u)), 4);
        ui = ui + uVar16 * 0x2fu + 2u;
        __builtin_memcpy((char *)(DAT_00a08104 + 0x22a + (int)((unsigned int)param_2 * 4u)), &ui, 4);
    }
    else if (iVar6 == DAT_009ecc34) {
        FUN_00450650(0x82, DAT_009ecc34);
        return;
    }
    return;
}
