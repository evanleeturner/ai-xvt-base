/* SPDX-License-Identifier: MIT */
/* z_bop 00430700 */
extern void __stdcall FUN_00430830(void);
extern unsigned int __cdecl FUN_004a9300(int param_1, int param_2);
extern void __cdecl FUN_0042d230(void *param_1, int param_2, int param_3, int param_4);

extern char DAT_009ecc52;
extern int DAT_00a0aa10;
extern int DAT_00a0fc80;
extern int DAT_009a20a4;
extern char DAT_00a0a870;
extern char DAT_00a0aa20;
extern char DAT_00a0a900;
extern char DAT_00a0aaa0;
extern char DAT_00a1050e;
extern char DAT_00a0a880;
extern unsigned short _DAT_00a1050c;
extern unsigned short DAT_00a0a910;
extern char DAT_00a0aa21;
extern char DAT_00a0aaa1;
extern char DAT_00a0a881;
extern unsigned short DAT_00a0fe80;
extern char DAT_00bb737d;
extern char DAT_00a0adf0;

void __stdcall FUN_00430700(void)
{
    int iVar1;
    unsigned int uVar2;
    unsigned int uVar3;
    unsigned int uVar4;
    unsigned int *puVar5;
    unsigned short *puVar6;

    if (DAT_009ecc52 != '\0') {
        FUN_00430830();
        if (DAT_00a0aa10 == 0 || FUN_004a9300(DAT_00a0aa10, 0x100) == 0) {
            iVar1 = DAT_00a0fc80;
            DAT_00a0aa10 = 0;
            if (DAT_009a20a4 != 0) {
                DAT_00a0a870 = DAT_00a0aa20;
                DAT_009a20a4 = DAT_009a20a4 - 1;
                DAT_00a0a900 = DAT_00a0aaa0;
                DAT_00a1050e = DAT_00a0a880;
                _DAT_00a1050c = DAT_00a0a910;
                if (DAT_009a20a4 != 0) {
                    uVar4 = (unsigned int)DAT_009a20a4;
                    uVar2 = 0;
                    puVar5 = (unsigned int *)&DAT_00a0fc80;
                    puVar6 = (unsigned short *)&DAT_00a0a910;
                    do {
                        uVar3 = uVar2 + 1;
                        *puVar5 = puVar5[1];
                        ((char *)&DAT_00a0aa20)[uVar2] = ((char *)&DAT_00a0aa21)[uVar2];
                        ((char *)&DAT_00a0aaa0)[uVar2] = ((char *)&DAT_00a0aaa1)[uVar2];
                        ((char *)&DAT_00a0a880)[uVar2] = ((char *)&DAT_00a0a881)[uVar2];
                        *puVar6 = puVar6[1];
                        uVar2 = uVar3;
                        puVar5 = puVar5 + 1;
                        puVar6 = puVar6 + 1;
                    } while (uVar3 < uVar4);
                }
                if (((unsigned short *)&DAT_00a0fe80)[iVar1] != 0 && DAT_00bb737d != '\0') {
                    FUN_0042d230((char *)&DAT_00a0adf0 + iVar1 * 6, 1, 0, 0x7e);
                    DAT_00a0aa10 = iVar1;
                }
            }
        }
    }
    return;
}
