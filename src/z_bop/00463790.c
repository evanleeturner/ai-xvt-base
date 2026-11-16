/* SPDX-License-Identifier: MIT */
/* z_bop 00463790 */
extern unsigned int DAT_00557560[8];
extern unsigned int DAT_00556ed8[8];
extern int DAT_00559788;
extern int DAT_00557348;
extern int DAT_005242dc;
extern int DAT_00556ef8;
extern int _DAT_0055714c;
extern int DAT_00557358;
extern unsigned int DAT_005242e0;
extern unsigned int DAT_0052340c;
extern int DAT_009a8c24;
extern int DAT_00557354;
extern int _DAT_00557148;
extern int DAT_00557580;
extern int DAT_009fd394;
extern int DAT_00a07cc8;
extern unsigned int _DAT_009ed23c;
extern int DAT_009e9658;
extern unsigned int DAT_009a8c2c;
extern unsigned int _DAT_00557360;
extern char *DAT_00a60a28;
extern char *DAT_00a60a2c;
extern char *DAT_00a60a30;
extern char *DAT_00a60a34;
extern int DAT_00523428;
extern char DAT_00bb7370;
extern int DAT_00523418;

int FUN_004ac760(void);
int FUN_0046fa30(void);
int FUN_0046f700(void);
void FUN_0046d350(int, unsigned int *, int);
int FUN_0046e040(void);
int *FUN_0046f5e0(int *, unsigned int *, int);
void FUN_0046d300(unsigned int *, int);
void FUN_00448da0(void);
void FUN_00448f90(int, void *, int);
DWORD __stdcall timeGetTime(void);
void *FUN_0046f660(int);
char *FUN_0046f720(void *);
void FUN_00448ed0(void);
void FUN_00464c60(void);
void FUN_00463d00(void);

