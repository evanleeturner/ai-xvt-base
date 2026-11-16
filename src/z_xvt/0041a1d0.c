/* SPDX-License-Identifier: MIT */
/* z_xvt 0041a1d0 */
extern int DAT_00732e54;
extern int DAT_006d4988;
extern char DAT_007092c1;
extern short DAT_007092d0;
extern int DAT_007092c4;
extern unsigned char DAT_0070ac35;
extern char DAT_0070ac34;
extern int DAT_0070ab18;
extern int DAT_0070ab14;
extern int DAT_0070ac20;
extern int DAT_0071ecf4;
extern unsigned char DAT_0071b730;

unsigned int FUN_0044a600(unsigned int a, unsigned int b);
unsigned short FUN_00422710(int a);
unsigned int FUN_00449b30(unsigned int a, unsigned int b);
int FUN_00449bf0(unsigned int a, unsigned int b);
int FUN_00449cb0(unsigned int a, unsigned int b);
long long FUN_00418450(short a);
long long FUN_004187b0(short a);
unsigned int FUN_00413d50(int a, int b, unsigned int c, int d);
void FUN_0042a230(int a, int b, int c, short d, short e, short f);
unsigned int FUN_00478470(void);
void FUN_0040a1a0(unsigned int a, unsigned int b, int c);
int FUN_0044a510(unsigned int a, unsigned int b);
unsigned int FUN_00449790(unsigned int a, unsigned int b);
void FUN_0040b730(int a, int b, int c, int d, unsigned int e, int f);

