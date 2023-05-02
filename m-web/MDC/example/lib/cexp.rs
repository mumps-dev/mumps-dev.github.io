2-Jul-96, 6:54:35
Routine Save for Complex Mathematics Library function $%CEXP^MATH
 ;
 ; Unless otherwise noted, the code below
 ; was approved in document X11/95-14
 ;
 ; If corrections have been applied,
 ; first the original line appears,
 ; with three semicolons at the beginning of the line.
 ;
 ; Then the source of the correction is acknowledged,
 ; then the corrected line appears, followed by a
 ; line containing three semicolons.
 ;
CEXP(Z,PREC) ;
 New R,ZIM,ZRE
 ;
 ;;;
 ; Alan Frank (October 1995)
 Set PREC=$Get(PREC,11)
 ;;;
 ;
 Set ZRE=+Z,ZIM=+$Piece(Z,"%",2)
 Set R=$%EXP^MATH(ZRE,PREC)
 Quit R*$%COS^MATH(ZIM,PREC)_"%"_(R*$%SIN^MATH(ZIM,PREC))
 ;===
 ;
 ;
 ;


