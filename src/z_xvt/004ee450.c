/* SPDX-License-Identifier: MIT */
/* z_xvt 004ee450 */
extern unsigned int DAT_0085e7c0;
extern unsigned int DAT_00757ccc;
extern unsigned int DAT_00757cc0;
extern unsigned int DAT_00757cc8;
extern int DAT_0081bfba;           /* unaligned */
extern unsigned char DAT_0081bfbe; /* base of unaligned uint array */
extern unsigned int DAT_0081d402;  /* unaligned */
extern unsigned char DAT_0081d40a; /* byte array */
extern unsigned char DAT_0081d80e; /* base of unaligned uint array */
extern unsigned char DAT_0081dc0e; /* base of unaligned uint array */
extern unsigned int DAT_0081d326;  /* unaligned */
extern unsigned int DAT_0081c012;  /* unaligned */
extern unsigned int DAT_0081c016;  /* unaligned */
extern unsigned int DAT_0081d406;  /* unaligned */
extern char DAT_0085e7b7;
extern char DAT_0081b740[];
extern char DAT_0052a330;
extern char *PTR_s_train_005330e0[];
extern char s__s__s_005331e4[];

int _sprintf(char *, const char *, ...);
void *FUN_004be160(char *, void *);
char *_fgets(char *, int, void *);
int FUN_004be1f0(void *);
int _atoi(const char *);
void _srand(unsigned int);
int _rand(void);
int _tolower(int);
void FUN_004e43d0(int);
void FUN_004f2990(void);
void FUN_004f0100(void);

