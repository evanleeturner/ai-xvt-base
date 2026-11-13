/* SPDX-License-Identifier: MIT */
/* z_bop 004d5c20 */
extern char s_video_cfg_0052baf4[];
extern int DAT_0051a854;
extern int DAT_00665420;

void * __cdecl FUN_004cc4a0(void *, void *);
int __cdecl FUN_004cc6c0(void *, void *, unsigned int);
unsigned int __cdecl FUN_004cc590(void *);

unsigned int __stdcall FUN_004d5c20(void)
{
  void *pFVar2;
  int bVar1;
  
  pFVar2 = FUN_004cc4a0(s_video_cfg_0052baf4, &DAT_0051a854);
  if (pFVar2 == (void *)0x0) {
    return 0;
  }
  bVar1 = FUN_004cc6c0(pFVar2, &DAT_00665420, 0x10);
  FUN_004cc590(pFVar2);
  return (((short)bVar1 == 0) - 1) & 0x665420;
}
