/* SPDX-License-Identifier: MIT */
/* z_xvt 00465740 */
/* Unresolved global symbols referenced by absolute address.
   Types and widths derived from P-code operand sizes. */
extern unsigned int   DAT_006a8826;
extern unsigned short DAT_006a8824;
extern unsigned int   DAT_006a8838;
extern unsigned short DAT_006a8836;
extern unsigned int   DAT_006a8854;
extern unsigned short DAT_006a884e;
extern unsigned int   DAT_006a8862;
extern unsigned short DAT_006a885c;
extern unsigned int   DAT_006a8870;
extern unsigned short DAT_006a886a;
extern unsigned short DAT_006a8878;
extern unsigned int   DAT_006a887a;
extern unsigned short DAT_006a8882;
extern unsigned int   DAT_006a8884;
extern unsigned short DAT_006a888c;
extern unsigned short DAT_006a8892;
extern unsigned short DAT_006a8898;
extern unsigned int   DAT_006a889a;
extern unsigned short DAT_006a8844;
extern unsigned int   DAT_005f1d3c;
extern unsigned int   DAT_005f1d68;
extern unsigned int   DAT_005f262c;
extern unsigned int   DAT_005f1d34;
extern unsigned int   DAT_005f2600;
extern unsigned int   DAT_0052a390;

/* Forward declarations matching evidence signatures and P-code widths */
unsigned short __cdecl FUN_0048f3c0(unsigned int param_1, unsigned int param_2);
void           __cdecl FUN_0047f010(unsigned short param_1);
void           __cdecl FUN_004a7340(void *param_1, unsigned int param_2);

void __stdcall FUN_00465740(void)
{
  DAT_006a8826 = 20000;
  DAT_006a8824 = FUN_0048f3c0(480000, 0);
  if (DAT_006a8824 == 0) {
    FUN_0047f010(0);
  }
  DAT_006a8838 = 20000;
  DAT_006a8836 = FUN_0048f3c0(80000, 0);
  if (DAT_006a8836 == 0) {
    FUN_0047f010(0);
  }
  DAT_006a8854 = 5000;
  DAT_006a884e = FUN_0048f3c0(560000, 0);
  if (DAT_006a884e == 0) {
    FUN_0047f010(0);
  }
  DAT_006a8862 = DAT_006a887a * 2;
  DAT_006a885c = FUN_0048f3c0(DAT_006a887a * 0x30, 0);
  if (DAT_006a885c == 0) {
    FUN_0047f010(0);
  }
  DAT_006a8870 = DAT_006a8884 * 2;
  DAT_006a886a = FUN_0048f3c0(DAT_006a8884 * 0x38, 0);
  if (DAT_006a886a == 0) {
    FUN_0047f010(0);
  }
  DAT_006a8878 = FUN_0048f3c0(DAT_006a887a << 2, 0);
  if (DAT_006a8878 == 0) {
    FUN_0047f010(0);
  }
  DAT_006a8882 = FUN_0048f3c0(DAT_006a8884 << 2, 0);
  if (DAT_006a8882 == 0) {
    FUN_0047f010(0);
  }
  DAT_006a888c = FUN_0048f3c0(0xc00, 0);
  if (DAT_006a888c == 0) {
    FUN_0047f010(0);
  }
  DAT_006a8892 = FUN_0048f3c0(0xc00, 0);
  if (DAT_006a8892 == 0) {
    FUN_0047f010(0);
  }
  DAT_005f1d3c = 0x10;
  DAT_005f1d68 = DAT_0052a390;
  DAT_005f262c = 4;
  DAT_005f1d34 = 0xf;
  DAT_005f2600 = 0x41800000;
  DAT_006a8844 = FUN_0048f3c0(0x25800, 0);
  if (DAT_006a8844 == 0) {
    FUN_0047f010(0);
  }
  DAT_006a889a = 500;
  DAT_006a8898 = FUN_0048f3c0(0x19640, 0);
  if (DAT_006a8898 == 0) {
    FUN_0047f010(0);
  }
  FUN_004a7340((void *)0x465957, 0x80000);
  return;
}
