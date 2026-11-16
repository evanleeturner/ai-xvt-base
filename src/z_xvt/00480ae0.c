/* SPDX-License-Identifier: MIT */
/* z_xvt 00480ae0 */
typedef struct _iobuf FILE;

extern FILE *DAT_00732948;
extern char DAT_0052a334;
extern char DAT_0052a330;
extern char DAT_0052a328;
extern char DAT_0052a324;
extern unsigned char DAT_006db7c0;
extern char DAT_00703ca0;
extern char DAT_00703cf0;
extern char DAT_00703cf1;
extern char DAT_007091a0;
extern unsigned char *DAT_00732a40;
extern int DAT_006da9e8;
extern char s_LOCATARGET_005285a0[];
extern char s_NULLMANR_005288d8[];
extern char s_NULLORDR_00529370[];
extern unsigned char DAT_005285f0;
extern unsigned char DAT_00528928;
extern unsigned char DAT_005293c0;

int FUN_0047ed40(char *param_1, char *param_2, int param_3, int param_4);
int FUN_00480a00(char *param_1, FILE *param_2);
int _strncmp(const char *a, const char *b, unsigned int n);
int _fclose(FILE *f);
char *_strncpy(char *d, const char *s, unsigned int n);
unsigned int _fwrite(const void *p, unsigned int sz, unsigned int n, FILE *f);

