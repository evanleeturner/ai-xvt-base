/* SPDX-License-Identifier: MIT */
/* z_xvt 00435c70 */
extern int DAT_00732a24;
extern int DAT_00732a28;
extern int DAT_00732a2c;
extern int DAT_0071ecf4;
extern unsigned char DAT_0071bb1b;
extern unsigned char DAT_0071bb1f;
extern unsigned char DAT_0071bb23;
extern int DAT_0072f450;
extern int DAT_0072f44c;
extern int DAT_0072f448;
extern int DAT_006db7b4;
extern int DAT_006db7ac;
extern int DAT_006db7b0;
extern int DAT_00732568;
extern int DAT_0073255c;
extern int DAT_00732558;
extern int DAT_00732530;
extern int DAT_00732584;
extern int DAT_00732578;
extern short DAT_00732e8c;   /* 2-byte, signedness assumed from (int) cast */
extern short DAT_006d9980;   /* 2-byte, signedness assumed from (int) cast */
extern void (*DAT_006fedd4)(int, int, int, int, unsigned int);

int  FUN_0040ca60(int, int, int);
int  FUN_0040ca00(int, int, int);
int  FUN_0040ca30(int, int, int);
void FUN_0040c970(unsigned int, unsigned int, unsigned int);
void FUN_004a6a80(void);
int  FUN_0040cad0(unsigned int, unsigned int);
int  FUN_0040ca90(unsigned int, unsigned int);
void FUN_004a6f90(void);

