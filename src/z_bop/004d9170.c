/* SPDX-License-Identifier: MIT */
/* z_bop 004d9170 */
extern int __cdecl FUN_004de090(int *param_1, int *param_2);
extern int __cdecl FUN_004d6460(int *param_1, int param_2, int param_3, int param_4, int param_5);
extern int __cdecl FUN_004d6b90(int *param_1, int param_2, int param_3);
extern int __stdcall FUN_004dc1e0(void);
extern int __stdcall FUN_004dc200(void);
extern int __stdcall FUN_004dc220(void);
extern int __stdcall FUN_004dc240(void);
extern int __cdecl FUN_004daba0(int *param_1);
extern char * __cdecl FUN_004ddc10(int param_1);
extern int __cdecl FUN_004da780(int *param_1, char *param_2, char *param_3, char *param_4, int param_5, int param_6, int param_7, char *param_8);
extern int __stdcall FUN_004d9780(void);

extern char * s_jewelsound_0052ad74;
extern char * s_join1u_0052bfe4;
extern char * s_join1d_0052bfec;

int __stdcall FUN_004d9170(void)
{
    int local_34[13];
    int local_38;
    int iVar2;
    char *pcVar3;
    char *pcVar6;
    int iVar5;
    int uVar4;

    local_34[2] = 0;
    local_34[3] = 0;
    local_34[4] = 0;
    local_34[5] = 0;
    local_34[6] = 0;
    local_34[7] = 0;
    local_34[1] = 1;
    local_34[8] = 0;
    FUN_004de090(local_34, &local_38);
    FUN_004d6460(&local_34[9], 0x16, 0x72, 0x2a, 0x8a);
    iVar2 = FUN_004d6b90(&local_34[9], local_34[0], local_38);
    if (iVar2 != 0) {
        if (FUN_004dc1e0() == 0) {
            if (FUN_004dc200() == 0) {
                if (FUN_004dc220() == 0) {
                    if (FUN_004dc240() == 0) {
                        goto LAB_004d920a;
                    }
                }
            }
        }
        local_34[1] = 2;
    }
LAB_004d920a:
    FUN_004daba0(&local_34[1]);
    FUN_004d6460(&local_34[9], 0x16, 0x72, 0x2a, 0x8a);
    pcVar6 = s_jewelsound_0052ad74;
    iVar5 = 0xb;
    uVar4 = 0;
    iVar2 = 0xc;
    pcVar3 = FUN_004ddc10(0x2de);
    iVar2 = FUN_004da780(&local_34[9], s_join1u_0052bfe4, s_join1d_0052bfec, pcVar3, iVar2, uVar4, iVar5, pcVar6);
    if (iVar2 != 0) {
        FUN_004d9780();
    }
    return 1;
}