unsigned int FUN_00480ae0(char *param_1)
{
    char cVar1;
    unsigned char bVar2;
    FILE *pFVar3;
    unsigned int uVar4;
    unsigned int uVar5;
    int iVar6;
    unsigned char *pbVar7;
    char *pcVar8;
    int iVar9;
    unsigned char *puVar10;
    unsigned char *puVar11;
    unsigned int sVar12;
    unsigned char *pbVar13;
    char *pcVar14;
    char *pcVar15;
    int bVar16;
    unsigned char *puVar17;
    unsigned int sStack_204;
    unsigned char abStack_200[256];
    char acStack_100[256];

    uVar4 = 0xffffffff;
    pcVar8 = param_1;
    do {
        pcVar15 = pcVar8;
        if (uVar4 == 0) break;
        uVar4 = uVar4 - 1;
        pcVar15 = pcVar8 + 1;
        cVar1 = *pcVar8;
        pcVar8 = pcVar15;
    } while (cVar1 != '\0');
    uVar4 = ~uVar4;
    pcVar8 = pcVar15 + -uVar4;
    pcVar15 = acStack_100;
    for (uVar5 = uVar4 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
        __builtin_memcpy(pcVar15, pcVar8, 4);
        pcVar8 = pcVar8 + 4;
        pcVar15 = pcVar15 + 4;
    }
    for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
        *pcVar15 = *pcVar8;
        pcVar8 = pcVar8 + 1;
        pcVar15 = pcVar15 + 1;
    }
    uVar4 = 0xffffffff;
    pcVar8 = &DAT_0052a334;
    do {
        pcVar15 = pcVar8;
        if (uVar4 == 0) break;
        uVar4 = uVar4 - 1;
        pcVar15 = pcVar8 + 1;
        cVar1 = *pcVar8;
        pcVar8 = pcVar15;
    } while (cVar1 != '\0');
    uVar4 = ~uVar4;
    iVar6 = -1;
    pcVar8 = acStack_100;
    do {
        pcVar14 = pcVar8;
        if (iVar6 == 0) break;
        iVar6 = iVar6 + -1;
        pcVar14 = pcVar8 + 1;
        cVar1 = *pcVar8;
        pcVar8 = pcVar14;
    } while (cVar1 != '\0');
    pcVar8 = pcVar15 + -uVar4;
    pcVar15 = pcVar14 + -1;
    for (uVar5 = uVar4 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
        __builtin_memcpy(pcVar15, pcVar8, 4);
        pcVar8 = pcVar8 + 4;
        pcVar15 = pcVar15 + 4;
    }
    for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
        *pcVar15 = *pcVar8;
        pcVar8 = pcVar8 + 1;
        pcVar15 = pcVar15 + 1;
    }
    FUN_0047ed40(acStack_100, &DAT_0052a330, 0, 0);
    pFVar3 = DAT_00732948;
    if (DAT_00732948 == (FILE *)0) {
        return 0;
    }
    puVar10 = &DAT_006db7c0;
    while (1) {
        iVar6 = FUN_00480a00((char *)abStack_200, pFVar3);
        if (iVar6 == 0) {
            _fclose(pFVar3);
            return 0;
        }
        if (abStack_200[0] == 0x2a) break;
        sVar12 = 0;
        pcVar8 = &DAT_00703ca0;
        do {
            iVar6 = _strncmp(pcVar8, (char *)abStack_200, 0x50);
            if (iVar6 == 0) break;
            pcVar8 = pcVar8 + 0x55;
            sVar12 = sVar12 + 1;
        } while (pcVar8 < &DAT_007091a0);
        if (sVar12 == 0x100) {
            sStack_204 = 0;
            pcVar8 = &DAT_00703ca0;
            do {
                if (*pcVar8 == '\0') break;
                pcVar8 = pcVar8 + 0x55;
                sStack_204 = sStack_204 + 1;
            } while (pcVar8 < &DAT_007091a0);
            if (sStack_204 == 0x100) {
                _fclose(pFVar3);
                return 0;
            }
        }
        else {
            sStack_204 = sVar12;
            if ((&DAT_00703cf0)[sVar12 * 0x55] == '\x01') {
                _fclose(pFVar3);
                return 0;
            }
        }
        _strncpy((char *)((int)&DAT_00703ca0 + sStack_204 * 0x55), (char *)abStack_200, 0x50);
        iVar9 = 0;
        *(unsigned char *)(sStack_204 * 0x55 + 0x703cef) = 0;
        (&DAT_00703cf0)[sStack_204 * 0x55] = 1;
        puVar17 = puVar10 + -0x6db7c0;
        __builtin_memcpy((char *)((int)&DAT_00703cf1 + sStack_204 * 0x55), &puVar17, 4);
        (&DAT_00732a40)[sStack_204] = puVar10;
        iVar6 = FUN_00480a00((char *)abStack_200, pFVar3);
        if (iVar6 == 0) {
            _fclose(pFVar3);
            return 0;
        }
        if (s_LOCATARGET_005285a0[0] != '\0') {
            pcVar8 = s_LOCATARGET_005285a0;
            do {
                pbVar13 = abStack_200;
                pbVar7 = (unsigned char *)pcVar8;
                do {
                    bVar2 = *pbVar7;
                    bVar16 = bVar2 < *pbVar13;
                    if (bVar2 != *pbVar13) {
LAB_00480ccc:
                        iVar6 = (1 - (unsigned int)bVar16) - (unsigned int)(bVar16 != 0);
                        goto LAB_00480cd1;
                    }
                    if (bVar2 == 0) break;
                    bVar2 = pbVar7[1];
                    bVar16 = bVar2 < pbVar13[1];
                    if (bVar2 != pbVar13[1]) goto LAB_00480ccc;
                    pbVar7 = pbVar7 + 2;
                    pbVar13 = pbVar13 + 2;
                } while (bVar2 != 0);
                iVar6 = 0;
LAB_00480cd1:
                if (iVar6 == 0) break;
                pbVar13 = (unsigned char *)(pcVar8 + 0x52);
                pcVar8 = pcVar8 + 0x52;
                iVar9 = iVar9 + 1;
            } while (*pbVar13 != 0);
        }
        if (s_LOCATARGET_005285a0[iVar9 * 0x52] == '\0') {
            _fclose(pFVar3);
            return 0;
        }
        *puVar10 = (&DAT_005285f0)[iVar9 * 0x52];
        iVar6 = FUN_00480a00((char *)abStack_200, pFVar3);
        if (iVar6 == 0) {
            _fclose(pFVar3);
            return 0;
        }
        iVar6 = 0;
        if (s_NULLMANR_005288d8[0] != '\0') {
            pcVar8 = s_NULLMANR_005288d8;
            do {
                pbVar13 = abStack_200;
                pbVar7 = (unsigned char *)pcVar8;
                do {
                    bVar2 = *pbVar7;
                    bVar16 = bVar2 < *pbVar13;
                    if (bVar2 != *pbVar13) {
LAB_00480d4c:
                        iVar9 = (1 - (unsigned int)bVar16) - (unsigned int)(bVar16 != 0);
                        goto LAB_00480d51;
                    }
                    if (bVar2 == 0) break;
                    bVar2 = pbVar7[1];
                    bVar16 = bVar2 < pbVar13[1];
                    if (bVar2 != pbVar13[1]) goto LAB_00480d4c;
                    pbVar7 = pbVar7 + 2;
                    pbVar13 = pbVar13 + 2;
                } while (bVar2 != 0);
                iVar9 = 0;
LAB_00480d51:
                if (iVar9 == 0) break;
                pbVar13 = (unsigned char *)(pcVar8 + 0x52);
                pcVar8 = pcVar8 + 0x52;
                iVar6 = iVar6 + 1;
            } while (*pbVar13 != 0);
        }
        if (s_NULLMANR_005288d8[iVar6 * 0x52] == '\0') {
            _fclose(pFVar3);
            return 0;
        }
        puVar10[1] = (&DAT_00528928)[iVar6 * 0x52];
        puVar11 = puVar10 + 1;
LAB_00480d7e:
        iVar6 = FUN_00480a00((char *)abStack_200, pFVar3);
        if (iVar6 == 0) {
            _fclose(pFVar3);
            return 0;
        }
        iVar6 = 0;
        if (s_NULLORDR_00529370[0] != '\0') {
            pcVar8 = s_NULLORDR_00529370;
            do {
                pbVar13 = abStack_200;
                pbVar7 = (unsigned char *)pcVar8;
                do {
                    bVar2 = *pbVar7;
                    bVar16 = bVar2 < *pbVar13;
                    if (bVar2 != *pbVar13) {
LAB_00480dcb:
                        iVar9 = (1 - (unsigned int)bVar16) - (unsigned int)(bVar16 != 0);
                        goto LAB_00480dd0;
                    }
                    if (bVar2 == 0) break;
                    bVar2 = pbVar7[1];
                    bVar16 = bVar2 < pbVar13[1];
                    if (bVar2 != pbVar13[1]) goto LAB_00480dcb;
                    pbVar7 = pbVar7 + 2;
                    pbVar13 = pbVar13 + 2;
                } while (bVar2 != 0);
                iVar9 = 0;
LAB_00480dd0:
                if (iVar9 == 0) break;
                pbVar13 = (unsigned char *)(pcVar8 + 0x52);
                pcVar8 = pcVar8 + 0x52;
                iVar6 = iVar6 + 1;
            } while (*pbVar13 != 0);
        }
        if (s_NULLORDR_00529370[iVar6 * 0x52] == '\0') {
            _fclose(pFVar3);
            return 0;
        }
        puVar10 = puVar11 + 2;
        puVar11[1] = (&DAT_005293c0)[iVar6 * 0x52];
        if (iVar6 != 0) {
            iVar6 = FUN_00480a00((char *)abStack_200, pFVar3);
            if (iVar6 == 0) {
                _fclose(pFVar3);
                return 0;
            }
            iVar6 = 0;
            pcVar8 = &DAT_00703ca0;
            do {
                iVar9 = _strncmp(pcVar8, (char *)abStack_200, 0x50);
                if (iVar9 == 0) break;
                pcVar8 = pcVar8 + 0x55;
                iVar6 = iVar6 + 1;
            } while (pcVar8 < &DAT_007091a0);
            puVar11 = puVar10;
            if (iVar6 == 0x100) {
                iVar6 = 0;
                pcVar8 = &DAT_00703ca0;
                do {
                    if (*pcVar8 == '\0') break;
                    pcVar8 = pcVar8 + 0x55;
                    iVar6 = iVar6 + 1;
                } while (pcVar8 < &DAT_007091a0);
                if (iVar6 == 0x100) {
                    _fclose(pFVar3);
                    return 0;
                }
                _strncpy((char *)((int)&DAT_00703ca0 + iVar6 * 0x55), (char *)abStack_200, 0x50);
                *(unsigned char *)(iVar6 * 0x55 + 0x703cef) = 0;
                (&DAT_00703cf0)[iVar6 * 0x55] = 0;
                *puVar10 = (char)iVar6;
            }
            else {
                *puVar10 = (char)iVar6;
            }
            goto LAB_00480d7e;
        }
        DAT_006da9e8 = DAT_006da9e8 + 1;
    }
    _fclose(pFVar3);
    sStack_204 = 0;
    do {
        if ((*(char *)((int)&DAT_00703ca0 + sStack_204 * 0x55) != '\0') &&
            ((&DAT_00703cf0)[sStack_204 * 0x55] != '\x01')) {
            return 0;
        }
        sStack_204 = sStack_204 + 1;
    } while ((int)sStack_204 < 0x100);
    uVar4 = 0xffffffff;
    do {
        pcVar8 = param_1;
        if (uVar4 == 0) break;
        uVar4 = uVar4 - 1;
        pcVar8 = param_1 + 1;
        cVar1 = *param_1;
        param_1 = pcVar8;
    } while (cVar1 != '\0');
    uVar4 = ~uVar4;
    pcVar8 = pcVar8 + -uVar4;
    pcVar15 = acStack_100;
    for (uVar5 = uVar4 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
        __builtin_memcpy(pcVar15, pcVar8, 4);
        pcVar8 = pcVar8 + 4;
        pcVar15 = pcVar15 + 4;
    }
    for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
        *pcVar15 = *pcVar8;
        pcVar8 = pcVar8 + 1;
        pcVar15 = pcVar15 + 1;
    }
    uVar4 = 0xffffffff;
    pcVar8 = &DAT_0052a328;
    do {
        pcVar15 = pcVar8;
        if (uVar4 == 0) break;
        uVar4 = uVar4 - 1;
        pcVar15 = pcVar8 + 1;
        cVar1 = *pcVar8;
        pcVar8 = pcVar15;
    } while (cVar1 != '\0');
    uVar4 = ~uVar4;
    iVar6 = -1;
    pcVar8 = acStack_100;
    do {
        pcVar14 = pcVar8;
        if (iVar6 == 0) break;
        iVar6 = iVar6 + -1;
        pcVar14 = pcVar8 + 1;
        cVar1 = *pcVar8;
        pcVar8 = pcVar14;
    } while (cVar1 != '\0');
    pcVar8 = pcVar15 + -uVar4;
    pcVar15 = pcVar14 + -1;
    for (uVar5 = uVar4 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
        __builtin_memcpy(pcVar15, pcVar8, 4);
        pcVar8 = pcVar8 + 4;
        pcVar15 = pcVar15 + 4;
    }
    for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
        *pcVar15 = *pcVar8;
        pcVar8 = pcVar8 + 1;
        pcVar15 = pcVar15 + 1;
    }
    FUN_0047ed40(acStack_100, &DAT_0052a324, 0, 1);
    pFVar3 = DAT_00732948;
    if (DAT_00732948 != (FILE *)0) {
        sStack_204 = 0x5500;
        _fwrite(&sStack_204, 4, 1, DAT_00732948);
        _fwrite(&DAT_00703ca0, sStack_204, 1, pFVar3);
        sStack_204 = 0xffff;
        _fwrite(&sStack_204, 4, 1, pFVar3);
        _fwrite(&DAT_006db7c0, sStack_204, 1, pFVar3);
        _fclose(pFVar3);
    }
    return 1;
}
