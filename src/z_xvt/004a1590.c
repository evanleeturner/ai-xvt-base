/* SPDX-License-Identifier: MIT */
/* z_xvt 004a1590 */
extern unsigned char DAT_0071f2db;
extern unsigned char DAT_0071f2dc;
extern unsigned char DAT_0071f2dd;
extern char DAT_0070aee5[];
extern char DAT_0070aee9[];
extern int DAT_00709310[];
extern char DAT_0071b79d[];
extern int DAT_0071ecf4;
extern short DAT_00732950;
extern void FUN_004a1d70(int, int);

int __cdecl FUN_004a1590(unsigned short param_1, unsigned int param_2, int param_3, unsigned short param_4, int param_5, int param_6)
{
    unsigned int uVar2;
    unsigned int uVar3;
    unsigned int uVar4;
    unsigned int uVar5;
    int iVar6;
    int iVar7;
    int iVar8;
    char *pcVar9;
    int local_10;

    uVar4 = DAT_0071f2db;
    uVar2 = DAT_0071f2dc;
    local_10 = 0;
    uVar5 = param_2 & 0xffff;
    uVar3 = DAT_0071f2dd;
    iVar7 = 0;
    pcVar9 = DAT_0070aee5 + uVar5 * 0x566;
    do {
        if (((*pcVar9 == 0x02) &&
            (((((unsigned char)pcVar9[1] == param_1 && pcVar9[2] == 0x12) &&
              (DAT_0070aee9[iVar7 + param_6 + uVar5 * 0x566] != 0)) ||
             (((*pcVar9 == 0x02 && ((unsigned char)pcVar9[1] == param_1)) &&
              (pcVar9[2] == 0x13 &&
               (((DAT_0070aee9[iVar7 + param_6 + uVar5 * 0x566] != 0) && (param_5 == 1)))))))) &&
           (((unsigned int)(unsigned char)pcVar9[0xe] * 5 == 0) ||
            (uVar3 + (uVar4 * 0x3c + uVar2) * 0x3c <= (unsigned int)(unsigned char)pcVar9[0xe] * 5)))) {
            iVar8 = pcVar9[3] * 0xfa;
            if (10 < param_4) {
                param_4 = 9;
            }
            if (1 < param_4) {
                if (iVar8 < 1) {
                    iVar6 = param_4 - 1;
                }
                else {
                    iVar6 = 1 - (unsigned int)param_4;
                }
                iVar8 = iVar8 + iVar6 * (iVar8 / 10);
            }
            if (param_3 == -1) {
                DAT_00709310[param_6] = DAT_00709310[param_6] + iVar8;
            }
            else {
                int local_val;
                __builtin_memcpy(&local_val, (void*)((int)&DAT_0071b79d + param_3 * 0x5bd), sizeof(int));
                local_val = local_val + iVar8;
                __builtin_memcpy((void*)((int)&DAT_0071b79d + param_3 * 0x5bd), &local_val, sizeof(int));
            }
            local_10 = local_10 + iVar8;
        }
        pcVar9 = pcVar9 + 0x4e;
        iVar7 = iVar7 + 0x4e;
    } while (iVar7 < 0x270);
    if ((local_10 != 0) && (param_3 == DAT_0071ecf4)) {
        DAT_00732950 = (short)local_10;
        if (local_10 < 1) {
            DAT_00732950 = -DAT_00732950;
            iVar7 = 199;
        }
        else {
            iVar7 = 0xc6;
        }
        FUN_004a1d70(iVar7, param_3);
    }
    return local_10;
}
