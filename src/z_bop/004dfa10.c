/* SPDX-License-Identifier: MIT */
/* z_bop 004dfa10 */
/* Prototypes for non-Windows callees. */
extern char *__cdecl __getcwd(char *buf, int size);
extern void *__cdecl _malloc(unsigned int size);
extern int __cdecl __chdir(const char *path);
extern void __cdecl _free(void *p);
extern void __cdecl FUN_004dfc70(unsigned int *param_1, unsigned int *param_2);

/* FindFirstFileA, FindNextFileA and FindClose are Win32 APIs; xvt.h is
   assumed to declare them. */

extern WIN32_FIND_DATAA DAT_00665a90;
extern char DAT_00665abc[]; /* DAT_00665a90 + 0x2c (cFileName) */

unsigned int *__cdecl FUN_004dfa10(LPCSTR param_1)
{
    char cVar1;
    unsigned int *_Memory;
    HANDLE hFindFile;
    unsigned int *puVar2;
    char *pcVar3;
    int iVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    char *pcVar7;
    char *pcVar8;
    char local_100[256];

    __getcwd(local_100, 0x100);
    _Memory = (unsigned int *)_malloc(8);
    if (_Memory == (unsigned int *)0) {
        __chdir(local_100);
        return (unsigned int *)0;
    }
    hFindFile = FindFirstFileA(param_1, &DAT_00665a90);
    if (hFindFile == (HANDLE)0xffffffff) {
        __chdir(local_100);
        *_Memory = 0;
        _Memory[1] = 0;
        return _Memory;
    }
    puVar2 = (unsigned int *)_malloc(8);
    if (puVar2 == (unsigned int *)0) {
        __chdir(local_100);
        _free(_Memory);
        FindClose(hFindFile);
        return (unsigned int *)0;
    }
    uVar5 = 0xffffffff;
    pcVar3 = (char *)&DAT_00665abc;
    do {
        if (uVar5 == 0) break;
        uVar5 = uVar5 - 1;
        cVar1 = *pcVar3;
        pcVar3 = pcVar3 + 1;
    } while (cVar1 != '\0');
    pcVar3 = (char *)_malloc(~uVar5 + 1);
    *puVar2 = (unsigned int)pcVar3;
    if (pcVar3 == (char *)0) {
        __chdir(local_100);
        _free(puVar2);
        _free(_Memory);
        FindClose(hFindFile);
        return (unsigned int *)0;
    }
    uVar5 = 0xffffffff;
    pcVar7 = (char *)&DAT_00665abc;
    do {
        pcVar8 = pcVar7;
        if (uVar5 == 0) break;
        uVar5 = uVar5 - 1;
        pcVar8 = pcVar7 + 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar8;
    } while (cVar1 != '\0');
    uVar5 = ~uVar5;
    pcVar7 = pcVar8 - uVar5;
    for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
        *(unsigned int *)pcVar3 = *(unsigned int *)pcVar7;
        pcVar7 = pcVar7 + 4;
        pcVar3 = pcVar3 + 4;
    }
    for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
        *pcVar3 = *pcVar7;
        pcVar7 = pcVar7 + 1;
        pcVar3 = pcVar3 + 1;
    }
    puVar2[1] = 0;
    *_Memory = (unsigned int)puVar2;
    _Memory[1] = 1;
    iVar4 = FindNextFileA(hFindFile, &DAT_00665a90);
    do {
        if (iVar4 == 0 || (puVar2 = (unsigned int *)_malloc(8), puVar2 == (unsigned int *)0)) goto LAB_004dfc06;
        uVar5 = 0xffffffff;
        pcVar3 = (char *)&DAT_00665abc;
        do {
            if (uVar5 == 0) break;
            uVar5 = uVar5 - 1;
            cVar1 = *pcVar3;
            pcVar3 = pcVar3 + 1;
        } while (cVar1 != '\0');
        pcVar3 = (char *)_malloc(~uVar5 + 1);
        *puVar2 = (unsigned int)pcVar3;
        if (pcVar3 == (char *)0) {
            _free(puVar2);
LAB_004dfc06:
            __chdir(local_100);
            FindClose(hFindFile);
            return _Memory;
        }
        uVar5 = 0xffffffff;
        pcVar7 = (char *)&DAT_00665abc;
        do {
            pcVar8 = pcVar7;
            if (uVar5 == 0) break;
            uVar5 = uVar5 - 1;
            pcVar8 = pcVar7 + 1;
            cVar1 = *pcVar7;
            pcVar7 = pcVar8;
        } while (cVar1 != '\0');
        uVar5 = ~uVar5;
        pcVar7 = pcVar8 - uVar5;
        for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
            *(unsigned int *)pcVar3 = *(unsigned int *)pcVar7;
            pcVar7 = pcVar7 + 4;
            pcVar3 = pcVar3 + 4;
        }
        for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
            *pcVar3 = *pcVar7;
            pcVar7 = pcVar7 + 1;
            pcVar3 = pcVar3 + 1;
        }
        puVar2[1] = 0;
        FUN_004dfc70(_Memory, puVar2);
        iVar4 = FindNextFileA(hFindFile, &DAT_00665a90);
    } while (1);
}
