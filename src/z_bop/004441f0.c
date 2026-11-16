/* SPDX-License-Identifier: MIT */
/* z_bop 004441f0 */
extern unsigned char *DAT_00556928;
extern unsigned int DAT_0055692c;
extern unsigned int DAT_00556930;
extern unsigned char DAT_00556959;   /* packed 4-byte pointer-sized value at odd address */
extern unsigned char DAT_0055695d;   /* packed 4-byte uint at odd address */
extern double _DAT_00518148;

void FUN_00444e60(unsigned char *param_1);
unsigned char *FUN_00444d90(signed char param_1, char param_2, unsigned char *param_3,
                            char param_4, char param_5, char param_6);
void FUN_004438b0(const char *param_1, const char *param_2);
void FUN_00443890(const char *param_1, unsigned int param_2, unsigned int param_3);
void __cdecl _exit(int);
void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);

void FUN_004441f0(int param_1)
{
    unsigned char *pbParam = (unsigned char *)param_1;
    unsigned char *pbVar12;
    unsigned char *pbVar13;
    unsigned char *puVar7;
    unsigned char bVar4;
    unsigned char bVar9;
    char cVar5;
    char cVar8;
    char cVar11;
    unsigned int uVar2;
    unsigned int uVar6;
    unsigned int uVar10;
    unsigned int local_18;
    unsigned int local_10;
    unsigned int uTmp;
    unsigned int uTmp2;
    unsigned int uTmp3;
    unsigned int uCount;
    unsigned int uTable;
    unsigned int uSize;
    unsigned int uMod;
    unsigned int uAddr;
    unsigned int uPix;
    double dVar3;
    double dAcc;
    double dSum;
    double dT0;
    double dT1;
    double dT2;
    unsigned int uShift;

    dVar3 = (double)*(unsigned int *)(pbParam + 0x1030);
    uSize = *(unsigned int *)(pbParam + 0x1034);
    local_10 = 0;
    __builtin_memcpy(&dAcc, DAT_00556928 + 0xe, 8);
    dAcc = (double)uSize * dVar3 * _DAT_00518148 + dAcc;
    __builtin_memcpy(DAT_00556928 + 0xe, &dAcc, 8);

    __builtin_memcpy(&pbVar13, pbParam + 0x1086, 4);
    __builtin_memcpy(&uCount, pbParam + 0x108e, 4);
    if (uCount == 0) {
        return;
    }
    do {
        __builtin_memcpy(&uTmp, &DAT_0055695d, 4);
        if (0x41241 < uTmp) {
            FUN_00444e60(DAT_00556928);
            DAT_0055692c = DAT_0055692c - 1;
        }
        bVar9 = 7;
        local_18 = 1;
        uVar6 = (unsigned int)pbVar13[3] + 1;
        pbVar12 = DAT_00556928;
        if (DAT_0055692c != 0) {
            do {
                uShift = bVar9 & 0x1f;
                bVar4 = (unsigned char)(pbVar13[2] >> uShift);
                uVar2 = (((((unsigned int)pbVar13[0] >> uShift) & 1) * 2) |
                         (((unsigned int)pbVar13[1] >> uShift) & 1)) * 2;
                uVar10 = uVar2 | (bVar4 & 1);
                __builtin_memcpy(&uTmp, pbVar12 + uVar10 * 4 + 0x32, 4);
                if (uTmp == 0) {
                    pbVar12[2] = (unsigned char)(pbVar12[2] | (1 << (signed char)uVar10));
                    cVar5 = (char)((1U << ((8U - (unsigned int)(int)(char)local_18) & 0x1f)) >> 1);
                    cVar11 = cVar5;
                    if ((bVar4 & 1) == 0) {
                        cVar11 = (char)(-(int)cVar5);
                    }
                    cVar8 = cVar5;
                    if ((uVar2 & 2) == 0) {
                        cVar8 = (char)(-(int)cVar5);
                    }
                    if ((uVar2 & 4) == 0) {
                        cVar5 = (char)(-(int)cVar5);
                    }
                    puVar7 = FUN_00444d90((signed char)uVar10, (char)local_18, pbVar12,
                                          (char)(pbVar12[3] + cVar5),
                                          (char)(pbVar12[4] + cVar8),
                                          (char)(pbVar12[5] + cVar11));
                    __builtin_memcpy(pbVar12 + uVar10 * 4 + 0x32, &puVar7, 4);
                    if (puVar7 == (unsigned char *)0) {
                        FUN_004438b0("Unable to quantize image", "Memory allocation failed");
                        _exit(1);
                    }
                    if (local_18 == DAT_0055692c) {
                        DAT_00556930 = DAT_00556930 + 1;
                    }
                }
                __builtin_memcpy(&pbVar12, pbVar12 + uVar10 * 4 + 0x32, 4);
                bVar9 = (unsigned char)(bVar9 - 1);
                local_18 = local_18 + 1;

                __builtin_memcpy(&uTable, &DAT_00556959, 4);
                uAddr = uTable + ((unsigned int)pbVar13[2] - (unsigned int)pbVar12[5]) * 4U;
                __builtin_memcpy(&uTmp, (void *)uAddr, 4);
                dT2 = (double)uTmp;
                __builtin_memcpy(&uTable, &DAT_00556959, 4);
                uAddr = uTable + ((unsigned int)pbVar13[1] - (unsigned int)pbVar12[4]) * 4U;
                __builtin_memcpy(&uTmp2, (void *)uAddr, 4);
                dT1 = (double)uTmp2;
                __builtin_memcpy(&uTable, &DAT_00556959, 4);
                uAddr = uTable + ((unsigned int)pbVar13[0] - (unsigned int)pbVar12[3]) * 4U;
                __builtin_memcpy(&uTmp3, (void *)uAddr, 4);
                dT0 = (double)uTmp3;
                dSum = dT2 + dT1 + dT0;
                __builtin_memcpy(&dAcc, pbVar12 + 0xe, 8);
                dAcc = dSum * (double)uVar6 + dAcc;
                __builtin_memcpy(pbVar12 + 0xe, &dAcc, 8);
            } while (local_18 <= DAT_0055692c);
        }
        __builtin_memcpy(&uTmp, pbVar12 + 10, 4);
        uTmp = uTmp + uVar6;
        __builtin_memcpy(pbVar12 + 10, &uTmp, 4);

        uPix = (unsigned int)pbVar13[0] * uVar6;
        __builtin_memcpy(&dAcc, pbVar12 + 0x16, 8);
        dAcc = (double)uPix + dAcc;
        __builtin_memcpy(pbVar12 + 0x16, &dAcc, 8);

        uPix = (unsigned int)pbVar13[1] * uVar6;
        __builtin_memcpy(&dAcc, pbVar12 + 0x1e, 8);
        dAcc = (double)uPix + dAcc;
        __builtin_memcpy(pbVar12 + 0x1e, &dAcc, 8);

        uPix = (unsigned int)pbVar13[2] * uVar6;
        pbVar13 = pbVar13 + 6;
        __builtin_memcpy(&dAcc, pbVar12 + 0x26, 8);
        dAcc = (double)uPix + dAcc;
        __builtin_memcpy(pbVar12 + 0x26, &dAcc, 8);

        __builtin_memcpy(&uCount, pbParam + 0x108e, 4);
        uMod = *(unsigned int *)(pbParam + 0x1034);
        if ((uCount - local_10 == 1) ||
            (uMod == 0 ? (RaiseException(0xC0000094UL, 0, 0, NULL), 0) : (local_10 % uMod) == 0)) {
            __builtin_memcpy(&uCount, pbParam + 0x108e, 4);
            FUN_00443890("  Classifying image colors...  ", local_10, uCount);
        }
        local_10 = local_10 + 1;
        __builtin_memcpy(&uCount, pbParam + 0x108e, 4);
    } while (local_10 < uCount);
    return;
}
