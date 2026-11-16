/* SPDX-License-Identifier: MIT */
/* z_xvt 004ec560 */
extern int DAT_0081b850;
extern unsigned char DAT_0081bfb6[4];   /* packed: 0x81bfb6 not 4-aligned */
extern unsigned char DAT_0081bfba[4];   /* packed */
extern unsigned char DAT_0081c012[4];   /* packed */
extern unsigned char DAT_0081d326[4];   /* packed */
extern unsigned char DAT_0081fb96[4];   /* packed */

extern int DAT_00757680[];
extern int DAT_00757870[];

extern int DAT_00757c80;
extern int DAT_00757c84;
extern int DAT_00757c88;
extern int DAT_00757c8c;
extern int DAT_00757c90;
extern unsigned int DAT_00757c94;
extern unsigned int DAT_00757c98;
extern int DAT_00757c9c;
extern int DAT_00757ca4;
extern int DAT_00757cac;
extern int DAT_00757cb0;
extern int DAT_00757cb4;
extern int DAT_00757cb8;

extern unsigned char DAT_007438d2[];
extern unsigned char DAT_007438d3[];
extern unsigned char DAT_007438d5[];
extern unsigned char DAT_007438d6[];
extern unsigned char DAT_007438e1[];
extern unsigned char DAT_00743da3[];
extern unsigned char DAT_00743db0[];
extern unsigned char DAT_00743db8[];
extern unsigned char DAT_00743dbe[];
extern unsigned char DAT_00743dc2[];
extern unsigned char DAT_00743dc3[];
extern unsigned char DAT_00743dcc[];
extern unsigned char DAT_00743dd6[];

extern unsigned char DAT_005333e8[];
extern unsigned int DAT_005333fc[];
extern unsigned int DAT_0053340c[];

int FUN_004cdbc0(void);
unsigned int FUN_004ecce0(int param_1);

