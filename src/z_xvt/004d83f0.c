/* SPDX-License-Identifier: MIT */
/* z_xvt 004d83f0 */
int __cdecl FUN_004ddc70(int *param_1, int *param_2);
int __cdecl FUN_004dcfd0(int *param_1, int param_2, int param_3, int param_4, int param_5);
int __cdecl FUN_004dd7d0(int *param_1, int param_2, int param_3);
int __cdecl FUN_004db530(void);
int __cdecl FUN_004db550(void);
int __cdecl FUN_004db570(void);
int __cdecl FUN_004db590(void);
void __cdecl FUN_004d9ea0(int *param_1);
char * __cdecl FUN_004dcfa0(unsigned int param_1);
int __cdecl FUN_004d9a80(int *param_1, char *param_2, char *param_3, char *param_4, int param_5, int param_6, int param_7, char *param_8);
void __cdecl FUN_004d8a00(void);

extern char s_jewelsound_00531990[];
extern char s_join1u_00532abc[];
extern char s_join1d_00532ac4[];

int __stdcall FUN_004d83f0(void)
{
    int local_34;
    int local_30[12];
    int iVar2;
    char *pcVar3;
    char *pcVar6;
    int iVar5;
    int uVar4;

    local_30[2] = 0;
    local_30[3] = 0;
    local_30[4] = 0;
    local_30[5] = 0;
    local_30[6] = 0;
    local_30[7] = 0;
    local_30[1] = 1;
    FUN_004ddc70(local_30, &local_34);
    FUN_004dcfd0(&local_30[8], 0x16, 0x72, 0x2a, 0x8a);
    iVar2 = FUN_004dd7d0(&local_30[8], local_30[0], local_34);
    if (iVar2 != 0) {
        if (FUN_004db530() == 0) {
            if (FUN_004db550() == 0) {
                if (FUN_004db570() == 0) {
                    if (FUN_004db590() == 0) {
                        goto LAB_004d8486;
                    }
                }
            }
        }
        local_30[1] = 2;
    }
LAB_004d8486:
    FUN_004d9ea0(&local_30[1]);
    FUN_004dcfd0(&local_30[8], 0x16, 0x72, 0x2a, 0x8a);
    pcVar6 = s_jewelsound_00531990;
    iVar5 = 0xb;
    uVar4 = 0;
    iVar2 = 0xc;
    pcVar3 = FUN_004dcfa0(0x2d0);
    iVar2 = FUN_004d9a80(&local_30[8], s_join1u_00532abc, s_join1d_00532ac4, pcVar3, iVar2, uVar4, iVar5, pcVar6);
    if (iVar2 != 0) {
        FUN_004d8a00();
    }
    return 1;
}
