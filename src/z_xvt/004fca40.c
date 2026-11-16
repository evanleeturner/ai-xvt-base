/* SPDX-License-Identifier: MIT */
/* z_xvt 004fca40 */
extern char DAT_00534360[];
extern char DAT_00654d68[];
extern unsigned int DAT_0081bcd0;
extern unsigned char DAT_0081d402[];   /* packed uint */
extern unsigned char DAT_0081d40a[];   /* packed int */
extern unsigned char DAT_0081d40e[];   /* packed int[256] */
extern unsigned char DAT_0081fb96[];   /* packed int */
extern unsigned char DAT_0081fc4e[];   /* packed int, stride 0xe824 per DAT_0081fb96 */
extern char DAT_0081b740[];
extern char DAT_005319c4[];
extern char s__c_s___c_s_005319cc[];
extern char s__s__s____s_00534354[];
extern char s_medlvl_d_005321f8[];
extern char s__c_s__c_d_005333b4[];
extern char s__c_s__c_d__c_s__c_d_0053433c[];
extern int DAT_00757cc0;

unsigned char FUN_004dbe00(int);
void FUN_004d0e60(char *, int, int);
void FUN_004dcfd0(int *, int, int, int, int);
char *FUN_004dcfa0(int);
void FUN_004da920(int, char *, int *, unsigned int);
void FUN_004dd030(void *, int, int);
int _sprintf(char *, const char *, ...);
int FUN_004db2f0(char *, int);
void FUN_004d0500(char *, int *);
void FUN_004da7c0(int, char *, int, int, unsigned int);

