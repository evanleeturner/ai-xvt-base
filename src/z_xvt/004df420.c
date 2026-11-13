/* SPDX-License-Identifier: MIT */
/* z_xvt 004df420 */
void __cdecl FUN_004dd3e0(int *param_1, int param_2, int param_3, unsigned short param_4, int param_5);
void __cdecl _sprintf(char *buffer, const char *format, ...);
void __cdecl FUN_004da920(int param_1, char *param_2, int *param_3, int param_4);
unsigned char __cdecl FUN_004df260(int param_1);
unsigned char __cdecl FUN_004df290(int param_1);
void __stdcall FUN_004dbea0(void);
void __stdcall FUN_004dbb40(void);

extern char DAT_00758ee8[];
extern int DAT_0064fca8;
extern int DAT_0077dc66;
extern int DAT_0077dc42[];
extern unsigned short DAT_0064fcac;
extern const char s_Center_joystick__d_and_press_a_b_00532c3c[];

int __stdcall FUN_004df420(void)
{
    char acStack_64[100];
    int iVar2;

    if (DAT_00758ee8[DAT_0064fca8] != '\0') {
        iVar2 = DAT_0077dc66 * 0x440;
        FUN_004dd3e0((int *)((char *)DAT_0077dc42 + iVar2), 0, 0, (unsigned short)DAT_0064fcac, 1);
        _sprintf(acStack_64, s_Center_joystick__d_and_press_a_b_00532c3c, DAT_0064fca8 + 1);
        FUN_004da920(0x14, acStack_64, (int *)((char *)DAT_0077dc42 + iVar2), 0xff);
        if (FUN_004df260(DAT_0064fca8) == 0) {
            if (FUN_004df290(DAT_0064fca8) == 0) {
                goto LAB_004df4bc;
            }
        }
    }
    DAT_0064fca8 = DAT_0064fca8 + 1;
LAB_004df4bc:
    if (DAT_0064fca8 > 1) {
        FUN_004dbea0();
        FUN_004dbb40();
    }
    return 0;
}
