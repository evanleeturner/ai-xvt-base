/* SPDX-License-Identifier: MIT */
/* z_xvt 00426970 */
extern unsigned int   DAT_00536e98;
extern unsigned int   DAT_00536e9c;
extern unsigned int   DAT_00536ea8;
extern unsigned int   DAT_00536ea0;
extern unsigned int   DAT_00536ea4;
extern unsigned int   DAT_00536eb4;
extern unsigned int   DAT_00536eac;
extern unsigned int   DAT_00536eb0;
extern unsigned int   DAT_00536eb8;
extern unsigned short DAT_00536ec4;
extern unsigned short DAT_00536ec0;
extern unsigned short DAT_00536ebc;
extern unsigned short DAT_00536e94;
extern unsigned short DAT_00536e90;

extern unsigned int   DAT_0073255c;
extern unsigned int   DAT_00732558;
extern unsigned int   DAT_00732568;
extern unsigned int   DAT_00732584;
extern unsigned int   DAT_00732578;
extern unsigned int   DAT_00732530;
extern unsigned int   DAT_0073253c;
extern unsigned int   DAT_00732538;
extern unsigned int   DAT_00732548;
extern unsigned short DAT_007091ae;
extern unsigned short DAT_006ffbf2;
extern unsigned short DAT_00732e44;
extern unsigned short DAT_006d4984;
extern unsigned short DAT_00703be4;
extern unsigned short DAT_006d4a42;
extern unsigned int   _DAT_0071f2e4;
extern unsigned short DAT_00527e74;
extern unsigned int   _DAT_006da4fc;
extern unsigned int   _DAT_006db7bc;

void __stdcall FUN_00426970(void)
{
    DAT_0073255c = DAT_00536e98;
    DAT_00732558 = DAT_00536e9c;
    DAT_00732568 = DAT_00536ea8;
    DAT_00732584 = DAT_00536ea0;
    DAT_00732578 = DAT_00536ea4;
    DAT_00732530 = DAT_00536eb4;
    DAT_0073253c = DAT_00536eac;
    DAT_00732538 = DAT_00536eb0;
    DAT_00732548 = DAT_00536eb8;
    DAT_007091ae = DAT_00536ec4;
    DAT_006ffbf2 = (unsigned short)(DAT_00536ec4 - 1);
    DAT_00732e44 = (unsigned short)(DAT_00536ec4 >> 1);
    DAT_006d4984 = DAT_00536ec0;
    DAT_00703be4 = (unsigned short)(DAT_00536ec0 - 1);
    DAT_006d4a42 = (unsigned short)(DAT_00536ec0 >> 1);
    _DAT_0071f2e4 = (unsigned int)DAT_00536ebc;
    DAT_00527e74 = 0xc000;
    _DAT_006da4fc = (unsigned int)DAT_00536e94;
    _DAT_006db7bc = (unsigned int)DAT_00536e90;
    return;
}
