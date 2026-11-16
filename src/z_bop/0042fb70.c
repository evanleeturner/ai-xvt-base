/* SPDX-License-Identifier: MIT */
/* z_bop 0042fb70 */
extern unsigned char DAT_00bb7374;
extern int DAT_009ecc34;
extern unsigned char DAT_009e9670[];
extern unsigned int DAT_009a1fe0;
extern int DAT_009a1fe8;
extern unsigned int DAT_00a07cec;
extern unsigned char DAT_00521038[];
extern unsigned char DAT_00521039;
extern unsigned char DAT_0052103a;
extern unsigned char DAT_00521049;
extern unsigned char DAT_0052104d;
extern unsigned char DAT_00521050[];
extern unsigned char bRam00521051;
extern unsigned char bRam00521067;
extern unsigned char DAT_00a0aba0[];

unsigned short FUN_00425d10(void);
int FUN_00430610(unsigned int a, char b, char c, char d, short e);
unsigned short FUN_004305d0(unsigned short a);
unsigned char FUN_00430600(void);
unsigned int FUN_00430540(int a, unsigned int b);

unsigned int FUN_0042fb70(int param_1, unsigned int param_2, int param_3, int param_4,
                          unsigned int param_5, unsigned short param_6)
{
    unsigned char bVar1;
    unsigned short uVar2;
    unsigned int uVar3;
    unsigned int uVar4;
    int iVar5;
    unsigned int *puVar7;
    int iVar8;
    unsigned char *pcVar9;
    short sVar10;
    unsigned int uVar11;
    char cVar12;
    unsigned int uStack_24;
    unsigned int uStack_1c;
    unsigned int auStack_18[6];
    unsigned char *pp;
    unsigned char *q;

    if (DAT_00bb7374 == '\0') {
        return 0;
    }
    if (param_1 == -1) {
        return 0;
    }
    if (param_1 != DAT_009ecc34) {
        return 0;
    }
    __builtin_memcpy(&iVar5, DAT_009e9670 + (unsigned int)param_1 * 0x5bdU, 4);
    if (iVar5 == -1) {
        return 0;
    }
    if (param_6 != 0xffff) {
        if (DAT_00bb7374 == '\x01') {
            param_6 = param_6 >> 1;
        }
        uVar2 = FUN_00425d10();
        if (param_6 <= uVar2) {
            return 0;
        }
    }
    if (param_2 == 0xffffffff) {
        uStack_24 = 0;
        if (DAT_00a07cec < DAT_009a1fe0) {
            puVar7 = auStack_18;
            pcVar9 = (unsigned char *)(DAT_00a07cec * 0x23U + 3U + (unsigned int)DAT_009a1fe8);
            uVar11 = DAT_00a07cec;
            do {
                unsigned char *pt;
                int pi;
                __builtin_memcpy(&pt, pcVar9 + 0x1c, 4);
                __builtin_memcpy(&pi, pcVar9 + 0x18, 4);
                if ((((*pcVar9 != 0) && (*pt == 0)) &&
                     (pi != param_1)) &&
                    (*(char *)((unsigned int)iVar5 * 0x23U + 0x16U + (unsigned int)DAT_009a1fe8) == (char)pcVar9[0x13])) {
                    *puVar7 = uVar11;
                    puVar7 = puVar7 + 1;
                    uStack_24 = uStack_24 + 1;
                }
                pcVar9 = pcVar9 + 0x23;
                uVar11 = uVar11 + 1;
            } while (uVar11 < DAT_009a1fe0);
        }
        if (uStack_24 == 0) {
            if (param_3 == 0xc) {
                FUN_00430610(0x25, '\0', '\0', '\0', -1);
                FUN_00430610(0x25, '\0', '\0', '\0', -1);
            }
            return 0;
        }
        uVar2 = FUN_004305d0((unsigned short)uStack_24);
        uVar11 = (unsigned int)uVar2;
        param_2 = auStack_18[uVar11];
        if (param_5 == param_2) {
            uVar4 = uVar11 + 1;
            if (uStack_24 <= uVar4) {
                uVar4 = 0;
            }
            if (uVar4 == uVar11) {
                return 0;
            }
        }
    }
    else {
        if ((int)DAT_009a1fe0 <= (int)param_2) {
            return 0;
        }
        if ((param_3 != 1) &&
            (*(char *)((unsigned int)iVar5 * 0x23U + 0x16U + (unsigned int)DAT_009a1fe8) !=
             *(char *)(param_2 * 0x23U + 0x16U + (unsigned int)DAT_009a1fe8))) {
            return 0;
        }
    }
    __builtin_memcpy(&pp, (unsigned char *)(unsigned int)DAT_009a1fe8 + param_2 * 0x23U + 0x1f, 4);
    __builtin_memcpy(&puVar7, pp + 0xa9, 4);
    uStack_1c = *puVar7;
    if (6 < uStack_1c) {
        uStack_1c = 0;
    }
    auStack_18[0] = (unsigned int)(unsigned char)puVar7[0x2c] * 0x61U;
    uVar11 = 0;
    iVar5 = (int)(auStack_18[0] + 0x72U);
    if ((param_5 == 0xffffffff) || ((int)DAT_009a1fe0 <= (int)param_5)) {
        sVar10 = -1;
    }
    else {
        pcVar9 = (unsigned char *)(param_5 * 0x23U + (unsigned int)DAT_009a1fe8);
        __builtin_memcpy(&sVar10, pcVar9, 2);
        __builtin_memcpy(&pp, pcVar9 + 0x1f, 4);
        __builtin_memcpy(&q, pp + 0xa9, 4);
        uVar11 = (unsigned int)*(unsigned char *)(q + 0xb0);
    }
    uVar4 = (unsigned int)DAT_00521038[param_3];
    cVar12 = (char)param_3;
    if (param_4 == -1) {
        switch (param_3) {
        case 3:
        case 5:
            bVar1 = FUN_00430600();
            if (bVar1 == 0) {
                return 0;
            }
            uVar3 = FUN_00430540(param_3, uVar11);
            if (uVar3 == 0xffffffff) {
                return 0;
            }
            if (uStack_1c != 0) {
                iVar8 = (int)(uVar11 * 0x61U + uVar3);
                DAT_00a0aba0[uVar4 + (unsigned int)iVar8] =
                    (unsigned char)(DAT_00a0aba0[uVar4 + (unsigned int)iVar8] + '\x01');
                FUN_00430610(DAT_00521038[2] + uStack_1c + (unsigned int)iVar5 - 1U, '\x01', '\x02', '\0', sVar10);
            }
            FUN_00430610(uVar3 + (unsigned int)iVar5 + uVar4, '\x01', cVar12, '\x01', sVar10);
            return 1;
        case 6:
            bVar1 = FUN_00430600();
            if (bVar1 == 0) {
                return 0;
            }
            uVar3 = FUN_00430540(param_3, uVar11);
            if (uVar3 == 0xffffffff) {
                return 0;
            }
            iVar8 = (int)(uVar11 * 0x61U + uVar3);
            DAT_00a0aba0[uVar4 + (unsigned int)iVar8] =
                (unsigned char)(DAT_00a0aba0[uVar4 + (unsigned int)iVar8] + '\x01');
            FUN_00430610(uVar3 + (unsigned int)iVar5 + uVar4, '\x01', cVar12, '\0', sVar10);
            return 1;
        case 9:
        case 10:
        case 0x15:
            uVar3 = FUN_00430540(param_3, uVar11);
            if (uVar3 == 0xffffffff) {
                return 0;
            }
            iVar8 = (int)(uVar11 * 0x61U + uVar3);
            DAT_00a0aba0[uVar4 + (unsigned int)iVar8] =
                (unsigned char)(DAT_00a0aba0[uVar4 + (unsigned int)iVar8] + '\x01');
            FUN_00430610(uVar3 + (unsigned int)iVar5 + uVar4, '\x01', cVar12, '\0', sVar10);
            return 1;
        case 0xc:
            uVar2 = FUN_004305d0(4);
            FUN_00430610((unsigned int)uVar2 + (unsigned int)iVar5 + uVar4, '\x01', cVar12, '\0', sVar10);
            return 1;
        case 0x10:
            uVar3 = FUN_00430540(param_3, uVar11);
            if (uVar3 == 0xffffffff) {
                return 0;
            }
            pcVar9 = DAT_00a0aba0 + uVar4 + uVar11 * 0x61U + uVar3;
            *pcVar9 = (unsigned char)(*pcVar9 + '\x01');
            FUN_00430610(uVar3 + (unsigned int)iVar5 + uVar4, '\x01', cVar12, '\0', sVar10);
            FUN_00430610(DAT_00521049 + uVar11 + (unsigned int)iVar5, '\x01', '\x11', '\x01', sVar10);
            return 1;
        }
    }
    else {
        if (param_3 == 1) {
            if (uStack_1c != 0) {
                FUN_00430610(DAT_00521038[0] + uStack_1c + (unsigned int)iVar5 - 1U, '\x01', '\0', '\0', sVar10);
            }
            uVar2 = FUN_00425d10();
            if (uVar2 < 0x5555) {
                FUN_00430610((unsigned int)DAT_00521039 + (unsigned int)iVar5, '\x01', '\x01', '\x01', sVar10);
            }
            if (param_4 < (int)(unsigned int)bRam00521051) {
                FUN_00430610((unsigned int)param_4 + (unsigned int)iVar5 + uVar4, '\x01', '\x01', '\x02', sVar10);
                return 1;
            }
            return 0;
        }
        if (param_3 != 0x17) {
            if (param_4 < (int)(unsigned int)DAT_00521050[param_3]) {
                FUN_00430610((unsigned int)param_4 + (unsigned int)iVar5 + uVar4, '\x01', cVar12, '\0', sVar10);
                return 1;
            }
            return 0;
        }
        uVar2 = FUN_00425d10();
        if (uVar2 < 0x8000) {
            uVar2 = FUN_004305d0((unsigned short)bRam00521067);
            FUN_00430610((unsigned int)DAT_0052104d + (unsigned int)iVar5 + (unsigned int)uVar2, '\x01', '\x15', '\0', sVar10);
        }
        if ((int)(unsigned int)bRam00521067 <= param_4) {
            return 0;
        }
        FUN_00430610((unsigned int)iVar5 + (unsigned int)param_4 + uVar4, '\x01', '\x17', '\x01', sVar10);
        pcVar9 = DAT_00a0aba0 + uVar4 + (unsigned int)param_4 + auStack_18[0];
        *pcVar9 = (unsigned char)(*pcVar9 + '\x01');
    }
    return 1;
}
