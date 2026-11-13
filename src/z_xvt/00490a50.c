/* SPDX-License-Identifier: MIT */
/* z_xvt 00490a50 */
extern int DAT_00658654;
extern char DAT_0065865c;
extern int DAT_006a8814;
extern int DAT_006a8810;
extern int DAT_006a86bc;
extern char DAT_006a86c0;
extern char DAT_006a866c;
extern int DAT_006a86b4;
extern int DAT_006a86b8;
extern int DAT_006a86ac;
extern int DAT_006a86b0;

extern int _strncmp(const char *s1, const char *s2, unsigned int n);
extern void *memcpy(void *dest, const void *src, unsigned int n);
extern char *_strncpy(char *dest, const char *src, unsigned int n);

int __cdecl FUN_00490a50(char *param_1, int param_2, int param_3, int param_4, int param_5, int param_6)
{
    int iVar1;
    int *piVar2;
    int iVar3;
    int iVar4;
    int iVar5;
    char *pcStack_4;

    if (DAT_00658654 == 0) {
        return 0;
    }
    if (*param_1 == '\0') {
        return 0;
    }
    iVar3 = 0;
    pcStack_4 = &DAT_0065865c;
    iVar4 = DAT_006a8814 - 1;
    while (iVar4 >= 0) {
        iVar5 = iVar4 >> 1;
        iVar1 = _strncmp(pcStack_4 + iVar5 * 0x145, param_1, 0x40);
        if (iVar1 == 0) {
            iVar5 = iVar5 + iVar3;
            goto LAB_00490ae6;
        }
        if (iVar4 < 1) break;
        if (iVar1 < 0) {
            iVar3 = iVar5 + 1 + iVar3;
            iVar4 = iVar4 - (iVar5 + 1);
            pcStack_4 = pcStack_4 + iVar5 * 0x145 + 0x145;
        }
        else {
            iVar4 = iVar5 - 1;
        }
    }
    iVar5 = -1;
LAB_00490ae6:
    if (iVar5 == -1) {
        return 0;
    }
    iVar4 = 0;
    if (DAT_006a8810 > 0) {
        piVar2 = &DAT_006a86bc;
        do {
            if (*piVar2 < param_4) break;
            piVar2 = piVar2 + 0x15;
            iVar4 = iVar4 + 1;
        } while (iVar4 < DAT_006a8810);
    }
    if ((iVar4 == DAT_006a8810) && (iVar4 == 4)) {
        return 0;
    }
    memcpy(&DAT_006a86c0 + iVar4 * 0x54, &DAT_006a866c + iVar4 * 0x54, (DAT_006a8810 - iVar4) * 0x54);
    _strncpy(&DAT_006a866c + iVar4 * 0x54, param_1, 0x40);
    (&DAT_006a86b4)[iVar4 * 0x15] = param_3;
    (&DAT_006a86b8)[iVar4 * 0x15] = param_2;
    (&DAT_006a86bc)[iVar4 * 0x15] = param_4;
    (&DAT_006a86ac)[iVar4 * 0x15] = param_5;
    (&DAT_006a86b0)[iVar4 * 0x15] = param_6;
    DAT_006a8810 = DAT_006a8810 + 1;
    if (4 < DAT_006a8810) {
        DAT_006a8810 = 4;
    }
    return 1;
}
