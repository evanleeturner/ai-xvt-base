/* SPDX-License-Identifier: MIT */
/* z_bop 00438df0 */
void FUN_00439900(void);
void FUN_004abe50(void);
void FUN_0042ec80(int);
void FUN_004aba50(void);
void FUN_0043ec40(void);
void FUN_00439710(void);
void FUN_004398b0(void);
void FUN_00439c60(void);
void FUN_004419d0(void);
void FUN_0042f030(int);

extern int DAT_009ecc34;
extern char DAT_009e9681;
extern char DAT_009d6940;
extern char DAT_009e9683;
extern char DAT_009e9a77;

void __cdecl FUN_00438df0(int param_1)
{
  char cVar1;
  
  if (param_1 == DAT_009ecc34) {
    if ((&DAT_009e9681)[DAT_009ecc34 * 0x5bd] == '\0' && DAT_009d6940 == '\0') {
      if ((&DAT_009e9683)[param_1 * 0x5bd] != '\0') {
        FUN_00439900();
        FUN_004abe50();
        FUN_0042ec80(0);
        FUN_004aba50();
        FUN_0043ec40();
        return;
      }
      cVar1 = (&DAT_009e9a77)[DAT_009ecc34 * 0x5bd];
      if (cVar1 == '\0') {
        FUN_00439710();
        FUN_0043ec40();
        return;
      }
      if (cVar1 == '\x13') {
        FUN_004398b0();
        FUN_0043ec40();
        return;
      }
      if (cVar1 == '\x14') {
        FUN_00439c60();
        FUN_004abe50();
        FUN_0042ec80(0);
        FUN_004aba50();
        FUN_0043ec40();
        return;
      }
      FUN_004abe50();
      FUN_0042ec80(0);
      FUN_004419d0();
      FUN_004aba50();
      FUN_0043ec40();
      return;
    }
    FUN_004abe50();
    FUN_0042f030(0);
    FUN_004aba50();
  }
  return;
}
