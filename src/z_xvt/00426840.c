/* SPDX-License-Identifier: MIT */
/* z_xvt 00426840 */
extern unsigned short DAT_00536ec4;
extern unsigned short DAT_00536ec0;
extern unsigned short DAT_00536ebc;
extern unsigned short DAT_00536e94;
extern unsigned int   DAT_00536e98;
extern unsigned short DAT_00536e90;
extern unsigned int   DAT_00536e9c;
extern unsigned int   DAT_00536ea8;
extern unsigned int   DAT_00536ea0;
extern unsigned int   DAT_00536ea4;
extern unsigned int   DAT_00536eb4;
extern unsigned int   DAT_00536eac;
extern unsigned int   DAT_00536eb0;
extern unsigned int   DAT_00536eb8;

extern unsigned short DAT_007091ae;
extern unsigned short DAT_006d4984;
extern unsigned int   DAT_0071f2e4;
extern unsigned int   DAT_006da4fc;
extern unsigned int   DAT_0073255c;
extern unsigned int   DAT_006db7bc;
extern unsigned int   DAT_00732558;
extern unsigned int   DAT_00732568;
extern unsigned int   DAT_00732584;
extern unsigned int   DAT_00732578;
extern unsigned int   DAT_00732530;
extern unsigned int   DAT_0073253c;
extern unsigned int   DAT_00732538;
extern unsigned int   DAT_00732548;

extern unsigned short DAT_006ffbf2;
extern unsigned short DAT_00732e44;
extern unsigned short DAT_00703be4;
extern unsigned short DAT_006d4a42;
extern unsigned short DAT_00527e74;
extern unsigned int   DAT_0070acc8;
extern unsigned int   DAT_00525518;

void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);

void __cdecl FUN_00426840(unsigned short param_1, unsigned short param_2, unsigned int param_3, unsigned int param_4)
{
    DAT_00536ec4 = DAT_007091ae;
    DAT_00536ec0 = DAT_006d4984;
    DAT_00536ebc = (unsigned short)DAT_0071f2e4;
    DAT_00536e94 = (unsigned short)DAT_006da4fc;
    DAT_00536e98 = DAT_0073255c;
    DAT_00536e90 = (unsigned short)DAT_006db7bc;
    DAT_00536e9c = DAT_00732558;
    DAT_00536ea8 = DAT_00732568;
    DAT_00536ea0 = DAT_00732584;
    DAT_00536ea4 = DAT_00732578;
    DAT_00536eb4 = DAT_00732530;
    DAT_00536eac = DAT_0073253c;
    DAT_00536eb0 = DAT_00732538;
    DAT_00536eb8 = DAT_00732548;

    DAT_007091ae = param_1;
    DAT_006ffbf2 = (unsigned short)(param_1 - 1);
    DAT_00732e44 = (unsigned short)(param_1 >> 1);
    DAT_0071f2e4 = param_4;
    DAT_006d4984 = param_2;
    DAT_00703be4 = (unsigned short)(param_2 - 1);
    DAT_006d4a42 = (unsigned short)(param_2 >> 1);

    if (DAT_0070acc8 == 0) {
        RaiseException(0xC0000094UL, 0, 0, NULL);
    }
    DAT_006da4fc = param_4 / DAT_0070acc8;

    DAT_00527e74 = 0xe000;

    if (DAT_0070acc8 == 0 || DAT_00525518 == 0) {
        RaiseException(0xC0000094UL, 0, 0, NULL);
    }
    DAT_006db7bc = (unsigned int)(int)(((unsigned __int64)param_4 % (unsigned __int64)DAT_0070acc8) / (unsigned __int64)DAT_00525518);
}
