/* SPDX-License-Identifier: MIT */
/* z_bop 00456040 */
extern unsigned short DAT_009a1834;
extern char DAT_00521f2e;
extern char DAT_00524170;
extern char DAT_009d8c82;
extern char DAT_009a20c0;
extern char DAT_009d8c94;
extern char DAT_009d695b;

void __cdecl FUN_00456190(unsigned short);
void __cdecl FUN_004587c0(unsigned short);

void __stdcall FUN_00456040(void)
{
    char *pcVar1;
    char cVar2;
    unsigned int uVar3;

    if (((&DAT_00521f2e)[(unsigned int)(unsigned char)(&DAT_00524170)[(unsigned char)(&DAT_009d8c82)[(unsigned int)DAT_009a1834 * 0x566]] * 0x1a] & 0x80) == 0) {
        FUN_004587c0(0xffff);
    }
    else {
        FUN_00456190(0xffff);
    }

    uVar3 = (unsigned int)DAT_009a1834;
    pcVar1 = (char *)((int)&DAT_009a20c0 + uVar3 * 0x126 + 1);
    cVar2 = *(char *)((int)&DAT_009a20c0 + uVar3 * 0x126 + 1);
    if (cVar2 != '\0') {
        if ((&DAT_009d8c94)[uVar3 * 0x566] == '\0') {
            *pcVar1 = cVar2 + -1;
        }
        else if (DAT_009d695b != '\x02') {
            *pcVar1 = cVar2 + -1;
            return;
        }
    }
    return;
}
