/* SPDX-License-Identifier: MIT */
/* z_bop 004fad00 */
extern int DAT_00b69e30;
extern int DAT_00b6a24c;
extern int DAT_00aa5cb0[];
extern int DAT_00aa5ac0[];
extern int DAT_00aa5e10[];
extern int DAT_00aa5e68;
extern short DAT_00aa5aac;
extern char DAT_00bb73a4;
extern char DAT_00a92203[];
extern char DAT_00a91d24;
extern char DAT_00a91d17[];
extern char DAT_00a91d12[];

/* Globals not aligned to their access width: byte arrays, accessed via __builtin_memcpy only */
extern unsigned char DAT_00aa615e[];
extern unsigned char DAT_00aa617a[];
extern unsigned char DAT_00aa629e[];
extern unsigned char DAT_00aa62ae[];
extern unsigned char DAT_00b6bcbe[4];
extern unsigned char DAT_00b6bcc2[];
extern unsigned char DAT_00b6bce2[];
extern unsigned char DAT_00b6bd02[];
extern unsigned char DAT_00b6bdc2[];
extern unsigned char DAT_00b6be82[];
extern unsigned char DAT_00b6bea2[];
extern unsigned char DAT_00b6bec2[];
extern unsigned char DAT_00b6bf82[];
extern unsigned char DAT_00b6c102[];
extern unsigned char DAT_00b6d84a[];
extern unsigned char DAT_00b6d5a6[];
extern unsigned char DAT_00b6d5ca[];
extern unsigned char DAT_00b6d5ce[];
extern unsigned char DAT_00b6a5a6[4];
extern unsigned char DAT_00b6a5aa[4];
extern unsigned char DAT_00b6a5ae[24];
extern unsigned char DAT_00b6a5b2[4];
extern unsigned char DAT_00b6a5b6[4];
extern unsigned char DAT_00b6a5ba[4];
extern unsigned char DAT_00b6a5be[4];
extern unsigned char DAT_00b6a606[4];
extern unsigned char DAT_00b6a60a[4];
extern unsigned char DAT_00b6baca[4];
extern unsigned char DAT_00b6bad2[];
extern unsigned char DAT_00b6bb9a[4];
extern unsigned char DAT_00b6bb9e[4];
extern unsigned char DAT_00b6bc2a[4];
extern unsigned char DAT_00bb00f6[4];
extern unsigned char DAT_00b6d962[4];
extern unsigned char DAT_00b6d9aa[4];
extern unsigned char DAT_00b7e38a[4];
extern unsigned char DAT_00b8ed2a[4];
extern unsigned char DAT_00b8ed2e[4];
extern unsigned char DAT_00b8ed32[24];
extern unsigned char DAT_00b8ed6a[4];
extern unsigned char DAT_00b8ed6e[4];
extern unsigned char DAT_00b6d96a[];
extern unsigned char DAT_00b6d96e[];
extern unsigned char DAT_00b6d972[];
extern unsigned char DAT_00b6d976[];
extern unsigned char DAT_00b6d97a[];
extern unsigned char DAT_00b6d97e[];
extern unsigned char DAT_00b6d982[];
extern unsigned char DAT_00b6d9ae[];

/* Calling conventions of these callees are not confirmed by the evidence; cdecl assumed. */
void * __cdecl FUN_004d12f0(int param_1);
int __cdecl FUN_004d11e0(void);
void __cdecl FUN_004d1ca0(void);

