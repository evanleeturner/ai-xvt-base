/* SPDX-License-Identifier: MIT */
/* z_bop 004e5000 */
extern void __cdecl _free(void *p);
extern void * __cdecl _malloc(unsigned int size);
extern int __cdecl _sprintf(char *dst, const char *fmt, ...);
extern char * __cdecl _fgets(char *buf, int n, void *file);
extern int __cdecl _atoi(const char *s);
extern int __cdecl _tolower(int c);
extern void * __cdecl FUN_004cc4a0(char *path, const char *mode);
extern char * __cdecl FUN_004ddc10(int id);
extern int __cdecl FUN_004dcb90(char *a, char *b, char *c, char *d, char *e);
extern int __cdecl FUN_004e9230(void *file);
extern void __cdecl FUN_004cc5c0(void *file, int a, int b);
extern void __cdecl FUN_004cc590(void *file);

extern void *DAT_00aa6114;
extern unsigned int DAT_00aa6138;
extern int DAT_00b69e30;
extern int DAT_00b6d962;
extern char DAT_00b69d20[];
extern char DAT_0051c4bc[];
extern char *PTR_s_train_0052c1e8[];
extern char s__s_rebel_lst_0052c2e0[];
extern char s__s_imperial_lst_0052c2d0[];
extern char s__s_mission_lst_0052c2f0[];
extern unsigned char DAT_00b7ca4a[];
extern unsigned char DAT_00b7d6ca[];
extern unsigned char DAT_00b8e0aa[];

