/* SPDX-License-Identifier: MIT */
/* z_bop 0043c900 */
extern int DAT_009ecc34;
extern char DAT_009e9698;
extern char DAT_00a0a1d0;
extern char DAT_009e968a;
extern char DAT_009e9696;
extern int DAT_00a08374;

void __cdecl FUN_00440760(int param_1, unsigned int param_2);
void __stdcall FUN_004abe50(void);
void __cdecl FUN_0042ec80(unsigned int param_1);
void __stdcall FUN_004aba50(void);

void __stdcall FUN_0043c900(void)
{
  int iVar1;
  unsigned int uVar2;
  
  iVar1 = DAT_009ecc34 * 0x5bd;
  if ((&DAT_009e9698)[iVar1] == '\0') {
    uVar2 = (unsigned int)((DAT_00a0a1d0 == '\0') - 1 & 4);
  }
  else {
    if (*(short *)((int)&DAT_009e968a + iVar1) == -1) {
      uVar2 = 1;
    }
    else {
      uVar2 = (unsigned int)(unsigned short)((unsigned char)(&DAT_009e9696)[iVar1] + 1);
    }
    DAT_00a0a1d0 = '\x01';
    if ((&DAT_009e9696)[iVar1] != '\x02') {
      DAT_00a0a1d0 = '\0';
    }
  }
  FUN_00440760(DAT_00a08374 + 0x34, uVar2);
  FUN_004abe50();
  FUN_0042ec80(uVar2);
  FUN_004aba50();
  return;
}
