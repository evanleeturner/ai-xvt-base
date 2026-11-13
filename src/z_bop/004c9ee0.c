/* SPDX-License-Identifier: MIT */
/* z_bop 004c9ee0 */
extern unsigned int DAT_00b6a250;
extern char DAT_0051a854;
extern const char s_c_train_1TA01BF_TIE_0052b5e8[];

extern char __stdcall FUN_004ccb90(void);
extern void * __stdcall FID_conflict___wfopen(const char *, const char *);
extern void __stdcall _fclose(void *);

unsigned int __stdcall FUN_004c9ee0(void)
{
  void *_File;
  int iVar1;
  char *pcVar2;
  char *pcVar3;
  unsigned int *puVar4;
  char local_80[21];
  unsigned int local_6b;
  
  pcVar2 = (char *)s_c_train_1TA01BF_TIE_0052b5e8;
  pcVar3 = local_80;
  for (iVar1 = 5; iVar1 != 0; iVar1 = iVar1 - 1) {
    *(unsigned int *)pcVar3 = *(unsigned int *)pcVar2;
    pcVar2 = pcVar2 + 4;
    pcVar3 = pcVar3 + 4;
  }
  *pcVar3 = *pcVar2;
  puVar4 = &local_6b;
  for (iVar1 = 0x1a; iVar1 != 0; iVar1 = iVar1 - 1) {
    *puVar4 = 0;
    puVar4 = puVar4 + 1;
  }
  *(unsigned short *)puVar4 = 0;
  *(unsigned char *)((int)puVar4 + 2) = 0;
  local_80[0] = FUN_004ccb90();
  if (local_80[0] != '\0') {
    _File = FID_conflict___wfopen(local_80, &DAT_0051a854);
    if (_File != (void *)0) {
      _fclose(_File);
    }
    DAT_00b6a250 = (unsigned int)(_File != (void *)0);
    return DAT_00b6a250;
  }
  return 0;
}
