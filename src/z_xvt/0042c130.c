/* SPDX-License-Identifier: MIT */
/* z_xvt 0042c130 */
extern unsigned int DAT_005375d0[8];
extern unsigned int DAT_00536f48[8];
extern int DAT_005397f8;
extern int DAT_005373b8;
extern int DAT_005250d4;
extern int DAT_00536f68;
extern int _DAT_005371bc;
extern int DAT_005373c8;
extern int DAT_005250d8;
extern int DAT_0052554c;
extern unsigned int DAT_006db5a4;
extern int DAT_005373c4;
extern int _DAT_005371b8;
extern int DAT_005375f0;
extern int DAT_0072f444;
extern int DAT_00732a18;
extern int ram0x0071f2ec;
extern int DAT_0071b71c;
extern unsigned int DAT_006db5a8;
extern unsigned int _DAT_005373d0;
extern char *DAT_006a89c8;
extern char *DAT_006a89cc;
extern char *DAT_006a89d0;
extern char *DAT_006a89d4;
extern int DAT_00525568;
extern char DAT_0085e790;
extern int DAT_00525558;
extern int DAT_005250dc;
extern int DAT_005250e0;
extern int _DAT_005250e4;

int FUN_0048f530(void);
int FUN_004330b0(void);
int FUN_00432ca0(void);
void FUN_00431110(int, unsigned int *, int);
int FUN_00431d30(void);
int *FUN_00432b70(int *, unsigned int *, int);
void FUN_004310c0(unsigned int *, int);
void FUN_00438a40(void);
void FUN_00438c30(int, void *, int);
DWORD __stdcall timeGetTime(void);
int FUN_00432c10(int);
int *FUN_00432cc0(int);
void FUN_00438b70(void);
void FUN_0042c710(void);

