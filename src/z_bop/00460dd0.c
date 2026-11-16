/* SPDX-License-Identifier: MIT */
/* z_bop 00460dd0 */
/* Prototypes */
uint __cdecl FUN_0041f410(int param_1, int param_2, int param_3);
int __cdecl FUN_004a5490(ushort param_1, ushort param_2);
unsigned char __cdecl FUN_00433390(uint param_1, unsigned short param_2, ushort param_3);
void __cdecl FUN_00403540(uint param_1, uint param_2);
void __cdecl FUN_00458e20(uint param_1, int param_2);

/* Globals (types taken from the decompile) */
extern ushort DAT_009a73b0;
extern int DAT_009a1fe8;
extern int DAT_009a1fe0;
extern uint DAT_009a1ff8;
extern ushort DAT_00a07cec;
extern int DAT_00a07ce8;
extern int DAT_009d1308;
extern int DAT_00a08104;
extern int DAT_00a08294;
extern int DAT_00a8f750;
extern int DAT_009a1fd0;
extern int DAT_009a1fd4;
extern int DAT_009a1fd8;
extern int DAT_00a08140;
extern int DAT_00a08144;
extern int DAT_00a08148;
extern int DAT_00a07cd4;
extern int DAT_00a07cd8;
extern int DAT_00a07cdc;
extern int iRam00522498;
extern unsigned char DAT_0099f930[];
extern unsigned char DAT_00521f2e[];
extern unsigned char DAT_00521f1c[];
extern unsigned char DAT_009d8c88[];
extern unsigned char DAT_009fd458[];

