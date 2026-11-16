/* SPDX-License-Identifier: MIT */
/* z_bop 004ef100 */
typedef int (__cdecl *PFN_004ef100_cb)(unsigned int, int, unsigned int);

extern unsigned int *DAT_0052c818;
extern int DAT_0052c81c;
extern int DAT_0052c820[2];
extern int DAT_0052c82c;
extern unsigned int DAT_0052c83c;
extern unsigned int DAT_0052c840;
extern unsigned short DAT_0052c844;
extern unsigned char DAT_0052c846;
extern char DAT_0052c848;
extern char s_movies__0052c850[8];
extern unsigned int DAT_0052c808;
extern unsigned int DAT_0052c80c;
extern unsigned int _DAT_0052c810;
extern unsigned int DAT_0052c814;
extern char DAT_0051c4bc;
extern void *DAT_00aa6078;
extern int DAT_00aa6080;
extern int DAT_00aa6084;
extern int DAT_00665d98;
extern int DAT_00b69cb0;
extern int DAT_00b69cbc;

BOOL __stdcall ClientToScreen(HWND, LPPOINT);
BOOL __stdcall PeekMessageA(LPMSG, HWND, UINT, UINT, UINT);
BOOL __stdcall TranslateMessage(const MSG *);
LRESULT __stdcall DispatchMessageA(const MSG *);

void __cdecl FUN_004ef590(HWND hWnd);
int __cdecl FUN_004ef500(void);
void __cdecl FUN_004d4c10(void);
void __cdecl FUN_004d4f80(void);
void __cdecl FUN_004d49a0(void);
unsigned char __cdecl FUN_004ccb90(void);
int __cdecl FUN_004d5b80(void);
void __cdecl FUN_004d5b70(unsigned char param_1);
void __cdecl FUN_004ce130(void);
void __cdecl FUN_004ef600(void);
void * __cdecl FID_conflict___wfopen(char *, const char *);
int __cdecl _fclose(void *);
void __stdcall _SmackSoundUseDirectSound_4(unsigned int);
int __stdcall _SmackOpen_12(char *, unsigned int, unsigned int);
int __stdcall _SmackWait_4(int);
void __stdcall _SmackClose_4(int);