unsigned int __stdcall FUN_00463790(void)
{
    char cVar1;
    unsigned char bVar2;
    int bVar3;
    int *piVar4;
    DWORD DVar5;
    void *puVar6;
    char *pcVar7;
    int iVar8;
    unsigned int uVar9;
    unsigned int uVar10;
    unsigned char *pbVar11;
    unsigned int *puVar12;
    char *pcVar13;
    unsigned char *pbVar14;
    DWORD local_110;
    int local_10c;
    int local_108;
    unsigned int local_104;
    unsigned char local_100[256];
    unsigned int tmp;

    local_110 = 0;
    bVar3 = 1;
    puVar12 = DAT_00557560;
    for (iVar8 = 8; iVar8 != 0; iVar8 = iVar8 + -1) {
        *puVar12 = 0;
        puVar12 = puVar12 + 1;
    }
    puVar12 = DAT_00556ed8;
    for (iVar8 = 8; iVar8 != 0; iVar8 = iVar8 + -1) {
        *puVar12 = 0;
        puVar12 = puVar12 + 1;
    }
    DAT_00559788 = 0;
    DAT_00557348 = 0;
    DAT_005242dc = 0;
    DAT_00556ef8 = 0;
    _DAT_0055714c = 0;
    DAT_00557358 = 0;
    DAT_005242e0 = 0;
    DAT_0052340c = 0;
    DAT_009a8c24 = 0;
    DAT_00557354 = 5;
    _DAT_00557148 = 0x16;
    DAT_00557580 = 0x17;
    if (DAT_009fd394 == 1) {
        DAT_00a07cc8 = 0;
        _DAT_009ed23c = 0x1e;
        DAT_009e9658 = 0;
        DAT_009a8c2c = 0x1e;
        FUN_004ac760();
        return 1;
    }
    local_108 = FUN_0046fa30();
    iVar8 = FUN_0046f700();
    _DAT_00557360 = 0x14;
    FUN_0046d350(iVar8, &_DAT_00557360, 4);
    iVar8 = FUN_0046e040();
    if (iVar8 != 0) {
        for (iVar8 = 0; iVar8 < local_108; iVar8 = iVar8 + 1) {
            do {
                piVar4 = FUN_0046f5e0(&local_10c, &local_104, 0x3c);
                if (piVar4 == (int *)0) {
                    return 0;
                }
            } while (*piVar4 != 0x14);
        }
        _DAT_00557360 = 0x13;
        FUN_0046d300(&_DAT_00557360, 4);
    }
    FUN_00448da0();
    FUN_00448f90(1, DAT_00a60a28, 0x30);
    do {
        piVar4 = FUN_0046f5e0(&local_10c, &local_104, 0x3c);
        if (piVar4 == (int *)0) {
            return 0;
        }
        if ((*piVar4 == 0x1d) && (DVar5 = timeGetTime(), 200 < (int)(DVar5 - local_110))) {
            puVar6 = FUN_0046f660(local_10c);
            pcVar7 = FUN_0046f720(puVar6);
            if (pcVar7 == (char *)0) {
                uVar9 = 0xffffffff;
                pcVar7 = DAT_00a60a2c;
                do {
                    pcVar13 = pcVar7;
                    if (uVar9 == 0) break;
                    uVar9 = uVar9 - 1;
                    pcVar13 = pcVar7 + 1;
                    cVar1 = *pcVar7;
                    pcVar7 = pcVar13;
                } while (cVar1 != '\0');
                uVar9 = ~uVar9;
                pbVar11 = (unsigned char *)(pcVar13 - uVar9);
                pbVar14 = local_100;
                for (uVar10 = uVar9 >> 2; uVar10 != 0; uVar10 = uVar10 - 1) {
                    __builtin_memcpy(&tmp, pbVar11, 4);
                    __builtin_memcpy(pbVar14, &tmp, 4);
                    pbVar11 = pbVar11 + 4;
                    pbVar14 = pbVar14 + 4;
                }
            }
            else {
                uVar9 = 0xffffffff;
                do {
                    pcVar13 = pcVar7;
                    if (uVar9 == 0) break;
                    uVar9 = uVar9 - 1;
                    pcVar13 = pcVar7 + 1;
                    cVar1 = *pcVar7;
                    pcVar7 = pcVar13;
                } while (cVar1 != '\0');
                uVar9 = ~uVar9;
                pbVar11 = (unsigned char *)(pcVar13 - uVar9);
                pbVar14 = local_100;
                for (uVar10 = uVar9 >> 2; uVar10 != 0; uVar10 = uVar10 - 1) {
                    __builtin_memcpy(&tmp, pbVar11, 4);
                    __builtin_memcpy(pbVar14, &tmp, 4);
                    pbVar11 = pbVar11 + 4;
                    pbVar14 = pbVar14 + 4;
                }
                for (uVar9 = uVar9 & 3; uVar9 != 0; uVar9 = uVar9 - 1) {
                    *pbVar14 = *pbVar11;
                    pbVar11 = pbVar11 + 1;
                    pbVar14 = pbVar14 + 1;
                }
                bVar3 = !bVar3;
                pcVar7 = DAT_00a60a34;
                if (bVar3) {
                    pcVar7 = DAT_00a60a30;
                }
                uVar9 = 0xffffffff;
                do {
                    pcVar13 = pcVar7;
                    if (uVar9 == 0) break;
                    uVar9 = uVar9 - 1;
                    pcVar13 = pcVar7 + 1;
                    cVar1 = *pcVar7;
                    pcVar7 = pcVar13;
                } while (cVar1 != '\0');
                uVar9 = ~uVar9;
                iVar8 = -1;
                pbVar11 = local_100;
                do {
                    pbVar14 = pbVar11;
                    if (iVar8 == 0) break;
                    iVar8 = iVar8 + -1;
                    pbVar14 = pbVar11 + 1;
                    bVar2 = *pbVar11;
                    pbVar11 = pbVar14;
                } while (bVar2 != 0);
                pbVar11 = (unsigned char *)(pcVar13 - uVar9);
                pbVar14 = pbVar14 - 1;
                for (uVar10 = uVar9 >> 2; uVar10 != 0; uVar10 = uVar10 - 1) {
                    __builtin_memcpy(&tmp, pbVar11, 4);
                    __builtin_memcpy(pbVar14, &tmp, 4);
                    pbVar11 = pbVar11 + 4;
                    pbVar14 = pbVar14 + 4;
                }
            }
            for (uVar9 = uVar9 & 3; uVar9 != 0; uVar9 = uVar9 - 1) {
                *pbVar14 = *pbVar11;
                pbVar11 = pbVar11 + 1;
                pbVar14 = pbVar14 + 1;
            }
            FUN_00448f90(3, local_100, 0x30);
            local_110 = DVar5;
        }
        if (*piVar4 == 0x13) {
            FUN_00448ed0();
            iVar8 = FUN_0046f700();
            _DAT_00557360 = 0x1b;
            FUN_0046d350(iVar8, &_DAT_00557360, 4);
            FUN_004ac760();
            DAT_00a07cc8 = 0;
            DAT_009e9658 = 0;
            DAT_009a8c2c = 0;
            if (DAT_00523428 == 0) {
                _DAT_009ed23c = 0x1e;
            }
            else {
                _DAT_009ed23c = 0x82;
            }
            if (DAT_00bb7370 == '\x04') {
                DAT_00523418 = 0x3b;
            }
            else if (DAT_00bb7370 == '\x06') {
                DAT_00523418 = 0x27;
            }
            else {
                DAT_00523418 = 0x1d;
            }
            iVar8 = FUN_0046e040();
            if (iVar8 != 0) {
                DAT_005242e0 = 1;
                if (local_108 != 1) {
                    DAT_005242e0 = 2;
                }
                FUN_00464c60();
                for (; (DAT_005242e0 != 0 && (DAT_009a8c2c < 100)); DAT_009a8c2c = DAT_009a8c2c + (unsigned int)iVar8) {
                    FUN_00463d00();
                    iVar8 = FUN_004ac760();
                }
                DAT_005242e0 = 0;
                iVar8 = FUN_004ac760();
                DAT_009a8c2c = DAT_009a8c2c + (unsigned int)iVar8;
                _DAT_009ed23c = DAT_009a8c2c;
                if ((int)DAT_009a8c2c < 0x23) {
                    iVar8 = (int)(0x23u - DAT_009a8c2c);
                    _DAT_009ed23c = DAT_009a8c2c + (unsigned int)iVar8;
                    DAT_009a8c2c = DAT_009a8c2c + (unsigned int)iVar8;
                    DAT_0052340c = DAT_0052340c - (unsigned int)iVar8;
                }
            }
            return 1;
        }
    } while (1);
}
