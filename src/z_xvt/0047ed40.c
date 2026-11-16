/* SPDX-License-Identifier: MIT */
/* z_xvt 0047ed40 */
typedef struct _iobuf FILE;

extern char DAT_0070ab20[];
extern unsigned int DAT_005281c4;
extern FILE *DAT_00732948;

FILE *__cdecl _fopen(const char *, const char *);
char FUN_004be6c0(void);
char FUN_0047e680(void);
void FUN_0047f010(int);

unsigned int __cdecl FUN_0047ed40(char *param_1, char *param_2, int param_3, int param_4)
{
    char uVar1;
    char cVar2;
    unsigned int uVar3;
    unsigned int uVar4;
    int iVar5;
    char *pcVar6;
    char *pcVar7;
    char *pcVar8;
    unsigned int tmp;

    uVar3 = 0xffffffff;
    pcVar6 = param_1;
    do {
        pcVar8 = pcVar6;
        if (uVar3 == 0) break;
        uVar3 = uVar3 - 1;
        pcVar8 = pcVar6 + 1;
        cVar2 = *pcVar6;
        pcVar6 = pcVar8;
    } while (cVar2 != '\0');
    uVar3 = ~uVar3;
    pcVar6 = pcVar8 - uVar3;
    pcVar8 = DAT_0070ab20;
    for (uVar4 = uVar3 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
        __builtin_memcpy(&tmp, pcVar6, 4);
        __builtin_memcpy(pcVar8, &tmp, 4);
        pcVar6 = pcVar6 + 4;
        pcVar8 = pcVar8 + 4;
    }
    for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
        *pcVar8 = *pcVar6;
        pcVar6 = pcVar6 + 1;
        pcVar8 = pcVar8 + 1;
    }
    do {
        iVar5 = 4;
        if (param_4 != 2) {
            while (iVar5 != 0) {
                DAT_00732948 = _fopen(param_1, param_2);
                iVar5 = iVar5 + -1;
                if (DAT_00732948 != (FILE *)0) {
                    return 1;
                }
            }
        }
        if (param_4 != 1) {
            __builtin_memcpy(DAT_0070ab20, &DAT_005281c4, 4);
            uVar1 = FUN_004be6c0();
            uVar3 = 0xffffffff;
            DAT_0070ab20[0] = uVar1;
            pcVar6 = param_1;
            do {
                pcVar8 = pcVar6;
                if (uVar3 == 0) break;
                uVar3 = uVar3 - 1;
                pcVar8 = pcVar6 + 1;
                cVar2 = *pcVar6;
                pcVar6 = pcVar8;
            } while (cVar2 != '\0');
            uVar3 = ~uVar3;
            iVar5 = -1;
            pcVar6 = DAT_0070ab20;
            do {
                pcVar7 = pcVar6;
                if (iVar5 == 0) break;
                iVar5 = iVar5 + -1;
                pcVar7 = pcVar6 + 1;
                cVar2 = *pcVar6;
                pcVar6 = pcVar7;
            } while (cVar2 != '\0');
            pcVar6 = pcVar8 - uVar3;
            pcVar8 = pcVar7 - 1;
            for (uVar4 = uVar3 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
                __builtin_memcpy(&tmp, pcVar6, 4);
                __builtin_memcpy(pcVar8, &tmp, 4);
                pcVar6 = pcVar6 + 4;
                pcVar8 = pcVar8 + 4;
            }
            for (uVar3 = uVar3 & 3; iVar5 = 4, uVar3 != 0; uVar3 = uVar3 - 1) {
                *pcVar8 = *pcVar6;
                pcVar6 = pcVar6 + 1;
                pcVar8 = pcVar8 + 1;
            }
            while (iVar5 != 0) {
                DAT_00732948 = _fopen(DAT_0070ab20, param_2);
                iVar5 = iVar5 + -1;
                if (DAT_00732948 != (FILE *)0) {
                    return 1;
                }
            }
        }
        if (param_3 == 0) {
            DAT_00732948 = (FILE *)0;
            return 0;
        }
        while ((cVar2 = FUN_0047e680()) != 'R' && cVar2 != 'r') {
            if (cVar2 == 'F' || cVar2 == 'f') {
                FUN_0047f010(1);
                DAT_00732948 = (FILE *)0;
                return 0;
            }
        }
    } while (1);
}
