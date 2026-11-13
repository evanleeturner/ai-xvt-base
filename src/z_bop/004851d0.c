/* SPDX-License-Identifier: MIT */
/* z_bop 004851d0 */
extern void *DAT_009ecc34;
extern char DAT_009e9680[];
extern char DAT_009ec468;
extern unsigned short _DAT_00a07be0;

void *FUN_0046f720(void *param_1);
void FUN_00451bf0(int param_1, void *param_2);
void FUN_00450650(int param_1, int param_2);

void FUN_004851d0(void *param_1)
{
  void *puVar1;
  char *pcVar2;
  int iVar3;

  if (DAT_009ecc34 != param_1) {
    puVar1 = FUN_0046f720(param_1);
    FUN_00451bf0(0, puVar1);
    FUN_00450650(0x17d, (int)DAT_009ecc34);
    iVar3 = 0;
    pcVar2 = DAT_009e9680;
    do {
      if (*pcVar2 == 1) {
        iVar3 = iVar3 + 1;
      }
      pcVar2 = pcVar2 + 0x5bd;
    } while (pcVar2 < &DAT_009ec468);
    if (DAT_009e9680[(int)DAT_009ecc34 * 0x5bd] == 1) {
      if (iVar3 == 1) {
        FUN_00450650(0x17f, (int)DAT_009ecc34);
        return;
      }
      _DAT_00a07be0 = (unsigned short)iVar3;
      FUN_00450650(0x17e, (int)DAT_009ecc34);
    }
  }
  return;
}
