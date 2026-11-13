/* SPDX-License-Identifier: MIT */
/* z_bop 004b18f0 */
extern int DAT_00528cbc;
extern void *DAT_00528c94;
extern void *DAT_00528c98;
extern void *DAT_00664974;
extern void *DAT_0066497c;
extern void *DAT_00a91a34;
extern void *DAT_00664548;

void __stdcall FUN_004079e0(void);
void __cdecl FUN_004b17d0(void *param_1);
void __stdcall FUN_004b3070(void);

void __stdcall FUN_004b18f0(void)
{
  if (DAT_00528cbc == 0) {
    FUN_004079e0();
    return;
  }
  if (DAT_00528c94 != NULL) {
    FUN_004b17d0(DAT_00528c94);
    DAT_00528c94 = NULL;
  }
  ((void (__stdcall *)(void *))*(void **)((char *)*(void **)DAT_00528c98 + 8))(DAT_00528c98);
  FUN_004b3070();
  if (DAT_00664974 != NULL) {
    ((void (__stdcall *)(void *))*(void **)((char *)*(void **)DAT_00664974 + 8))(DAT_00664974);
    DAT_00664974 = NULL;
  }
  if (DAT_0066497c != NULL) {
    ((void (__stdcall *)(void *))*(void **)((char *)*(void **)DAT_0066497c + 8))(DAT_0066497c);
    DAT_0066497c = NULL;
  }
  if (DAT_00a91a34 != NULL) {
    ((void (__stdcall *)(void *))*(void **)((char *)*(void **)DAT_00a91a34 + 8))(DAT_00a91a34);
    DAT_00a91a34 = NULL;
  }
  if (DAT_00664548 != NULL) {
    ((void (__stdcall *)(void *))*(void **)((char *)*(void **)DAT_00664548 + 8))(DAT_00664548);
    DAT_00664548 = NULL;
  }
  FUN_004079e0();
  DAT_00528cbc = 0;
  return;
}
