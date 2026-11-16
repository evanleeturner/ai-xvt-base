/* SPDX-License-Identifier: MIT */
/* z_xvt 0042d530 */
/* Prototypes for every callee. Calling conventions are unconfirmed; defaults assumed. */
void  FUN_00405cf0(int a);
int   FUN_0048f530(void);
void *_fopen(const char *name, const char *mode);
int   _fprintf(void *fp, const char *fmt, ...);
int   _fflush(void *fp);
void  FUN_00431110(int a, unsigned int *buf, unsigned int len);
int   FUN_00432ca0(void);
int   FUN_00431d30(void);
int   FUN_004a7530(void);
int   FUN_004a7560(unsigned char *buf, int max);
int  *FUN_0049b0d0(int a, unsigned int b, int *c);
void  FUN_004a5320(void);

extern unsigned char  DAT_00732ff2;
extern unsigned char  DAT_00732ff3;
extern unsigned char  DAT_00732ff4;
extern unsigned char  DAT_00732ff5;
extern int            DAT_00732ff0;
extern unsigned char  DAT_006db592;
extern unsigned short DAT_0073254c;
extern unsigned short DAT_00732554;
extern unsigned short DAT_00732f94;
extern unsigned char  DAT_005373d0[];
extern unsigned char  DAT_005371b8[];
extern unsigned int   DAT_006db5a8;
extern unsigned int   DAT_005397f8;
extern unsigned int   DAT_005373b8;
extern unsigned int   DAT_005373c4;
extern unsigned int   DAT_005375f0;
extern unsigned int   DAT_00536f68;
extern int            DAT_00732e48;
extern int            DAT_005250c8;
extern void          *DAT_005250cc;
extern int            DAT_00525568;
extern int            DAT_0071ecf4;
extern int            DAT_0072f444;
extern char           s_inputlog_txt_00525154[];
extern char           DAT_00525164[];
extern char           s__8x__2x__2x__2x__2x_0052513c[];
extern char           DAT_0071b740;
extern int            DAT_00732920;
extern int            DAT_00732940;
extern int            DAT_0071bcb8;
extern int            DAT_0071eaa0;