unsigned char __cdecl FUN_004ef100(unsigned int *param_1)
{
    char cVar1;
    unsigned char uVar2;
    BOOL BVar3;
    unsigned int uVar4;
    int iVar5;
    unsigned int uVar6;
    char *pcVar7;
    char *pcVar8;
    char *pcVar9;
    unsigned int auStack_12c[7]; /* MSG at stack0xfffffed4 (0x1c bytes) */
    char acStack_114[4];
    unsigned int uStack_110;
    __attribute__((unused)) unsigned int uStack_10c;
    __attribute__((unused)) unsigned short uStack_108;
    __attribute__((unused)) unsigned char uStack_106;
    char acStack_104[4];
    char acStack_100[256];

    DAT_0052c818 = param_1;
    FUN_004ef590((HWND)param_1[8]);
    DAT_0052c82c = FUN_004ef500();
    ClientToScreen((HWND)DAT_0052c818[8], (LPPOINT)&DAT_0052c820);
    FUN_004d4c10();
    FUN_004d4f80();
    FUN_004d49a0();
    FUN_004d4c10();
    _SmackSoundUseDirectSound_4(DAT_0052c818[6]);
    acStack_104[0] = s_movies__0052c850[0];
    acStack_104[1] = s_movies__0052c850[1];
    acStack_104[2] = s_movies__0052c850[2];
    acStack_104[3] = s_movies__0052c850[3];
    uVar4 = 0xffffffff;
    acStack_100[0] = s_movies__0052c850[4];
    acStack_100[1] = s_movies__0052c850[5];
    acStack_100[2] = s_movies__0052c850[6];
    acStack_100[3] = s_movies__0052c850[7];
    pcVar7 = (char *)*DAT_0052c818;
    do {
        pcVar9 = pcVar7;
        if (uVar4 == 0) break;
        uVar4 = uVar4 - 1;
        pcVar9 = pcVar7 + 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar9;
    } while (cVar1 != '\0');
    uVar4 = ~uVar4;
    iVar5 = -1;
    pcVar7 = acStack_104;
    do {
        pcVar8 = pcVar7;
        if (iVar5 == 0) break;
        iVar5 = (int)((unsigned int)iVar5 - 1U);
        pcVar8 = pcVar7 + 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar8;
    } while (cVar1 != '\0');
    pcVar7 = pcVar9 + -uVar4;
    pcVar9 = pcVar8 + -1;
    for (uVar6 = uVar4 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
        *(unsigned int *)pcVar9 = *(unsigned int *)pcVar7;
        pcVar7 = pcVar7 + 4;
        pcVar9 = pcVar9 + 4;
    }
    for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
        *pcVar9 = *pcVar7;
        pcVar7 = pcVar7 + 1;
        pcVar9 = pcVar9 + 1;
    }
    uVar4 = 0xffffffff;
    pcVar7 = &DAT_0052c848;
    do {
        pcVar9 = pcVar7;
        if (uVar4 == 0) break;
        uVar4 = uVar4 - 1;
        pcVar9 = pcVar7 + 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar9;
    } while (cVar1 != '\0');
    uVar4 = ~uVar4;
    iVar5 = -1;
    pcVar7 = acStack_104;
    do {
        pcVar8 = pcVar7;
        if (iVar5 == 0) break;
        iVar5 = (int)((unsigned int)iVar5 - 1U);
        pcVar8 = pcVar7 + 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar8;
    } while (cVar1 != '\0');
    pcVar7 = pcVar9 + -uVar4;
    pcVar9 = pcVar8 + -1;
    for (uVar6 = uVar4 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
        *(unsigned int *)pcVar9 = *(unsigned int *)pcVar7;
        pcVar7 = pcVar7 + 4;
        pcVar9 = pcVar9 + 4;
    }
    for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
        *pcVar9 = *pcVar7;
        pcVar7 = pcVar7 + 1;
        pcVar9 = pcVar9 + 1;
    }
    DAT_0052c81c = _SmackOpen_12(acStack_104, 0xfe000, 0xffffffff);
    if (DAT_0052c81c == 0) {
        uStack_110 = DAT_0052c83c;
        uStack_10c = DAT_0052c840;
        uStack_108 = DAT_0052c844;
        uStack_106 = DAT_0052c846;
        uVar2 = FUN_004ccb90();
        uVar4 = 0xffffffff;
        uStack_110 = (uStack_110 & 0xffffff00U) | (unsigned int)uVar2;
        pcVar7 = (char *)*DAT_0052c818;
        do {
            pcVar9 = pcVar7;
            if (uVar4 == 0) break;
            uVar4 = uVar4 - 1;
            pcVar9 = pcVar7 + 1;
            cVar1 = *pcVar7;
            pcVar7 = pcVar9;
        } while (cVar1 != '\0');
        uVar4 = ~uVar4;
        iVar5 = -1;
        pcVar7 = (char *)&uStack_110;
        do {
            pcVar8 = pcVar7;
            if (iVar5 == 0) break;
            iVar5 = (int)((unsigned int)iVar5 - 1U);
            pcVar8 = pcVar7 + 1;
            cVar1 = *pcVar7;
            pcVar7 = pcVar8;
        } while (cVar1 != '\0');
        pcVar7 = pcVar9 + -uVar4;
        pcVar9 = pcVar8 + -1;
        for (uVar6 = uVar4 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
            *(unsigned int *)pcVar9 = *(unsigned int *)pcVar7;
            pcVar7 = pcVar7 + 4;
            pcVar9 = pcVar9 + 4;
        }
        for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
            *pcVar9 = *pcVar7;
            pcVar7 = pcVar7 + 1;
            pcVar9 = pcVar9 + 1;
        }
        uVar4 = 0xffffffff;
        pcVar7 = &DAT_0052c848;
        do {
            pcVar9 = pcVar7;
            if (uVar4 == 0) break;
            uVar4 = uVar4 - 1;
            pcVar9 = pcVar7 + 1;
            cVar1 = *pcVar7;
            pcVar7 = pcVar9;
        } while (cVar1 != '\0');
        uVar4 = ~uVar4;
        iVar5 = -1;
        pcVar7 = (char *)&uStack_110;
        do {
            pcVar8 = pcVar7;
            if (iVar5 == 0) break;
            iVar5 = (int)((unsigned int)iVar5 - 1U);
            pcVar8 = pcVar7 + 1;
            cVar1 = *pcVar7;
            pcVar7 = pcVar8;
        } while (cVar1 != '\0');
        pcVar7 = pcVar9 + -uVar4;
        pcVar9 = pcVar8 + -1;
        for (uVar6 = uVar4 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
            *(unsigned int *)pcVar9 = *(unsigned int *)pcVar7;
            pcVar7 = pcVar7 + 4;
            pcVar9 = pcVar9 + 4;
        }
        for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
            *pcVar9 = *pcVar7;
            pcVar7 = pcVar7 + 1;
            pcVar9 = pcVar9 + 1;
        }
        DAT_0052c81c = _SmackOpen_12((char *)&uStack_110, 0xfe000, 0xffffffff);
        if (DAT_0052c81c == 0) {
            DAT_0052c818 = (unsigned int *)0;
            return 2;
        }
    }
    uVar4 = 0xffffffff;
    pcVar7 = (char *)&uStack_110;
    do {
        if (uVar4 == 0) break;
        uVar4 = uVar4 - 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar7 + 1;
    } while (cVar1 != '\0');
    uVar4 = ~uVar4;
    acStack_114[uVar4 + 2] = 't';
    acStack_114[uVar4] = 't';
    acStack_114[uVar4 + 1] = 'x';
    DAT_00aa6078 = FID_conflict___wfopen((char *)&uStack_110, &DAT_0051c4bc);
    uVar4 = *(unsigned int *)(DAT_0052c81c + 4);
    uVar6 = DAT_0052c818[1];
    if ((uVar6 < uVar4) && (DAT_0052c818[2] < *(unsigned int *)(DAT_0052c81c + 8))) {
        DAT_0052c818 = (unsigned int *)0;
        _SmackClose_4(DAT_0052c81c);
        return 3;
    }
    DAT_0052c808 = (uVar6 - uVar4) >> 1;
    DAT_0052c80c = (unsigned int)(DAT_0052c818[2] - *(unsigned int *)(DAT_0052c81c + 8)) >> 1;
    _DAT_0052c810 = (DAT_0052c818[1] - *(unsigned int *)(DAT_0052c81c + 4)) - DAT_0052c808;
    DAT_0052c814 = (DAT_0052c818[2] - *(unsigned int *)(DAT_0052c81c + 8)) - DAT_0052c80c;
    DAT_00aa6080 = FUN_004d5b80();
    FUN_004d5b70(2);
    DAT_00aa6084 = 0;
    DAT_00665d98 = 0;
    iVar5 = FUN_004d5b80();
    while (iVar5 == 2) {
        FUN_004ce130();
        BVar3 = PeekMessageA((LPMSG)auStack_12c, (HWND)0, 0, 0, 1);
        if (BVar3 == 0) {
            if (DAT_00aa6084 == 0) {
                iVar5 = _SmackWait_4(DAT_0052c81c);
                if (iVar5 == 0) {
                    FUN_004ef600();
                }
            } else if ((PFN_004ef100_cb)DAT_0052c818[0xb] == (PFN_004ef100_cb)0) {
                FUN_004d5b70((unsigned char)DAT_00aa6080);
            } else {
                iVar5 = ((PFN_004ef100_cb)DAT_0052c818[0xb])(
                            *(unsigned int *)(DAT_0052c81c + 0x374),
                            (int)((unsigned int)*(int *)(DAT_0052c81c + 0xc) - 1U),
                            DAT_0052c818[0xc]);
                if (iVar5 == 1) {
                    FUN_004d5b70((unsigned char)DAT_00aa6080);
                }
                if ((DAT_00b69cb0 == 0) && (DAT_00b69cbc == 0)) {
                    (*(int (__stdcall **)(int *, int, int))(*(int *)DAT_0052c818[3] + 0x2c))(
                        (int *)DAT_0052c818[3], 0, 1);
                }
            }
        } else {
            TranslateMessage((LPMSG)auStack_12c);
            DispatchMessageA((LPMSG)auStack_12c);
        }
        iVar5 = FUN_004d5b80();
    }
    if (DAT_00aa6078 != (void *)0) {
        _fclose(DAT_00aa6078);
        DAT_00aa6078 = (void *)0;
    }
    _SmackClose_4(DAT_0052c81c);
    DAT_0052c818 = (unsigned int *)0;
    return (unsigned char)(((unsigned int)(DAT_00665d98 == 0) - 1U) & 5);
}