void __cdecl FUN_004e5000(int param_1)
{
    char cVar1;
    void *_File;
    char *pcVar2;
    char *pcVar3;
    char *pcVar4;
    char *pcVar5;
    int iVar6;
    int iVar7;
    unsigned int uVar8;
    unsigned int uVar9;
    int iVar10;
    unsigned int uVar11;
    char *pcVar12;
    char *puVar13;
    unsigned int local_84;
    char local_80[128];
    int tmpA;
    int tmpB;

    if (DAT_00aa6114 != (void *)0) {
        _free(DAT_00aa6114);
        DAT_00aa6114 = (void *)0;
    }
    DAT_00aa6138 = 0;
    if ((DAT_00b69e30 == 2) || (DAT_00b69e30 == 0)) {
        switch (param_1) {
        case 0:
        case 3:
        case 5:
            puVar13 = PTR_s_train_0052c1e8[param_1];
            if (DAT_00b6d962 == 0) {
                pcVar12 = s__s_rebel_lst_0052c2e0;
            } else {
                pcVar12 = s__s_imperial_lst_0052c2d0;
            }
            break;
        case 1:
        case 2:
        case 4:
            puVar13 = PTR_s_train_0052c1e8[param_1];
            goto LAB_004e5092;
        default:
            goto switchD_004e5064_default;
        }
    } else {
        puVar13 = PTR_s_train_0052c1e8[param_1];
LAB_004e5092:
        pcVar12 = s__s_mission_lst_0052c2f0;
    }
    _sprintf(DAT_00b69d20, pcVar12, puVar13);
switchD_004e5064_default:
    while ((_File = FUN_004cc4a0(DAT_00b69d20, DAT_0051c4bc)) == (void *)0) {
        pcVar12 = FUN_004ddc10(0x13);
        pcVar2 = FUN_004ddc10(0x20b);
        pcVar3 = FUN_004ddc10(0x2c4);
        pcVar4 = FUN_004ddc10(0x2c3);
        pcVar5 = FUN_004ddc10(0x2c2);
        iVar6 = FUN_004dcb90(pcVar5, pcVar4, pcVar3, pcVar2, pcVar12);
        if (iVar6 == 0) {
            return;
        }
    }
    DAT_00aa6138 = (unsigned int)FUN_004e9230(_File);
    FUN_004cc5c0(_File, 0, 0);
    if (DAT_00aa6138 == 0) {
        FUN_004cc590(_File);
        return;
    }
    DAT_00aa6114 = _malloc(DAT_00aa6138 * 0x148);
    if (DAT_00aa6114 == (void *)0) {
        FUN_004cc590(_File);
        return;
    }
    iVar10 = 0;
    pcVar12 = local_80;
    for (iVar6 = 0x20; iVar6 != 0; iVar6 = iVar6 + -1) {
        pcVar12[0] = '\0';
        pcVar12[1] = '\0';
        pcVar12[2] = '\0';
        pcVar12[3] = '\0';
        pcVar12 = pcVar12 + 4;
    }
    local_84 = 0;
    do {
        if (DAT_00aa6138 <= local_84) {
            FUN_004cc590(_File);
            return;
        }
LAB_004e51a5:
        do {
            pcVar12 = _fgets(DAT_00b69d20, 0x100, _File);
            if (pcVar12 == (char *)0) {
                DAT_00aa6138 = local_84;
                FUN_004cc590(_File);
                return;
            }
        } while ((DAT_00b69d20[0] == '/') && (DAT_00b69d20[1] == '/'));
        uVar8 = 0xffffffff;
        pcVar12 = DAT_00b69d20;
        do {
            if (uVar8 == 0) break;
            uVar8 = uVar8 - 1;
            cVar1 = *pcVar12;
            pcVar12 = pcVar12 + 1;
        } while (cVar1 != '\0');
        if ((DAT_00b69d20 - 2)[~uVar8] == '\n') {
            uVar8 = 0xffffffff;
            pcVar12 = DAT_00b69d20;
            do {
                if (uVar8 == 0) break;
                uVar8 = uVar8 - 1;
                cVar1 = *pcVar12;
                pcVar12 = pcVar12 + 1;
            } while (cVar1 != '\0');
            (DAT_00b69d20 - 2)[~uVar8] = 0;
        }
        if (DAT_00b69d20[0] == '[') {
            uVar8 = 0xffffffff;
            pcVar12 = DAT_00b69d20;
            do {
                if (uVar8 == 0) break;
                uVar8 = uVar8 - 1;
                cVar1 = *pcVar12;
                pcVar12 = pcVar12 + 1;
            } while (cVar1 != '\0');
            (DAT_00b69d20 - 2)[~uVar8] = 0;
            uVar8 = 0xffffffff;
            pcVar12 = DAT_00b69d20 + 1;
            do {
                pcVar2 = pcVar12;
                if (uVar8 == 0) break;
                uVar8 = uVar8 - 1;
                pcVar2 = pcVar12 + 1;
                cVar1 = *pcVar12;
                pcVar12 = pcVar2;
            } while (cVar1 != '\0');
            uVar8 = ~uVar8;
            pcVar12 = pcVar2 + -uVar8;
            pcVar2 = local_80;
            for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
                __builtin_memcpy(pcVar2, pcVar12, 4);
                pcVar12 = pcVar12 + 4;
                pcVar2 = pcVar2 + 4;
            }
            for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
                *pcVar2 = *pcVar12;
                pcVar12 = pcVar12 + 1;
                pcVar2 = pcVar2 + 1;
            }
            goto LAB_004e51a5;
        }
        uVar8 = 0xffffffff;
        pcVar12 = local_80;
        do {
            pcVar2 = pcVar12;
            if (uVar8 == 0) break;
            uVar8 = uVar8 - 1;
            pcVar2 = pcVar12 + 1;
            cVar1 = *pcVar12;
            pcVar12 = pcVar2;
        } while (cVar1 != '\0');
        uVar8 = ~uVar8;
        pcVar12 = pcVar2 + -uVar8;
        pcVar2 = (char *)DAT_00aa6114 + iVar10 + 0xc0;
        for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
            __builtin_memcpy(pcVar2, pcVar12, 4);
            pcVar12 = pcVar12 + 4;
            pcVar2 = pcVar2 + 4;
        }
        for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
            *pcVar2 = *pcVar12;
            pcVar12 = pcVar12 + 1;
            pcVar2 = pcVar2 + 1;
        }
        *(int *)((char *)DAT_00aa6114 + iVar10 + 0x144) = 0;
        iVar6 = _atoi(DAT_00b69d20);
        *(int *)((char *)DAT_00aa6114 + iVar10 + 0x140) = iVar6;
        pcVar12 = _fgets(DAT_00b69d20, 0x100, _File);
        if (pcVar12 == (char *)0) {
            DAT_00aa6138 = local_84;
            FUN_004cc590(_File);
            return;
        }
        uVar8 = 0xffffffff;
        pcVar12 = DAT_00b69d20;
        do {
            if (uVar8 == 0) break;
            uVar8 = uVar8 - 1;
            cVar1 = *pcVar12;
            pcVar12 = pcVar12 + 1;
        } while (cVar1 != '\0');
        if ((DAT_00b69d20 - 2)[~uVar8] == '\n') {
            uVar8 = 0xffffffff;
            pcVar12 = DAT_00b69d20;
            do {
                if (uVar8 == 0) break;
                uVar8 = uVar8 - 1;
                cVar1 = *pcVar12;
                pcVar12 = pcVar12 + 1;
            } while (cVar1 != '\0');
            (DAT_00b69d20 - 2)[~uVar8] = 0;
        }
        iVar6 = -1;
        pcVar12 = DAT_00b69d20;
        do {
            if (iVar6 == 0) break;
            iVar6 = iVar6 + -1;
            cVar1 = *pcVar12;
            pcVar12 = pcVar12 + 1;
        } while (cVar1 != '\0');
        uVar8 = 0;
        if (iVar6 != -2) {
            do {
                uVar11 = uVar8 + 1;
                iVar6 = _tolower((unsigned int)*(unsigned char *)(DAT_00b69d20 + uVar8));
                uVar9 = 0xffffffff;
                *(DAT_00b69d20 + uVar8) = (char)iVar6;
                pcVar12 = DAT_00b69d20;
                do {
                    if (uVar9 == 0) break;
                    uVar9 = uVar9 - 1;
                    cVar1 = *pcVar12;
                    pcVar12 = pcVar12 + 1;
                } while (cVar1 != '\0');
                uVar8 = uVar11;
            } while (uVar11 < ~uVar9 - 1);
        }
        if (DAT_00b69d20[0] == '*') {
            iVar6 = 2;
            cVar1 = DAT_00b69d20[2];
            while (cVar1 != ' ') {
                cVar1 = DAT_00b69d20[iVar6 + 1];
                iVar6 = iVar6 + 1;
            }
            DAT_00b69d20[iVar6] = 0;
            _atoi(DAT_00b69d20 + 2);
            pcVar12 = DAT_00b69d20 + iVar6 + 1;
            cVar1 = *pcVar12;
            while (cVar1 != ' ') {
                cVar1 = DAT_00b69d20[iVar6 + 2];
                iVar6 = iVar6 + 1;
            }
            DAT_00b69d20[iVar6 + 1] = 0;
            _atoi(pcVar12);
            if (DAT_00b69e30 == 2) {
                __builtin_memcpy(&tmpA,
                    DAT_00b7ca4a + (unsigned int)((unsigned int)DAT_00b6d962 * 0x84f +
                        (unsigned int)*(int *)((char *)DAT_00aa6114 + iVar10 + 0x140)) * 0x20,
                    4);
                if (tmpA == 0) {
                    *(int *)((char *)DAT_00aa6114 + iVar10 + 0x144) = 1;
                }
            } else {
                iVar7 = (int)((unsigned int)*(int *)((char *)DAT_00aa6114 + iVar10 + 0x140) * 0x20);
                __builtin_memcpy(&tmpA, DAT_00b7d6ca + iVar7, 4);
                __builtin_memcpy(&tmpB, DAT_00b8e0aa + iVar7, 4);
                if ((tmpA == 0) && (tmpB == 0)) {
                    *(int *)((char *)DAT_00aa6114 + iVar10 + 0x144) = 1;
                }
            }
            pcVar12 = DAT_00b69d20 + iVar6 + 2;
LAB_004e5455:
            uVar8 = 0xffffffff;
            do {
                pcVar2 = pcVar12;
                if (uVar8 == 0) break;
                uVar8 = uVar8 - 1;
                pcVar2 = pcVar12 + 1;
                cVar1 = *pcVar12;
                pcVar12 = pcVar2;
            } while (cVar1 != '\0');
            uVar8 = ~uVar8;
            pcVar12 = pcVar2 + -uVar8;
            pcVar2 = (char *)DAT_00aa6114 + iVar10;
            for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
                __builtin_memcpy(pcVar2, pcVar12, 4);
                pcVar12 = pcVar12 + 4;
                pcVar2 = pcVar2 + 4;
            }
        } else {
            if (DAT_00b69d20[0] != '&') {
                pcVar12 = DAT_00b69d20;
                goto LAB_004e5455;
            }
            uVar8 = 0xffffffff;
            *(int *)((char *)DAT_00aa6114 + iVar10 + 0x144) = 1;
            pcVar12 = DAT_00b69d20 + 2;
            do {
                pcVar2 = pcVar12;
                if (uVar8 == 0) break;
                uVar8 = uVar8 - 1;
                pcVar2 = pcVar12 + 1;
                cVar1 = *pcVar12;
                pcVar12 = pcVar2;
            } while (cVar1 != '\0');
            uVar8 = ~uVar8;
            pcVar12 = pcVar2 + -uVar8;
            pcVar2 = (char *)DAT_00aa6114 + iVar10;
            for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
                __builtin_memcpy(pcVar2, pcVar12, 4);
                pcVar12 = pcVar12 + 4;
                pcVar2 = pcVar2 + 4;
            }
        }
        for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
            *pcVar2 = *pcVar12;
            pcVar12 = pcVar12 + 1;
            pcVar2 = pcVar2 + 1;
        }
        pcVar12 = _fgets(DAT_00b69d20, 0x100, _File);
        if (pcVar12 == (char *)0) {
            DAT_00aa6138 = local_84;
            FUN_004cc590(_File);
            return;
        }
        uVar8 = 0xffffffff;
        pcVar12 = DAT_00b69d20;
        do {
            if (uVar8 == 0) break;
            uVar8 = uVar8 - 1;
            cVar1 = *pcVar12;
            pcVar12 = pcVar12 + 1;
        } while (cVar1 != '\0');
        if ((DAT_00b69d20 - 2)[~uVar8] == '\n') {
            uVar8 = 0xffffffff;
            pcVar12 = DAT_00b69d20;
            do {
                if (uVar8 == 0) break;
                uVar8 = uVar8 - 1;
                cVar1 = *pcVar12;
                pcVar12 = pcVar12 + 1;
            } while (cVar1 != '\0');
            (DAT_00b69d20 - 2)[~uVar8] = 0;
        }
        uVar8 = 0xffffffff;
        pcVar12 = DAT_00b69d20;
        do {
            pcVar2 = pcVar12;
            if (uVar8 == 0) break;
            uVar8 = uVar8 - 1;
            pcVar2 = pcVar12 + 1;
            cVar1 = *pcVar12;
            pcVar12 = pcVar2;
        } while (cVar1 != '\0');
        uVar8 = ~uVar8;
        pcVar12 = pcVar2 + -uVar8;
        pcVar2 = (char *)DAT_00aa6114 + iVar10 + 0x40;
        for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
            __builtin_memcpy(pcVar2, pcVar12, 4);
            pcVar12 = pcVar12 + 4;
            pcVar2 = pcVar2 + 4;
        }
        iVar10 = iVar10 + 0x148;
        for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
            *pcVar2 = *pcVar12;
            pcVar12 = pcVar12 + 1;
            pcVar2 = pcVar2 + 1;
        }
        local_84 = local_84 + 1;
    } while (1);
}
