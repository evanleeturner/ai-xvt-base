/* SPDX-License-Identifier: MIT */
/* z_bop 004f8620 */
extern int DAT_0052dc84;
extern unsigned char *PTR_DAT_0052da78;
extern int DAT_00b6a2c0;

void FUN_004d6480(undefined4 *param_1, undefined4 *param_2);
int  __isctype(unsigned int c, int mask);
int  FUN_004dbfa0(char *param_1, int param_2);
void FUN_004d64a0(int *param_1, int param_2, int param_3);
void FUN_004db640(int param_1, char *param_2, int *param_3, int param_4);
unsigned int FUN_004db7d0(int param_1, char *param_2, int *param_3, int param_4, int param_5, unsigned int param_6);

void __cdecl FUN_004f8620(undefined4 *param_1, int param_2, short param_3)
{
    unsigned char bVar1;
    int bVar2;
    int bVar3;
    int bVar4;
    int iVar5;
    unsigned char *pbVar6;
    unsigned int uVar7;
    short sVar8;
    unsigned short uVar9;
    short sVar10;
    short sVar11;
    short sVar12;
    short local_15c;
    short local_15a;
    int local_150[4];
    unsigned char local_140[320];

    FUN_004d6480((undefined4 *)local_150, param_1);
    local_150[3] = local_150[1] + 0xd;
    sVar10 = -1;
    sVar12 = 0;
    sVar11 = 0;
    local_140[0] = 0;
    local_15a = -1;
    local_15c = 0;
    bVar4 = 0;
    do {
        iVar5 = (int)sVar12;
        pbVar6 = (unsigned char *)(iVar5 + param_2);
        bVar1 = *pbVar6;
        if ((bVar1 == 0x24) || (bVar1 == 0)) {
            local_15a = sVar12;
            if ((param_3 == 0) && (bVar1 == 0x24)) {
                local_15a = (short)(sVar12 - 1);
            }
            sVar12 = (short)(sVar12 + 1);
            sVar8 = sVar12;
            sVar10 = sVar11;
            if (*(char *)(sVar12 + param_2) == '\0') {
                bVar4 = 1;
            }
        }
        else {
            if (bVar1 == 0x20) {
                local_15c = sVar12;
            }
            if (DAT_0052dc84 < 2) {
                uVar7 = *(unsigned short *)(PTR_DAT_0052da78 + (unsigned int)bVar1 * 2) & 8;
            }
            else {
                uVar7 = __isctype((unsigned int)bVar1, 8);
            }
            if (uVar7 == 0) {
                while (1) {
                    if (DAT_0052dc84 < 2) {
                        uVar7 = *(unsigned short *)(PTR_DAT_0052da78 + (unsigned int)*(unsigned char *)(sVar12 + param_2) * 2) & 8;
                    }
                    else {
                        uVar7 = __isctype((unsigned int)*(unsigned char *)(sVar12 + param_2), 8);
                    }
                    if (uVar7 != 0) break;
                    iVar5 = (int)sVar12;
                    bVar1 = *(unsigned char *)(param_2 + iVar5);
                    if ((bVar1 == 0x24) || (bVar1 == 0)) break;
                    if (bVar1 == 0x5b) {
                        local_140[iVar5 - sVar11] = 2;
                    }
                    else if (bVar1 == 0x5d) {
                        local_140[iVar5 - sVar11] = 1;
                    }
                    else {
                        local_140[iVar5 - sVar11] = bVar1;
                    }
                    sVar12 = (short)(sVar12 + 1);
                }
            }
            else {
                sVar12 = (short)(sVar12 + 1);
                local_140[iVar5 - sVar11] = *pbVar6;
            }
            pbVar6 = local_140;
            iVar5 = 10;
            local_140[(int)sVar12 - (int)sVar11] = 0;
            iVar5 = FUN_004dbfa0((char *)pbVar6, iVar5);
            sVar8 = sVar11;
            if ((short)(*(short *)&local_150[2] - *(short *)&local_150[0]) <= (short)iVar5) {
                sVar12 = (short)(local_15c + 1);
                local_15a = local_15c;
                sVar8 = sVar12;
                sVar10 = sVar11;
            }
        }
        sVar11 = sVar8;
        bVar2 = 0;
        sVar8 = 0;
        bVar3 = 0;
        if (0 < sVar10) {
            do {
                bVar2 = bVar3;
                if (*(char *)(sVar8 + param_2) == '[') {
                    bVar2 = 1;
                }
                if (*(char *)(sVar8 + param_2) == ']') {
                    bVar2 = 0;
                }
                sVar8 = (short)(sVar8 + 1);
                bVar3 = bVar2;
            } while (sVar8 < sVar10);
        }
        if (sVar10 != -1) {
            if (bVar2) {
                local_140[0] = 2;
            }
            uVar9 = (unsigned short)bVar2;
            local_140[(short)uVar9] = 0;
            for (; sVar10 <= local_15a; sVar10 = (short)(sVar10 + 1)) {
                bVar1 = *(unsigned char *)(sVar10 + param_2);
                if (bVar1 == 0x5b) {
                    local_140[(short)uVar9] = 2;
                }
                else if (bVar1 == 0x5d) {
                    local_140[(short)uVar9] = 1;
                }
                else {
                    local_140[(short)uVar9] = bVar1;
                }
                uVar9 = (unsigned short)(uVar9 + 1);
            }
            local_140[(short)uVar9] = 0;
            if ((param_3 == 0) && (local_140[0] == 0x3e)) {
                FUN_004d64a0(local_150, 0, 1);
                FUN_004db640(10, (char *)(local_140 + 1), local_150, DAT_00b6a2c0);
                FUN_004d64a0(local_150, 0, -1);
            }
            else {
                FUN_004db7d0(10, (char *)local_140, local_150, 0, 1, 0xffff);
            }
            sVar10 = -1;
            local_15a = -1;
            FUN_004d64a0(local_150, 0, 0xe);
        }
        if (bVar4) {
            return;
        }
    } while (1);
}
