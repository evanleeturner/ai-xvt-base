/* SPDX-License-Identifier: MIT */
/* z_xvt 0051c4f0 */
extern ushort DAT_00655110;
extern uint DAT_006550dc;
extern void *DAT_006550f4;
extern ushort DAT_0074244e;
extern ushort DAT_0074244c;
extern ushort DAT_006550e4;
extern ushort DAT_006550e0;
extern uint DAT_00655108;
extern ushort _DAT_006550fc;
extern uint DAT_006550f0;
extern ushort DAT_006550e8;
extern ushort DAT_0053670c;
extern uint _DAT_00655100;
extern uchar DAT_00536704;

int __cdecl FUN_0048f4f0(uint param_1);
void __cdecl FUN_0048f510(int param_1);
void __cdecl FUN_0048f4b0(uint param_1);
int __cdecl _fclose(void *stream);
void __stdcall FUN_0051d360(void);
void __cdecl FUN_0051c130(int param_1);

uint __stdcall FUN_0051c4f0(void)
{
    if (DAT_00655110 == 0) {
        return 1;
    }
    uint uVar2 = 0;
    if (DAT_006550dc != 0) {
        do {
            int iVar1 = FUN_0048f4f0(DAT_00655110);
            ushort uVar3 = *(ushort *)(iVar1 + uVar2 * 2);
            iVar1 = (uVar3 == 0) - 1;
            uVar3 = (ushort)iVar1 & uVar3;
            FUN_0048f510(iVar1);
            if (uVar3 != 0) {
                FUN_0048f4b0(((uint)iVar1 >> 16) << 16 | uVar3);
            }
            uVar2++;
        } while (uVar2 < DAT_006550dc);
    }
    FUN_0048f4b0(DAT_00655110);
    DAT_00655110 = 0;
    if (DAT_006550f4 != (void *)0x0) {
        _fclose(DAT_006550f4);
    }
    FUN_0051d360();
    DAT_0074244e = 0;
    DAT_0074244c = 0;
    FUN_0051c130(5);
    DAT_006550e4 = 0;
    DAT_006550e0 = 0;
    DAT_00655108 = 0;
    DAT_006550f4 = (void *)0x0;
    _DAT_006550fc = 0;
    DAT_006550f0 = 0;
    DAT_006550e8 = DAT_0053670c;
    _DAT_00655100 = 0;
    DAT_00536704 = 0;
    return 1;
}
