/* SPDX-License-Identifier: MIT */
/* z_xvt 0044c960 */
extern unsigned short DAT_00732a3c;
extern int DAT_006d4980;
extern int DAT_006d4988;
extern unsigned short DAT_006d9d60;
extern char DAT_006d9d78;
extern unsigned short DAT_006d9d50;
extern int DAT_00732a38;
extern int DAT_00703c88;
extern unsigned char DAT_0052687a[];

extern unsigned short __cdecl FUN_0049ec30(unsigned short param_1, unsigned short param_2, unsigned short param_3);
extern int __cdecl FUN_00429a20(unsigned int param_1);

int __cdecl FUN_0044c960(unsigned short param_1, unsigned short param_2, short param_3, unsigned short param_4, unsigned short param_5)
{
    unsigned short local_6;
    int iVar1;
    int iVar8;
    int iVar7;
    unsigned short uVar4;
    unsigned short uVar5;
    unsigned short uVar6;
    int bVar3;

    local_6 = DAT_00732a3c;
    iVar1 = 0;
    iVar8 = 0;
    if ((int)(unsigned int)DAT_00732a3c < DAT_006d4980) {
        do {
            iVar1 = iVar8;
            iVar8 = (unsigned int)local_6 * 0x23;
            iVar7 = DAT_006d4988 + iVar8;
            if ((*(char *)(iVar7 + 3) != '\0') &&
               (uVar5 = (unsigned short)*(unsigned char *)(iVar7 + 0x16), uVar5 != DAT_006d9d60)) {
                uVar4 = FUN_0049ec30(uVar5, param_1, param_2);
                uVar5 = FUN_0049ec30(uVar5, param_4, param_5);
                if (param_3 == 1) {
                    uVar4 = uVar4 | uVar5;
                }
                else {
                    uVar4 = uVar4 & uVar5;
                }
                if (uVar4 != 0) {
                    bVar3 = FUN_00429a20((unsigned int)local_6);
                    if (bVar3 != 0) {
                        iVar8 = *(int *)(DAT_006d4988 + 0x1f + iVar8);
                        iVar7 = *(int *)(iVar8 + 0xa7);
                        if ((DAT_006d9d78 == '\0') ||
                           ((*(short *)(iVar7 + 0x1df) != 0 &&
                            (((DAT_006d9d78 == '\0' || (*(char *)(iVar7 + 0x1e5) == '\0')) ||
                             (*(char *)(*(int *)((unsigned int)DAT_006d9d50 * 0x23 + 0x1f + DAT_006d4988) + 0x87) !=
                              *(char *)(iVar8 + 0x87))))))) {
                            iVar1 = iVar1 + 1;
                        }
                    }
                }
            }
            local_6 = local_6 + 1;
            iVar8 = iVar1;
        } while ((int)(unsigned int)local_6 < DAT_006d4980);
    }
    uVar5 = (unsigned short)DAT_00732a38;
    if ((int)(DAT_00732a38 & 0xffff) < (int)(DAT_00703c88 + DAT_00732a38)) {
        do {
            iVar8 = (unsigned int)uVar5 * 0x23 + DAT_006d4988;
            bVar3 = *(unsigned char *)(iVar8 + 3);
            if ((bVar3 != 0) && ((DAT_0052687a[(unsigned int)bVar3 * 0x16] & 2) != 0)) {
                bVar3 = *(unsigned char *)(iVar8 + 0x16);
                uVar4 = FUN_0049ec30((unsigned short)bVar3, param_1, param_2);
                uVar6 = FUN_0049ec30((unsigned short)bVar3, param_4, param_5);
                if (param_3 == 1) {
                    uVar4 = uVar4 | uVar6;
                }
                else {
                    uVar4 = uVar4 & uVar6;
                }
                if (uVar4 != 0) {
                    bVar3 = FUN_00429a20((unsigned int)uVar5);
                    if (bVar3 != 0) {
                        iVar1 = iVar1 + 1;
                    }
                }
            }
            uVar5 = uVar5 + 1;
        } while ((int)(unsigned int)uVar5 < (int)(DAT_00703c88 + DAT_00732a38));
    }
    if (iVar1 == 0) {
        return 0xffff;
    }
    return iVar1;
}
