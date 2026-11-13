/* SPDX-License-Identifier: MIT */
/* z_bop 00417960 */
extern int DAT_009d1308;
extern int DAT_00a07ce8;
extern int DAT_009a8e24;
extern int DAT_009cd274;

int __cdecl FUN_00417960(int *param_1, char *param_2)
{
    unsigned char *pbVar1;
    unsigned char bVar2;
    int iVar3;
    int iVar4;
    char *pcVar5;

    iVar3 = 0;
    *param_1 = DAT_009d1308 + DAT_00a07ce8;
    pbVar1 = (unsigned char *)(param_1 + 1);
    iVar4 = 0;
    if (0 < DAT_009d1308 + DAT_00a07ce8) {
        do {
            if (iVar4 < DAT_009a8e24 || (pcVar5 = param_2, DAT_009cd274 <= iVar4)) {
                bVar2 = 0;
                pcVar5 = param_2 + 1;
                if (*param_2 != '\0') {
                    bVar2 = 1;
                    pcVar5 = param_2 + 0x24;
                    if (*(int *)(param_2 + 0x20) != 0) {
                        bVar2 = 3;
                        pcVar5 = param_2 + 0xd5;
                        {
                            int field_cd;
                            __builtin_memcpy(&field_cd, param_2 + 0xcd, 4);
                            if (field_cd != 0) {
                                bVar2 = 7;
                                pcVar5 = param_2 + 0x537;
                            }
                        }
                        {
                            int field_c9;
                            __builtin_memcpy(&field_c9, param_2 + 0xc9, 4);
                            if (field_c9 != 0) {
                                bVar2 = bVar2 | 8;
                                pcVar5 = pcVar5 + 10;
                            }
                        }
                        {
                            int field_d1;
                            __builtin_memcpy(&field_d1, param_2 + 0xd1, 4);
                            if (field_d1 != 0) {
                                bVar2 = bVar2 | 0x10;
                                pcVar5 = pcVar5 + 0x4c;
                            }
                        }
                    }
                }
                if (bVar2 == 0) {
                    iVar3 = iVar3 + 1;
                    if (0x7d < iVar3) {
                        *pbVar1 = (unsigned char)iVar3 | 0x80;
                        pbVar1 = pbVar1 + 1;
                        iVar3 = 0;
                    }
                } else {
                    if (iVar3 != 0) {
                        *pbVar1 = (unsigned char)iVar3 | 0x80;
                        pbVar1 = pbVar1 + 1;
                        iVar3 = 0;
                    }
                    *pbVar1 = bVar2;
                    pbVar1 = pbVar1 + 1;
                }
            }
            iVar4 = iVar4 + 1;
            param_2 = pcVar5;
        } while (iVar4 < DAT_009d1308 + DAT_00a07ce8);
    }
    if (iVar3 != 0) {
        *pbVar1 = (unsigned char)iVar3 | 0x80;
        pbVar1 = pbVar1 + 1;
    }
    return (int)pbVar1 - (int)param_1;
}
