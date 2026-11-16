/* SPDX-License-Identifier: MIT */
/* z_bop 004e5590 */
/* Globals (decompiler placeholders kept). DAT_00b6a5aa and DAT_00b6a5ae are
   at addresses == 2 (mod 4): accessed only through __builtin_memcpy. */
extern unsigned int DAT_00aa6138;
extern unsigned int DAT_00aa6114;
extern int DAT_00b6a5aa;
extern int DAT_00b6a5ae;
extern char DAT_00b69d20[];
extern char *PTR_s_train_0052c1e8[];
extern char s__s__s_0052c300[];
extern char DAT_0051a854;
extern int DAT_0052dc84;
extern unsigned char *PTR_DAT_0052da78;

/* Prototypes. Return types of the FUN_ helpers are unused here; stream handle type is unresolved (void *). */
int __cdecl _sprintf(char *, const char *, ...);
void * __cdecl FUN_004cc4a0(char *, char *);
void __cdecl FUN_004cc660(void *, short *);
void __cdecl FUN_004cc5c0(void *, int, int);
void __cdecl FUN_004cc6c0(void *, unsigned char *, int);
char * __cdecl _fgets(char *, int, void *);
int __cdecl _atoi(const char *);
int __cdecl __isctype(int, int);
void __cdecl FUN_004cc590(void *);

void __cdecl FUN_004e5590(unsigned char *param_1)
{
    char cVar1;
    unsigned char bVar2;
    char *pcVar3;
    unsigned int uVar4;
    int iVar5;
    int *piVar6;
    unsigned int uVar7;
    unsigned int uVar8;
    unsigned int _C;
    unsigned char *pbVar9;
    short local_a;
    void *local_8;
    unsigned int local_4;
    int iSel;
    int iKey;

    if (param_1 != (unsigned char *)0) {
        uVar8 = 0;
        pbVar9 = param_1;
        for (iVar5 = 0x400; iVar5 != 0; iVar5 = iVar5 - 1) {
            pbVar9[0] = 0;
            pbVar9[1] = 0;
            pbVar9[2] = 0;
            pbVar9[3] = 0;
            pbVar9 = pbVar9 + 4;
        }
        if (DAT_00aa6138 != 0) {
            piVar6 = (int *)(DAT_00aa6114 + 0x140);
            do {
                __builtin_memcpy(&iSel, &DAT_00b6a5aa, 4);
                __builtin_memcpy(&iKey, (unsigned char *)&DAT_00b6a5ae + (unsigned int)iSel * 4u, 4);
                if (*piVar6 == iKey) break;
                piVar6 = piVar6 + 0x52;
                uVar8 = uVar8 + 1;
            } while (uVar8 < DAT_00aa6138);
        }
        if (uVar8 < DAT_00aa6138) {
            __builtin_memcpy(&iSel, &DAT_00b6a5aa, 4);
            _sprintf(DAT_00b69d20, s__s__s_0052c300, PTR_s_train_0052c1e8[iSel],
                     (char *)(uVar8 * 0x148u + DAT_00aa6114));
            local_8 = FUN_004cc4a0(DAT_00b69d20, &DAT_0051a854);
            if (local_8 != (void *)0) {
                __builtin_memcpy(&iSel, &DAT_00b6a5aa, 4);
                if (iSel == 2) goto text_mode;
                __builtin_memcpy(&iSel, &DAT_00b6a5aa, 4);
                if (iSel == 4) goto text_mode;
                __builtin_memcpy(&iSel, &DAT_00b6a5aa, 4);
                if (iSel == 5) {
text_mode:
                    _fgets(DAT_00b69d20, 0x100, local_8);
                    for (iVar5 = _atoi(DAT_00b69d20); iVar5 != 0; iVar5 = iVar5 - 1) {
                        _fgets(DAT_00b69d20, 0x100, local_8);
                    }
                    uVar8 = 0;
                    pbVar9 = param_1;
                    while (pcVar3 = _fgets(DAT_00b69d20, 0x100, local_8), pcVar3 != (char *)0) {
                        uVar7 = 0xffffffffu;
                        pcVar3 = DAT_00b69d20;
                        do {
                            if (uVar7 == 0) break;
                            uVar7 = uVar7 - 1;
                            cVar1 = *pcVar3;
                            pcVar3 = pcVar3 + 1;
                        } while (cVar1 != '\0');
                        local_4 = ~uVar7 - 1;
                        uVar7 = 0;
                        if (local_4 != 0) {
                            do {
                                bVar2 = *(unsigned char *)(DAT_00b69d20 + uVar7);
                                _C = (unsigned int)bVar2;
                                if (DAT_0052dc84 < 2) {
                                    uVar4 = *(unsigned short *)(PTR_DAT_0052da78 + _C * 2) & 0x157;
                                } else {
                                    uVar4 = __isctype((int)_C, 0x157);
                                }
                                if ((uVar4 != 0) || (_C == 10)) {
                                    *pbVar9 = bVar2;
                                    pbVar9 = pbVar9 + 1;
                                    uVar8 = uVar8 + 1;
                                    if (0xffe < uVar8) goto LAB_004e57e4;
                                }
                                uVar7 = uVar7 + 1;
                            } while (uVar7 < local_4);
                        }
                        if (0xffe < uVar8) break;
                    }
LAB_004e57e4:
                    param_1[uVar8] = 0;
                } else {
                    FUN_004cc660(local_8, &local_a);
                    if ((local_a == 0xe) || (local_a == 0xd)) {
                        FUN_004cc5c0(local_8, -0x1000, 2);
                        FUN_004cc6c0(local_8, param_1, 0x1000);
                        param_1[0xfff] = 0;
                    } else if (local_a == 0xc) {
                        FUN_004cc5c0(local_8, -0x400, 2);
                        FUN_004cc6c0(local_8, param_1, 0x400);
                        param_1[0x3ff] = 0;
                    }
                }
                FUN_004cc590(local_8);
            }
        }
    }
    return;
}
