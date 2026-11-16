/* SPDX-License-Identifier: MIT */
/* z_xvt 00440d20 */
extern unsigned short DAT_007092b0;
extern unsigned char DAT_0071bb19[];
extern unsigned char DAT_0071bb1a[];
extern unsigned char DAT_0071bae7[];
extern unsigned char DAT_0070ac48[];
extern int DAT_0071ecf4;
extern unsigned char DAT_0071b73c[];
extern unsigned char DAT_0071e522;
extern unsigned char DAT_0072f508[];
extern unsigned short DAT_00732fe2;

void __cdecl FUN_004a3490(int, void *);
void __cdecl FUN_004a1d70(int, int);
int * __cdecl FUN_00432cc0(int);

void __cdecl FUN_00440d20(int param_1)
{
    int iVar2;
    unsigned char bVar4;
    unsigned char *puVar6;
    int local_8;
    int bVar3;
    int bVar7;
    unsigned short a;
    unsigned short b;
    int *piVar5;

    switch (DAT_007092b0) {
    case 8:
        iVar2 = (int)((unsigned int)param_1 * 0x5bdu);
        if (DAT_0071bb19[iVar2] != 0) {
            bVar4 = (unsigned char)(DAT_0071bb19[iVar2] - 1);
            DAT_0071bb19[iVar2] = bVar4;
            DAT_0071bae7[iVar2 + (unsigned int)bVar4] = 0x5f;
            DAT_0071bae7[1 + (unsigned int)DAT_0071bb19[iVar2] + iVar2] = 0;
        }
        FUN_004a3490(0, DAT_0071bae7 + iVar2);
        FUN_004a1d70((int)((unsigned int)DAT_0071bb1a[iVar2] + 0x16c), param_1);
        return;
    case 9:
        iVar2 = (int)((unsigned int)param_1 * 0x5bdu);
        DAT_0071bb1a[iVar2] = (unsigned char)(DAT_0071bb1a[iVar2] + 1);
        if (3 < DAT_0071bb1a[iVar2]) {
            DAT_0071bb1a[iVar2] = 1;
        }
        FUN_004a3490(0, DAT_0071bae7 + iVar2);
        FUN_004a1d70((int)((unsigned int)DAT_0071bb1a[iVar2] + 0x16c), param_1);
        return;
    default:
        if (DAT_007092b0 != 0) {
            iVar2 = (int)((unsigned int)param_1 * 0x5bdu);
            if (DAT_0071bb19[iVar2] < 0x30) {
                DAT_0071bae7[iVar2 + (unsigned int)DAT_0071bb19[iVar2]] = (unsigned char)DAT_007092b0;
                DAT_0071bae7[1 + (unsigned int)DAT_0071bb19[iVar2] + iVar2] = 0x5f;
                DAT_0071bae7[2 + (unsigned int)DAT_0071bb19[iVar2] + iVar2] = 0;
                DAT_0071bb19[iVar2] = (unsigned char)(DAT_0071bb19[iVar2] + 1);
            }
            FUN_004a3490(0, DAT_0071bae7 + iVar2);
            FUN_004a1d70((int)((unsigned int)DAT_0071bb1a[iVar2] + 0x16c), param_1);
            return;
        }
        if ((param_1 == DAT_0071ecf4) &&
            (*(short *)(DAT_0070ac48 + (int)((unsigned int)param_1 * 0x10u)) < 0xec)) {
            iVar2 = (int)((unsigned int)param_1 * 0x5bdu);
            FUN_004a3490(0, DAT_0071bae7 + iVar2);
            FUN_004a1d70((int)((unsigned int)DAT_0071bb1a[iVar2] + 0x16c), param_1);
            return;
        }
        break;
    case 0xd:
        iVar2 = (int)((unsigned int)param_1 * 0x5bdu);
        DAT_0071bae7[iVar2 + (unsigned int)DAT_0071bb19[iVar2]] = 0;
        piVar5 = FUN_00432cc0(param_1);
        FUN_004a3490(0, piVar5);
        puVar6 = DAT_0071b73c;
        FUN_004a3490(1, DAT_0071bae7 + iVar2);
        local_8 = 0;
        do {
            if (puVar6[4] != 0) {
                bVar3 = 0;
                DAT_00732fe2 = 3;
                if (DAT_0071bb1a[iVar2] == 1) {
                    __builtin_memcpy(&a, DAT_0071b73c + iVar2, 2);
                    __builtin_memcpy(&b, puVar6, 2);
                    if (a == b) {
                        bVar3 = 1;
                    }
                } else {
                    if (DAT_0071bb1a[iVar2] == 2) {
                        __builtin_memcpy(&a, DAT_0071b73c + iVar2, 2);
                        __builtin_memcpy(&b, puVar6, 2);
                        if (a == b) {
                            bVar7 = 0;
                        } else {
                            bVar7 = DAT_0072f508[(unsigned int)a + (unsigned int)b * 0x1e5u] == 0;
                        }
                        if (!bVar7) {
                            goto LAB_00440fd7;
                        }
                    }
                    bVar3 = 1;
                }
LAB_00440fd7:
                if (bVar3) {
                    FUN_004a1d70(0x16c, local_8);
                }
            }
            puVar6 = puVar6 + 0x5bd;
            local_8 = local_8 + 1;
            if ((unsigned char *)&DAT_0071e522 + 1 < puVar6) {
                DAT_0071bb1a[iVar2] = 0;
                FUN_004a1d70(0x170, param_1);
                return;
            }
        } while (1);
    case 0x1b:
        DAT_0071bb1a[(int)((unsigned int)param_1 * 0x5bdu)] = 0;
        FUN_004a1d70(0x171, param_1);
    }
    return;
}
