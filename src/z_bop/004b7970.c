/* SPDX-License-Identifier: MIT */
/* z_bop 004b7970 */
int __cdecl _strncmp(const char *s1, const char *s2, unsigned int n);

extern int DAT_00aa7205;
extern void *DAT_00aa7201;

void __cdecl FUN_004b7970(char *param_1)
{
    int iVar1;
    int iVar2;
    int iVar3;
    int iVar4;
    unsigned int *puVar5;
    unsigned int *puVar6;

    iVar3 = 0;
    if (0 < DAT_00aa7205) {
        iVar4 = 0;
        do {
            iVar1 = _strncmp(param_1, (char *)(DAT_00aa7201 + iVar4), 0x40);
            if (iVar1 < 0) break;
            iVar4 = iVar4 + 0x44;
            iVar3 = iVar3 + 1;
        } while (iVar3 < DAT_00aa7205);
    }
    if (iVar3 < DAT_00aa7205) {
        iVar4 = DAT_00aa7205 * 0x44;
        iVar1 = DAT_00aa7205 - iVar3;
        do {
            puVar6 = (unsigned int *)(DAT_00aa7201 + iVar4);
            iVar4 = iVar4 + -0x44;
            puVar5 = puVar6 + -0x11;
            for (iVar2 = 0x11; iVar2 != 0; iVar2 = iVar2 + -1) {
                *puVar6 = *puVar5;
                puVar5 = puVar5 + 1;
                puVar6 = puVar6 + 1;
            }
            iVar1 = iVar1 + -1;
        } while (iVar1 != 0);
    }
    puVar5 = (unsigned int *)(iVar3 * 0x44 + DAT_00aa7201);
    for (iVar4 = 0x11; iVar4 != 0; iVar4 = iVar4 + -1) {
        *puVar5 = *(unsigned int *)param_1;
        param_1 = param_1 + 4;
        puVar5 = puVar5 + 1;
    }
    DAT_00aa7205 = DAT_00aa7205 + 1;
    return;
}