void __stdcall FUN_004ec560(void)
{
    unsigned char bVar1;
    unsigned char bVar2;
    int iVar3;
    unsigned int uVar4;
    int iVar5;
    int iVar6;
    unsigned int uVar7;
    int local_bfb6;
    int local_bfba;
    int local_c012;
    int local_d326;
    int local_fb96;

    __builtin_memcpy(&local_bfb6, DAT_0081bfb6, 4);

    if (DAT_0081b850 == 2) {
        DAT_00757cb0 = DAT_00757680[(int)((unsigned int)local_bfb6 * 0x20u) / 4];
    }
    else {
        iVar5 = 0;
        do {
            iVar6 = DAT_00757870[iVar5 + (int)((unsigned int)local_bfb6 * 8u)];
            iVar3 = FUN_004cdbc0();
            if (iVar6 == iVar3) {
                DAT_00757cb0 = DAT_00757680[iVar5 + (int)((unsigned int)local_bfb6 * 8u)];
            }
            iVar5 = iVar5 + 1;
        } while (iVar5 < 8);
    }
    DAT_00757ca4 = 0;
    DAT_00757cac = 0;
    DAT_00757c84 = 0;
    DAT_00757c90 = 0;
    DAT_00757cb8 = 0;
    iVar5 = (int)((unsigned int)DAT_00757cb0 * 0x562u);
    DAT_00757c94 = (unsigned int)DAT_007438e1[iVar5];
    DAT_00757c98 = (unsigned int)DAT_007438d3[iVar5];
    switch (DAT_00743dc2[iVar5]) {
    case 0:
        DAT_00757cb4 = 0;
        DAT_00757c9c = 1;
        break;
    case 1:
        DAT_00757cb4 = 10;
        DAT_00757c9c = 0;
        break;
    case 2:
        DAT_00757cb4 = 5;
        DAT_00757c9c = 0;
        break;
    case 3:
        DAT_00757cb4 = 6;
        DAT_00757c9c = 0;
        break;
    case 4:
        DAT_00757c9c = 1;
        iVar6 = 0;
        DAT_00757cb4 = 0;
        do {
            if (DAT_00743dc3[iVar6 + iVar5] != 0) {
                DAT_00757c9c = DAT_00757c9c + 1;
            }
            iVar6 = iVar6 + 1;
        } while (iVar6 < 10);
    }
    DAT_00757c80 = 0;
    iVar6 = 0;
    do {
        if (DAT_00743db0[iVar6 + iVar5] != 0) {
            DAT_00757c80 = DAT_00757c80 + 1;
        }
        iVar6 = iVar6 + 1;
    } while (iVar6 < 8);
    if ((DAT_007438d5[iVar5] != 0) || (DAT_00757c80 != 0)) {
        DAT_00757c80 = DAT_00757c80 + 1;
    }
    DAT_00757c88 = 0;
    iVar6 = 0;
    do {
        if (DAT_00743db8[iVar6 + iVar5] != 0) {
            DAT_00757c88 = DAT_00757c88 + 1;
        }
        iVar6 = iVar6 + 1;
    } while (iVar6 < 6);
    if ((DAT_007438d6[iVar5] != 0) || (DAT_00757c88 != 0)) {
        DAT_00757c88 = DAT_00757c88 + 1;
    }
    DAT_00757c8c = 0;
    iVar6 = 0;
    do {
        if (DAT_00743dbe[iVar6 + iVar5] != 0) {
            DAT_00757c8c = DAT_00757c8c + 1;
        }
        iVar6 = iVar6 + 1;
    } while (iVar6 < 4);
    if ((DAT_00743da3[iVar5] != 0) || (DAT_00757c8c != 0)) {
        DAT_00757c8c = DAT_00757c8c + 1;
    }

    __builtin_memcpy(&local_c012, DAT_0081c012, 4);
    if (local_c012 != 1) {
        DAT_00757c84 = 0;
        DAT_00757c90 = 0;
        DAT_00757ca4 = 0;
        DAT_00757cac = 0;
        DAT_00757cb8 = 0;
        return;
    }
    __builtin_memcpy(&local_bfba, DAT_0081bfba, 4);
    if ((local_bfba != 1) && (local_bfba != 2)) {
        DAT_00757c84 = 0;
        DAT_00757c90 = 0;
        DAT_00757ca4 = 0;
        DAT_00757cac = 0;
        DAT_00757cb8 = 0;
        return;
    }
    __builtin_memcpy(&local_d326, DAT_0081d326, 4);
    if (local_d326 == 0) {
        DAT_00757c84 = 0;
        DAT_00757c90 = 0;
        DAT_00757ca4 = 0;
        DAT_00757cac = 0;
        DAT_00757cb8 = 0;
        return;
    }
    uVar7 = 0;
    uVar4 = FUN_004ecce0(-1);
    __builtin_memcpy(&local_fb96, DAT_0081fb96, 4);
    if (((int)uVar4 < 1) || ((3 < (int)uVar4 && (uVar4 != 0xe)))) {
        if (local_fb96 != 0) goto LAB_004ec78f;
    }
    else if (local_fb96 != 1) goto LAB_004ec78f;
    uVar7 = (unsigned int)DAT_005333e8[uVar4];
LAB_004ec78f:
    if (uVar7 != 0) {
        while (uVar4 = FUN_004ecce0(-1), uVar7 != uVar4) {
            if (DAT_00757cb4 == 0) {
                DAT_00757ca4 = DAT_00757ca4 + 1;
                if (DAT_00757c9c <= DAT_00757ca4) {
                    DAT_00757ca4 = 0;
                }
                if (DAT_00757ca4 == 0) {
                    bVar1 = DAT_007438d3[(int)((unsigned int)DAT_00757cb0 * 0x562u)];
                    bVar2 = DAT_007438e1[(int)((unsigned int)DAT_00757cb0 * 0x562u)];
                }
                else {
                    iVar5 = (int)((unsigned int)DAT_00757cb0 * 0x562u) + DAT_00757ca4;
                    bVar1 = DAT_00743dcc[iVar5];
                    bVar2 = DAT_00743dd6[iVar5];
                }
                DAT_00757c98 = (unsigned int)bVar1;
                DAT_00757c94 = (unsigned int)bVar2;
            }
            else {
                DAT_00757cac = DAT_00757cac + 1;
                if (DAT_00757cb4 <= DAT_00757cac) {
                    DAT_00757cac = 0;
                }
                if (DAT_00757cac != 0) {
                    iVar5 = (int)((unsigned int)DAT_00757cb0 * 0x562u);
                    bVar1 = DAT_00743dc2[iVar5];
                    if (bVar1 != 0) {
                        if (bVar1 < 3) {
                            if (DAT_005333fc[DAT_00757cac] == (unsigned int)DAT_007438d2[iVar5]) {
                                DAT_00757cac = DAT_00757cac + 1;
                            }
                        }
                        else if ((bVar1 == 3) &&
                                 (DAT_0053340c[DAT_00757cac] == (unsigned int)DAT_007438d2[iVar5])) {
                            DAT_00757cac = DAT_00757cac + 1;
                        }
                    }
                }
                if (DAT_00757cb4 <= DAT_00757cac) {
                    DAT_00757cac = 0;
                }
            }
        }
    }
    return;
}
