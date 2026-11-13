/* SPDX-License-Identifier: MIT */
/* z_xvt 004f82f0 */
extern int DAT_007596b7;
extern int DAT_007596bb;
extern int DAT_00759657;
extern int DAT_007596bf;
extern int DAT_007596c3;
extern int DAT_0075964b;
extern int DAT_00758830;

void __cdecl FUN_004f82f0(int param_1, int param_2, int param_3, unsigned short param_4)
{
  int iVar1;
  unsigned short *puVar2;
  
  if ((DAT_007596b7 <= param_3) && (param_3 <= DAT_007596bb)) {
    iVar1 = DAT_00759657 >> 4;
    if (param_2 < param_1) {
      if (((DAT_007596bf <= param_2) || (param_2 = DAT_007596bf, DAT_007596bf < param_1)) &&
         ((param_1 <= DAT_007596c3 || (param_1 = DAT_007596c3, param_2 < DAT_007596c3)))) {
        puVar2 = (unsigned short *)
                 (param_2 * DAT_0075964b + (param_3 << ((unsigned char)iVar1 & 0x1f)) + DAT_00758830);
        if (iVar1 == 0) {
          if (param_2 <= param_1) {
            iVar1 = (param_1 - param_2) + 1;
            do {
              *(unsigned char *)puVar2 = (unsigned char)param_4;
              puVar2 = (unsigned short *)((int)puVar2 + DAT_0075964b);
              iVar1 = iVar1 + -1;
            } while (iVar1 != 0);
          }
        }
        else if ((iVar1 == 1) && (param_2 <= param_1)) {
          iVar1 = (param_1 - param_2) + 1;
          do {
            *puVar2 = param_4;
            puVar2 = (unsigned short *)((int)puVar2 + (DAT_0075964b & 0xfffffffe));
            iVar1 = iVar1 + -1;
          } while (iVar1 != 0);
          return;
        }
      }
    }
    else if (((param_2 <= DAT_007596c3) || (param_2 = DAT_007596c3, param_1 <= DAT_007596c3)) &&
            ((DAT_007596bf <= param_1 || (param_1 = DAT_007596bf, DAT_007596bf <= param_2)))) {
      puVar2 = (unsigned short *)
               (param_1 * DAT_0075964b + (param_3 << ((unsigned char)iVar1 & 0x1f)) + DAT_00758830);
      if (iVar1 == 0) {
        if (param_1 <= param_2) {
          iVar1 = (param_2 - param_1) + 1;
          do {
            *(unsigned char *)puVar2 = (unsigned char)param_4;
            puVar2 = (unsigned short *)((int)puVar2 + DAT_0075964b);
            iVar1 = iVar1 + -1;
          } while (iVar1 != 0);
          return;
        }
      }
      else if ((iVar1 == 1) && (param_1 <= param_2)) {
        iVar1 = (param_2 - param_1) + 1;
        do {
          *puVar2 = param_4;
          puVar2 = (unsigned short *)((int)puVar2 + (DAT_0075964b & 0xfffffffe));
          iVar1 = iVar1 + -1;
        } while (iVar1 != 0);
        return;
      }
    }
  }
  return;
}
