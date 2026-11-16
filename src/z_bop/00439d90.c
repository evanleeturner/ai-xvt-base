/* SPDX-License-Identifier: MIT */
/* z_bop 00439d90 */
extern int DAT_009ecc34;
extern unsigned char DAT_009e9670[];
extern int DAT_009a1fe8;
extern int DAT_009a1fe0;
extern int _DAT_00a07cec;
extern int DAT_009e9640;
extern int DAT_00a00854;
extern int DAT_009d1308;
extern int DAT_00a07ce8;
extern unsigned char DAT_00521f2e[];

extern unsigned short DAT_00a083ba;
extern unsigned short DAT_00a08a02;
extern unsigned short DAT_00a08344;
extern unsigned short DAT_00a08c7e;
extern unsigned short DAT_00a08c88;
extern unsigned short DAT_00a08c86;
extern unsigned short DAT_00a0a1d2;
extern unsigned short DAT_00a0a1d4;
extern unsigned char DAT_00a08364;
extern unsigned char DAT_00a0837a;

extern unsigned char DAT_00a083c0[];
extern unsigned char DAT_00a084e0[];
extern unsigned char DAT_00a0a540[];
extern unsigned char DAT_00a0a660[];
extern void *DAT_00a08360;
extern void *DAT_00a08a04;
extern void *DAT_00a08bb0;
extern void *DAT_00a08340;

extern void (*DAT_009a1fec)(void);
extern void (*DAT_009d12e4)(void *, unsigned int);
extern void (*DAT_009a20a8)(void *, unsigned int);
extern void (*DAT_009ec46c)(void);

unsigned int FUN_00484f80(unsigned short a);
void FUN_0043a0c0(unsigned short a);