unsigned int __stdcall FUN_0042c130(void)
{
    char cVar1;
    unsigned char bVar2;
    unsigned char bVar3;
    int *piVar4;
    DWORD DVar5;
    int *piVar6;
    int iVar7;
    unsigned int uVar8;
    unsigned int uVar9;
    unsigned char *pbVar10;
    unsigned int *puVar11;
    int *piVar12;
    char *pcVar13;
    char *pcVar14;
    unsigned char *pbVar15;
    DWORD local_110;
    int local_10c;
    int local_108;
    unsigned int local_104;
    unsigned char local_100[256];

    local_110 = 0;
    bVar3 = 1;
    puVar11 = DAT_005375d0;
    for (iVar7 = 8; iVar7 != 0; iVar7 = iVar7 + -1) {
        *puVar11 = 0;
        puVar11 = puVar11 + 1;
    }
    puVar11 = DAT_00536f48;
    for (iVar7 = 8; iVar7 != 0; iVar7 = iVar7 + -1) {
        *puVar11 = 0;
        puVar11 = puVar11 + 1;
    }
    DAT_005397f8 = 0;
    DAT_005373b8 = 0;
    DAT_005250d4 = 0;
    DAT_00536f68 = 0;
    _DAT_005371bc = 0;
    DAT_005373c8 = 0;
    DAT_005250d8 = 0;
    DAT_0052554c = 0;
    DAT_006db5a4 = 0;
    DAT_005373c4 = 5;
    _DAT_005371b8 = 0x16;
    DAT_005375f0 = 0x17;
    if (DAT_0072f444 == 1) {
        DAT_00732a18 = 0;
        ram0x0071f2ec = 0x1e;
        DAT_0071b71c = 0;
        DAT_006db5a8 = 0x1e;
        FUN_0048f530();
        return 1;
    }
    local_108 = FUN_004330b0();
    iVar7 = FUN_00432ca0();
    _DAT_005373d0 = 0x14;
    FUN_00431110(iVar7, &_DAT_005373d0, 4);
    iVar7 = FUN_00431d30();
    if (iVar7 != 0) {
        for (iVar7 = 0; iVar7 < local_108; iVar7 = iVar7 + 1) {
            do {
                piVar4 = FUN_00432b70(&local_10c, &local_104, 0x3c);
                if (piVar4 == (int *)0) {
                    return 0;
                }
            } while (*piVar4 != 0x14);
        }
        _DAT_005373d0 = 0x13;
        FUN_004310c0(&_DAT_005373d0, 4);
    }
    FUN_00438a40();
    FUN_00438c30(1, DAT_006a89c8, 0x30);
    do {
        piVar4 = FUN_00432b70(&local_10c, &local_104, 0x3c);
        if (piVar4 == (int *)0) {
            return 0;
        }
        if ((*piVar4 == 0x1d) && (DVar5 = timeGetTime(), 200 < (int)(DVar5 - local_110))) {
            iVar7 = FUN_00432c10(local_10c);
            piVar6 = FUN_00432cc0(iVar7);
            if (piVar6 == (int *)0) {
                uVar8 = 0xffffffff;
                pcVar13 = DAT_006a89cc;
                do {
                    pcVar14 = pcVar13;
                    if (uVar8 == 0) break;
                    uVar8 = uVar8 - 1;
                    pcVar14 = pcVar13 + 1;
                    cVar1 = *pcVar13;
                    pcVar13 = pcVar14;
                } while (cVar1 != '\0');
                uVar8 = ~uVar8;
                pbVar10 = (unsigned char *)(pcVar14 - uVar8);
                pbVar15 = local_100;
                for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
                    __builtin_memcpy(pbVar15, pbVar10, 4);
                    pbVar10 = pbVar10 + 4;
                    pbVar15 = pbVar15 + 4;
                }
            }
            else {
                uVar8 = 0xffffffff;
                do {
                    piVar12 = piVar6;
                    if (uVar8 == 0) break;
                    uVar8 = uVar8 - 1;
                    piVar12 = (int *)((unsigned int)piVar6 + 1);
                    iVar7 = *piVar6;
                    piVar6 = piVar12;
                } while ((char)iVar7 != '\0');
                uVar8 = ~uVar8;
                pbVar10 = (unsigned char *)((unsigned int)piVar12 - uVar8);
                pbVar15 = local_100;
                for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
                    __builtin_memcpy(pbVar15, pbVar10, 4);
                    pbVar10 = pbVar10 + 4;
                    pbVar15 = pbVar15 + 4;
                }
                for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
                    *pbVar15 = *pbVar10;
                    pbVar10 = pbVar10 + 1;
                    pbVar15 = pbVar15 + 1;
                }
                bVar3 = !bVar3;
                pcVar13 = DAT_006a89d4;
                if (bVar3) {
                    pcVar13 = DAT_006a89d0;
                }
                uVar8 = 0xffffffff;
                do {
                    pcVar14 = pcVar13;
                    if (uVar8 == 0) break;
                    uVar8 = uVar8 - 1;
                    pcVar14 = pcVar13 + 1;
                    cVar1 = *pcVar13;
                    pcVar13 = pcVar14;
                } while (cVar1 != '\0');
                uVar8 = ~uVar8;
                iVar7 = -1;
                pbVar10 = local_100;
                do {
                    pbVar15 = pbVar10;
                    if (iVar7 == 0) break;
                    iVar7 = iVar7 + -1;
                    pbVar15 = pbVar10 + 1;
                    bVar2 = *pbVar10;
                    pbVar10 = pbVar15;
                } while (bVar2 != 0);
                pbVar10 = (unsigned char *)(pcVar14 - uVar8);
                pbVar15 = pbVar15 - 1;
                for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
                    __builtin_memcpy(pbVar15, pbVar10, 4);
                    pbVar10 = pbVar10 + 4;
                    pbVar15 = pbVar15 + 4;
                }
            }
            for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
                *pbVar15 = *pbVar10;
                pbVar10 = pbVar10 + 1;
                pbVar15 = pbVar15 + 1;
            }
            FUN_00438c30(3, local_100, 0x30);
            local_110 = DVar5;
        }
        if (*piVar4 == 0x13) {
            FUN_00438b70();
            iVar7 = FUN_00432ca0();
            _DAT_005373d0 = 0x1b;
            FUN_00431110(iVar7, &_DAT_005373d0, 4);
            FUN_0048f530();
            DAT_00732a18 = 0;
            DAT_0071b71c = 0;
            DAT_006db5a8 = 0;
            if (DAT_00525568 == 0) {
                ram0x0071f2ec = 0x1e;
            }
            else {
                ram0x0071f2ec = 0x82;
            }
            if (DAT_0085e790 == '\x04') {
                DAT_00525558 = 0x3b;
            }
            else if (DAT_0085e790 == '\x06') {
                DAT_00525558 = 0x27;
            }
            else {
                DAT_00525558 = 0x1d;
            }
            iVar7 = FUN_00431d30();
            if (iVar7 != 0) {
                DAT_005250d8 = 1;
                if (local_108 != 1) {
                    DAT_005250d8 = 2;
                }
                DAT_005250dc = 0;
                DAT_005250e0 = 0;
                _DAT_005250e4 = 1;
                for (; (DAT_005250d8 != 0 && (DAT_006db5a8 < 100)); DAT_006db5a8 = DAT_006db5a8 + (unsigned int)iVar7) {
                    FUN_0042c710();
                    iVar7 = FUN_0048f530();
                }
                DAT_005250d8 = 0;
                iVar7 = FUN_0048f530();
                DAT_006db5a8 = DAT_006db5a8 + (unsigned int)iVar7;
                ram0x0071f2ec = (int)DAT_006db5a8;
                if ((int)DAT_006db5a8 < 0x23) {
                    iVar7 = (int)(0x23U - DAT_006db5a8);
                    DAT_006db5a8 = DAT_006db5a8 + (unsigned int)iVar7;
                    DAT_0052554c = (int)((unsigned int)DAT_0052554c - (unsigned int)iVar7);
                }
            }
            return 1;
        }
    } while (1);
}
