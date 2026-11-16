/* SPDX-License-Identifier: MIT */
/* z_xvt 0040c5b0 */
extern void __cdecl FUN_0040c6e0(void);
extern unsigned int __cdecl FUN_00439830(int param_1, int param_2);
extern void __cdecl FUN_00490be0(char *param_1, int param_2, int param_3, char *param_4);

extern char DAT_0071ed0e;
extern int DAT_0073b650;
extern int DAT_00740880;
extern int DAT_006d4a44;
extern char DAT_0073b4b0;
extern char DAT_0073b660;
extern char DAT_0073b540;
extern char DAT_0073b6e0;
extern char DAT_00741108;
extern char DAT_0073b4c0;
extern short _DAT_00741106;
extern short DAT_0073b550;
extern char DAT_0073b661;
extern char DAT_0073b6e1;
extern char DAT_0073b4c1;
extern short DAT_00740a80;
extern char DAT_0085e79d;
extern char DAT_0073ba30;

void __stdcall FUN_0040c5b0(void)
{
    int iVar1;
    unsigned int uVar2;
    unsigned int uVar3;
    unsigned int uVar4;
    int *puVar5;
    short *puVar6;
    
    if (DAT_0071ed0e != '\0') {
        FUN_0040c6e0();
        if (DAT_0073b650 == 0 || (uVar2 = FUN_00439830(DAT_0073b650, 0x100), uVar2 == 0)) {
            iVar1 = DAT_00740880;
            DAT_0073b650 = 0;
            if (DAT_006d4a44 != 0) {
                DAT_0073b4b0 = DAT_0073b660;
                DAT_006d4a44 = DAT_006d4a44 - 1;
                DAT_0073b540 = DAT_0073b6e0;
                DAT_00741108 = DAT_0073b4c0;
                _DAT_00741106 = DAT_0073b550;
                if (DAT_006d4a44 != 0) {
                    uVar3 = (unsigned int)DAT_006d4a44;
                    uVar2 = 0;
                    puVar5 = &DAT_00740880;
                    puVar6 = &DAT_0073b550;
                    do {
                        uVar4 = uVar2 + 1;
                        *puVar5 = puVar5[1];
                        (&DAT_0073b660)[uVar2] = (&DAT_0073b661)[uVar2];
                        (&DAT_0073b6e0)[uVar2] = (&DAT_0073b6e1)[uVar2];
                        (&DAT_0073b4c0)[uVar2] = (&DAT_0073b4c1)[uVar2];
                        *puVar6 = puVar6[1];
                        uVar2 = uVar4;
                        puVar5 = puVar5 + 1;
                        puVar6 = puVar6 + 1;
                    } while (uVar4 < uVar3);
                }
                if ((*(short *)((int)&DAT_00740a80 + iVar1 * 2) != 0) && (DAT_0085e79d != '\0')) {
                    FUN_00490be0((char *)(&DAT_0073ba30 + iVar1 * 6), 1, 0, (char *)0x7e);
                    DAT_0073b650 = iVar1;
                }
            }
        }
    }
    return;
}
