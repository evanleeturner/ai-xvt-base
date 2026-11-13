/* SPDX-License-Identifier: MIT */
/* z_bop 004cc4a0 */
extern short _DAT_0052b6b8;
extern char DAT_00b69aa6;
extern const char s__c__BalanceOfPower__s_0052b6c4[];
extern const char s__c__s_0052b6bc[];

void * __cdecl FID_conflict___wfopen(char *param_1, char *param_2);
void __cdecl FUN_004ccf50(void);
void __cdecl FUN_004ccf70(void);
void __cdecl FUN_004d3350(void);
int __cdecl _sprintf(char *dst, const char *fmt, ...);
void __cdecl FUN_004d3400(void);

void * __cdecl FUN_004cc4a0(char *param_1, char *param_2)
{
  void *pFVar1;
  char local_100[256];

  pFVar1 = FID_conflict___wfopen(param_1, param_2);
  if (pFVar1 != (void *)0x0) {
    _DAT_0052b6b8 = _DAT_0052b6b8 + 1;
    goto LAB_004cc576;
  }
  FUN_004ccf50();
  pFVar1 = FID_conflict___wfopen(param_1, param_2);
  FUN_004ccf70();
  if (pFVar1 != (void *)0x0) {
    _DAT_0052b6b8 = _DAT_0052b6b8 + 1;
    goto LAB_004cc576;
  }
  if (*param_2 == 'w') {
    goto LAB_004cc576;
  }
  if (*param_2 == 'a') {
    goto LAB_004cc576;
  }
  if (DAT_00b69aa6 == 0) {
    goto LAB_004cc576;
  }
  FUN_004d3350();
  _sprintf(local_100, s__c__BalanceOfPower__s_0052b6c4, (unsigned int)DAT_00b69aa6, param_1);
  pFVar1 = FID_conflict___wfopen(local_100, param_2);
  if (pFVar1 == (void *)0x0) {
    _sprintf(local_100, s__c__s_0052b6bc, (unsigned int)DAT_00b69aa6, param_1);
    pFVar1 = FID_conflict___wfopen(local_100, param_2);
    if (pFVar1 == (void *)0x0) goto LAB_004cc576;
  }
  _DAT_0052b6b8 = _DAT_0052b6b8 + 1;
LAB_004cc576:
  FUN_004d3400();
  return pFVar1;
}
