/* SPDX-License-Identifier: MIT */
/* z_xvt 00488490 */
typedef struct _iobuf FILE;

extern FILE *DAT_00732948;
extern unsigned short DAT_0052a53c;
extern unsigned int DAT_0052a540;
extern char DAT_00523570;
extern char DAT_0052a324;
extern int DAT_00602678;
extern int DAT_006026a0;
extern int DAT_006026c0;
extern int DAT_006026a8;
extern int DAT_00602690;
extern int DAT_006026bc;

unsigned int __cdecl _fread(void *_DstBuf, unsigned int _ElementSize, unsigned int _Count, FILE *_File);
unsigned int __cdecl _fwrite(const void *_Str, unsigned int _Size, unsigned int _Count, FILE *_File);
int __cdecl _fclose(FILE *_File);

void __cdecl FUN_0047ed40(char *param_1, char *param_2, int param_3, int param_4);
void __cdecl FUN_0048f4b0(unsigned int param_1);
unsigned short __cdecl FUN_0048f3c0(unsigned int param_1, int param_2);
void __cdecl FUN_0047f010(int param_1);
int * __cdecl FUN_0048f4f0(unsigned int param_1);
void __cdecl FUN_00486000(int *param_1, int param_2);
void __cdecl FUN_0048f510(void);
int __cdecl FUN_00488830(unsigned int param_1);
int __cdecl FUN_0048ac10(unsigned int *param_1, unsigned int *param_2);

