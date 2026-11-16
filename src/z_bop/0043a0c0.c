/* SPDX-License-Identifier: MIT */
/* z_bop 0043a0c0 */
extern unsigned char *DAT_009a1fe8;
extern int DAT_009ecc34;
extern unsigned char DAT_009e9670[];
extern unsigned char DAT_009e968a[];
extern int DAT_009a1ff8;
extern unsigned char _DAT_009ed22e[4];
extern short DAT_00a08368;
extern unsigned short DAT_00a08374;
extern short DAT_00a08c7c;
extern short DAT_00a08c78;
extern unsigned short DAT_00a08c7e;
extern unsigned short DAT_00a08a02;
extern unsigned char *DAT_00a08340;
extern unsigned char *DAT_00a08a04;
extern short DAT_00a08cac;
extern short DAT_00a08cae;
extern short DAT_00a08ca0;
extern short DAT_00a08ca2;
extern short DAT_00a0a1d2;
extern short DAT_00a0a1d4;

void __cdecl FUN_00427bd0(short a, short b, int c);
void __cdecl FUN_00427d30(short a, int b, int c);
void __cdecl FUN_00403540(unsigned int a, unsigned int b);
void __cdecl FUN_00425e30(int a, int b, int c);

void __cdecl FUN_0043a0c0(unsigned short param_1)
{
    unsigned char *pcVar2;
    unsigned char *rec;
    unsigned char *pEnt;
    unsigned char *pSelf;
    unsigned int iVar10, iVar13;
    int iVar14, iVar15, iVar12;
    unsigned short uVar9;
    int bVar16;
    unsigned int selfIdx;
    short s93, s95, s97, s99, s9b, s9d, s9f, sa1, sa3;
    long long lVar3, lVar4, lVar5, lVar6, lVar7, lVar8;
    int v;
    unsigned int v32;

    iVar10 = (unsigned int)param_1 * 0x23;
    pSelf = DAT_009a1fe8 + iVar10;
    __builtin_memcpy(&selfIdx, DAT_009e9670 + (unsigned int)DAT_009ecc34 * 0x5bd, 4);
    pEnt = DAT_009a1fe8 + selfIdx * 0x23;

    iVar14 = (int)(*(unsigned int *)(pSelf + 4) - *(unsigned int *)(pEnt + 4));
    iVar15 = (int)(*(unsigned int *)(pSelf + 8) - *(unsigned int *)(pEnt + 8));
    iVar12 = (int)(*(unsigned int *)(pSelf + 0xc) - *(unsigned int *)(pEnt + 0xc));

    __builtin_memcpy(&rec, pEnt + 0x1f, 4);
    if (rec[0x92] != 0) {
        FUN_00427bd0(*(short *)(pEnt + 0x12), *(short *)(pEnt + 0x10), (int)pEnt);
        FUN_00427d30(*(short *)(DAT_009a1fe8 + selfIdx * 0x23 + 0x14), 0, (int)(DAT_009a1fe8 + selfIdx * 0x23));
    }

    __builtin_memcpy(&rec, DAT_009a1fe8 + 0x1f + selfIdx * 0x23, 4);
    __builtin_memcpy(&s93, rec + 0x93, 2);
    __builtin_memcpy(&s95, rec + 0x95, 2);
    __builtin_memcpy(&s97, rec + 0x97, 2);
    lVar3 = (long long)iVar14 * (long long)(int)s93;
    lVar4 = (long long)iVar15 * (long long)(int)s95;
    lVar5 = (long long)iVar12 * (long long)(int)s97;
    iVar13 = (unsigned int)(lVar4 >> 15) + (unsigned int)(lVar3 >> 15) + (unsigned int)(lVar5 >> 15);

    __builtin_memcpy(&s99, rec + 0x99, 2);
    __builtin_memcpy(&s9b, rec + 0x9b, 2);
    __builtin_memcpy(&s9d, rec + 0x9d, 2);
    __builtin_memcpy(&s9f, rec + 0x9f, 2);
    __builtin_memcpy(&sa1, rec + 0xa1, 2);
    __builtin_memcpy(&sa3, rec + 0xa3, 2);
    lVar3 = (long long)iVar14 * (long long)(int)s99;
    lVar4 = (long long)iVar15 * (long long)(int)s9b;
    lVar5 = (long long)iVar12 * (long long)(int)s9d;
    lVar6 = (long long)iVar14 * (long long)(int)s9f;
    lVar7 = (long long)iVar15 * (long long)(int)sa1;
    lVar8 = (long long)iVar12 * (long long)(int)sa3;

    bVar16 = (int)iVar13 < 0;
    if (bVar16) {
        iVar13 = 0u - iVar13;
    }

    pSelf = DAT_009a1fe8 + iVar10;
    __builtin_memcpy(&pcVar2, pSelf + 0x1f, 4);
    if (pcVar2 == (unsigned char *)0) {
        DAT_00a08368 = 0x2f;
    } else if (pSelf[2] == '\t') {
        DAT_00a08368 = 0x2f;
    } else if (*pcVar2 == 1) {
        __builtin_memcpy(&v, _DAT_009ed22e, 4);
        if ((((int)((unsigned int)v + ((unsigned int)(v >> 31) & 3U)) >> 2) & 1U) == 0) {
            DAT_00a08368 = 0x37;
        } else {
            DAT_00a08368 = 0x3b;
        }
    } else {
        switch ((signed char)pcVar2[0x88]) {
        case 0:
            DAT_00a08368 = 0x3f;
            break;
        case 1:
        case 4:
            DAT_00a08368 = 0x37;
            break;
        case 2:
            DAT_00a08368 = 0x33;
            break;
        case 3:
            DAT_00a08368 = 0x3b;
            break;
        case 5:
            DAT_00a08368 = 0xd3;
        }
    }

    __builtin_memcpy(&v32, DAT_009e9670 + (unsigned int)DAT_009ecc34 * 0x5bd, 4);
    FUN_00403540(v32, (unsigned int)param_1);

    if (DAT_009a1ff8 < 0x1dd37) {
        if (0xee9b < DAT_009a1ff8) {
            if (DAT_00a08368 == 0x2f) {
                DAT_00a08368 = 0x2e;
            } else if (DAT_00a08368 == 0xd3) {
                DAT_00a08368 = 0xd4;
            } else {
                DAT_00a08368 = DAT_00a08368 + -1;
            }
        }
    } else if (DAT_00a08368 == 0x2f) {
        DAT_00a08368 = 0x2d;
    } else if (DAT_00a08368 == 0xd3) {
        DAT_00a08368 = 0xd5;
    } else {
        DAT_00a08368 = DAT_00a08368 + -2;
    }

    FUN_00425e30((int)((unsigned int)(lVar3 >> 15) + (unsigned int)(lVar4 >> 15) + (unsigned int)(lVar5 >> 15)),
                 (int)(0u - ((unsigned int)(lVar7 >> 15) + (unsigned int)(lVar6 >> 15) + (unsigned int)(lVar8 >> 15))),
                 (int)iVar13);

    if (bVar16) {
        DAT_00a08c7c = DAT_00a08c7c + *(short *)((char *)&DAT_00a08cac + (unsigned int)DAT_00a08374 * 0xc);
        DAT_00a08c78 = DAT_00a08c78 + *(short *)((char *)&DAT_00a08cae + (unsigned int)DAT_00a08374 * 0xc);
        if (DAT_00a08c78 < 0) {
            DAT_00a08c78 = 0;
        }
        *(short *)(DAT_00a08340 + (unsigned int)DAT_00a08c7e * 6) = DAT_00a08c7c;
        *(short *)(DAT_00a08340 + 2 + (unsigned int)DAT_00a08c7e * 6) = DAT_00a08c78;
        *(short *)(DAT_00a08340 + 4 + (unsigned int)DAT_00a08c7e * 6) = DAT_00a08368;
        uVar9 = DAT_00a08c7e;
        DAT_00a08c7e = DAT_00a08c7e + 1;
        if (DAT_00a08c7e == 0x30) {
            DAT_00a08c7e = uVar9;
        }
    } else {
        DAT_00a08c7c = DAT_00a08c7c + *(short *)((char *)&DAT_00a08ca0 + (unsigned int)DAT_00a08374 * 0xc);
        DAT_00a08c78 = DAT_00a08c78 + *(short *)((char *)&DAT_00a08ca2 + (unsigned int)DAT_00a08374 * 0xc);
        if (DAT_00a08c78 < 0) {
            DAT_00a08c78 = 0;
        }
        *(short *)(DAT_00a08a04 + (unsigned int)DAT_00a08a02 * 6) = DAT_00a08c7c;
        *(short *)(DAT_00a08a04 + 2 + (unsigned int)DAT_00a08a02 * 6) = DAT_00a08c78;
        *(short *)(DAT_00a08a04 + 4 + (unsigned int)DAT_00a08a02 * 6) = DAT_00a08368;
        uVar9 = DAT_00a08a02;
        DAT_00a08a02 = DAT_00a08a02 + 1;
        if (DAT_00a08a02 == 0x30) {
            DAT_00a08a02 = uVar9;
        }
    }

    {
        unsigned short us;
        __builtin_memcpy(&us, DAT_009e968a + (unsigned int)DAT_009ecc34 * 0x5bd, 2);
        if (us == param_1) {
            DAT_00a0a1d2 = DAT_00a08c7c;
            DAT_00a0a1d4 = DAT_00a08c78;
        }
    }
}
