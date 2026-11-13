/* SPDX-License-Identifier: MIT */
/* z_bop 0045dba0 */
extern unsigned short DAT_00a07cec;
extern int DAT_009a1fe0;
extern int DAT_009a1fe8;
extern char DAT_009d6954;

bool __cdecl FUN_0045dba0(unsigned short param_1, short param_2)
{
    char cVar1;
    int iVar2;
    unsigned short uVar3;
    unsigned int uVar4;
    unsigned short uVar5;

    uVar3 = 0;
    for (uVar5 = DAT_00a07cec; (int)(unsigned int)uVar5 < DAT_009a1fe0; uVar5 = uVar5 + 1) {
        iVar2 = (unsigned int)uVar5 * 0x23 + DAT_009a1fe8;
        if (*(char *)(iVar2 + 3) != '\0') {
            iVar2 = *(int *)(*(int *)(iVar2 + 0x1f) + 0xa9);
            if ((((param_1 == *(unsigned short *)(iVar2 + 0x42)) && (uVar5 != param_1)) &&
                (*(unsigned short *)(iVar2 + 0x36e) != *(unsigned short *)(iVar2 + 0x42))) &&
               (((cVar1 = *(char *)(iVar2 + 0x5e), cVar1 == '\v' || (cVar1 == '\f')) || (cVar1 == '\x17'))
               )) {
                uVar3 = uVar3 + 1;
            }
        }
    }
    uVar4 = (unsigned int)param_1;
    if ((int)uVar4 < DAT_009a1fe0) {
        if (param_2 == 1) {
            uVar5 = 100;
        }
        else {
            cVar1 = *(char *)(uVar4 * 0x23 + 2 + DAT_009a1fe8);
            if ((cVar1 == '\x04') || (cVar1 == '\x05')) {
                uVar5 = 6;
            }
            else {
                uVar5 = (-(unsigned short)(cVar1 == '\x03') & 2) + 2;
            }
        }
    }
    else {
        uVar5 = 2;
    }
    if (*(int *)(uVar4 * 0x23 + 0x1b + DAT_009a1fe8) != -1) {
        if (DAT_009d6954 == '\0') {
            uVar5 = (-(unsigned short)(param_2 == 1) & 2) + 2;
        }
        else if (DAT_009d6954 == '\x01') {
            uVar5 = (-(unsigned short)(param_2 == 1) & 5) + 3;
        }
        else if (DAT_009d6954 == '\x02') {
            uVar5 = (-(unsigned short)(param_2 == 1) & 0x60) + 4;
        }
    }
    return uVar3 < uVar5;
}
