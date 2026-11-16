/* SPDX-License-Identifier: MIT */
/* z_bop 0040bbf0 */
/* Prototypes */
void __cdecl FUN_004079e0(void);
int  __cdecl FUN_0046a760(unsigned int, unsigned short);
int  __cdecl FUN_0046a650(unsigned int, unsigned short);
void __cdecl FUN_004096d0(int, int, int);
void __cdecl FUN_00409c10(int, int, int);
void __cdecl FUN_0040a1a0(int, int, int);
void __cdecl FUN_0040a6e0(int, int, int);
void __cdecl FUN_00408140(void);
void __cdecl FUN_004b1c10(void);
void __cdecl FUN_004b1c90(void);
void __cdecl FUN_004b1d10(float *, int);
void __cdecl FUN_004b1d70(void);
void __cdecl FUN_004b1de0(unsigned int *, int);
void __cdecl FUN_004b2020(void);
void __cdecl FUN_004b1c50(void);
void __cdecl FUN_00408110(void);
unsigned char * __cdecl FUN_00407af0(int, int, unsigned short *, unsigned char *, int);

/* Globals (types inferred from the decompile; unverified) */
extern unsigned short DAT_009a1fe4;
extern unsigned short DAT_009ed664;
extern unsigned int   DAT_00a07ce8;
extern int            DAT_009a1fe8;
extern unsigned int   DAT_0051a558[];
extern unsigned int   DAT_009a8e34;
extern float          _DAT_00518060;
extern float          _DAT_00518080;
extern int            DAT_00a90b20;
extern int            DAT_00528c5c;
extern int            DAT_0054f978;
extern int            DAT_0054f99c;
extern unsigned int   DAT_0053f8f0[];   /* DAT_0053f8f0..fc, index buffer of 4+ entries */
extern int            DAT_0054f998;
extern int            DAT_0052f870[];   /* DAT_0052f86c is element [-1] */
extern int            DAT_0052f868;
extern int            DAT_0054f990;
extern int            DAT_0054f994;
extern int            DAT_0054f9a0;
extern float          DAT_0053f970[];
extern unsigned int   DAT_0054f988[];
extern float          _DAT_0054f9a4;
extern float          _DAT_0053f974;
extern int            DAT_0051a54c;
extern int            DAT_00527e94;

