/* SPDX-License-Identifier: MIT */
/* z_xvt 0044efc0 */
extern unsigned short DAT_00732a3c;
extern int DAT_006d4980;
extern int DAT_006d4988;
extern char DAT_006d9d78;
extern int DAT_00732e94;
extern int DAT_00732e90;
extern int DAT_00732e98;
extern unsigned int DAT_006d499c;
extern int DAT_006d4974;
extern int DAT_006d4978;
extern int DAT_006d4970;
extern int DAT_00732a2c;
extern int DAT_00732a28;
extern int DAT_00732a24;
extern short DAT_006d9d50;
extern int DAT_00732a38;
extern int DAT_00703c88;
extern unsigned char DAT_0052687a[];

unsigned char __cdecl FUN_0049f010(unsigned int param_1, short param_2, unsigned short param_3);
unsigned char __cdecl FUN_00429a20(unsigned int param_1);
unsigned int __cdecl FUN_00412810(int param_1, int param_2, int param_3);
void __cdecl FUN_00421c70(unsigned int param_1, int param_2);
int __cdecl FUN_00419190(short param_1, short param_2);

unsigned int __cdecl FUN_0044efc0(short param_1, unsigned short param_2, short param_3,
                                  short param_4, unsigned short param_5, short param_6)
{
    unsigned char bVar1;
    unsigned char bVar2;
    unsigned short uVar3;
    int iVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    unsigned short uVar7;
    unsigned int uVar8;
    unsigned short local_4;
    short local_2;

    local_2 = -1;
    uVar8 = (unsigned int)DAT_00732a3c;
    uVar5 = 0xffffffff;
    if ((int)(unsigned int)DAT_00732a3c < DAT_006d4980) {
        do {
            uVar6 = uVar8 & 0xffff;
            iVar4 = (int)(uVar6 * 0x23);
            if (*(char *)((unsigned int)DAT_006d4988 + 3 + (unsigned int)iVar4) != '\0') {
                bVar2 = FUN_0049f010(uVar8, param_1, param_2);
                local_4 = (unsigned short)bVar2;
                bVar2 = FUN_0049f010(uVar8, param_4, param_5);
                uVar7 = (unsigned short)bVar2;
                if (param_3 == 1) {
                    local_4 = local_4 | uVar7;
                } else {
                    local_4 = local_4 & uVar7;
                }
                if (local_4 != 0) {
                    int cond = 0;
                    if (DAT_006d9d78 == '\0') {
                        cond = 1;
                    } else {
                        unsigned int ptr1;
                        unsigned int ptr2;
                        short sval;
                        __builtin_memcpy(&ptr1, (unsigned char *)((unsigned int)DAT_006d4988 + 0x1f + (unsigned int)iVar4), 4);
                        __builtin_memcpy(&ptr2, (unsigned char *)(ptr1 + 0xa7), 4);
                        __builtin_memcpy(&sval, (unsigned char *)(ptr2 + 0x1df), 2);
                        if (sval != 0) cond = 1;
                    }
                    if (cond && FUN_00429a20(uVar6) != 0) {
                        unsigned int p;
                        int f4, f8, fc;
                        p = (unsigned int)DAT_006d4988 + (unsigned int)iVar4;
                        __builtin_memcpy(&f4, (unsigned char *)(p + 4), 4);
                        __builtin_memcpy(&f8, (unsigned char *)(p + 8), 4);
                        __builtin_memcpy(&fc, (unsigned char *)(p + 0xc), 4);
                        DAT_006d499c = FUN_00412810((int)((unsigned int)f4 - (unsigned int)DAT_00732e94),
                                                    (int)((unsigned int)f8 - (unsigned int)DAT_00732e90),
                                                    (int)((unsigned int)fc - (unsigned int)DAT_00732e98));
                        if (DAT_006d499c < uVar5) {
                            if (param_6 != 0) {
                                FUN_00421c70(uVar6, 0);
                                DAT_006d4974 = DAT_00732a2c;
                                DAT_006d4978 = DAT_00732a28;
                                DAT_006d4970 = DAT_00732a24;
                                iVar4 = FUN_00419190(DAT_006d9d50, DAT_006d9d50);
                                if (iVar4 != 0) goto LAB_0044f12b;
                            }
                            uVar5 = DAT_006d499c;
                            local_2 = (short)uVar8;
                        }
                    }
                }
            }
LAB_0044f12b:
            uVar7 = (unsigned short)((short)uVar8 + 1);
            uVar8 = (uVar8 & 0xffff0000u) | (unsigned int)uVar7;
        } while ((int)(unsigned int)uVar7 < DAT_006d4980);
    }
    uVar8 = (uVar8 & 0xffff0000u) | (unsigned int)(unsigned short)DAT_00732a38;
    iVar4 = (int)((unsigned int)DAT_00703c88 + (unsigned int)DAT_00732a38);
    if ((int)((unsigned int)DAT_00732a38 & 0xffff) < iVar4) {
        do {
            uVar6 = uVar8 & 0xffff;
            bVar1 = *(unsigned char *)(uVar6 * 0x23 + 3 + (unsigned int)DAT_006d4988);
            if ((bVar1 != 0) && ((DAT_0052687a[(unsigned int)bVar1 * 0x16] & 2) != 0)) {
                bVar2 = FUN_0049f010(uVar8, param_1, param_2);
                uVar7 = (unsigned short)bVar2;
                bVar2 = FUN_0049f010(uVar8, param_4, param_5);
                uVar3 = (unsigned short)bVar2;
                if (param_3 == 1) {
                    uVar7 = uVar7 | uVar3;
                } else {
                    uVar7 = uVar7 & uVar3;
                }
                if ((uVar7 != 0) && FUN_00429a20(uVar6) != 0) {
                    FUN_00421c70(uVar6, 0);
                    DAT_006d499c = FUN_00412810((int)((unsigned int)DAT_00732a2c - (unsigned int)DAT_00732e94),
                                                (int)((unsigned int)DAT_00732a28 - (unsigned int)DAT_00732e90),
                                                (int)((unsigned int)DAT_00732a24 - (unsigned int)DAT_00732e98));
                    if (DAT_006d499c < uVar5) {
                        if (param_6 != 0) {
                            DAT_006d4974 = DAT_00732a2c;
                            DAT_006d4978 = DAT_00732a28;
                            DAT_006d4970 = DAT_00732a24;
                            iVar4 = FUN_00419190(DAT_006d9d50, DAT_006d9d50);
                            if (iVar4 != 0) goto LAB_0044f292;
                        }
                        uVar5 = DAT_006d499c;
                        local_2 = (short)uVar8;
                    }
                }
            }
LAB_0044f292:
            uVar7 = (unsigned short)((short)uVar8 + 1);
            uVar8 = (uVar8 & 0xffff0000u) | (unsigned int)uVar7;
            iVar4 = (int)((unsigned int)DAT_00703c88 + (unsigned int)DAT_00732a38);
        } while ((int)(unsigned int)uVar7 < iVar4);
    }
    if (0x10000 < uVar5) {
        local_2 = -1;
    }
    return (((unsigned int)iVar4 >> 16) << 16) | (unsigned int)(unsigned short)local_2;
}
