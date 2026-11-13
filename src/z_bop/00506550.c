/* SPDX-License-Identifier: MIT */
/* z_bop 00506550 */
typedef unsigned char undefined1;
typedef unsigned short undefined2;

extern int DAT_00aa7b82;
extern int DAT_00aa7b86;
extern int DAT_00aa7b1f;
extern int DAT_00aa7b8a;
extern int DAT_00aa7b8e;
extern int DAT_00aa7b13;
extern int DAT_00aa6cfc;

void __cdecl FUN_00506550(int param_1, int param_2, int param_3, undefined2 param_4)
{
  int iVar1;
  undefined2 *puVar2;
  
  if ((DAT_00aa7b82 <= param_3) && (param_3 <= DAT_00aa7b86)) {
    iVar1 = DAT_00aa7b1f >> 4;
    if (param_2 < param_1) {
      if (((DAT_00aa7b8a <= param_2) || (param_2 = DAT_00aa7b8a, DAT_00aa7b8a < param_1)) &&
         ((param_1 <= DAT_00aa7b8e || (param_1 = DAT_00aa7b8e, param_2 < DAT_00aa7b8e)))) {
        puVar2 = (undefined2 *)
                 (DAT_00aa7b13 * param_2 + (param_3 << ((unsigned char)iVar1 & 0x1f)) + DAT_00aa6cfc);
        if (iVar1 == 0) {
          if (param_2 <= param_1) {
            iVar1 = (param_1 - param_2) + 1;
            do {
              *(undefined1 *)puVar2 = (undefined1)param_4;
              puVar2 = (undefined2 *)((int)puVar2 + DAT_00aa7b13);
              iVar1 = iVar1 + -1;
            } while (iVar1 != 0);
          }
        }
        else if ((iVar1 == 1) && (param_2 <= param_1)) {
          iVar1 = (param_1 - param_2) + 1;
          do {
            *puVar2 = param_4;
            puVar2 = (undefined2 *)((int)puVar2 + (DAT_00aa7b13 & 0xfffffffe));
            iVar1 = iVar1 + -1;
          } while (iVar1 != 0);
          return;
        }
      }
    }
    else if (((param_2 <= DAT_00aa7b8e) || (param_2 = DAT_00aa7b8e, param_1 <= DAT_00aa7b8e)) &&
            ((DAT_00aa7b8a <= param_1 || (param_1 = DAT_00aa7b8a, DAT_00aa7b8a <= param_2)))) {
      puVar2 = (undefined2 *)
               (DAT_00aa7b13 * param_1 + (param_3 << ((unsigned char)iVar1 & 0x1f)) + DAT_00aa6cfc);
      if (iVar1 == 0) {
        if (param_1 <= param_2) {
          iVar1 = (param_2 - param_1) + 1;
          do {
            *(undefined1 *)puVar2 = (undefined1)param_4;
            puVar2 = (undefined2 *)((int)puVar2 + DAT_00aa7b13);
            iVar1 = iVar1 + -1;
          } while (iVar1 != 0);
          return;
        }
      }
      else if ((iVar1 == 1) && (param_1 <= param_2)) {
        iVar1 = (param_2 - param_1) + 1;
        do {
          *puVar2 = param_4;
          puVar2 = (undefined2 *)((int)puVar2 + (DAT_00aa7b13 & 0xfffffffe));
          iVar1 = iVar1 + -1;
        } while (iVar1 != 0);
        return;
      }
    }
  }
  return;
}