void __stdcall FUN_0042d530(void)
{
    int iVar1;
    unsigned int uVar2;
    int iVar3;
    int *piVar4;
    unsigned int *puVar5;
    char *pcVar6;
    unsigned int *puVar7;
    int *piVar8;
    unsigned int *puVar9;
    unsigned int tmp;
    int tmpi;

    FUN_00405cf0(-2);
    DAT_00732ff2 = (unsigned char)DAT_006db592;
    DAT_00732ff3 = (unsigned char)DAT_0073254c & 0xfe;
    tmp = 1;
    __builtin_memcpy(&DAT_005373d0[0], &tmp, 4);
    DAT_00732ff4 = (unsigned char)DAT_00732554 & 0xfe;
    DAT_00732ff5 = (unsigned char)DAT_00732f94 & 3;
    iVar1 = FUN_0048f530();
    DAT_006db5a8 = DAT_006db5a8 + (unsigned int)iVar1;
    if ((((int)(DAT_006db5a8 - DAT_005397f8) < 0x7f) && (-1 < (int)(DAT_006db5a8 - DAT_005397f8))) &&
        (DAT_005397f8 != 0)) {
        uVar2 = DAT_006db5a8 & 0x7f;
    } else {
        uVar2 = 0x7f;
    }
    if (0xec < (int)(DAT_005397f8 - DAT_005373b8)) {
        uVar2 = 0x7f;
    }
    if (uVar2 == 0x7f) {
        iVar1 = 9;
        DAT_005373d0[5] = (unsigned char)DAT_006db5a8;
        DAT_005373d0[6] = (unsigned char)(DAT_006db5a8 >> 8);
        DAT_005373d0[7] = (unsigned char)(DAT_006db5a8 >> 16);
        DAT_005373d0[4] = 0x7f;
        DAT_005373d0[8] = (unsigned char)(DAT_006db5a8 >> 0x18);
        DAT_005373b8 = DAT_006db5a8;
    } else {
        iVar1 = 5;
        DAT_005373d0[4] = (unsigned char)uVar2;
    }
    iVar3 = iVar1;
    DAT_005397f8 = DAT_006db5a8;
    if (DAT_00732ff2 != 0) {
        iVar3 = iVar1 + 1;
        DAT_005373d0[4] = DAT_005373d0[4] | 0x80;
        DAT_005373d0[iVar1] = DAT_00732ff2;
    }
    DAT_005373d0[iVar3] = DAT_00732ff3;
    DAT_005373d0[iVar3 + 1] = DAT_00732ff4;
    if ((DAT_00732ff5 & 1) != 0) {
        DAT_005373d0[iVar3] = DAT_005373d0[iVar3] | 1;
    }
    if ((DAT_00732ff5 & 2) != 0) {
        DAT_005373d0[iVar3 + 1] = DAT_005373d0[iVar3 + 1] | 1;
    }
    if (1 < DAT_00732e48) {
        if ((DAT_005250c8 == 1) &&
            ((DAT_005250cc != 0) ||
             ((DAT_005250cc = _fopen(s_inputlog_txt_00525154, DAT_00525164)), DAT_005250cc != 0))) {
            __builtin_memcpy(&tmp, &DAT_005373d0[4], 4);
            _fprintf(DAT_005250cc, s__8x__2x__2x__2x__2x_0052513c, tmp,
                     (unsigned int)DAT_00732ff2, (unsigned int)DAT_00732ff3,
                     (unsigned int)DAT_00732ff4, (unsigned int)DAT_00732ff5);
            _fflush(DAT_005250cc);
        }
        if (DAT_00525568 == 0) {
            iVar1 = 0;
            pcVar6 = &DAT_0071b740;
            piVar4 = &DAT_00732920;
            do {
                if ((*pcVar6 != '\0') && ((*piVar4 != 0) || (iVar1 == DAT_0071ecf4))) {
                    __builtin_memcpy(&tmpi, pcVar6 + 0x578, 4);
                    FUN_00431110(tmpi, (unsigned int *)&DAT_005373d0[0],
                                 (unsigned int)(iVar3 + 2));
                }
                pcVar6 = pcVar6 + 0x5bd;
                piVar4 = piVar4 + 1;
                iVar1 = iVar1 + 1;
            } while (piVar4 < &DAT_00732940);
        } else {
            tmp = 0x16;
            __builtin_memcpy(&DAT_005371b8[0], &tmp, 4);
            DAT_005371b8[4] = DAT_005371b8[4] + 1;
            puVar5 = (unsigned int *)&DAT_005373d0[4];
            puVar7 = (unsigned int *)&DAT_005371b8[DAT_005373c4];
            for (uVar2 = ((unsigned int)iVar3 - 2U) >> 2; uVar2 != 0; uVar2 = uVar2 - 1) {
                __builtin_memcpy(puVar7, puVar5, 4);
                puVar5 = puVar5 + 1;
                puVar7 = puVar7 + 1;
            }
            for (uVar2 = ((unsigned int)iVar3 - 2U) & 3; uVar2 != 0; uVar2 = uVar2 - 1) {
                *(unsigned char *)puVar7 = *(unsigned char *)puVar5;
                puVar5 = (unsigned int *)((unsigned char *)puVar5 + 1);
                puVar7 = (unsigned int *)((unsigned char *)puVar7 + 1);
            }
            DAT_005373c4 = DAT_005373c4 + (unsigned int)iVar3 + 0xfffffffeU;
            if (DAT_005375f0 < DAT_006db5a8 - DAT_00536f68) {
                puVar9 = (unsigned int *)&DAT_005371b8[0];
                DAT_00536f68 = DAT_006db5a8;
                uVar2 = DAT_005373c4;
                iVar1 = FUN_00432ca0();
                FUN_00431110(iVar1, puVar9, uVar2);
                iVar1 = FUN_00431d30();
                if (iVar1 == 0) {
                    if (DAT_0072f444 < 4) {
                        iVar1 = 0;
                        piVar4 = &DAT_00732920;
                        piVar8 = &DAT_0071bcb8;
                        do {
                            if (*((char *)piVar8 - 0x578) != '\0' && (iVar1 != DAT_0071ecf4)) {
                                iVar3 = FUN_00432ca0();
                                __builtin_memcpy(&tmpi, piVar8, 4);
                                if ((iVar3 != tmpi) && (*piVar4 != 0)) {
                                    __builtin_memcpy(&tmpi, piVar8, 4);
                                    FUN_00431110(tmpi, (unsigned int *)&DAT_005371b8[0],
                                                 DAT_005373c4);
                                }
                            }
                            piVar4 = piVar4 + 1;
                            piVar8 = (int *)((char *)piVar8 + 0x5bd);
                            iVar1 = iVar1 + 1;
                        } while (piVar8 < &DAT_0071eaa0);
                    }
                } else if (DAT_0072f444 == 2) {
                    iVar1 = 0;
                    pcVar6 = &DAT_0071b740;
                    piVar4 = &DAT_00732920;
                    do {
                        if (((*pcVar6 != '\0') && (iVar1 != DAT_0071ecf4)) && (*piVar4 != 0)) {
                            __builtin_memcpy(&tmpi, pcVar6 + 0x578, 4);
                            FUN_00431110(tmpi, (unsigned int *)&DAT_005371b8[0], DAT_005373c4);
                        }
                        pcVar6 = pcVar6 + 0x5bd;
                        piVar4 = piVar4 + 1;
                        iVar1 = iVar1 + 1;
                    } while (piVar4 < &DAT_00732940);
                }
                DAT_005373c4 = 5;
                tmp = 0x16;
                __builtin_memcpy(&DAT_005371b8[0], &tmp, 4);
                DAT_005371b8[4] = 0;
            }
        }
        iVar1 = FUN_004a7530();
        if (iVar1 != 0) {
            tmp = 0x19;
            __builtin_memcpy(&DAT_005373d0[0], &tmp, 4);
            tmpi = FUN_004a7560(&DAT_005373d0[8], 500);
            __builtin_memcpy(&DAT_005373d0[4], &tmpi, 4);
            __builtin_memcpy(&tmp, &DAT_005373d0[4], 4);
            uVar2 = tmp + 8;
            puVar9 = (unsigned int *)&DAT_005373d0[0];
            iVar1 = FUN_00432ca0();
            FUN_00431110(iVar1, puVar9, uVar2);
        }
    }
    piVar4 = FUN_0049b0d0(DAT_0071ecf4, DAT_006db5a8, &DAT_00732ff0);
    if (piVar4 != (int *)0) {
        *piVar4 = 0;
        piVar4[1] = 1;
    }
    FUN_004a5320();
    return;
}
