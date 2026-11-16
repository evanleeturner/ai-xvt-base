/* SPDX-License-Identifier: MIT */
/* z_bop 004df250 */
extern void *DAT_00b6a248;
extern char DAT_00b69d20[];
extern char DAT_00b69d1e;
extern unsigned int DAT_00b69e34;
extern char DAT_0051c4bc[];
extern char s__d__d__d_0052c164[];

extern void __cdecl _free(void *);
extern void *__cdecl _malloc(unsigned int);
extern char *__cdecl _fgets(char *, int, void *);
extern int __cdecl _atoi(const char *);
extern int __cdecl _sscanf(const char *, const char *, ...);
extern void *__cdecl FUN_004cc4a0(char *, char *);
extern void __cdecl FUN_004cc590(void *);

unsigned int __cdecl FUN_004df250(char *param_1)
{
    char cVar1;
    void *_File;
    char *pcVar2;
    unsigned int uVar3;
    unsigned int uVar4;
    int iVar5;
    unsigned int *puVar6;
    unsigned int *puVar7;

    if (DAT_00b6a248 != (void *)0x0) {
        _free(DAT_00b6a248);
        DAT_00b6a248 = (void *)0x0;
    }
    _File = FUN_004cc4a0(param_1, DAT_0051c4bc);
    if (_File == (void *)0x0) {
        return 0;
    }
    pcVar2 = _fgets(DAT_00b69d20, 0xff, _File);
    if (pcVar2 == (char *)0x0) {
        FUN_004cc590(_File);
        return 0;
    }
    uVar3 = (unsigned int)_atoi(DAT_00b69d20);
    DAT_00b6a248 = _malloc(uVar3 * 300u);
    if (DAT_00b6a248 == (void *)0x0) {
        return 0;
    }
    puVar6 = (unsigned int *)DAT_00b6a248;
    for (uVar4 = (uVar3 * 300u) >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
        *puVar6 = 0;
        puVar6 = puVar6 + 1;
    }
    for (iVar5 = 0; iVar5 != 0; iVar5 = iVar5 - 1) {
        *(char *)puVar6 = 0;
        puVar6 = (unsigned int *)((int)puVar6 + 1);
    }
    DAT_00b69e34 = 0;
    for (;;) {
        if (uVar3 <= DAT_00b69e34) {
            FUN_004cc590(_File);
            return 1;
        }
        do {
            pcVar2 = _fgets(DAT_00b69d20, 0xff, _File);
            if (pcVar2 == (char *)0x0) {
                FUN_004cc590(_File);
                return 1;
            }
        } while ((*pcVar2 == '/') && (pcVar2[1] == '/'));
        uVar4 = 0xffffffffu;
        pcVar2 = DAT_00b69d20;
        do {
            if (uVar4 == 0) break;
            uVar4 = uVar4 - 1;
            cVar1 = *pcVar2;
            pcVar2 = pcVar2 + 1;
        } while (cVar1 != '\0');
        if ((&DAT_00b69d1e)[~uVar4] == '\n') {
            uVar4 = 0xffffffffu;
            pcVar2 = DAT_00b69d20;
            do {
                if (uVar4 == 0) break;
                uVar4 = uVar4 - 1;
                cVar1 = *pcVar2;
                pcVar2 = pcVar2 + 1;
            } while (cVar1 != '\0');
            (&DAT_00b69d1e)[~uVar4] = 0;
        }
        puVar6 = (unsigned int *)DAT_00b69d20;
        puVar7 = (unsigned int *)DAT_00b6a248 + DAT_00b69e34 * 0x4bu;
        for (iVar5 = 0x20; iVar5 != 0; iVar5 = iVar5 - 1) {
            *puVar7 = *puVar6;
            puVar6 = puVar6 + 1;
            puVar7 = puVar7 + 1;
        }
        do {
            pcVar2 = _fgets(DAT_00b69d20, 0xff, _File);
            if (pcVar2 == (char *)0x0) {
                *(char *)((unsigned int *)DAT_00b6a248 + DAT_00b69e34 * 0x4bu) = 0;
                FUN_004cc590(_File);
                return 1;
            }
        } while ((*pcVar2 == '/') && (pcVar2[1] == '/'));
        iVar5 = _sscanf(DAT_00b69d20, s__d__d__d_0052c164,
                        (unsigned int *)DAT_00b6a248 + DAT_00b69e34 * 0x4bu + 0x48,
                        (unsigned int *)DAT_00b6a248 + DAT_00b69e34 * 0x4bu + 0x49,
                        (unsigned int *)DAT_00b6a248 + DAT_00b69e34 * 0x4bu + 0x4a);
        if (iVar5 != 3) {
            puVar6 = (unsigned int *)DAT_00b6a248 + DAT_00b69e34 * 0x4bu;
            for (iVar5 = 0x4b; iVar5 != 0; iVar5 = iVar5 - 1) {
                *puVar6 = 0;
                puVar6 = puVar6 + 1;
            }
            FUN_004cc590(_File);
            return 1;
        }
        do {
            pcVar2 = _fgets(DAT_00b69d20, 0xff, _File);
            if (pcVar2 == (char *)0x0) {
                FUN_004cc590(_File);
                return 1;
            }
        } while ((*pcVar2 == '/') && (pcVar2[1] == '/'));
        uVar4 = 0xffffffffu;
        pcVar2 = DAT_00b69d20;
        do {
            if (uVar4 == 0) break;
            uVar4 = uVar4 - 1;
            cVar1 = *pcVar2;
            pcVar2 = pcVar2 + 1;
        } while (cVar1 != '\0');
        if ((&DAT_00b69d1e)[~uVar4] == '\n') {
            uVar4 = 0xffffffffu;
            pcVar2 = DAT_00b69d20;
            do {
                if (uVar4 == 0) break;
                uVar4 = uVar4 - 1;
                cVar1 = *pcVar2;
                pcVar2 = pcVar2 + 1;
            } while (cVar1 != '\0');
            (&DAT_00b69d1e)[~uVar4] = 0;
        }
        puVar6 = (unsigned int *)DAT_00b69d20;
        puVar7 = (unsigned int *)DAT_00b6a248 + DAT_00b69e34 * 0x4bu + 0x20;
        for (iVar5 = 8; iVar5 != 0; iVar5 = iVar5 - 1) {
            *puVar7 = *puVar6;
            puVar6 = puVar6 + 1;
            puVar7 = puVar7 + 1;
        }
        do {
            pcVar2 = _fgets(DAT_00b69d20, 0xff, _File);
            if (pcVar2 == (char *)0x0) {
                FUN_004cc590(_File);
                return 1;
            }
        } while ((*pcVar2 == '/') && (pcVar2[1] == '/'));
        uVar4 = 0xffffffffu;
        pcVar2 = DAT_00b69d20;
        do {
            if (uVar4 == 0) break;
            uVar4 = uVar4 - 1;
            cVar1 = *pcVar2;
            pcVar2 = pcVar2 + 1;
        } while (cVar1 != '\0');
        if ((&DAT_00b69d1e)[~uVar4] == '\n') {
            uVar4 = 0xffffffffu;
            pcVar2 = DAT_00b69d20;
            do {
                if (uVar4 == 0) break;
                uVar4 = uVar4 - 1;
                cVar1 = *pcVar2;
                pcVar2 = pcVar2 + 1;
            } while (cVar1 != '\0');
            (&DAT_00b69d1e)[~uVar4] = 0;
        }
        puVar6 = (unsigned int *)DAT_00b69d20;
        puVar7 = (unsigned int *)DAT_00b6a248 + DAT_00b69e34 * 0x4bu + 0x28;
        for (iVar5 = 0x20; iVar5 != 0; iVar5 = iVar5 - 1) {
            *puVar7 = *puVar6;
            puVar6 = puVar6 + 1;
            puVar7 = puVar7 + 1;
        }
        DAT_00b69e34 = DAT_00b69e34 + 1;
    }
}
