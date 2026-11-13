/* SPDX-License-Identifier: MIT */
/* z_bop 004ea8c0 */
HFILE __cdecl FUN_004cc4a0(char *param_1, char *param_2);
uint __cdecl FUN_004cc590(HFILE param_1);
void * __cdecl _malloc(uint size);
void __cdecl _free(void *ptr);
char * __cdecl _fgets(char *ptr, int n, HFILE stream);
char * __cdecl _strncpy(char *dst, const char *src, uint n);

extern char s_specdesc_txt_0052c53c[];
extern void DAT_0051c4bc;
extern void *DAT_00aa6110;
extern char DAT_00b69d20[];
extern char DAT_00b69e1f;
extern char DAT_00b69d1e;

uint __stdcall FUN_004ea8c0(void)
{
    HFILE _File;
    char *pcVar3;
    int iVar4;
    uint uVar5;
    int iVar6;
    uint *puVar7;
    char cVar1;

    _File = FUN_004cc4a0(s_specdesc_txt_0052c53c, &DAT_0051c4bc);
    if (_File == (HFILE)0x0) {
        return 0;
    }
    if (DAT_00aa6110 != (void *)0x0) {
        _free(DAT_00aa6110);
        DAT_00aa6110 = (void *)0x0;
    }
    puVar7 = (uint *)_malloc(0xba00);
    DAT_00aa6110 = puVar7;
    for (iVar4 = 0x2e80; iVar4 != 0; iVar4 = iVar4 + -1) {
        *puVar7 = 0;
        puVar7 = puVar7 + 1;
    }
    if (DAT_00aa6110 == (void *)0x0) {
        FUN_004cc590(_File);
        return 0;
    }
    iVar4 = 0;
    do {
        iVar6 = 0;
        while (iVar6 < 5) {
            do {
                pcVar3 = _fgets((char *)&DAT_00b69d20, 0x400, _File);
                if (pcVar3 == (char *)0x0) {
                    FUN_004cc590(_File);
                    return 1;
                }
                DAT_00b69e1f = 0;
            } while (DAT_00b69d20[0] == '/' && DAT_00b69d20[1] == '/');
            
            uVar5 = 0xffffffff;
            pcVar3 = (char *)&DAT_00b69d20;
            do {
                if (uVar5 == 0) break;
                uVar5 = uVar5 - 1;
                cVar1 = *pcVar3;
                pcVar3 = pcVar3 + 1;
            } while (cVar1 != '\0');
            
            if (((char *)&DAT_00b69d1e)[~uVar5] == '\n') {
                ((char *)&DAT_00b69d1e)[~uVar5] = 0;
            }

            switch(iVar6) {
            case 0:
                iVar6 = iVar6 + 1;
                _strncpy((char *)((int)DAT_00aa6110 + iVar4), (char *)&DAT_00b69d20, 0x40);
                break;
            case 1:
                iVar6 = iVar6 + 1;
                _strncpy((char *)((int)DAT_00aa6110 + iVar4 + 0x40), (char *)&DAT_00b69d20, 0x40);
                break;
            case 2:
                iVar6 = iVar6 + 1;
                _strncpy((char *)((int)DAT_00aa6110 + iVar4 + 0x80), (char *)&DAT_00b69d20, 0x40);
                break;
            case 3:
                iVar6 = iVar6 + 1;
                _strncpy((char *)((int)DAT_00aa6110 + iVar4 + 0xc0), (char *)&DAT_00b69d20, 0x100);
                break;
            case 4:
                _strncpy((char *)((int)DAT_00aa6110 + iVar4 + 0x1c0), (char *)&DAT_00b69d20, 0x40);
            default:
                iVar6 = iVar6 + 1;
            }
        }
        iVar4 = iVar4 + 0x200;
        if (0xb9ff < iVar4) {
            uVar5 = FUN_004cc590(_File);
            return uVar5;
        }
    } while(1);
}
