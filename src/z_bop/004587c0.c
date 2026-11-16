/* SPDX-License-Identifier: MIT */
/* z_bop 004587c0 */
/* Globals (types inferred from the decompile; DAT_009d113c and DAT_009a73f0
   are treated as 16-bit because uVar6 is undefined2 -- unverified). */
extern unsigned short DAT_009a1834;
extern unsigned short DAT_009a20ca[];      /* stride 0x93 ushorts (0x126 bytes) */
extern unsigned char  DAT_009a20cc[];      /* ushort read at +idx*0x126 */
extern unsigned char  DAT_009d8c82[];      /* stride 0x566 bytes */
extern unsigned char  DAT_009d8c83[];
extern unsigned char  DAT_009d8c84[];
extern unsigned char  DAT_009d8c97[];
extern unsigned char  DAT_009d8c98[];
extern unsigned char  DAT_009d8c99[];
extern unsigned char  DAT_009d9096[];      /* short at +idx*0x566 */
extern unsigned char  DAT_009d90c2[];
extern unsigned char  DAT_009d90ee[];
extern unsigned char  DAT_00524170[];
extern unsigned char  DAT_00521f2e[];      /* stride 0x1a */
extern char           DAT_00521f1b[];      /* stride 0x1a */
extern int            DAT_0099f970;
extern int            DAT_0099f974;
extern int            DAT_0099f978;
extern unsigned short DAT_0099f97c;
extern unsigned short DAT_0099f97e;
extern unsigned short DAT_0099f980;
extern unsigned short DAT_009d113c;
extern unsigned short DAT_009a73f0;
extern int            DAT_00a07ce8;
extern int            DAT_009d1308;
extern int            DAT_009a1fe8;

/* Return type and parameter types of the two callees are unresolved. */
void __cdecl FUN_00458c80(unsigned short param_1, int param_2, unsigned char param_3);
unsigned int __cdecl FUN_00425cb0(void);

