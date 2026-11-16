/* SPDX-License-Identifier: MIT */
/* z_xvt 004f3f10 */
/* Prototypes for every callee */
void         __cdecl FUN_004dd000(unsigned int *param_1, unsigned int *param_2);
unsigned int __cdecl __isctype(unsigned int c, int mask);
int          __cdecl FUN_004db2f0(char *param_1, int param_2);
void         __cdecl FUN_004dd030(int *param_1, int param_2, int param_3);
unsigned int __cdecl FUN_004da920(int param_1, char *param_2, int *param_3, unsigned int param_4);
int          __cdecl FUN_004daac0(int param_1, char *param_2, int *param_3, int param_4, int param_5, int param_6);

/* Globals */
extern unsigned char *PTR_DAT_005348a0;
extern int DAT_00534aac;
extern unsigned int DAT_0081bcd0;

void __cdecl FUN_004f3f10(unsigned int *param_1, int param_2, short param_3)
{
    unsigned char bVar1;
    char cVar2;
    _Bool bVar3;
    _Bool bVar4;
    _Bool bVar5;
    int iVar6;
    unsigned int uVar7;
    short sVar8;
    unsigned short uVar9;
    short sVar10;
    short sVar11;
    short sVar12;
    char *pcVar13;
    short local_98;
    short local_96;
    struct {
        short local_90[2];  /* +0x0 */
        int   local_8c;     /* +0x4 */
        short local_88;     /* +0x8 */
        int   local_84;     /* +0xc */
    } local_obj;
    char local_80[128];

    FUN_004dd000((unsigned int *)&local_obj, param_1);
    local_obj.local_84 = (int)((unsigned int)local_obj.local_8c + 0xdU);
    sVar10 = -1;
    sVar12 = 0;
    sVar11 = 0;
    local_80[0] = '\0';
    local_96 = -1;
    local_98 = 0;
    bVar5 = 0;
    do {
        iVar6 = (int)sVar12;
        bVar1 = *(unsigned char *)(iVar6 + param_2);
        if ((bVar1 == 0x24) || (bVar1 == 0)) {
            local_96 = sVar12;
            if ((param_3 == 0) && (bVar1 == 0x24)) {
                local_96 = (short)(sVar12 + -1);
            }
            sVar12 = (short)(sVar12 + 1);
            sVar8 = sVar12;
            sVar10 = sVar11;
            if (*(char *)(sVar12 + param_2) == '\0') {
                bVar5 = 1;
            }
        }
        else {
            if (bVar1 == 0x20) {
                local_98 = sVar12;
            }
            if (DAT_00534aac < 2) {
                uVar7 = *(unsigned short *)(PTR_DAT_005348a0 + (unsigned int)bVar1 * 2) & 8;
            }
            else {
                uVar7 = __isctype((unsigned int)bVar1, 8);
            }
            if (uVar7 == 0) {
                while (1) {
                    if (DAT_00534aac < 2) {
                        uVar7 = *(unsigned short *)(PTR_DAT_005348a0 + (unsigned int)*(unsigned char *)(sVar12 + param_2) * 2) & 8;
                    }
                    else {
                        uVar7 = __isctype((unsigned int)*(unsigned char *)(sVar12 + param_2), 8);
                    }
                    if (uVar7 != 0) break;
                    iVar6 = (int)sVar12;
                    cVar2 = *(char *)(iVar6 + param_2);
                    if ((cVar2 == '$') || (cVar2 == '\0')) break;
                    if (cVar2 == '[') {
                        local_80[iVar6 - sVar11] = '\x02';
                    }
                    else if (cVar2 == ']') {
                        local_80[iVar6 - sVar11] = '\x01';
                    }
                    else {
                        local_80[iVar6 - sVar11] = cVar2;
                    }
                    sVar12 = (short)(sVar12 + 1);
                }
            }
            else {
                sVar12 = (short)(sVar12 + 1);
                local_80[iVar6 - sVar11] = *(char *)(iVar6 + param_2);
            }
            pcVar13 = local_80;
            iVar6 = 0xc;
            local_80[(int)sVar12 - (int)sVar11] = '\0';
            iVar6 = FUN_004db2f0(pcVar13, iVar6);
            sVar8 = sVar11;
            if ((short)(local_obj.local_88 - local_obj.local_90[0]) <= (short)iVar6) {
                sVar12 = (short)(local_98 + 1);
                local_96 = local_98;
                sVar8 = sVar12;
                sVar10 = sVar11;
            }
        }
        sVar11 = sVar8;
        bVar3 = 0;
        sVar8 = 0;
        bVar4 = 0;
        if (0 < sVar10) {
            do {
                bVar3 = bVar4;
                if (*(char *)(sVar8 + param_2) == '[') {
                    bVar3 = 1;
                }
                if (*(char *)(sVar8 + param_2) == ']') {
                    bVar3 = 0;
                }
                sVar8 = (short)(sVar8 + 1);
                bVar4 = bVar3;
            } while (sVar8 < sVar10);
        }
        if (sVar10 != -1) {
            if (bVar3) {
                local_80[0] = '\x02';
            }
            uVar9 = (unsigned short)bVar3;
            local_80[(short)uVar9] = '\0';
            for (; sVar10 <= local_96; sVar10 = (short)(sVar10 + 1)) {
                cVar2 = *(char *)(sVar10 + param_2);
                if (cVar2 == '[') {
                    local_80[(short)uVar9] = '\x02';
                }
                else if (cVar2 == ']') {
                    local_80[(short)uVar9] = '\x01';
                }
                else {
                    local_80[(short)uVar9] = cVar2;
                }
                uVar9 = (unsigned short)(uVar9 + 1);
            }
            local_80[(short)uVar9] = '\0';
            if ((param_3 == 0) && (local_80[0] == '>')) {
                FUN_004dd030((int *)&local_obj, 0, 1);
                FUN_004da920(10, local_80 + 1, (int *)&local_obj, DAT_0081bcd0);
                FUN_004dd030((int *)&local_obj, 0, -1);
            }
            else {
                FUN_004daac0(10, local_80, (int *)&local_obj, 0, 1, 0xffff);
            }
            sVar10 = -1;
            local_96 = -1;
            FUN_004dd030((int *)&local_obj, 0, 0xe);
        }
        if (bVar5) {
            return;
        }
    } while (1);
}