void __stdcall FUN_004fad00(void)
{
    unsigned char uVar1;
    unsigned int uVar2;
    unsigned int uVar3;
    int iVar4;
    unsigned char *puVar5;
    int *piVar6;
    int iVar7;
    char *pcVar8;
    unsigned int uVar9;
    unsigned char *puVar10;
    int iVar11;
    int iVar12;
    int *piVar13;
    int *piVar14;
    int *local_10;
    int local_c;
    int local_8;
    int local_4;
    int local_14;
    int local_18;
    int local_1c;
    short local_20;
    int local_24;
    int local_28;
    int local_2c;
    int local_30;
    int local_34;
    int local_38;
    int local_3c;
    int local_40;
    int local_44;
    int local_4c;

    local_14 = 0;
    local_18 = -1;
    local_1c = 1;
    local_20 = 0;

    if (DAT_00b69e30 == 2) {
        puVar5 = DAT_00aa617a;
        do {
            puVar10 = puVar5 + 0x2a;
            for (iVar7 = 10; iVar7 != 0; iVar7 = iVar7 + -1) {
                __builtin_memcpy(puVar5, &local_14, 4);
                puVar5 = puVar5 + 4;
            }
            __builtin_memcpy(puVar5, &local_20, 2);
            puVar5 = puVar10;
        } while (puVar10 < DAT_00aa629e + 2);
    }
    else {
        piVar13 = (int *)DAT_00aa615e;
        do {
            __builtin_memcpy(&local_30, piVar13, 4);
            if ((local_30 != 0) &&
                (puVar5 = (unsigned char *)FUN_004d12f0(local_30), puVar5 == (unsigned char *)0)) {
                puVar5 = (unsigned char *)piVar13 - 0xe;
                for (iVar7 = 10; iVar7 != 0; iVar7 = iVar7 + -1) {
                    __builtin_memcpy(puVar5, &local_14, 4);
                    puVar5 = puVar5 + 4;
                }
                __builtin_memcpy(puVar5, &local_20, 2);
            }
            piVar13 = (int *)((int)piVar13 + 0x2a);
        } while (piVar13 < (int *)DAT_00aa62ae);
    }
    __builtin_memcpy(DAT_00b6bcbe, &local_14, 4);
    DAT_00b6a24c = 0;
    puVar5 = DAT_00b6bcc2;
    for (iVar7 = 8; iVar7 != 0; iVar7 = iVar7 + -1) {
        __builtin_memcpy(puVar5, &local_14, 4);
        puVar5 = puVar5 + 4;
    }
    puVar5 = DAT_00b6bce2;
    for (iVar7 = 8; iVar7 != 0; iVar7 = iVar7 + -1) {
        __builtin_memcpy(puVar5, &local_14, 4);
        puVar5 = puVar5 + 4;
    }
    puVar5 = DAT_00b6bd02;
    for (iVar7 = 0x30; iVar7 != 0; iVar7 = iVar7 + -1) {
        __builtin_memcpy(puVar5, &local_14, 4);
        puVar5 = puVar5 + 4;
    }
    puVar5 = DAT_00b6bdc2;
    for (iVar7 = 0x30; iVar7 != 0; iVar7 = iVar7 + -1) {
        __builtin_memcpy(puVar5, &local_14, 4);
        puVar5 = puVar5 + 4;
    }
    puVar5 = DAT_00b6be82;
    for (iVar7 = 8; iVar7 != 0; iVar7 = iVar7 + -1) {
        __builtin_memcpy(puVar5, &local_14, 4);
        puVar5 = puVar5 + 4;
    }
    puVar5 = DAT_00b6bea2;
    for (iVar7 = 8; iVar7 != 0; iVar7 = iVar7 + -1) {
        __builtin_memcpy(puVar5, &local_14, 4);
        puVar5 = puVar5 + 4;
    }
    puVar5 = DAT_00b6bec2;
    for (iVar7 = 0x30; iVar7 != 0; iVar7 = iVar7 + -1) {
        __builtin_memcpy(puVar5, &local_14, 4);
        puVar5 = puVar5 + 4;
    }
    puVar5 = DAT_00b6bf82;
    for (iVar7 = 0x30; iVar7 != 0; iVar7 = iVar7 + -1) {
        __builtin_memcpy(puVar5, &local_14, 4);
        puVar5 = puVar5 + 4;
    }
    puVar5 = DAT_00b6c102;
    for (iVar7 = 0x522; iVar7 != 0; iVar7 = iVar7 + -1) {
        __builtin_memcpy(puVar5, &local_14, 4);
        puVar5 = puVar5 + 4;
    }
    puVar5 = DAT_00b6d84a;
    for (iVar7 = 0x46; iVar7 != 0; iVar7 = iVar7 + -1) {
        __builtin_memcpy(puVar5, &local_14, 4);
        puVar5 = puVar5 + 4;
    }
    local_c = 0;
    __builtin_memcpy(&local_24, DAT_00b6a5aa, 4);
    __builtin_memcpy(&local_28, DAT_00b6a606, 4);
    __builtin_memcpy(&local_2c, DAT_00b6baca, 4);
    if (((local_24 == 1) && (local_28 == 1)) && (local_2c == 0)) {
        puVar5 = DAT_00b6bad2;
        do {
            __builtin_memcpy(puVar5, &local_18, 4);
            puVar5 = puVar5 + 20;
        } while (puVar5 < DAT_00b6bb9a);
    }
    local_4 = 0;
    local_10 = (int *)DAT_00aa615e;
    piVar13 = (int *)DAT_00b6d5ce;
    do {
        __builtin_memcpy(&local_30, local_10, 4);
        if (local_30 != 0) {
            __builtin_memcpy(&uVar2, (unsigned char *)local_10 - 10, 4);
            local_c = local_c + 1;
            __builtin_memcpy(&uVar3, (unsigned char *)local_10 - 6, 4);
            __builtin_memcpy((unsigned char *)piVar13 - 0x36, (unsigned char *)local_10 - 0xe, 4);
            uVar1 = *((unsigned char *)local_10 - 2);
            __builtin_memcpy((unsigned char *)piVar13 - 0x32, &uVar2, 4);
            __builtin_memcpy((unsigned char *)piVar13 - 0x2e, &uVar3, 4);
            __builtin_memcpy(&iVar7, (unsigned char *)local_10 + 8, 4);
            *((unsigned char *)piVar13 - 0x2a) = uVar1;
            __builtin_memcpy(&iVar12, (unsigned char *)local_10 + 12, 4);
            *((unsigned char *)piVar13 - 0x2a) = 0;
            __builtin_memcpy((unsigned char *)piVar13 - 4, &iVar7, 4);
            __builtin_memcpy((unsigned char *)piVar13, &iVar12, 4);
            __builtin_memcpy(&local_34, (unsigned char *)local_10 + 16, 4);
            local_34 = (int)((unsigned int)local_34 - 1u);
            __builtin_memcpy((unsigned char *)piVar13 + 4, &local_34, 4);
            __builtin_memcpy(&iVar7, local_10, 4);
            __builtin_memcpy(&iVar12, (unsigned char *)local_10 + 4, 4);
            __builtin_memcpy(&local_34, (unsigned char *)local_10 + 20, 4);
            local_34 = (int)((unsigned int)local_34 - 1u);
            __builtin_memcpy((unsigned char *)piVar13 + 8, &local_34, 4);
            __builtin_memcpy(&local_34, (unsigned char *)local_10 + 24, 4);
            local_34 = (int)((unsigned int)local_34 - 1u);
            __builtin_memcpy((unsigned char *)piVar13 + 12, &local_34, 4);
            __builtin_memcpy((unsigned char *)piVar13 - 36, &iVar7, 4);
            __builtin_memcpy((unsigned char *)piVar13 - 32, &iVar12, 4);
            __builtin_memcpy((unsigned char *)piVar13 - 28, &local_14, 4);
            __builtin_memcpy((unsigned char *)piVar13 - 24, &local_14, 4);
            __builtin_memcpy((unsigned char *)piVar13 - 20, &local_14, 4);
            __builtin_memcpy((unsigned char *)piVar13 - 16, &local_14, 4);
            __builtin_memcpy((unsigned char *)piVar13 - 12, &local_14, 4);
            __builtin_memcpy((unsigned char *)piVar13 - 8, &local_14, 4);
            __builtin_memcpy((unsigned char *)piVar13 + 16, &local_14, 4);
            local_8 = 10;
            iVar12 = 0;
            do {
                iVar11 = 8;
                do {
                    if (*(int *)((char *)DAT_00aa5cb0 + iVar12) == iVar7) {
                        iVar4 = *(int *)((char *)DAT_00aa5ac0 + iVar12);
                        __builtin_memcpy((unsigned char *)piVar13 - 40, &iVar4, 4);
                        __builtin_memcpy(&local_38, (unsigned char *)piVar13 - 4, 4);
                        __builtin_memcpy(&local_3c, (unsigned char *)piVar13, 4);
                        if (((local_38 == 0) && (local_3c != -1)) &&
                            (DAT_00a92203[(int)((unsigned int)local_3c + (unsigned int)iVar4 * 0x562u)] == '\0')) {
                            __builtin_memcpy((unsigned char *)piVar13, &local_18, 4);
                        }
                    }
                    iVar12 = iVar12 + 4;
                    iVar11 = iVar11 + -1;
                } while (iVar11 != 0);
                local_8 = local_8 + -1;
            } while (local_8 != 0);
            iVar12 = FUN_004d11e0();
            if (iVar12 == iVar7) {
                DAT_00b6a24c = local_4;
            }
        }
        piVar13 = (int *)((int)piVar13 + 0x58);
        local_4 = local_4 + 1;
        local_10 = (int *)((int)local_10 + 0x2a);
    } while (local_10 < (int *)DAT_00aa62ae);

    __builtin_memcpy(&local_24, DAT_00b6a5aa, 4);
    __builtin_memcpy(&local_28, DAT_00b6a606, 4);
    if ((local_24 == 3) && (local_28 == 1)) {
        __builtin_memcpy(DAT_00b6bc2a, &local_c, 4);
    }
    if ((local_24 == 0) && (local_28 == 1)) {
        __builtin_memcpy(DAT_00bb00f6, &local_c, 4);
    }
    if (local_24 == 1) {
        __builtin_memcpy(&local_2c, DAT_00b6baca, 4);
        if ((local_28 == 1) && (local_2c == 0)) {
            if (0 < DAT_00aa5e68) {
                puVar5 = DAT_00b6bad2;
                piVar13 = DAT_00aa5cb0;
                piVar14 = DAT_00aa5e10;
                iVar7 = DAT_00aa5e68;
                do {
                    iVar11 = 0;
                    iVar12 = *piVar14;
                    piVar6 = piVar13;
                    if (0 < iVar12) {
                        do {
                            if (*piVar6 != 0) break;
                            iVar11 = iVar11 + 1;
                            piVar6 = piVar6 + 1;
                        } while (iVar11 < iVar12);
                        if (iVar11 < iVar12) {
                            __builtin_memcpy(puVar5, &local_14, 4);
                        }
                    }
                    puVar5 = puVar5 + 20;
                    piVar13 = piVar13 + 8;
                    piVar14 = piVar14 + 1;
                    iVar7 = iVar7 + -1;
                } while (iVar7 != 0);
            }
            if (((DAT_00bb73a4 == '\x01') || (DAT_00b69e30 == 2)) && (0 < DAT_00aa5aac)) {
                pcVar8 = &DAT_00a91d24;
                iVar7 = (int)DAT_00aa5aac;
                do {
                    if (*pcVar8 != '\0') {
                        __builtin_memcpy(DAT_00b6bad2 + (unsigned int)(unsigned char)pcVar8[-0xc] * 20u,
                                         &local_14, 4);
                    }
                    pcVar8 = pcVar8 + 0x562;
                    iVar7 = iVar7 + -1;
                } while (iVar7 != 0);
            }
            __builtin_memcpy(DAT_00b6bb9a, &local_c, 4);
            if ((DAT_00b69e30 == 2) || (DAT_00bb73a4 != '\0')) {
                __builtin_memcpy(DAT_00b6bb9e, &DAT_00aa5e68, 4);
            }
            else {
                __builtin_memcpy(DAT_00b6bb9e, &local_14, 4);
                if (0 < DAT_00aa5e68) {
                    puVar5 = DAT_00b6bad2;
                    piVar13 = DAT_00aa5cb0;
                    piVar14 = DAT_00aa5e10;
                    local_8 = DAT_00aa5e68;
                    do {
                        iVar12 = 0;
                        iVar7 = *piVar14;
                        piVar6 = piVar13;
                        if (0 < iVar7) {
                            do {
                                if (*piVar6 != 0) break;
                                iVar12 = iVar12 + 1;
                                piVar6 = piVar6 + 1;
                            } while (iVar12 < iVar7);
                            if (iVar12 < iVar7) {
                                __builtin_memcpy(&local_40, DAT_00b6bb9e, 4);
                                local_40 = (int)((unsigned int)local_40 + 1u);
                                __builtin_memcpy(DAT_00b6bb9e, &local_40, 4);
                                __builtin_memcpy(puVar5, &local_14, 4);
                            }
                        }
                        puVar5 = puVar5 + 20;
                        piVar13 = piVar13 + 8;
                        piVar14 = piVar14 + 1;
                        local_8 = local_8 + -1;
                    } while (local_8 != 0);
                }
            }
        }
        __builtin_memcpy(&local_24, DAT_00b6a5aa, 4);
        if (local_24 != 1) goto LAB_004fb0bc;
    }
    else {
LAB_004fb0bc:
        __builtin_memcpy(&local_24, DAT_00b6a5aa, 4);
        if (local_24 != 2) {
            __builtin_memcpy(&local_44, DAT_00b6d5a6 + (unsigned int)DAT_00b6a24c * 88u, 4);
            uVar9 = (unsigned int)(unsigned char)DAT_00a91d17[(int)((unsigned int)local_44 * 0x562u)];
            __builtin_memcpy(DAT_00b6d962, &uVar9, 4);
            goto LAB_004fb188;
        }
    }
    __builtin_memcpy(&uVar9, DAT_00b6d5ca + (unsigned int)DAT_00b6a24c * 88u, 4);
    if (uVar9 == 0) {
        __builtin_memcpy(&iVar7, DAT_00b6d5ce + (unsigned int)DAT_00b6a24c * 88u, 4);
        if ((iVar7 == -1) || (9 < iVar7)) {
            __builtin_memcpy(&local_44, DAT_00b6d5a6 + (unsigned int)DAT_00b6a24c * 88u, 4);
            uVar9 = (unsigned int)(unsigned char)DAT_00a91d12[(int)((unsigned int)local_44 * 0x562u)];
        }
        else {
            __builtin_memcpy(&local_44, DAT_00b6d5a6 + (unsigned int)DAT_00b6a24c * 88u, 4);
            uVar9 = (unsigned int)(unsigned char)
                    DAT_00a92203[(int)((unsigned int)iVar7 + (unsigned int)local_44 * 0x562u)];
        }
    }
    if (((int)uVar9 < 1) || ((4 < (int)uVar9 && (uVar9 != 0xe)))) {
        __builtin_memcpy(DAT_00b6d962, &local_1c, 4);
        __builtin_memcpy(DAT_00b6d9aa, &local_14, 4);
    }
    else {
        __builtin_memcpy(DAT_00b6d962, &local_14, 4);
        __builtin_memcpy(DAT_00b7e38a, &local_14, 4);
    }
LAB_004fb188:
    if (DAT_00b69e30 == 2) {
        __builtin_memcpy(&local_4c, DAT_00b6d962, 4);
        __builtin_memcpy(DAT_00b6d96a + (unsigned int)local_4c * 0x109e0u, DAT_00b6a5a6, 4);
        __builtin_memcpy(&local_4c, DAT_00b6d962, 4);
        __builtin_memcpy(DAT_00b6d96e + (unsigned int)local_4c * 0x109e0u, DAT_00b6a5aa, 4);
        __builtin_memcpy(&local_24, DAT_00b6a5aa, 4);
        if (local_24 == 0) {
            __builtin_memcpy(&local_4c, DAT_00b6d962, 4);
            __builtin_memcpy(DAT_00b6d972 + (unsigned int)local_4c * 0x109e0u, DAT_00b6a5ae, 4);
        }
        else if ((local_24 == 1) || (local_24 == 2)) {
            __builtin_memcpy(&local_4c, DAT_00b6d962, 4);
            __builtin_memcpy(DAT_00b6d976 + (unsigned int)local_4c * 0x109e0u, DAT_00b6a5b2, 4);
            __builtin_memcpy(&local_4c, DAT_00b6d962, 4);
            __builtin_memcpy(DAT_00b6d97a + (unsigned int)local_4c * 0x109e0u, DAT_00b6a5b6, 4);
        }
        else {
            __builtin_memcpy(&local_4c, DAT_00b6d962, 4);
            __builtin_memcpy(DAT_00b6d97e + (unsigned int)local_4c * 0x109e0u, DAT_00b6a5ba, 4);
            __builtin_memcpy(&local_4c, DAT_00b6d962, 4);
            __builtin_memcpy(DAT_00b6d982 + (unsigned int)local_4c * 0x109e0u, DAT_00b6a5be, 4);
        }
        __builtin_memcpy(&local_4c, DAT_00b6d962, 4);
        __builtin_memcpy(DAT_00b6d9aa + (unsigned int)local_4c * 0x109e0u, DAT_00b6a606, 4);
        __builtin_memcpy(DAT_00b8ed6a, &local_14, 4);
        __builtin_memcpy(&local_4c, DAT_00b6d962, 4);
        __builtin_memcpy(DAT_00b6d9ae + (unsigned int)local_4c * 0x109e0u, DAT_00b6a60a, 4);
    }
    else {
        __builtin_memcpy(DAT_00b8ed2a, DAT_00b6a5a6, 4);
        __builtin_memcpy(DAT_00b8ed2e, DAT_00b6a5aa, 4);
        puVar5 = DAT_00b6a5ae;
        puVar10 = DAT_00b8ed32;
        for (iVar7 = 6; iVar7 != 0; iVar7 = iVar7 + -1) {
            __builtin_memcpy(puVar10, puVar5, 4);
            puVar5 = puVar5 + 4;
            puVar10 = puVar10 + 4;
        }
        __builtin_memcpy(DAT_00b8ed6a, DAT_00b6a606, 4);
        __builtin_memcpy(DAT_00b8ed6e, DAT_00b6a60a, 4);
        __builtin_memcpy(DAT_00b6d9aa, &local_14, 4);
        __builtin_memcpy(DAT_00b7e38a, &local_14, 4);
    }
    FUN_004d1ca0();
    return;
}