ushort __cdecl
FUN_00460dd0(unsigned short param_1, ushort param_2, short param_3, unsigned short param_4,
             ushort param_5, int param_6)
{
    unsigned char bVar1;
    char cVar2;
    ushort uVar3;
    uint uVar4;
    int iVar5;
    int iVar6;
    uint uVar8;
    ushort *puVar9;
    ushort uVar10;
    ushort uVar11;
    int unaff_EDI;
    int bVar13;
    int bVar14;
    ushort local_a;
    uint local_4;
    int t0, t1, t2;
    short s79;
    int v;
    unsigned char *rec;

    /* original: unaff_EDI is an unspecified register value on entry (upper 16 bits unspecified) */
    unaff_EDI = 0;
    local_a = 0xffff;
    local_4 = 0x10000;
    bVar14 = *(char *)((unsigned char *)(uint)((uint)DAT_009a73b0 * 0x23 + 3 + (uint)DAT_009a1fe8)) == '6';
    if (bVar14) {
        local_4 = (uint)iRam00522498 + 0x10000;
    }
    uVar10 = DAT_00a07cec;
    if ((int)(uint)DAT_00a07cec < DAT_009a1fe0) {
        do {
            if (DAT_0099f930[(uint)uVar10 * 2 + (uint)param_6] != '\0') {
                unaff_EDI = (int)((uint)uVar10 * 0x23);
                iVar6 = (int)((uint)DAT_009a1fe8 + (uint)unaff_EDI);
                __builtin_memcpy(&t0, (unsigned char *)iVar6 + 4, 4);
                __builtin_memcpy(&t1, (unsigned char *)iVar6 + 8, 4);
                __builtin_memcpy(&t2, (unsigned char *)iVar6 + 0xc, 4);
                DAT_009a1ff8 = FUN_0041f410((int)((uint)t0 - (uint)DAT_00a08144),
                                            (int)((uint)t1 - (uint)DAT_00a08140),
                                            (int)((uint)t2 - (uint)DAT_00a08148));
                if (bVar14) {
                    iVar6 = 0;
                    if (*(char *)((unsigned char *)DAT_00a08104 + 0x21d) != '\0') {
                        puVar9 = (ushort *)((unsigned char *)DAT_00a08104 + 0x3b2);
                        do {
                            if (*puVar9 == uVar10) {
                                DAT_009a1ff8 = DAT_009a1ff8 + 0x8000;
                            }
                            puVar9 = puVar9 + 2;
                            iVar6 = (int)((uint)iVar6 + 1U);
                        } while (iVar6 < (int)(uint)*(unsigned char *)((unsigned char *)DAT_00a08104 + 0x21d));
                    }
                }
                if (DAT_009a1ff8 < local_4) {
                    bVar13 = 1;
                    if (!bVar14) {
                        iVar6 = (int)((uint)DAT_009a1fe8 + (uint)unaff_EDI);
                        DAT_00a8f750 = (int)((uint)DAT_00a8f750 + 1U);
                        __builtin_memcpy(&DAT_009a1fd4, (unsigned char *)iVar6 + 4, 4);
                        __builtin_memcpy(&DAT_009a1fd8, (unsigned char *)iVar6 + 8, 4);
                        __builtin_memcpy(&DAT_009a1fd0, (unsigned char *)iVar6 + 0xc, 4);
                        iVar6 = FUN_004a5490(DAT_009a73b0, DAT_009a73b0);
                        bVar13 = iVar6 == 0;
                    }
                    if (bVar13) {
                        local_4 = DAT_009a1ff8;
                        local_a = uVar10;
                    }
                }
            }
            uVar10 = (ushort)(uVar10 + 1);
        } while ((int)(uint)uVar10 < DAT_009a1fe0);
    }
    uVar4 = ((uint)unaff_EDI & 0xffff0000U) | (uint)(ushort)DAT_00a07ce8;
    if ((int)((uint)DAT_00a07ce8 & 0xffff) < (int)((uint)DAT_009d1308 + (uint)DAT_00a07ce8)) {
        do {
            uVar8 = uVar4 & 0xffff;
            bVar1 = *(unsigned char *)((unsigned char *)(uint)(uVar8 * 0x23 + 3 + (uint)DAT_009a1fe8));
            uVar10 = (ushort)uVar4;
            if ((bVar1 != 0) && ((DAT_00521f2e[(uint)bVar1 * 0x1a] & 2) != 0)) {
                bVar13 = FUN_00433390(uVar4, param_1, param_2);
                uVar11 = (ushort)(unsigned char)bVar13;
                bVar13 = FUN_00433390(uVar4, param_4, param_5);
                uVar3 = (ushort)(unsigned char)bVar13;
                if (param_3 == 1) {
                    uVar11 = (ushort)(uVar11 | uVar3);
                } else {
                    uVar11 = (ushort)(uVar11 & uVar3);
                }
                if (uVar11 != 0) {
                    if (uVar10 == 0xffff) {
                        iVar6 = 0;
                    } else {
                        rec = (unsigned char *)(uint)((uint)DAT_009a1fe8 + uVar8 * 0x23);
                        __builtin_memcpy(&t0, rec + 0x1f, 4);
                        if (t0 == 0) {
                            bVar13 = rec[3] == '\0';
LAB_00461158:
                            iVar6 = 1 - (int)bVar13;
                        } else if (rec[3] == '\0') {
                            iVar6 = 0;
                        } else {
                            __builtin_memcpy(&iVar6, (unsigned char *)t0 + 0xa9, 4);
                            if (iVar6 == 0) {
                                iVar6 = 1;
                            } else if ((*(char *)((unsigned char *)iVar6 + 0x5e) == '\x15') &&
                                       (*(char *)((unsigned char *)iVar6 + 0x5f) != '\0')) {
                                iVar6 = 0;
                            } else {
                                cVar2 = *(char *)((unsigned char *)iVar6 + 7);
                                if ((cVar2 == '\x01') || ((cVar2 == '\x03') || (cVar2 == '\x04'))) {
                                    iVar6 = 0;
                                } else if ((*(char *)((unsigned char *)iVar6 + 0x23d) == '\x03') &&
                                           (*(char *)((unsigned char *)iVar6 + 0x241) != '\0')) {
                                    FUN_00403540(uVar8, (uint)DAT_009a73b0);
                                    cVar2 = *(char *)((unsigned char *)(uint)((uint)DAT_009a73b0 * 0x23 + 2 + (uint)DAT_009a1fe8));
                                    if ((cVar2 == '\0') || ((cVar2 == '\x01') || (cVar2 == '\x02'))) {
                                        if ((int)DAT_009a1ff8 < 0x8001) goto LAB_0046111c;
                                        iVar6 = 0;
                                    } else {
                                        if ((int)DAT_009a1ff8 < 0x10001) goto LAB_0046111c;
                                        iVar6 = 0;
                                    }
                                } else {
LAB_0046111c:
                                    iVar5 = (int)((uint)DAT_009a1fe8 + uVar8 * 0x23);
                                    __builtin_memcpy(&t1, (unsigned char *)iVar5 + 0x1b, 4);
                                    if (t1 == -1) {
                                        uint a, b;
                                        __builtin_memcpy(&t2, (unsigned char *)iVar5 + 0x1f, 4);
                                        __builtin_memcpy(&t2, (unsigned char *)t2 + 0xa9, 4);
                                        __builtin_memcpy(&a, (unsigned char *)t2 + 0xcb, 4);
                                        __builtin_memcpy(&b, (unsigned char *)iVar6 + 0xc3, 4);
                                        bVar13 = a < b;
                                        goto LAB_00461158;
                                    }
                                    iVar6 = 1;
                                }
                            }
                        }
                    }
                    if (iVar6 != 0) {
                        FUN_00458e20(uVar8, 0);
                        DAT_009a1ff8 = FUN_0041f410((int)((uint)DAT_00a07cdc - (uint)DAT_00a08144),
                                                    (int)((uint)DAT_00a07cd8 - (uint)DAT_00a08140),
                                                    (int)((uint)DAT_00a07cd4 - (uint)DAT_00a08148));
                        if (bVar14) {
                            iVar6 = 0;
                            if (*(char *)((unsigned char *)DAT_00a08104 + 0x21d) != '\0') {
                                puVar9 = (ushort *)((unsigned char *)DAT_00a08104 + 0x3b2);
                                do {
                                    if (*puVar9 == uVar10) {
                                        DAT_009a1ff8 = DAT_009a1ff8 + 0x8000;
                                    }
                                    puVar9 = puVar9 + 2;
                                    iVar6 = (int)((uint)iVar6 + 1U);
                                } while (iVar6 < (int)(uint)*(unsigned char *)((unsigned char *)DAT_00a08104 + 0x21d));
                            }
                        }
                        if (DAT_009a1ff8 < local_4) {
                            bVar13 = 1;
                            if (!bVar14) {
                                DAT_00a8f750 = (int)((uint)DAT_00a8f750 + 1U);
                                DAT_009a1fd4 = DAT_00a07cdc;
                                DAT_009a1fd8 = DAT_00a07cd8;
                                DAT_009a1fd0 = DAT_00a07cd4;
                                iVar6 = FUN_004a5490(DAT_009a73b0, DAT_009a73b0);
                                bVar13 = iVar6 == 0;
                            }
                            if (bVar13) {
                                local_4 = DAT_009a1ff8;
                                local_a = uVar10;
                            }
                        }
                    }
                }
            }
            uVar4 = (uVar4 & 0xffff0000U) | (uint)(ushort)(uVar10 + 1);
        } while ((int)(uint)(ushort)(uVar10 + 1) < (int)((uint)DAT_009d1308 + (uint)DAT_00a07ce8));
    }
    iVar6 = DAT_00a08104;
    if ((!bVar14) && (0x10000 < local_4)) {
        local_a = 0xffff;
    }
    if (((local_a != 0xffff) && (DAT_00a08294 == 0xe)) &&
        (uVar10 = DAT_00a07cec, (int)(uint)DAT_00a07cec < DAT_009a1fe0)) {
        do {
            unsigned char *r7 = (unsigned char *)(uint)((uint)DAT_009a1fe8 + (uint)uVar10 * 0x23);
            __builtin_memcpy(&iVar5, (unsigned char *)(uint)((uint)DAT_009a73b0 * 0x23 + 0x1f + (uint)DAT_009a1fe8), 4);
            uVar4 = (uint)*(unsigned char *)((unsigned char *)iVar5 + 0x89);
            if (uVar4 == (unsigned char)DAT_009d8c88[(uint)*(unsigned char *)(r7 + 0x16) * 0x566]) {
                bVar14 = 0;
            } else {
                bVar14 = DAT_009fd458[(uint)(unsigned char)DAT_009d8c88[(uint)*(unsigned char *)(r7 + 0x16) * 0x566] + uVar4 * 0x1e5] == '\0';
            }
            __builtin_memcpy(&s79, (unsigned char *)iVar5 + 0x79, 2);
            if ((!bVar14) &&
                ((s79 == 0) ||
                 (((cVar2 = *(char *)(r7 + 2), cVar2 != '\0') && (cVar2 != '\x01')) && (cVar2 != '\x02')))) {
                __builtin_memcpy(&t0, r7 + 4, 4);
                __builtin_memcpy(&t1, r7 + 8, 4);
                __builtin_memcpy(&t2, r7 + 0xc, 4);
                iVar5 = (int)FUN_0041f410((int)((uint)t0 - (uint)DAT_00a08144),
                                          (int)((uint)t1 - (uint)DAT_00a08140),
                                          (int)((uint)t2 - (uint)DAT_00a08148));
                __builtin_memcpy(&v, DAT_00521f1c +
                                 (uint)*(unsigned char *)((unsigned char *)(uint)((uint)DAT_009a1fe8 + 3 + (uint)uVar10 * 0x23)) * 0x1a, 4);
                DAT_009a1ff8 = (uint)iVar5 - (uint)v;
                if (DAT_009a1ff8 <= local_4) {
                    unsigned char *r5 = (unsigned char *)(uint)((uint)local_a * 0x23 + (uint)DAT_009a1fe8);
                    __builtin_memcpy(&DAT_009a1fd4, r5 + 4, 4);
                    __builtin_memcpy(&DAT_009a1fd8, r5 + 8, 4);
                    __builtin_memcpy(&DAT_009a1fd0, r5 + 0xc, 4);
                    iVar5 = FUN_004a5490(uVar10, uVar10);
                    if (iVar5 != 0) {
                        DAT_00a08104 = iVar6;
                        return 0xffff;
                    }
                }
            }
            uVar10 = (ushort)(uVar10 + 1);
        } while ((int)(uint)uVar10 < DAT_009a1fe0);
    }
    DAT_00a08104 = iVar6;
    return local_a;
}
