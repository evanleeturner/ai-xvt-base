/* SPDX-License-Identifier: MIT */
/* z_bop 0042c0a0 */
extern unsigned short DAT_00a00738[3];
extern unsigned short DAT_009ec5fe;
extern unsigned short DAT_005234c8[];
extern unsigned short DAT_005234c6[];
extern unsigned char  DAT_009d6943;
extern int            DAT_00520ee8;
extern int            _DAT_00520eec;
extern int            DAT_009a1fe8;
extern int            DAT_00a08104;
extern int            DAT_00a00870[];
extern int            DAT_00a00874[];
extern unsigned short DAT_009a8078;
extern short          DAT_009d694a;
extern short          DAT_009d694e;
extern short          DAT_009d6952;
extern int            DAT_009d6944;
extern unsigned short DAT_009d8b74;
extern int            DAT_009a8c2c;
extern int            DAT_009ecc34;
extern short          _DAT_00a07be0;

int  FUN_004adc40(unsigned int a);
int  FUN_004ae2a0(unsigned int a, int b);
int  FUN_0042c410(unsigned int a);
void FUN_00450650(int a, int b);
void FUN_0042cc40(void);
void FUN_0042e4d0(int a, int b, int c);
int  FUN_004ac760(void);
void FUN_0042bd80(unsigned short a);

