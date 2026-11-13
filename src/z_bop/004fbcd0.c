/* SPDX-License-Identifier: MIT */
/* z_bop 004fbcd0 */
extern int FUN_004d6e30(unsigned char, unsigned char, unsigned char);
extern int _atoi(const char *);
extern unsigned short DAT_006697c8;
extern unsigned short DAT_0066d800[];
extern char DAT_00669800[];

int __cdecl FUN_004fbcd0(char *param_1, int param_2, int param_3)
{
    char cVar1;
    int iVar2;
    unsigned int uVar3;
    unsigned int uVar4;
    int iVar5;
    int iVar6;
    int iVar7;
    char *pcVar8;
    int iVar9;
    char *pcVar10;
    char local_100[256];

    uVar3 = 0xffffffff;
    pcVar8 = param_1;
    do {
        if (uVar3 == 0) break;
        uVar3 = uVar3 - 1;
        cVar1 = *pcVar8;
        pcVar8 = pcVar8 + 1;
    } while (cVar1 != '\0');
    iVar9 = ~uVar3 - 1;
    if (*param_1 == '~') {
        param_1 = param_1 + 1;
        iVar5 = 0;
        iVar2 = 0;
        if (0 < iVar9) {
            do {
                cVar1 = *param_1;
                if ((cVar1 == ' ') || (cVar1 == '\0')) {
                    param_1 = param_1 + 1;
                    iVar9 = iVar9 + -1;
                    local_100[iVar5] = '\0';
                    break;
                }
                local_100[iVar5] = cVar1;
                iVar5 = iVar5 + 1;
                param_1 = param_1 + 1;
                iVar9 = iVar9 + -1;
                iVar2 = iVar2 + 1;
            } while (iVar2 < iVar9);
        }
        iVar2 = _atoi(local_100);
        iVar6 = 0;
        iVar5 = 0;
        if (0 < iVar9) {
            do {
                cVar1 = *param_1;
                if ((cVar1 == ' ') || (cVar1 == '\0')) {
                    param_1 = param_1 + 1;
                    iVar9 = iVar9 + -1;
                    local_100[iVar6] = '\0';
                    break;
                }
                local_100[iVar6] = cVar1;
                iVar6 = iVar6 + 1;
                param_1 = param_1 + 1;
                iVar9 = iVar9 + -1;
                iVar5 = iVar5 + 1;
            } while (iVar5 < iVar9);
        }
        iVar5 = _atoi(local_100);
        iVar7 = 0;
        iVar6 = 0;
        if (0 < iVar9) {
            do {
                cVar1 = *param_1;
                if ((cVar1 == ' ') || (cVar1 == '\0')) {
                    local_100[iVar7] = '\0';
                    param_1 = param_1 + 1;
                    break;
                }
                local_100[iVar7] = cVar1;
                iVar7 = iVar7 + 1;
                param_1 = param_1 + 1;
                iVar9 = iVar9 + -1;
                iVar6 = iVar6 + 1;
            } while (iVar6 < iVar9);
        }
        iVar9 = _atoi(local_100);
        iVar9 = FUN_004d6e30((unsigned char)iVar2, (unsigned char)iVar5, (unsigned char)iVar9);
        DAT_006697c8 = (unsigned short)iVar9;
    }
    iVar9 = param_3 + param_2 * 0x20;
    DAT_0066d800[iVar9] = DAT_006697c8;
    uVar3 = 0xffffffff;
    do {
        pcVar8 = param_1;
        if (uVar3 == 0) break;
        uVar3 = uVar3 - 1;
        pcVar8 = param_1 + 1;
        cVar1 = *param_1;
        param_1 = pcVar8;
    } while (cVar1 != '\0');
    uVar3 = ~uVar3;
    pcVar8 = pcVar8 + -uVar3;
    pcVar10 = DAT_00669800 + iVar9 * 0x100;
    for (uVar4 = uVar3 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
        *(int *)pcVar10 = *(int *)pcVar8;
        pcVar8 = pcVar8 + 4;
        pcVar10 = pcVar10 + 4;
    }
    for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
        *pcVar10 = *pcVar8;
        pcVar8 = pcVar8 + 1;
        pcVar10 = pcVar10 + 1;
    }
    return 1;
}