void __cdecl FUN_004587c0(unsigned short param_1)
{
    unsigned char bVar1;
    char cVar2;
    short sVar3;
    short sVar4;
    short sVar5;
    unsigned short uVar6;
    unsigned int uVar7;
    char *pcVar8;
    int iVar9;
    unsigned char bVar10;
    int iVar11;
    int iVar12;
    int iVar13;
    unsigned int local_24;
    int local_20;
    int local_1c;
    int local_18;
    int local_14;

    uVar6 = DAT_009d113c;
    if (DAT_009a20ca[(unsigned int)DAT_009a1834 * 0x93] != 0) {
        iVar12 = (int)((unsigned int)DAT_009a1834 * 0x566);
        bVar1 = DAT_00524170[DAT_009d8c82[iVar12]];
        if ((DAT_00521f2e[(unsigned int)bVar1 * 0x1a] & 0x80) != 0) {
            cVar2 = DAT_00521f1b[(unsigned int)bVar1 * 0x1a];
            if (cVar2 == '\b') {
                bVar10 = DAT_009d8c84[iVar12] & 3;
                local_18 = 0;
                local_1c = 0;
                local_20 = 0;
                local_14 = 0;
                if ((DAT_009d8c84[iVar12] & 3) == 0) {
                    local_18 = 0x40;
                    local_1c = 0x40;
                }
                else if (bVar10 == 1) {
                    local_1c = 0x40;
                    local_14 = 0x40;
                }
                else if (bVar10 == 2) {
                    local_18 = 0x40;
                    local_20 = 0x40;
                }
                iVar11 = (int)DAT_009d8c83[iVar12] - 1;
                sVar3 = *(short *)(DAT_009d9096 + iVar12);
                sVar4 = *(short *)(DAT_009d90c2 + iVar12);
                sVar5 = *(short *)(DAT_009d90ee + iVar12);
                DAT_0099f97c = (unsigned short)DAT_009d8c97[iVar12];
                uVar7 = 0;
                DAT_0099f97e = (unsigned short)DAT_009d8c98[iVar12];
                local_24 = 0;
                DAT_0099f980 = (unsigned short)DAT_009d8c99[iVar12];
                if (DAT_009d8c83[iVar12] != 0) {
                    do {
                        iVar12 = 0;
                        if (DAT_009d8c83[(unsigned int)DAT_009a1834 * 0x566] != 0) {
                            iVar9 = (int)((((unsigned int)(int)sVar5 -
                                            (((unsigned int)local_20 * (unsigned int)iVar11) >> 1)) -
                                           (((unsigned int)local_14 * (unsigned int)iVar11) >> 1)) +
                                          (unsigned int)local_20 * local_24);
                            iVar13 = (int)((unsigned int)(int)sVar3 -
                                           (((unsigned int)local_18 * (unsigned int)iVar11) >> 1));
                            do {
                                if (((param_1 == 0xffff) || (param_1 == uVar7)) &&
                                    (*(unsigned short *)(DAT_009a20cc + (unsigned int)DAT_009a1834 * 0x126) <
                                     DAT_009a20ca[(unsigned int)DAT_009a1834 * 0x93])) {
                                    DAT_0099f974 = (int)((unsigned int)local_1c * local_24 -
                                                         ((((unsigned int)local_1c * (unsigned int)iVar11) >> 1) +
                                                          (unsigned int)(int)sVar4));
                                    DAT_0099f970 = iVar13;
                                    DAT_0099f978 = iVar9;
                                    FUN_00458c80(DAT_009a1834, 8, bVar1);
                                }
                                uVar7 = uVar7 + 1;
                                iVar9 = (int)((unsigned int)iVar9 + (unsigned int)local_14);
                                iVar13 = (int)((unsigned int)iVar13 + (unsigned int)local_18);
                                iVar12 = iVar12 + 1;
                            } while (iVar12 < (int)(unsigned int)DAT_009d8c83[(unsigned int)DAT_009a1834 * 0x566]);
                        }
                        local_24 = local_24 + 1;
                    } while (local_24 < DAT_009d8c83[(unsigned int)DAT_009a1834 * 0x566]);
                    return;
                }
            }
            else {
                if (cVar2 == '\t') {
                    DAT_0099f970 = (int)*(short *)(DAT_009d9096 + iVar12);
                    DAT_0099f978 = (int)*(short *)(DAT_009d90ee + iVar12);
                    DAT_0099f974 = (int)(0u - (unsigned int)(int)*(short *)(DAT_009d90c2 + iVar12));
                    DAT_0099f97c = (unsigned short)DAT_009d8c97[iVar12];
                    DAT_0099f97e = (unsigned short)DAT_009d8c98[iVar12];
                    DAT_0099f980 = (unsigned short)DAT_009d8c99[iVar12];
                    FUN_00458c80(DAT_009a1834, 9, bVar1);
                    return;
                }
                if (cVar2 != '\n') {
                    return;
                }
                sVar3 = *(short *)(DAT_009d9096 + iVar12);
                local_24 = 0;
                sVar4 = *(short *)(DAT_009d90c2 + iVar12);
                sVar5 = *(short *)(DAT_009d90ee + iVar12);
                DAT_009d113c = DAT_009a73f0;
                for (; local_24 < DAT_009d8c83[(unsigned int)DAT_009a1834 * 0x566]; local_24 = local_24 + 1) {
                    do {
                        uVar7 = FUN_00425cb0();
                        iVar12 = (int)((((uVar7 & 0x1ff) - 0x100) + (unsigned int)(int)sVar3));
                        uVar7 = FUN_00425cb0();
                        iVar11 = (int)((((uVar7 & 0x1ff) - 0x100) - (unsigned int)(int)sVar4));
                        uVar7 = FUN_00425cb0();
                        iVar13 = (int)((((uVar7 & 0x1ff) - 0x100) + (unsigned int)(int)sVar5));
                        iVar9 = DAT_00a07ce8;
                        if (DAT_00a07ce8 < (int)((unsigned int)DAT_009d1308 + (unsigned int)DAT_00a07ce8)) {
                            pcVar8 = (char *)((unsigned int)DAT_00a07ce8 * 0x23u + 3u + (unsigned int)DAT_009a1fe8);
                            do {
                                int t1;
                                int t2;
                                int t3;
                                __builtin_memcpy(&t1, pcVar8 + 1, 4);
                                __builtin_memcpy(&t2, pcVar8 + 5, 4);
                                __builtin_memcpy(&t3, pcVar8 + 9, 4);
                                if (((*pcVar8 != '\0') && (t1 == iVar12)) &&
                                    ((t2 == iVar11 && (t3 == iVar13)))) break;
                                pcVar8 = pcVar8 + 0x23;
                                iVar9 = iVar9 + 1;
                            } while (iVar9 < (int)((unsigned int)DAT_009d1308 + (unsigned int)DAT_00a07ce8));
                        }
                    } while (iVar9 < DAT_009d1308);
                    DAT_0099f97c = 0;
                    DAT_0099f97e = 0;
                    DAT_0099f980 = 0;
                    DAT_0099f970 = iVar12;
                    DAT_0099f974 = iVar11;
                    DAT_0099f978 = iVar13;
                    uVar7 = FUN_00425cb0();
                    FUN_00458c80(DAT_009a1834, 10, (unsigned char)((char)((uVar7 & 0xffff) % 6) + 'd'));
                }
                DAT_009a73f0 = DAT_009d113c;
            }
        }
    }
    DAT_009d113c = uVar6;
    return;
}