void __stdcall FUN_0042c0a0(void)
{
    unsigned char bVar2;
    unsigned short uVar3;
    char cVar4;
    unsigned int uVar5;
    int iVar6;
    int iVar7;
    int iVar8;
    short sVar9;
    int iVar10;
    short sVar11;
    unsigned short uVar12;
    int iVar13;
    unsigned int local_c;
    short local_s[3];
    int ptr;
    int doAdd;

    uVar12 = 0;
    do {
        uVar5 = (unsigned int)uVar12;
        sVar9 = (short)((int)DAT_00a00738[uVar5] - (int)DAT_009ec5fe);
        DAT_00a00738[uVar5] = (unsigned short)sVar9;
        if (sVar9 < 0) {
            uVar3 = DAT_005234c8[DAT_009d6943];
            if (uVar12 == 2) {
                uVar3 = DAT_005234c6[DAT_009d6943];
            }
            sVar11 = (short)(1 - (short)((int)sVar9 / (int)(unsigned int)uVar3));
            local_s[uVar5] = sVar11;
            DAT_00a00738[uVar5] = (unsigned short)((unsigned int)uVar3 * (unsigned int)(int)sVar11 + (unsigned int)(int)sVar9);
        } else {
            local_s[uVar5] = 0;
        }
        uVar12 = (unsigned short)(uVar12 + 1);
    } while (uVar12 < 3);

    DAT_00520ee8 = (int)((unsigned int)DAT_00520ee8 - (unsigned int)DAT_009ec5fe);
    if (DAT_00520ee8 < 0) {
        DAT_00520ee8 = (int)((unsigned int)DAT_00520ee8 + 0x1d);
        _DAT_00520eec = (int)((unsigned int)_DAT_00520eec + 1);
    }

    local_c = 1;
    do {
        iVar8 = (int)(local_c * 0x23);

        __builtin_memcpy(&ptr, (unsigned char *)(DAT_009a1fe8 + 0x1f + iVar8), 4);
        __builtin_memcpy(&DAT_00a08104, (unsigned char *)(ptr + 0xa9), 4);

        if (_DAT_00520eec == 4) {
            _DAT_00520eec = 0;
        }
        __builtin_memcpy(&ptr, (unsigned char *)(DAT_009a1fe8 + 0x1f + iVar8), 4);
        *(unsigned char *)(ptr + 0x8a) = (unsigned char)_DAT_00520eec;
        if (_DAT_00520eec == 3) {
            __builtin_memcpy(&ptr, (unsigned char *)(DAT_009a1fe8 + 0x1f + iVar8), 4);
            *(unsigned char *)(ptr + 0x8a) = 1;
        }

        bVar2 = *(unsigned char *)(DAT_009a1fe8 + 3 + iVar8);
        if (bVar2 < 0x49) {
            iVar6 = DAT_00a00870[(unsigned int)bVar2 * 0x65];
        } else {
            iVar6 = FUN_004adc40((unsigned int)bVar2);
        }
        iVar10 = 0;
        iVar13 = 0;
        if (0 < iVar6) {
            do {
                bVar2 = *(unsigned char *)(DAT_009a1fe8 + 3 + iVar8);
                uVar5 = (unsigned int)bVar2;
                if (bVar2 < 0x49) {
                    if (iVar10 < 0) {
                        iVar7 = 0;
                    } else {
                        iVar7 = iVar10;
                        if (DAT_00a00870[uVar5 * 0x65] <= iVar10) {
                            iVar7 = DAT_00a00870[uVar5 * 0x65] - 1;
                        }
                        iVar7 = DAT_00a00874[iVar7 + uVar5 * 0x65];
                    }
                } else {
                    iVar7 = FUN_004ae2a0(uVar5, iVar10);
                }
                doAdd = 0;
                cVar4 = 0;
                if (iVar7 == 0x11) {
                    cVar4 = (char)local_s[0];
                    if (2 < DAT_009d6943) doAdd = 1;
                } else if (iVar7 == 0x12) {
                    cVar4 = (char)local_s[2];
                    if (5 < DAT_009d6943) doAdd = 1;
                } else if (iVar7 == 0x13) {
                    cVar4 = (char)local_s[1];
                    if (3 < DAT_009d6943) doAdd = 1;
                }
                if (doAdd) {
                    char *pcVar1 = (char *)(iVar13 + 0x30a + DAT_00a08104);
                    *pcVar1 = (char)(*pcVar1 + cVar4);
                }
                iVar10 = iVar10 + 1;
                iVar13 = iVar13 + 1;
            } while (iVar10 < iVar6);
        }

        uVar12 = (unsigned short)(local_c + 1);
        local_c = (unsigned int)uVar12;
        if (0xc < uVar12) {
            uVar12 = 1;
            if (DAT_009a8078 != 0xc) {
                uVar12 = (unsigned short)(DAT_009a8078 + 1);
            }
            iVar8 = FUN_0042c410((unsigned int)uVar12);
            if (iVar8 != 0) {
                DAT_009d694a = (short)(DAT_009d694a + 1);
                DAT_009d694e = (short)(DAT_009d694e - 1);
                DAT_009a8078 = uVar12;
                if (uVar12 == 1) {
                    FUN_00450650(0xc6, DAT_009ecc34);
                    DAT_009d6952 = 0;
                    FUN_0042cc40();
                    while (((char)(DAT_009d8b74 & 0xff) != 0) || ((DAT_009d8b74 >> 8) != 0)) {
                        if ((DAT_009d8b74 >> 8) == 0) {
                            DAT_009d8b74 = (unsigned short)(((unsigned int)';' << 8) |
                                           (unsigned char)((char)(DAT_009d8b74 & 0xff) - 1));
                        } else {
                            DAT_009d8b74 = (unsigned short)((DAT_009d8b74 & 0xff) |
                                           ((unsigned int)(unsigned char)((DAT_009d8b74 >> 8) - 1) << 8));
                        }
                        DAT_009d6952 = (short)(DAT_009d6952 + 10);
                        DAT_009d6944 = (int)((unsigned int)DAT_009d6944 + 10);
                        if (DAT_009d6944 % 100 == 0) {
                            FUN_0042e4d0(0x2a, -1, DAT_009ecc34);
                        }
                        FUN_0042cc40();
                        do {
                            iVar8 = FUN_004ac760();
                            DAT_009a8c2c = (int)((unsigned int)DAT_009a8c2c + (unsigned int)iVar8);
                        } while (DAT_009a8c2c < 4);
                        DAT_009a8c2c = 0;
                    }
                    _DAT_00a07be0 = DAT_009d6952;
                    FUN_00450650(199, DAT_009ecc34);
                    DAT_009d6943 = (unsigned char)(DAT_009d6943 + 1);
                    FUN_0042bd80((unsigned short)DAT_009d6943);
                }
            }
            return;
        }
    } while (1);
}
