/* SPDX-License-Identifier: MIT */
/* z_bop 00468cf0 */
extern unsigned short DAT_009a2000;
extern char DAT_009a20c0[];
extern char DAT_009d9192[];
extern unsigned short DAT_009a73c0;
extern char DAT_009d8ccd[];
extern unsigned char DAT_009d8ccc[];
extern unsigned short DAT_00a07cec;
extern unsigned short DAT_009a1fe0;
extern int DAT_009a1fe8;

unsigned __int64 __fastcall FUN_00468cf0(unsigned int param_1, unsigned int param_2)
{
    unsigned short uVar4 = 0;
    int in_EAX = DAT_009a2000;
    unsigned int uVar2;
    int iVar3;
    unsigned short uVar1;

    if (DAT_009a2000 > 0) {
        param_2 = 0;
        do {
            uVar2 = uVar4;
            if ( (DAT_009a20c0[uVar2 * 0x126 + 3] != '\0' || *(int *)(DAT_009d9192 + uVar2 * 0x566) != -1) &&
                 DAT_009a73c0 != uVar4 &&
                 DAT_009d8ccd[uVar2 * 0x566] != '\0' &&
                 DAT_009d8ccc[uVar2 * 0x566] == DAT_009a73c0 ) {
                if (DAT_009a20c0[uVar2 * 0x126] == '\0' || (uVar1 = DAT_00a07cec, DAT_009a20c0[uVar2 * 0x126 + 1] != '\0')) {
                    return 0;
                }
                for (; uVar1 < DAT_009a1fe0; uVar1++) {
                    iVar3 = uVar1 * 0x23 + DAT_009a1fe8;
                    if (*(char *)(iVar3 + 3) != '\0' && *(unsigned char *)(iVar3 + 0x16) == uVar2) {
                        return 0;
                    }
                }
            }
            in_EAX = DAT_009a2000;
            uVar4++;
        } while (uVar4 < in_EAX);
    }
    return ((unsigned __int64)param_2 << 32) | (((unsigned int)in_EAX >> 16) << 16) | 1;
}
