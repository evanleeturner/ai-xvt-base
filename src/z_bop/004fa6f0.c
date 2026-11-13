/* SPDX-License-Identifier: MIT */
/* z_bop 004fa6f0 */
extern int DAT_00b6a5a6;
extern int DAT_00aa5e10[];
extern int DAT_00aa5ac0[];
extern int DAT_00aa5cb0[];
extern short DAT_00aa5aac;
extern char DAT_00a91d24;
extern int DAT_00aa6af4;
extern int DAT_00aa6af8;
extern int _DAT_00aa6af0;
extern int DAT_00aa6afc;

int __cdecl FUN_004cef70(int param_1, unsigned int *param_2, unsigned int param_3);

int __stdcall FUN_004fa6f0(void)
{
  int local_4 = 0;
  int count = DAT_00aa5e10[DAT_00b6a5a6];
  
  if (count > 0) {
    do {
      int iVar3 = DAT_00b6a5a6 * 8;
      int iVar2 = local_4 + iVar3;
      
      if (DAT_00aa5ac0[iVar2] == -1 && DAT_00aa5cb0[iVar2] != 0) {
        int iVar5 = 0;
        if (DAT_00aa5aac > 0) {
          char *pcVar6 = &DAT_00a91d24;
          do {
            if (*pcVar6 != '\0' && *(unsigned char *)(pcVar6 - 0xc) == DAT_00b6a5a6) {
              int iVar4 = 0;
              int iVar1 = DAT_00aa5e10[DAT_00b6a5a6];
              
              if (iVar1 > 0) {
                do {
                  if (DAT_00aa5ac0[iVar3 + iVar4] == iVar5) break;
                  iVar4++;
                } while (iVar4 < iVar1);
              }
              
              DAT_00aa5ac0[iVar2] = iVar5;
              
              if (iVar4 == iVar1) {
                DAT_00aa6af4 = DAT_00b6a5a6;
                DAT_00aa6af8 = local_4;
                _DAT_00aa6af0 = 0x6e;
                DAT_00aa6afc = iVar5;
                FUN_004cef70(0, (unsigned int *)&_DAT_00aa6af0, 0x10);
                break;
              }
            }
            pcVar6 += 0x562;
            iVar5++;
          } while (iVar5 < DAT_00aa5aac);
        }
      }
      local_4++;
    } while (local_4 < count);
  }
  return 1;
}