unsigned int __stdcall FUN_004fca40(void)
{
    /* contiguous frame: local_54 @ -0x54, local_50 @ [1], local_4c @ [2],
       local_44 @ [4], local_3c @ [6], local_34 (52 bytes) @ [8] */
    int local_54[21];
    unsigned char bVar1;
    char *pcVar2;
    char *pcVar3;
    char *pcVar4;
    int iVar5;
    int iVar6;
    int iVar7;
    int iVar9;
    unsigned char *pbVar8;
    unsigned int uVar10;
    unsigned int uVar11;
    unsigned int uVar12;
    unsigned int uVar13;
    int tmpA;
    int tmpIdx;
    int tmpVal;
    unsigned int tmpU;
    unsigned char *local_34;

    local_34 = (unsigned char *)&local_54[8];

    iVar7 = 0;
    iVar9 = 0;
    bVar1 = FUN_004dbe00(0x12);
    if (((bVar1 != 0) &&
         (bVar1 = FUN_004dbe00(0x10), bVar1 != 0)) &&
        (bVar1 = FUN_004dbe00(0x78), bVar1 != 0)) {
        FUN_004d0e60(DAT_00534360, 0xaf, 0x156);
    }
    FUN_004dcfd0(&local_54[0], 0x54, 0x5a, 0x1b2, 0x6a);
    uVar13 = 0xffff;
    pcVar2 = FUN_004dcfa0(0x12d);
    FUN_004da920(0xf, pcVar2, &local_54[0], uVar13);
    FUN_004dcfd0(&local_54[0], 0x54, 0x6e, 0x1b2, 0x73);
    FUN_004da920(0xc, DAT_00654d68, &local_54[0], DAT_0081bcd0);
    FUN_004dd030(&local_54[0], 0, 0x14);
    uVar13 = 0;
    do {
        if (0xff < uVar13) break;
        __builtin_memcpy(&tmpA, DAT_0081d40e + uVar13 * 4u, 4);
        if (tmpA == 0) {
            iVar7 = iVar7 + 1;
        }
        else {
            __builtin_memcpy(&tmpA, DAT_0081d40e + uVar13 * 4u, 4);
            if (tmpA == 1) {
                iVar9 = iVar9 + 1;
            }
        }
        uVar13 = uVar13 + 1;
        __builtin_memcpy(&tmpU, DAT_0081d402, 4);
    } while (uVar13 <= tmpU);

    __builtin_memcpy(&tmpA, DAT_0081d40a, 4);
    if (tmpA == iVar7) {
        pcVar2 = FUN_004dcfa0(0x14c);
        uVar11 = 5;
        pcVar3 = FUN_004dcfa0(0x2b7);
        _sprintf(DAT_0081b740, s__c_s___c_s_005319cc, 4, pcVar3, uVar11, pcVar2);
        FUN_004da920(0xc, DAT_0081b740, &local_54[0], 0xffff);
        __builtin_memcpy(&tmpIdx, DAT_0081fb96, 4);
        __builtin_memcpy(&tmpVal, DAT_0081fc4e + (unsigned int)tmpIdx * 0xe824u, 4);
        if (tmpVal != 0) {
            FUN_004dd030(&local_54[0], 0, 0xf);
            __builtin_memcpy(&tmpIdx, DAT_0081fb96, 4);
            __builtin_memcpy(&tmpVal, DAT_0081fc4e + (unsigned int)tmpIdx * 0xe824u, 4);
            pcVar2 = FUN_004dcfa0((int)((unsigned int)tmpVal + 0x170u));
            pcVar3 = FUN_004dcfa0(0x170);
            pcVar4 = FUN_004dcfa0(0x16c);
            _sprintf(DAT_0081b740, s__s__s____s_00534354, pcVar4, pcVar3, pcVar2);
            iVar5 = FUN_004db2f0(DAT_0081b740, 0xc);
            __builtin_memcpy(&tmpIdx, DAT_0081fb96, 4);
            __builtin_memcpy(&tmpVal, DAT_0081fc4e + (unsigned int)tmpIdx * 0xe824u, 4);
            _sprintf((char *)local_34, s_medlvl_d_005321f8, tmpVal);
            FUN_004d0500((char *)local_34, &local_54[4]);
            iVar6 = (int)(((unsigned int)((int)((unsigned int)local_54[2] - (unsigned int)local_54[0]) >> 1) -
                           (((((unsigned int)iVar5 + 10u) - (unsigned int)local_54[4]) +
                             (unsigned int)local_54[6] + 0xbU) >> 1)) +
                          (unsigned int)local_54[0]);
            FUN_004da7c0(0xc, DAT_0081b740, iVar6, local_54[1], 0xffff);
            FUN_004d0e60((char *)local_34,
                         (int)((unsigned int)iVar6 + (unsigned int)iVar5 + 10u + 10u),
                         local_54[1]);
        }
    }
    else {
        __builtin_memcpy(&tmpA, DAT_0081d40a, 4);
        if (tmpA == iVar9) {
            pcVar2 = FUN_004dcfa0(0x14d);
            uVar11 = 3;
            pcVar3 = FUN_004dcfa0(0x2b7);
            _sprintf(DAT_0081b740, s__c_s___c_s_005319cc, 4, pcVar3, uVar11, pcVar2);
            FUN_004da920(0xc, DAT_0081b740, &local_54[0], 0xffff);
            __builtin_memcpy(&tmpIdx, DAT_0081fb96, 4);
            __builtin_memcpy(&tmpVal, DAT_0081fc4e + (unsigned int)tmpIdx * 0xe824u, 4);
            if (tmpVal != 0) {
                FUN_004dd030(&local_54[0], 0, 0xf);
                __builtin_memcpy(&tmpIdx, DAT_0081fb96, 4);
                __builtin_memcpy(&tmpVal, DAT_0081fc4e + (unsigned int)tmpIdx * 0xe824u, 4);
                pcVar2 = FUN_004dcfa0((int)((unsigned int)tmpVal + 0x170u));
                pcVar3 = FUN_004dcfa0(0x170);
                pcVar4 = FUN_004dcfa0(0x16c);
                _sprintf(DAT_0081b740, s__s__s____s_00534354, pcVar4, pcVar3, pcVar2);
                iVar5 = FUN_004db2f0(DAT_0081b740, 0xc);
                __builtin_memcpy(&tmpIdx, DAT_0081fb96, 4);
                __builtin_memcpy(&tmpVal, DAT_0081fc4e + (unsigned int)tmpIdx * 0xe824u, 4);
                _sprintf((char *)local_34, s_medlvl_d_005321f8, tmpVal);
                FUN_004d0500((char *)local_34, &local_54[4]);
                iVar6 = (int)(((unsigned int)((int)((unsigned int)local_54[2] - (unsigned int)local_54[0]) >> 1) -
                               (((((unsigned int)iVar5 + 10u) - (unsigned int)local_54[4]) +
                                 (unsigned int)local_54[6] + 0xbU) >> 1)) +
                              (unsigned int)local_54[0]);
                FUN_004da7c0(0xc, DAT_0081b740, iVar6, local_54[1], 0xffff);
                FUN_004d0e60((char *)local_34,
                             (int)((unsigned int)iVar6 + (unsigned int)iVar5 + 10u + 10u),
                             local_54[1]);
            }
        }
        else {
            uVar11 = 1;
            __builtin_memcpy(&iVar5, DAT_0081d40a, 4);
            pcVar2 = FUN_004dcfa0(0x148);
            _sprintf(DAT_0081b740, s__c_s__c_d_005333b4, 4, pcVar2, uVar11, iVar5);
            FUN_004da920(0xc, DAT_0081b740, &local_54[0], 0xffff);
        }
    }
    FUN_004dd030(&local_54[0], 0, 0x14);
    uVar12 = 1;
    pcVar2 = FUN_004dcfa0(0x14a);
    uVar10 = 3;
    uVar11 = 1;
    pcVar3 = FUN_004dcfa0(0x149);
    _sprintf(DAT_0081b740, s__c_s__c_d__c_s__c_d_0053433c, 5, pcVar3, uVar11, iVar7, uVar10, pcVar2,
             uVar12, iVar9);
    FUN_004da920(0xc, DAT_0081b740, &local_54[0], 0xffff);
    uVar13 = 0;
    FUN_004dd030(&local_54[0], 0, 0x14);
    pbVar8 = DAT_0081d40e;
    do {
        if ((unsigned char *)0x81d80d < pbVar8) {
            return 1;
        }
        __builtin_memcpy(&iVar7, pbVar8, 4);
        if (iVar7 == 0) {
            local_34[0] = 5;
        }
        else if (iVar7 == 1) {
            local_34[0] = 3;
        }
        else if (iVar7 == 2) {
            local_34[0] = 1;
        }
        uVar13 = uVar13 + 1;
        __builtin_memcpy(&tmpA, pbVar8 + 0x800, 4);
        _sprintf(DAT_0081b740, DAT_005319c4, 4,
                 (int)((unsigned int)tmpA * 0x144u + (unsigned int)DAT_00757cc0 + 0x40u));
        FUN_004da920(0xc, DAT_0081b740, &local_54[0], 0xffff);
        FUN_004dd030(&local_54[0], 0, 0xf);
        __builtin_memcpy(&tmpA, pbVar8, 4);
        pcVar2 = FUN_004dcfa0((int)((unsigned int)tmpA + 0x14cu));
        _sprintf(DAT_0081b740, DAT_005319c4, (unsigned int)local_34[0], pcVar2);
        FUN_004da920(0xc, DAT_0081b740, &local_54[0], 0xffff);
        FUN_004dd030(&local_54[0], 0, 0xf);
        pbVar8 = pbVar8 + 4;
        __builtin_memcpy(&tmpU, DAT_0081d402, 4);
    } while (uVar13 <= tmpU);
    return 1;
}