unsigned int __stdcall FUN_004ee450(void)
{
    unsigned int *puVar1;
    char cVar2;
    unsigned char bVar3;
    void *_File;
    char *pcVar4;
    unsigned int uVar5;
    unsigned char *puVar6;
    int iVar7;
    unsigned char *pbVar8;
    unsigned int uVar9;
    unsigned int uVar10;
    unsigned char *pbVar11;
    int *piVar12;
    unsigned int iVar13;
    int iVar14;
    char *pcVar15;
    int bVar16;
    unsigned int uBA;
    unsigned int uTmp;
    unsigned int uTmp2;
    unsigned char local_80[128];

    uVar10 = DAT_0085e7c0;
    uVar9 = 0;
    if (DAT_00757ccc != 0) {
        piVar12 = (int *)(DAT_00757cc0 + 0x140);
        do {
            __builtin_memcpy(&uBA, &DAT_0081bfba, 4);
            __builtin_memcpy(&uTmp, &DAT_0081bfbe + uBA * 4U, 4);
            if ((unsigned int)*piVar12 == uTmp) break;
            piVar12 = piVar12 + 0x51;
            uVar9 = uVar9 + 1;
        } while (uVar9 < DAT_00757ccc);
    }
    __builtin_memcpy(&uBA, &DAT_0081bfba, 4);
    _sprintf((char *)local_80, s__s__s_005331e4, PTR_s_train_005330e0[uBA],
             (char *)(uVar9 * 0x144U + DAT_00757cc0));
    _File = FUN_004be160((char *)local_80, &DAT_0052a330);
    if (_File == (void *)0) {
        return 0;
    }
    pcVar4 = _fgets(DAT_0081b740, 0xff, _File);
    if (pcVar4 == (char *)0) {
        FUN_004be1f0(_File);
        return 0;
    }
    uVar5 = (unsigned int)_atoi(DAT_0081b740);
    __builtin_memcpy(&uVar9, &DAT_0081d326, 4);
    __builtin_memcpy(&uBA, &DAT_0081bfba, 4);
    if (uBA != 2) {
        __builtin_memcpy(&uTmp, &DAT_0081d402, 4);
        if ((&DAT_0081d40a)[uTmp] == 2) {
            __builtin_memcpy(&uTmp, &DAT_0081d402, 4);
            uTmp = uTmp - 1;
            __builtin_memcpy(&DAT_0081d402, &uTmp, 4);
            __builtin_memcpy(&uTmp, &DAT_0081d402, 4);
            __builtin_memcpy(&uVar9, &DAT_0081d80e + uTmp * 4U, 4);
        } else {
            __builtin_memcpy(&uVar9, &DAT_0081d402, 4);
            if (DAT_0085e7b7 != '\0') {
                _srand(uVar10);
                do {
                    uVar9 = (unsigned int)_rand();
                    bVar16 = 0;
                    uVar9 = uVar9 % uVar5;
                    __builtin_memcpy(&uTmp, &DAT_0081d402, 4);
                    if (-1 < (int)(uTmp - 1U)) {
                        puVar6 = &DAT_0081d80e + (uTmp - 1U) * 4U;
                        do {
                            __builtin_memcpy(&uTmp2, puVar6, 4);
                            if (uTmp2 == uVar9) {
                                bVar16 = 1;
                                break;
                            }
                            puVar6 = puVar6 - 4;
                        } while ((unsigned char *)0x81d80d < puVar6);
                    }
                } while (bVar16);
            }
        }
        __builtin_memcpy(&uTmp, &DAT_0081d402, 4);
        __builtin_memcpy(&DAT_0081d80e + uTmp * 4U, &uVar9, 4);
    }
    if (-1 < (int)uVar9) {
        iVar13 = uVar9 + 1U;
        do {
            _fgets(DAT_0081b740, 0xff, _File);
            uVar10 = 0xffffffff;
            pcVar4 = DAT_0081b740;
            do {
                if (uVar10 == 0) break;
                uVar10 = uVar10 - 1;
                cVar2 = *pcVar4;
                pcVar4 = pcVar4 + 1;
            } while (cVar2 != '\0');
            if (DAT_0081b740[(int)(~uVar10) - 2] == '\n') {
                uVar10 = 0xffffffff;
                pcVar4 = DAT_0081b740;
                do {
                    if (uVar10 == 0) break;
                    uVar10 = uVar10 - 1;
                    cVar2 = *pcVar4;
                    pcVar4 = pcVar4 + 1;
                } while (cVar2 != '\0');
                DAT_0081b740[(int)(~uVar10) - 2] = 0;
            }
            iVar13 = iVar13 - 1U;
        } while (iVar13 != 0);
    }
    FUN_004be1f0(_File);
    if (DAT_0081b740[0] == '\0') {
        return 0;
    }
    uTmp = 1;
    __builtin_memcpy(&DAT_0081c012, &uTmp, 4);
    __builtin_memcpy(&uBA, &DAT_0081bfba, 4);
    uBA = uBA - 1U;
    __builtin_memcpy(&DAT_0081bfba, &uBA, 4);
    __builtin_memcpy(&uBA, &DAT_0081bfba, 4);
    __builtin_memcpy(&uTmp, &DAT_0081bfbe + uBA * 4U, 4);
    __builtin_memcpy(&DAT_0081c016, &uTmp, 4);
    uVar10 = 0xffffffff;
    pcVar4 = DAT_0081b740;
    do {
        pcVar15 = pcVar4;
        if (uVar10 == 0) break;
        uVar10 = uVar10 - 1;
        pcVar15 = pcVar4 + 1;
        cVar2 = *pcVar4;
        pcVar4 = pcVar15;
    } while (cVar2 != '\0');
    uVar10 = ~uVar10;
    pbVar8 = (unsigned char *)(pcVar15 - uVar10);
    pbVar11 = local_80;
    for (uVar9 = uVar10 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
        *(unsigned int *)pbVar11 = *(unsigned int *)pbVar8;
        pbVar8 = pbVar8 + 4;
        pbVar11 = pbVar11 + 4;
    }
    for (uVar10 = uVar10 & 3; uVar10 != 0; uVar10 = uVar10 - 1) {
        *pbVar11 = *pbVar8;
        pbVar8 = pbVar8 + 1;
        pbVar11 = pbVar11 + 1;
    }
    uVar10 = 0xffffffff;
    pbVar8 = local_80;
    do {
        if (uVar10 == 0) break;
        uVar10 = uVar10 - 1;
        bVar3 = *pbVar8;
        pbVar8 = pbVar8 + 1;
    } while (bVar3 != 0);
    iVar13 = 0;
    if (~uVar10 != 1 && -1 < (int)(~uVar10 - 1)) {
        do {
            iVar14 = (int)(iVar13 + 1U);
            iVar7 = _tolower((unsigned int)local_80[iVar13]);
            local_80[iVar13] = (unsigned char)iVar7;
            uVar10 = 0xffffffff;
            pbVar8 = local_80;
            do {
                if (uVar10 == 0) break;
                uVar10 = uVar10 - 1;
                bVar3 = *pbVar8;
                pbVar8 = pbVar8 + 1;
            } while (bVar3 != 0);
            iVar13 = (unsigned int)iVar14;
        } while (iVar14 < (int)(~uVar10 - 1));
    }
    iVar7 = 0;
    __builtin_memcpy(&uBA, &DAT_0081bfba, 4);
    FUN_004e43d0((int)uBA);
    iVar13 = 0;
    if (0 < (int)DAT_00757ccc) {
        do {
            pbVar8 = local_80;
            pbVar11 = (unsigned char *)(DAT_00757cc0 + iVar13);
            do {
                bVar3 = *pbVar8;
                bVar16 = bVar3 < *pbVar11;
                if (bVar3 != *pbVar11) {
                    goto LAB_004ee709;
                }
                if (bVar3 == 0) break;
                bVar3 = pbVar8[1];
                bVar16 = bVar3 < pbVar11[1];
                if (bVar3 != pbVar11[1]) goto LAB_004ee709;
                pbVar8 = pbVar8 + 2;
                pbVar11 = pbVar11 + 2;
            } while (bVar3 != 0);
            iVar14 = 0;
            goto LAB_004ee70e;
LAB_004ee709:
            iVar14 = (int)((1U - (unsigned int)bVar16) - (unsigned int)(bVar16 != 0));
LAB_004ee70e:
            if (iVar14 == 0) {
                puVar1 = (unsigned int *)((unsigned int)iVar7 * 0x144U + 0x140U + DAT_00757cc0);
                __builtin_memcpy(&uBA, &DAT_0081bfba, 4);
                uTmp = *puVar1;
                __builtin_memcpy(&DAT_0081bfbe + uBA * 4U, &uTmp, 4);
                __builtin_memcpy(&uBA, &DAT_0081bfba, 4);
                if (uBA == 3) {
                    uTmp = *puVar1;
                    __builtin_memcpy(&DAT_0081d406, &uTmp, 4);
                }
                break;
            }
            iVar13 = iVar13 + 0x144U;
            iVar7 = (int)((unsigned int)iVar7 + 1U);
        } while (iVar7 < (int)DAT_00757ccc);
    }
    if ((DAT_00757cc0 != 0) && (DAT_00757cc8 = 0, DAT_00757ccc != 0)) {
        piVar12 = (int *)(DAT_00757cc0 + 0x140);
        do {
            __builtin_memcpy(&uBA, &DAT_0081bfba, 4);
            __builtin_memcpy(&uTmp, &DAT_0081bfbe + uBA * 4U, 4);
            if ((unsigned int)*piVar12 == uTmp) {
                __builtin_memcpy(&uTmp2, &DAT_0081d402, 4);
                uTmp = DAT_00757cc8;
                __builtin_memcpy(&DAT_0081dc0e + uTmp2 * 4U, &uTmp, 4);
                break;
            }
            piVar12 = piVar12 + 0x51;
            DAT_00757cc8 = DAT_00757cc8 + 1;
        } while (DAT_00757cc8 < DAT_00757ccc);
    }
    FUN_004f2990();
    FUN_004f0100();
    return 1;
}
