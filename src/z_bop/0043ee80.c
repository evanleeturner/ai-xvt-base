/* SPDX-License-Identifier: MIT */
/* z_bop 0043ee80 */
extern unsigned short DAT_00a08ee2;
extern unsigned short DAT_00a08ee0;
extern short DAT_00a08374;
extern int DAT_009e9670;
extern int DAT_009ecc34;
extern int DAT_009a1fe8;
extern unsigned short DAT_00a0a240;
extern short DAT_009a20ae;
extern int DAT_005215e0;
extern void (*DAT_009d1300)(void);

void __cdecl FUN_004a9640(char param_1);
unsigned __int64 __cdecl FUN_004278c0(void *param_1);
void __cdecl FUN_004a9520(unsigned short param_1, unsigned short param_2, short param_3, short param_4);
void __cdecl FUN_004a9500(unsigned short param_1, unsigned short param_2);
void __cdecl FUN_004a95c0(unsigned int param_1);
void __cdecl FUN_004a9590(unsigned int param_1);
void __cdecl FUN_004277f0(unsigned short param_1, unsigned int param_2, unsigned int param_3);
void __cdecl FUN_00440760(unsigned int param_1, unsigned int param_2);

void __stdcall FUN_0043ee80(void)
{
    unsigned short uVar1;
    unsigned short uVar2;
    unsigned short uVar3;
    short sVar4;
    unsigned __int64 uVar5;
    int p0;
    int p1;
    int p2;
    int p3;
    int p4;
    int p5;

    uVar3 = DAT_00a08ee2;
    uVar2 = DAT_00a08ee0;
    if (DAT_00a08374 == 0) {
        p0 = (int)&DAT_009e9670 + DAT_009ecc34 * 0x5bd;
        p1 = *(int *)p0;
        p2 = p1 * 0x23 + 0x1f + DAT_009a1fe8;
        p3 = *(int *)p2;
        p4 = p3 + 0xa9;
        p5 = *(int *)p4;
        uVar1 = (unsigned short)*(unsigned char *)(p5 + 0x247);
        
        if (DAT_00a0a240 != uVar1) {
            if ((unsigned int)DAT_00a08ee0 + (unsigned int)DAT_00a08ee2 == 0) {
                return;
            }
            FUN_004a9640('\x02');
            sVar4 = DAT_009a20ae + uVar3;
            uVar5 = FUN_004278c0(&DAT_005215e0);
            FUN_004a9520(uVar2, uVar3, (short)uVar5 + uVar2, sVar4);
            FUN_004a9500(uVar2, uVar3);
            FUN_004a95c0(0x2c);
            (*DAT_009d1300)();
            FUN_004a9590(0x4e);
            FUN_004277f0(uVar1, 3, 1);
            *(unsigned short *)(&DAT_00a0a240 + (unsigned int)DAT_00a08374 * 2) = uVar1;
        }
        
        if (*(char *)(p5 + 0x248) != '\0') {
            FUN_00440760(0x2f, 1);
            return;
        }
        FUN_00440760(0x2f, 0);
    }
    return;
}
