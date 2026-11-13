/* SPDX-License-Identifier: MIT */
/* z_bop 00435bc0 */
extern short DAT_009a8e3c;
extern short DAT_009a7b60;
extern short DAT_009d682c;
extern short DAT_009a1fe4;
extern int DAT_0055635c;
extern int DAT_009ecc34;
extern int DAT_009e9a63;

void __cdecl FUN_004a9520(short, short, short, short);
void __stdcall FUN_00436320(void);
void __stdcall FUN_00436580(void);
void __cdecl FUN_00436780(int);
void __stdcall FUN_00437b20(void);

void __stdcall FUN_00435bc0(void)
{
  FUN_004a9520(DAT_009a8e3c, DAT_009a7b60, (short)(DAT_009d682c + DAT_009a8e3c), (short)(DAT_009a1fe4 + DAT_009a7b60));
  FUN_00436320();
  FUN_00436580();
  DAT_0055635c = 1;
  {
    int local_val;
    __builtin_memcpy(&local_val, (char*)&DAT_009e9a63 + DAT_009ecc34 * 0x5bd, 4);
    if (local_val < -0x10000) {
      FUN_00436780(1);
      FUN_00437b20();
      FUN_00436780(0);
    }
    else {
      FUN_00436780(0);
      FUN_00437b20();
      FUN_00436780(1);
    }
  }
  return;
}