unsigned short __cdecl FUN_00488490(char *param_1)
{
    char cVar1;
    char cVar2;
    int iVar3;
    FILE *pFVar4;
    int *_DstBuf;
    int *piVar5;
    unsigned int uVar6;
    int iVar7;
    int iVar8;
    int iVar9;
    char *pcVar10;
    unsigned int *puVar11;
    unsigned short local_da;
    unsigned int uStack_d8;
    int local_d4;
    unsigned int local_d0[52];
    int f6;
    int f10;
    int tmp;

    FUN_0047ed40(param_1, &DAT_00523570, 1, 0);
    pFVar4 = DAT_00732948;
    if (DAT_00732948 == (FILE *)0) {
        return 0;
    }
    _fread(&local_d4, 1, 4, DAT_00732948);
    iVar7 = local_d4;
    if (local_d4 < 1) {
        local_d4 = (int)(0u - (unsigned int)local_d4);
        _fread(&uStack_d8, 1, 4, pFVar4);
    }
    else {
        local_d4 = 0;
        uStack_d8 = (unsigned int)iVar7;
    }
    if ((DAT_0052a540 < uStack_d8) && (DAT_0052a53c != 0)) {
        FUN_0048f4b0((unsigned int)DAT_0052a53c);
        DAT_0052a53c = 0;
        DAT_0052a540 = 0;
    }
    if (DAT_0052a53c == 0) {
        DAT_0052a53c = FUN_0048f3c0(uStack_d8, 0);
        if (DAT_0052a53c == 0) {
            FUN_0047f010(0);
        }
        DAT_0052a540 = uStack_d8;
    }
    local_da = DAT_0052a53c;
    _DstBuf = FUN_0048f4f0(((uStack_d8 & 0xffffu) << 16) | (unsigned int)DAT_0052a53c);
    _fread(_DstBuf, 1, uStack_d8, pFVar4);
    _fclose(pFVar4);
    piVar5 = (int *)*_DstBuf;
    if (piVar5 != _DstBuf) {
        iVar7 = 0;
        iVar9 = (int)((unsigned int)_DstBuf - (unsigned int)piVar5);
        *_DstBuf = (int)((unsigned int)piVar5 + (unsigned int)iVar9);
        __builtin_memcpy(&f10, (char *)_DstBuf + 10, 4);
        if (f10 != 0) {
            tmp = (int)((unsigned int)f10 + (unsigned int)iVar9);
            __builtin_memcpy((char *)_DstBuf + 10, &tmp, 4);
            __builtin_memcpy(&f6, (char *)_DstBuf + 6, 4);
            if (0 < f6) {
                iVar8 = 0;
                do {
                    __builtin_memcpy(&f10, (char *)_DstBuf + 10, 4);
                    piVar5 = (int *)(f10 + iVar8);
                    iVar3 = *piVar5;
                    if (iVar3 != 0) {
                        *piVar5 = (int)((unsigned int)iVar3 + (unsigned int)iVar9);
                        __builtin_memcpy(&f10, (char *)_DstBuf + 10, 4);
                        FUN_00486000(*(int **)(f10 + iVar8), iVar9);
                    }
                    iVar8 = iVar8 + 4;
                    iVar7 = iVar7 + 1;
                    __builtin_memcpy(&f6, (char *)_DstBuf + 6, 4);
                } while (iVar7 < f6);
            }
        }
    }
    if (local_d4 == 0) {
        uVar6 = 0xffffffff;
        pcVar10 = param_1;
        do {
            if (uVar6 == 0) break;
            uVar6 = uVar6 - 1;
            cVar1 = *pcVar10;
            pcVar10 = pcVar10 + 1;
        } while (cVar1 != '\0');
        cVar1 = param_1[~uVar6 - 2];
        uVar6 = 0xffffffff;
        pcVar10 = param_1;
        do {
            if (uVar6 == 0) break;
            uVar6 = uVar6 - 1;
            cVar2 = *pcVar10;
            pcVar10 = pcVar10 + 1;
        } while (cVar2 != '\0');
        param_1[~uVar6 - 2] = '0';
        FUN_0047ed40(param_1, &DAT_0052a324, 0, 1);
        pFVar4 = DAT_00732948;
        if (DAT_00732948 != (FILE *)0) {
            _fwrite(&uStack_d8, 1, 4, DAT_00732948);
            _fwrite(_DstBuf, 1, uStack_d8, pFVar4);
            _fclose(pFVar4);
        }
        uVar6 = 0xffffffff;
        pcVar10 = param_1;
        do {
            if (uVar6 == 0) break;
            uVar6 = uVar6 - 1;
            cVar2 = *pcVar10;
            pcVar10 = pcVar10 + 1;
        } while (cVar2 != '\0');
        param_1[~uVar6 - 2] = cVar1;
        FUN_0048f510();
        iVar7 = FUN_00488830(uStack_d8);
        uStack_d8 = (unsigned int)iVar7;
        local_da = DAT_0052a53c;
        _DstBuf = FUN_0048f4f0(((uStack_d8 & 0xffffu) << 16) | (unsigned int)DAT_0052a53c);
        piVar5 = (int *)*_DstBuf;
        if (piVar5 != _DstBuf) {
            iVar7 = 0;
            iVar9 = (int)((unsigned int)_DstBuf - (unsigned int)piVar5);
            *_DstBuf = (int)((unsigned int)piVar5 + (unsigned int)iVar9);
            __builtin_memcpy(&f10, (char *)_DstBuf + 10, 4);
            if (f10 != 0) {
                iVar8 = 0;
                tmp = (int)((unsigned int)f10 + (unsigned int)iVar9);
                __builtin_memcpy((char *)_DstBuf + 10, &tmp, 4);
                __builtin_memcpy(&f6, (char *)_DstBuf + 6, 4);
                if (0 < f6) {
                    do {
                        __builtin_memcpy(&f10, (char *)_DstBuf + 10, 4);
                        piVar5 = (int *)(f10 + iVar7);
                        iVar3 = *piVar5;
                        if (iVar3 != 0) {
                            *piVar5 = (int)((unsigned int)iVar3 + (unsigned int)iVar9);
                            __builtin_memcpy(&f10, (char *)_DstBuf + 10, 4);
                            FUN_00486000(*(int **)(f10 + iVar7), iVar9);
                        }
                        iVar7 = iVar7 + 4;
                        iVar8 = iVar8 + 1;
                        __builtin_memcpy(&f6, (char *)_DstBuf + 6, 4);
                    } while (iVar8 < f6);
                }
            }
        }
        local_d4 = -1;
        FUN_0047ed40(param_1, &DAT_0052a324, 0, 1);
        pFVar4 = DAT_00732948;
        if (DAT_00732948 != (FILE *)0) {
            _fwrite(&local_d4, 1, 4, DAT_00732948);
            _fwrite(&uStack_d8, 1, 4, pFVar4);
            _fwrite(_DstBuf, 1, uStack_d8, pFVar4);
            _fclose(pFVar4);
        }
        local_d4 = (int)(0u - (unsigned int)local_d4);
    }
    puVar11 = local_d0;
    for (iVar7 = 0x34; iVar7 != 0; iVar7 = iVar7 + -1) {
        *puVar11 = 0;
        puVar11 = puVar11 + 1;
    }
    iVar9 = 0;
    DAT_00602678 = 0;
    DAT_006026a0 = 0;
    DAT_006026c0 = 0;
    DAT_006026a8 = 0;
    iVar7 = 0;
    DAT_00602690 = 0;
    DAT_006026bc = 0;
    __builtin_memcpy(&f6, (char *)_DstBuf + 6, 4);
    uStack_d8 = (unsigned int)f6 * 4u + 0xeu;
    __builtin_memcpy(&f6, (char *)_DstBuf + 6, 4);
    if (0 < f6) {
        do {
            iVar9 = iVar9 + 4;
            iVar7 = iVar7 + 1;
            __builtin_memcpy(&f10, (char *)_DstBuf + 10, 4);
            iVar8 = FUN_0048ac10(*(unsigned int **)(f10 + -4 + iVar9), local_d0);
            uStack_d8 = uStack_d8 + (unsigned int)iVar8;
            __builtin_memcpy(&f6, (char *)_DstBuf + 6, 4);
        } while (iVar7 < f6);
    }
    FUN_0048f510();
    return local_da;
}
