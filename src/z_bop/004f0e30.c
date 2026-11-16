/* SPDX-License-Identifier: MIT */
/* z_bop 004f0e30 */
HRSRC  __stdcall FindResourceA(HMODULE hModule, LPCSTR lpName, LPCSTR lpType);
HGLOBAL __stdcall LoadResource(HMODULE hModule, HRSRC hResInfo);
LPVOID __stdcall LockResource(HGLOBAL hResData);
HFILE  __stdcall _lopen(LPCSTR lpPathName, int iReadWrite);
UINT   __stdcall _lread(HFILE hFile, LPVOID lpBuffer, UINT uBytes);
HFILE  __stdcall _lclose(HFILE hFile);

/* Odd-address 0x400-byte table, plus Ghidra's separate placeholders. */
extern unsigned char DAT_00aa7b9e[1024];
extern unsigned char DAT_00aa7f9a;
extern unsigned char DAT_00aa7f9b;
extern unsigned char DAT_00aa7f9c;

unsigned char * __cdecl FUN_004f0e30(int *param_1, LPCSTR param_2)
{
    unsigned char uVar1;
    HRSRC hResInfo;
    HGLOBAL hResData;
    unsigned int *puVar2;
    unsigned int uVar3;
    HFILE hFile;
    unsigned char *puVar4;
    unsigned char *puVar6;
    _Bool bVar9;
    /* One 0x43c-byte frame holding Ghidra's locals:
       local_43c +0x00, local_438 +0x04, local_42a +0x12, local_418 +0x24,
       uStack_414 +0x28, local_410 +0x2c, local_400 +0x3c. */
    unsigned int frame_buf[0x43c / 4];
    unsigned char *frame = (unsigned char *)frame_buf;
    unsigned char *local_43c = frame;
    int *local_438 = (int *)(frame + 4);
    unsigned short *local_42a = (unsigned short *)(frame + 0x12);
    int *local_418 = (int *)(frame + 0x24);
    unsigned char *uStack_414 = frame + 0x28;
    unsigned char *local_410 = frame + 0x2c;
    unsigned char *local_400 = frame + 0x3c;

    puVar4 = local_400;
    uVar3 = 0;
    do {
        puVar6 = puVar4 + 4;
        *puVar4 = (unsigned char)((((uVar3 & 0xe0) >> 5) * 0xff) / 7);
        puVar4[1] = (unsigned char)((((uVar3 & 0x1c) >> 2) * 0xff) / 7);
        puVar4[2] = (unsigned char)(((uVar3 & 3) * 0xff) / 3);
        puVar4[3] = 0;
        puVar4 = puVar6;
        uVar3 = uVar3 + 1;
    } while (puVar6 < frame + 0x43c);

    if (param_2 == (LPCSTR)0) goto LAB_004f0fb3;

    hResInfo = FindResourceA((HMODULE)0, param_2, (LPCSTR)2);
    if (hResInfo == (HRSRC)0) {
        if ((param_2 != (LPCSTR)0) && (hFile = _lopen(param_2, 0), hFile != -1)) {
            _lread(hFile, local_410, 0xe);
            _lread(hFile, local_438, 0x28);
            _lread(hFile, local_400, 0x400);
            _lclose(hFile);
            if (local_438[0] == 0x28) {
                if (*local_42a < 9) {
                    if (*local_418 == 0) {
                        *local_418 = (int)(1u << ((unsigned char)*local_42a & 0x1f));
                    }
                } else {
                    *local_418 = 0;
                }
            } else {
                *local_418 = 0;
            }
            if (0 < *local_418) {
                puVar4 = local_400;
                do {
                    uVar1 = *puVar4;
                    *puVar4 = puVar4[2];
                    *local_418 = (int)((unsigned int)*local_418 - 1u);
                    puVar4[2] = uVar1;
                    puVar4 = puVar4 + 4;
                } while (*local_418 != 0);
            }
        }
        goto LAB_004f0fb3;
    }

    hResData = LoadResource((HMODULE)0, hResInfo);
    puVar2 = (unsigned int *)LockResource(hResData);
    if (((puVar2 == (unsigned int *)0) || (*puVar2 < 0x28)) ||
        (8 < *(unsigned short *)((int)puVar2 + 0xe))) {
        uVar3 = 0;
LAB_004f0ef3:
        bVar9 = (uVar3 == 0);
    } else {
        uVar3 = puVar2[8];
        bVar9 = 0;
        if (uVar3 == 0) {
            uVar3 = 1u << ((unsigned char)*(unsigned short *)((int)puVar2 + 0xe) & 0x1f);
            goto LAB_004f0ef3;
        }
    }
    if (!bVar9 && -1 < (int)uVar3) {
        puVar4 = (unsigned char *)((int)puVar2 + *puVar2 + 2);
        puVar6 = local_400;
        do {
            uVar3 = uVar3 - 1;
            *puVar6 = *puVar4;
            puVar6[1] = puVar4[-1];
            puVar6[2] = puVar4[-2];
            puVar6[3] = 0;
            puVar4 = puVar4 + 4;
            puVar6 = puVar6 + 4;
        } while (uVar3 != 0);
    }

LAB_004f0fb3:
    puVar4 = local_43c;
    (*(int (__stdcall *)(int *, int, void *, void *, int))
        *(int *)(*param_1 + 0x14))(param_1, 0x44, local_400, puVar4, 0);
    if (puVar4 != (unsigned char *)0) {
        __builtin_memcpy(DAT_00aa7b9e, uStack_414, 0x400);
        DAT_00aa7b9e[0] = 0;
        DAT_00aa7b9e[1] = 0;
        DAT_00aa7b9e[2] = 0;
        DAT_00aa7f9a = 0xff;
        DAT_00aa7f9b = 0xff;
        DAT_00aa7f9c = 0xff;
    }
    return puVar4;
}
