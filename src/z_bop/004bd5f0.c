/* SPDX-License-Identifier: MIT */
/* z_bop 004bd5f0 */
extern HFILE __cdecl FUN_004cc4a0(char *param_1, char *param_2);
extern uint __cdecl FUN_004cc590(HFILE param_1);
extern char * __cdecl _fgets(char *param_1, int param_2, HFILE param_3);
extern int __cdecl _atoi(char *param_1);

extern char s_joystick_txt_0052addc[];
extern char DAT_0051c4bc[];
extern int DAT_00bb72a0;
extern char DAT_00b69d20[];
extern char DAT_00b69d1e[];
extern char DAT_00bb2820[];
extern char DAT_00bb2821[];
extern char DAT_00bb2835[];

int __stdcall FUN_004bd5f0(void)
{
    HFILE _File;
    char *pcVar2;
    int iVar3;
    uint uVar4;
    uint uVar5;
    char *pcVar6;
    char *pcVar7;
    char local_100[256];
    char cVar1;

    _File = FUN_004cc4a0(s_joystick_txt_0052addc, DAT_0051c4bc);
    if (_File == (HFILE)0x0) {
        return 0;
    }
    DAT_00bb72a0 = 0;
    do {
        pcVar2 = _fgets(DAT_00b69d20, 0x80, _File);
        if (pcVar2 == (char *)0x0) {
            FUN_004cc590(_File);
            return 1;
        }
        uVar4 = 0xffffffff;
        pcVar6 = DAT_00b69d20;
        do {
            if (uVar4 == 0) break;
            uVar4 = uVar4 - 1;
            cVar1 = *pcVar6;
            pcVar6 = pcVar6 + 1;
        } while (cVar1 != '\0');
        if (DAT_00b69d1e[~uVar4] == '\n') {
            uVar4 = 0xffffffff;
            pcVar6 = DAT_00b69d20;
            do {
                if (uVar4 == 0) break;
                uVar4 = uVar4 - 1;
                cVar1 = *pcVar6;
                pcVar6 = pcVar6 + 1;
            } while (cVar1 != '\0');
            DAT_00b69d1e[~uVar4] = 0;
        }
        cVar1 = *pcVar2;
        for (iVar3 = 0; ((cVar1 != ' ' && (cVar1 = *pcVar2, cVar1 != '\0')) && (iVar3 < 0x100));
            iVar3 = iVar3 + 1) {
            pcVar2 = pcVar2 + 1;
            local_100[iVar3] = cVar1;
            cVar1 = *pcVar2;
        }
        local_100[iVar3] = '\0';
        pcVar2 = pcVar2 + 1;
        iVar3 = _atoi(local_100);
        DAT_00bb2820[DAT_00bb72a0 * 0x95] = (char)iVar3;
        cVar1 = *pcVar2;
        for (iVar3 = 0; ((cVar1 != ' ' && (cVar1 = *pcVar2, cVar1 != '\0')) && (iVar3 < 0x100));
            iVar3 = iVar3 + 1) {
            pcVar2 = pcVar2 + 1;
            local_100[iVar3] = cVar1;
            cVar1 = *pcVar2;
        }
        local_100[iVar3] = '\0';
        iVar3 = DAT_00bb72a0 * 0x95;
        uVar4 = 0xffffffff;
        pcVar6 = local_100;
        do {
            pcVar7 = pcVar6;
            if (uVar4 == 0) break;
            uVar4 = uVar4 - 1;
            pcVar7 = pcVar6 + 1;
            cVar1 = *pcVar6;
            pcVar6 = pcVar7;
        } while (cVar1 != '\0');
        uVar4 = ~uVar4;
        pcVar6 = pcVar7 + -uVar4;
        pcVar7 = DAT_00bb2821 + iVar3;
        for (uVar5 = uVar4 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
            *(uint *)pcVar7 = *(uint *)pcVar6;
            pcVar6 = pcVar6 + 4;
            pcVar7 = pcVar7 + 4;
        }
        for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
            *pcVar7 = *pcVar6;
            pcVar6 = pcVar6 + 1;
            pcVar7 = pcVar7 + 1;
        }
        uVar4 = 0xffffffff;
        pcVar2 = pcVar2 + 1;
        do {
            pcVar6 = pcVar2;
            if (uVar4 == 0) break;
            uVar4 = uVar4 - 1;
            pcVar6 = pcVar2 + 1;
            cVar1 = *pcVar2;
            pcVar2 = pcVar6;
        } while (cVar1 != '\0');
        uVar4 = ~uVar4;
        pcVar2 = pcVar6 + -uVar4;
        pcVar6 = DAT_00bb2835 + iVar3;
        for (uVar5 = uVar4 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
            *(uint *)pcVar6 = *(uint *)pcVar2;
            pcVar2 = pcVar2 + 4;
            pcVar6 = pcVar6 + 4;
        }
        for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
            *pcVar6 = *pcVar2;
            pcVar2 = pcVar2 + 1;
            pcVar6 = pcVar6 + 1;
        }
        DAT_00bb72a0 = DAT_00bb72a0 + 1;
    } while (1);
}
