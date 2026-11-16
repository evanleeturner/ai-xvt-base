/* SPDX-License-Identifier: MIT */
/* z_bop 0041c1d0 */
extern unsigned char *DAT_009a1fe8;
extern unsigned char DAT_00521f1c[];
extern int DAT_009a1fd4;
extern int DAT_009a1fd8;
extern int DAT_009a1fd0;
extern int DAT_00a07bdc;
extern int DAT_00a07c54;
extern int DAT_00a07c58;
extern int DAT_009a7398;
extern int DAT_009ec604;
extern int DAT_00a081e8;
extern int DAT_00a08298;
extern int DAT_009d80c0;
extern int DAT_009d8c18;
extern int DAT_009d1150;
extern int DAT_00523428;
extern unsigned char DAT_0051a44f[];
extern int DAT_0051bf58;

unsigned int __cdecl FUN_0041f3d0(unsigned int a, unsigned int b, unsigned int c);
unsigned int __cdecl FUN_004a5490(unsigned short a, unsigned short b);
unsigned int __cdecl FUN_0041c570(int a);

unsigned int __cdecl FUN_0041c1d0(unsigned short param_1, unsigned short param_2)
{
    char cVar1;
    int iVar2;
    int iVar3;
    int bVar4;
    unsigned int uVar5;
    unsigned short uVar9;
    int iVar6;
    int iVar7;
    int iVar8;
    unsigned int uVar10;
    int iVar11;
    unsigned int uVar12;
    int iVar13;
    int iVar14;
    int iVar15;
    int iVar16;
    int iVar17;
    unsigned int local_20;
    int tmp_a;
    int tmp_b;
    unsigned short tmp_s;

    bVar4 = 1;
    iVar3 = (int)((unsigned int)param_2 * 0x23u);
    __builtin_memcpy(&tmp_a, &DAT_00521f1c[(unsigned int)DAT_009a1fe8[3 + iVar3] * 0x1a], sizeof(tmp_a));
    __builtin_memcpy(&tmp_b, &DAT_00521f1c[(unsigned int)DAT_009a1fe8[3 + (unsigned int)param_1 * 0x23u] * 0x1a], sizeof(tmp_b));
    iVar11 = (int)((unsigned int)tmp_a + 0x20000u + (unsigned int)tmp_b);
    uVar5 = (unsigned int)DAT_009a1fd4 - (unsigned int)DAT_00a07bdc;
    DAT_009a7398 = iVar11;
    if ((int)uVar5 < 0) {
        uVar5 = 0u - uVar5;
    }
    if (iVar11 < (int)uVar5) {
        return uVar5 & 0xffff0000u;
    }
    uVar10 = (unsigned int)DAT_009a1fd8 - (unsigned int)DAT_00a07c54;
    if ((int)uVar10 < 0) {
        uVar10 = 0u - uVar10;
    }
    uVar9 = (unsigned short)(uVar5 >> 0x10);
    if (iVar11 < (int)uVar10) {
        return (unsigned int)uVar9 << 0x10;
    }
    uVar12 = (unsigned int)DAT_009a1fd0 - (unsigned int)DAT_00a07c58;
    if ((int)uVar12 < 0) {
        uVar12 = 0u - uVar12;
    }
    if (iVar11 < (int)uVar12) {
        return (unsigned int)uVar9 << 0x10;
    }
    uVar5 = FUN_0041f3d0(uVar5, uVar10, uVar12);
    DAT_009a7398 = (int)uVar5;
    if (iVar11 < (int)uVar5) {
        return uVar5 & 0xffff0000u;
    }
    iVar11 = (int)((unsigned int)DAT_009a1fd4 - (unsigned int)DAT_009ec604);
    if (iVar11 < 0) {
        iVar11 = (int)(0u - (unsigned int)iVar11);
    }
    iVar17 = (int)((unsigned int)DAT_009a1fd8 - (unsigned int)DAT_00a081e8);
    if (iVar17 < 0) {
        iVar17 = (int)(0u - (unsigned int)iVar17);
    }
    iVar15 = (int)((unsigned int)DAT_009a1fd0 - (unsigned int)DAT_00a08298);
    if (iVar15 < 0) {
        iVar15 = (int)(0u - (unsigned int)iVar15);
    }
    iVar6 = (int)((unsigned int)DAT_00a07bdc - (unsigned int)DAT_009d80c0);
    if (iVar6 < 0) {
        iVar6 = (int)(0u - (unsigned int)iVar6);
    }
    iVar7 = (int)((unsigned int)DAT_00a07c54 - (unsigned int)DAT_009d8c18);
    if (iVar7 < 0) {
        iVar7 = (int)(0u - (unsigned int)iVar7);
    }
    iVar8 = (int)((unsigned int)DAT_00a07c58 - (unsigned int)DAT_009d1150);
    if (iVar8 < 0) {
        iVar8 = (int)(0u - (unsigned int)iVar8);
    }
    iVar13 = (int)((unsigned int)iVar3 + (unsigned int)(int)DAT_009a1fe8);
    __builtin_memcpy(&iVar16, &DAT_00521f1c[(unsigned int)((unsigned char *)iVar13)[3] * 0x1a], sizeof(iVar16));
    if ((((unsigned char *)iVar13)[2] == 7) || (((unsigned char *)iVar13)[2] == 6)) {
        if (((unsigned char *)iVar13)[3] != 0x9b) {
            iVar16 = iVar16 >> 1;
        }
    }
    else if (((unsigned char *)iVar13)[2] == 0) {
        iVar14 = (int)((unsigned int)param_1 * 0x23u + (unsigned int)(int)DAT_009a1fe8);
        cVar1 = (char)((unsigned char *)iVar14)[2];
        if ((cVar1 == 7) || (cVar1 == 6)) {
            __builtin_memcpy(&iVar2, (unsigned char *)iVar13 + 0x1f, sizeof(iVar2));
            __builtin_memcpy(&iVar2, (unsigned char *)iVar2 + 0xa9, sizeof(iVar2));
            __builtin_memcpy(&tmp_a, (unsigned char *)iVar2 + 0x215, sizeof(tmp_a));
            __builtin_memcpy(&tmp_b, (unsigned char *)iVar2 + 0x211, sizeof(tmp_b));
            if ((int)((unsigned int)tmp_a + (unsigned int)tmp_b) != 0) {
                iVar16 = (int)((unsigned int)iVar16 + (unsigned int)(iVar16 >> 1));
                if (0x447 < iVar16) {
                    iVar16 = 0x446;
                }
            }
            if (DAT_00523428 != 0) {
                __builtin_memcpy(&tmp_a, (unsigned char *)iVar13 + 0x1b, sizeof(tmp_a));
                if ((tmp_a != -1) && (cVar1 == 6)) {
                    local_20 = (unsigned int)((unsigned char *)iVar14)[3];
                    if (DAT_0051a44f[local_20] == 0) {
                        switch ((unsigned int)((unsigned char *)iVar13)[3]) {
                        case 2:
                        case 4:
                            break;
                        case 3:
                        case 6:
                        case 8:
                            iVar16 = (int)((unsigned int)iVar16 * 2u);
                            break;
                        case 5:
                            iVar16 = (int)((unsigned int)iVar16 + (unsigned int)(iVar16 / 2) +
                                           (unsigned int)((int)((unsigned int)iVar16 + ((unsigned int)(iVar16 >> 0x1f) & 3u)) >> 2));
                            break;
                        default:
                            iVar16 = (int)((unsigned int)iVar16 + (unsigned int)(iVar16 / 2));
                        }
                        bVar4 = 0;
                    }
                }
            }
        }
    }
    uVar10 = FUN_0041f3d0((unsigned int)iVar11 + (unsigned int)iVar6 + (unsigned int)iVar16,
                          (unsigned int)iVar17 + (unsigned int)iVar7 + (unsigned int)iVar16,
                          (unsigned int)iVar15 + (unsigned int)iVar8 + (unsigned int)iVar16);
    if (uVar5 <= uVar10) {
        if ((bVar4 && (0x446 < iVar16)) || (DAT_009a1fe8[3 + iVar3] == 0x3a)) {
            if (DAT_0051bf58 == 0) {
                if ((int)((unsigned int)iVar17 + (unsigned int)iVar15 + (unsigned int)iVar11) == 0) {
                    return 0;
                }
                uVar5 = FUN_004a5490(param_1, param_2);
                return uVar5;
            }
            DAT_0051bf58 = 0;
            __builtin_memcpy(&iVar2, (unsigned char *)iVar13 + 0x1f, sizeof(iVar2));
            __builtin_memcpy(&tmp_s, (unsigned char *)iVar2 + 0x79, sizeof(tmp_s));
            if (tmp_s < 0x28) {
                uVar5 = FUN_004a5490(param_1, param_2);
                return uVar5;
            }
        }
        uVar5 = FUN_0041c570((int)((unsigned int)(iVar16 >> 3) + (unsigned int)(iVar16 >> 2)));
        return uVar5;
    }
    return uVar10 & 0xffff0000u;
}