unsigned short __cdecl FUN_0041a1d0(unsigned int param_1, short param_2, unsigned short param_3, unsigned int param_4)
{
    unsigned char *pcVar2;
    unsigned char bVar3;
    int iVar4;
    int iVar5;
    char cVar6;
    unsigned short uVar7;
    unsigned int uVar8;
    int iVar9;
    unsigned int uVar10;
    unsigned int uVar11;
    short sVar12;
    unsigned int uVar13;
    long long lVar14;
    long long lVar15;
    unsigned int local_14;
    int local_10;
    unsigned char *pb;
    unsigned char *pp;
    unsigned char *pq;
    unsigned int tmpu;
    unsigned short tmps;

    uVar13 = (unsigned int)(unsigned short)(param_2 - 1);
    bVar3 = *(unsigned char *)(DAT_00732e54 + 0x31a + uVar13);
    pcVar2 = (unsigned char *)(DAT_00732e54 + 0x31a + uVar13);
    if (bVar3 == 0) {
        return param_3;
    }
    if (bVar3 == 0xff) {
        return param_3;
    }
    param_3 = param_3 >> 4;
    if (param_3 == 0) {
        param_3 = 1;
    }
    if (param_3 < bVar3) {
        cVar6 = (char)param_3;
        param_3 = 0;
        *pcVar2 = (unsigned char)(bVar3 - cVar6);
    }
    else {
        *pcVar2 = 0;
        param_3 = (unsigned short)((unsigned int)(param_3 - bVar3) * 0x10u);
        iVar4 = (int)((((param_1 & 0xffff) * 8u) - (param_1 & 0xffff)) * 5u);
        uVar8 = FUN_0044a600((unsigned int)*(unsigned char *)(DAT_006d4988 + 3 + iVar4), uVar13);
        if (uVar8 != 0) {
            *(unsigned char *)(DAT_00732e54 + 0x2ca + uVar13) = 2;
            bVar3 = DAT_0070ac35;
            if (DAT_007092c1 != '\0') {
                DAT_007092d0 = (short)(DAT_007092d0 + 1);
                iVar5 = (int)((unsigned int)DAT_007092c4 + 0x32u);
                if (*(char *)(DAT_00732e54 + 0x2f2 + uVar13) != '\0') {
                    iVar5 = (int)((unsigned int)DAT_007092c4 + 100u);
                }
                DAT_007092c4 = iVar5;
                bVar3 = (unsigned char)(DAT_0070ac35 + 2);
                if (0x3b < (unsigned char)(DAT_0070ac35 + 2)) {
                    DAT_0070ac34 = (char)(DAT_0070ac34 + 1);
                    bVar3 = (unsigned char)(DAT_0070ac35 - 0x3a);
                }
            }
            DAT_0070ac35 = bVar3;
            uVar7 = FUN_00422710(0xd);
            if (uVar7 != 0xffff) {
                uVar8 = (unsigned int)uVar7;
                iVar5 = (int)(uVar8 * 0x23u);

                pb = (unsigned char *)DAT_006d4988;
                __builtin_memcpy(&tmpu, pb + 4 + iVar4, 4);
                __builtin_memcpy(pb + 4 + iVar5, &tmpu, 4);
                pb = (unsigned char *)DAT_006d4988;
                __builtin_memcpy(&tmpu, pb + 8 + iVar4, 4);
                __builtin_memcpy(pb + 8 + iVar5, &tmpu, 4);
                pb = (unsigned char *)DAT_006d4988;
                __builtin_memcpy(&tmpu, pb + 0xc + iVar4, 4);
                __builtin_memcpy(pb + 0xc + iVar5, &tmpu, 4);

                local_14 = FUN_00449b30((unsigned int)*(unsigned char *)(DAT_006d4988 + 3 + iVar4), uVar13);
                iVar9 = FUN_00449bf0((unsigned int)*(unsigned char *)(DAT_006d4988 + 3 + iVar4), uVar13);
                local_10 = FUN_00449cb0((unsigned int)*(unsigned char *)(DAT_006d4988 + 3 + iVar4), uVar13);
                if ((DAT_007092c1 != '\0') &&
                    (bVar3 = *(unsigned char *)(DAT_00732e54 + 0x2f2 + uVar13), bVar3 != 0)) {
                    sVar12 = (short)((unsigned int)bVar3 * (unsigned int)-0x100);
                    lVar14 = FUN_00418450(sVar12);
                    lVar15 = FUN_004187b0(sVar12);
                    uVar10 = FUN_00413d50((int)(short)lVar15,
                                          (int)(0u - (unsigned int)(int)(short)lVar14),
                                          local_14, local_10);
                    local_10 = (int)FUN_00413d50((int)(short)lVar14, (int)(short)lVar15,
                                                 local_14, local_10);
                    local_14 = uVar10 & 0xffff;
                }
                DAT_0070ab18 = (int)local_14;
                DAT_0070ab14 = local_10;
                DAT_0070ac20 = (int)(0u - (unsigned int)iVar9);
                iVar9 = DAT_006d4988 + iVar4;
                if (*(char *)(iVar9 + 3) == '5') {
                    DAT_0070ab18 = (int)local_14 >> 1;
                    DAT_0070ac20 = DAT_0070ac20 >> 1;
                    DAT_0070ab14 = local_10 >> 1;
                }
                FUN_0042a230(iVar9, DAT_0070ab14, iVar9, (short)DAT_0070ab18,
                             (short)DAT_0070ab14, (short)DAT_0070ac20);
                if (*(char *)(DAT_006d4988 + 3 + iVar4) == '5') {
                    DAT_0070ab18 = (int)((unsigned int)DAT_0070ab18 * 2u);
                    DAT_0070ab14 = (int)((unsigned int)DAT_0070ab14 * 2u);
                    DAT_0070ac20 = (int)((unsigned int)DAT_0070ac20 * 2u);
                }
                pb = (unsigned char *)DAT_006d4988;
                __builtin_memcpy(&tmpu, pb + 4 + iVar5, 4);
                tmpu = tmpu + (unsigned int)DAT_0070ab18;
                __builtin_memcpy(pb + 4 + iVar5, &tmpu, 4);
                pb = (unsigned char *)DAT_006d4988;
                __builtin_memcpy(&tmpu, pb + 8 + iVar5, 4);
                tmpu = tmpu + (unsigned int)DAT_0070ab14;
                __builtin_memcpy(pb + 8 + iVar5, &tmpu, 4);
                pb = (unsigned char *)DAT_006d4988;
                __builtin_memcpy(&tmpu, pb + 0xc + iVar5, 4);
                tmpu = tmpu + (unsigned int)DAT_0070ac20;
                __builtin_memcpy(pb + 0xc + iVar5, &tmpu, 4);

                *(unsigned char *)(DAT_006d4988 + 3 + iVar5) = 0x81;
                *(unsigned char *)(DAT_006d4988 + 2 + iVar5) = 0xd;
                __builtin_memcpy(&pp, (unsigned char *)DAT_006d4988 + 0x1f + iVar5, 4);
                *pp = 5;
                *(unsigned char *)(DAT_006d4988 + 0x19 + iVar5) = 2;
                tmps = 0;
                __builtin_memcpy(&pp, (unsigned char *)DAT_006d4988 + 0x1f + iVar5, 4);
                __builtin_memcpy(pp + 0x81, &tmps, 2);
                __builtin_memcpy(&pp, (unsigned char *)DAT_006d4988 + 0x1f + iVar5, 4);
                __builtin_memcpy(pp + 0x7f, &tmps, 2);
                __builtin_memcpy(&pq, (unsigned char *)DAT_006d4988 + 0x1f + iVar4, 4);
                __builtin_memcpy(&tmps, pq + 0x79, 2);
                __builtin_memcpy(&pp, (unsigned char *)DAT_006d4988 + 0x1f + iVar5, 4);
                __builtin_memcpy(pp + 0x79, &tmps, 2);

                pb = (unsigned char *)DAT_006d4988;
                __builtin_memcpy(&tmps, pb + 0x12 + iVar4, 2);
                __builtin_memcpy(pb + 0x12 + iVar5, &tmps, 2);
                pb = (unsigned char *)DAT_006d4988;
                __builtin_memcpy(&tmps, pb + 0x10 + iVar4, 2);
                __builtin_memcpy(pb + 0x10 + iVar5, &tmps, 2);
                tmps = 0;
                __builtin_memcpy((unsigned char *)DAT_006d4988 + 0x14 + iVar5, &tmps, 2);

                __builtin_memcpy(&pp, (unsigned char *)DAT_006d4988 + 0x1f + iVar5, 4);
                pp[0x90] = 1;
                __builtin_memcpy(&pp, (unsigned char *)DAT_006d4988 + 0x1f + iVar5, 4);
                pp[0x89] = pp[0x90];
                iVar9 = DAT_0071ecf4;
                uVar10 = FUN_00478470();
                FUN_0040a1a0((uVar10 & 3) + 0x16, uVar8, iVar9);
                iVar9 = FUN_0044a510((unsigned int)*(unsigned char *)(DAT_006d4988 + 3 + iVar4), uVar13);
                iVar9 = iVar9 >> 9;
                if (0xff < iVar9) {
                    iVar9 = 0xff;
                }
                __builtin_memcpy(&pp, (unsigned char *)DAT_006d4988 + 0x1f + iVar5, 4);
                pp[1] = (unsigned char)iVar9;
            }
            __builtin_memcpy(&tmpu, &DAT_0071b730 + (unsigned int)DAT_0071ecf4 * 0x5bdu, 4);
            if (tmpu == (param_4 & 0xffff)) {
                uVar11 = FUN_00449790((unsigned int)*(unsigned char *)(DAT_006d4988 + 3 + iVar4), uVar13);
                switch (uVar11) {
                case 4:
                case 5:
                case 0x15:
                    FUN_0040b730(DAT_0071ecf4, -1, 0x17, 2, param_1 & 0xffff, 0x4000);
                    break;
                case 8:
                    FUN_0040b730(DAT_0071ecf4, -1, 0x17, 3, param_1 & 0xffff, 0xffff);
                    break;
                case 10:
                case 0x16:
                    FUN_0040b730(DAT_0071ecf4, -1, 0x17, 1, param_1 & 0xffff, 0xffff);
                    break;
                case 0xb:
                case 0xc:
                case 0x17:
                case 0x18:
                    FUN_0040b730(DAT_0071ecf4, -1, 0x17, 4, param_1 & 0xffff, 0xffff);
                }
            }
        }
    }
    return param_3;
}
