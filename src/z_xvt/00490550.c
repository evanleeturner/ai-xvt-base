/* SPDX-License-Identifier: MIT */
/* z_xvt 00490550 */
extern int *DAT_00658654;
extern char DAT_0065865c[];
extern int DAT_006a8814;
extern int DAT_006a860c[];
extern int DAT_006a866c;

int __cdecl _strncmp(const char *, const char *, unsigned int);
char *__cdecl _strncpy(char *, const char *, unsigned int);
int *__cdecl FUN_00417a40(int *param_1, char *param_2, int param_3);

bool __cdecl FUN_00490550(char *param_1, char *param_2, int param_3)
{
    int iVar1;
    int *piVar2;
    unsigned int *puVar3;
    int iVar4;
    int iVar5;
    char *_Str2;
    unsigned int *puVar6;
    int iVar7;
    unsigned int *puVar8;
    char *local_14c;
    unsigned int local_150[0x52];
    int iStack_10;
    int *local_8;

    if (*param_1 == '\0') {
        return 0;
    }
    if (*param_2 == '\0') {
        return 0;
    }
    if (999 < DAT_006a8814) {
        return 0;
    }
    if (DAT_00658654 == (int *)0) {
        return 0;
    }
    iVar4 = 0;
    local_14c = DAT_0065865c;
    iVar5 = (int)((unsigned int)DAT_006a8814 - 1u);
    while (-1 < iVar5) {
        iVar7 = iVar5 >> 1;
        iVar1 = _strncmp(local_14c + (int)((unsigned int)iVar7 * 0x145u), param_2, 0x40);
        if (iVar1 == 0) {
            iVar7 = (int)((unsigned int)iVar7 + (unsigned int)iVar4);
            goto LAB_00490624;
        }
        if (iVar5 < 1) {
            break;
        }
        if (iVar1 < 0) {
            iVar4 = (int)(((unsigned int)iVar7 + 1u) + (unsigned int)iVar4);
            iVar5 = (int)((unsigned int)iVar5 - ((unsigned int)iVar7 + 1u));
            local_14c = local_14c + (int)((unsigned int)iVar7 * 0x145u + 0x145u);
        } else {
            iVar5 = (int)((unsigned int)iVar7 - 1u);
        }
    }
    iVar7 = -1;
LAB_00490624:
    if (iVar7 != -1) {
        return 0;
    }
    iVar5 = 0;
    local_8 = FUN_00417a40(DAT_00658654, param_1, param_3);
    if (local_8 == (int *)0) {
        return 0;
    }
    (*(void (__stdcall **)(int *, int))(*local_8 + 0x34))(local_8, 0);
    _strncpy((char *)local_150, param_2, 0x40);
    _strncpy((char *)local_150 + 0x40, param_1, 0x100);
    ((char *)local_150)[0xFF] = 0;
    ((char *)local_150)[0x144] = 0;
    if (0 < DAT_006a8814) {
        _Str2 = DAT_0065865c;
        do {
            iVar4 = _strncmp((char *)local_150, _Str2, 0x40);
            if (iVar4 < 0) {
                break;
            }
            _Str2 = _Str2 + 0x145;
            iVar5 = (int)((unsigned int)iVar5 + 1u);
        } while (iVar5 < DAT_006a8814);
        if (iVar5 < DAT_006a8814) {
            iVar4 = (int)((unsigned int)DAT_006a8814 - (unsigned int)iVar5);
            puVar3 = (unsigned int *)(DAT_0065865c + (int)((unsigned int)DAT_006a8814 * 0x145u));
            do {
                puVar6 = (unsigned int *)((char *)puVar3 - 0x145);
                puVar8 = puVar3;
                for (iVar7 = 0x51; iVar7 != 0; iVar7 = (int)((unsigned int)iVar7 - 1u)) {
                    __builtin_memcpy(puVar8, puVar6, 4);
                    puVar6 = puVar6 + 1;
                    puVar8 = puVar8 + 1;
                }
                *(char *)puVar8 = *(char *)puVar6;
                iVar4 = (int)((unsigned int)iVar4 - 1u);
                puVar3 = (unsigned int *)((char *)puVar3 - 0x145);
            } while (iVar4 != 0);
        }
    }
    puVar3 = local_150;
    puVar6 = (unsigned int *)(DAT_0065865c + (int)((unsigned int)iVar5 * 0x145u));
    for (iVar4 = 0x51; iVar4 != 0; iVar4 = (int)((unsigned int)iVar4 - 1u)) {
        __builtin_memcpy(puVar6, puVar3, 4);
        puVar3 = puVar3 + 1;
        puVar6 = puVar6 + 1;
    }
    *(char *)puVar6 = *(char *)puVar3;
    piVar2 = DAT_006a860c;
    DAT_006a8814 = (int)((unsigned int)DAT_006a8814 + 1u);
    do {
        if (iVar5 <= *piVar2) {
            *piVar2 = (int)((unsigned int)*piVar2 + 1u);
        }
        piVar2 = piVar2 + 3;
    } while (piVar2 < &DAT_006a866c);
    __builtin_memcpy(&iStack_10, (char *)local_150 + 0x140, 4);
    return (bool)((int)(1u - (unsigned int)(iStack_10 == 0)));
}