void __cdecl FUN_0040bbf0(unsigned short param_1, int param_2, int param_3,
                          unsigned int param_4, int param_5)
{
    float fVar1, fVar3, fVar4, fVar5;
    int iVar7, iVar8, iVar9, iVar10, iVar11, iVar12, iVar14, iVar15, iVar18, iVar19;
    unsigned int uVar16, uVar20, uVar21;
    unsigned int local_3cc;
    float local_3e0;
    float local_3c0[24];
    unsigned char *puVar13;
    int *piVar17;
    unsigned int *puVar22;
    unsigned int *puVar23;
    char *pcVar;

    iVar14 = (int)((unsigned int)DAT_009a1fe4 - (unsigned int)param_3);
    if (DAT_009ed664 < DAT_00a07ce8) {
        iVar7 = (int)((unsigned int)DAT_009ed664 * 0x23u + (unsigned int)DAT_009a1fe8);
        pcVar = (char *)iVar7;
        if ((pcVar[2] == '\r') && ((unsigned char)pcVar[0x19] < 0x20)) {
            local_3cc = DAT_0051a558[(unsigned int)(unsigned char)pcVar[0x19]];
        } else {
            local_3cc = 0xffffffff;
        }
    }
    if (DAT_009a8e34 < 0x1000001) {
        local_3e0 = _DAT_00518060 / ((float)(int)DAT_009a8e34 * _DAT_00518080 + _DAT_00518060);
        if (DAT_00a90b20 == 2) {
            local_3e0 = _DAT_00518060 - local_3e0;
        }
    } else {
        local_3cc = 0xffffffff;
        local_3e0 = 0.00012205541f;
        if (DAT_00a90b20 == 2) {
            local_3e0 = 0.9998779f;
        }
    }
    iVar7  = *(int *)(param_5 + 0x10);
    iVar18 = *(int *)(param_5 + 0x14);
    if (0x100 < iVar7) {
        iVar7 = 0x100;
        FUN_004079e0();
    }
    if (0x100 < iVar18) {
        iVar18 = 0x100;
        FUN_004079e0();
    }
    iVar8 = 0;
    iVar15 = 1;
    do {
        iVar10 = iVar15;
        iVar15 = iVar10 * 2;
        if (iVar7 <= iVar15) break;
        iVar8 = iVar8 + 1;
    } while (iVar8 < 8);
    iVar8 = 1;
    iVar9 = 0;
    do {
        iVar8 = iVar8 * 2;
        if (iVar18 <= iVar8) break;
        iVar9 = iVar9 + 1;
    } while (iVar9 < 8);
    if (*(int *)(DAT_00528c5c + 0x1c) != 0) {
        /* both dimensions become max(iVar15, iVar8) (Ghidra flag logic collapsed) */
        if (iVar15 <= iVar8) {
            iVar15 = iVar8;
        } else {
            iVar8 = iVar15;
        }
    }
    iVar10 = *(int *)(param_5 + 0x14);
    uVar20 = (unsigned int)((int)((unsigned int)*(int *)(param_5 + 0x10) * (param_4 & 0xffff)) >> 9);
    iVar9  = FUN_0046a760(uVar20, param_1);
    uVar16 = (unsigned int)((int)((unsigned int)iVar10 * (param_4 & 0xffff)) >> 9);
    iVar10 = FUN_0046a650(uVar16, param_1);
    iVar11 = FUN_0046a760(uVar16, param_1);
    iVar12 = FUN_0046a650(uVar20, param_1);
    iVar19 = 0;
    DAT_0054f978 = 4;
    DAT_0054f99c = 4;
    DAT_0053f8f0[0] = 0;
    DAT_0053f8f0[1] = 1;
    DAT_0053f8f0[2] = 2;
    DAT_0053f8f0[3] = 3;
    local_3c0[0] = (float)(int)((unsigned int)param_2 + (unsigned int)iVar10 + (unsigned int)iVar9);
    local_3c0[1] = (float)(int)(((unsigned int)iVar11 - (unsigned int)iVar12) + (unsigned int)iVar14);
    local_3c0[3] = 0.0f;
    local_3c0[4] = 0.0f;
    local_3c0[5] = 0.0f;
    uVar21 = 0u - uVar20;
    local_3c0[2] = local_3e0;
    iVar10 = FUN_0046a760(uVar21, param_1);
    iVar9  = FUN_0046a650(uVar16, param_1);
    iVar11 = FUN_0046a760(uVar16, param_1);
    iVar12 = FUN_0046a650(uVar21, param_1);
    local_3c0[6] = (float)(int)((unsigned int)param_2 + (unsigned int)iVar9 + (unsigned int)iVar10);
    local_3c0[9] = 0.0f;
    local_3c0[7] = (float)(int)(((unsigned int)iVar11 - (unsigned int)iVar12) + (unsigned int)iVar14);
    local_3c0[11] = 0.0f;
    local_3c0[8] = local_3e0;
    local_3c0[10] = (float)iVar7 / (float)iVar15;
    iVar10 = FUN_0046a760(uVar21, param_1);
    uVar16 = 0u - uVar16;
    iVar9  = FUN_0046a650(uVar16, param_1);
    iVar11 = FUN_0046a760(uVar16, param_1);
    iVar12 = FUN_0046a650(uVar21, param_1);
    local_3c0[12] = (float)(int)((unsigned int)param_2 + (unsigned int)iVar9 + (unsigned int)iVar10);
    local_3c0[15] = 0.0f;
    local_3c0[13] = (float)(int)(((unsigned int)iVar11 - (unsigned int)iVar12) + (unsigned int)iVar14);
    local_3c0[14] = local_3e0;
    local_3c0[16] = (float)iVar7 / (float)iVar15;
    local_3c0[17] = (float)iVar18 / (float)iVar8;
    iVar7  = FUN_0046a760(uVar20, param_1);
    iVar10 = FUN_0046a650(uVar16, param_1);
    iVar9  = FUN_0046a760(uVar16, param_1);
    iVar11 = FUN_0046a650(uVar20, param_1);
    local_3c0[18] = (float)(int)((unsigned int)param_2 + (unsigned int)iVar10 + (unsigned int)iVar7);
    local_3c0[21] = 0.0f;
    local_3c0[22] = 0.0f;
    local_3c0[23] = (float)iVar18 / (float)iVar8;
    local_3c0[20] = local_3e0;
    DAT_0054f998 = 0;
    local_3c0[19] = (float)(int)(((unsigned int)iVar9 - (unsigned int)iVar11) + (unsigned int)iVar14);

    if (0 < DAT_0054f978) {
        piVar17 = (int *)&DAT_0053f8f0[0];
        iVar14 = (int)DAT_0053f8f0[DAT_0054f978 - 1];
        do {
            iVar7 = *piVar17;
            piVar17 = piVar17 + 1;
            iVar19 = iVar19 + 1;
            FUN_004096d0(iVar14, iVar7, (int)local_3c0);
            iVar14 = iVar7;
        } while (iVar19 < DAT_0054f978);
    }
    iVar14 = 0;
    DAT_0054f978 = 0;
    if (0 < DAT_0054f998) {
        piVar17 = &DAT_0052f870[0];
        iVar7 = DAT_0052f870[DAT_0054f998 - 1];
        do {
            iVar18 = *piVar17;
            piVar17 = piVar17 + 1;
            iVar14 = iVar14 + 1;
            FUN_00409c10(iVar7, iVar18, (int)local_3c0);
            iVar7 = iVar18;
        } while (iVar14 < DAT_0054f998);
    }
    iVar14 = 0;
    DAT_0054f998 = 0;
    if (0 < DAT_0054f978) {
        piVar17 = (int *)&DAT_0053f8f0[0];
        iVar7 = (int)DAT_0053f8f0[DAT_0054f978 - 1];
        do {
            iVar18 = *piVar17;
            piVar17 = piVar17 + 1;
            iVar14 = iVar14 + 1;
            FUN_0040a1a0(iVar7, iVar18, (int)local_3c0);
            iVar7 = iVar18;
        } while (iVar14 < DAT_0054f978);
    }
    iVar14 = 0;
    DAT_0054f978 = 0;
    if (0 < DAT_0054f998) {
        piVar17 = &DAT_0052f870[0];
        iVar7 = DAT_0052f870[DAT_0054f998 - 1];
        do {
            iVar18 = *piVar17;
            piVar17 = piVar17 + 1;
            iVar14 = iVar14 + 1;
            FUN_0040a6e0(iVar7, iVar18, (int)local_3c0);
            iVar7 = iVar18;
        } while (iVar14 < DAT_0054f998);
    }
    if (2 < DAT_0054f978) {
        if ((DAT_0054f990 < (int)((unsigned int)DAT_0054f978 + (unsigned int)DAT_0054f994)) ||
            (DAT_0052f868 < (int)((unsigned int)DAT_0054f9a0 + (unsigned int)DAT_0054f978))) {
            FUN_00408140();
            FUN_004b1c10();
            FUN_004b1c90();
            FUN_004b1d10(DAT_0053f970, DAT_0054f994);
            FUN_004b1d70();
            FUN_004b1de0(DAT_0054f988, DAT_0054f9a0);
            FUN_004b2020();
            FUN_004b1c50();
            FUN_00408110();
            DAT_0054f9a0 = 0;
            DAT_0054f994 = 0;
        }
        if (DAT_0051a54c != 0) {
            local_3cc = 0xfeffffff;
            DAT_0051a54c = 0;
        }
        iVar14 = 0;
        if (0 < DAT_0054f978) {
            puVar22 = &DAT_0053f8f0[0];
            do {
                uVar16 = *puVar22;
                iVar14 = iVar14 + 1;
                fVar1 = local_3c0[uVar16 * 6 + 1];
                fVar3 = local_3c0[uVar16 * 6 + 4];
                fVar4 = local_3c0[uVar16 * 6 + 2];
                fVar5 = local_3c0[uVar16 * 6 + 5];
                DAT_0053f970[DAT_0054f994 * 8]     = local_3c0[uVar16 * 6] + _DAT_0054f9a4;
                DAT_0053f970[DAT_0054f994 * 8 + 1] = fVar1 + _DAT_0053f974;
                DAT_0053f970[DAT_0054f994 * 8 + 2] = fVar4;
                DAT_0053f970[DAT_0054f994 * 8 + 3] = fVar4;
                DAT_0053f970[DAT_0054f994 * 8 + 6] = fVar3;
                DAT_0053f970[DAT_0054f994 * 8 + 7] = fVar5;
                __builtin_memcpy(&DAT_0053f970[DAT_0054f994 * 8 + 4], &local_3cc, 4);
                DAT_0053f970[DAT_0054f994 * 8 + 5] = 0.0f;
                *puVar22 = (unsigned int)DAT_0054f994;
                DAT_0054f994 = DAT_0054f994 + 1;
                puVar22 = puVar22 + 1;
            } while (iVar14 < DAT_0054f978);
        }
        iVar14 = 2;
        puVar13 = FUN_00407af0(iVar15, iVar8,
                               (unsigned short *)(*(int *)(param_5 + 0xc) + param_5),
                               (unsigned char *)(*(int *)(param_5 + 8) + 0x10 + param_5),
                               *(int *)(param_5 + 0x20));
        if (2 < DAT_0054f978) {
            puVar23 = &DAT_0053f8f0[1];
            do {
                DAT_0054f988[DAT_0054f9a0 * 5]     = DAT_0053f8f0[0];
                DAT_0054f988[DAT_0054f9a0 * 5 + 1] = *puVar23;
                DAT_0054f988[DAT_0054f9a0 * 5 + 2] = puVar23[1];
                DAT_0054f988[DAT_0054f9a0 * 5 + 4] = (unsigned int)puVar13;
                DAT_0054f988[DAT_0054f9a0 * 5 + 3] = 0x812;
                if (DAT_00527e94 != 0) {
                    DAT_0054f988[DAT_0054f9a0 * 5 + 3] = DAT_0054f988[DAT_0054f9a0 * 5 + 3] + 0x180;
                }
                iVar14 = iVar14 + 1;
                DAT_0054f988[DAT_0054f9a0 * 5 + 3] = DAT_0054f988[DAT_0054f9a0 * 5 + 3] + 0x200;
                DAT_0054f9a0 = DAT_0054f9a0 + 1;
                puVar23 = puVar23 + 1;
            } while (iVar14 < DAT_0054f978);
        }
    }
    return;
}
