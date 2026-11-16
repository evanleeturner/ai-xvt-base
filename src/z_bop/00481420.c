/* SPDX-License-Identifier: MIT */
/* z_bop 00481420 */
extern unsigned short DAT_009d6930;
extern unsigned char DAT_009e9a27[];
extern unsigned char DAT_009e9a59[];
extern unsigned char DAT_009e9a5a[];
extern unsigned char DAT_009e967c[];
extern unsigned char DAT_009fd458[];
extern unsigned char DAT_009d8b88[];
extern unsigned char DAT_009d4d9e[];
extern unsigned short DAT_00a08292;
extern int DAT_009ecc34;

void FUN_00451bf0(int param_1, void *param_2);
void FUN_00450650(int param_1, int param_2);
void *FUN_0046f720(int param_1);

void __cdecl FUN_00481420(int param_1)
{
    int iVar2;
    unsigned char bVar4;
    unsigned char cVar1;
    void *puVar5;
    unsigned char *puVar6;
    int bVar3;
    int bVar7;
    int local_8;
    unsigned short local_a;
    unsigned short local_b;

    switch (DAT_009d6930) {
    case 8:
        iVar2 = (int)((unsigned int)param_1 * 0x5bdU);
        if (DAT_009e9a59[iVar2] != 0) {
            bVar4 = (unsigned char)(DAT_009e9a59[iVar2] - 1);
            DAT_009e9a59[iVar2] = bVar4;
            DAT_009e9a27[iVar2 + (unsigned int)bVar4] = 0x5f;
            DAT_009e9a27[1 + iVar2 + (unsigned int)DAT_009e9a59[iVar2]] = 0;
        }
        FUN_00451bf0(0, &DAT_009e9a27[iVar2]);
        FUN_00450650((int)DAT_009e9a5a[iVar2] + 0x176, param_1);
        return;
    case 9:
        iVar2 = (int)((unsigned int)param_1 * 0x5bdU);
        cVar1 = DAT_009e9a5a[iVar2];
        DAT_009e9a5a[iVar2] = (unsigned char)(cVar1 + 1U);
        if (3 < (unsigned char)(cVar1 + 1U)) {
            DAT_009e9a5a[iVar2] = 1;
        }
        FUN_00451bf0(0, &DAT_009e9a27[iVar2]);
        FUN_00450650((int)DAT_009e9a5a[iVar2] + 0x176, param_1);
        return;
    default:
        if (DAT_009d6930 != 0) {
            iVar2 = (int)((unsigned int)param_1 * 0x5bdU);
            if (DAT_009e9a59[iVar2] < 0x30) {
                DAT_009e9a27[iVar2 + (unsigned int)DAT_009e9a59[iVar2]] = (unsigned char)DAT_009d6930;
                DAT_009e9a27[1 + iVar2 + (unsigned int)DAT_009e9a59[iVar2]] = 0x5f;
                DAT_009e9a27[2 + iVar2 + (unsigned int)DAT_009e9a59[iVar2]] = 0;
                DAT_009e9a59[iVar2] = (unsigned char)(DAT_009e9a59[iVar2] + 1);
            }
            FUN_00451bf0(0, &DAT_009e9a27[iVar2]);
            FUN_00450650((int)DAT_009e9a5a[iVar2] + 0x176, param_1);
            return;
        }
        if (param_1 == DAT_009ecc34 &&
            *(short *)&DAT_009d8b88[(unsigned int)param_1 * 0x10U] < 0xec) {
            FUN_00451bf0(0, &DAT_009e9a27[(int)((unsigned int)param_1 * 0x5bdU)]);
            FUN_00450650((int)DAT_009e9a5a[(int)((unsigned int)param_1 * 0x5bdU)] + 0x176, param_1);
            return;
        }
        break;
    case 0xd:
        iVar2 = (int)((unsigned int)param_1 * 0x5bdU);
        DAT_009e9a27[iVar2 + (unsigned int)DAT_009e9a59[iVar2]] = 0;
        puVar5 = FUN_0046f720(param_1);
        FUN_00451bf0(0, puVar5);
        puVar6 = DAT_009e967c;
        FUN_00451bf0(1, &DAT_009e9a27[iVar2]);
        local_8 = 0;
        do {
            if (puVar6[4] != 0) {
                bVar3 = 0;
                DAT_00a08292 = 3;
                if (DAT_009e9a5a[iVar2] == 1) {
                    __builtin_memcpy(&local_a, &DAT_009e967c[iVar2], 2);
                    __builtin_memcpy(&local_b, puVar6, 2);
                    if (local_a == local_b) goto LAB_004816d5;
                } else {
                    if (DAT_009e9a5a[iVar2] == 2) {
                        __builtin_memcpy(&local_a, &DAT_009e967c[iVar2], 2);
                        __builtin_memcpy(&local_b, puVar6, 2);
                        if (local_b == local_a) {
                            bVar7 = 0;
                        } else {
                            bVar7 = DAT_009fd458[(unsigned int)local_a + (unsigned int)local_b * 0x1e5U] == 0;
                        }
                        if (!bVar7) goto LAB_004816da;
                    }
LAB_004816d5:
                    bVar3 = 1;
                }
LAB_004816da:
                if (bVar3) {
                    FUN_00450650(0x176, local_8);
                }
            }
            puVar6 = puVar6 + 0x5bd;
            local_8 = local_8 + 1;
            if (&DAT_009e967c[0x2de8 - 1] < puVar6) {
                DAT_009e9a5a[iVar2] = 0;
                FUN_00450650(0x17a, param_1);
                return;
            }
        } while (1);
    case 0x1b:
        DAT_009e9a5a[(int)((unsigned int)param_1 * 0x5bdU)] = 0;
        FUN_00450650(0x17b, param_1);
        return;
    case 0x9b:
    case 0x9c:
    case 0x9d:
    case 0x9e:
        iVar2 = (int)((unsigned int)param_1 * 0x5bdU);
        DAT_009e9a27[iVar2 + (unsigned int)DAT_009e9a59[iVar2]] = 0;
        puVar5 = FUN_0046f720(param_1);
        FUN_00451bf0(0, puVar5);
        puVar6 = DAT_009e967c;
        FUN_00451bf0(1, &DAT_009d4d9e[((unsigned int)DAT_009d6930 + (unsigned int)param_1 * 4U) * 0x46U]);
        local_8 = 0;
        do {
            if (puVar6[4] != 0) {
                bVar3 = 0;
                DAT_00a08292 = 3;
                if (DAT_009e9a5a[iVar2] == 1) {
                    __builtin_memcpy(&local_a, &DAT_009e967c[iVar2], 2);
                    __builtin_memcpy(&local_b, puVar6, 2);
                    if (local_a == local_b) goto LAB_00481839;
                } else {
                    if (DAT_009e9a5a[iVar2] == 2) {
                        __builtin_memcpy(&local_a, &DAT_009e967c[iVar2], 2);
                        __builtin_memcpy(&local_b, puVar6, 2);
                        if (local_b == local_a) {
                            bVar7 = 0;
                        } else {
                            bVar7 = DAT_009fd458[(unsigned int)local_a + (unsigned int)local_b * 0x1e5U] == 0;
                        }
                        if (!bVar7) goto LAB_0048183e;
                    }
LAB_00481839:
                    bVar3 = 1;
                }
LAB_0048183e:
                if (bVar3) {
                    FUN_00450650(0x176, local_8);
                }
            }
            puVar6 = puVar6 + 0x5bd;
            local_8 = local_8 + 1;
        } while (puVar6 < &DAT_009e967c[0x2de8]);
        DAT_009e9a5a[iVar2] = 0;
        FUN_00450650(0x17a, param_1);
    }
    return;
}
