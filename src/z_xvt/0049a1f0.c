/* SPDX-License-Identifier: MIT */
/* z_xvt 0049a1f0 */
extern int DAT_00703c88;
extern int DAT_00732a38;
extern int DAT_006db7a4;
extern int DAT_00703ac0;

int __cdecl FUN_0049a1f0(int *param_1, char *param_2)
{
    unsigned char *pbVar1;
    unsigned char bVar2;
    int iVar3;
    int iVar4;
    char *pcVar5;
    int local_cb;
    int local_199;
    int local_cf;

    iVar3 = 0;
    *param_1 = DAT_00703c88 + DAT_00732a38;
    pbVar1 = (unsigned char *)(param_1 + 1);
    iVar4 = 0;
    if (0 < DAT_00703c88 + DAT_00732a38) {
        do {
            if ((iVar4 < DAT_006db7a4) || (pcVar5 = param_2, DAT_00703ac0 <= iVar4)) {
                bVar2 = 0;
                pcVar5 = param_2 + 1;
                if (*param_2 != '\0') {
                    bVar2 = 1;
                    pcVar5 = param_2 + 0x24;
                    if (*(int *)(param_2 + 0x20) != 0) {
                        bVar2 = 3;
                        pcVar5 = param_2 + 0xd3;
                        __builtin_memcpy(&local_cb, param_2 + 0xcb, 4);
                        if (local_cb != 0) {
                            bVar2 = 7;
                            pcVar5 = param_2 + 0x509;
                        }
                        __builtin_memcpy(&local_199, param_2 + 199, 4);
                        if (local_199 != 0) {
                            bVar2 = bVar2 | 8;
                            pcVar5 = pcVar5 + 8;
                        }
                        __builtin_memcpy(&local_cf, param_2 + 0xcf, 4);
                        if (local_cf != 0) {
                            bVar2 = bVar2 | 0x10;
                            pcVar5 = pcVar5 + 0x4c;
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
                }
                else {
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
        } while (iVar4 < DAT_00703c88 + DAT_00732a38);
    }
    if (iVar3 != 0) {
        *pbVar1 = (unsigned char)iVar3 | 0x80;
        pbVar1 = pbVar1 + 1;
    }
    return (int)pbVar1 - (int)param_1;
}
