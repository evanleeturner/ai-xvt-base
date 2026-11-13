/* SPDX-License-Identifier: MIT */
/* z_bop 00448d00 */
extern int DAT_009fd394;
extern char DAT_009e9680;
extern char DAT_009ec468;
extern char DAT_009d6940;
extern int DAT_009ecc34;

char __stdcall FUN_00448d00(void)
{
  char cVar1;
  int iVar2;
  char *pcVar3;
  
  DAT_009fd394 = 0;
  iVar2 = 0;
  pcVar3 = &DAT_009e9680;
  do {
    cVar1 = *pcVar3;
    if ((cVar1 == '\x01') || (cVar1 == '\x02')) {
      DAT_009fd394 = DAT_009fd394 + 1;
    }
    if (cVar1 == '\x01') {
      iVar2 = iVar2 + 1;
    }
    pcVar3 = pcVar3 + 0x5bd;
  } while (pcVar3 < &DAT_009ec468);
  if (iVar2 == 0) {
    DAT_009d6940 = 1;
  }
  if (((char *)&DAT_009e9680)[DAT_009ecc34 * 0x5bd] == '\0') {
    DAT_009d6940 = 1;
  }
  return DAT_009d6940;
}