void FUN_00439d90(void)
{
    char cVar1;
    unsigned short uVar2;
    unsigned int uVar3;
    unsigned int uVar5;
    int iVar6;
    int iVar7;
    int iVar8;
    int bVar9;
    int iVar10;
    unsigned int uVar11;
    unsigned short uVar12;
    unsigned char *pbVar13;

    pbVar13 = DAT_009e9670 + (unsigned int)DAT_009ecc34 * 0x5bdu;

    __builtin_memcpy(&iVar10, pbVar13, 4);
    uVar11 = (unsigned int)iVar10 * 0x23u + 0x1fu + (unsigned int)DAT_009a1fe8;
    __builtin_memcpy(&iVar10, (unsigned char *)uVar11, 4);
    uVar11 = (unsigned int)iVar10 + 0xa9u;
    __builtin_memcpy(&iVar10, (unsigned char *)uVar11, 4);
    uVar11 = (unsigned int)iVar10 + 0x1e1u;
    __builtin_memcpy(&uVar2, (unsigned char *)uVar11, 2);

    if (((uVar2 & 0x80) != 0) && ((uVar2 & 0x100) != 0)) {
        DAT_00a083ba = DAT_00a08a02;
        DAT_00a08344 = DAT_00a08c7e;
        DAT_00a08c7e = 0;
        DAT_00a08a02 = 0;
        DAT_00a08c88 = DAT_00a0a1d2;
        DAT_00a08c86 = DAT_00a0a1d4;
        if (DAT_00a08364 == 0) {
            DAT_00a08360 = DAT_00a083c0;
            DAT_00a08a04 = DAT_00a084e0;
            DAT_00a08bb0 = DAT_00a0a540;
            DAT_00a08340 = DAT_00a0a660;
        } else {
            DAT_00a08360 = DAT_00a084e0;
            DAT_00a08a04 = DAT_00a083c0;
            DAT_00a08bb0 = DAT_00a0a660;
            DAT_00a08340 = DAT_00a0a540;
        }

        if (_DAT_00a07cec < DAT_009a1fe0) {
            iVar8 = (int)((unsigned int)_DAT_00a07cec * 0x23u);
            uVar5 = (unsigned int)_DAT_00a07cec;
            do {
                pbVar13 = DAT_009e9670 + (unsigned int)DAT_009ecc34 * 0x5bdu;
                __builtin_memcpy(&uVar11, pbVar13, 4);
                if ((uVar11 != uVar5) &&
                    ((DAT_00521f2e[(unsigned int)*(unsigned char *)((unsigned int)DAT_009a1fe8 + (unsigned int)iVar8 + 3u) * 0x1au] & 1) != 0)) {
                    uVar11 = (unsigned int)DAT_009a1fe8 + (unsigned int)iVar8 + 0x1fu;
                    __builtin_memcpy(&iVar10, (unsigned char *)uVar11, 4);
                    uVar11 = (unsigned int)iVar10 + 0xa9u;
                    __builtin_memcpy(&iVar6, (unsigned char *)uVar11, 4);
                    __builtin_memcpy(&uVar12, pbVar13 + 0x1a, 2);
                    if (((unsigned int)uVar12 == uVar5) ||
                        (((uVar3 = FUN_00484f80((unsigned short)uVar5), (char)uVar3 == '\0') &&
                          (cVar1 = *(char *)((unsigned int)iVar6 + 7u), cVar1 != '\x03')) && (cVar1 != '\x04'))) {
                        FUN_0043a0c0((unsigned short)uVar5);
                    }
                }
                iVar8 = (int)((unsigned int)iVar8 + 0x23u);
                uVar5 = uVar5 + 1u;
            } while ((int)uVar5 < DAT_009a1fe0);
        }

        if (DAT_009e9640 < DAT_00a00854) {
            iVar8 = (int)((unsigned int)DAT_009e9640 * 0x23u);
            iVar6 = DAT_009e9640;
            do {
                if ((DAT_00521f2e[(unsigned int)*(unsigned char *)((unsigned int)DAT_009a1fe8 + 3u + (unsigned int)iVar8) * 0x1au] & 1) != 0) {
                    FUN_0043a0c0((unsigned short)iVar6);
                }
                iVar8 = (int)((unsigned int)iVar8 + 0x23u);
                iVar6 = (int)((unsigned int)iVar6 + 1u);
            } while (iVar6 < DAT_00a00854);
        }

        iVar8 = (int)((unsigned int)DAT_009d1308 + (unsigned int)DAT_00a07ce8);
        if (DAT_00a07ce8 < iVar8) {
            iVar6 = (int)((unsigned int)DAT_00a07ce8 * 0x23u);
            iVar7 = DAT_00a07ce8;
            do {
                if ((DAT_00521f2e[(unsigned int)*(unsigned char *)((unsigned int)DAT_009a1fe8 + 3u + (unsigned int)iVar6) * 0x1au] & 1) != 0) {
                    FUN_0043a0c0((unsigned short)iVar7);
                }
                iVar6 = (int)((unsigned int)iVar6 + 0x23u);
                iVar7 = (int)((unsigned int)iVar7 + 1u);
                iVar8 = (int)((unsigned int)DAT_009d1308 + (unsigned int)DAT_00a07ce8);
            } while (iVar7 < iVar8);
        }

        if (DAT_00a0837a != 0) {
            (*DAT_009a1fec)();
        }
        if (DAT_00a083ba != 0) {
            (*DAT_009d12e4)(DAT_00a08360, (unsigned int)DAT_00a083ba);
        }
        if (DAT_00a08a02 != 0) {
            (*DAT_009a20a8)(DAT_00a08a04, (unsigned int)DAT_00a08a02);
        }
        if (DAT_00a08344 != 0) {
            (*DAT_009d12e4)(DAT_00a08bb0, (unsigned int)DAT_00a08344);
        }
        if (DAT_00a08c7e != 0) {
            (*DAT_009a20a8)(DAT_00a08340, (unsigned int)DAT_00a08c7e);
        }

        pbVar13 = DAT_009e9670 + (unsigned int)DAT_009ecc34 * 0x5bdu;
        __builtin_memcpy(&uVar12, pbVar13 + 0x1a, 2);
        bVar9 = ((short)uVar12 != -1);
        if (bVar9) {
            (*DAT_009ec46c)();
        }
        DAT_00a08364 = DAT_00a08364 ^ 1;
        DAT_00a0837a = (unsigned char)bVar9;
    }
    return;
}