void __stdcall FUN_00435c70(void)
{
    unsigned int uVar1;
    unsigned int uVar2;
    int iVar3;
    int iVar4;
    int iVar5;
    int iVar6;
    unsigned int uVar7;
    int iVar8;
    int iVar9;
    unsigned int uVar10;
    unsigned int uVar11;
    unsigned int uVar12;
    unsigned int uVar13;
    int local_c;
    int local_8;
    int local_4;
    int f1f;
    int f1b;
    int f23;

    DAT_00732a24 = 0;
    DAT_00732a28 = 0;
    DAT_00732a2c = 0;
    local_8 = -0x100000;
    local_c = 0x21;
    do {
        __builtin_memcpy(&f1f, &DAT_0071bb1f + (unsigned int)DAT_0071ecf4 * 0x5bd, 4);
        DAT_0072f450 = (int)((unsigned int)local_8 - (unsigned int)f1f);
        __builtin_memcpy(&f1b, &DAT_0071bb1b + (unsigned int)DAT_0071ecf4 * 0x5bd, 4);
        DAT_0072f44c = (int)(0xfff00000u - (unsigned int)f1b);
        __builtin_memcpy(&f23, &DAT_0071bb23 + (unsigned int)DAT_0071ecf4 * 0x5bd, 4);
        DAT_0072f448 = (int)(0xffff0000u - (unsigned int)f23);
        DAT_006db7b4 = FUN_0040ca60(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
        uVar11 = (unsigned int)DAT_00732568 * 0x40 + (unsigned int)DAT_006db7b4;
        if ((0 < (int)uVar11) || (0 < DAT_006db7b4)) {
            DAT_006db7ac = FUN_0040ca00(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
            DAT_006db7b0 = FUN_0040ca30(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
            uVar1 = (unsigned int)DAT_0073255c * 0x40 + (unsigned int)DAT_006db7ac;
            uVar2 = (unsigned int)DAT_00732558 * 0x40 + (unsigned int)DAT_006db7b0;
            uVar7 = (unsigned int)DAT_006db7b4;
            uVar10 = uVar2;
            uVar12 = uVar1;
            if ((int)uVar11 < 1) {
                uVar7 = uVar11;
                uVar10 = (unsigned int)DAT_006db7b0;
                uVar11 = (unsigned int)DAT_006db7b4;
                uVar12 = (unsigned int)DAT_006db7ac;
                DAT_006db7ac = (int)uVar1;
                DAT_006db7b0 = (int)uVar2;
            }
            DAT_006db7b4 = (int)uVar7;
            if ((int)uVar7 < 1) {
                FUN_0040c970(uVar12, uVar10, uVar11);
            }
            FUN_004a6a80();
            iVar8 = (int)DAT_00732e8c;
            iVar3 = (int)DAT_006d9980;
            uVar13 = 0x31;
            iVar4 = FUN_0040cad0(uVar10, uVar11);
            iVar4 = (int)((unsigned int)iVar4 + (unsigned int)iVar8);
            iVar5 = FUN_0040ca90(uVar12, uVar11);
            uVar11 = (unsigned int)DAT_006db7b4;
            iVar5 = (int)((unsigned int)iVar5 + (unsigned int)iVar3);
            iVar6 = FUN_0040cad0((unsigned int)DAT_006db7b0, uVar11);
            iVar6 = (int)((unsigned int)iVar6 + (unsigned int)iVar8);
            iVar8 = FUN_0040ca90((unsigned int)DAT_006db7ac, uVar11);
            (*DAT_006fedd4)((int)((unsigned int)iVar8 + (unsigned int)iVar3), iVar6, iVar5, iVar4, uVar13);
            FUN_004a6f90();
        }
        local_c = local_c + -1;
        local_8 = (int)((unsigned int)local_8 + 0x10000u);
    } while (local_c != 0);
    local_c = 0x21;
    iVar3 = (int)((unsigned int)DAT_00732a28 - 0x100000u);
    local_4 = (int)((unsigned int)DAT_00732a2c - 0x100000u);
    do {
        __builtin_memcpy(&f1f, &DAT_0071bb1f + (unsigned int)DAT_0071ecf4 * 0x5bd, 4);
        DAT_0072f450 = (int)((unsigned int)iVar3 - (unsigned int)f1f);
        __builtin_memcpy(&f1b, &DAT_0071bb1b + (unsigned int)DAT_0071ecf4 * 0x5bd, 4);
        DAT_0072f44c = (int)((unsigned int)local_4 - (unsigned int)f1b);
        __builtin_memcpy(&f23, &DAT_0071bb23 + (unsigned int)DAT_0071ecf4 * 0x5bd, 4);
        DAT_0072f448 = (int)(0xffff0000u - (unsigned int)f23);
        DAT_006db7b4 = FUN_0040ca60(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
        uVar11 = (unsigned int)DAT_00732530 * 0x40 + (unsigned int)DAT_006db7b4;
        if ((0 < (int)uVar11) || (0 < DAT_006db7b4)) {
            DAT_006db7ac = FUN_0040ca00(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
            DAT_006db7b0 = FUN_0040ca30(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
            uVar1 = (unsigned int)DAT_00732584 * 0x40 + (unsigned int)DAT_006db7ac;
            uVar2 = (unsigned int)DAT_00732578 * 0x40 + (unsigned int)DAT_006db7b0;
            uVar7 = (unsigned int)DAT_006db7b4;
            uVar10 = uVar2;
            uVar12 = uVar1;
            if ((int)uVar11 < 1) {
                uVar7 = uVar11;
                uVar10 = (unsigned int)DAT_006db7b0;
                uVar11 = (unsigned int)DAT_006db7b4;
                uVar12 = (unsigned int)DAT_006db7ac;
                DAT_006db7ac = (int)uVar1;
                DAT_006db7b0 = (int)uVar2;
            }
            DAT_006db7b4 = (int)uVar7;
            if ((int)uVar7 < 1) {
                FUN_0040c970(uVar12, uVar10, uVar11);
            }
            FUN_004a6a80();
            iVar9 = (int)DAT_00732e8c;
            iVar4 = (int)DAT_006d9980;
            uVar13 = 0x31;
            iVar5 = FUN_0040cad0(uVar10, uVar11);
            iVar5 = (int)((unsigned int)iVar5 + (unsigned int)iVar9);
            iVar6 = FUN_0040ca90(uVar12, uVar11);
            uVar11 = (unsigned int)DAT_006db7b4;
            iVar6 = (int)((unsigned int)iVar6 + (unsigned int)iVar4);
            iVar8 = FUN_0040cad0((unsigned int)DAT_006db7b0, uVar11);
            iVar8 = (int)((unsigned int)iVar8 + (unsigned int)iVar9);
            iVar9 = FUN_0040ca90((unsigned int)DAT_006db7ac, uVar11);
            (*DAT_006fedd4)((int)((unsigned int)iVar9 + (unsigned int)iVar4), iVar8, iVar6, iVar5, uVar13);
            FUN_004a6f90();
        }
        local_c = local_c + -1;
        local_4 = (int)((unsigned int)local_4 + 0x10000u);
    } while (local_c != 0);
    return;
}
